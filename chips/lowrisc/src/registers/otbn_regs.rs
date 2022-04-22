// Generated register constants for otbn

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
pub const OTBN_PARAM_NUM_ALERTS: u32 = 2;
// Register width
pub const OTBN_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub OtbnRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Command Register
        (0x0010 => pub(crate) cmd: ReadWrite<u32, CMD::Register>),
        // Control Register
        (0x0014 => pub(crate) ctrl: ReadWrite<u32, CTRL::Register>),
        // Status Register
        (0x0018 => pub(crate) status: ReadWrite<u32, STATUS::Register>),
        // Operation Result Register
        (0x001c => pub(crate) err_bits: ReadWrite<u32, ERR_BITS::Register>),
        // Fatal Alert Cause Register
        (0x0020 => pub(crate) fatal_alert_cause: ReadWrite<u32, FATAL_ALERT_CAUSE::Register>),
        // Instruction Count Register
        (0x0024 => pub(crate) insn_cnt: ReadWrite<u32, INSN_CNT::Register>),
        // A 32-bit CRC checksum of data written to memory
        (0x0028 => pub(crate) load_checksum: ReadWrite<u32, LOAD_CHECKSUM::Register>),
        // Memory area: Instruction Memory Access
        (0x4000 => pub(crate) imem: [ReadWrite<u32>; 1024]),
        // Memory area: Data Memory Access
        (0x8000 => pub(crate) dmem: [ReadWrite<u32>; 512]),
        (0x8800 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) DONE OFFSET(0) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) FATAL OFFSET(0) NUMBITS(1) [],
        pub(crate) RECOV OFFSET(1) NUMBITS(1) [],
    ],
    CMD [
        pub(crate) CMD OFFSET(0) NUMBITS(8) [],
    ],
    CTRL [
        pub(crate) SOFTWARE_ERRS_FATAL OFFSET(0) NUMBITS(1) [],
    ],
    STATUS [
        pub(crate) STATUS OFFSET(0) NUMBITS(8) [],
    ],
    ERR_BITS [
        pub(crate) BAD_DATA_ADDR OFFSET(0) NUMBITS(1) [],
        pub(crate) BAD_INSN_ADDR OFFSET(1) NUMBITS(1) [],
        pub(crate) CALL_STACK OFFSET(2) NUMBITS(1) [],
        pub(crate) ILLEGAL_INSN OFFSET(3) NUMBITS(1) [],
        pub(crate) LOOP OFFSET(4) NUMBITS(1) [],
        pub(crate) KEY_INVALID OFFSET(5) NUMBITS(1) [],
        pub(crate) IMEM_INTG_VIOLATION OFFSET(16) NUMBITS(1) [],
        pub(crate) DMEM_INTG_VIOLATION OFFSET(17) NUMBITS(1) [],
        pub(crate) REG_INTG_VIOLATION OFFSET(18) NUMBITS(1) [],
        pub(crate) BUS_INTG_VIOLATION OFFSET(19) NUMBITS(1) [],
        pub(crate) BAD_INTERNAL_STATE OFFSET(20) NUMBITS(1) [],
        pub(crate) ILLEGAL_BUS_ACCESS OFFSET(21) NUMBITS(1) [],
        pub(crate) LIFECYCLE_ESCALATION OFFSET(22) NUMBITS(1) [],
        pub(crate) FATAL_SOFTWARE OFFSET(23) NUMBITS(1) [],
    ],
    FATAL_ALERT_CAUSE [
        pub(crate) IMEM_INTG_VIOLATION OFFSET(0) NUMBITS(1) [],
        pub(crate) DMEM_INTG_VIOLATION OFFSET(1) NUMBITS(1) [],
        pub(crate) REG_INTG_VIOLATION OFFSET(2) NUMBITS(1) [],
        pub(crate) BUS_INTG_VIOLATION OFFSET(3) NUMBITS(1) [],
        pub(crate) BAD_INTERNAL_STATE OFFSET(4) NUMBITS(1) [],
        pub(crate) ILLEGAL_BUS_ACCESS OFFSET(5) NUMBITS(1) [],
        pub(crate) LIFECYCLE_ESCALATION OFFSET(6) NUMBITS(1) [],
        pub(crate) FATAL_SOFTWARE OFFSET(7) NUMBITS(1) [],
    ],
    INSN_CNT [],
    LOAD_CHECKSUM [],
];

// End generated register constants for otbn