// Generated register constants for uart

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
pub const UART_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const UART_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub UartRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // UART control register
        (0x0010 => pub(crate) ctrl: ReadWrite<u32, CTRL::Register>),
        // UART live status register
        (0x0014 => pub(crate) status: ReadWrite<u32, STATUS::Register>),
        // UART read data
        (0x0018 => pub(crate) rdata: ReadWrite<u32, RDATA::Register>),
        // UART write data
        (0x001c => pub(crate) wdata: ReadWrite<u32, WDATA::Register>),
        // UART FIFO control register
        (0x0020 => pub(crate) fifo_ctrl: ReadWrite<u32, FIFO_CTRL::Register>),
        // UART FIFO status register
        (0x0024 => pub(crate) fifo_status: ReadWrite<u32, FIFO_STATUS::Register>),
        // TX pin override control. Gives direct SW control over TX pin state
        (0x0028 => pub(crate) ovrd: ReadWrite<u32, OVRD::Register>),
        // UART oversampled values
        (0x002c => pub(crate) val: ReadWrite<u32, VAL::Register>),
        // UART RX timeout control
        (0x0030 => pub(crate) timeout_ctrl: ReadWrite<u32, TIMEOUT_CTRL::Register>),
        (0x0034 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) TX_WATERMARK OFFSET(0) NUMBITS(1) [],
        pub(crate) RX_WATERMARK OFFSET(1) NUMBITS(1) [],
        pub(crate) TX_EMPTY OFFSET(2) NUMBITS(1) [],
        pub(crate) RX_OVERFLOW OFFSET(3) NUMBITS(1) [],
        pub(crate) RX_FRAME_ERR OFFSET(4) NUMBITS(1) [],
        pub(crate) RX_BREAK_ERR OFFSET(5) NUMBITS(1) [],
        pub(crate) RX_TIMEOUT OFFSET(6) NUMBITS(1) [],
        pub(crate) RX_PARITY_ERR OFFSET(7) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    CTRL [
        pub(crate) TX OFFSET(0) NUMBITS(1) [],
        pub(crate) RX OFFSET(1) NUMBITS(1) [],
        pub(crate) NF OFFSET(2) NUMBITS(1) [],
        pub(crate) SLPBK OFFSET(4) NUMBITS(1) [],
        pub(crate) LLPBK OFFSET(5) NUMBITS(1) [],
        pub(crate) PARITY_EN OFFSET(6) NUMBITS(1) [],
        pub(crate) PARITY_ODD OFFSET(7) NUMBITS(1) [],
        pub(crate) RXBLVL OFFSET(8) NUMBITS(2) [
            BREAK2 = 0,
            BREAK4 = 1,
            BREAK8 = 2,
            BREAK16 = 3,
        ],
        pub(crate) NCO OFFSET(16) NUMBITS(16) [],
    ],
    STATUS [
        pub(crate) TXFULL OFFSET(0) NUMBITS(1) [],
        pub(crate) RXFULL OFFSET(1) NUMBITS(1) [],
        pub(crate) TXEMPTY OFFSET(2) NUMBITS(1) [],
        pub(crate) TXIDLE OFFSET(3) NUMBITS(1) [],
        pub(crate) RXIDLE OFFSET(4) NUMBITS(1) [],
        pub(crate) RXEMPTY OFFSET(5) NUMBITS(1) [],
    ],
    RDATA [
        pub(crate) RDATA OFFSET(0) NUMBITS(8) [],
    ],
    WDATA [
        pub(crate) WDATA OFFSET(0) NUMBITS(8) [],
    ],
    FIFO_CTRL [
        pub(crate) RXRST OFFSET(0) NUMBITS(1) [],
        pub(crate) TXRST OFFSET(1) NUMBITS(1) [],
        pub(crate) RXILVL OFFSET(2) NUMBITS(3) [
            RXLVL1 = 0,
            RXLVL4 = 1,
            RXLVL8 = 2,
            RXLVL16 = 3,
            RXLVL30 = 4,
        ],
        pub(crate) TXILVL OFFSET(5) NUMBITS(2) [
            TXLVL1 = 0,
            TXLVL4 = 1,
            TXLVL8 = 2,
            TXLVL16 = 3,
        ],
    ],
    FIFO_STATUS [
        pub(crate) TXLVL OFFSET(0) NUMBITS(6) [],
        pub(crate) RXLVL OFFSET(16) NUMBITS(6) [],
    ],
    OVRD [
        pub(crate) TXEN OFFSET(0) NUMBITS(1) [],
        pub(crate) TXVAL OFFSET(1) NUMBITS(1) [],
    ],
    VAL [
        pub(crate) RX OFFSET(0) NUMBITS(16) [],
    ],
    TIMEOUT_CTRL [
        pub(crate) VAL OFFSET(0) NUMBITS(24) [],
        pub(crate) EN OFFSET(31) NUMBITS(1) [],
    ],
];

// End generated register constants for uart