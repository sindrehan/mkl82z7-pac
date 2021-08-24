#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_crchu: [u8; 0x04],
    _reserved_1_gpolyhu: [u8; 0x04],
    _reserved_2_ctrl: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - CRC_CRCLL register."]
    #[inline(always)]
    pub fn crc_crcll(&self) -> &crate::Reg<crc_crcll::CRC_CRCLL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_crcll::CRC_CRCLL_SPEC>)
        }
    }
    #[doc = "0x00 - CRC_CRCL register."]
    #[inline(always)]
    pub fn crc_crcl(&self) -> &crate::Reg<crc_crcl::CRC_CRCL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_crcl::CRC_CRCL_SPEC>)
        }
    }
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub fn crc_data(&self) -> &crate::Reg<crc_data::CRC_DATA_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_data::CRC_DATA_SPEC>)
        }
    }
    #[doc = "0x01 - CRC_CRCLU register."]
    #[inline(always)]
    pub fn crclu(&self) -> &crate::Reg<crclu::CRCLU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1usize)
                as *const crate::Reg<crclu::CRCLU_SPEC>)
        }
    }
    #[doc = "0x02 - CRC_CRCHL register."]
    #[inline(always)]
    pub fn crc_crchl(&self) -> &crate::Reg<crc_crchl::CRC_CRCHL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<crc_crchl::CRC_CRCHL_SPEC>)
        }
    }
    #[doc = "0x02 - CRC_CRCH register."]
    #[inline(always)]
    pub fn crc_crch(&self) -> &crate::Reg<crc_crch::CRC_CRCH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<crc_crch::CRC_CRCH_SPEC>)
        }
    }
    #[doc = "0x03 - CRC_CRCHU register."]
    #[inline(always)]
    pub fn crchu(&self) -> &crate::Reg<crchu::CRCHU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3usize)
                as *const crate::Reg<crchu::CRCHU_SPEC>)
        }
    }
    #[doc = "0x04 - CRC_GPOLYLL register."]
    #[inline(always)]
    pub fn crc_gpolyll(&self) -> &crate::Reg<crc_gpolyll::CRC_GPOLYLL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<crc_gpolyll::CRC_GPOLYLL_SPEC>)
        }
    }
    #[doc = "0x04 - CRC_GPOLYL register."]
    #[inline(always)]
    pub fn crc_gpolyl(&self) -> &crate::Reg<crc_gpolyl::CRC_GPOLYL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<crc_gpolyl::CRC_GPOLYL_SPEC>)
        }
    }
    #[doc = "0x04 - CRC Polynomial register"]
    #[inline(always)]
    pub fn crc_gpoly(&self) -> &crate::Reg<crc_gpoly::CRC_GPOLY_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<crc_gpoly::CRC_GPOLY_SPEC>)
        }
    }
    #[doc = "0x05 - CRC_GPOLYLU register."]
    #[inline(always)]
    pub fn gpolylu(&self) -> &crate::Reg<gpolylu::GPOLYLU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5usize)
                as *const crate::Reg<gpolylu::GPOLYLU_SPEC>)
        }
    }
    #[doc = "0x06 - CRC_GPOLYHL register."]
    #[inline(always)]
    pub fn crc_gpolyhl(&self) -> &crate::Reg<crc_gpolyhl::CRC_GPOLYHL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(6usize)
                as *const crate::Reg<crc_gpolyhl::CRC_GPOLYHL_SPEC>)
        }
    }
    #[doc = "0x06 - CRC_GPOLYH register."]
    #[inline(always)]
    pub fn crc_gpolyh(&self) -> &crate::Reg<crc_gpolyh::CRC_GPOLYH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(6usize)
                as *const crate::Reg<crc_gpolyh::CRC_GPOLYH_SPEC>)
        }
    }
    #[doc = "0x07 - CRC_GPOLYHU register."]
    #[inline(always)]
    pub fn gpolyhu(&self) -> &crate::Reg<gpolyhu::GPOLYHU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(7usize)
                as *const crate::Reg<gpolyhu::GPOLYHU_SPEC>)
        }
    }
    #[doc = "0x08 - CRC Control register"]
    #[inline(always)]
    pub fn ctrl(&self) -> &crate::Reg<ctrl::CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<ctrl::CTRL_SPEC>)
        }
    }
    #[doc = "0x0b - CRC_CTRLHU register."]
    #[inline(always)]
    pub fn ctrlhu(&self) -> &crate::Reg<ctrlhu::CTRLHU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(11usize)
                as *const crate::Reg<ctrlhu::CTRLHU_SPEC>)
        }
    }
}
#[doc = "CRC_DATA register accessor: an alias for `Reg<CRC_DATA_SPEC>`"]
pub type CRC_DATA = crate::Reg<crc_data::CRC_DATA_SPEC>;
#[doc = "CRC Data register"]
pub mod crc_data;
#[doc = "CRC_CRCL register accessor: an alias for `Reg<CRC_CRCL_SPEC>`"]
pub type CRC_CRCL = crate::Reg<crc_crcl::CRC_CRCL_SPEC>;
#[doc = "CRC_CRCL register."]
pub mod crc_crcl;
#[doc = "CRC_CRCLL register accessor: an alias for `Reg<CRC_CRCLL_SPEC>`"]
pub type CRC_CRCLL = crate::Reg<crc_crcll::CRC_CRCLL_SPEC>;
#[doc = "CRC_CRCLL register."]
pub mod crc_crcll;
#[doc = "CRCLU register accessor: an alias for `Reg<CRCLU_SPEC>`"]
pub type CRCLU = crate::Reg<crclu::CRCLU_SPEC>;
#[doc = "CRC_CRCLU register."]
pub mod crclu;
#[doc = "CRC_CRCH register accessor: an alias for `Reg<CRC_CRCH_SPEC>`"]
pub type CRC_CRCH = crate::Reg<crc_crch::CRC_CRCH_SPEC>;
#[doc = "CRC_CRCH register."]
pub mod crc_crch;
#[doc = "CRC_CRCHL register accessor: an alias for `Reg<CRC_CRCHL_SPEC>`"]
pub type CRC_CRCHL = crate::Reg<crc_crchl::CRC_CRCHL_SPEC>;
#[doc = "CRC_CRCHL register."]
pub mod crc_crchl;
#[doc = "CRCHU register accessor: an alias for `Reg<CRCHU_SPEC>`"]
pub type CRCHU = crate::Reg<crchu::CRCHU_SPEC>;
#[doc = "CRC_CRCHU register."]
pub mod crchu;
#[doc = "CRC_GPOLY register accessor: an alias for `Reg<CRC_GPOLY_SPEC>`"]
pub type CRC_GPOLY = crate::Reg<crc_gpoly::CRC_GPOLY_SPEC>;
#[doc = "CRC Polynomial register"]
pub mod crc_gpoly;
#[doc = "CRC_GPOLYL register accessor: an alias for `Reg<CRC_GPOLYL_SPEC>`"]
pub type CRC_GPOLYL = crate::Reg<crc_gpolyl::CRC_GPOLYL_SPEC>;
#[doc = "CRC_GPOLYL register."]
pub mod crc_gpolyl;
#[doc = "CRC_GPOLYLL register accessor: an alias for `Reg<CRC_GPOLYLL_SPEC>`"]
pub type CRC_GPOLYLL = crate::Reg<crc_gpolyll::CRC_GPOLYLL_SPEC>;
#[doc = "CRC_GPOLYLL register."]
pub mod crc_gpolyll;
#[doc = "GPOLYLU register accessor: an alias for `Reg<GPOLYLU_SPEC>`"]
pub type GPOLYLU = crate::Reg<gpolylu::GPOLYLU_SPEC>;
#[doc = "CRC_GPOLYLU register."]
pub mod gpolylu;
#[doc = "CRC_GPOLYH register accessor: an alias for `Reg<CRC_GPOLYH_SPEC>`"]
pub type CRC_GPOLYH = crate::Reg<crc_gpolyh::CRC_GPOLYH_SPEC>;
#[doc = "CRC_GPOLYH register."]
pub mod crc_gpolyh;
#[doc = "CRC_GPOLYHL register accessor: an alias for `Reg<CRC_GPOLYHL_SPEC>`"]
pub type CRC_GPOLYHL = crate::Reg<crc_gpolyhl::CRC_GPOLYHL_SPEC>;
#[doc = "CRC_GPOLYHL register."]
pub mod crc_gpolyhl;
#[doc = "GPOLYHU register accessor: an alias for `Reg<GPOLYHU_SPEC>`"]
pub type GPOLYHU = crate::Reg<gpolyhu::GPOLYHU_SPEC>;
#[doc = "CRC_GPOLYHU register."]
pub mod gpolyhu;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRC Control register"]
pub mod ctrl;
#[doc = "CTRLHU register accessor: an alias for `Reg<CTRLHU_SPEC>`"]
pub type CTRLHU = crate::Reg<ctrlhu::CTRLHU_SPEC>;
#[doc = "CRC_CTRLHU register."]
pub mod ctrlhu;
