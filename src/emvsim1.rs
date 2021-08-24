#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub ver_id: crate::Reg<ver_id::VER_ID_SPEC>,
    #[doc = "0x04 - Parameter Register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    #[doc = "0x08 - Clock Configuration Register"]
    pub clkcfg: crate::Reg<clkcfg::CLKCFG_SPEC>,
    #[doc = "0x0c - Baud Rate Divisor Register"]
    pub divisor: crate::Reg<divisor::DIVISOR_SPEC>,
    #[doc = "0x10 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x14 - Interrupt Mask Register"]
    pub int_mask: crate::Reg<int_mask::INT_MASK_SPEC>,
    #[doc = "0x18 - Receiver Threshold Register"]
    pub rx_thd: crate::Reg<rx_thd::RX_THD_SPEC>,
    #[doc = "0x1c - Transmitter Threshold Register"]
    pub tx_thd: crate::Reg<tx_thd::TX_THD_SPEC>,
    #[doc = "0x20 - Receive Status Register"]
    pub rx_status: crate::Reg<rx_status::RX_STATUS_SPEC>,
    #[doc = "0x24 - Transmitter Status Register"]
    pub tx_status: crate::Reg<tx_status::TX_STATUS_SPEC>,
    #[doc = "0x28 - Port Control and Status Register"]
    pub pcsr: crate::Reg<pcsr::PCSR_SPEC>,
    #[doc = "0x2c - Receive Data Read Buffer"]
    pub rx_buf: crate::Reg<rx_buf::RX_BUF_SPEC>,
    #[doc = "0x30 - Transmit Data Buffer"]
    pub tx_buf: crate::Reg<tx_buf::TX_BUF_SPEC>,
    #[doc = "0x34 - Transmitter Guard ETU Value Register"]
    pub tx_getu: crate::Reg<tx_getu::TX_GETU_SPEC>,
    #[doc = "0x38 - Character Wait Time Value Register"]
    pub cwt_val: crate::Reg<cwt_val::CWT_VAL_SPEC>,
    #[doc = "0x3c - Block Wait Time Value Register"]
    pub bwt_val: crate::Reg<bwt_val::BWT_VAL_SPEC>,
    #[doc = "0x40 - Block Guard Time Value Register"]
    pub bgt_val: crate::Reg<bgt_val::BGT_VAL_SPEC>,
    #[doc = "0x44 - General Purpose Counter 0 Timeout Value Register"]
    pub gpcnt0_val: crate::Reg<gpcnt0_val::GPCNT0_VAL_SPEC>,
    #[doc = "0x48 - General Purpose Counter 1 Timeout Value"]
    pub gpcnt1_val: crate::Reg<gpcnt1_val::GPCNT1_VAL_SPEC>,
}
#[doc = "VER_ID register accessor: an alias for `Reg<VER_ID_SPEC>`"]
pub type VER_ID = crate::Reg<ver_id::VER_ID_SPEC>;
#[doc = "Version ID Register"]
pub mod ver_id;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CLKCFG register accessor: an alias for `Reg<CLKCFG_SPEC>`"]
pub type CLKCFG = crate::Reg<clkcfg::CLKCFG_SPEC>;
#[doc = "Clock Configuration Register"]
pub mod clkcfg;
#[doc = "DIVISOR register accessor: an alias for `Reg<DIVISOR_SPEC>`"]
pub type DIVISOR = crate::Reg<divisor::DIVISOR_SPEC>;
#[doc = "Baud Rate Divisor Register"]
pub mod divisor;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "INT_MASK register accessor: an alias for `Reg<INT_MASK_SPEC>`"]
pub type INT_MASK = crate::Reg<int_mask::INT_MASK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod int_mask;
#[doc = "RX_THD register accessor: an alias for `Reg<RX_THD_SPEC>`"]
pub type RX_THD = crate::Reg<rx_thd::RX_THD_SPEC>;
#[doc = "Receiver Threshold Register"]
pub mod rx_thd;
#[doc = "TX_THD register accessor: an alias for `Reg<TX_THD_SPEC>`"]
pub type TX_THD = crate::Reg<tx_thd::TX_THD_SPEC>;
#[doc = "Transmitter Threshold Register"]
pub mod tx_thd;
#[doc = "RX_STATUS register accessor: an alias for `Reg<RX_STATUS_SPEC>`"]
pub type RX_STATUS = crate::Reg<rx_status::RX_STATUS_SPEC>;
#[doc = "Receive Status Register"]
pub mod rx_status;
#[doc = "TX_STATUS register accessor: an alias for `Reg<TX_STATUS_SPEC>`"]
pub type TX_STATUS = crate::Reg<tx_status::TX_STATUS_SPEC>;
#[doc = "Transmitter Status Register"]
pub mod tx_status;
#[doc = "PCSR register accessor: an alias for `Reg<PCSR_SPEC>`"]
pub type PCSR = crate::Reg<pcsr::PCSR_SPEC>;
#[doc = "Port Control and Status Register"]
pub mod pcsr;
#[doc = "RX_BUF register accessor: an alias for `Reg<RX_BUF_SPEC>`"]
pub type RX_BUF = crate::Reg<rx_buf::RX_BUF_SPEC>;
#[doc = "Receive Data Read Buffer"]
pub mod rx_buf;
#[doc = "TX_BUF register accessor: an alias for `Reg<TX_BUF_SPEC>`"]
pub type TX_BUF = crate::Reg<tx_buf::TX_BUF_SPEC>;
#[doc = "Transmit Data Buffer"]
pub mod tx_buf;
#[doc = "TX_GETU register accessor: an alias for `Reg<TX_GETU_SPEC>`"]
pub type TX_GETU = crate::Reg<tx_getu::TX_GETU_SPEC>;
#[doc = "Transmitter Guard ETU Value Register"]
pub mod tx_getu;
#[doc = "CWT_VAL register accessor: an alias for `Reg<CWT_VAL_SPEC>`"]
pub type CWT_VAL = crate::Reg<cwt_val::CWT_VAL_SPEC>;
#[doc = "Character Wait Time Value Register"]
pub mod cwt_val;
#[doc = "BWT_VAL register accessor: an alias for `Reg<BWT_VAL_SPEC>`"]
pub type BWT_VAL = crate::Reg<bwt_val::BWT_VAL_SPEC>;
#[doc = "Block Wait Time Value Register"]
pub mod bwt_val;
#[doc = "BGT_VAL register accessor: an alias for `Reg<BGT_VAL_SPEC>`"]
pub type BGT_VAL = crate::Reg<bgt_val::BGT_VAL_SPEC>;
#[doc = "Block Guard Time Value Register"]
pub mod bgt_val;
#[doc = "GPCNT0_VAL register accessor: an alias for `Reg<GPCNT0_VAL_SPEC>`"]
pub type GPCNT0_VAL = crate::Reg<gpcnt0_val::GPCNT0_VAL_SPEC>;
#[doc = "General Purpose Counter 0 Timeout Value Register"]
pub mod gpcnt0_val;
#[doc = "GPCNT1_VAL register accessor: an alias for `Reg<GPCNT1_VAL_SPEC>`"]
pub type GPCNT1_VAL = crate::Reg<gpcnt1_val::GPCNT1_VAL_SPEC>;
#[doc = "General Purpose Counter 1 Timeout Value"]
pub mod gpcnt1_val;
