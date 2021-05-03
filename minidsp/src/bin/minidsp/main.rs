//! MiniDSP Control Program

#![allow(clippy::upper_case_acronyms)]

use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::PathBuf,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use bytes::Bytes;
use clap::Clap;
use debug::DebugCommands;
use futures::{stream, StreamExt};
use handlers::run_server;
use minidsp::{
    builder::{Builder, DeviceHandle},
    device::DeviceKind,
    tcp_server,
    transport::net::{self, discovery},
    Gain, MiniDSP, MiniDSPError, Source,
};

mod debug;
mod handlers;

use std::{io::Read, time::Duration};

#[cfg(feature = "hid")]
use minidsp::transport::hid;

#[derive(Clone, Clap, Debug)]
#[clap(version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    /// Verbosity level. -v display decoded commands and responses -vv display decoded commands including readfloats -vvv display hex data frames
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    /// Output response format (text (default), json, jsonline)
    #[clap(long = "output", short = 'o', default_value = "text")]
    output_format: OutputFormat,

    #[clap(long, env = "MINIDSP_LOG")]
    /// Log commands and responses to a file
    log: Option<PathBuf>,

    /// Apply the given commands to all matching local usb devices
    #[clap(long)]
    all_local_devices: bool,

    /// The USB vendor and product id (2752:0011 for the 2x4HD)
    #[clap(name = "usb", env = "MINIDSP_USB", long)]
    #[cfg(feature = "hid")]
    hid_option: Option<hid::Device>,

    #[clap(name = "tcp", env = "MINIDSP_TCP", long)]
    /// The target address of the server component
    tcp_option: Option<String>,

    /// Force the device to a specific product instead of probing its hardware id. May break things, use at your own risk.
    #[clap(name = "force-kind", long)]
    force_kind: Option<DeviceKind>,

    /// Directly connect to this transport url
    #[clap(long, env = "MINIDSP_URL")]
    url: Option<String>,

    /// Discover devices that are managed by the remote instance of minidspd
    #[clap(long, env = "MINIDSPD_URL")]
    daemon_url: Option<String>,

    /// Discover devices that are managed by the local instance of minidspd
    #[clap(long, env = "MINIDSP_SOCK")]
    #[cfg(target_family = "unix")]
    daemon_sock: Option<String>,

    #[clap(short = 'f')]
    /// Read commands to run from the given filename (use - for stdin)
    file: Option<PathBuf>,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

impl Opts {
    // Applies transport and logging options to this builder
    async fn apply_builder(&self, builder: &mut Builder) -> Result<(), MiniDSPError> {
        let mut bound = false;
        #[cfg(target_family = "unix")]
        if let Some(socket_path) = &self.daemon_sock {
            builder.with_unix_socket(socket_path).await?;
            bound = true;
        }

        if bound {
        } else if let Some(tcp) = &self.tcp_option {
            let tcp = if tcp.contains(':') {
                tcp.to_string()
            } else {
                format!("{}:5333", tcp)
            };
            builder.with_tcp(tcp).unwrap();
        } else if let Some(url) = &self.url {
            builder
                .with_url(url)
                .map_err(|_| MiniDSPError::InvalidURL)?;
        } else if let Some(url) = &self.daemon_url {
            builder.with_http(url).await?;
        } else if let Some(device) = self.hid_option.as_ref() {
            if let Some(ref path) = device.path {
                builder.with_usb_path(path);
            } else if let Some((vid, pid)) = device.id {
                builder.with_usb_product_id(vid, pid)?;
            }
        } else {
            #[cfg(target_family = "unix")]
            let _ = builder.with_unix_socket("/tmp/minidsp.sock").await;
            let _ = builder.with_default_usb();
        }

        builder.with_logging(self.verbose as u8, self.log.clone());

        if let Some(force_kind) = self.force_kind {
            builder.force_device_kind(force_kind);
        }

        Ok(())
    }
}

#[derive(Clone, Clap, Debug)]
enum SubCommand {
    /// Try to find reachable devices
    Probe {
        #[clap(short)]
        net: bool,
    },

    /// Prints the master status and current levels
    Status,

    /// Set the master output gain [-127, 0]
    Gain { value: Gain },

    /// Set the master mute status
    Mute {
        #[clap(parse(try_from_str = on_or_off))]
        value: bool,
    },
    /// Set the active input source
    Source { value: Source },

    /// Set the current active configuration,
    Config { value: u8 },

    /// Control settings regarding input channels
    Input {
        /// Index of the input channel, starting at 0
        input_index: usize,

        #[clap(subcommand)]
        cmd: InputCommand,
    },

