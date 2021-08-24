#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Options Register 1"]
    pub sopt1: crate::Reg<sopt1::SOPT1_SPEC>,
    _reserved1: [u8; 0x1000],
    #[doc = "0x1004 - System Options Register 2"]
    pub sopt2: crate::Reg<sopt2::SOPT2_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x1010 - System Options Register 5"]
    pub sopt5: crate::Reg<sopt5::SOPT5_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x1018 - System Options Register 7"]
    pub sopt7: crate::Reg<sopt7::SOPT7_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x1020 - System Options Register 9"]
    pub sopt9: crate::Reg<sopt9::SOPT9_SPEC>,
    #[doc = "0x1024 - System Device Identification Register"]
    pub sdid: crate::Reg<sdid::SDID_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0x1034 - System Clock Gating Control Register 4"]
    pub scgc4: crate::Reg<scgc4::SCGC4_SPEC>,
    #[doc = "0x1038 - System Clock Gating Control Register 5"]
    pub scgc5: crate::Reg<scgc5::SCGC5_SPEC>,
    #[doc = "0x103c - System Clock Gating Control Register 6"]
    pub scgc6: crate::Reg<scgc6::SCGC6_SPEC>,
    #[doc = "0x1040 - System Clock Gating Control Register 7"]
    pub scgc7: crate::Reg<scgc7::SCGC7_SPEC>,
    #[doc = "0x1044 - System Clock Divider Register 1"]
    pub clkdiv1: crate::Reg<clkdiv1::CLKDIV1_SPEC>,
    #[doc = "0x1048 - System Clock Divider Register 2"]
    pub clkdiv2: crate::Reg<clkdiv2::CLKDIV2_SPEC>,
    #[doc = "0x104c - Flash Configuration Register 1"]
    pub fcfg1: crate::Reg<fcfg1::FCFG1_SPEC>,
    #[doc = "0x1050 - Flash Configuration Register 2"]
    pub fcfg2: crate::Reg<fcfg2::FCFG2_SPEC>,
    #[doc = "0x1054 - Unique Identification Register High"]
    pub uidh: crate::Reg<uidh::UIDH_SPEC>,
    #[doc = "0x1058 - Unique Identification Register Mid-High"]
    pub uidmh: crate::Reg<uidmh::UIDMH_SPEC>,
    #[doc = "0x105c - Unique Identification Register Mid Low"]
    pub uidml: crate::Reg<uidml::UIDML_SPEC>,
    #[doc = "0x1060 - Unique Identification Register Low"]
    pub uidl: crate::Reg<uidl::UIDL_SPEC>,
    #[doc = "0x1064 - System Clock Divider Register 3"]
    pub clkdiv3: crate::Reg<clkdiv3::CLKDIV3_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x106c - Misc Control Register"]
    pub miscctrl: crate::Reg<miscctrl::MISCCTRL_SPEC>,
    _reserved20: [u8; 0x20],
    #[doc = "0x1090 - Secure Key Register 0"]
    pub seckey0: crate::Reg<seckey0::SECKEY0_SPEC>,
    #[doc = "0x1094 - Secure Key Register 1"]
    pub seckey1: crate::Reg<seckey1::SECKEY1_SPEC>,
    #[doc = "0x1098 - Secure Key Register 2"]
    pub seckey2: crate::Reg<seckey2::SECKEY2_SPEC>,
    #[doc = "0x109c - Secure Key Register 3"]
    pub seckey3: crate::Reg<seckey3::SECKEY3_SPEC>,
}
#[doc = "SOPT1 register accessor: an alias for `Reg<SOPT1_SPEC>`"]
pub type SOPT1 = crate::Reg<sopt1::SOPT1_SPEC>;
#[doc = "System Options Register 1"]
pub mod sopt1;
#[doc = "SOPT2 register accessor: an alias for `Reg<SOPT2_SPEC>`"]
pub type SOPT2 = crate::Reg<sopt2::SOPT2_SPEC>;
#[doc = "System Options Register 2"]
pub mod sopt2;
#[doc = "SOPT5 register accessor: an alias for `Reg<SOPT5_SPEC>`"]
pub type SOPT5 = crate::Reg<sopt5::SOPT5_SPEC>;
#[doc = "System Options Register 5"]
pub mod sopt5;
#[doc = "SOPT7 register accessor: an alias for `Reg<SOPT7_SPEC>`"]
pub type SOPT7 = crate::Reg<sopt7::SOPT7_SPEC>;
#[doc = "System Options Register 7"]
pub mod sopt7;
#[doc = "SOPT9 register accessor: an alias for `Reg<SOPT9_SPEC>`"]
pub type SOPT9 = crate::Reg<sopt9::SOPT9_SPEC>;
#[doc = "System Options Register 9"]
pub mod sopt9;
#[doc = "SDID register accessor: an alias for `Reg<SDID_SPEC>`"]
pub type SDID = crate::Reg<sdid::SDID_SPEC>;
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "SCGC4 register accessor: an alias for `Reg<SCGC4_SPEC>`"]
pub type SCGC4 = crate::Reg<scgc4::SCGC4_SPEC>;
#[doc = "System Clock Gating Control Register 4"]
pub mod scgc4;
#[doc = "SCGC5 register accessor: an alias for `Reg<SCGC5_SPEC>`"]
pub type SCGC5 = crate::Reg<scgc5::SCGC5_SPEC>;
#[doc = "System Clock Gating Control Register 5"]
pub mod scgc5;
#[doc = "SCGC6 register accessor: an alias for `Reg<SCGC6_SPEC>`"]
pub type SCGC6 = crate::Reg<scgc6::SCGC6_SPEC>;
#[doc = "System Clock Gating Control Register 6"]
pub mod scgc6;
#[doc = "SCGC7 register accessor: an alias for `Reg<SCGC7_SPEC>`"]
pub type SCGC7 = crate::Reg<scgc7::SCGC7_SPEC>;
#[doc = "System Clock Gating Control Register 7"]
pub mod scgc7;
#[doc = "CLKDIV1 register accessor: an alias for `Reg<CLKDIV1_SPEC>`"]
pub type CLKDIV1 = crate::Reg<clkdiv1::CLKDIV1_SPEC>;
#[doc = "System Clock Divider Register 1"]
pub mod clkdiv1;
#[doc = "CLKDIV2 register accessor: an alias for `Reg<CLKDIV2_SPEC>`"]
pub type CLKDIV2 = crate::Reg<clkdiv2::CLKDIV2_SPEC>;
#[doc = "System Clock Divider Register 2"]
pub mod clkdiv2;
#[doc = "FCFG1 register accessor: an alias for `Reg<FCFG1_SPEC>`"]
pub type FCFG1 = crate::Reg<fcfg1::FCFG1_SPEC>;
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "FCFG2 register accessor: an alias for `Reg<FCFG2_SPEC>`"]
pub type FCFG2 = crate::Reg<fcfg2::FCFG2_SPEC>;
#[doc = "Flash Configuration Register 2"]
pub mod fcfg2;
#[doc = "UIDH register accessor: an alias for `Reg<UIDH_SPEC>`"]
pub type UIDH = crate::Reg<uidh::UIDH_SPEC>;
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "UIDMH register accessor: an alias for `Reg<UIDMH_SPEC>`"]
pub type UIDMH = crate::Reg<uidmh::UIDMH_SPEC>;
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "UIDML register accessor: an alias for `Reg<UIDML_SPEC>`"]
pub type UIDML = crate::Reg<uidml::UIDML_SPEC>;
#[doc = "Unique Identification Register Mid Low"]
pub mod uidml;
#[doc = "UIDL register accessor: an alias for `Reg<UIDL_SPEC>`"]
pub type UIDL = crate::Reg<uidl::UIDL_SPEC>;
#[doc = "Unique Identification Register Low"]
pub mod uidl;
#[doc = "CLKDIV3 register accessor: an alias for `Reg<CLKDIV3_SPEC>`"]
pub type CLKDIV3 = crate::Reg<clkdiv3::CLKDIV3_SPEC>;
#[doc = "System Clock Divider Register 3"]
pub mod clkdiv3;
#[doc = "MISCCTRL register accessor: an alias for `Reg<MISCCTRL_SPEC>`"]
pub type MISCCTRL = crate::Reg<miscctrl::MISCCTRL_SPEC>;
#[doc = "Misc Control Register"]
pub mod miscctrl;
#[doc = "SECKEY0 register accessor: an alias for `Reg<SECKEY0_SPEC>`"]
pub type SECKEY0 = crate::Reg<seckey0::SECKEY0_SPEC>;
#[doc = "Secure Key Register 0"]
pub mod seckey0;
#[doc = "SECKEY1 register accessor: an alias for `Reg<SECKEY1_SPEC>`"]
pub type SECKEY1 = crate::Reg<seckey1::SECKEY1_SPEC>;
#[doc = "Secure Key Register 1"]
pub mod seckey1;
#[doc = "SECKEY2 register accessor: an alias for `Reg<SECKEY2_SPEC>`"]
pub type SECKEY2 = crate::Reg<seckey2::SECKEY2_SPEC>;
#[doc = "Secure Key Register 2"]
pub mod seckey2;
#[doc = "SECKEY3 register accessor: an alias for `Reg<SECKEY3_SPEC>`"]
pub type SECKEY3 = crate::Reg<seckey3::SECKEY3_SPEC>;
#[doc = "Secure Key Register 3"]
pub mod seckey3;
