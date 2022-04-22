// Generated register constants for pattgen

// Built for earlgrey_silver_release_v5-4985-g294f125d9
// https://github.com/lowRISC/opentitan/tree/294f125d9556533de1f92c171f5827c15b193962
// Tree status: modified
// Build date: 2022-04-22T18:29:14

// Copyright information found in source file:
// Copyright lowRISC contributors.

// Licensing information found in source file:
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use kernel::utilities::registers::{
    register_bitfields, register_structs, ReadWrite
};
// Number of data registers per each channel
pub const PATTGEN_PARAM_NUM_REGS_DATA: u32 = 2;
// Number of alerts
pub const PATTGEN_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const PATTGEN_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub PattgenRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // PATTGEN control register
        (0x0010 => pub(crate) ctrl: ReadWrite<u32, CTRL::Register>),
        // PATTGEN pre-divider register for Channel 0
        (0x0014 => pub(crate) prediv_ch0: ReadWrite<u32, PREDIV_CH0::Register>),
        // PATTGEN pre-divider register for Channel 1
        (0x0018 => pub(crate) prediv_ch1: ReadWrite<u32, PREDIV_CH1::Register>),
        // PATTGEN seed pattern multi-registers for Channel 0.
        (0x001c => pub(crate) data_ch0: [ReadWrite<u32, DATA_CH0::Register>; 2]),
        // PATTGEN seed pattern multi-registers for Channel 1.
        (0x0024 => pub(crate) data_ch1: [ReadWrite<u32, DATA_CH1::Register>; 2]),
        // PATTGEN pattern length
        (0x002c => pub(crate) size: ReadWrite<u32, SIZE::Register>),
        (0x0030 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) DONE_CH0 OFFSET(0) NUMBITS(1) [],
        pub(crate) DONE_CH1 OFFSET(1) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    CTRL [
        pub(crate) ENABLE_CH0 OFFSET(0) NUMBITS(1) [],
        pub(crate) ENABLE_CH1 OFFSET(1) NUMBITS(1) [],
        pub(crate) POLARITY_CH0 OFFSET(2) NUMBITS(1) [],
        pub(crate) POLARITY_CH1 OFFSET(3) NUMBITS(1) [],
    ],
    PREDIV_CH0 [],
    PREDIV_CH1 [],
    DATA_CH0 [],
    DATA_CH1 [],
    SIZE [
        pub(crate) LEN_CH0 OFFSET(0) NUMBITS(6) [],
        pub(crate) REPS_CH0 OFFSET(6) NUMBITS(10) [],
        pub(crate) LEN_CH1 OFFSET(16) NUMBITS(6) [],
        pub(crate) REPS_CH1 OFFSET(22) NUMBITS(10) [],
    ],
];

// End generated register constants for pattgen