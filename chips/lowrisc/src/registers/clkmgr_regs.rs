// Generated register constants for CLKMGR

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
// Number of clock groups
pub const CLKMGR_PARAM_NUM_GROUPS: u32 = 7;
// Register width
pub const CLKMGR_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub ClkmgrRegisters {
        // Clock enable for software gateable clocks.
        (0x0000 => pub(crate) clk_enables: ReadWrite<u32, CLK_ENABLES::Register>),
        // Clock hint for software gateable clocks.
        (0x0004 => pub(crate) clk_hints: ReadWrite<u32, CLK_HINTS::Register>),
        // Since the final state of !!CLK_HINTS is not always determined by software,
        (0x0008 => pub(crate) clk_hints_status: ReadWrite<u32, CLK_HINTS_STATUS::Register>),
        (0x000c => @END),
    }
}

register_bitfields![u32,
    CLK_ENABLES [
        pub(crate) CLK_FIXED_PERI_EN OFFSET(0) NUMBITS(1) [],
        pub(crate) CLK_USB_48MHZ_PERI_EN OFFSET(1) NUMBITS(1) [],
    ],
    CLK_HINTS [
        pub(crate) CLK_MAIN_AES_HINT OFFSET(0) NUMBITS(1) [],
        pub(crate) CLK_MAIN_HMAC_HINT OFFSET(1) NUMBITS(1) [],
    ],
    CLK_HINTS_STATUS [
        pub(crate) CLK_MAIN_AES_VAL OFFSET(0) NUMBITS(1) [],
        pub(crate) CLK_MAIN_HMAC_VAL OFFSET(1) NUMBITS(1) [],
    ],
];

// End generated register constants for CLKMGR