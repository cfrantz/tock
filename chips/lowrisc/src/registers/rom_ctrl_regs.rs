// Generated register constants for rom_ctrl.
// This file is licensed under either of:
//   Apache License, Version 2.0 (LICENSE-APACHE <http://www.apache.org/licenses/LICENSE-2.0>)
//   MIT License (LICENSE-MIT <http://opensource.org/licenses/MIT>)

// Built for earlgrey_silver_release_v5-6422-g05dcfbd00
// https://github.com/lowRISC/opentitan/tree/05dcfbd00ca893dba034b468d1754f3f50780080
// Tree status: clean
// Build date: 2022-07-20T21:02:26

// Original reference file: hw/ip/rom_ctrl/data/rom_ctrl.hjson
use kernel::utilities::registers::ReadOnly;
use kernel::utilities::registers::ReadWrite;
use kernel::utilities::registers::{register_bitfields, register_structs};
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
    pub(crate) ALERT_TEST [
        FATAL OFFSET(0) NUMBITS(1) [],
    ],
    pub(crate) FATAL_ALERT_CAUSE [
        CHECKER_ERROR OFFSET(0) NUMBITS(1) [],
        INTEGRITY_ERROR OFFSET(1) NUMBITS(1) [],
    ],
    pub(crate) DIGEST [],
    pub(crate) EXP_DIGEST [],
];

// End generated register constants for rom_ctrl
