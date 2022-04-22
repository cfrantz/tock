// Generated register constants for adc_ctrl

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
// Number for ADC filters
pub const ADC_CTRL_PARAM_NUM_ADC_FILTER: u32 = 8;
// Number for ADC channels
pub const ADC_CTRL_PARAM_NUM_ADC_CHANNEL: u32 = 2;
// Number of alerts
pub const ADC_CTRL_PARAM_NUM_ALERTS: u32 = 1;
// Register width
pub const ADC_CTRL_PARAM_REG_WIDTH: u32 = 32;

register_structs! {
    pub AdcCtrlRegisters {
        // Interrupt State Register
        (0x0000 => pub(crate) intr_state: ReadWrite<u32, INTR::Register>),
        // Interrupt Enable Register
        (0x0004 => pub(crate) intr_enable: ReadWrite<u32, INTR::Register>),
        // Interrupt Test Register
        (0x0008 => pub(crate) intr_test: ReadWrite<u32, INTR::Register>),
        // Alert Test Register
        (0x000c => pub(crate) alert_test: ReadWrite<u32, ALERT_TEST::Register>),
        // ADC enable control register
        (0x0010 => pub(crate) adc_en_ctl: ReadWrite<u32, ADC_EN_CTL::Register>),
        // ADC PowerDown(PD) control register
        (0x0014 => pub(crate) adc_pd_ctl: ReadWrite<u32, ADC_PD_CTL::Register>),
        // ADC Low-Power(LP) sample control register
        (0x0018 => pub(crate) adc_lp_sample_ctl: ReadWrite<u32, ADC_LP_SAMPLE_CTL::Register>),
        // ADC sample control register
        (0x001c => pub(crate) adc_sample_ctl: ReadWrite<u32, ADC_SAMPLE_CTL::Register>),
        // ADC FSM reset control
        (0x0020 => pub(crate) adc_fsm_rst: ReadWrite<u32, ADC_FSM_RST::Register>),
        // ADC channel0 filter range
        (0x0024 => pub(crate) adc_chn0_filter_ctl: [ReadWrite<u32, ADC_CHN0_FILTER_CTL::Register>; 8]),
        // ADC channel1 filter range
        (0x0044 => pub(crate) adc_chn1_filter_ctl: [ReadWrite<u32, ADC_CHN1_FILTER_CTL::Register>; 8]),
        // ADC value sampled on channel
        (0x0064 => pub(crate) adc_chn_val: [ReadWrite<u32, ADC_CHN_VAL::Register>; 2]),
        // Enable filter matches as wakeups
        (0x006c => pub(crate) adc_wakeup_ctl: ReadWrite<u32, ADC_WAKEUP_CTL::Register>),
        // Adc filter match status
        (0x0070 => pub(crate) filter_status: ReadWrite<u32, FILTER_STATUS::Register>),
        // Interrupt enable controls.
        (0x0074 => pub(crate) adc_intr_ctl: ReadWrite<u32, ADC_INTR_CTL::Register>),
        // Debug cable internal status
        (0x0078 => pub(crate) adc_intr_status: ReadWrite<u32, ADC_INTR_STATUS::Register>),
        (0x007c => @END),
    }
}

register_bitfields![u32,
    // Common Interrupt Offsets
    INTR [
        pub(crate) DEBUG_CABLE OFFSET(0) NUMBITS(1) [],
    ],
    ALERT_TEST [
        pub(crate) FATAL_FAULT OFFSET(0) NUMBITS(1) [],
    ],
    ADC_EN_CTL [
        pub(crate) ADC_ENABLE OFFSET(0) NUMBITS(1) [],
        pub(crate) ONESHOT_MODE OFFSET(1) NUMBITS(1) [],
    ],
    ADC_PD_CTL [
        pub(crate) LP_MODE OFFSET(0) NUMBITS(1) [],
        pub(crate) PWRUP_TIME OFFSET(4) NUMBITS(4) [],
        pub(crate) WAKEUP_TIME OFFSET(8) NUMBITS(24) [],
    ],
    ADC_LP_SAMPLE_CTL [
        pub(crate) LP_SAMPLE_CNT OFFSET(0) NUMBITS(8) [],
    ],
    ADC_SAMPLE_CTL [
        pub(crate) NP_SAMPLE_CNT OFFSET(0) NUMBITS(16) [],
    ],
    ADC_FSM_RST [
        pub(crate) RST_EN OFFSET(0) NUMBITS(1) [],
    ],
    ADC_CHN0_FILTER_CTL [
        pub(crate) MIN_V_0 OFFSET(2) NUMBITS(10) [],
        pub(crate) COND_0 OFFSET(12) NUMBITS(1) [],
        pub(crate) MAX_V_0 OFFSET(18) NUMBITS(10) [],
        pub(crate) EN_0 OFFSET(31) NUMBITS(1) [],
    ],
    ADC_CHN1_FILTER_CTL [
        pub(crate) MIN_V_0 OFFSET(2) NUMBITS(10) [],
        pub(crate) COND_0 OFFSET(12) NUMBITS(1) [],
        pub(crate) MAX_V_0 OFFSET(18) NUMBITS(10) [],
        pub(crate) EN_0 OFFSET(31) NUMBITS(1) [],
    ],
    ADC_CHN_VAL [
        pub(crate) ADC_CHN_VALUE_EXT_0 OFFSET(0) NUMBITS(2) [],
        pub(crate) ADC_CHN_VALUE_0 OFFSET(2) NUMBITS(10) [],
        pub(crate) ADC_CHN_VALUE_INTR_EXT_0 OFFSET(16) NUMBITS(2) [],
        pub(crate) ADC_CHN_VALUE_INTR_0 OFFSET(18) NUMBITS(10) [],
    ],
    ADC_WAKEUP_CTL [
        pub(crate) EN OFFSET(0) NUMBITS(8) [],
    ],
    FILTER_STATUS [
        pub(crate) COND OFFSET(0) NUMBITS(8) [],
    ],
    ADC_INTR_CTL [
        pub(crate) EN OFFSET(0) NUMBITS(9) [],
    ],
    ADC_INTR_STATUS [
        pub(crate) CC_SINK_DET OFFSET(0) NUMBITS(1) [],
        pub(crate) CC_1A5_SINK_DET OFFSET(1) NUMBITS(1) [],
        pub(crate) CC_3A0_SINK_DET OFFSET(2) NUMBITS(1) [],
        pub(crate) CC_SRC_DET OFFSET(3) NUMBITS(1) [],
        pub(crate) CC_1A5_SRC_DET OFFSET(4) NUMBITS(1) [],
        pub(crate) CC_SRC_DET_FLIP OFFSET(5) NUMBITS(1) [],
        pub(crate) CC_1A5_SRC_DET_FLIP OFFSET(6) NUMBITS(1) [],
        pub(crate) CC_DISCON OFFSET(7) NUMBITS(1) [],
        pub(crate) ONESHOT OFFSET(8) NUMBITS(1) [],
    ],
];

// End generated register constants for adc_ctrl