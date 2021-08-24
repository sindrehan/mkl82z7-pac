#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - VBAT register file register"]
    pub reg: [crate::Reg<reg::REG_SPEC>; 8],
}
#[doc = "REG register accessor: an alias for `Reg<REG_SPEC>`"]
pub type REG = crate::Reg<reg::REG_SPEC>;
#[doc = "VBAT register file register"]
pub mod reg;
