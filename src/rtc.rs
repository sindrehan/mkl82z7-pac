#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Time Seconds Register"]
    pub tsr: crate::Reg<tsr::TSR_SPEC>,
    #[doc = "0x04 - RTC Time Prescaler Register"]
    pub tpr: crate::Reg<tpr::TPR_SPEC>,
    #[doc = "0x08 - RTC Time Alarm Register"]
    pub tar: crate::Reg<tar::TAR_SPEC>,
    #[doc = "0x0c - RTC Time Compensation Register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x10 - RTC Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x14 - RTC Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x18 - RTC Lock Register"]
    pub lr: crate::Reg<lr::LR_SPEC>,
    #[doc = "0x1c - RTC Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    _reserved8: [u8; 0x07e0],
    #[doc = "0x800 - RTC Write Access Register"]
    pub war: crate::Reg<war::WAR_SPEC>,
    #[doc = "0x804 - RTC Read Access Register"]
    pub rar: crate::Reg<rar::RAR_SPEC>,
}
#[doc = "TSR register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "RTC Time Seconds Register"]
pub mod tsr;
#[doc = "TPR register accessor: an alias for `Reg<TPR_SPEC>`"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "RTC Time Prescaler Register"]
pub mod tpr;
#[doc = "TAR register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "RTC Time Alarm Register"]
pub mod tar;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "RTC Time Compensation Register"]
pub mod tcr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RTC Control Register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "RTC Status Register"]
pub mod sr;
#[doc = "LR register accessor: an alias for `Reg<LR_SPEC>`"]
pub type LR = crate::Reg<lr::LR_SPEC>;
#[doc = "RTC Lock Register"]
pub mod lr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "RTC Interrupt Enable Register"]
pub mod ier;
#[doc = "WAR register accessor: an alias for `Reg<WAR_SPEC>`"]
pub type WAR = crate::Reg<war::WAR_SPEC>;
#[doc = "RTC Write Access Register"]
pub mod war;
#[doc = "RAR register accessor: an alias for `Reg<RAR_SPEC>`"]
pub type RAR = crate::Reg<rar::RAR_SPEC>;
#[doc = "RTC Read Access Register"]
pub mod rar;
