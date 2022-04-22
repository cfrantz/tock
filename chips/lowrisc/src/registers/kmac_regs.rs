// Generated register constants for kmac

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
    register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly
};
// Number of words for the secret key
pub const KMAC_PARAM_NUM_WORDS_KEY: u32 = 16;
// Number of words for Encoded NsPrefix.
pub const KMAC_PARAM_NUM_WORDS_PREFIX: u32 = 11;
// Number of alerts
pub const KMAC_PARAM_NUM_ALERTS: u32 = 2;
// Register width
pub const KMAC_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub KmacRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Controls the configurability of !!CFG register.
        (0x0010 => pub(crate) cfg_regwen: ReadWrite<u32, CFG_REGWEN::Register>),
        // KMAC Configuration register.
        (0x0014 => pub(crate) cfg_shadowed: ReadWrite<u32, CFG_SHADOWED::Register>),
        // KMAC/ SHA3 command register.
        (0x0018 => pub(crate) cmd: ReadWrite<u32, CMD::Register>),
        // KMAC/SHA3 Status register.
        (0x001c => pub(crate) status: ReadWrite<u32, STATUS::Register>),
        // Entropy Timer Periods.
        (0x0020 => pub(crate) entropy_period: ReadWrite<u32, ENTROPY_PERIOD::Register>),
        // Entropy Refresh Counter
        (0x0024 => pub(crate) entropy_refresh_hash_cnt: ReadWrite<u32, ENTROPY_REFRESH_HASH_CNT::Register>),
        // Entropy Refresh Threshold
        (0x0028 => pub(crate) entropy_refresh_threshold_shadowed: ReadWrite<u32, ENTROPY_REFRESH_THRESHOLD_SHADOWED::Register>),
        // Entropy Seed [31:0].
        (0x002c => pub(crate) entropy_seed_lower: ReadWrite<u32, ENTROPY_SEED_LOWER::Register>),
        // Entropy Seed [63:32].
        (0x0030 => pub(crate) entropy_seed_upper: ReadWrite<u32, ENTROPY_SEED_UPPER::Register>),
        // KMAC Secret Key
        (0x0034 => pub(crate) key_share0: [ReadWrite<u32, KEY_SHARE0::Register>; 16]),
        // KMAC Secret Key, 2nd share.
        (0x0074 => pub(crate) key_share1: [ReadWrite<u32, KEY_SHARE1::Register>; 16]),
        // Secret Key length in bit.
        (0x00b4 => pub(crate) key_len: ReadWrite<u32, KEY_LEN::Register>),
        // cSHAKE Prefix register.
        (0x00b8 => pub(crate) prefix: [ReadWrite<u32, PREFIX::Register>; 11]),
        // KMAC/SHA3 Error Code
        (0x00e4 => pub(crate) err_code: ReadWrite<u32, ERR_CODE::Register>),
        // Memory area: Keccak State (1600 bit) memory.
        (0x0400 => pub(crate) state: [ReadOnly<u32>; 128]),
        // Memory area: Message FIFO.
        (0x0800 => pub(crate) msg_fifo: [WriteOnly<u32>; 512]),
        (0x1000 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) KMAC_DONE OFFSET(0) NUMBITS(1) [],
        pub(crate) FIFO_EMPTY OFFSET(1) NUMBITS(1) [],
        pub(crate) KMAC_ERR OFFSET(2) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) RECOV_OPERATION_ERR OFFSET(0) NUMBITS(1) [],
        pub(crate) FATAL_FAULT_ERR OFFSET(1) NUMBITS(1) [],
    ],
    CFG_REGWEN [
        pub(crate) EN OFFSET(0) NUMBITS(1) [],
    ],
    CFG_SHADOWED [
        pub(crate) KMAC_EN OFFSET(0) NUMBITS(1) [],
        pub(crate) KSTRENGTH OFFSET(1) NUMBITS(3) [
            L128 = 0,
            L224 = 1,
            L256 = 2,
            L384 = 3,
            L512 = 4,
        ],
        pub(crate) MODE OFFSET(4) NUMBITS(2) [
            SHA3 = 0,
            SHAKE = 2,
            CSHAKE = 3,
        ],
        pub(crate) MSG_ENDIANNESS OFFSET(8) NUMBITS(1) [],
        pub(crate) STATE_ENDIANNESS OFFSET(9) NUMBITS(1) [],
        pub(crate) SIDELOAD OFFSET(12) NUMBITS(1) [],
        pub(crate) ENTROPY_MODE OFFSET(16) NUMBITS(2) [
            IDLE_MODE = 0,
            EDN_MODE = 1,
            SW_MODE = 2,
        ],
        pub(crate) ENTROPY_FAST_PROCESS OFFSET(19) NUMBITS(1) [],
        pub(crate) MSG_MASK OFFSET(20) NUMBITS(1) [],
        pub(crate) ENTROPY_READY OFFSET(24) NUMBITS(1) [],
        pub(crate) ERR_PROCESSED OFFSET(25) NUMBITS(1) [],
        pub(crate) EN_UNSUPPORTED_MODESTRENGTH OFFSET(26) NUMBITS(1) [],
    ],
    CMD [
        pub(crate) CMD OFFSET(0) NUMBITS(4) [
            START = 1,
            PROCESS = 2,
            RUN = 4,
            DONE = 8,
        ],
        pub(crate) ENTROPY_REQ OFFSET(8) NUMBITS(1) [],
        pub(crate) HASH_CNT_CLR OFFSET(9) NUMBITS(1) [],
    ],
    STATUS [
        pub(crate) SHA3_IDLE OFFSET(0) NUMBITS(1) [],
        pub(crate) SHA3_ABSORB OFFSET(1) NUMBITS(1) [],
        pub(crate) SHA3_SQUEEZE OFFSET(2) NUMBITS(1) [],
        pub(crate) FIFO_DEPTH OFFSET(8) NUMBITS(5) [],
        pub(crate) FIFO_EMPTY OFFSET(14) NUMBITS(1) [],
        pub(crate) FIFO_FULL OFFSET(15) NUMBITS(1) [],
        pub(crate) ALERT_FATAL_FAULT OFFSET(16) NUMBITS(1) [],
        pub(crate) ALERT_RECOV_CTRL_UPDATE_ERR OFFSET(17) NUMBITS(1) [],
    ],
    ENTROPY_PERIOD [
        pub(crate) PRESCALER OFFSET(0) NUMBITS(10) [],
        pub(crate) WAIT_TIMER OFFSET(16) NUMBITS(16) [],
    ],
    ENTROPY_REFRESH_HASH_CNT [
        pub(crate) HASH_CNT OFFSET(0) NUMBITS(10) [],
    ],
    ENTROPY_REFRESH_THRESHOLD_SHADOWED [
        pub(crate) THRESHOLD OFFSET(0) NUMBITS(10) [],
    ],
    ENTROPY_SEED_LOWER [],
    ENTROPY_SEED_UPPER [],
    KEY_SHARE0 [],
    KEY_SHARE1 [],
    KEY_LEN [
        pub(crate) LEN OFFSET(0) NUMBITS(3) [
            KEY128 = 0,
            KEY192 = 1,
            KEY256 = 2,
            KEY384 = 3,
            KEY512 = 4,
        ],
    ],
    PREFIX [],
    ERR_CODE [],
];

// End generated register constants for kmac