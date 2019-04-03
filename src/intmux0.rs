#[doc = r" Register block"]
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
#[doc = "Channel n Control Status Register"]
pub struct CH_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Control Status Register"]
pub mod ch_csr;
#[doc = "Channel n Vector Number Register"]
pub struct CH_VEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Vector Number Register"]
pub mod ch_vec;
#[doc = "Channel n Interrupt Enable Register"]
pub struct CH_IER_31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Interrupt Enable Register"]
pub mod ch_ier_31_0;
#[doc = "Channel n Interrupt Pending Register"]
pub struct CH_IPR_31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel n Interrupt Pending Register"]
pub mod ch_ipr_31_0;
