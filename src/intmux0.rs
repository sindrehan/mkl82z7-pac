#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel n Control Status Register"]
    pub ch0_csr: crate::Reg<ch_csr::CH_CSR_SPEC>,
    #[doc = "0x04 - Channel n Vector Number Register"]
    pub ch0_vec: crate::Reg<ch_vec::CH_VEC_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Channel n Interrupt Enable Register"]
    pub ch0_ier_31_0: crate::Reg<ch_ier_31_0::CH_IER_31_0_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x20 - Channel n Interrupt Pending Register"]
    pub ch0_ipr_31_0: crate::Reg<ch_ipr_31_0::CH_IPR_31_0_SPEC>,
    _reserved4: [u8; 0x1c],
    #[doc = "0x40 - Channel n Control Status Register"]
    pub ch1_csr: crate::Reg<ch_csr::CH_CSR_SPEC>,
    #[doc = "0x44 - Channel n Vector Number Register"]
    pub ch1_vec: crate::Reg<ch_vec::CH_VEC_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x50 - Channel n Interrupt Enable Register"]
    pub ch1_ier_31_0: crate::Reg<ch_ier_31_0::CH_IER_31_0_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x60 - Channel n Interrupt Pending Register"]
    pub ch1_ipr_31_0: crate::Reg<ch_ipr_31_0::CH_IPR_31_0_SPEC>,
    _reserved8: [u8; 0x1c],
    #[doc = "0x80 - Channel n Control Status Register"]
    pub ch2_csr: crate::Reg<ch_csr::CH_CSR_SPEC>,
    #[doc = "0x84 - Channel n Vector Number Register"]
    pub ch2_vec: crate::Reg<ch_vec::CH_VEC_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x90 - Channel n Interrupt Enable Register"]
    pub ch2_ier_31_0: crate::Reg<ch_ier_31_0::CH_IER_31_0_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0xa0 - Channel n Interrupt Pending Register"]
    pub ch2_ipr_31_0: crate::Reg<ch_ipr_31_0::CH_IPR_31_0_SPEC>,
    _reserved12: [u8; 0x1c],
    #[doc = "0xc0 - Channel n Control Status Register"]
    pub ch3_csr: crate::Reg<ch_csr::CH_CSR_SPEC>,
    #[doc = "0xc4 - Channel n Vector Number Register"]
    pub ch3_vec: crate::Reg<ch_vec::CH_VEC_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0xd0 - Channel n Interrupt Enable Register"]
    pub ch3_ier_31_0: crate::Reg<ch_ier_31_0::CH_IER_31_0_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0xe0 - Channel n Interrupt Pending Register"]
    pub ch3_ipr_31_0: crate::Reg<ch_ipr_31_0::CH_IPR_31_0_SPEC>,
}
#[doc = "CH_CSR register accessor: an alias for `Reg<CH_CSR_SPEC>`"]
pub type CH_CSR = crate::Reg<ch_csr::CH_CSR_SPEC>;
#[doc = "Channel n Control Status Register"]
pub mod ch_csr;
#[doc = "CH_VEC register accessor: an alias for `Reg<CH_VEC_SPEC>`"]
pub type CH_VEC = crate::Reg<ch_vec::CH_VEC_SPEC>;
#[doc = "Channel n Vector Number Register"]
pub mod ch_vec;
#[doc = "CH_IER_31_0 register accessor: an alias for `Reg<CH_IER_31_0_SPEC>`"]
pub type CH_IER_31_0 = crate::Reg<ch_ier_31_0::CH_IER_31_0_SPEC>;
#[doc = "Channel n Interrupt Enable Register"]
pub mod ch_ier_31_0;
#[doc = "CH_IPR_31_0 register accessor: an alias for `Reg<CH_IPR_31_0_SPEC>`"]
pub type CH_IPR_31_0 = crate::Reg<ch_ipr_31_0::CH_IPR_31_0_SPEC>;
#[doc = "Channel n Interrupt Pending Register"]
pub mod ch_ipr_31_0;
