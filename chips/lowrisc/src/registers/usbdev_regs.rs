// Generated register constants for usbdev

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
// Number of endpoints
pub const USBDEV_PARAM_N_ENDPOINTS: u32 = 12;
// Number of alerts
pub const USBDEV_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const USBDEV_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub UsbdevRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // USB Control
        (0x0010 => pub(crate) usbctrl: ReadWrite<u32, USBCTRL::Register>),
        // Enable an endpoint to respond to transactions in the downstream direction.
        (0x0014 => pub(crate) ep_out_enab: [ReadWrite<u32, EP_OUT_ENAB::Register>; 1]),
        // Enable an endpoint to respond to transactions in the upstream direction.
        (0x0018 => pub(crate) ep_in_enab: [ReadWrite<u32, EP_IN_ENAB::Register>; 1]),
        // USB Status
        (0x001c => pub(crate) usbstat: ReadWrite<u32, USBSTAT::Register>),
        // Available Buffer FIFO
        (0x0020 => pub(crate) avbuffer: ReadWrite<u32, AVBUFFER::Register>),
        // Received Buffer FIFO
        (0x0024 => pub(crate) rxfifo: ReadWrite<u32, RXFIFO::Register>),
        // Receive SETUP transaction enable
        (0x0028 => pub(crate) rxenable_set: [ReadWrite<u32, RXENABLE_SET::Register>; 1]),
        // Receive OUT transaction enable
        (0x002c => pub(crate) rxenable_o: [ReadWrite<u32, RXENABLE_O::Register>; 1]),
        // Set NAK after OUT transactions
        (0x0030 => pub(crate) set_nak_o: [ReadWrite<u32, SET_NAK_O::Register>; 1]),
        // IN Transaction Sent
        (0x0034 => pub(crate) in_se: [ReadWrite<u32, IN_SE::Register>; 1]),
        // OUT Endpoint STALL control
        (0x0038 => pub(crate) out_sta: [ReadWrite<u32, OUT_STA::Register>; 1]),
        // IN Endpoint STALL control
        (0x003c => pub(crate) in_sta: [ReadWrite<u32, IN_STA::Register>; 1]),
        // Configure IN Transaction
        (0x0040 => pub(crate) configin: [ReadWrite<u32, CONFIGIN::Register>; 12]),
        // OUT Endpoint isochronous setting
        (0x0070 => pub(crate) out_i: [ReadWrite<u32, OUT_I::Register>; 1]),
        // IN Endpoint isochronous setting
        (0x0074 => pub(crate) in_i: [ReadWrite<u32, IN_I::Register>; 1]),
        // Clear the data toggle flag
        (0x0078 => pub(crate) data_toggle_cle: [ReadWrite<u32, DATA_TOGGLE_CLE::Register>; 1]),
        // USB PHY pins sense.
        (0x007c => pub(crate) phy_pins_sense: ReadWrite<u32, PHY_PINS_SENSE::Register>),
        // USB PHY pins drive.
        (0x0080 => pub(crate) phy_pins_drive: ReadWrite<u32, PHY_PINS_DRIVE::Register>),
        // USB PHY Configuration
        (0x0084 => pub(crate) phy_config: ReadWrite<u32, PHY_CONFIG::Register>),
        // USB wake configuration for suspend / resume
        (0x0088 => pub(crate) wake_config: ReadWrite<u32, WAKE_CONFIG::Register>),
        // USB wake module events and debug
        (0x008c => pub(crate) wake_events: ReadWrite<u32, WAKE_EVENTS::Register>),
        // Memory area: 2 kB packet buffer. Divided into 32 64-byte buffers.
        (0x0800 => pub(crate) buffer: [ReadWrite<u32>; 512]),
        (0x1000 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) PKT_RECEIVED OFFSET(0) NUMBITS(1) [],
        pub(crate) PKT_SENT OFFSET(1) NUMBITS(1) [],
        pub(crate) DISCONNECTED OFFSET(2) NUMBITS(1) [],
        pub(crate) HOST_LOST OFFSET(3) NUMBITS(1) [],
        pub(crate) LINK_RESET OFFSET(4) NUMBITS(1) [],
        pub(crate) LINK_SUSPEND OFFSET(5) NUMBITS(1) [],
        pub(crate) LINK_RESUME OFFSET(6) NUMBITS(1) [],
        pub(crate) AV_EMPTY OFFSET(7) NUMBITS(1) [],
        pub(crate) RX_FULL OFFSET(8) NUMBITS(1) [],
        pub(crate) AV_OVERFLOW OFFSET(9) NUMBITS(1) [],
        pub(crate) LINK_IN_ERR OFFSET(10) NUMBITS(1) [],
        pub(crate) RX_CRC_ERR OFFSET(11) NUMBITS(1) [],
        pub(crate) RX_PID_ERR OFFSET(12) NUMBITS(1) [],
        pub(crate) RX_BITSTUFF_ERR OFFSET(13) NUMBITS(1) [],
        pub(crate) FRAME OFFSET(14) NUMBITS(1) [],
        pub(crate) POWERED OFFSET(15) NUMBITS(1) [],
        pub(crate) LINK_OUT_ERR OFFSET(16) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    USBCTRL [
        pub(crate) ENABLE OFFSET(0) NUMBITS(1) [],
        pub(crate) RESUME_LINK_ACTIVE OFFSET(1) NUMBITS(1) [],
        pub(crate) DEVICE_ADDRESS OFFSET(16) NUMBITS(7) [],
    ],
    EP_OUT_ENAB [
        pub(crate) ENABLE_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) ENABLE_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) ENABLE_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) ENABLE_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) ENABLE_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) ENABLE_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) ENABLE_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) ENABLE_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) ENABLE_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) ENABLE_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) ENABLE_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) ENABLE_11 OFFSET(11) NUMBITS(1) [],
    ],
    EP_IN_ENAB [
        pub(crate) ENABLE_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) ENABLE_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) ENABLE_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) ENABLE_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) ENABLE_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) ENABLE_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) ENABLE_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) ENABLE_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) ENABLE_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) ENABLE_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) ENABLE_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) ENABLE_11 OFFSET(11) NUMBITS(1) [],
    ],
    USBSTAT [
        pub(crate) FRAME OFFSET(0) NUMBITS(11) [],
        pub(crate) HOST_LOST OFFSET(11) NUMBITS(1) [],
        pub(crate) LINK_STATE OFFSET(12) NUMBITS(3) [
            DISCONNECTED = 0,
            POWERED = 1,
            POWERED_SUSPENDED = 2,
            ACTIVE = 3,
            SUSPENDED = 4,
            ACTIVE_NOSOF = 5,
            RESUMING = 6,
        ],
        pub(crate) SENSE OFFSET(15) NUMBITS(1) [],
        pub(crate) AV_DEPTH OFFSET(16) NUMBITS(3) [],
        pub(crate) AV_FULL OFFSET(23) NUMBITS(1) [],
        pub(crate) RX_DEPTH OFFSET(24) NUMBITS(3) [],
        pub(crate) RX_EMPTY OFFSET(31) NUMBITS(1) [],
    ],
    AVBUFFER [
        pub(crate) BUFFER OFFSET(0) NUMBITS(5) [],
    ],
    RXFIFO [
        pub(crate) BUFFER OFFSET(0) NUMBITS(5) [],
        pub(crate) SIZE OFFSET(8) NUMBITS(7) [],
        pub(crate) SETUP OFFSET(19) NUMBITS(1) [],
        pub(crate) EP OFFSET(20) NUMBITS(4) [],
    ],
    RXENABLE_SET [
        pub(crate) SETUP_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) SETUP_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) SETUP_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) SETUP_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) SETUP_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) SETUP_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) SETUP_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) SETUP_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) SETUP_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) SETUP_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) SETUP_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) SETUP_11 OFFSET(11) NUMBITS(1) [],
    ],
    RXENABLE_O [
        pub(crate) OUT_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) OUT_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) OUT_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) OUT_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) OUT_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) OUT_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) OUT_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) OUT_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) OUT_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) OUT_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) OUT_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) OUT_11 OFFSET(11) NUMBITS(1) [],
    ],
    SET_NAK_O [
        pub(crate) ENABLE_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) ENABLE_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) ENABLE_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) ENABLE_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) ENABLE_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) ENABLE_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) ENABLE_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) ENABLE_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) ENABLE_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) ENABLE_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) ENABLE_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) ENABLE_11 OFFSET(11) NUMBITS(1) [],
    ],
    IN_SE [
        pub(crate) SENT_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) SENT_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) SENT_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) SENT_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) SENT_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) SENT_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) SENT_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) SENT_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) SENT_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) SENT_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) SENT_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) SENT_11 OFFSET(11) NUMBITS(1) [],
    ],
    OUT_STA [
        pub(crate) ENDPOINT_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) ENDPOINT_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) ENDPOINT_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) ENDPOINT_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) ENDPOINT_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) ENDPOINT_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) ENDPOINT_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) ENDPOINT_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) ENDPOINT_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) ENDPOINT_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) ENDPOINT_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) ENDPOINT_11 OFFSET(11) NUMBITS(1) [],
    ],
    IN_STA [
        pub(crate) ENDPOINT_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) ENDPOINT_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) ENDPOINT_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) ENDPOINT_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) ENDPOINT_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) ENDPOINT_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) ENDPOINT_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) ENDPOINT_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) ENDPOINT_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) ENDPOINT_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) ENDPOINT_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) ENDPOINT_11 OFFSET(11) NUMBITS(1) [],
    ],
    CONFIGIN [
        pub(crate) BUFFER_0 OFFSET(0) NUMBITS(5) [],
        pub(crate) SIZE_0 OFFSET(8) NUMBITS(7) [],
        pub(crate) PEND_0 OFFSET(30) NUMBITS(1) [],
        pub(crate) RDY_0 OFFSET(31) NUMBITS(1) [],
    ],
    OUT_I [
        pub(crate) ISO_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) ISO_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) ISO_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) ISO_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) ISO_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) ISO_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) ISO_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) ISO_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) ISO_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) ISO_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) ISO_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) ISO_11 OFFSET(11) NUMBITS(1) [],
    ],
    IN_I [
        pub(crate) ISO_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) ISO_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) ISO_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) ISO_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) ISO_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) ISO_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) ISO_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) ISO_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) ISO_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) ISO_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) ISO_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) ISO_11 OFFSET(11) NUMBITS(1) [],
    ],
    DATA_TOGGLE_CLE [
        pub(crate) CLEAR_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) CLEAR_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) CLEAR_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) CLEAR_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) CLEAR_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) CLEAR_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) CLEAR_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) CLEAR_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) CLEAR_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) CLEAR_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) CLEAR_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) CLEAR_11 OFFSET(11) NUMBITS(1) [],
    ],
    PHY_PINS_SENSE [
        pub(crate) RX_DP_I OFFSET(0) NUMBITS(1) [],
        pub(crate) RX_DN_I OFFSET(1) NUMBITS(1) [],
        pub(crate) RX_D_I OFFSET(2) NUMBITS(1) [],
        pub(crate) TX_DP_O OFFSET(8) NUMBITS(1) [],
        pub(crate) TX_DN_O OFFSET(9) NUMBITS(1) [],
        pub(crate) TX_D_O OFFSET(10) NUMBITS(1) [],
        pub(crate) TX_SE0_O OFFSET(11) NUMBITS(1) [],
        pub(crate) TX_OE_O OFFSET(12) NUMBITS(1) [],
        pub(crate) SUSPEND_O OFFSET(13) NUMBITS(1) [],
        pub(crate) PWR_SENSE OFFSET(16) NUMBITS(1) [],
    ],
    PHY_PINS_DRIVE [
        pub(crate) DP_O OFFSET(0) NUMBITS(1) [],
        pub(crate) DN_O OFFSET(1) NUMBITS(1) [],
        pub(crate) D_O OFFSET(2) NUMBITS(1) [],
        pub(crate) SE0_O OFFSET(3) NUMBITS(1) [],
        pub(crate) OE_O OFFSET(4) NUMBITS(1) [],
        pub(crate) RX_ENABLE_O OFFSET(5) NUMBITS(1) [],
        pub(crate) DP_PULLUP_EN_O OFFSET(6) NUMBITS(1) [],
        pub(crate) DN_PULLUP_EN_O OFFSET(7) NUMBITS(1) [],
        pub(crate) SUSPEND_O OFFSET(8) NUMBITS(1) [],
        pub(crate) EN OFFSET(16) NUMBITS(1) [],
    ],
    PHY_CONFIG [
        pub(crate) USE_DIFF_RCVR OFFSET(0) NUMBITS(1) [],
        pub(crate) TX_USE_D_SE0 OFFSET(1) NUMBITS(1) [],
        pub(crate) EOP_SINGLE_BIT OFFSET(2) NUMBITS(1) [],
        pub(crate) PINFLIP OFFSET(5) NUMBITS(1) [],
        pub(crate) USB_REF_DISABLE OFFSET(6) NUMBITS(1) [],
        pub(crate) TX_OSC_TEST_MODE OFFSET(7) NUMBITS(1) [],
    ],
    WAKE_CONFIG [
        pub(crate) WAKE_EN OFFSET(0) NUMBITS(1) [],
        pub(crate) WAKE_ACK OFFSET(1) NUMBITS(1) [],
    ],
    WAKE_EVENTS [
        pub(crate) STATE OFFSET(0) NUMBITS(3) [],
        pub(crate) DISCONNECTED OFFSET(8) NUMBITS(1) [],
        pub(crate) BUS_RESET OFFSET(9) NUMBITS(1) [],
    ],
];

// End generated register constants for usbdev