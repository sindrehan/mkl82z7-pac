#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Privilege Register A"]
    pub mpra: crate::Reg<mpra::MPRA_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Peripheral Access Control Register"]
    pub pacra: crate::Reg<pacra::PACRA_SPEC>,
    #[doc = "0x24 - Peripheral Access Control Register"]
    pub pacrb: crate::Reg<pacrb::PACRB_SPEC>,
    #[doc = "0x28 - Peripheral Access Control Register"]
    pub pacrc: crate::Reg<pacrc::PACRC_SPEC>,
    #[doc = "0x2c - Peripheral Access Control Register"]
    pub pacrd: crate::Reg<pacrd::PACRD_SPEC>,
    _reserved5: [u8; 0x10],
    #[doc = "0x40 - Peripheral Access Control Register"]
    pub pacre: crate::Reg<pacre::PACRE_SPEC>,
    #[doc = "0x44 - Peripheral Access Control Register"]
    pub pacrf: crate::Reg<pacrf::PACRF_SPEC>,
    #[doc = "0x48 - Peripheral Access Control Register"]
    pub pacrg: crate::Reg<pacrg::PACRG_SPEC>,
    #[doc = "0x4c - Peripheral Access Control Register"]
    pub pacrh: crate::Reg<pacrh::PACRH_SPEC>,
    #[doc = "0x50 - Peripheral Access Control Register"]
    pub pacri: crate::Reg<pacri::PACRI_SPEC>,
    #[doc = "0x54 - Peripheral Access Control Register"]
    pub pacrj: crate::Reg<pacrj::PACRJ_SPEC>,
    #[doc = "0x58 - Peripheral Access Control Register"]
    pub pacrk: crate::Reg<pacrk::PACRK_SPEC>,
    #[doc = "0x5c - Peripheral Access Control Register"]
    pub pacrl: crate::Reg<pacrl::PACRL_SPEC>,
    #[doc = "0x60 - Peripheral Access Control Register"]
    pub pacrm: crate::Reg<pacrm::PACRM_SPEC>,
    #[doc = "0x64 - Peripheral Access Control Register"]
    pub pacrn: crate::Reg<pacrn::PACRN_SPEC>,
    #[doc = "0x68 - Peripheral Access Control Register"]
    pub pacro: crate::Reg<pacro::PACRO_SPEC>,
    #[doc = "0x6c - Peripheral Access Control Register"]
    pub pacrp: crate::Reg<pacrp::PACRP_SPEC>,
}
#[doc = "MPRA register accessor: an alias for `Reg<MPRA_SPEC>`"]
pub type MPRA = crate::Reg<mpra::MPRA_SPEC>;
#[doc = "Master Privilege Register A"]
pub mod mpra;
#[doc = "PACRA register accessor: an alias for `Reg<PACRA_SPEC>`"]
pub type PACRA = crate::Reg<pacra::PACRA_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacra;
#[doc = "PACRB register accessor: an alias for `Reg<PACRB_SPEC>`"]
pub type PACRB = crate::Reg<pacrb::PACRB_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrb;
#[doc = "PACRC register accessor: an alias for `Reg<PACRC_SPEC>`"]
pub type PACRC = crate::Reg<pacrc::PACRC_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrc;
#[doc = "PACRD register accessor: an alias for `Reg<PACRD_SPEC>`"]
pub type PACRD = crate::Reg<pacrd::PACRD_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrd;
#[doc = "PACRE register accessor: an alias for `Reg<PACRE_SPEC>`"]
pub type PACRE = crate::Reg<pacre::PACRE_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacre;
#[doc = "PACRF register accessor: an alias for `Reg<PACRF_SPEC>`"]
pub type PACRF = crate::Reg<pacrf::PACRF_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrf;
#[doc = "PACRG register accessor: an alias for `Reg<PACRG_SPEC>`"]
pub type PACRG = crate::Reg<pacrg::PACRG_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrg;
#[doc = "PACRH register accessor: an alias for `Reg<PACRH_SPEC>`"]
pub type PACRH = crate::Reg<pacrh::PACRH_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrh;
#[doc = "PACRI register accessor: an alias for `Reg<PACRI_SPEC>`"]
pub type PACRI = crate::Reg<pacri::PACRI_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacri;
#[doc = "PACRJ register accessor: an alias for `Reg<PACRJ_SPEC>`"]
pub type PACRJ = crate::Reg<pacrj::PACRJ_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrj;
#[doc = "PACRK register accessor: an alias for `Reg<PACRK_SPEC>`"]
pub type PACRK = crate::Reg<pacrk::PACRK_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrk;
#[doc = "PACRL register accessor: an alias for `Reg<PACRL_SPEC>`"]
pub type PACRL = crate::Reg<pacrl::PACRL_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrl;
#[doc = "PACRM register accessor: an alias for `Reg<PACRM_SPEC>`"]
pub type PACRM = crate::Reg<pacrm::PACRM_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrm;
#[doc = "PACRN register accessor: an alias for `Reg<PACRN_SPEC>`"]
pub type PACRN = crate::Reg<pacrn::PACRN_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrn;
#[doc = "PACRO register accessor: an alias for `Reg<PACRO_SPEC>`"]
pub type PACRO = crate::Reg<pacro::PACRO_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacro;
#[doc = "PACRP register accessor: an alias for `Reg<PACRP_SPEC>`"]
pub type PACRP = crate::Reg<pacrp::PACRP_SPEC>;
#[doc = "Peripheral Access Control Register"]
pub mod pacrp;
