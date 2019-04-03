#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub ver_id: VER_ID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - Clock Configuration Register"]
    pub clkcfg: CLKCFG,
    #[doc = "0x0c - Baud Rate Divisor Register"]
    pub divisor: DIVISOR,
    #[doc = "0x10 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x14 - Interrupt Mask Register"]
    pub int_mask: INT_MASK,
    #[doc = "0x18 - Receiver Threshold Register"]
    pub rx_thd: RX_THD,
    #[doc = "0x1c - Transmitter Threshold Register"]
    pub tx_thd: TX_THD,
    #[doc = "0x20 - Receive Status Register"]
    pub rx_status: RX_STATUS,
    #[doc = "0x24 - Transmitter Status Register"]
    pub tx_status: TX_STATUS,
    #[doc = "0x28 - Port Control and Status Register"]
    pub pcsr: PCSR,
    #[doc = "0x2c - Receive Data Read Buffer"]
    pub rx_buf: RX_BUF,
    #[doc = "0x30 - Transmit Data Buffer"]
    pub tx_buf: TX_BUF,
    #[doc = "0x34 - Transmitter Guard ETU Value Register"]
    pub tx_getu: TX_GETU,
    #[doc = "0x38 - Character Wait Time Value Register"]
    pub cwt_val: CWT_VAL,
    #[doc = "0x3c - Block Wait Time Value Register"]
    pub bwt_val: BWT_VAL,
    #[doc = "0x40 - Block Guard Time Value Register"]
    pub bgt_val: BGT_VAL,
    #[doc = "0x44 - General Purpose Counter 0 Timeout Value Register"]
    pub gpcnt0_val: GPCNT0_VAL,
    #[doc = "0x48 - General Purpose Counter 1 Timeout Value"]
    pub gpcnt1_val: GPCNT1_VAL,
}
#[doc = "Version ID Register"]
pub struct VER_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod ver_id;
#[doc = "Parameter Register"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "Clock Configuration Register"]
pub struct CLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Configuration Register"]
pub mod clkcfg;
#[doc = "Baud Rate Divisor Register"]
pub struct DIVISOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Divisor Register"]
pub mod divisor;
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Interrupt Mask Register"]
pub struct INT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod int_mask;
#[doc = "Receiver Threshold Register"]
pub struct RX_THD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Threshold Register"]
pub mod rx_thd;
#[doc = "Transmitter Threshold Register"]
pub struct TX_THD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Threshold Register"]
pub mod tx_thd;
#[doc = "Receive Status Register"]
pub struct RX_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Register"]
pub mod rx_status;
#[doc = "Transmitter Status Register"]
pub struct TX_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Status Register"]
pub mod tx_status;
#[doc = "Port Control and Status Register"]
pub struct PCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control and Status Register"]
pub mod pcsr;
#[doc = "Receive Data Read Buffer"]
pub struct RX_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Read Buffer"]
pub mod rx_buf;
#[doc = "Transmit Data Buffer"]
pub struct TX_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Buffer"]
pub mod tx_buf;
#[doc = "Transmitter Guard ETU Value Register"]
pub struct TX_GETU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Guard ETU Value Register"]
pub mod tx_getu;
#[doc = "Character Wait Time Value Register"]
pub struct CWT_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Character Wait Time Value Register"]
pub mod cwt_val;
#[doc = "Block Wait Time Value Register"]
pub struct BWT_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Wait Time Value Register"]
pub mod bwt_val;
#[doc = "Block Guard Time Value Register"]
pub struct BGT_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Guard Time Value Register"]
pub mod bgt_val;
#[doc = "General Purpose Counter 0 Timeout Value Register"]
pub struct GPCNT0_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Counter 0 Timeout Value Register"]
pub mod gpcnt0_val;
#[doc = "General Purpose Counter 1 Timeout Value"]
pub struct GPCNT1_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Counter 1 Timeout Value"]
pub mod gpcnt1_val;
