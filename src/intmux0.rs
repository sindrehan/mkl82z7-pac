#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel n Control Status Register"]
    pub ch0_csr: CH_CSR,
    #[doc = "0x04 - Channel n Vector Number Register"]
    pub ch0_vec: CH_VEC,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Channel n Interrupt Enable Register"]
    pub ch0_ier_31_0: CH_IER_31_0,
    _reserved3: [u8; 12usize],
    #[doc = "0x20 - Channel n Interrupt Pending Register"]
    pub ch0_ipr_31_0: CH_IPR_31_0,
    _reserved4: [u8; 28usize],
    #[doc = "0x40 - Channel n Control Status Register"]
    pub ch1_csr: CH_CSR,
    #[doc = "0x44 - Channel n Vector Number Register"]
    pub ch1_vec: CH_VEC,
    _reserved6: [u8; 8usize],
    #[doc = "0x50 - Channel n Interrupt Enable Register"]
    pub ch1_ier_31_0: CH_IER_31_0,
    _reserved7: [u8; 12usize],
    #[doc = "0x60 - Channel n Interrupt Pending Register"]
    pub ch1_ipr_31_0: CH_IPR_31_0,
    _reserved8: [u8; 28usize],
    #[doc = "0x80 - Channel n Control Status Register"]
    pub ch2_csr: CH_CSR,
    #[doc = "0x84 - Channel n Vector Number Register"]
    pub ch2_vec: CH_VEC,
    _reserved10: [u8; 8usize],
    #[doc = "0x90 - Channel n Interrupt Enable Register"]
    pub ch2_ier_31_0: CH_IER_31_0,
    _reserved11: [u8; 12usize],
    #[doc = "0xa0 - Channel n Interrupt Pending Register"]
    pub ch2_ipr_31_0: CH_IPR_31_0,
    _reserved12: [u8; 28usize],
    #[doc = "0xc0 - Channel n Control Status Register"]
    pub ch3_csr: CH_CSR,
    #[doc = "0xc4 - Channel n Vector Number Register"]
    pub ch3_vec: CH_VEC,
    _reserved14: [u8; 8usize],
    #[doc = "0xd0 - Channel n Interrupt Enable Register"]
    pub ch3_ier_31_0: CH_IER_31_0,
    _reserved15: [u8; 12usize],
    #[doc = "0xe0 - Channel n Interrupt Pending Register"]
    pub ch3_ipr_31_0: CH_IPR_31_0,
}
#[doc = "Channel n Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_csr](ch_csr) module"]
pub type CH_CSR = crate::Reg<u32, _CH_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_CSR;
#[doc = "`read()` method returns [ch_csr::R](ch_csr::R) reader structure"]
impl crate::Readable for CH_CSR {}
#[doc = "`write(|w| ..)` method takes [ch_csr::W](ch_csr::W) writer structure"]
impl crate::Writable for CH_CSR {}
#[doc = "Channel n Control Status Register"]
pub mod ch_csr;
#[doc = "Channel n Vector Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_vec](ch_vec) module"]
pub type CH_VEC = crate::Reg<u32, _CH_VEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_VEC;
#[doc = "`read()` method returns [ch_vec::R](ch_vec::R) reader structure"]
impl crate::Readable for CH_VEC {}
#[doc = "Channel n Vector Number Register"]
pub mod ch_vec;
#[doc = "Channel n Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ier_31_0](ch_ier_31_0) module"]
pub type CH_IER_31_0 = crate::Reg<u32, _CH_IER_31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_IER_31_0;
#[doc = "`read()` method returns [ch_ier_31_0::R](ch_ier_31_0::R) reader structure"]
impl crate::Readable for CH_IER_31_0 {}
#[doc = "`write(|w| ..)` method takes [ch_ier_31_0::W](ch_ier_31_0::W) writer structure"]
impl crate::Writable for CH_IER_31_0 {}
#[doc = "Channel n Interrupt Enable Register"]
pub mod ch_ier_31_0;
#[doc = "Channel n Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ipr_31_0](ch_ipr_31_0) module"]
pub type CH_IPR_31_0 = crate::Reg<u32, _CH_IPR_31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_IPR_31_0;
#[doc = "`read()` method returns [ch_ipr_31_0::R](ch_ipr_31_0::R) reader structure"]
impl crate::Readable for CH_IPR_31_0 {}
#[doc = "Channel n Interrupt Pending Register"]
pub mod ch_ipr_31_0;
