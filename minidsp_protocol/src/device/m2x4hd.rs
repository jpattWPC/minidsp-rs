//
// This file is generated by `minidsp-devtools codegen`. DO NOT EDIT.
//
use super::*;
pub mod sym {
    #[allow(dead_code)]
    pub const D_GAIN_1_0_STATUS: u16 = 0;
    pub const D_GAIN_2_0_STATUS: u16 = 1;
    pub const D_GAIN_3_0_STATUS: u16 = 2;
    pub const D_GAIN_4_0_STATUS: u16 = 3;
    pub const D_GAIN_5_0_STATUS: u16 = 4;
    pub const D_GAIN_6_0_STATUS: u16 = 5;
    pub const MIXER_NX_M_SMOOTHED_1_0_0_STATUS: u16 = 6;
    pub const MIXER_NX_M_SMOOTHED_1_0_1_STATUS: u16 = 7;
    pub const MIXER_NX_M_SMOOTHED_1_0_2_STATUS: u16 = 8;
    pub const MIXER_NX_M_SMOOTHED_1_0_3_STATUS: u16 = 9;
    pub const MIXER_NX_M_SMOOTHED_1_1_0_STATUS: u16 = 10;
    pub const MIXER_NX_M_SMOOTHED_1_1_1_STATUS: u16 = 11;
    pub const MIXER_NX_M_SMOOTHED_1_1_2_STATUS: u16 = 12;
    pub const MIXER_NX_M_SMOOTHED_1_1_3_STATUS: u16 = 13;
    pub const FIR_3_0_STATUS: u16 = 14;
    pub const FIR_4_0_STATUS: u16 = 15;
    pub const FIR_5_0_STATUS: u16 = 16;
    pub const FIR_6_0_STATUS: u16 = 17;
    pub const DELAY_3_0_STATUS: u16 = 18;
    pub const DELAY_4_0_STATUS: u16 = 19;
    pub const DELAY_5_0_STATUS: u16 = 20;
    pub const DELAY_6_0_STATUS: u16 = 21;
    pub const COMP_3_0_STATUS: u16 = 22;
    pub const COMP_4_0_STATUS: u16 = 23;
    pub const COMP_5_0_STATUS: u16 = 24;
    pub const COMP_6_0_STATUS: u16 = 25;
    pub const D_GAIN_1_0: u16 = 26;
    pub const D_GAIN_2_0: u16 = 27;
    pub const D_GAIN_3_0: u16 = 28;
    pub const D_GAIN_4_0: u16 = 29;
    pub const D_GAIN_5_0: u16 = 30;
    pub const D_GAIN_6_0: u16 = 31;
    pub const MIXER_NX_M_SMOOTHED_1_0_0: u16 = 32;
    pub const MIXER_NX_M_SMOOTHED_1_0_1: u16 = 33;
    pub const MIXER_NX_M_SMOOTHED_1_0_2: u16 = 34;
    pub const MIXER_NX_M_SMOOTHED_1_0_3: u16 = 35;
    pub const MIXER_NX_M_SMOOTHED_1_1_0: u16 = 36;
    pub const MIXER_NX_M_SMOOTHED_1_1_1: u16 = 37;
    pub const MIXER_NX_M_SMOOTHED_1_1_2: u16 = 38;
    pub const MIXER_NX_M_SMOOTHED_1_1_3: u16 = 39;
    pub const COMP_3_0_THRESHOLD: u16 = 40;
    pub const COMP_3_0_GAIN: u16 = 41;
    pub const COMP_3_0_RATIO: u16 = 42;
    pub const COMP_3_0_KNEE: u16 = 43;
    pub const COMP_3_0_ATIME: u16 = 44;
    pub const COMP_3_0_RTIME: u16 = 45;
    pub const COMP_4_0_THRESHOLD: u16 = 46;
    pub const COMP_4_0_GAIN: u16 = 47;
    pub const COMP_4_0_RATIO: u16 = 48;
    pub const COMP_4_0_KNEE: u16 = 49;
    pub const COMP_4_0_ATIME: u16 = 50;
    pub const COMP_4_0_RTIME: u16 = 51;
    pub const COMP_5_0_THRESHOLD: u16 = 52;
    pub const COMP_5_0_GAIN: u16 = 53;
    pub const COMP_5_0_RATIO: u16 = 54;
    pub const COMP_5_0_KNEE: u16 = 55;
    pub const COMP_5_0_ATIME: u16 = 56;
    pub const COMP_5_0_RTIME: u16 = 57;
    pub const COMP_6_0_THRESHOLD: u16 = 58;
    pub const COMP_6_0_GAIN: u16 = 59;
    pub const COMP_6_0_RATIO: u16 = 60;
    pub const COMP_6_0_KNEE: u16 = 61;
    pub const COMP_6_0_ATIME: u16 = 62;
    pub const COMP_6_0_RTIME: u16 = 63;
    pub const DELAY_3_0: u16 = 64;
    pub const DELAY_4_0: u16 = 65;
    pub const DELAY_5_0: u16 = 66;
    pub const DELAY_6_0: u16 = 67;
    pub const METER_02_C1_0: u16 = 68;
    pub const METER_02_C1_1: u16 = 69;
    pub const METER_10_C1_0: u16 = 70;
    pub const METER_10_C1_1: u16 = 71;
    pub const METER_10_C1_2: u16 = 72;
    pub const METER_10_C1_3: u16 = 73;
    pub const METER_10_C1_4: u16 = 74;
    pub const METER_10_C1_5: u16 = 75;
    pub const METER_10_C1_6: u16 = 76;
    pub const METER_10_C1_7: u16 = 77;
    pub const POLARITY_IN_1_0: u16 = 78;
    pub const POLARITY_IN_2_0: u16 = 79;
    pub const POLARITY_OUT_1_0: u16 = 80;
    pub const POLARITY_OUT_2_0: u16 = 81;
    pub const POLARITY_OUT_3_0: u16 = 82;
    pub const POLARITY_OUT_4_0: u16 = 83;
    pub const FIR_3_0_TAPS: u16 = 84;
    pub const FIR_3_0: u16 = 85;
    pub const FIR_4_0_TAPS: u16 = 2133;
    pub const FIR_4_0: u16 = 2134;
    pub const FIR_5_0_TAPS: u16 = 4182;
    pub const FIR_5_0: u16 = 4183;
    pub const FIR_6_0_TAPS: u16 = 6231;
    pub const FIR_6_0: u16 = 6232;
    pub const PEQ_1_1: u16 = 8280;
    pub const PEQ_1_2: u16 = 8285;
    pub const PEQ_1_3: u16 = 8290;
    pub const PEQ_1_4: u16 = 8295;
    pub const PEQ_1_5: u16 = 8300;
    pub const PEQ_1_6: u16 = 8305;
    pub const PEQ_1_7: u16 = 8310;
    pub const PEQ_1_8: u16 = 8315;
    pub const PEQ_1_9: u16 = 8320;
    pub const PEQ_1_10: u16 = 8325;
    pub const PEQ_2_1: u16 = 8330;
    pub const PEQ_2_2: u16 = 8335;
    pub const PEQ_2_3: u16 = 8340;
    pub const PEQ_2_4: u16 = 8345;
    pub const PEQ_2_5: u16 = 8350;
    pub const PEQ_2_6: u16 = 8355;
    pub const PEQ_2_7: u16 = 8360;
    pub const PEQ_2_8: u16 = 8365;
    pub const PEQ_2_9: u16 = 8370;
    pub const PEQ_2_10: u16 = 8375;
    pub const PEQ_3_1: u16 = 8380;
    pub const PEQ_3_2: u16 = 8385;
    pub const PEQ_3_3: u16 = 8390;
    pub const PEQ_3_4: u16 = 8395;
    pub const PEQ_3_5: u16 = 8400;
    pub const PEQ_3_6: u16 = 8405;
    pub const PEQ_3_7: u16 = 8410;
    pub const PEQ_3_8: u16 = 8415;
    pub const PEQ_3_9: u16 = 8420;
    pub const PEQ_3_10: u16 = 8425;
    pub const PEQ_4_1: u16 = 8430;
    pub const PEQ_4_2: u16 = 8435;
    pub const PEQ_4_3: u16 = 8440;
    pub const PEQ_4_4: u16 = 8445;
    pub const PEQ_4_5: u16 = 8450;
    pub const PEQ_4_6: u16 = 8455;
    pub const PEQ_4_7: u16 = 8460;
    pub const PEQ_4_8: u16 = 8465;
    pub const PEQ_4_9: u16 = 8470;
    pub const PEQ_4_10: u16 = 8475;
    pub const PEQ_5_1: u16 = 8480;
    pub const PEQ_5_2: u16 = 8485;
    pub const PEQ_5_3: u16 = 8490;
    pub const PEQ_5_4: u16 = 8495;
    pub const PEQ_5_5: u16 = 8500;
    pub const PEQ_5_6: u16 = 8505;
    pub const PEQ_5_7: u16 = 8510;
    pub const PEQ_5_8: u16 = 8515;
    pub const PEQ_5_9: u16 = 8520;
    pub const PEQ_5_10: u16 = 8525;
    pub const PEQ_6_1: u16 = 8530;
    pub const PEQ_6_2: u16 = 8535;
    pub const PEQ_6_3: u16 = 8540;
    pub const PEQ_6_4: u16 = 8545;
    pub const PEQ_6_5: u16 = 8550;
    pub const PEQ_6_6: u16 = 8555;
    pub const PEQ_6_7: u16 = 8560;
    pub const PEQ_6_8: u16 = 8565;
    pub const PEQ_6_9: u16 = 8570;
    pub const PEQ_6_10: u16 = 8575;
    pub const BPF_3_1: u16 = 8580;
    pub const BPF_3_5: u16 = 8600;
    pub const BPF_4_1: u16 = 8620;
    pub const BPF_4_5: u16 = 8640;
    pub const BPF_5_1: u16 = 8660;
    pub const BPF_5_5: u16 = 8680;
    pub const BPF_6_1: u16 = 8700;
    pub const BPF_6_5: u16 = 8720;
    #[cfg(feature = "symbols")]
    pub const SYMBOLS: &[(&str, u16)] = &[
        ("D_GAIN_1_0_STATUS", D_GAIN_1_0_STATUS),
        ("D_GAIN_2_0_STATUS", D_GAIN_2_0_STATUS),
        ("D_GAIN_3_0_STATUS", D_GAIN_3_0_STATUS),
        ("D_GAIN_4_0_STATUS", D_GAIN_4_0_STATUS),
        ("D_GAIN_5_0_STATUS", D_GAIN_5_0_STATUS),
        ("D_GAIN_6_0_STATUS", D_GAIN_6_0_STATUS),
        (
            "MIXER_NX_M_SMOOTHED_1_0_0_STATUS",
            MIXER_NX_M_SMOOTHED_1_0_0_STATUS,
        ),
        (
            "MIXER_NX_M_SMOOTHED_1_0_1_STATUS",
            MIXER_NX_M_SMOOTHED_1_0_1_STATUS,
        ),
        (
            "MIXER_NX_M_SMOOTHED_1_0_2_STATUS",
            MIXER_NX_M_SMOOTHED_1_0_2_STATUS,
        ),
        (
            "MIXER_NX_M_SMOOTHED_1_0_3_STATUS",
            MIXER_NX_M_SMOOTHED_1_0_3_STATUS,
        ),
        (
            "MIXER_NX_M_SMOOTHED_1_1_0_STATUS",
            MIXER_NX_M_SMOOTHED_1_1_0_STATUS,
        ),
        (
            "MIXER_NX_M_SMOOTHED_1_1_1_STATUS",
            MIXER_NX_M_SMOOTHED_1_1_1_STATUS,
        ),
        (
            "MIXER_NX_M_SMOOTHED_1_1_2_STATUS",
            MIXER_NX_M_SMOOTHED_1_1_2_STATUS,
        ),
        (
            "MIXER_NX_M_SMOOTHED_1_1_3_STATUS",
            MIXER_NX_M_SMOOTHED_1_1_3_STATUS,
        ),
        ("FIR_3_0_STATUS", FIR_3_0_STATUS),
        ("FIR_4_0_STATUS", FIR_4_0_STATUS),
        ("FIR_5_0_STATUS", FIR_5_0_STATUS),
        ("FIR_6_0_STATUS", FIR_6_0_STATUS),
        ("DELAY_3_0_STATUS", DELAY_3_0_STATUS),
        ("DELAY_4_0_STATUS", DELAY_4_0_STATUS),
        ("DELAY_5_0_STATUS", DELAY_5_0_STATUS),
        ("DELAY_6_0_STATUS", DELAY_6_0_STATUS),
        ("COMP_3_0_STATUS", COMP_3_0_STATUS),
        ("COMP_4_0_STATUS", COMP_4_0_STATUS),
        ("COMP_5_0_STATUS", COMP_5_0_STATUS),
        ("COMP_6_0_STATUS", COMP_6_0_STATUS),
        ("D_GAIN_1_0", D_GAIN_1_0),
        ("D_GAIN_2_0", D_GAIN_2_0),
        ("D_GAIN_3_0", D_GAIN_3_0),
        ("D_GAIN_4_0", D_GAIN_4_0),
        ("D_GAIN_5_0", D_GAIN_5_0),
        ("D_GAIN_6_0", D_GAIN_6_0),
        ("MIXER_NX_M_SMOOTHED_1_0_0", MIXER_NX_M_SMOOTHED_1_0_0),
        ("MIXER_NX_M_SMOOTHED_1_0_1", MIXER_NX_M_SMOOTHED_1_0_1),
        ("MIXER_NX_M_SMOOTHED_1_0_2", MIXER_NX_M_SMOOTHED_1_0_2),
        ("MIXER_NX_M_SMOOTHED_1_0_3", MIXER_NX_M_SMOOTHED_1_0_3),
        ("MIXER_NX_M_SMOOTHED_1_1_0", MIXER_NX_M_SMOOTHED_1_1_0),
        ("MIXER_NX_M_SMOOTHED_1_1_1", MIXER_NX_M_SMOOTHED_1_1_1),
        ("MIXER_NX_M_SMOOTHED_1_1_2", MIXER_NX_M_SMOOTHED_1_1_2),
        ("MIXER_NX_M_SMOOTHED_1_1_3", MIXER_NX_M_SMOOTHED_1_1_3),
        ("COMP_3_0_THRESHOLD", COMP_3_0_THRESHOLD),
        ("COMP_3_0_GAIN", COMP_3_0_GAIN),
        ("COMP_3_0_RATIO", COMP_3_0_RATIO),
        ("COMP_3_0_KNEE", COMP_3_0_KNEE),
        ("COMP_3_0_ATIME", COMP_3_0_ATIME),
        ("COMP_3_0_RTIME", COMP_3_0_RTIME),
        ("COMP_4_0_THRESHOLD", COMP_4_0_THRESHOLD),
        ("COMP_4_0_GAIN", COMP_4_0_GAIN),
        ("COMP_4_0_RATIO", COMP_4_0_RATIO),
        ("COMP_4_0_KNEE", COMP_4_0_KNEE),
        ("COMP_4_0_ATIME", COMP_4_0_ATIME),
        ("COMP_4_0_RTIME", COMP_4_0_RTIME),
        ("COMP_5_0_THRESHOLD", COMP_5_0_THRESHOLD),
        ("COMP_5_0_GAIN", COMP_5_0_GAIN),
        ("COMP_5_0_RATIO", COMP_5_0_RATIO),
        ("COMP_5_0_KNEE", COMP_5_0_KNEE),
        ("COMP_5_0_ATIME", COMP_5_0_ATIME),
        ("COMP_5_0_RTIME", COMP_5_0_RTIME),
        ("COMP_6_0_THRESHOLD", COMP_6_0_THRESHOLD),
        ("COMP_6_0_GAIN", COMP_6_0_GAIN),
        ("COMP_6_0_RATIO", COMP_6_0_RATIO),
        ("COMP_6_0_KNEE", COMP_6_0_KNEE),
        ("COMP_6_0_ATIME", COMP_6_0_ATIME),
        ("COMP_6_0_RTIME", COMP_6_0_RTIME),
        ("DELAY_3_0", DELAY_3_0),
        ("DELAY_4_0", DELAY_4_0),
        ("DELAY_5_0", DELAY_5_0),
        ("DELAY_6_0", DELAY_6_0),
        ("METER_02_C1_0", METER_02_C1_0),
        ("METER_02_C1_1", METER_02_C1_1),
        ("METER_10_C1_0", METER_10_C1_0),
        ("METER_10_C1_1", METER_10_C1_1),
        ("METER_10_C1_2", METER_10_C1_2),
        ("METER_10_C1_3", METER_10_C1_3),
        ("METER_10_C1_4", METER_10_C1_4),
        ("METER_10_C1_5", METER_10_C1_5),
        ("METER_10_C1_6", METER_10_C1_6),
        ("METER_10_C1_7", METER_10_C1_7),
        ("POLARITY_IN_1_0", POLARITY_IN_1_0),
        ("POLARITY_IN_2_0", POLARITY_IN_2_0),
        ("POLARITY_OUT_1_0", POLARITY_OUT_1_0),
        ("POLARITY_OUT_2_0", POLARITY_OUT_2_0),
        ("POLARITY_OUT_3_0", POLARITY_OUT_3_0),
        ("POLARITY_OUT_4_0", POLARITY_OUT_4_0),
        ("FIR_3_0_TAPS", FIR_3_0_TAPS),
        ("FIR_3_0", FIR_3_0),
        ("FIR_4_0_TAPS", FIR_4_0_TAPS),
        ("FIR_4_0", FIR_4_0),
        ("FIR_5_0_TAPS", FIR_5_0_TAPS),
        ("FIR_5_0", FIR_5_0),
        ("FIR_6_0_TAPS", FIR_6_0_TAPS),
        ("FIR_6_0", FIR_6_0),
        ("PEQ_1_1", PEQ_1_1),
        ("PEQ_1_2", PEQ_1_2),
        ("PEQ_1_3", PEQ_1_3),
        ("PEQ_1_4", PEQ_1_4),
        ("PEQ_1_5", PEQ_1_5),
        ("PEQ_1_6", PEQ_1_6),
        ("PEQ_1_7", PEQ_1_7),
        ("PEQ_1_8", PEQ_1_8),
        ("PEQ_1_9", PEQ_1_9),
        ("PEQ_1_10", PEQ_1_10),
        ("PEQ_2_1", PEQ_2_1),
        ("PEQ_2_2", PEQ_2_2),
        ("PEQ_2_3", PEQ_2_3),
        ("PEQ_2_4", PEQ_2_4),
        ("PEQ_2_5", PEQ_2_5),
        ("PEQ_2_6", PEQ_2_6),
        ("PEQ_2_7", PEQ_2_7),
        ("PEQ_2_8", PEQ_2_8),
        ("PEQ_2_9", PEQ_2_9),
        ("PEQ_2_10", PEQ_2_10),
        ("PEQ_3_1", PEQ_3_1),
        ("PEQ_3_2", PEQ_3_2),
        ("PEQ_3_3", PEQ_3_3),
        ("PEQ_3_4", PEQ_3_4),
        ("PEQ_3_5", PEQ_3_5),
        ("PEQ_3_6", PEQ_3_6),
        ("PEQ_3_7", PEQ_3_7),
        ("PEQ_3_8", PEQ_3_8),
        ("PEQ_3_9", PEQ_3_9),
        ("PEQ_3_10", PEQ_3_10),
        ("PEQ_4_1", PEQ_4_1),
        ("PEQ_4_2", PEQ_4_2),
        ("PEQ_4_3", PEQ_4_3),
        ("PEQ_4_4", PEQ_4_4),
        ("PEQ_4_5", PEQ_4_5),
        ("PEQ_4_6", PEQ_4_6),
        ("PEQ_4_7", PEQ_4_7),
        ("PEQ_4_8", PEQ_4_8),
        ("PEQ_4_9", PEQ_4_9),
        ("PEQ_4_10", PEQ_4_10),
        ("PEQ_5_1", PEQ_5_1),
        ("PEQ_5_2", PEQ_5_2),
        ("PEQ_5_3", PEQ_5_3),
        ("PEQ_5_4", PEQ_5_4),
        ("PEQ_5_5", PEQ_5_5),
        ("PEQ_5_6", PEQ_5_6),
        ("PEQ_5_7", PEQ_5_7),
        ("PEQ_5_8", PEQ_5_8),
        ("PEQ_5_9", PEQ_5_9),
        ("PEQ_5_10", PEQ_5_10),
        ("PEQ_6_1", PEQ_6_1),
        ("PEQ_6_2", PEQ_6_2),
        ("PEQ_6_3", PEQ_6_3),
        ("PEQ_6_4", PEQ_6_4),
        ("PEQ_6_5", PEQ_6_5),
        ("PEQ_6_6", PEQ_6_6),
        ("PEQ_6_7", PEQ_6_7),
        ("PEQ_6_8", PEQ_6_8),
        ("PEQ_6_9", PEQ_6_9),
        ("PEQ_6_10", PEQ_6_10),
        ("BPF_3_1", BPF_3_1),
        ("BPF_3_5", BPF_3_5),
        ("BPF_4_1", BPF_4_1),
        ("BPF_4_5", BPF_4_5),
        ("BPF_5_1", BPF_5_1),
        ("BPF_5_5", BPF_5_5),
        ("BPF_6_1", BPF_6_1),
        ("BPF_6_5", BPF_6_5),
    ];
}
use sym::*;
pub const DEVICE: Device = Device {
    product_name: "2x4HD",
    sources: &[Analog, Toslink, Usb],
    inputs: &[
        Input {
            gate: Gate {
                enable: D_GAIN_1_0_STATUS,
                gain: D_GAIN_1_0,
            },
            meter: METER_02_C1_0,
            routing: &[
                Gate {
                    enable: MIXER_NX_M_SMOOTHED_1_0_0_STATUS,
                    gain: MIXER_NX_M_SMOOTHED_1_0_0,
                },
                Gate {
                    enable: MIXER_NX_M_SMOOTHED_1_0_1_STATUS,
                    gain: MIXER_NX_M_SMOOTHED_1_0_1,
                },
                Gate {
                    enable: MIXER_NX_M_SMOOTHED_1_0_2_STATUS,
                    gain: MIXER_NX_M_SMOOTHED_1_0_2,
                },
                Gate {
                    enable: MIXER_NX_M_SMOOTHED_1_0_3_STATUS,
                    gain: MIXER_NX_M_SMOOTHED_1_0_3,
                },
            ],
            peq: &[
                PEQ_1_10, PEQ_1_9, PEQ_1_8, PEQ_1_7, PEQ_1_6, PEQ_1_5, PEQ_1_4, PEQ_1_3, PEQ_1_2,
                PEQ_1_1,
            ],
        },
        Input {
            gate: Gate {
                enable: D_GAIN_2_0_STATUS,
                gain: D_GAIN_2_0,
            },
            meter: METER_02_C1_1,
            routing: &[
                Gate {
                    enable: MIXER_NX_M_SMOOTHED_1_1_0_STATUS,
                    gain: MIXER_NX_M_SMOOTHED_1_1_0,
                },
                Gate {
                    enable: MIXER_NX_M_SMOOTHED_1_1_1_STATUS,
                    gain: MIXER_NX_M_SMOOTHED_1_1_1,
                },
                Gate {
                    enable: MIXER_NX_M_SMOOTHED_1_1_2_STATUS,
                    gain: MIXER_NX_M_SMOOTHED_1_1_2,
                },
                Gate {
                    enable: MIXER_NX_M_SMOOTHED_1_1_3_STATUS,
                    gain: MIXER_NX_M_SMOOTHED_1_1_3,
                },
            ],
            peq: &[
                PEQ_2_10, PEQ_2_9, PEQ_2_8, PEQ_2_7, PEQ_2_6, PEQ_2_5, PEQ_2_4, PEQ_2_3, PEQ_2_2,
                PEQ_2_1,
            ],
        },
    ],
    outputs: &[
        Output {
            gate: Gate {
                enable: D_GAIN_3_0_STATUS,
                gain: D_GAIN_3_0,
            },
            meter: METER_10_C1_4,
            peq: &[
                PEQ_3_10, PEQ_3_9, PEQ_3_8, PEQ_3_7, PEQ_3_6, PEQ_3_5, PEQ_3_4, PEQ_3_3, PEQ_3_2,
                PEQ_3_1,
            ],
            delay_addr: DELAY_3_0,
            invert_addr: POLARITY_OUT_1_0,
            xover: Some(Crossover {
                peqs: &[BPF_3_1, BPF_3_5],
            }),
            compressor: Some(Compressor {
                bypass: COMP_3_0_STATUS,
                threshold: COMP_3_0_THRESHOLD,
                ratio: COMP_3_0_RATIO,
                attack: COMP_3_0_ATIME,
                release: COMP_3_0_RTIME,
                meter: METER_10_C1_0,
            }),
            fir: Some(Fir {
                index: 0,
                bypass: FIR_3_0_STATUS,
                num_coefficients: FIR_3_0_TAPS,
                max_coefficients: 4096,
            }),
        },
        Output {
            gate: Gate {
                enable: D_GAIN_4_0_STATUS,
                gain: D_GAIN_4_0,
            },
            meter: METER_10_C1_5,
            peq: &[
                PEQ_4_10, PEQ_4_9, PEQ_4_8, PEQ_4_7, PEQ_4_6, PEQ_4_5, PEQ_4_4, PEQ_4_3, PEQ_4_2,
                PEQ_4_1,
            ],
            delay_addr: DELAY_4_0,
            invert_addr: POLARITY_OUT_2_0,
            xover: Some(Crossover {
                peqs: &[BPF_4_1, BPF_4_5],
            }),
            compressor: Some(Compressor {
                bypass: COMP_4_0_STATUS,
                threshold: COMP_4_0_THRESHOLD,
                ratio: COMP_4_0_RATIO,
                attack: COMP_4_0_ATIME,
                release: COMP_4_0_RTIME,
                meter: METER_10_C1_1,
            }),
            fir: Some(Fir {
                index: 1,
                bypass: FIR_4_0_STATUS,
                num_coefficients: FIR_4_0_TAPS,
                max_coefficients: 4096,
            }),
        },
        Output {
            gate: Gate {
                enable: D_GAIN_5_0_STATUS,
                gain: D_GAIN_5_0,
            },
            meter: METER_10_C1_6,
            peq: &[
                PEQ_5_10, PEQ_5_9, PEQ_5_8, PEQ_5_7, PEQ_5_6, PEQ_5_5, PEQ_5_4, PEQ_5_3, PEQ_5_2,
                PEQ_5_1,
            ],
            delay_addr: DELAY_5_0,
            invert_addr: POLARITY_OUT_3_0,
            xover: Some(Crossover {
                peqs: &[BPF_5_1, BPF_5_5],
            }),
            compressor: Some(Compressor {
                bypass: COMP_5_0_STATUS,
                threshold: COMP_5_0_THRESHOLD,
                ratio: COMP_5_0_RATIO,
                attack: COMP_5_0_ATIME,
                release: COMP_5_0_RTIME,
                meter: METER_10_C1_2,
            }),
            fir: Some(Fir {
                index: 2,
                bypass: FIR_5_0_STATUS,
                num_coefficients: FIR_5_0_TAPS,
                max_coefficients: 4096,
            }),
        },
        Output {
            gate: Gate {
                enable: D_GAIN_6_0_STATUS,
                gain: D_GAIN_6_0,
            },
            meter: METER_10_C1_7,
            peq: &[
                PEQ_6_10, PEQ_6_9, PEQ_6_8, PEQ_6_7, PEQ_6_6, PEQ_6_5, PEQ_6_4, PEQ_6_3, PEQ_6_2,
                PEQ_6_1,
            ],
            delay_addr: DELAY_6_0,
            invert_addr: POLARITY_OUT_4_0,
            xover: Some(Crossover {
                peqs: &[BPF_6_1, BPF_6_5],
            }),
            compressor: Some(Compressor {
                bypass: COMP_6_0_STATUS,
                threshold: COMP_6_0_THRESHOLD,
                ratio: COMP_6_0_RATIO,
                attack: COMP_6_0_ATIME,
                release: COMP_6_0_RTIME,
                meter: METER_10_C1_3,
            }),
            fir: Some(Fir {
                index: 3,
                bypass: FIR_6_0_STATUS,
                num_coefficients: FIR_6_0_TAPS,
                max_coefficients: 4096,
            }),
        },
    ],
    fir_max_taps: 4096,
    internal_sampling_rate: 96000,
};
