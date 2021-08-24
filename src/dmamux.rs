#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x08 - Channel Configuration register"]
    pub chcfg: [crate::Reg<chcfg::CHCFG_SPEC>; 8],
}
#[doc = "CHCFG register accessor: an alias for `Reg<CHCFG_SPEC>`"]
pub type CHCFG = crate::Reg<chcfg::CHCFG_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg;
