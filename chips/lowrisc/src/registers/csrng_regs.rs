// Generated register constants for csrng

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
pub const CSRNG_PARAM_NUM_ALERTS: u32 = 2;
// Register width
pub const CSRNG_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub CsrngRegisters {
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
        // Control register
        (0x0014 => pub(crate) ctrl: ReadWrite<u32, CTRL::Register>),
        // Command request register
        (0x0018 => pub(crate) cmd_req: ReadWrite<u32, CMD_REQ::Register>),
        // Application interface command status register
        (0x001c => pub(crate) sw_cmd_sts: ReadWrite<u32, SW_CMD_STS::Register>),
        // Generate bits returned valid register
        (0x0020 => pub(crate) genbits_vld: ReadWrite<u32, GENBITS_VLD::Register>),
        // Generate bits returned register
        (0x0024 => pub(crate) genbits: ReadWrite<u32, GENBITS::Register>),
        // Internal state number register
        (0x0028 => pub(crate) int_state_num: ReadWrite<u32, INT_STATE_NUM::Register>),
        // Internal state read access register
        (0x002c => pub(crate) int_state_val: ReadWrite<u32, INT_STATE_VAL::Register>),
        // Hardware instance exception status register
        (0x0030 => pub(crate) hw_exc_sts: ReadWrite<u32, HW_EXC_STS::Register>),
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
        pub(crate) CS_CMD_REQ_DONE OFFSET(0) NUMBITS(1) [],
        pub(crate) CS_ENTROPY_REQ OFFSET(1) NUMBITS(1) [],
        pub(crate) CS_HW_INST_EXC OFFSET(2) NUMBITS(1) [],
        pub(crate) CS_FATAL_ERR OFFSET(3) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) RECOV_ALERT OFFSET(0) NUMBITS(1) [],
        pub(crate) FATAL_ALERT OFFSET(1) NUMBITS(1) [],
    ],
    REGWEN [
        pub(crate) REGWEN OFFSET(0) NUMBITS(1) [],
    ],
    CTRL [
        pub(crate) ENABLE OFFSET(0) NUMBITS(4) [],
        pub(crate) SW_APP_ENABLE OFFSET(4) NUMBITS(4) [],
        pub(crate) READ_INT_STATE OFFSET(8) NUMBITS(4) [],
    ],
    CMD_REQ [],
    SW_CMD_STS [
        pub(crate) CMD_RDY OFFSET(0) NUMBITS(1) [],
        pub(crate) CMD_STS OFFSET(1) NUMBITS(1) [],
    ],
    GENBITS_VLD [
        pub(crate) GENBITS_VLD OFFSET(0) NUMBITS(1) [],
        pub(crate) GENBITS_FIPS OFFSET(1) NUMBITS(1) [],
    ],
    GENBITS [],
    INT_STATE_NUM [
        pub(crate) INT_STATE_NUM OFFSET(0) NUMBITS(4) [],
    ],
    INT_STATE_VAL [],
    HW_EXC_STS [
        pub(crate) HW_EXC_STS OFFSET(0) NUMBITS(15) [],
    ],
    RECOV_ALERT_STS [
        pub(crate) ENABLE_FIELD_ALERT OFFSET(0) NUMBITS(1) [],
        pub(crate) SW_APP_ENABLE_FIELD_ALERT OFFSET(1) NUMBITS(1) [],
        pub(crate) READ_INT_STATE_FIELD_ALERT OFFSET(2) NUMBITS(1) [],
        pub(crate) CS_BUS_CMP_ALERT OFFSET(12) NUMBITS(1) [],
    ],
    ERR_CODE [
        pub(crate) SFIFO_CMD_ERR OFFSET(0) NUMBITS(1) [],
        pub(crate) SFIFO_GENBITS_ERR OFFSET(1) NUMBITS(1) [],
        pub(crate) SFIFO_CMDREQ_ERR OFFSET(2) NUMBITS(1) [],
        pub(crate) SFIFO_RCSTAGE_ERR OFFSET(3) NUMBITS(1) [],
        pub(crate) SFIFO_KEYVRC_ERR OFFSET(4) NUMBITS(1) [],
        pub(crate) SFIFO_UPDREQ_ERR OFFSET(5) NUMBITS(1) [],
        pub(crate) SFIFO_BENCREQ_ERR OFFSET(6) NUMBITS(1) [],
        pub(crate) SFIFO_BENCACK_ERR OFFSET(7) NUMBITS(1) [],
        pub(crate) SFIFO_PDATA_ERR OFFSET(8) NUMBITS(1) [],
        pub(crate) SFIFO_FINAL_ERR OFFSET(9) NUMBITS(1) [],
        pub(crate) SFIFO_GBENCACK_ERR OFFSET(10) NUMBITS(1) [],
        pub(crate) SFIFO_GRCSTAGE_ERR OFFSET(11) NUMBITS(1) [],
        pub(crate) SFIFO_GGENREQ_ERR OFFSET(12) NUMBITS(1) [],
        pub(crate) SFIFO_GADSTAGE_ERR OFFSET(13) NUMBITS(1) [],
        pub(crate) SFIFO_GGENBITS_ERR OFFSET(14) NUMBITS(1) [],
        pub(crate) SFIFO_BLKENC_ERR OFFSET(15) NUMBITS(1) [],
        pub(crate) CMD_STAGE_SM_ERR OFFSET(20) NUMBITS(1) [],
        pub(crate) MAIN_SM_ERR OFFSET(21) NUMBITS(1) [],
        pub(crate) DRBG_GEN_SM_ERR OFFSET(22) NUMBITS(1) [],
        pub(crate) DRBG_UPDBE_SM_ERR OFFSET(23) NUMBITS(1) [],
        pub(crate) DRBG_UPDOB_SM_ERR OFFSET(24) NUMBITS(1) [],
        pub(crate) AES_CIPHER_SM_ERR OFFSET(25) NUMBITS(1) [],
        pub(crate) CMD_GEN_CNT_ERR OFFSET(26) NUMBITS(1) [],
        pub(crate) FIFO_WRITE_ERR OFFSET(28) NUMBITS(1) [],
        pub(crate) FIFO_READ_ERR OFFSET(29) NUMBITS(1) [],
        pub(crate) FIFO_STATE_ERR OFFSET(30) NUMBITS(1) [],
    ],
    ERR_CODE_TEST [
        pub(crate) ERR_CODE_TEST OFFSET(0) NUMBITS(5) [],
    ],
    MAIN_SM_STATE [
        pub(crate) MAIN_SM_STATE OFFSET(0) NUMBITS(8) [],
    ],
];

// End generated register constants for csrng