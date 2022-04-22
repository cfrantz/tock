// Generated register constants for SPI_DEVICE

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
pub const SPI_DEVICE_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const SPI_DEVICE_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub SpiDeviceRegisters {
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
        // Configuration Register
        (0x0014 => pub(crate) cfg: ReadWrite<u32, CFG::Register>),
        // RX/ TX FIFO levels.
        (0x0018 => pub(crate) fifo_level: ReadWrite<u32, FIFO_LEVEL::Register>),
        // RX/ TX Async FIFO levels between main clk and spi clock
        (0x001c => pub(crate) async_fifo_level: ReadWrite<u32, ASYNC_FIFO_LEVEL::Register>),
        // SPI Device status register
        (0x0020 => pub(crate) status: ReadWrite<u32, STATUS::Register>),
        // Receiver FIFO (SRAM) pointers
        (0x0024 => pub(crate) rxf_ptr: ReadWrite<u32, RXF_PTR::Register>),
        // Transmitter FIFO (SRAM) pointers
        (0x0028 => pub(crate) txf_ptr: ReadWrite<u32, TXF_PTR::Register>),
        // Receiver FIFO (SRAM) Addresses
        (0x002c => pub(crate) rxf_addr: ReadWrite<u32, RXF_ADDR::Register>),
        // Transmitter FIFO (SRAM) Addresses
        (0x0030 => pub(crate) txf_addr: ReadWrite<u32, TXF_ADDR::Register>),
        // Intercept Passthrough datapath.
        (0x0034 => pub(crate) intercept_en: ReadWrite<u32, INTERCEPT_EN::Register>),
        // Last Read Address
        (0x0038 => pub(crate) last_read_addr: ReadWrite<u32, LAST_READ_ADDR::Register>),
        // SPI Flash Status register.
        (0x003c => pub(crate) flash_status: ReadWrite<u32, FLASH_STATUS::Register>),
        // JEDEC Continuation Code configuration register.
        (0x0040 => pub(crate) jedec_cc: ReadWrite<u32, JEDEC_CC::Register>),
        // JEDEC ID register.
        (0x0044 => pub(crate) jedec_id: ReadWrite<u32, JEDEC_ID::Register>),
        // Read Buffer threshold register.
        (0x0048 => pub(crate) read_threshold: ReadWrite<u32, READ_THRESHOLD::Register>),
        // Mailbox Base address register.
        (0x004c => pub(crate) mailbox_addr: ReadWrite<u32, MAILBOX_ADDR::Register>),
        // Upload module status register.
        (0x0050 => pub(crate) upload_status: ReadWrite<u32, UPLOAD_STATUS::Register>),
        // Upload module status 2 register.
        (0x0054 => pub(crate) upload_status2: ReadWrite<u32, UPLOAD_STATUS2::Register>),
        // Command Fifo Read Port.
        (0x0058 => pub(crate) upload_cmdfifo: ReadWrite<u32, UPLOAD_CMDFIFO::Register>),
        // Address Fifo Read Port.
        (0x005c => pub(crate) upload_addrfifo: ReadWrite<u32, UPLOAD_ADDRFIFO::Register>),
        // Command Filter
        (0x0060 => pub(crate) cmd_filter: [ReadWrite<u32, CMD_FILTER::Register>; 8]),
        // Address Swap Mask register.
        (0x0080 => pub(crate) addr_swap_mask: ReadWrite<u32, ADDR_SWAP_MASK::Register>),
        // The address value for the address swap feature.
        (0x0084 => pub(crate) addr_swap_data: ReadWrite<u32, ADDR_SWAP_DATA::Register>),
        // Write Data Swap in the passthrough mode.
        (0x0088 => pub(crate) payload_swap_mask: ReadWrite<u32, PAYLOAD_SWAP_MASK::Register>),
        // Write Data Swap in the passthrough mode.
        (0x008c => pub(crate) payload_swap_data: ReadWrite<u32, PAYLOAD_SWAP_DATA::Register>),
        // Command Info register.
        (0x0090 => pub(crate) cmd_info: [ReadWrite<u32, CMD_INFO::Register>; 24]),
        // Opcode for EN4B.
        (0x00f0 => pub(crate) cmd_info_en4b: ReadWrite<u32, CMD_INFO_EN4B::Register>),
        // Opcode for EX4B
        (0x00f4 => pub(crate) cmd_info_ex4b: ReadWrite<u32, CMD_INFO_EX4B::Register>),
        // Opcode for Write Enable (WREN)
        (0x00f8 => pub(crate) cmd_info_wren: ReadWrite<u32, CMD_INFO_WREN::Register>),
        // Opcode for Write Disable (WRDI)
        (0x00fc => pub(crate) cmd_info_wrdi: ReadWrite<u32, CMD_INFO_WRDI::Register>),
        // TPM HWIP Capability register.
        (0x0800 => pub(crate) tpm_cap: ReadWrite<u32, TPM_CAP::Register>),
        // TPM Configuration register.
        (0x0804 => pub(crate) tpm_cfg: ReadWrite<u32, TPM_CFG::Register>),
        // TPM submodule state register.
        (0x0808 => pub(crate) tpm_status: ReadWrite<u32, TPM_STATUS::Register>),
        // TPM_ACCESS_x register.
        (0x080c => pub(crate) tpm_access: [ReadWrite<u32, TPM_ACCESS::Register>; 2]),
        // TPM_STS_x register.
        (0x0814 => pub(crate) tpm_sts: ReadWrite<u32, TPM_STS::Register>),
        // TPM_INTF_CAPABILITY
        (0x0818 => pub(crate) tpm_intf_capability: ReadWrite<u32, TPM_INTF_CAPABILITY::Register>),
        // TPM_INT_ENABLE
        (0x081c => pub(crate) tpm_int_enable: ReadWrite<u32, TPM_INT_ENABLE::Register>),
        // TPM_INT_VECTOR
        (0x0820 => pub(crate) tpm_int_vector: ReadWrite<u32, TPM_INT_VECTOR::Register>),
        // TPM_INT_STATUS
        (0x0824 => pub(crate) tpm_int_status: ReadWrite<u32, TPM_INT_STATUS::Register>),
        // TPM_DID/ TPM_VID register
        (0x0828 => pub(crate) tpm_did_vid: ReadWrite<u32, TPM_DID_VID::Register>),
        // TPM_RID
        (0x082c => pub(crate) tpm_rid: ReadWrite<u32, TPM_RID::Register>),
        // TPM Command and Address buffer
        (0x0830 => pub(crate) tpm_cmd_addr: ReadWrite<u32, TPM_CMD_ADDR::Register>),
        // TPM Read command return data FIFO.
        (0x0834 => pub(crate) tpm_read_fifo: ReadWrite<u32, TPM_READ_FIFO::Register>),
        // TPM Write command received data FIFO.
        (0x0838 => pub(crate) tpm_write_fifo: ReadWrite<u32, TPM_WRITE_FIFO::Register>),
        // Memory area: SPI internal buffer.
        (0x1000 => pub(crate) buffer: [ReadWrite<u32>; 1024]),
        (0x2000 => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) GENERIC_RX_FULL OFFSET(0) NUMBITS(1) [],
        pub(crate) GENERIC_RX_WATERMARK OFFSET(1) NUMBITS(1) [],
        pub(crate) GENERIC_TX_WATERMARK OFFSET(2) NUMBITS(1) [],
        pub(crate) GENERIC_RX_ERROR OFFSET(3) NUMBITS(1) [],
        pub(crate) GENERIC_RX_OVERFLOW OFFSET(4) NUMBITS(1) [],
        pub(crate) GENERIC_TX_UNDERFLOW OFFSET(5) NUMBITS(1) [],
        pub(crate) UPLOAD_CMDFIFO_NOT_EMPTY OFFSET(6) NUMBITS(1) [],
        pub(crate) UPLOAD_PAYLOAD_NOT_EMPTY OFFSET(7) NUMBITS(1) [],
        pub(crate) UPLOAD_PAYLOAD_OVERFLOW OFFSET(8) NUMBITS(1) [],
        pub(crate) READBUF_WATERMARK OFFSET(9) NUMBITS(1) [],
        pub(crate) READBUF_FLIP OFFSET(10) NUMBITS(1) [],
        pub(crate) TPM_HEADER_NOT_EMPTY OFFSET(11) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    CONTROL [
        pub(crate) ABORT OFFSET(0) NUMBITS(1) [],
        pub(crate) MODE OFFSET(4) NUMBITS(2) [
            FWMODE = 0,
            FLASHMODE = 1,
            PASSTHROUGH = 2,
        ],
        pub(crate) RST_TXFIFO OFFSET(16) NUMBITS(1) [],
        pub(crate) RST_RXFIFO OFFSET(17) NUMBITS(1) [],
        pub(crate) SRAM_CLK_EN OFFSET(31) NUMBITS(1) [],
    ],
    CFG [
        pub(crate) CPOL OFFSET(0) NUMBITS(1) [],
        pub(crate) CPHA OFFSET(1) NUMBITS(1) [],
        pub(crate) TX_ORDER OFFSET(2) NUMBITS(1) [],
        pub(crate) RX_ORDER OFFSET(3) NUMBITS(1) [],
        pub(crate) TIMER_V OFFSET(8) NUMBITS(8) [],
        pub(crate) ADDR_4B_EN OFFSET(16) NUMBITS(1) [],
        pub(crate) MAILBOX_EN OFFSET(24) NUMBITS(1) [],
    ],
    FIFO_LEVEL [
        pub(crate) RXLVL OFFSET(0) NUMBITS(16) [],
        pub(crate) TXLVL OFFSET(16) NUMBITS(16) [],
    ],
    ASYNC_FIFO_LEVEL [
        pub(crate) RXLVL OFFSET(0) NUMBITS(8) [],
        pub(crate) TXLVL OFFSET(16) NUMBITS(8) [],
    ],
    STATUS [
        pub(crate) RXF_FULL OFFSET(0) NUMBITS(1) [],
        pub(crate) RXF_EMPTY OFFSET(1) NUMBITS(1) [],
        pub(crate) TXF_FULL OFFSET(2) NUMBITS(1) [],
        pub(crate) TXF_EMPTY OFFSET(3) NUMBITS(1) [],
        pub(crate) ABORT_DONE OFFSET(4) NUMBITS(1) [],
        pub(crate) CSB OFFSET(5) NUMBITS(1) [],
    ],
    RXF_PTR [
        pub(crate) RPTR OFFSET(0) NUMBITS(16) [],
        pub(crate) WPTR OFFSET(16) NUMBITS(16) [],
    ],
    TXF_PTR [
        pub(crate) RPTR OFFSET(0) NUMBITS(16) [],
        pub(crate) WPTR OFFSET(16) NUMBITS(16) [],
    ],
    RXF_ADDR [
        pub(crate) BASE OFFSET(0) NUMBITS(16) [],
        pub(crate) LIMIT OFFSET(16) NUMBITS(16) [],
    ],
    TXF_ADDR [
        pub(crate) BASE OFFSET(0) NUMBITS(16) [],
        pub(crate) LIMIT OFFSET(16) NUMBITS(16) [],
    ],
    INTERCEPT_EN [
        pub(crate) STATUS OFFSET(0) NUMBITS(1) [],
        pub(crate) JEDEC OFFSET(1) NUMBITS(1) [],
        pub(crate) SFDP OFFSET(2) NUMBITS(1) [],
        pub(crate) MBX OFFSET(3) NUMBITS(1) [],
    ],
    LAST_READ_ADDR [],
    FLASH_STATUS [
        pub(crate) BUSY OFFSET(0) NUMBITS(1) [],
        pub(crate) STATUS OFFSET(1) NUMBITS(23) [],
    ],
    JEDEC_CC [
        pub(crate) CC OFFSET(0) NUMBITS(8) [],
        pub(crate) NUM_CC OFFSET(8) NUMBITS(8) [],
    ],
    JEDEC_ID [
        pub(crate) ID OFFSET(0) NUMBITS(16) [],
        pub(crate) MF OFFSET(16) NUMBITS(8) [],
    ],
    READ_THRESHOLD [
        pub(crate) THRESHOLD OFFSET(0) NUMBITS(10) [],
    ],
    MAILBOX_ADDR [],
    UPLOAD_STATUS [
        pub(crate) CMDFIFO_DEPTH OFFSET(0) NUMBITS(5) [],
        pub(crate) CMDFIFO_NOTEMPTY OFFSET(7) NUMBITS(1) [],
        pub(crate) ADDRFIFO_DEPTH OFFSET(8) NUMBITS(5) [],
        pub(crate) ADDRFIFO_NOTEMPTY OFFSET(15) NUMBITS(1) [],
    ],
    UPLOAD_STATUS2 [
        pub(crate) PAYLOAD_DEPTH OFFSET(0) NUMBITS(9) [],
        pub(crate) PAYLOAD_START_IDX OFFSET(16) NUMBITS(8) [],
    ],
    UPLOAD_CMDFIFO [
        pub(crate) DATA OFFSET(0) NUMBITS(8) [],
    ],
    UPLOAD_ADDRFIFO [],
    CMD_FILTER [
        pub(crate) FILTER_0 OFFSET(0) NUMBITS(1) [],
        pub(crate) FILTER_1 OFFSET(1) NUMBITS(1) [],
        pub(crate) FILTER_2 OFFSET(2) NUMBITS(1) [],
        pub(crate) FILTER_3 OFFSET(3) NUMBITS(1) [],
        pub(crate) FILTER_4 OFFSET(4) NUMBITS(1) [],
        pub(crate) FILTER_5 OFFSET(5) NUMBITS(1) [],
        pub(crate) FILTER_6 OFFSET(6) NUMBITS(1) [],
        pub(crate) FILTER_7 OFFSET(7) NUMBITS(1) [],
        pub(crate) FILTER_8 OFFSET(8) NUMBITS(1) [],
        pub(crate) FILTER_9 OFFSET(9) NUMBITS(1) [],
        pub(crate) FILTER_10 OFFSET(10) NUMBITS(1) [],
        pub(crate) FILTER_11 OFFSET(11) NUMBITS(1) [],
        pub(crate) FILTER_12 OFFSET(12) NUMBITS(1) [],
        pub(crate) FILTER_13 OFFSET(13) NUMBITS(1) [],
        pub(crate) FILTER_14 OFFSET(14) NUMBITS(1) [],
        pub(crate) FILTER_15 OFFSET(15) NUMBITS(1) [],
        pub(crate) FILTER_16 OFFSET(16) NUMBITS(1) [],
        pub(crate) FILTER_17 OFFSET(17) NUMBITS(1) [],
        pub(crate) FILTER_18 OFFSET(18) NUMBITS(1) [],
        pub(crate) FILTER_19 OFFSET(19) NUMBITS(1) [],
        pub(crate) FILTER_20 OFFSET(20) NUMBITS(1) [],
        pub(crate) FILTER_21 OFFSET(21) NUMBITS(1) [],
        pub(crate) FILTER_22 OFFSET(22) NUMBITS(1) [],
        pub(crate) FILTER_23 OFFSET(23) NUMBITS(1) [],
        pub(crate) FILTER_24 OFFSET(24) NUMBITS(1) [],
        pub(crate) FILTER_25 OFFSET(25) NUMBITS(1) [],
        pub(crate) FILTER_26 OFFSET(26) NUMBITS(1) [],
        pub(crate) FILTER_27 OFFSET(27) NUMBITS(1) [],
        pub(crate) FILTER_28 OFFSET(28) NUMBITS(1) [],
        pub(crate) FILTER_29 OFFSET(29) NUMBITS(1) [],
        pub(crate) FILTER_30 OFFSET(30) NUMBITS(1) [],
        pub(crate) FILTER_31 OFFSET(31) NUMBITS(1) [],
    ],
    ADDR_SWAP_MASK [],
    ADDR_SWAP_DATA [],
    PAYLOAD_SWAP_MASK [],
    PAYLOAD_SWAP_DATA [],
    CMD_INFO [
        pub(crate) OPCODE_0 OFFSET(0) NUMBITS(8) [],
        pub(crate) ADDR_MODE_0 OFFSET(8) NUMBITS(2) [
            ADDRDISABLED = 0,
            ADDRCFG = 1,
            ADDR3B = 2,
            ADDR4B = 3,
        ],
        pub(crate) ADDR_SWAP_EN_0 OFFSET(10) NUMBITS(1) [],
        pub(crate) MBYTE_EN_0 OFFSET(11) NUMBITS(1) [],
        pub(crate) DUMMY_SIZE_0 OFFSET(12) NUMBITS(3) [],
        pub(crate) DUMMY_EN_0 OFFSET(15) NUMBITS(1) [],
        pub(crate) PAYLOAD_EN_0 OFFSET(16) NUMBITS(4) [],
        pub(crate) PAYLOAD_DIR_0 OFFSET(20) NUMBITS(1) [
            PAYLOADIN = 0,
            PAYLOADOUT = 1,
        ],
        pub(crate) PAYLOAD_SWAP_EN_0 OFFSET(21) NUMBITS(1) [],
        pub(crate) UPLOAD_0 OFFSET(24) NUMBITS(1) [],
        pub(crate) BUSY_0 OFFSET(25) NUMBITS(1) [],
        pub(crate) VALID_0 OFFSET(31) NUMBITS(1) [],
    ],
    CMD_INFO_EN4B [
        pub(crate) OPCODE OFFSET(0) NUMBITS(8) [],
        pub(crate) VALID OFFSET(31) NUMBITS(1) [],
    ],
    CMD_INFO_EX4B [
        pub(crate) OPCODE OFFSET(0) NUMBITS(8) [],
        pub(crate) VALID OFFSET(31) NUMBITS(1) [],
    ],
    CMD_INFO_WREN [
        pub(crate) OPCODE OFFSET(0) NUMBITS(8) [],
        pub(crate) VALID OFFSET(31) NUMBITS(1) [],
    ],
    CMD_INFO_WRDI [
        pub(crate) OPCODE OFFSET(0) NUMBITS(8) [],
        pub(crate) VALID OFFSET(31) NUMBITS(1) [],
    ],
    TPM_CAP [
        pub(crate) REV OFFSET(0) NUMBITS(8) [],
        pub(crate) LOCALITY OFFSET(8) NUMBITS(1) [],
        pub(crate) MAX_XFER_SIZE OFFSET(16) NUMBITS(3) [],
    ],
    TPM_CFG [
        pub(crate) EN OFFSET(0) NUMBITS(1) [],
        pub(crate) TPM_MODE OFFSET(1) NUMBITS(1) [],
        pub(crate) HW_REG_DIS OFFSET(2) NUMBITS(1) [],
        pub(crate) TPM_REG_CHK_DIS OFFSET(3) NUMBITS(1) [],
        pub(crate) INVALID_LOCALITY OFFSET(4) NUMBITS(1) [],
    ],
    TPM_STATUS [
        pub(crate) CMDADDR_NOTEMPTY OFFSET(0) NUMBITS(1) [],
        pub(crate) RDFIFO_NOTEMPTY OFFSET(1) NUMBITS(1) [],
        pub(crate) RDFIFO_DEPTH OFFSET(4) NUMBITS(3) [],
        pub(crate) WRFIFO_DEPTH OFFSET(8) NUMBITS(3) [],
    ],
    TPM_ACCESS [
        pub(crate) ACCESS_0 OFFSET(0) NUMBITS(8) [],
        pub(crate) ACCESS_1 OFFSET(8) NUMBITS(8) [],
        pub(crate) ACCESS_2 OFFSET(16) NUMBITS(8) [],
        pub(crate) ACCESS_3 OFFSET(24) NUMBITS(8) [],
    ],
    TPM_STS [],
    TPM_INTF_CAPABILITY [],
    TPM_INT_ENABLE [],
    TPM_INT_VECTOR [
        pub(crate) INT_VECTOR OFFSET(0) NUMBITS(8) [],
    ],
    TPM_INT_STATUS [],
    TPM_DID_VID [
        pub(crate) VID OFFSET(0) NUMBITS(16) [],
        pub(crate) DID OFFSET(16) NUMBITS(16) [],
    ],
    TPM_RID [
        pub(crate) RID OFFSET(0) NUMBITS(8) [],
    ],
    TPM_CMD_ADDR [
        pub(crate) ADDR OFFSET(0) NUMBITS(24) [],
        pub(crate) CMD OFFSET(24) NUMBITS(8) [],
    ],
    TPM_READ_FIFO [
        pub(crate) VALUE OFFSET(0) NUMBITS(8) [],
    ],
    TPM_WRITE_FIFO [
        pub(crate) VALUE OFFSET(0) NUMBITS(8) [],
    ],
];

// End generated register constants for SPI_DEVICE