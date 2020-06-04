#[doc = r"Register block"]
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
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "IP Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr](ipcr) module"]
pub type IPCR = crate::Reg<u32, _IPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCR;
#[doc = "`read()` method returns [ipcr::R](ipcr::R) reader structure"]
impl crate::Readable for IPCR {}
#[doc = "`write(|w| ..)` method takes [ipcr::W](ipcr::W) writer structure"]
impl crate::Writable for IPCR {}
#[doc = "IP Configuration Register"]
pub mod ipcr;
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshcr](flshcr) module"]
pub type FLSHCR = crate::Reg<u32, _FLSHCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSHCR;
#[doc = "`read()` method returns [flshcr::R](flshcr::R) reader structure"]
impl crate::Readable for FLSHCR {}
#[doc = "`write(|w| ..)` method takes [flshcr::W](flshcr::W) writer structure"]
impl crate::Writable for FLSHCR {}
#[doc = "Flash Configuration Register"]
pub mod flshcr;
#[doc = "Buffer0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf0cr](buf0cr) module"]
pub type BUF0CR = crate::Reg<u32, _BUF0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF0CR;
#[doc = "`read()` method returns [buf0cr::R](buf0cr::R) reader structure"]
impl crate::Readable for BUF0CR {}
#[doc = "`write(|w| ..)` method takes [buf0cr::W](buf0cr::W) writer structure"]
impl crate::Writable for BUF0CR {}
#[doc = "Buffer0 Configuration Register"]
pub mod buf0cr;
#[doc = "Buffer1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf1cr](buf1cr) module"]
pub type BUF1CR = crate::Reg<u32, _BUF1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF1CR;
#[doc = "`read()` method returns [buf1cr::R](buf1cr::R) reader structure"]
impl crate::Readable for BUF1CR {}
#[doc = "`write(|w| ..)` method takes [buf1cr::W](buf1cr::W) writer structure"]
impl crate::Writable for BUF1CR {}
#[doc = "Buffer1 Configuration Register"]
pub mod buf1cr;
#[doc = "Buffer2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf2cr](buf2cr) module"]
pub type BUF2CR = crate::Reg<u32, _BUF2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF2CR;
#[doc = "`read()` method returns [buf2cr::R](buf2cr::R) reader structure"]
impl crate::Readable for BUF2CR {}
#[doc = "`write(|w| ..)` method takes [buf2cr::W](buf2cr::W) writer structure"]
impl crate::Writable for BUF2CR {}
#[doc = "Buffer2 Configuration Register"]
pub mod buf2cr;
#[doc = "Buffer3 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf3cr](buf3cr) module"]
pub type BUF3CR = crate::Reg<u32, _BUF3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF3CR;
#[doc = "`read()` method returns [buf3cr::R](buf3cr::R) reader structure"]
impl crate::Readable for BUF3CR {}
#[doc = "`write(|w| ..)` method takes [buf3cr::W](buf3cr::W) writer structure"]
impl crate::Writable for BUF3CR {}
#[doc = "Buffer3 Configuration Register"]
pub mod buf3cr;
#[doc = "Buffer Generic Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfgencr](bfgencr) module"]
pub type BFGENCR = crate::Reg<u32, _BFGENCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFGENCR;
#[doc = "`read()` method returns [bfgencr::R](bfgencr::R) reader structure"]
impl crate::Readable for BFGENCR {}
#[doc = "`write(|w| ..)` method takes [bfgencr::W](bfgencr::W) writer structure"]
impl crate::Writable for BFGENCR {}
#[doc = "Buffer Generic Configuration Register"]
pub mod bfgencr;
#[doc = "SOC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soccr](soccr) module"]
pub type SOCCR = crate::Reg<u32, _SOCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOCCR;
#[doc = "`read()` method returns [soccr::R](soccr::R) reader structure"]
impl crate::Readable for SOCCR {}
#[doc = "`write(|w| ..)` method takes [soccr::W](soccr::W) writer structure"]
impl crate::Writable for SOCCR {}
#[doc = "SOC Configuration Register"]
pub mod soccr;
#[doc = "Buffer0 Top Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf0ind](buf0ind) module"]
pub type BUF0IND = crate::Reg<u32, _BUF0IND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF0IND;
#[doc = "`read()` method returns [buf0ind::R](buf0ind::R) reader structure"]
impl crate::Readable for BUF0IND {}
#[doc = "`write(|w| ..)` method takes [buf0ind::W](buf0ind::W) writer structure"]
impl crate::Writable for BUF0IND {}
#[doc = "Buffer0 Top Index Register"]
pub mod buf0ind;
#[doc = "Buffer1 Top Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf1ind](buf1ind) module"]
pub type BUF1IND = crate::Reg<u32, _BUF1IND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF1IND;
#[doc = "`read()` method returns [buf1ind::R](buf1ind::R) reader structure"]
impl crate::Readable for BUF1IND {}
#[doc = "`write(|w| ..)` method takes [buf1ind::W](buf1ind::W) writer structure"]
impl crate::Writable for BUF1IND {}
#[doc = "Buffer1 Top Index Register"]
pub mod buf1ind;
#[doc = "Buffer2 Top Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf2ind](buf2ind) module"]
pub type BUF2IND = crate::Reg<u32, _BUF2IND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF2IND;
#[doc = "`read()` method returns [buf2ind::R](buf2ind::R) reader structure"]
impl crate::Readable for BUF2IND {}
#[doc = "`write(|w| ..)` method takes [buf2ind::W](buf2ind::W) writer structure"]
impl crate::Writable for BUF2IND {}
#[doc = "Buffer2 Top Index Register"]
pub mod buf2ind;
#[doc = "Serial Flash Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfar](sfar) module"]
pub type SFAR = crate::Reg<u32, _SFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFAR;
#[doc = "`read()` method returns [sfar::R](sfar::R) reader structure"]
impl crate::Readable for SFAR {}
#[doc = "`write(|w| ..)` method takes [sfar::W](sfar::W) writer structure"]
impl crate::Writable for SFAR {}
#[doc = "Serial Flash Address Register"]
pub mod sfar;
#[doc = "Serial Flash Address Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfacr](sfacr) module"]
pub type SFACR = crate::Reg<u32, _SFACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFACR;
#[doc = "`read()` method returns [sfacr::R](sfacr::R) reader structure"]
impl crate::Readable for SFACR {}
#[doc = "`write(|w| ..)` method takes [sfacr::W](sfacr::W) writer structure"]
impl crate::Writable for SFACR {}
#[doc = "Serial Flash Address Configuration Register"]
pub mod sfacr;
#[doc = "Sampling Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](smpr) module"]
pub type SMPR = crate::Reg<u32, _SMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR;
#[doc = "`read()` method returns [smpr::R](smpr::R) reader structure"]
impl crate::Readable for SMPR {}
#[doc = "`write(|w| ..)` method takes [smpr::W](smpr::W) writer structure"]
impl crate::Writable for SMPR {}
#[doc = "Sampling Register"]
pub mod smpr;
#[doc = "RX Buffer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbsr](rbsr) module"]
pub type RBSR = crate::Reg<u32, _RBSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBSR;
#[doc = "`read()` method returns [rbsr::R](rbsr::R) reader structure"]
impl crate::Readable for RBSR {}
#[doc = "RX Buffer Status Register"]
pub mod rbsr;
#[doc = "RX Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbct](rbct) module"]
pub type RBCT = crate::Reg<u32, _RBCT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBCT;
#[doc = "`read()` method returns [rbct::R](rbct::R) reader structure"]
impl crate::Readable for RBCT {}
#[doc = "`write(|w| ..)` method takes [rbct::W](rbct::W) writer structure"]
impl crate::Writable for RBCT {}
#[doc = "RX Buffer Control Register"]
pub mod rbct;
#[doc = "TX Buffer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbsr](tbsr) module"]
pub type TBSR = crate::Reg<u32, _TBSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBSR;
#[doc = "`read()` method returns [tbsr::R](tbsr::R) reader structure"]
impl crate::Readable for TBSR {}
#[doc = "TX Buffer Status Register"]
pub mod tbsr;
#[doc = "TX Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbdr](tbdr) module"]
pub type TBDR = crate::Reg<u32, _TBDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBDR;
#[doc = "`read()` method returns [tbdr::R](tbdr::R) reader structure"]
impl crate::Readable for TBDR {}
#[doc = "`write(|w| ..)` method takes [tbdr::W](tbdr::W) writer structure"]
impl crate::Writable for TBDR {}
#[doc = "TX Buffer Data Register"]
pub mod tbdr;
#[doc = "Tx Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbct](tbct) module"]
pub type TBCT = crate::Reg<u32, _TBCT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBCT;
#[doc = "`read()` method returns [tbct::R](tbct::R) reader structure"]
impl crate::Readable for TBCT {}
#[doc = "`write(|w| ..)` method takes [tbct::W](tbct::W) writer structure"]
impl crate::Writable for TBCT {}
#[doc = "Tx Buffer Control Register"]
pub mod tbct;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "`write(|w| ..)` method takes [fr::W](fr::W) writer structure"]
impl crate::Writable for FR {}
#[doc = "Flag Register"]
pub mod fr;
#[doc = "Interrupt and DMA Request Select and Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rser](rser) module"]
pub type RSER = crate::Reg<u32, _RSER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSER;
#[doc = "`read()` method returns [rser::R](rser::R) reader structure"]
impl crate::Readable for RSER {}
#[doc = "`write(|w| ..)` method takes [rser::W](rser::W) writer structure"]
impl crate::Writable for RSER {}
#[doc = "Interrupt and DMA Request Select and Enable Register"]
pub mod rser;
#[doc = "Sequence Suspend Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spndst](spndst) module"]
pub type SPNDST = crate::Reg<u32, _SPNDST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPNDST;
#[doc = "`read()` method returns [spndst::R](spndst::R) reader structure"]
impl crate::Readable for SPNDST {}
#[doc = "Sequence Suspend Status Register"]
pub mod spndst;
#[doc = "Sequence Pointer Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptrclr](sptrclr) module"]
pub type SPTRCLR = crate::Reg<u32, _SPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPTRCLR;
#[doc = "`read()` method returns [sptrclr::R](sptrclr::R) reader structure"]
impl crate::Readable for SPTRCLR {}
#[doc = "`write(|w| ..)` method takes [sptrclr::W](sptrclr::W) writer structure"]
impl crate::Writable for SPTRCLR {}
#[doc = "Sequence Pointer Clear Register"]
pub mod sptrclr;
#[doc = "Serial Flash A1 Top Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfa1ad](sfa1ad) module"]
pub type SFA1AD = crate::Reg<u32, _SFA1AD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFA1AD;
#[doc = "`read()` method returns [sfa1ad::R](sfa1ad::R) reader structure"]
impl crate::Readable for SFA1AD {}
#[doc = "`write(|w| ..)` method takes [sfa1ad::W](sfa1ad::W) writer structure"]
impl crate::Writable for SFA1AD {}
#[doc = "Serial Flash A1 Top Address"]
pub mod sfa1ad;
#[doc = "Serial Flash A2 Top Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfa2ad](sfa2ad) module"]
pub type SFA2AD = crate::Reg<u32, _SFA2AD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFA2AD;
#[doc = "`read()` method returns [sfa2ad::R](sfa2ad::R) reader structure"]
impl crate::Readable for SFA2AD {}
#[doc = "`write(|w| ..)` method takes [sfa2ad::W](sfa2ad::W) writer structure"]
impl crate::Writable for SFA2AD {}
#[doc = "Serial Flash A2 Top Address"]
pub mod sfa2ad;
#[doc = "Serial Flash B1Top Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfb1ad](sfb1ad) module"]
pub type SFB1AD = crate::Reg<u32, _SFB1AD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFB1AD;
#[doc = "`read()` method returns [sfb1ad::R](sfb1ad::R) reader structure"]
impl crate::Readable for SFB1AD {}
#[doc = "`write(|w| ..)` method takes [sfb1ad::W](sfb1ad::W) writer structure"]
impl crate::Writable for SFB1AD {}
#[doc = "Serial Flash B1Top Address"]
pub mod sfb1ad;
#[doc = "Serial Flash B2Top Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfb2ad](sfb2ad) module"]
pub type SFB2AD = crate::Reg<u32, _SFB2AD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFB2AD;
#[doc = "`read()` method returns [sfb2ad::R](sfb2ad::R) reader structure"]
impl crate::Readable for SFB2AD {}
#[doc = "`write(|w| ..)` method takes [sfb2ad::W](sfb2ad::W) writer structure"]
impl crate::Writable for SFB2AD {}
#[doc = "Serial Flash B2Top Address"]
pub mod sfb2ad;
#[doc = "Data Learn Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlpr](dlpr) module"]
pub type DLPR = crate::Reg<u32, _DLPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLPR;
#[doc = "`read()` method returns [dlpr::R](dlpr::R) reader structure"]
impl crate::Readable for DLPR {}
#[doc = "`write(|w| ..)` method takes [dlpr::W](dlpr::W) writer structure"]
impl crate::Writable for DLPR {}
#[doc = "Data Learn Pattern Register"]
pub mod dlpr;
#[doc = "RX Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbdr](rbdr) module"]
pub type RBDR = crate::Reg<u32, _RBDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBDR;
#[doc = "`read()` method returns [rbdr::R](rbdr::R) reader structure"]
impl crate::Readable for RBDR {}
#[doc = "RX Buffer Data Register"]
pub mod rbdr;
#[doc = "LUT Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutkey](lutkey) module"]
pub type LUTKEY = crate::Reg<u32, _LUTKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUTKEY;
#[doc = "`read()` method returns [lutkey::R](lutkey::R) reader structure"]
impl crate::Readable for LUTKEY {}
#[doc = "`write(|w| ..)` method takes [lutkey::W](lutkey::W) writer structure"]
impl crate::Writable for LUTKEY {}
#[doc = "LUT Key Register"]
pub mod lutkey;
#[doc = "LUT Lock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckcr](lckcr) module"]
pub type LCKCR = crate::Reg<u32, _LCKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCKCR;
#[doc = "`read()` method returns [lckcr::R](lckcr::R) reader structure"]
impl crate::Readable for LCKCR {}
#[doc = "`write(|w| ..)` method takes [lckcr::W](lckcr::W) writer structure"]
impl crate::Writable for LCKCR {}
#[doc = "LUT Lock Configuration Register"]
pub mod lckcr;
#[doc = "Look-up Table register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut](lut) module"]
pub type LUT = crate::Reg<u32, _LUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT;
#[doc = "`read()` method returns [lut::R](lut::R) reader structure"]
impl crate::Readable for LUT {}
#[doc = "`write(|w| ..)` method takes [lut::W](lut::W) writer structure"]
impl crate::Writable for LUT {}
#[doc = "Look-up Table register"]
pub mod lut;
