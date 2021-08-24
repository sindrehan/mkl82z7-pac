#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    pub fstat: crate::Reg<fstat::FSTAT_SPEC>,
    #[doc = "0x01 - Flash Configuration Register"]
    pub fcnfg: crate::Reg<fcnfg::FCNFG_SPEC>,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: crate::Reg<fsec::FSEC_SPEC>,
    #[doc = "0x03 - Flash Option Register"]
    pub fopt: crate::Reg<fopt::FOPT_SPEC>,
    #[doc = "0x04 - Flash Common Command Object Registers"]
    pub fccob3: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x05 - Flash Common Command Object Registers"]
    pub fccob2: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x06 - Flash Common Command Object Registers"]
    pub fccob1: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x07 - Flash Common Command Object Registers"]
    pub fccob0: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x08 - Flash Common Command Object Registers"]
    pub fccob7: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x09 - Flash Common Command Object Registers"]
    pub fccob6: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0a - Flash Common Command Object Registers"]
    pub fccob5: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0b - Flash Common Command Object Registers"]
    pub fccob4: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0c - Flash Common Command Object Registers"]
    pub fccobb: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0d - Flash Common Command Object Registers"]
    pub fccoba: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0e - Flash Common Command Object Registers"]
    pub fccob9: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0f - Flash Common Command Object Registers"]
    pub fccob8: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot3: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x11 - Program Flash Protection Registers"]
    pub fprot2: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x12 - Program Flash Protection Registers"]
    pub fprot1: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x13 - Program Flash Protection Registers"]
    pub fprot0: crate::Reg<fprot::FPROT_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x18 - Execute-only Access Registers"]
    pub xacch3: crate::Reg<xacc::XACC_SPEC>,
    #[doc = "0x19 - Execute-only Access Registers"]
    pub xacch2: crate::Reg<xacc::XACC_SPEC>,
    #[doc = "0x1a - Execute-only Access Registers"]
    pub xacch1: crate::Reg<xacc::XACC_SPEC>,
    #[doc = "0x1b - Execute-only Access Registers"]
    pub xacch0: crate::Reg<xacc::XACC_SPEC>,
    #[doc = "0x1c - Execute-only Access Registers"]
    pub xaccl3: crate::Reg<xacc::XACC_SPEC>,
    #[doc = "0x1d - Execute-only Access Registers"]
    pub xaccl2: crate::Reg<xacc::XACC_SPEC>,
    #[doc = "0x1e - Execute-only Access Registers"]
    pub xaccl1: crate::Reg<xacc::XACC_SPEC>,
    #[doc = "0x1f - Execute-only Access Registers"]
    pub xaccl0: crate::Reg<xacc::XACC_SPEC>,
    #[doc = "0x20 - Supervisor-only Access Registers"]
    pub sacch3: crate::Reg<sacc::SACC_SPEC>,
    #[doc = "0x21 - Supervisor-only Access Registers"]
    pub sacch2: crate::Reg<sacc::SACC_SPEC>,
    #[doc = "0x22 - Supervisor-only Access Registers"]
    pub sacch1: crate::Reg<sacc::SACC_SPEC>,
    #[doc = "0x23 - Supervisor-only Access Registers"]
    pub sacch0: crate::Reg<sacc::SACC_SPEC>,
    #[doc = "0x24 - Supervisor-only Access Registers"]
    pub saccl3: crate::Reg<sacc::SACC_SPEC>,
    #[doc = "0x25 - Supervisor-only Access Registers"]
    pub saccl2: crate::Reg<sacc::SACC_SPEC>,
    #[doc = "0x26 - Supervisor-only Access Registers"]
    pub saccl1: crate::Reg<sacc::SACC_SPEC>,
    #[doc = "0x27 - Supervisor-only Access Registers"]
    pub saccl0: crate::Reg<sacc::SACC_SPEC>,
    #[doc = "0x28 - Flash Access Segment Size Register"]
    pub facss: crate::Reg<facss::FACSS_SPEC>,
    _reserved37: [u8; 0x02],
    #[doc = "0x2b - Flash Access Segment Number Register"]
    pub facsn: crate::Reg<facsn::FACSN_SPEC>,
}
#[doc = "FSTAT register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FCNFG register accessor: an alias for `Reg<FCNFG_SPEC>`"]
pub type FCNFG = crate::Reg<fcnfg::FCNFG_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FSEC register accessor: an alias for `Reg<FSEC_SPEC>`"]
pub type FSEC = crate::Reg<fsec::FSEC_SPEC>;
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "FOPT register accessor: an alias for `Reg<FOPT_SPEC>`"]
pub type FOPT = crate::Reg<fopt::FOPT_SPEC>;
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "FCCOB register accessor: an alias for `Reg<FCCOB_SPEC>`"]
pub type FCCOB = crate::Reg<fccob::FCCOB_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "FPROT register accessor: an alias for `Reg<FPROT_SPEC>`"]
pub type FPROT = crate::Reg<fprot::FPROT_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot;
#[doc = "XACC register accessor: an alias for `Reg<XACC_SPEC>`"]
pub type XACC = crate::Reg<xacc::XACC_SPEC>;
#[doc = "Execute-only Access Registers"]
pub mod xacc;
#[doc = "SACC register accessor: an alias for `Reg<SACC_SPEC>`"]
pub type SACC = crate::Reg<sacc::SACC_SPEC>;
#[doc = "Supervisor-only Access Registers"]
pub mod sacc;
#[doc = "FACSS register accessor: an alias for `Reg<FACSS_SPEC>`"]
pub type FACSS = crate::Reg<facss::FACSS_SPEC>;
#[doc = "Flash Access Segment Size Register"]
pub mod facss;
#[doc = "FACSN register accessor: an alias for `Reg<FACSN_SPEC>`"]
pub type FACSN = crate::Reg<facsn::FACSN_SPEC>;
#[doc = "Flash Access Segment Number Register"]
pub mod facsn;
