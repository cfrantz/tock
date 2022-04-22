// Generated register constants for gpio

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
pub const GPIO_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const GPIO_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub GpioRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // GPIO Input data read value
        (0x0010 => pub(crate) data_in: ReadWrite<u32, DATA_IN::Register>),
        // GPIO direct output data write value
        (0x0014 => pub(crate) direct_out: ReadWrite<u32, DIRECT_OUT::Register>),
        // GPIO write data lower with mask.
        (0x0018 => pub(crate) masked_out_lower: ReadWrite<u32, MASKED_OUT_LOWER::Register>),
        // GPIO write data upper with mask.
        (0x001c => pub(crate) masked_out_upper: ReadWrite<u32, MASKED_OUT_UPPER::Register>),
        // GPIO Output Enable.
        (0x0020 => pub(crate) direct_oe: ReadWrite<u32, DIRECT_OE::Register>),
        // GPIO write Output Enable lower with mask.
        (0x0024 => pub(crate) masked_oe_lower: ReadWrite<u32, MASKED_OE_LOWER::Register>),
        // GPIO write Output Enable upper with mask.
        (0x0028 => pub(crate) masked_oe_upper: ReadWrite<u32, MASKED_OE_UPPER::Register>),
        // GPIO interrupt enable for GPIO, rising edge.
        (0x002c => pub(crate) intr_ctrl_en_rising: ReadWrite<u32, INTR::Register>),
        // GPIO interrupt enable for GPIO, falling edge.
        (0x0030 => pub(crate) intr_ctrl_en_falling: ReadWrite<u32, INTR::Register>),
        // GPIO interrupt enable for GPIO, level high.
        (0x0034 => pub(crate) intr_ctrl_en_lvlhigh: ReadWrite<u32, INTR::Register>),
        // GPIO interrupt enable for GPIO, level low.
        (0x0038 => pub(crate) intr_ctrl_en_lvllow: ReadWrite<u32, INTR::Register>),
        // filter enable for GPIO input bits.
        (0x003c => pub(crate) ctrl_en_input_filter: ReadWrite<u32, CTRL_EN_INPUT_FILTER::Register>),
        (0x0040 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [],
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    DATA_IN [],
    DIRECT_OUT [],
    MASKED_OUT_LOWER [
        pub(crate) DATA OFFSET(0) NUMBITS(16) [],
        pub(crate) MASK OFFSET(16) NUMBITS(16) [],
    ],
    MASKED_OUT_UPPER [
        pub(crate) DATA OFFSET(0) NUMBITS(16) [],
        pub(crate) MASK OFFSET(16) NUMBITS(16) [],
    ],
    DIRECT_OE [],
    MASKED_OE_LOWER [
        pub(crate) DATA OFFSET(0) NUMBITS(16) [],
        pub(crate) MASK OFFSET(16) NUMBITS(16) [],
    ],
    MASKED_OE_UPPER [
        pub(crate) DATA OFFSET(0) NUMBITS(16) [],
        pub(crate) MASK OFFSET(16) NUMBITS(16) [],
    ],
    CTRL_EN_INPUT_FILTER [],
];

// End generated register constants for gpio