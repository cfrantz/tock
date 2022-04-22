// Generated register constants for rom_ctrl

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
    register_bitfields, register_structs, ReadOnly, ReadWrite
};
// Number of alerts
pub const ROM_CTRL_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const ROM_CTRL_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub RomCtrlRegisters {
        // Alert Test Register
        (0x0000 => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // The cause of a fatal alert.
        (0x0004 => pub(crate) fatal_alert_cause: ReadWrite<u32, FATAL_ALERT_CAUSE::Register>),
        // The digest computed from the contents of ROM
        (0x0008 => pub(crate) digest: [ReadWrite<u32, DIGEST::Register>; 8]),
        // The expected digest, stored in the top words of ROM
        (0x0028 => pub(crate) exp_digest: [ReadWrite<u32, EXP_DIGEST::Register>; 8]),
        // Memory area: ROM data
        (0x0000 => pub(crate) rom: [ReadOnly<u32>; 8192]),
        (0x8000 => @END),
    }
}

register_bitfields![u32,
    ALERT_TEST [
        pub(crate) FATAL OFFSET(0) NUMBITS(1) [],
    ],
    FATAL_ALERT_CAUSE [
        pub(crate) CHECKER_ERROR OFFSET(0) NUMBITS(1) [],
        pub(crate) INTEGRITY_ERROR OFFSET(1) NUMBITS(1) [],
    ],
    DIGEST [],
    EXP_DIGEST [],
];

// End generated register constants for rom_ctrl