// Generated register constants for RV_CORE_IBEX

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
// Number of software triggerable alerts
pub const RV_CORE_IBEX_PARAM_NUM_SW_ALERTS: u32 = 2;
// Number of translatable regions per ibex bus
pub const RV_CORE_IBEX_PARAM_NUM_REGIONS: u32 = 2;
// Number of scratch words maintained.
pub const RV_CORE_IBEX_PARAM_NUM_SCRATCH_WORDS: u32 = 8;
// Number of alerts
pub const RV_CORE_IBEX_PARAM_NUM_ALERTS: u32 = 4;
// Register width
pub const RV_CORE_IBEX_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub RvCoreIbexRegisters {
        // Alert Test Register
        (0x0000 => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Software recoverable error
        (0x0004 => pub(crate) sw_recov_err: ReadWrite<u32, SW_RECOV_ERR::Register>),
        // Software fatal error
        (0x0008 => pub(crate) sw_fatal_err: ReadWrite<u32, SW_FATAL_ERR::Register>),
        // Ibus address control regwen.
        (0x000c => pub(crate) ibus_regwen: [ReadWrite<u32, IBUS_REGWEN::Register>; 2]),
        //   Enable Ibus address matching
        (0x0014 => pub(crate) ibus_addr_en: [ReadWrite<u32, IBUS_ADDR_EN::Register>; 2]),
        //   Matching region programming for ibus.
        (0x001c => pub(crate) ibus_addr_matching: [ReadWrite<u32, IBUS_ADDR_MATCHING::Register>; 2]),
        //   The remap address after a match has been made.
        (0x0024 => pub(crate) ibus_remap_addr: [ReadWrite<u32, IBUS_REMAP_ADDR::Register>; 2]),
        // Dbus address control regwen.
        (0x002c => pub(crate) dbus_regwen: [ReadWrite<u32, DBUS_REGWEN::Register>; 2]),
        //   Enable dbus address matching
        (0x0034 => pub(crate) dbus_addr_en: [ReadWrite<u32, DBUS_ADDR_EN::Register>; 2]),
        //   See !!IBUS_ADDR_MATCHING_0 for detailed description.
        (0x003c => pub(crate) dbus_addr_matching: [ReadWrite<u32, DBUS_ADDR_MATCHING::Register>; 2]),
        //   See !!IBUS_REMAP_ADDR_0 for a detailed description.
        (0x0044 => pub(crate) dbus_remap_addr: [ReadWrite<u32, DBUS_REMAP_ADDR::Register>; 2]),
        // Enable mask for NMI.
        (0x004c => pub(crate) nmi_enable: ReadWrite<u32, NMI_ENABLE::Register>),
        // Current NMI state
        (0x0050 => pub(crate) nmi_state: ReadWrite<u32, NMI_STATE::Register>),
        // error status
        (0x0054 => pub(crate) err_status: ReadWrite<u32, ERR_STATUS::Register>),
        // Random data from EDN
        (0x0058 => pub(crate) rnd_data: ReadWrite<u32, RND_DATA::Register>),
        // Status of random data in !!RND_DATA
        (0x005c => pub(crate) rnd_status: ReadWrite<u32, RND_STATUS::Register>),
        // FPGA build timestamp info.
        (0x0060 => pub(crate) fpga_info: ReadWrite<u32, FPGA_INFO::Register>),
        // Memory area: Exposed tlul window for DV only purposes.
        (0x0080 => pub(crate) dv_sim_window: [ReadWrite<u32>; 8]),
        (0x00a0 => @END),
    }
}

register_bitfields![u32,
    ALERT_TEST [
        pub(crate) FATAL_SW_ERR OFFSET(0) NUMBITS(1) [],
        pub(crate) RECOV_SW_ERR OFFSET(1) NUMBITS(1) [],
        pub(crate) FATAL_HW_ERR OFFSET(2) NUMBITS(1) [],
        pub(crate) RECOV_HW_ERR OFFSET(3) NUMBITS(1) [],
    ],
    SW_RECOV_ERR [
        pub(crate) VAL OFFSET(0) NUMBITS(4) [],
    ],
    SW_FATAL_ERR [
        pub(crate) VAL OFFSET(0) NUMBITS(4) [],
    ],
    IBUS_REGWEN [
        pub(crate) EN_0 OFFSET(0) NUMBITS(1) [
            LOCKED = 0,
            ENABLED = 1,
        ],
    ],
    IBUS_ADDR_EN [
        pub(crate) EN_0 OFFSET(0) NUMBITS(1) [],
    ],
    IBUS_ADDR_MATCHING [],
    IBUS_REMAP_ADDR [],
    DBUS_REGWEN [
        pub(crate) EN_0 OFFSET(0) NUMBITS(1) [
            LOCKED = 0,
            ENABLED = 1,
        ],
    ],
    DBUS_ADDR_EN [
        pub(crate) EN_0 OFFSET(0) NUMBITS(1) [],
    ],
    DBUS_ADDR_MATCHING [],
    DBUS_REMAP_ADDR [],
    NMI_ENABLE [
        pub(crate) ALERT_EN OFFSET(0) NUMBITS(1) [],
        pub(crate) WDOG_EN OFFSET(1) NUMBITS(1) [],
    ],
    NMI_STATE [
        pub(crate) ALERT OFFSET(0) NUMBITS(1) [],
        pub(crate) WDOG OFFSET(1) NUMBITS(1) [],
    ],
    ERR_STATUS [
        pub(crate) REG_INTG_ERR OFFSET(0) NUMBITS(1) [],
        pub(crate) FATAL_INTG_ERR OFFSET(8) NUMBITS(1) [],
        pub(crate) FATAL_CORE_ERR OFFSET(9) NUMBITS(1) [],
        pub(crate) RECOV_CORE_ERR OFFSET(10) NUMBITS(1) [],
    ],
    RND_DATA [],
    RND_STATUS [
        pub(crate) RND_DATA_VALID OFFSET(0) NUMBITS(1) [],
        pub(crate) RND_DATA_FIPS OFFSET(1) NUMBITS(1) [],
    ],
    FPGA_INFO [],
];

// End generated register constants for RV_CORE_IBEX