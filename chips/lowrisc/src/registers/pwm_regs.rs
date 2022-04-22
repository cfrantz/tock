// Generated register constants for pwm

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
// Number of PWM outputs
pub const PWM_PARAM_N_OUTPUTS: u32 = 6;
// Number of alerts
pub const PWM_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const PWM_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub PwmRegisters {
        // Alert Test Register
        (0x0000 => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Register write enable for all control registers
        (0x0004 => pub(crate) regwen: ReadWrite<u32, REGWEN::Register>),
        // Configuration register
        (0x0008 => pub(crate) cfg: ReadWrite<u32, CFG::Register>),
        // Enable PWM operation for each channel
        (0x000c => pub(crate) pwm_: [ReadWrite<u32, PWM_::Register>; 1]),
        // Invert the PWM output for each channel
        (0x0010 => pub(crate) inve: [ReadWrite<u32, INVE::Register>; 1]),
        // Basic PWM Channel Parameters
        (0x0014 => pub(crate) pwm_param: [ReadWrite<u32, PWM_PARAM::Register>; 6]),
        // Controls the duty_cycle of each channel.
        (0x002c => pub(crate) duty_cycle: [ReadWrite<u32, DUTY_CYCLE::Register>; 6]),
        // Hardware controlled blink/heartbeat parameters.
        (0x0044 => pub(crate) blink_param: [ReadWrite<u32, BLINK_PARAM::Register>; 6]),
        (0x005c => @END),
    }
}

register_bitfields![u32,
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    REGWEN [
        pub(crate) REGWEN OFFSET(0) NUMBITS(1) [],
    ],
    CFG [
        pub(crate) CLK_DIV OFFSET(0) NUMBITS(27) [],
        pub(crate) DC_RESN OFFSET(27) NUMBITS(4) [],
        pub(crate) CNTR_EN OFFSET(31) NUMBITS(1) [],
    ],
    PWM_ [
        pub(crate) EN_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) EN_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) EN_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) EN_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) EN_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) EN_5 OFFSET(5) NUMBITS(1) [],
    ],
    INVE [
        pub(crate) INVERT_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) INVERT_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) INVERT_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) INVERT_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) INVERT_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) INVERT_5 OFFSET(5) NUMBITS(1) [],
    ],
    PWM_PARAM [
        pub(crate) PHASE_DELAY_0 OFFSET(0) NUMBITS(16) [],
        pub(crate) HTBT_EN_0 OFFSET(30) NUMBITS(1) [],
        pub(crate) BLINK_EN_0 OFFSET(31) NUMBITS(1) [],
    ],
    DUTY_CYCLE [
        pub(crate) A_0 OFFSET(0) NUMBITS(16) [],
        pub(crate) B_0 OFFSET(16) NUMBITS(16) [],
    ],
    BLINK_PARAM [
        pub(crate) X_0 OFFSET(0) NUMBITS(16) [],
        pub(crate) Y_0 OFFSET(16) NUMBITS(16) [],
    ],
];

// End generated register constants for pwm