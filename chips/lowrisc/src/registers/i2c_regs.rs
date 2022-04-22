// Generated register constants for i2c

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
// Depth of FMT, RX, TX, and ACQ FIFOs
pub const I2C_PARAM_FIFO_DEPTH: u32 = 64;
// Number of alerts
pub const I2C_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const I2C_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub I2cRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // I2C control register (Functions TBD)
        (0x0010 => pub(crate) ctrl: ReadWrite<u32, CTRL::Register>),
        // I2C live status register
        (0x0014 => pub(crate) status: ReadWrite<u32, STATUS::Register>),
        // I2C read data
        (0x0018 => pub(crate) rdata: ReadWrite<u32, RDATA::Register>),
        // I2C Format Data
        (0x001c => pub(crate) fdata: ReadWrite<u32, FDATA::Register>),
        // I2C FIFO control register
        (0x0020 => pub(crate) fifo_ctrl: ReadWrite<u32, FIFO_CTRL::Register>),
        // I2C FIFO status register
        (0x0024 => pub(crate) fifo_status: ReadWrite<u32, FIFO_STATUS::Register>),
        // I2C override control register
        (0x0028 => pub(crate) ovrd: ReadWrite<u32, OVRD::Register>),
        // Oversampled RX values
        (0x002c => pub(crate) val: ReadWrite<u32, VAL::Register>),
        // Detailed I2C Timings (directly corresponding to table 10 in the I2C Specification).
        (0x0030 => pub(crate) timing0: ReadWrite<u32, TIMING0::Register>),
        // Detailed I2C Timings (directly corresponding to table 10 in the I2C Specification).
        (0x0034 => pub(crate) timing1: ReadWrite<u32, TIMING1::Register>),
        // Detailed I2C Timings (directly corresponding to table 10 in the I2C Specification).
        (0x0038 => pub(crate) timing2: ReadWrite<u32, TIMING2::Register>),
        // Detailed I2C Timings (directly corresponding to table 10, in the I2C Specification).
        (0x003c => pub(crate) timing3: ReadWrite<u32, TIMING3::Register>),
        // Detailed I2C Timings (directly corresponding to table 10, in the I2C Specification).
        (0x0040 => pub(crate) timing4: ReadWrite<u32, TIMING4::Register>),
        // I2C clock stretching timeout control
        (0x0044 => pub(crate) timeout_ctrl: ReadWrite<u32, TIMEOUT_CTRL::Register>),
        // I2C target address and mask pairs
        (0x0048 => pub(crate) target_id: ReadWrite<u32, TARGET_ID::Register>),
        // I2C target acquired data
        (0x004c => pub(crate) acqdata: ReadWrite<u32, ACQDATA::Register>),
        // I2C target transmit data
        (0x0050 => pub(crate) txdata: ReadWrite<u32, TXDATA::Register>),
        // I2C target clock stretching control
        (0x0054 => pub(crate) stretch_ctrl: ReadWrite<u32, STRETCH_CTRL::Register>),
        // I2C host clock generation timeout value (in units of input clock frequency)
        (0x0058 => pub(crate) host_timeout_ctrl: ReadWrite<u32, HOST_TIMEOUT_CTRL::Register>),
        (0x005c => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) FMT_WATERMARK OFFSET(0) NUMBITS(1) [],
        pub(crate) RX_WATERMARK OFFSET(1) NUMBITS(1) [],
        pub(crate) FMT_OVERFLOW OFFSET(2) NUMBITS(1) [],
        pub(crate) RX_OVERFLOW OFFSET(3) NUMBITS(1) [],
        pub(crate) NAK OFFSET(4) NUMBITS(1) [],
        pub(crate) SCL_INTERFERENCE OFFSET(5) NUMBITS(1) [],
        pub(crate) SDA_INTERFERENCE OFFSET(6) NUMBITS(1) [],
        pub(crate) STRETCH_TIMEOUT OFFSET(7) NUMBITS(1) [],
        pub(crate) SDA_UNSTABLE OFFSET(8) NUMBITS(1) [],
        pub(crate) TRANS_COMPLETE OFFSET(9) NUMBITS(1) [],
        pub(crate) TX_EMPTY OFFSET(10) NUMBITS(1) [],
        pub(crate) TX_NONEMPTY OFFSET(11) NUMBITS(1) [],
        pub(crate) TX_OVERFLOW OFFSET(12) NUMBITS(1) [],
        pub(crate) ACQ_OVERFLOW OFFSET(13) NUMBITS(1) [],
        pub(crate) ACK_STOP OFFSET(14) NUMBITS(1) [],
        pub(crate) HOST_TIMEOUT OFFSET(15) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    CTRL [
        pub(crate) ENABLEHOST OFFSET(0) NUMBITS(1) [],
        pub(crate) ENABLETARGET OFFSET(1) NUMBITS(1) [],
        pub(crate) LLPBK OFFSET(2) NUMBITS(1) [],
    ],
    STATUS [
        pub(crate) FMTFULL OFFSET(0) NUMBITS(1) [],
        pub(crate) RXFULL OFFSET(1) NUMBITS(1) [],
        pub(crate) FMTEMPTY OFFSET(2) NUMBITS(1) [],
        pub(crate) HOSTIDLE OFFSET(3) NUMBITS(1) [],
        pub(crate) TARGETIDLE OFFSET(4) NUMBITS(1) [],
        pub(crate) RXEMPTY OFFSET(5) NUMBITS(1) [],
        pub(crate) TXFULL OFFSET(6) NUMBITS(1) [],
        pub(crate) ACQFULL OFFSET(7) NUMBITS(1) [],
        pub(crate) TXEMPTY OFFSET(8) NUMBITS(1) [],
        pub(crate) ACQEMPTY OFFSET(9) NUMBITS(1) [],
    ],
    RDATA [
        pub(crate) RDATA OFFSET(0) NUMBITS(8) [],
    ],
    FDATA [
        pub(crate) FBYTE OFFSET(0) NUMBITS(8) [],
        pub(crate) START OFFSET(8) NUMBITS(1) [],
        pub(crate) STOP OFFSET(9) NUMBITS(1) [],
        pub(crate) READ OFFSET(10) NUMBITS(1) [],
        pub(crate) RCONT OFFSET(11) NUMBITS(1) [],
        pub(crate) NAKOK OFFSET(12) NUMBITS(1) [],
    ],
    FIFO_CTRL [
        pub(crate) RXRST OFFSET(0) NUMBITS(1) [],
        pub(crate) FMTRST OFFSET(1) NUMBITS(1) [],
        pub(crate) RXILVL OFFSET(2) NUMBITS(3) [
            RXLVL1 = 0,
            RXLVL4 = 1,
            RXLVL8 = 2,
            RXLVL16 = 3,
            RXLVL30 = 4,
        ],
        pub(crate) FMTILVL OFFSET(5) NUMBITS(2) [
            FMTLVL1 = 0,
            FMTLVL4 = 1,
            FMTLVL8 = 2,
            FMTLVL16 = 3,
        ],
        pub(crate) ACQRST OFFSET(7) NUMBITS(1) [],
        pub(crate) TXRST OFFSET(8) NUMBITS(1) [],
    ],
    FIFO_STATUS [
        pub(crate) FMTLVL OFFSET(0) NUMBITS(7) [],
        pub(crate) TXLVL OFFSET(8) NUMBITS(7) [],
        pub(crate) RXLVL OFFSET(16) NUMBITS(7) [],
        pub(crate) ACQLVL OFFSET(24) NUMBITS(7) [],
    ],
    OVRD [
        pub(crate) TXOVRDEN OFFSET(0) NUMBITS(1) [],
        pub(crate) SCLVAL OFFSET(1) NUMBITS(1) [],
        pub(crate) SDAVAL OFFSET(2) NUMBITS(1) [],
    ],
    VAL [
        pub(crate) SCL_RX OFFSET(0) NUMBITS(16) [],
        pub(crate) SDA_RX OFFSET(16) NUMBITS(16) [],
    ],
    TIMING0 [
        pub(crate) THIGH OFFSET(0) NUMBITS(16) [],
        pub(crate) TLOW OFFSET(16) NUMBITS(16) [],
    ],
    TIMING1 [
        pub(crate) T_R OFFSET(0) NUMBITS(16) [],
        pub(crate) T_F OFFSET(16) NUMBITS(16) [],
    ],
    TIMING2 [
        pub(crate) TSU_STA OFFSET(0) NUMBITS(16) [],
        pub(crate) THD_STA OFFSET(16) NUMBITS(16) [],
    ],
    TIMING3 [
        pub(crate) TSU_DAT OFFSET(0) NUMBITS(16) [],
        pub(crate) THD_DAT OFFSET(16) NUMBITS(16) [],
    ],
    TIMING4 [
        pub(crate) TSU_STO OFFSET(0) NUMBITS(16) [],
        pub(crate) T_BUF OFFSET(16) NUMBITS(16) [],
    ],
    TIMEOUT_CTRL [
        pub(crate) VAL OFFSET(0) NUMBITS(31) [],
        pub(crate) EN OFFSET(31) NUMBITS(1) [],
    ],
    TARGET_ID [
        pub(crate) ADDRESS0 OFFSET(0) NUMBITS(7) [],
        pub(crate) MASK0 OFFSET(7) NUMBITS(7) [],
        pub(crate) ADDRESS1 OFFSET(14) NUMBITS(7) [],
        pub(crate) MASK1 OFFSET(21) NUMBITS(7) [],
    ],
    ACQDATA [
        pub(crate) ABYTE OFFSET(0) NUMBITS(8) [],
        pub(crate) SIGNAL OFFSET(8) NUMBITS(2) [],
    ],
    TXDATA [
        pub(crate) TXDATA OFFSET(0) NUMBITS(8) [],
    ],
    STRETCH_CTRL [
        pub(crate) EN_ADDR_TX OFFSET(0) NUMBITS(1) [],
        pub(crate) EN_ADDR_ACQ OFFSET(1) NUMBITS(1) [],
        pub(crate) STOP_TX OFFSET(2) NUMBITS(1) [],
        pub(crate) STOP_ACQ OFFSET(3) NUMBITS(1) [],
    ],
    HOST_TIMEOUT_CTRL [],
];

// End generated register constants for i2c