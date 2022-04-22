// Generated register constants for aon_timer

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
pub const AON_TIMER_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const AON_TIMER_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub AonTimerRegisters {
        // Alert Test Register
        (0x0000 => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Wakeup Timer Control register
        (0x0004 => pub(crate) wkup_ctrl: ReadWrite<u32, WKUP_CTRL::Register>),
        // Wakeup Timer Threshold Register
        (0x0008 => pub(crate) wkup_thold: ReadWrite<u32, WKUP_THOLD::Register>),
        // Wakeup Timer Count Register
        (0x000c => pub(crate) wkup_count: ReadWrite<u32, WKUP_COUNT::Register>),
        // Watchdog Timer Write Enable Register
        (0x0010 => pub(crate) wdog_regwen: ReadWrite<u32, WDOG_REGWEN::Register>),
        // Watchdog Timer Control register
        (0x0014 => pub(crate) wdog_ctrl: ReadWrite<u32, WDOG_CTRL::Register>),
        // Watchdog Timer Bark Threshold Register
        (0x0018 => pub(crate) wdog_bark_thold: ReadWrite<u32, WDOG_BARK_THOLD::Register>),
        // Watchdog Timer Bite Threshold Register
        (0x001c => pub(crate) wdog_bite_thold: ReadWrite<u32, WDOG_BITE_THOLD::Register>),
        // Watchdog Timer Count Register
        (0x0020 => pub(crate) wdog_count: ReadWrite<u32, WDOG_COUNT::Register>),
        // Interrupt State Register
        (0x0024 => pub(crate) intr_state: ReadWrite<u32, INTR_STATE::Register>),
        // Interrupt Test Register
        (0x0028 => pub(crate) intr_test: ReadWrite<u32, INTR_TEST::Register>),
        // Wakeup request status
        (0x002c => pub(crate) wkup_cause: ReadWrite<u32, WKUP_CAUSE::Register>),
        (0x0030 => @END),
    }
}

register_bitfields![u32,
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    WKUP_CTRL [
        pub(crate) ENABLE OFFSET(0) NUMBITS(1) [],
        pub(crate) PRESCALER OFFSET(1) NUMBITS(12) [],
    ],
    WKUP_THOLD [],
    WKUP_COUNT [],
    WDOG_REGWEN [
        pub(crate) REGWEN OFFSET(0) NUMBITS(1) [],
    ],
    WDOG_CTRL [
        pub(crate) ENABLE OFFSET(0) NUMBITS(1) [],
        pub(crate) PAUSE_IN_SLEEP OFFSET(1) NUMBITS(1) [],
    ],
    WDOG_BARK_THOLD [],
    WDOG_BITE_THOLD [],
    WDOG_COUNT [],
    INTR_STATE [
        pub(crate) WKUP_TIMER_EXPIRED OFFSET(0) NUMBITS(1) [],
        pub(crate) WDOG_TIMER_BARK OFFSET(1) NUMBITS(1) [],
    ],
    INTR_TEST [
        pub(crate) WKUP_TIMER_EXPIRED OFFSET(0) NUMBITS(1) [],
        pub(crate) WDOG_TIMER_BARK OFFSET(1) NUMBITS(1) [],
    ],
    WKUP_CAUSE [
        pub(crate) CAUSE OFFSET(0) NUMBITS(1) [],
    ],
];

// End generated register constants for aon_timer