    /// Control settings regarding output channels
    Output {
        /// Index of the output channel, starting at 0
        output_index: usize,

        #[clap(subcommand)]
        cmd: OutputCommand,
    },

    /// Launch a server usable with `--tcp`, the mobile application, and the official client
    Server {
        #[clap(default_value = "0.0.0.0:5333")]
        bind_address: String,
        #[clap(long)]
        advertise: Option<String>,
        #[clap(long)]
        ip: Option<String>,
    },

    /// Low-level debug utilities
    Debug {
        #[clap(subcommand)]
        cmd: DebugCommands,
    },
}

#[derive(Clone, Clap, Debug)]
enum InputCommand {
    /// Set the input gain for this channel
    Gain {
        /// Gain in dB
        value: Gain,
    },

    /// Set the master mute status
    Mute {
        #[clap(parse(try_from_str = on_or_off))]
        value: bool,
    },

    /// Controls signal routing from this input
    Routing {
        /// Index of the output channel starting at 0
        output_index: usize,

        #[clap(subcommand)]
        cmd: RoutingCommand,
    },

    /// Control the parametric equalizer
    PEQ {
        /// Parametric EQ index (all | <id>) (0 to 9 inclusively)
        index: PEQTarget,

        #[clap(subcommand)]
        cmd: FilterCommand,
    },
}

#[derive(Clone, Clap, Debug)]
enum RoutingCommand {
    /// Controls whether the output matrix for this input is enabled for the given output index
    Enable {
        #[clap(parse(try_from_str = on_or_off))]
        /// Whether this input is enabled for the given output channel
        value: bool,
    },
    Gain {
        /// Output gain in dB
        value: Gain,
    },
}

#[derive(Clone, Clap, Debug)]
enum OutputCommand {
    /// Set the output gain for this channel
    Gain {
        /// Output gain in dB
        value: Gain,
    },

    /// Set the master mute status
    Mute {
        #[clap(parse(try_from_str = on_or_off))]
        value: bool,
    },

    /// Set the delay associated to this channel
    Delay {
        /// Delay in milliseconds
        delay: f32,
    },

    /// Set phase inversion on this channel
    Invert {
        #[clap(parse(try_from_str = on_or_off))]
        value: bool,
    },

    /// Control the parametric equalizer
    PEQ {
        /// Parametric EQ index (all | <id>) (0 to 9 inclusively)
        index: PEQTarget,

        #[clap(subcommand)]
        cmd: FilterCommand,
    },

    /// Control the FIR filter
    FIR {
        #[clap(subcommand)]
        cmd: FilterCommand,
    },

    /// Control crossovers (2x 4 biquads)
    Crossover {
        /// Group index (0 or 1)
        group: usize,

        /// Filter index (all | 0 | 1 | 3)
        index: PEQTarget,

        #[clap(subcommand)]
        cmd: FilterCommand,
    },

    /// Control the compressor
    Compressor {
        /// Bypasses the compressor (on | off)
        #[clap(short='b', long, parse(try_from_str = on_or_off))]
        bypass: Option<bool>,

        /// Sets the threshold in dBFS
        #[clap(short = 't', long, allow_hyphen_values(true))]
        threshold: Option<f32>,

        /// Sets the ratio
        #[clap(short = 'k', long)]
        ratio: Option<f32>,

        /// Sets the attack time in ms
        #[clap(short = 'a', long)]
        attack: Option<f32>,

        /// Sets the release time in ms
        #[clap(short = 'r', long)]
        release: Option<f32>,
    },
}

#[derive(Debug, Clone, Copy)]
enum PEQTarget {
    All,
    One(usize),
}

impl FromStr for PEQTarget {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "all" {
            Ok(PEQTarget::All)
        } else {
            Ok(PEQTarget::One(usize::from_str(s)?))
        }
    }
}

#[derive(Clone, Clap, Debug)]
enum FilterCommand {
    /// Set coefficients
    Set {
        /// Coefficients
        coeff: Vec<f32>,
    },

    /// Sets the bypass toggle
    Bypass {
        #[clap(parse(try_from_str = on_or_off))]
        value: bool,
    },

    /// Sets all coefficients back to their default values and un-bypass them
    Clear,

    /// Imports the coefficients from the given file
    Import {
        /// Filename containing the coefficients in REW format
        filename: PathBuf,
        /// Import file format
        format: Option<String>,
    },
}

#[derive(Debug, Clap)]
pub struct ProductId {
    pub vid: u16,
    pub pid: Option<u16>,
}

