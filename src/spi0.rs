#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Transfer Count Register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    _reserved_2_spi0_: [u8; 0x08],
    _reserved3: [u8; 0x18],
    #[doc = "0x2c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x30 - DMA/Interrupt Request Select and Enable Register"]
    pub rser: crate::Reg<rser::RSER_SPEC>,
    _reserved_5_spi0_: [u8; 0x04],
    #[doc = "0x38 - POP RX FIFO Register"]
    pub popr: crate::Reg<popr::POPR_SPEC>,
    #[doc = "0x3c..0x4c - Transmit FIFO Registers"]
    pub txfr: [crate::Reg<txfr::TXFR_SPEC>; 4],
    _reserved8: [u8; 0x30],
    #[doc = "0x7c..0x8c - Receive FIFO Registers"]
    pub rxfr: [crate::Reg<rxfr::RXFR_SPEC>; 4],
}
impl RegisterBlock {
    #[doc = "0x0c - Clock and Transfer Attributes Register (In Slave Mode)"]
    #[inline(always)]
    pub fn spi0_ctar_slave(&self) -> &crate::Reg<spi0_ctar_slave::SPI0_CTAR_SLAVE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<spi0_ctar_slave::SPI0_CTAR_SLAVE_SPEC>)
        }
    }
    #[doc = "0x0c..0x14 - Clock and Transfer Attributes Register (In Master Mode)"]
    #[inline(always)]
    pub fn spi0_ctar(&self) -> &[crate::Reg<spi0_ctar::SPI0_CTAR_SPEC>; 2] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const [crate::Reg<spi0_ctar::SPI0_CTAR_SPEC>; 2])
        }
    }
    #[doc = "0x34 - PUSH TX FIFO Register In Slave Mode"]
    #[inline(always)]
    pub fn spi0_pushr_slave(&self) -> &crate::Reg<spi0_pushr_slave::SPI0_PUSHR_SLAVE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<spi0_pushr_slave::SPI0_PUSHR_SLAVE_SPEC>)
        }
    }
    #[doc = "0x34 - PUSH TX FIFO Register In Master Mode"]
    #[inline(always)]
    pub fn spi0_pushr(&self) -> &crate::Reg<spi0_pushr::SPI0_PUSHR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<spi0_pushr::SPI0_PUSHR_SPEC>)
        }
    }
}
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transfer Count Register"]
pub mod tcr;
#[doc = "SPI0_CTAR register accessor: an alias for `Reg<SPI0_CTAR_SPEC>`"]
pub type SPI0_CTAR = crate::Reg<spi0_ctar::SPI0_CTAR_SPEC>;
#[doc = "Clock and Transfer Attributes Register (In Master Mode)"]
pub mod spi0_ctar;
#[doc = "SPI0_CTAR_SLAVE register accessor: an alias for `Reg<SPI0_CTAR_SLAVE_SPEC>`"]
pub type SPI0_CTAR_SLAVE = crate::Reg<spi0_ctar_slave::SPI0_CTAR_SLAVE_SPEC>;
#[doc = "Clock and Transfer Attributes Register (In Slave Mode)"]
pub mod spi0_ctar_slave;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "RSER register accessor: an alias for `Reg<RSER_SPEC>`"]
pub type RSER = crate::Reg<rser::RSER_SPEC>;
#[doc = "DMA/Interrupt Request Select and Enable Register"]
pub mod rser;
#[doc = "SPI0_PUSHR register accessor: an alias for `Reg<SPI0_PUSHR_SPEC>`"]
pub type SPI0_PUSHR = crate::Reg<spi0_pushr::SPI0_PUSHR_SPEC>;
#[doc = "PUSH TX FIFO Register In Master Mode"]
pub mod spi0_pushr;
#[doc = "SPI0_PUSHR_SLAVE register accessor: an alias for `Reg<SPI0_PUSHR_SLAVE_SPEC>`"]
pub type SPI0_PUSHR_SLAVE = crate::Reg<spi0_pushr_slave::SPI0_PUSHR_SLAVE_SPEC>;
#[doc = "PUSH TX FIFO Register In Slave Mode"]
pub mod spi0_pushr_slave;
#[doc = "POPR register accessor: an alias for `Reg<POPR_SPEC>`"]
pub type POPR = crate::Reg<popr::POPR_SPEC>;
#[doc = "POP RX FIFO Register"]
pub mod popr;
#[doc = "TXFR register accessor: an alias for `Reg<TXFR_SPEC>`"]
pub type TXFR = crate::Reg<txfr::TXFR_SPEC>;
#[doc = "Transmit FIFO Registers"]
pub mod txfr;
#[doc = "RXFR register accessor: an alias for `Reg<RXFR_SPEC>`"]
pub type RXFR = crate::Reg<rxfr::RXFR_SPEC>;
#[doc = "Receive FIFO Registers"]
pub mod rxfr;
