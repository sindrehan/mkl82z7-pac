#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x04 - Counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x08 - Modulo"]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    #[doc = "0x0c - Channel (n) Status and Control"]
    pub c0sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x14 - Channel (n) Status and Control"]
    pub c1sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x1c - Channel (n) Status and Control"]
    pub c2sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x20 - Channel (n) Value"]
    pub c2v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x24 - Channel (n) Status and Control"]
    pub c3sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x28 - Channel (n) Value"]
    pub c3v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x2c - Channel (n) Status and Control"]
    pub c4sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x30 - Channel (n) Value"]
    pub c4v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x34 - Channel (n) Status and Control"]
    pub c5sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x38 - Channel (n) Value"]
    pub c5v: crate::Reg<cv::CV_SPEC>,
    _reserved15: [u8; 0x14],
    #[doc = "0x50 - Capture and Compare Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved16: [u8; 0x10],
    #[doc = "0x64 - Combine Channel Register"]
    pub combine: crate::Reg<combine::COMBINE_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0x70 - Channel Polarity"]
    pub pol: crate::Reg<pol::POL_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x78 - Filter Control"]
    pub filter: crate::Reg<filter::FILTER_SPEC>,
    _reserved19: [u8; 0x08],
    #[doc = "0x84 - Configuration"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
}
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Status and Control"]
pub mod sc;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "MOD register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Modulo"]
pub mod mod_;
#[doc = "CSC register accessor: an alias for `Reg<CSC_SPEC>`"]
pub type CSC = crate::Reg<csc::CSC_SPEC>;
#[doc = "Channel (n) Status and Control"]
pub mod csc;
#[doc = "CV register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Channel (n) Value"]
pub mod cv;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Capture and Compare Status"]
pub mod status;
#[doc = "COMBINE register accessor: an alias for `Reg<COMBINE_SPEC>`"]
pub type COMBINE = crate::Reg<combine::COMBINE_SPEC>;
#[doc = "Combine Channel Register"]
pub mod combine;
#[doc = "POL register accessor: an alias for `Reg<POL_SPEC>`"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "Channel Polarity"]
pub mod pol;
#[doc = "FILTER register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Filter Control"]
pub mod filter;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configuration"]
pub mod conf;
