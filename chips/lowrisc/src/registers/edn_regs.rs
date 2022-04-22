// Generated register constants for edn

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
pub const EDN_PARAM_NUM_ALERTS: u32 = 2;
// Register width
pub const EDN_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub EdnRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Register write enable for all control registers
        (0x0010 => pub(crate) regwen: ReadWrite<u32, REGWEN::Register>),
        // EDN control register
        (0x0014 => pub(crate) ctrl: ReadWrite<u32, CTRL::Register>),
        // EDN boot instantiate command register
        (0x0018 => pub(crate) boot_ins_cmd: ReadWrite<u32, BOOT_INS_CMD::Register>),
        // EDN boot generate command register
        (0x001c => pub(crate) boot_gen_cmd: ReadWrite<u32, BOOT_GEN_CMD::Register>),
        // EDN csrng app command request register
        (0x0020 => pub(crate) sw_cmd_req: ReadWrite<u32, SW_CMD_REQ::Register>),
        // EDN command status register
        (0x0024 => pub(crate) sw_cmd_sts: ReadWrite<u32, SW_CMD_STS::Register>),
        // EDN csrng reseed command register
        (0x0028 => pub(crate) reseed_cmd: ReadWrite<u32, RESEED_CMD::Register>),
        // EDN csrng generate command register
        (0x002c => pub(crate) generate_cmd: ReadWrite<u32, GENERATE_CMD::Register>),
        // EDN maximum number of requests between reseeds register
        (0x0030 => pub(crate) max_num_reqs_between_reseeds: ReadWrite<u32, MAX_NUM_REQS_BETWEEN_RESEEDS::Register>),
        // Recoverable alert status register
        (0x0034 => pub(crate) recov_alert_sts: ReadWrite<u32, RECOV_ALERT_STS::Register>),
        // Hardware detection of error conditions status register
        (0x0038 => pub(crate) err_code: ReadWrite<u32, ERR_CODE::Register>),
        // Test error conditions register
        (0x003c => pub(crate) err_code_test: ReadWrite<u32, ERR_CODE_TEST::Register>),
        // Main state machine state debug register
        (0x0040 => pub(crate) main_sm_state: ReadWrite<u32, MAIN_SM_STATE::Register>),
        (0x0044 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) EDN_CMD_REQ_DONE OFFSET(0) NUMBITS(1) [],
        pub(crate) EDN_FATAL_ERR OFFSET(1) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) RECOV_ALERT OFFSET(0) NUMBITS(1) [],
        pub(crate) FATAL_ALERT OFFSET(1) NUMBITS(1) [],
    ],
    REGWEN [
        pub(crate) REGWEN OFFSET(0) NUMBITS(1) [],
    ],
    CTRL [
        pub(crate) EDN_ENABLE OFFSET(0) NUMBITS(4) [],
        pub(crate) BOOT_REQ_MODE OFFSET(4) NUMBITS(4) [],
        pub(crate) AUTO_REQ_MODE OFFSET(8) NUMBITS(4) [],
        pub(crate) CMD_FIFO_RST OFFSET(12) NUMBITS(4) [],
    ],
    BOOT_INS_CMD [],
    BOOT_GEN_CMD [],
    SW_CMD_REQ [],
    SW_CMD_STS [
        pub(crate) CMD_RDY OFFSET(0) NUMBITS(1) [],
        pub(crate) CMD_STS OFFSET(1) NUMBITS(1) [],
    ],
    RESEED_CMD [],
    GENERATE_CMD [],
    MAX_NUM_REQS_BETWEEN_RESEEDS [],
    RECOV_ALERT_STS [
        pub(crate) EDN_ENABLE_FIELD_ALERT OFFSET(0) NUMBITS(1) [],
        pub(crate) BOOT_REQ_MODE_FIELD_ALERT OFFSET(1) NUMBITS(1) [],
        pub(crate) AUTO_REQ_MODE_FIELD_ALERT OFFSET(2) NUMBITS(1) [],
        pub(crate) CMD_FIFO_RST_FIELD_ALERT OFFSET(3) NUMBITS(1) [],
        pub(crate) EDN_BUS_CMP_ALERT OFFSET(12) NUMBITS(1) [],
    ],
    ERR_CODE [
        pub(crate) SFIFO_RESCMD_ERR OFFSET(0) NUMBITS(1) [],
        pub(crate) SFIFO_GENCMD_ERR OFFSET(1) NUMBITS(1) [],
        pub(crate) EDN_ACK_SM_ERR OFFSET(20) NUMBITS(1) [],
        pub(crate) EDN_MAIN_SM_ERR OFFSET(21) NUMBITS(1) [],
        pub(crate) EDN_CNTR_ERR OFFSET(22) NUMBITS(1) [],
        pub(crate) FIFO_WRITE_ERR OFFSET(28) NUMBITS(1) [],
        pub(crate) FIFO_READ_ERR OFFSET(29) NUMBITS(1) [],
        pub(crate) FIFO_STATE_ERR OFFSET(30) NUMBITS(1) [],
    ],
    ERR_CODE_TEST [
        pub(crate) ERR_CODE_TEST OFFSET(0) NUMBITS(5) [],
    ],
    MAIN_SM_STATE [
        pub(crate) MAIN_SM_STATE OFFSET(0) NUMBITS(9) [],
    ],
];

// End generated register constants for edn