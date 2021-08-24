#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status Register 0"]
    pub srs0: crate::Reg<srs0::SRS0_SPEC>,
    #[doc = "0x01 - System Reset Status Register 1"]
    pub srs1: crate::Reg<srs1::SRS1_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x04 - Reset Pin Filter Control register"]
    pub rpfc: crate::Reg<rpfc::RPFC_SPEC>,
    #[doc = "0x05 - Reset Pin Filter Width register"]
    pub rpfw: crate::Reg<rpfw::RPFW_SPEC>,
    #[doc = "0x06 - Force Mode Register"]
    pub fm: crate::Reg<fm::FM_SPEC>,
    #[doc = "0x07 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x08 - Sticky System Reset Status Register 0"]
    pub ssrs0: crate::Reg<ssrs0::SSRS0_SPEC>,
    #[doc = "0x09 - Sticky System Reset Status Register 1"]
    pub ssrs1: crate::Reg<ssrs1::SSRS1_SPEC>,
}
#[doc = "SRS0 register accessor: an alias for `Reg<SRS0_SPEC>`"]
pub type SRS0 = crate::Reg<srs0::SRS0_SPEC>;
#[doc = "System Reset Status Register 0"]
pub mod srs0;
#[doc = "SRS1 register accessor: an alias for `Reg<SRS1_SPEC>`"]
pub type SRS1 = crate::Reg<srs1::SRS1_SPEC>;
#[doc = "System Reset Status Register 1"]
pub mod srs1;
#[doc = "RPFC register accessor: an alias for `Reg<RPFC_SPEC>`"]
pub type RPFC = crate::Reg<rpfc::RPFC_SPEC>;
#[doc = "Reset Pin Filter Control register"]
pub mod rpfc;
#[doc = "RPFW register accessor: an alias for `Reg<RPFW_SPEC>`"]
pub type RPFW = crate::Reg<rpfw::RPFW_SPEC>;
#[doc = "Reset Pin Filter Width register"]
pub mod rpfw;
#[doc = "FM register accessor: an alias for `Reg<FM_SPEC>`"]
pub type FM = crate::Reg<fm::FM_SPEC>;
#[doc = "Force Mode Register"]
pub mod fm;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SSRS0 register accessor: an alias for `Reg<SSRS0_SPEC>`"]
pub type SSRS0 = crate::Reg<ssrs0::SSRS0_SPEC>;
#[doc = "Sticky System Reset Status Register 0"]
pub mod ssrs0;
#[doc = "SSRS1 register accessor: an alias for `Reg<SSRS1_SPEC>`"]
pub type SSRS1 = crate::Reg<ssrs1::SSRS1_SPEC>;
#[doc = "Sticky System Reset Status Register 1"]
pub mod ssrs1;
