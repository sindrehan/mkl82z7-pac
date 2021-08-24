#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 register"]
    pub pe1: crate::Reg<pe1::PE1_SPEC>,
    #[doc = "0x01 - LLWU Pin Enable 2 register"]
    pub pe2: crate::Reg<pe2::PE2_SPEC>,
    #[doc = "0x02 - LLWU Pin Enable 3 register"]
    pub pe3: crate::Reg<pe3::PE3_SPEC>,
    #[doc = "0x03 - LLWU Pin Enable 4 register"]
    pub pe4: crate::Reg<pe4::PE4_SPEC>,
    #[doc = "0x04 - LLWU Pin Enable 5 register"]
    pub pe5: crate::Reg<pe5::PE5_SPEC>,
    #[doc = "0x05 - LLWU Pin Enable 6 register"]
    pub pe6: crate::Reg<pe6::PE6_SPEC>,
    #[doc = "0x06 - LLWU Pin Enable 7 register"]
    pub pe7: crate::Reg<pe7::PE7_SPEC>,
    #[doc = "0x07 - LLWU Pin Enable 8 register"]
    pub pe8: crate::Reg<pe8::PE8_SPEC>,
    #[doc = "0x08 - LLWU Module Enable register"]
    pub me: crate::Reg<me::ME_SPEC>,
    #[doc = "0x09 - LLWU Pin Flag 1 register"]
    pub pf1: crate::Reg<pf1::PF1_SPEC>,
    #[doc = "0x0a - LLWU Pin Flag 2 register"]
    pub pf2: crate::Reg<pf2::PF2_SPEC>,
    #[doc = "0x0b - LLWU Pin Flag 3 register"]
    pub pf3: crate::Reg<pf3::PF3_SPEC>,
    #[doc = "0x0c - LLWU Pin Flag 4 register"]
    pub pf4: crate::Reg<pf4::PF4_SPEC>,
    #[doc = "0x0d - LLWU Module Flag 5 register"]
    pub mf5: crate::Reg<mf5::MF5_SPEC>,
    #[doc = "0x0e - LLWU Pin Filter 1 register"]
    pub filt1: crate::Reg<filt1::FILT1_SPEC>,
    #[doc = "0x0f - LLWU Pin Filter 2 register"]
    pub filt2: crate::Reg<filt2::FILT2_SPEC>,
    #[doc = "0x10 - LLWU Pin Filter 3 register"]
    pub filt3: crate::Reg<filt3::FILT3_SPEC>,
    #[doc = "0x11 - LLWU Pin Filter 4 register"]
    pub filt4: crate::Reg<filt4::FILT4_SPEC>,
}
#[doc = "PE1 register accessor: an alias for `Reg<PE1_SPEC>`"]
pub type PE1 = crate::Reg<pe1::PE1_SPEC>;
#[doc = "LLWU Pin Enable 1 register"]
pub mod pe1;
#[doc = "PE2 register accessor: an alias for `Reg<PE2_SPEC>`"]
pub type PE2 = crate::Reg<pe2::PE2_SPEC>;
#[doc = "LLWU Pin Enable 2 register"]
pub mod pe2;
#[doc = "PE3 register accessor: an alias for `Reg<PE3_SPEC>`"]
pub type PE3 = crate::Reg<pe3::PE3_SPEC>;
#[doc = "LLWU Pin Enable 3 register"]
pub mod pe3;
#[doc = "PE4 register accessor: an alias for `Reg<PE4_SPEC>`"]
pub type PE4 = crate::Reg<pe4::PE4_SPEC>;
#[doc = "LLWU Pin Enable 4 register"]
pub mod pe4;
#[doc = "PE5 register accessor: an alias for `Reg<PE5_SPEC>`"]
pub type PE5 = crate::Reg<pe5::PE5_SPEC>;
#[doc = "LLWU Pin Enable 5 register"]
pub mod pe5;
#[doc = "PE6 register accessor: an alias for `Reg<PE6_SPEC>`"]
pub type PE6 = crate::Reg<pe6::PE6_SPEC>;
#[doc = "LLWU Pin Enable 6 register"]
pub mod pe6;
#[doc = "PE7 register accessor: an alias for `Reg<PE7_SPEC>`"]
pub type PE7 = crate::Reg<pe7::PE7_SPEC>;
#[doc = "LLWU Pin Enable 7 register"]
pub mod pe7;
#[doc = "PE8 register accessor: an alias for `Reg<PE8_SPEC>`"]
pub type PE8 = crate::Reg<pe8::PE8_SPEC>;
#[doc = "LLWU Pin Enable 8 register"]
pub mod pe8;
#[doc = "ME register accessor: an alias for `Reg<ME_SPEC>`"]
pub type ME = crate::Reg<me::ME_SPEC>;
#[doc = "LLWU Module Enable register"]
pub mod me;
#[doc = "PF1 register accessor: an alias for `Reg<PF1_SPEC>`"]
pub type PF1 = crate::Reg<pf1::PF1_SPEC>;
#[doc = "LLWU Pin Flag 1 register"]
pub mod pf1;
#[doc = "PF2 register accessor: an alias for `Reg<PF2_SPEC>`"]
pub type PF2 = crate::Reg<pf2::PF2_SPEC>;
#[doc = "LLWU Pin Flag 2 register"]
pub mod pf2;
#[doc = "PF3 register accessor: an alias for `Reg<PF3_SPEC>`"]
pub type PF3 = crate::Reg<pf3::PF3_SPEC>;
#[doc = "LLWU Pin Flag 3 register"]
pub mod pf3;
#[doc = "PF4 register accessor: an alias for `Reg<PF4_SPEC>`"]
pub type PF4 = crate::Reg<pf4::PF4_SPEC>;
#[doc = "LLWU Pin Flag 4 register"]
pub mod pf4;
#[doc = "MF5 register accessor: an alias for `Reg<MF5_SPEC>`"]
pub type MF5 = crate::Reg<mf5::MF5_SPEC>;
#[doc = "LLWU Module Flag 5 register"]
pub mod mf5;
#[doc = "FILT1 register accessor: an alias for `Reg<FILT1_SPEC>`"]
pub type FILT1 = crate::Reg<filt1::FILT1_SPEC>;
#[doc = "LLWU Pin Filter 1 register"]
pub mod filt1;
#[doc = "FILT2 register accessor: an alias for `Reg<FILT2_SPEC>`"]
pub type FILT2 = crate::Reg<filt2::FILT2_SPEC>;
#[doc = "LLWU Pin Filter 2 register"]
pub mod filt2;
#[doc = "FILT3 register accessor: an alias for `Reg<FILT3_SPEC>`"]
pub type FILT3 = crate::Reg<filt3::FILT3_SPEC>;
#[doc = "LLWU Pin Filter 3 register"]
pub mod filt3;
#[doc = "FILT4 register accessor: an alias for `Reg<FILT4_SPEC>`"]
pub type FILT4 = crate::Reg<filt4::FILT4_SPEC>;
#[doc = "LLWU Pin Filter 4 register"]
pub mod filt4;