#[derive(Debug, strum::EnumString, strum::ToString, Clone, Copy, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum OutputFormat {
    Text,
    Json,
    JsonLine,
}

impl OutputFormat {
    pub fn format<T>(self, obj: &T) -> String
    where
        T: serde::Serialize + fmt::Display,
    {
        match self {
            OutputFormat::Text => format!("{}", obj),
            OutputFormat::Json => {
                serde_json::to_string_pretty(obj).expect("couldn't serialize object as json")
            }
            OutputFormat::JsonLine => {
                serde_json::to_string(obj).expect("couldn't serialize object as json")
            }
        }
    }
}

impl FromStr for ProductId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() > 2 {
            return Err("");
        }

        let vid = u16::from_str_radix(parts[0], 16).map_err(|_| "couldn't parse vendor id")?;
        let mut pid: Option<u16> = None;
        if parts.len() > 1 {
            pid = Some(u16::from_str_radix(parts[1], 16).map_err(|_| "couldn't parse product id")?);
        }

        Ok(ProductId { vid, pid })
    }
}

async fn run_probe(devices: Vec<DeviceHandle>, net: bool) -> Result<()> {
    for dev in &devices {
        println!(
            "Found {} with serial {} at {} [hw_id: {}, dsp_version: {}]",
            dev.device_spec.product_name,
            dev.device_info.serial,
            dev.url,
            dev.device_info.hw_id,
            dev.device_info.dsp_version
        );
    }

    if net {
        println!("Probing for network devices...");
        let devices = net::discover_timeout(Duration::from_secs(8)).await?;
        if devices.is_empty() {
            println!("No network devices detected")
        } else {
            for device in &devices {
                println!("Found: {}", device);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let opts: Opts = Opts::parse();
    let mut builder = Builder::new();
    opts.apply_builder(&mut builder).await?;

    let mut devices: Vec<_> = builder
        .probe()
        .filter_map(|x| async move { x.ok() })
        .collect()
        .await;

    if let Some(SubCommand::Probe { net }) = opts.subcmd {
        run_probe(devices, net).await?;
        return Ok(());
    }

    if !opts.all_local_devices {
        devices.truncate(1);
    }

    if let Some(SubCommand::Server { .. }) = opts.subcmd {
        log::warn!("The `server` command is deprecated and will be removed in a future release. Use `minidspd` instead.");

        let transport = devices
            .first()
            .ok_or_else(|| anyhow!("No devices found"))?
            .transport
            .try_clone()
            .expect("device has disappeared");

        run_server(opts.subcmd.unwrap(), Box::pin(transport)).await?;
        return Ok(());
    }

    let devices: Vec<_> = devices
        .into_iter()
        .map(|dev| dev.to_minidsp().expect("device has disappeared"))
        .collect();

    if devices.is_empty() {
        return Err(anyhow!("No devices found"));
    }

    match &opts.file {
        Some(filename) => {
            let file: Box<dyn Read> = {
                if filename.to_string_lossy() == "-" {
                    Box::new(std::io::stdin())
                } else {
                    Box::new(File::open(filename)?)
                }
            };
            let reader = BufReader::new(file);
            let cmds = reader.lines().filter_map(|s| {
                let trimmed = s.ok()?.trim().to_string();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    Some(trimmed)
                } else {
                    None
                }
            });

            for cmd in cmds {
                let words = shellwords::split(&cmd)?;
                let prefix = &["minidsp".to_string()];
                let words = prefix.iter().chain(words.iter());
                let this_opts = Opts::try_parse_from(words);
                let this_opts = match this_opts {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("While executing: {}\n{}", cmd, e);
                        return Err(anyhow!("Command failure"));
                    }
                };

                // Run this command on all devices in parallel
                stream::iter(&devices)
                    .then(|dev| handlers::run_command(dev, this_opts.subcmd.as_ref(), &opts))
                    .collect::<Vec<_>>()
                    .await;
            }
        }
        None => {
            stream::iter(&devices)
                .then(|dev| handlers::run_command(dev, opts.subcmd.as_ref(), &opts))
                .collect::<Vec<_>>()
                .await;
        }
    }

    Ok(())
}

fn on_or_off(s: &str) -> Result<bool, &'static str> {
    match s {
        "on" => Ok(true),
        "true" => Ok(true),
        "off" => Ok(false),
        "false" => Ok(false),
        _ => Err("expected `on`, `true`, `off`, `false`"),
    }
}

fn parse_hex(s: &str) -> Result<Bytes, hex::FromHexError> {
    Ok(Bytes::from(hex::decode(s.replace(" ", ""))?))
}

fn parse_hex_u16(src: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(src, 16)
}