// Generated register constants for spi_host

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
// The number of active-low chip select (cs_n) lines to create.
pub const SPI_HOST_PARAM_NUM_C_S: u32 = 1;
// The size of the Tx FIFO (in words)
pub const SPI_HOST_PARAM_TX_DEPTH: u32 = 72;
// The size of the Rx FIFO (in words)
pub const SPI_HOST_PARAM_RX_DEPTH: u32 = 64;
// The size of the Cmd FIFO (one segment descriptor per entry)
pub const SPI_HOST_PARAM_CMD_DEPTH: u32 = 4;
// Number of alerts
pub const SPI_HOST_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const SPI_HOST_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub SpiHostRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // Control register
        (0x0010 => pub(crate) control: ReadWrite<u32, CONTROL::Register>),
        // Status register
        (0x0014 => pub(crate) status: ReadWrite<u32, STATUS::Register>),
        // Configuration options register.
        (0x0018 => pub(crate) configop: [ReadWrite<u32, CONFIGOP::Register>; 1]),
        // Chip-Select ID
        (0x001c => pub(crate) csid: ReadWrite<u32, CSID::Register>),
        // Command Register
        (0x0020 => pub(crate) command: ReadWrite<u32, COMMAND::Register>),
        // Memory area: SPI Receive Data.
        (0x0024 => pub(crate) rxdata: [ReadOnly<u32>; 1]),
        // Memory area: SPI Transmit Data.
        (0x0028 => pub(crate) txdata: [WriteOnly<u32>; 1]),
        // Controls which classes of errors raise an interrupt.
        (0x002c => pub(crate) error_enable: ReadWrite<u32, ERROR_ENABLE::Register>),
        // Indicates that any errors that have occurred.
        (0x0030 => pub(crate) error_status: ReadWrite<u32, ERROR_STATUS::Register>),
        // Controls which classes of SPI events raise an interrupt.
        (0x0034 => pub(crate) event_enable: ReadWrite<u32, EVENT_ENABLE::Register>),
        (0x0038 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) ERROR OFFSET(0) NUMBITS(1) [],
        pub(crate) SPI_EVENT OFFSET(1) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    CONTROL [
        pub(crate) RX_WATERMARK OFFSET(0) NUMBITS(8) [],
        pub(crate) TX_WATERMARK OFFSET(8) NUMBITS(8) [],
        pub(crate) OUTPUT_EN OFFSET(29) NUMBITS(1) [],
        pub(crate) SW_RST OFFSET(30) NUMBITS(1) [],
        pub(crate) SPIEN OFFSET(31) NUMBITS(1) [],
    ],
    STATUS [
        pub(crate) TXQD OFFSET(0) NUMBITS(8) [],
        pub(crate) RXQD OFFSET(8) NUMBITS(8) [],
        pub(crate) CMDQD OFFSET(16) NUMBITS(4) [],
        pub(crate) RXWM OFFSET(20) NUMBITS(1) [],
        pub(crate) BYTEORDER OFFSET(22) NUMBITS(1) [],
        pub(crate) RXSTALL OFFSET(23) NUMBITS(1) [],
        pub(crate) RXEMPTY OFFSET(24) NUMBITS(1) [],
        pub(crate) RXFULL OFFSET(25) NUMBITS(1) [],
        pub(crate) TXWM OFFSET(26) NUMBITS(1) [],
        pub(crate) TXSTALL OFFSET(27) NUMBITS(1) [],
        pub(crate) TXEMPTY OFFSET(28) NUMBITS(1) [],
        pub(crate) TXFULL OFFSET(29) NUMBITS(1) [],
        pub(crate) ACTIVE OFFSET(30) NUMBITS(1) [],
        pub(crate) READY OFFSET(31) NUMBITS(1) [],
    ],
    CONFIGOP [
        pub(crate) CLKDIV_0 OFFSET(0) NUMBITS(16) [],
        pub(crate) CSNIDLE_0 OFFSET(16) NUMBITS(4) [],
        pub(crate) CSNTRAIL_0 OFFSET(20) NUMBITS(4) [],
        pub(crate) CSNLEAD_0 OFFSET(24) NUMBITS(4) [],
        pub(crate) FULLCYC_0 OFFSET(29) NUMBITS(1) [],
        pub(crate) CPHA_0 OFFSET(30) NUMBITS(1) [],
        pub(crate) CPOL_0 OFFSET(31) NUMBITS(1) [],
    ],
    CSID [],
    COMMAND [
        pub(crate) LEN OFFSET(0) NUMBITS(9) [],
        pub(crate) CSAAT OFFSET(9) NUMBITS(1) [],
        pub(crate) SPEED OFFSET(10) NUMBITS(2) [],
        pub(crate) DIRECTION OFFSET(12) NUMBITS(2) [],
    ],
    ERROR_ENABLE [
        pub(crate) CMDBUSY OFFSET(0) NUMBITS(1) [],
        pub(crate) OVERFLOW OFFSET(1) NUMBITS(1) [],
        pub(crate) UNDERFLOW OFFSET(2) NUMBITS(1) [],
        pub(crate) CMDINVAL OFFSET(3) NUMBITS(1) [],
        pub(crate) CSIDINVAL OFFSET(4) NUMBITS(1) [],
    ],
    ERROR_STATUS [
        pub(crate) CMDBUSY OFFSET(0) NUMBITS(1) [],
        pub(crate) OVERFLOW OFFSET(1) NUMBITS(1) [],
        pub(crate) UNDERFLOW OFFSET(2) NUMBITS(1) [],
        pub(crate) CMDINVAL OFFSET(3) NUMBITS(1) [],
        pub(crate) CSIDINVAL OFFSET(4) NUMBITS(1) [],
        pub(crate) ACCESSINVAL OFFSET(5) NUMBITS(1) [],
    ],
    EVENT_ENABLE [
        pub(crate) RXFULL OFFSET(0) NUMBITS(1) [],
        pub(crate) TXEMPTY OFFSET(1) NUMBITS(1) [],
        pub(crate) RXWM OFFSET(2) NUMBITS(1) [],
        pub(crate) TXWM OFFSET(3) NUMBITS(1) [],
        pub(crate) READY OFFSET(4) NUMBITS(1) [],
        pub(crate) IDLE OFFSET(5) NUMBITS(1) [],
    ],
];

// End generated register constants for spi_host