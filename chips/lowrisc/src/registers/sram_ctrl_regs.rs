// Generated register constants for sram_ctrl

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
// Number of alerts
pub const SRAM_CTRL_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const SRAM_CTRL_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub SramCtrlRegisters {
        // Alert Test Register
        (0x0000 => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // SRAM status register.
        (0x0004 => pub(crate) status: ReadWrite<u32, STATUS::Register>),
        // Lock register for execution enable register.
        (0x0008 => pub(crate) exec_regwen: ReadWrite<u32, EXEC_REGWEN::Register>),
        // Sram execution enable.
        (0x000c => pub(crate) exec: ReadWrite<u32, EXEC::Register>),
        // Lock register for control register.
        (0x0010 => pub(crate) ctrl_regwen: ReadWrite<u32, CTRL_REGWEN::Register>),
        // SRAM ctrl register.
        (0x0014 => pub(crate) ctrl: ReadWrite<u32, CTRL::Register>),
        (0x0018 => @END),
    }
}

register_bitfields![u32,
    ALERT_TEST [
        pub(crate) FATAL_ERROR OFFSET(0) NUMBITS(1) [],
    ],
    STATUS [
        pub(crate) BUS_INTEG_ERROR OFFSET(0) NUMBITS(1) [],
        pub(crate) INIT_ERROR OFFSET(1) NUMBITS(1) [],
        pub(crate) ESCALATED OFFSET(2) NUMBITS(1) [],
        pub(crate) SCR_KEY_VALID OFFSET(3) NUMBITS(1) [],
        pub(crate) SCR_KEY_SEED_VALID OFFSET(4) NUMBITS(1) [],
        pub(crate) INIT_DONE OFFSET(5) NUMBITS(1) [],
    ],
    EXEC_REGWEN [
        pub(crate) EXEC_REGWEN OFFSET(0) NUMBITS(1) [],
    ],
    EXEC [
        pub(crate) EN OFFSET(0) NUMBITS(4) [],
    ],
    CTRL_REGWEN [
        pub(crate) CTRL_REGWEN OFFSET(0) NUMBITS(1) [],
    ],
    CTRL [
        pub(crate) RENEW_SCR_KEY OFFSET(0) NUMBITS(1) [],
        pub(crate) INIT OFFSET(1) NUMBITS(1) [],
    ],
];

// End generated register constants for sram_ctrl