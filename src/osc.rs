#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSC Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - OSC_DIV"]
    pub div: crate::Reg<div::DIV_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "OSC Control Register"]
pub mod cr;
#[doc = "DIV register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "OSC_DIV"]
pub mod div;
