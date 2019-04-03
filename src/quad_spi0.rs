#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - IP Configuration Register"]
    pub ipcr: IPCR,
    #[doc = "0x0c - Flash Configuration Register"]
    pub flshcr: FLSHCR,
    #[doc = "0x10 - Buffer0 Configuration Register"]
    pub buf0cr: BUF0CR,
    #[doc = "0x14 - Buffer1 Configuration Register"]
    pub buf1cr: BUF1CR,
    #[doc = "0x18 - Buffer2 Configuration Register"]
    pub buf2cr: BUF2CR,
    #[doc = "0x1c - Buffer3 Configuration Register"]
    pub buf3cr: BUF3CR,
    #[doc = "0x20 - Buffer Generic Configuration Register"]
    pub bfgencr: BFGENCR,
    #[doc = "0x24 - SOC Configuration Register"]
    pub soccr: SOCCR,
    _reserved9: [u8; 8usize],
    #[doc = "0x30 - Buffer0 Top Index Register"]
    pub buf0ind: BUF0IND,
    #[doc = "0x34 - Buffer1 Top Index Register"]
    pub buf1ind: BUF1IND,
    #[doc = "0x38 - Buffer2 Top Index Register"]
    pub buf2ind: BUF2IND,
    _reserved12: [u8; 196usize],
    #[doc = "0x100 - Serial Flash Address Register"]
    pub sfar: SFAR,
    #[doc = "0x104 - Serial Flash Address Configuration Register"]
    pub sfacr: SFACR,
    #[doc = "0x108 - Sampling Register"]
    pub smpr: SMPR,
    #[doc = "0x10c - RX Buffer Status Register"]
    pub rbsr: RBSR,
    #[doc = "0x110 - RX Buffer Control Register"]
    pub rbct: RBCT,
    _reserved17: [u8; 60usize],
    #[doc = "0x150 - TX Buffer Status Register"]
    pub tbsr: TBSR,
    #[doc = "0x154 - TX Buffer Data Register"]
    pub tbdr: TBDR,
    #[doc = "0x158 - Tx Buffer Control Register"]
    pub tbct: TBCT,
    #[doc = "0x15c - Status Register"]
    pub sr: SR,
    #[doc = "0x160 - Flag Register"]
    pub fr: FR,
    #[doc = "0x164 - Interrupt and DMA Request Select and Enable Register"]
    pub rser: RSER,
    #[doc = "0x168 - Sequence Suspend Status Register"]
    pub spndst: SPNDST,
    #[doc = "0x16c - Sequence Pointer Clear Register"]
    pub sptrclr: SPTRCLR,
    _reserved25: [u8; 16usize],
    #[doc = "0x180 - Serial Flash A1 Top Address"]
    pub sfa1ad: SFA1AD,
    #[doc = "0x184 - Serial Flash A2 Top Address"]
    pub sfa2ad: SFA2AD,
    #[doc = "0x188 - Serial Flash B1Top Address"]
    pub sfb1ad: SFB1AD,
    #[doc = "0x18c - Serial Flash B2Top Address"]
    pub sfb2ad: SFB2AD,
    #[doc = "0x190 - Data Learn Pattern Register"]
    pub dlpr: DLPR,
    _reserved30: [u8; 108usize],
    #[doc = "0x200 - RX Buffer Data Register"]
    pub rbdr: [RBDR; 16],
    _reserved31: [u8; 192usize],
    #[doc = "0x300 - LUT Key Register"]
    pub lutkey: LUTKEY,
    #[doc = "0x304 - LUT Lock Configuration Register"]
    pub lckcr: LCKCR,
    _reserved33: [u8; 8usize],
    #[doc = "0x310 - Look-up Table register"]
    pub lut: [LUT; 64],
}
#[doc = "Module Configuration Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "IP Configuration Register"]
pub struct IPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Configuration Register"]
pub mod ipcr;
#[doc = "Flash Configuration Register"]
pub struct FLSHCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Configuration Register"]
pub mod flshcr;
#[doc = "Buffer0 Configuration Register"]
pub struct BUF0CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer0 Configuration Register"]
pub mod buf0cr;
#[doc = "Buffer1 Configuration Register"]
pub struct BUF1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer1 Configuration Register"]
pub mod buf1cr;
#[doc = "Buffer2 Configuration Register"]
pub struct BUF2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer2 Configuration Register"]
pub mod buf2cr;
#[doc = "Buffer3 Configuration Register"]
pub struct BUF3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer3 Configuration Register"]
pub mod buf3cr;
#[doc = "Buffer Generic Configuration Register"]
pub struct BFGENCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Generic Configuration Register"]
pub mod bfgencr;
#[doc = "SOC Configuration Register"]
pub struct SOCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SOC Configuration Register"]
pub mod soccr;
#[doc = "Buffer0 Top Index Register"]
pub struct BUF0IND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer0 Top Index Register"]
pub mod buf0ind;
#[doc = "Buffer1 Top Index Register"]
pub struct BUF1IND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer1 Top Index Register"]
pub mod buf1ind;
#[doc = "Buffer2 Top Index Register"]
pub struct BUF2IND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer2 Top Index Register"]
pub mod buf2ind;
#[doc = "Serial Flash Address Register"]
pub struct SFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Flash Address Register"]
pub mod sfar;
#[doc = "Serial Flash Address Configuration Register"]
pub struct SFACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Flash Address Configuration Register"]
pub mod sfacr;
#[doc = "Sampling Register"]
pub struct SMPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sampling Register"]
pub mod smpr;
#[doc = "RX Buffer Status Register"]
pub struct RBSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Status Register"]
pub mod rbsr;
#[doc = "RX Buffer Control Register"]
pub struct RBCT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Control Register"]
pub mod rbct;
#[doc = "TX Buffer Status Register"]
pub struct TBSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Buffer Status Register"]
pub mod tbsr;
#[doc = "TX Buffer Data Register"]
pub struct TBDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Buffer Data Register"]
pub mod tbdr;
#[doc = "Tx Buffer Control Register"]
pub struct TBCT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Control Register"]
pub mod tbct;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Flag Register"]
pub struct FR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flag Register"]
pub mod fr;
#[doc = "Interrupt and DMA Request Select and Enable Register"]
pub struct RSER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt and DMA Request Select and Enable Register"]
pub mod rser;
#[doc = "Sequence Suspend Status Register"]
pub struct SPNDST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Suspend Status Register"]
pub mod spndst;
#[doc = "Sequence Pointer Clear Register"]
pub struct SPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sequence Pointer Clear Register"]
pub mod sptrclr;
#[doc = "Serial Flash A1 Top Address"]
pub struct SFA1AD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Flash A1 Top Address"]
pub mod sfa1ad;
#[doc = "Serial Flash A2 Top Address"]
pub struct SFA2AD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Flash A2 Top Address"]
pub mod sfa2ad;
#[doc = "Serial Flash B1Top Address"]
pub struct SFB1AD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Flash B1Top Address"]
pub mod sfb1ad;
#[doc = "Serial Flash B2Top Address"]
pub struct SFB2AD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Flash B2Top Address"]
pub mod sfb2ad;
#[doc = "Data Learn Pattern Register"]
pub struct DLPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Learn Pattern Register"]
pub mod dlpr;
#[doc = "RX Buffer Data Register"]
pub struct RBDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Data Register"]
pub mod rbdr;
#[doc = "LUT Key Register"]
pub struct LUTKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Key Register"]
pub mod lutkey;
#[doc = "LUT Lock Configuration Register"]
pub struct LCKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Lock Configuration Register"]
pub mod lckcr;
#[doc = "Look-up Table register"]
pub struct LUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Look-up Table register"]
pub mod lut;
