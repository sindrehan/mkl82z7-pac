#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC Data Low Register"]
    pub dat0l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x01 - DAC Data High Register"]
    pub dat0h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x02 - DAC Data Low Register"]
    pub dat1l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x03 - DAC Data High Register"]
    pub dat1h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x04 - DAC Data Low Register"]
    pub dat2l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x05 - DAC Data High Register"]
    pub dat2h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x06 - DAC Data Low Register"]
    pub dat3l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x07 - DAC Data High Register"]
    pub dat3h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x08 - DAC Data Low Register"]
    pub dat4l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x09 - DAC Data High Register"]
    pub dat4h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x0a - DAC Data Low Register"]
    pub dat5l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x0b - DAC Data High Register"]
    pub dat5h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x0c - DAC Data Low Register"]
    pub dat6l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x0d - DAC Data High Register"]
    pub dat6h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x0e - DAC Data Low Register"]
    pub dat7l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x0f - DAC Data High Register"]
    pub dat7h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x10 - DAC Data Low Register"]
    pub dat8l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x11 - DAC Data High Register"]
    pub dat8h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x12 - DAC Data Low Register"]
    pub dat9l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x13 - DAC Data High Register"]
    pub dat9h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x14 - DAC Data Low Register"]
    pub dat10l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x15 - DAC Data High Register"]
    pub dat10h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x16 - DAC Data Low Register"]
    pub dat11l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x17 - DAC Data High Register"]
    pub dat11h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x18 - DAC Data Low Register"]
    pub dat12l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x19 - DAC Data High Register"]
    pub dat12h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x1a - DAC Data Low Register"]
    pub dat13l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x1b - DAC Data High Register"]
    pub dat13h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x1c - DAC Data Low Register"]
    pub dat14l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x1d - DAC Data High Register"]
    pub dat14h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x1e - DAC Data Low Register"]
    pub dat15l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x1f - DAC Data High Register"]
    pub dat15h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x20 - DAC Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x21 - DAC Control Register"]
    pub c0: crate::Reg<c0::C0_SPEC>,
    #[doc = "0x22 - DAC Control Register 1"]
    pub c1: crate::Reg<c1::C1_SPEC>,
    #[doc = "0x23 - DAC Control Register 2"]
    pub c2: crate::Reg<c2::C2_SPEC>,
}
#[doc = "DATL register accessor: an alias for `Reg<DATL_SPEC>`"]
pub type DATL = crate::Reg<datl::DATL_SPEC>;
#[doc = "DAC Data Low Register"]
pub mod datl;
#[doc = "DATH register accessor: an alias for `Reg<DATH_SPEC>`"]
pub type DATH = crate::Reg<dath::DATH_SPEC>;
#[doc = "DAC Data High Register"]
pub mod dath;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "DAC Status Register"]
pub mod sr;
#[doc = "C0 register accessor: an alias for `Reg<C0_SPEC>`"]
pub type C0 = crate::Reg<c0::C0_SPEC>;
#[doc = "DAC Control Register"]
pub mod c0;
#[doc = "C1 register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "DAC Control Register 1"]
pub mod c1;
#[doc = "C2 register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "DAC Control Register 2"]
pub mod c2;
