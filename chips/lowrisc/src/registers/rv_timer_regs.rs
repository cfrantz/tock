// Generated register constants for rv_timer

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
// Number of harts
pub const RV_TIMER_PARAM_N_HARTS: u32 = 1;
// Number of timers per Hart
pub const RV_TIMER_PARAM_N_TIMERS: u32 = 1;
// Number of alerts
pub const RV_TIMER_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const RV_TIMER_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub RvTimerRegisters {
        // Alert Test Register
        (0x0000 => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Control register
        (0x0004 => pub(crate) ct: [ReadWrite<u32, CT::Register>; 1]),
        // Configuration for Hart 0
        (0x0100 => pub(crate) cfg0: ReadWrite<u32, CFG0::Register>),
        // Timer value Lower
        (0x0104 => pub(crate) timer_v_lower0: ReadWrite<u32, TIMER_V_LOWER0::Register>),
        // Timer value Upper
        (0x0108 => pub(crate) timer_v_upper0: ReadWrite<u32, TIMER_V_UPPER0::Register>),
        // Timer value Lower
        (0x010c => pub(crate) compare_lower0_0: ReadWrite<u32, COMPARE_LOWER0_0::Register>),
        // Timer value Upper
        (0x0110 => pub(crate) compare_upper0_0: ReadWrite<u32, COMPARE_UPPER0_0::Register>),
        // Interrupt Enable
        (0x0114 => pub(crate) intr_enabl: [ReadWrite<u32, INTR_ENABL::Register>; 1]),
        // Interrupt Status
        (0x0118 => pub(crate) intr_stat: [ReadWrite<u32, INTR_STAT::Register>; 1]),
        // Interrupt test register
        (0x011c => pub(crate) intr_tes: [ReadWrite<u32, INTR_TES::Register>; 1]),
        (0x0120 => @END),
    }
}

register_bitfields![u32,
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    CT [
        pub(crate) ACTIVE_0 OFFSET(0) NUMBITS(1) [],
    ],
    CFG0 [
        pub(crate) PRESCALE OFFSET(0) NUMBITS(12) [],
        pub(crate) STEP OFFSET(16) NUMBITS(8) [],
    ],
    TIMER_V_LOWER0 [],
    TIMER_V_UPPER0 [],
    COMPARE_LOWER0_0 [],
    COMPARE_UPPER0_0 [],
    INTR_ENABL [
        pub(crate) IE_0 OFFSET(0) NUMBITS(1) [],
    ],
    INTR_STAT [
        pub(crate) IS_0 OFFSET(0) NUMBITS(1) [],
    ],
    INTR_TES [
        pub(crate) T_0 OFFSET(0) NUMBITS(1) [],
    ],
];

// End generated register constants for rv_timer