// Generated register constants for KEYMGR

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
// Number of Registers for SW inputs (Salt)
pub const KEYMGR_PARAM_NUM_SALT_REG: u32 = 8;
// Number of Registers for SW inputs (SW binding)
pub const KEYMGR_PARAM_NUM_SW_BINDING_REG: u32 = 8;
// Number of Registers for SW outputs
pub const KEYMGR_PARAM_NUM_OUT_REG: u32 = 8;
// Number of Registers for key version
pub const KEYMGR_PARAM_NUM_KEY_VERSION: u32 = 1;
// Number of alerts
pub const KEYMGR_PARAM_NUM_ALERTS: u32 = 2;
// Register width
pub const KEYMGR_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub KeymgrRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Key manager configuration enable
        (0x0010 => pub(crate) cfg_regwen: ReadWrite<u32, CFG_REGWEN::Register>),
        // Key manager operation start
        (0x0014 => pub(crate) start: ReadWrite<u32, START::Register>),
        // Key manager operation controls
        (0x0018 => pub(crate) control_shadowed: ReadWrite<u32, CONTROL_SHADOWED::Register>),
        // sideload key slots clear
        (0x001c => pub(crate) sideload_clear: ReadWrite<u32, SIDELOAD_CLEAR::Register>),
        // regwen for reseed interval
        (0x0020 => pub(crate) reseed_interval_regwen: ReadWrite<u32, RESEED_INTERVAL_REGWEN::Register>),
        // Reseed interval for key manager entropy reseed
        (0x0024 => pub(crate) reseed_interval_shadowed: ReadWrite<u32, RESEED_INTERVAL_SHADOWED::Register>),
        // Register write enable for SOFTWARE_BINDING
        (0x0028 => pub(crate) sw_binding_regwen: ReadWrite<u32, SW_BINDING_REGWEN::Register>),
        // Software binding input to sealing portion of the key manager.
        (0x002c => pub(crate) sealing_sw_binding: [ReadWrite<u32, SEALING_SW_BINDING::Register>; 8]),
        // Software binding input to the attestation portion of the key manager.
        (0x004c => pub(crate) attest_sw_binding: [ReadWrite<u32, ATTEST_SW_BINDING::Register>; 8]),
        // Salt value used as part of output generation
        (0x006c => pub(crate) salt: [ReadWrite<u32, SALT::Register>; 8]),
        // Version used as part of output generation
        (0x008c => pub(crate) key_versi: [ReadWrite<u32, KEY_VERSI::Register>; 1]),
        // Register write enable for MAX_CREATOR_KEY_VERSION
        (0x0090 => pub(crate) max_creator_key_ver_regwen: ReadWrite<u32, MAX_CREATOR_KEY_VER_REGWEN::Register>),
        // Max creator key version
        (0x0094 => pub(crate) max_creator_key_ver_shadowed: ReadWrite<u32, MAX_CREATOR_KEY_VER_SHADOWED::Register>),
        // Register write enable for MAX_OWNER_INT_KEY_VERSION
        (0x0098 => pub(crate) max_owner_int_key_ver_regwen: ReadWrite<u32, MAX_OWNER_INT_KEY_VER_REGWEN::Register>),
        // Max owner intermediate key version
        (0x009c => pub(crate) max_owner_int_key_ver_shadowed: ReadWrite<u32, MAX_OWNER_INT_KEY_VER_SHADOWED::Register>),
        // Register write enable for MAX_OWNER_KEY_VERSION
        (0x00a0 => pub(crate) max_owner_key_ver_regwen: ReadWrite<u32, MAX_OWNER_KEY_VER_REGWEN::Register>),
        // Max owner key version
        (0x00a4 => pub(crate) max_owner_key_ver_shadowed: ReadWrite<u32, MAX_OWNER_KEY_VER_SHADOWED::Register>),
        // Key manager software output.
        (0x00a8 => pub(crate) sw_share0_output: [ReadWrite<u32, SW_SHARE0_OUTPUT::Register>; 8]),
        // Key manager software output.
        (0x00c8 => pub(crate) sw_share1_output: [ReadWrite<u32, SW_SHARE1_OUTPUT::Register>; 8]),
        // Key manager working state.
        (0x00e8 => pub(crate) working_state: ReadWrite<u32, WORKING_STATE::Register>),
        // Key manager status.
        (0x00ec => pub(crate) op_status: ReadWrite<u32, OP_STATUS::Register>),
        // Key manager error code.
        (0x00f0 => pub(crate) err_code: ReadWrite<u32, ERR_CODE::Register>),
        // This register represents both synchronous and asynchronous fatal faults.
        (0x00f4 => pub(crate) fault_status: ReadWrite<u32, FAULT_STATUS::Register>),
        (0x00f8 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) OP_DONE OFFSET(0) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) RECOV_OPERATION_ERR OFFSET(0) NUMBITS(1) [],
        pub(crate) FATAL_FAULT_ERR OFFSET(1) NUMBITS(1) [],
    ],
    CFG_REGWEN [
        pub(crate) EN OFFSET(0) NUMBITS(1) [],
    ],
    START [
        pub(crate) EN OFFSET(0) NUMBITS(1) [
            VALID STATE = 1,
        ],
    ],
    CONTROL_SHADOWED [
        pub(crate) OPERATION OFFSET(4) NUMBITS(3) [
            ADVANCE = 0,
            GENERATE ID = 1,
            GENERATE SW OUTPUT = 2,
            GENERATE HW OUTPUT = 3,
            DISABLE = 4,
        ],
        pub(crate) CDI_SEL OFFSET(7) NUMBITS(1) [
            SEALING CDI = 0,
            ATTESTATION CDI = 1,
        ],
        pub(crate) DEST_SEL OFFSET(12) NUMBITS(2) [
            NONE = 0,
            AES = 1,
            KMAC = 2,
            OTBN = 3,
        ],
    ],
    SIDELOAD_CLEAR [
        pub(crate) VAL OFFSET(0) NUMBITS(3) [
            NONE = 0,
            AES = 1,
            KMAC = 2,
            OTBN = 3,
        ],
    ],
    RESEED_INTERVAL_REGWEN [
        pub(crate) EN OFFSET(0) NUMBITS(1) [],
    ],
    RESEED_INTERVAL_SHADOWED [
        pub(crate) VAL OFFSET(0) NUMBITS(16) [],
    ],
    SW_BINDING_REGWEN [
        pub(crate) EN OFFSET(0) NUMBITS(1) [],
    ],
    SEALING_SW_BINDING [],
    ATTEST_SW_BINDING [],
    SALT [],
    KEY_VERSI [],
    MAX_CREATOR_KEY_VER_REGWEN [
        pub(crate) EN OFFSET(0) NUMBITS(1) [],
    ],
    MAX_CREATOR_KEY_VER_SHADOWED [],
    MAX_OWNER_INT_KEY_VER_REGWEN [
        pub(crate) EN OFFSET(0) NUMBITS(1) [],
    ],
    MAX_OWNER_INT_KEY_VER_SHADOWED [],
    MAX_OWNER_KEY_VER_REGWEN [
        pub(crate) EN OFFSET(0) NUMBITS(1) [],
    ],
    MAX_OWNER_KEY_VER_SHADOWED [],
    SW_SHARE0_OUTPUT [],
    SW_SHARE1_OUTPUT [],
    WORKING_STATE [
        pub(crate) STATE OFFSET(0) NUMBITS(3) [
            RESET = 0,
            INIT = 1,
            CREATOR ROOT KEY = 2,
            OWNER INTERMEDIATE KEY = 3,
            OWNER KEY = 4,
            DISABLED = 5,
            INVALID = 6,
        ],
    ],
    OP_STATUS [
        pub(crate) STATUS OFFSET(0) NUMBITS(2) [
            IDLE = 0,
            WIP = 1,
            DONE_SUCCESS = 2,
            DONE_ERROR = 3,
        ],
    ],
    ERR_CODE [
        pub(crate) INVALID_OP OFFSET(0) NUMBITS(1) [],
        pub(crate) INVALID_KMAC_INPUT OFFSET(1) NUMBITS(1) [],
        pub(crate) INVALID_SHADOW_UPDATE OFFSET(2) NUMBITS(1) [],
    ],
    FAULT_STATUS [
        pub(crate) CMD OFFSET(0) NUMBITS(1) [],
        pub(crate) KMAC_FSM OFFSET(1) NUMBITS(1) [],
        pub(crate) KMAC_DONE OFFSET(2) NUMBITS(1) [],
        pub(crate) KMAC_OP OFFSET(3) NUMBITS(1) [],
        pub(crate) KMAC_OUT OFFSET(4) NUMBITS(1) [],
        pub(crate) REGFILE_INTG OFFSET(5) NUMBITS(1) [],
        pub(crate) SHADOW OFFSET(6) NUMBITS(1) [],
        pub(crate) CTRL_FSM_INTG OFFSET(7) NUMBITS(1) [],
        pub(crate) CTRL_FSM_CHK OFFSET(8) NUMBITS(1) [],
        pub(crate) CTRL_FSM_CNT OFFSET(9) NUMBITS(1) [],
        pub(crate) RESEED_CNT OFFSET(10) NUMBITS(1) [],
        pub(crate) SIDE_CTRL_FSM OFFSET(11) NUMBITS(1) [],
        pub(crate) SIDE_CTRL_SEL OFFSET(12) NUMBITS(1) [],
        pub(crate) KEY_ECC OFFSET(13) NUMBITS(1) [],
    ],
];

// End generated register constants for KEYMGR