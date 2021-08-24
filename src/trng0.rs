#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRNG0 Miscellaneous Control Register"]
    pub trng0_mctl: crate::Reg<trng0_mctl::TRNG0_MCTL_SPEC>,
    #[doc = "0x04 - TRNG0 Statistical Check Miscellaneous Register"]
    pub trng0_scmisc: crate::Reg<trng0_scmisc::TRNG0_SCMISC_SPEC>,
    #[doc = "0x08 - TRNG0 Poker Range Register"]
    pub trng0_pkrrng: crate::Reg<trng0_pkrrng::TRNG0_PKRRNG_SPEC>,
    _reserved_3_trng0_trng0: [u8; 0x04],
    #[doc = "0x10 - TRNG0 Seed Control Register"]
    pub trng0_sdctl: crate::Reg<trng0_sdctl::TRNG0_SDCTL_SPEC>,
    _reserved_5_trng0_trng0: [u8; 0x04],
    #[doc = "0x18 - TRNG0 Frequency Count Minimum Limit Register"]
    pub trng0_frqmin: crate::Reg<trng0_frqmin::TRNG0_FRQMIN_SPEC>,
    _reserved_7_trng0_trng0: [u8; 0x04],
    _reserved_8_trng0_trng0: [u8; 0x04],
    _reserved_9_trng0_trng0_scr: [u8; 0x04],
    _reserved_10_trng0_trng0_scr: [u8; 0x04],
    _reserved_11_trng0_trng0_scr: [u8; 0x04],
    _reserved_12_trng0_trng0_scr: [u8; 0x04],
    _reserved_13_trng0_trng0_scr: [u8; 0x04],
    _reserved_14_trng0_trng0_scr: [u8; 0x04],
    #[doc = "0x3c - TRNG0 Status Register"]
    pub trng0_status: crate::Reg<trng0_status::TRNG0_STATUS_SPEC>,
    #[doc = "0x40 - RNG TRNG Entropy Read Register"]
    pub trng0_ent0: crate::Reg<trng0_ent0::TRNG0_ENT0_SPEC>,
    #[doc = "0x44 - RNG TRNG Entropy Read Register"]
    pub trng0_ent1: crate::Reg<trng0_ent1::TRNG0_ENT1_SPEC>,
    #[doc = "0x48 - RNG TRNG Entropy Read Register"]
    pub trng0_ent2: crate::Reg<trng0_ent2::TRNG0_ENT2_SPEC>,
    #[doc = "0x4c - RNG TRNG Entropy Read Register"]
    pub trng0_ent3: crate::Reg<trng0_ent3::TRNG0_ENT3_SPEC>,
    #[doc = "0x50 - RNG TRNG Entropy Read Register"]
    pub trng0_ent4: crate::Reg<trng0_ent4::TRNG0_ENT4_SPEC>,
    #[doc = "0x54 - RNG TRNG Entropy Read Register"]
    pub trng0_ent5: crate::Reg<trng0_ent5::TRNG0_ENT5_SPEC>,
    #[doc = "0x58 - RNG TRNG Entropy Read Register"]
    pub trng0_ent6: crate::Reg<trng0_ent6::TRNG0_ENT6_SPEC>,
    #[doc = "0x5c - RNG TRNG Entropy Read Register"]
    pub trng0_ent7: crate::Reg<trng0_ent7::TRNG0_ENT7_SPEC>,
    #[doc = "0x60 - RNG TRNG Entropy Read Register"]
    pub trng0_ent8: crate::Reg<trng0_ent8::TRNG0_ENT8_SPEC>,
    #[doc = "0x64 - RNG TRNG Entropy Read Register"]
    pub trng0_ent9: crate::Reg<trng0_ent9::TRNG0_ENT9_SPEC>,
    #[doc = "0x68 - RNG TRNG Entropy Read Register"]
    pub trng0_ent10: crate::Reg<trng0_ent10::TRNG0_ENT10_SPEC>,
    #[doc = "0x6c - RNG TRNG Entropy Read Register"]
    pub trng0_ent11: crate::Reg<trng0_ent11::TRNG0_ENT11_SPEC>,
    #[doc = "0x70 - RNG TRNG Entropy Read Register"]
    pub trng0_ent12: crate::Reg<trng0_ent12::TRNG0_ENT12_SPEC>,
    #[doc = "0x74 - RNG TRNG Entropy Read Register"]
    pub trng0_ent13: crate::Reg<trng0_ent13::TRNG0_ENT13_SPEC>,
    #[doc = "0x78 - RNG TRNG Entropy Read Register"]
    pub trng0_ent14: crate::Reg<trng0_ent14::TRNG0_ENT14_SPEC>,
    #[doc = "0x7c - RNG TRNG Entropy Read Register"]
    pub trng0_ent15: crate::Reg<trng0_ent15::TRNG0_ENT15_SPEC>,
    #[doc = "0x80 - TRNG0 Statistical Check Poker Count 1 and 0 Register"]
    pub trng0_pkrcnt10: crate::Reg<trng0_pkrcnt10::TRNG0_PKRCNT10_SPEC>,
    #[doc = "0x84 - TRNG0 Statistical Check Poker Count 3 and 2 Register"]
    pub trng0_pkrcnt32: crate::Reg<trng0_pkrcnt32::TRNG0_PKRCNT32_SPEC>,
    #[doc = "0x88 - TRNG0 Statistical Check Poker Count 5 and 4 Register"]
    pub trng0_pkrcnt54: crate::Reg<trng0_pkrcnt54::TRNG0_PKRCNT54_SPEC>,
    #[doc = "0x8c - TRNG0 Statistical Check Poker Count 7 and 6 Register"]
    pub trng0_pkrcnt76: crate::Reg<trng0_pkrcnt76::TRNG0_PKRCNT76_SPEC>,
    #[doc = "0x90 - TRNG0 Statistical Check Poker Count 9 and 8 Register"]
    pub trng0_pkrcnt98: crate::Reg<trng0_pkrcnt98::TRNG0_PKRCNT98_SPEC>,
    #[doc = "0x94 - TRNG0 Statistical Check Poker Count B and A Register"]
    pub trng0_pkrcntba: crate::Reg<trng0_pkrcntba::TRNG0_PKRCNTBA_SPEC>,
    #[doc = "0x98 - TRNG0 Statistical Check Poker Count D and C Register"]
    pub trng0_pkrcntdc: crate::Reg<trng0_pkrcntdc::TRNG0_PKRCNTDC_SPEC>,
    #[doc = "0x9c - TRNG0 Statistical Check Poker Count F and E Register"]
    pub trng0_pkrcntfe: crate::Reg<trng0_pkrcntfe::TRNG0_PKRCNTFE_SPEC>,
    #[doc = "0xa0 - TRNG0 Security Configuration Register"]
    pub trng0_sec_cfg: crate::Reg<trng0_sec_cfg::TRNG0_SEC_CFG_SPEC>,
    #[doc = "0xa4 - TRNG0 Interrupt Control Register"]
    pub trng0_int_ctrl: crate::Reg<trng0_int_ctrl::TRNG0_INT_CTRL_SPEC>,
    #[doc = "0xa8 - TRNG0 Mask Register"]
    pub trng0_int_mask: crate::Reg<trng0_int_mask::TRNG0_INT_MASK_SPEC>,
    #[doc = "0xac - TRNG0 Interrupt Status Register"]
    pub trng0_int_status: crate::Reg<trng0_int_status::TRNG0_INT_STATUS_SPEC>,
    _reserved44: [u8; 0x40],
    #[doc = "0xf0 - TRNG0 Version ID Register (MS)"]
    pub trng0_vid1: crate::Reg<trng0_vid1::TRNG0_VID1_SPEC>,
    #[doc = "0xf4 - TRNG0 Version ID Register (LS)"]
    pub trng0_vid2: crate::Reg<trng0_vid2::TRNG0_VID2_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x0c - TRNG0 Poker Square Calculation Result Register"]
    #[inline(always)]
    pub fn trng0_trng0_pkrsq(&self) -> &crate::Reg<trng0_trng0_pkrsq::TRNG0_TRNG0_PKRSQ_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<trng0_trng0_pkrsq::TRNG0_TRNG0_PKRSQ_SPEC>)
        }
    }
    #[doc = "0x0c - TRNG0 Poker Maximum Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_pkrmax(&self) -> &crate::Reg<trng0_trng0_pkrmax::TRNG0_TRNG0_PKRMAX_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<trng0_trng0_pkrmax::TRNG0_TRNG0_PKRMAX_SPEC>)
        }
    }
    #[doc = "0x14 - TRNG0 Total Samples Register"]
    #[inline(always)]
    pub fn trng0_trng0_totsam(&self) -> &crate::Reg<trng0_trng0_totsam::TRNG0_TRNG0_TOTSAM_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<trng0_trng0_totsam::TRNG0_TRNG0_TOTSAM_SPEC>)
        }
    }
    #[doc = "0x14 - TRNG0 Sparse Bit Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_sblim(&self) -> &crate::Reg<trng0_trng0_sblim::TRNG0_TRNG0_SBLIM_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<trng0_trng0_sblim::TRNG0_TRNG0_SBLIM_SPEC>)
        }
    }
    #[doc = "0x1c - TRNG0 Frequency Count Maximum Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_frqmax(&self) -> &crate::Reg<trng0_trng0_frqmax::TRNG0_TRNG0_FRQMAX_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<trng0_trng0_frqmax::TRNG0_TRNG0_FRQMAX_SPEC>)
        }
    }
    #[doc = "0x1c - TRNG0 Frequency Count Register"]
    #[inline(always)]
    pub fn trng0_trng0_frqcnt(&self) -> &crate::Reg<trng0_trng0_frqcnt::TRNG0_TRNG0_FRQCNT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<trng0_trng0_frqcnt::TRNG0_TRNG0_FRQCNT_SPEC>)
        }
    }
    #[doc = "0x20 - TRNG0 Statistical Check Monobit Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_scml(&self) -> &crate::Reg<trng0_trng0_scml::TRNG0_TRNG0_SCML_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<trng0_trng0_scml::TRNG0_TRNG0_SCML_SPEC>)
        }
    }
    #[doc = "0x20 - TRNG0 Statistical Check Monobit Count Register"]
    #[inline(always)]
    pub fn trng0_trng0_scmc(&self) -> &crate::Reg<trng0_trng0_scmc::TRNG0_TRNG0_SCMC_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<trng0_trng0_scmc::TRNG0_TRNG0_SCMC_SPEC>)
        }
    }
    #[doc = "0x24 - TRNG0 Statistical Check Run Length 1 Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr1l(&self) -> &crate::Reg<trng0_trng0_scr1l::TRNG0_TRNG0_SCR1L_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<trng0_trng0_scr1l::TRNG0_TRNG0_SCR1L_SPEC>)
        }
    }
    #[doc = "0x24 - TRNG0 Statistical Check Run Length 1 Count Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr1c(&self) -> &crate::Reg<trng0_trng0_scr1c::TRNG0_TRNG0_SCR1C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<trng0_trng0_scr1c::TRNG0_TRNG0_SCR1C_SPEC>)
        }
    }
    #[doc = "0x28 - TRNG0 Statistical Check Run Length 2 Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr2l(&self) -> &crate::Reg<trng0_trng0_scr2l::TRNG0_TRNG0_SCR2L_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<trng0_trng0_scr2l::TRNG0_TRNG0_SCR2L_SPEC>)
        }
    }
    #[doc = "0x28 - TRNG0 Statistical Check Run Length 2 Count Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr2c(&self) -> &crate::Reg<trng0_trng0_scr2c::TRNG0_TRNG0_SCR2C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<trng0_trng0_scr2c::TRNG0_TRNG0_SCR2C_SPEC>)
        }
    }
    #[doc = "0x2c - TRNG0 Statistical Check Run Length 3 Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr3l(&self) -> &crate::Reg<trng0_trng0_scr3l::TRNG0_TRNG0_SCR3L_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<trng0_trng0_scr3l::TRNG0_TRNG0_SCR3L_SPEC>)
        }
    }
    #[doc = "0x2c - TRNG0 Statistical Check Run Length 3 Count Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr3c(&self) -> &crate::Reg<trng0_trng0_scr3c::TRNG0_TRNG0_SCR3C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<trng0_trng0_scr3c::TRNG0_TRNG0_SCR3C_SPEC>)
        }
    }
    #[doc = "0x30 - TRNG0 Statistical Check Run Length 4 Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr4l(&self) -> &crate::Reg<trng0_trng0_scr4l::TRNG0_TRNG0_SCR4L_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<trng0_trng0_scr4l::TRNG0_TRNG0_SCR4L_SPEC>)
        }
    }
    #[doc = "0x30 - TRNG0 Statistical Check Run Length 4 Count Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr4c(&self) -> &crate::Reg<trng0_trng0_scr4c::TRNG0_TRNG0_SCR4C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<trng0_trng0_scr4c::TRNG0_TRNG0_SCR4C_SPEC>)
        }
    }
    #[doc = "0x34 - TRNG0 Statistical Check Run Length 5 Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr5l(&self) -> &crate::Reg<trng0_trng0_scr5l::TRNG0_TRNG0_SCR5L_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<trng0_trng0_scr5l::TRNG0_TRNG0_SCR5L_SPEC>)
        }
    }
    #[doc = "0x34 - TRNG0 Statistical Check Run Length 5 Count Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr5c(&self) -> &crate::Reg<trng0_trng0_scr5c::TRNG0_TRNG0_SCR5C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<trng0_trng0_scr5c::TRNG0_TRNG0_SCR5C_SPEC>)
        }
    }
    #[doc = "0x38 - TRNG0 Statistical Check Run Length 6+ Limit Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr6pl(&self) -> &crate::Reg<trng0_trng0_scr6pl::TRNG0_TRNG0_SCR6PL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<trng0_trng0_scr6pl::TRNG0_TRNG0_SCR6PL_SPEC>)
        }
    }
    #[doc = "0x38 - TRNG0 Statistical Check Run Length 6+ Count Register"]
    #[inline(always)]
    pub fn trng0_trng0_scr6pc(&self) -> &crate::Reg<trng0_trng0_scr6pc::TRNG0_TRNG0_SCR6PC_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<trng0_trng0_scr6pc::TRNG0_TRNG0_SCR6PC_SPEC>)
        }
    }
}
#[doc = "TRNG0_MCTL register accessor: an alias for `Reg<TRNG0_MCTL_SPEC>`"]
pub type TRNG0_MCTL = crate::Reg<trng0_mctl::TRNG0_MCTL_SPEC>;
#[doc = "TRNG0 Miscellaneous Control Register"]
pub mod trng0_mctl;
#[doc = "TRNG0_SCMISC register accessor: an alias for `Reg<TRNG0_SCMISC_SPEC>`"]
pub type TRNG0_SCMISC = crate::Reg<trng0_scmisc::TRNG0_SCMISC_SPEC>;
#[doc = "TRNG0 Statistical Check Miscellaneous Register"]
pub mod trng0_scmisc;
#[doc = "TRNG0_PKRRNG register accessor: an alias for `Reg<TRNG0_PKRRNG_SPEC>`"]
pub type TRNG0_PKRRNG = crate::Reg<trng0_pkrrng::TRNG0_PKRRNG_SPEC>;
#[doc = "TRNG0 Poker Range Register"]
pub mod trng0_pkrrng;
#[doc = "TRNG0_TRNG0_PKRMAX register accessor: an alias for `Reg<TRNG0_TRNG0_PKRMAX_SPEC>`"]
pub type TRNG0_TRNG0_PKRMAX = crate::Reg<trng0_trng0_pkrmax::TRNG0_TRNG0_PKRMAX_SPEC>;
#[doc = "TRNG0 Poker Maximum Limit Register"]
pub mod trng0_trng0_pkrmax;
#[doc = "TRNG0_TRNG0_PKRSQ register accessor: an alias for `Reg<TRNG0_TRNG0_PKRSQ_SPEC>`"]
pub type TRNG0_TRNG0_PKRSQ = crate::Reg<trng0_trng0_pkrsq::TRNG0_TRNG0_PKRSQ_SPEC>;
#[doc = "TRNG0 Poker Square Calculation Result Register"]
pub mod trng0_trng0_pkrsq;
#[doc = "TRNG0_SDCTL register accessor: an alias for `Reg<TRNG0_SDCTL_SPEC>`"]
pub type TRNG0_SDCTL = crate::Reg<trng0_sdctl::TRNG0_SDCTL_SPEC>;
#[doc = "TRNG0 Seed Control Register"]
pub mod trng0_sdctl;
#[doc = "TRNG0_TRNG0_SBLIM register accessor: an alias for `Reg<TRNG0_TRNG0_SBLIM_SPEC>`"]
pub type TRNG0_TRNG0_SBLIM = crate::Reg<trng0_trng0_sblim::TRNG0_TRNG0_SBLIM_SPEC>;
#[doc = "TRNG0 Sparse Bit Limit Register"]
pub mod trng0_trng0_sblim;
#[doc = "TRNG0_TRNG0_TOTSAM register accessor: an alias for `Reg<TRNG0_TRNG0_TOTSAM_SPEC>`"]
pub type TRNG0_TRNG0_TOTSAM = crate::Reg<trng0_trng0_totsam::TRNG0_TRNG0_TOTSAM_SPEC>;
#[doc = "TRNG0 Total Samples Register"]
pub mod trng0_trng0_totsam;
#[doc = "TRNG0_FRQMIN register accessor: an alias for `Reg<TRNG0_FRQMIN_SPEC>`"]
pub type TRNG0_FRQMIN = crate::Reg<trng0_frqmin::TRNG0_FRQMIN_SPEC>;
#[doc = "TRNG0 Frequency Count Minimum Limit Register"]
pub mod trng0_frqmin;
#[doc = "TRNG0_TRNG0_FRQCNT register accessor: an alias for `Reg<TRNG0_TRNG0_FRQCNT_SPEC>`"]
pub type TRNG0_TRNG0_FRQCNT = crate::Reg<trng0_trng0_frqcnt::TRNG0_TRNG0_FRQCNT_SPEC>;
#[doc = "TRNG0 Frequency Count Register"]
pub mod trng0_trng0_frqcnt;
#[doc = "TRNG0_TRNG0_FRQMAX register accessor: an alias for `Reg<TRNG0_TRNG0_FRQMAX_SPEC>`"]
pub type TRNG0_TRNG0_FRQMAX = crate::Reg<trng0_trng0_frqmax::TRNG0_TRNG0_FRQMAX_SPEC>;
#[doc = "TRNG0 Frequency Count Maximum Limit Register"]
pub mod trng0_trng0_frqmax;
#[doc = "TRNG0_TRNG0_SCMC register accessor: an alias for `Reg<TRNG0_TRNG0_SCMC_SPEC>`"]
pub type TRNG0_TRNG0_SCMC = crate::Reg<trng0_trng0_scmc::TRNG0_TRNG0_SCMC_SPEC>;
#[doc = "TRNG0 Statistical Check Monobit Count Register"]
pub mod trng0_trng0_scmc;
#[doc = "TRNG0_TRNG0_SCML register accessor: an alias for `Reg<TRNG0_TRNG0_SCML_SPEC>`"]
pub type TRNG0_TRNG0_SCML = crate::Reg<trng0_trng0_scml::TRNG0_TRNG0_SCML_SPEC>;
#[doc = "TRNG0 Statistical Check Monobit Limit Register"]
pub mod trng0_trng0_scml;
#[doc = "TRNG0_TRNG0_SCR1C register accessor: an alias for `Reg<TRNG0_TRNG0_SCR1C_SPEC>`"]
pub type TRNG0_TRNG0_SCR1C = crate::Reg<trng0_trng0_scr1c::TRNG0_TRNG0_SCR1C_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 1 Count Register"]
pub mod trng0_trng0_scr1c;
#[doc = "TRNG0_TRNG0_SCR1L register accessor: an alias for `Reg<TRNG0_TRNG0_SCR1L_SPEC>`"]
pub type TRNG0_TRNG0_SCR1L = crate::Reg<trng0_trng0_scr1l::TRNG0_TRNG0_SCR1L_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 1 Limit Register"]
pub mod trng0_trng0_scr1l;
#[doc = "TRNG0_TRNG0_SCR2C register accessor: an alias for `Reg<TRNG0_TRNG0_SCR2C_SPEC>`"]
pub type TRNG0_TRNG0_SCR2C = crate::Reg<trng0_trng0_scr2c::TRNG0_TRNG0_SCR2C_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 2 Count Register"]
pub mod trng0_trng0_scr2c;
#[doc = "TRNG0_TRNG0_SCR2L register accessor: an alias for `Reg<TRNG0_TRNG0_SCR2L_SPEC>`"]
pub type TRNG0_TRNG0_SCR2L = crate::Reg<trng0_trng0_scr2l::TRNG0_TRNG0_SCR2L_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 2 Limit Register"]
pub mod trng0_trng0_scr2l;
#[doc = "TRNG0_TRNG0_SCR3C register accessor: an alias for `Reg<TRNG0_TRNG0_SCR3C_SPEC>`"]
pub type TRNG0_TRNG0_SCR3C = crate::Reg<trng0_trng0_scr3c::TRNG0_TRNG0_SCR3C_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 3 Count Register"]
pub mod trng0_trng0_scr3c;
#[doc = "TRNG0_TRNG0_SCR3L register accessor: an alias for `Reg<TRNG0_TRNG0_SCR3L_SPEC>`"]
pub type TRNG0_TRNG0_SCR3L = crate::Reg<trng0_trng0_scr3l::TRNG0_TRNG0_SCR3L_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 3 Limit Register"]
pub mod trng0_trng0_scr3l;
#[doc = "TRNG0_TRNG0_SCR4C register accessor: an alias for `Reg<TRNG0_TRNG0_SCR4C_SPEC>`"]
pub type TRNG0_TRNG0_SCR4C = crate::Reg<trng0_trng0_scr4c::TRNG0_TRNG0_SCR4C_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 4 Count Register"]
pub mod trng0_trng0_scr4c;
#[doc = "TRNG0_TRNG0_SCR4L register accessor: an alias for `Reg<TRNG0_TRNG0_SCR4L_SPEC>`"]
pub type TRNG0_TRNG0_SCR4L = crate::Reg<trng0_trng0_scr4l::TRNG0_TRNG0_SCR4L_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 4 Limit Register"]
pub mod trng0_trng0_scr4l;
#[doc = "TRNG0_TRNG0_SCR5C register accessor: an alias for `Reg<TRNG0_TRNG0_SCR5C_SPEC>`"]
pub type TRNG0_TRNG0_SCR5C = crate::Reg<trng0_trng0_scr5c::TRNG0_TRNG0_SCR5C_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 5 Count Register"]
pub mod trng0_trng0_scr5c;
#[doc = "TRNG0_TRNG0_SCR5L register accessor: an alias for `Reg<TRNG0_TRNG0_SCR5L_SPEC>`"]
pub type TRNG0_TRNG0_SCR5L = crate::Reg<trng0_trng0_scr5l::TRNG0_TRNG0_SCR5L_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 5 Limit Register"]
pub mod trng0_trng0_scr5l;
#[doc = "TRNG0_TRNG0_SCR6PC register accessor: an alias for `Reg<TRNG0_TRNG0_SCR6PC_SPEC>`"]
pub type TRNG0_TRNG0_SCR6PC = crate::Reg<trng0_trng0_scr6pc::TRNG0_TRNG0_SCR6PC_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 6+ Count Register"]
pub mod trng0_trng0_scr6pc;
#[doc = "TRNG0_TRNG0_SCR6PL register accessor: an alias for `Reg<TRNG0_TRNG0_SCR6PL_SPEC>`"]
pub type TRNG0_TRNG0_SCR6PL = crate::Reg<trng0_trng0_scr6pl::TRNG0_TRNG0_SCR6PL_SPEC>;
#[doc = "TRNG0 Statistical Check Run Length 6+ Limit Register"]
pub mod trng0_trng0_scr6pl;
#[doc = "TRNG0_STATUS register accessor: an alias for `Reg<TRNG0_STATUS_SPEC>`"]
pub type TRNG0_STATUS = crate::Reg<trng0_status::TRNG0_STATUS_SPEC>;
#[doc = "TRNG0 Status Register"]
pub mod trng0_status;
#[doc = "TRNG0_ENT0 register accessor: an alias for `Reg<TRNG0_ENT0_SPEC>`"]
pub type TRNG0_ENT0 = crate::Reg<trng0_ent0::TRNG0_ENT0_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent0;
#[doc = "TRNG0_ENT1 register accessor: an alias for `Reg<TRNG0_ENT1_SPEC>`"]
pub type TRNG0_ENT1 = crate::Reg<trng0_ent1::TRNG0_ENT1_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent1;
#[doc = "TRNG0_ENT2 register accessor: an alias for `Reg<TRNG0_ENT2_SPEC>`"]
pub type TRNG0_ENT2 = crate::Reg<trng0_ent2::TRNG0_ENT2_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent2;
#[doc = "TRNG0_ENT3 register accessor: an alias for `Reg<TRNG0_ENT3_SPEC>`"]
pub type TRNG0_ENT3 = crate::Reg<trng0_ent3::TRNG0_ENT3_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent3;
#[doc = "TRNG0_ENT4 register accessor: an alias for `Reg<TRNG0_ENT4_SPEC>`"]
pub type TRNG0_ENT4 = crate::Reg<trng0_ent4::TRNG0_ENT4_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent4;
#[doc = "TRNG0_ENT5 register accessor: an alias for `Reg<TRNG0_ENT5_SPEC>`"]
pub type TRNG0_ENT5 = crate::Reg<trng0_ent5::TRNG0_ENT5_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent5;
#[doc = "TRNG0_ENT6 register accessor: an alias for `Reg<TRNG0_ENT6_SPEC>`"]
pub type TRNG0_ENT6 = crate::Reg<trng0_ent6::TRNG0_ENT6_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent6;
#[doc = "TRNG0_ENT7 register accessor: an alias for `Reg<TRNG0_ENT7_SPEC>`"]
pub type TRNG0_ENT7 = crate::Reg<trng0_ent7::TRNG0_ENT7_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent7;
#[doc = "TRNG0_ENT8 register accessor: an alias for `Reg<TRNG0_ENT8_SPEC>`"]
pub type TRNG0_ENT8 = crate::Reg<trng0_ent8::TRNG0_ENT8_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent8;
#[doc = "TRNG0_ENT9 register accessor: an alias for `Reg<TRNG0_ENT9_SPEC>`"]
pub type TRNG0_ENT9 = crate::Reg<trng0_ent9::TRNG0_ENT9_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent9;
#[doc = "TRNG0_ENT10 register accessor: an alias for `Reg<TRNG0_ENT10_SPEC>`"]
pub type TRNG0_ENT10 = crate::Reg<trng0_ent10::TRNG0_ENT10_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent10;
#[doc = "TRNG0_ENT11 register accessor: an alias for `Reg<TRNG0_ENT11_SPEC>`"]
pub type TRNG0_ENT11 = crate::Reg<trng0_ent11::TRNG0_ENT11_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent11;
#[doc = "TRNG0_ENT12 register accessor: an alias for `Reg<TRNG0_ENT12_SPEC>`"]
pub type TRNG0_ENT12 = crate::Reg<trng0_ent12::TRNG0_ENT12_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent12;
#[doc = "TRNG0_ENT13 register accessor: an alias for `Reg<TRNG0_ENT13_SPEC>`"]
pub type TRNG0_ENT13 = crate::Reg<trng0_ent13::TRNG0_ENT13_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent13;
#[doc = "TRNG0_ENT14 register accessor: an alias for `Reg<TRNG0_ENT14_SPEC>`"]
pub type TRNG0_ENT14 = crate::Reg<trng0_ent14::TRNG0_ENT14_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent14;
#[doc = "TRNG0_ENT15 register accessor: an alias for `Reg<TRNG0_ENT15_SPEC>`"]
pub type TRNG0_ENT15 = crate::Reg<trng0_ent15::TRNG0_ENT15_SPEC>;
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent15;
#[doc = "TRNG0_PKRCNT10 register accessor: an alias for `Reg<TRNG0_PKRCNT10_SPEC>`"]
pub type TRNG0_PKRCNT10 = crate::Reg<trng0_pkrcnt10::TRNG0_PKRCNT10_SPEC>;
#[doc = "TRNG0 Statistical Check Poker Count 1 and 0 Register"]
pub mod trng0_pkrcnt10;
#[doc = "TRNG0_PKRCNT32 register accessor: an alias for `Reg<TRNG0_PKRCNT32_SPEC>`"]
pub type TRNG0_PKRCNT32 = crate::Reg<trng0_pkrcnt32::TRNG0_PKRCNT32_SPEC>;
#[doc = "TRNG0 Statistical Check Poker Count 3 and 2 Register"]
pub mod trng0_pkrcnt32;
#[doc = "TRNG0_PKRCNT54 register accessor: an alias for `Reg<TRNG0_PKRCNT54_SPEC>`"]
pub type TRNG0_PKRCNT54 = crate::Reg<trng0_pkrcnt54::TRNG0_PKRCNT54_SPEC>;
#[doc = "TRNG0 Statistical Check Poker Count 5 and 4 Register"]
pub mod trng0_pkrcnt54;
#[doc = "TRNG0_PKRCNT76 register accessor: an alias for `Reg<TRNG0_PKRCNT76_SPEC>`"]
pub type TRNG0_PKRCNT76 = crate::Reg<trng0_pkrcnt76::TRNG0_PKRCNT76_SPEC>;
#[doc = "TRNG0 Statistical Check Poker Count 7 and 6 Register"]
pub mod trng0_pkrcnt76;
#[doc = "TRNG0_PKRCNT98 register accessor: an alias for `Reg<TRNG0_PKRCNT98_SPEC>`"]
pub type TRNG0_PKRCNT98 = crate::Reg<trng0_pkrcnt98::TRNG0_PKRCNT98_SPEC>;
#[doc = "TRNG0 Statistical Check Poker Count 9 and 8 Register"]
pub mod trng0_pkrcnt98;
#[doc = "TRNG0_PKRCNTBA register accessor: an alias for `Reg<TRNG0_PKRCNTBA_SPEC>`"]
pub type TRNG0_PKRCNTBA = crate::Reg<trng0_pkrcntba::TRNG0_PKRCNTBA_SPEC>;
#[doc = "TRNG0 Statistical Check Poker Count B and A Register"]
pub mod trng0_pkrcntba;
#[doc = "TRNG0_PKRCNTDC register accessor: an alias for `Reg<TRNG0_PKRCNTDC_SPEC>`"]
pub type TRNG0_PKRCNTDC = crate::Reg<trng0_pkrcntdc::TRNG0_PKRCNTDC_SPEC>;
#[doc = "TRNG0 Statistical Check Poker Count D and C Register"]
pub mod trng0_pkrcntdc;
#[doc = "TRNG0_PKRCNTFE register accessor: an alias for `Reg<TRNG0_PKRCNTFE_SPEC>`"]
pub type TRNG0_PKRCNTFE = crate::Reg<trng0_pkrcntfe::TRNG0_PKRCNTFE_SPEC>;
#[doc = "TRNG0 Statistical Check Poker Count F and E Register"]
pub mod trng0_pkrcntfe;
#[doc = "TRNG0_SEC_CFG register accessor: an alias for `Reg<TRNG0_SEC_CFG_SPEC>`"]
pub type TRNG0_SEC_CFG = crate::Reg<trng0_sec_cfg::TRNG0_SEC_CFG_SPEC>;
#[doc = "TRNG0 Security Configuration Register"]
pub mod trng0_sec_cfg;
#[doc = "TRNG0_INT_CTRL register accessor: an alias for `Reg<TRNG0_INT_CTRL_SPEC>`"]
pub type TRNG0_INT_CTRL = crate::Reg<trng0_int_ctrl::TRNG0_INT_CTRL_SPEC>;
#[doc = "TRNG0 Interrupt Control Register"]
pub mod trng0_int_ctrl;
#[doc = "TRNG0_INT_MASK register accessor: an alias for `Reg<TRNG0_INT_MASK_SPEC>`"]
pub type TRNG0_INT_MASK = crate::Reg<trng0_int_mask::TRNG0_INT_MASK_SPEC>;
#[doc = "TRNG0 Mask Register"]
pub mod trng0_int_mask;
#[doc = "TRNG0_INT_STATUS register accessor: an alias for `Reg<TRNG0_INT_STATUS_SPEC>`"]
pub type TRNG0_INT_STATUS = crate::Reg<trng0_int_status::TRNG0_INT_STATUS_SPEC>;
#[doc = "TRNG0 Interrupt Status Register"]
pub mod trng0_int_status;
#[doc = "TRNG0_VID1 register accessor: an alias for `Reg<TRNG0_VID1_SPEC>`"]
pub type TRNG0_VID1 = crate::Reg<trng0_vid1::TRNG0_VID1_SPEC>;
#[doc = "TRNG0 Version ID Register (MS)"]
pub mod trng0_vid1;
#[doc = "TRNG0_VID2 register accessor: an alias for `Reg<TRNG0_VID2_SPEC>`"]
pub type TRNG0_VID2 = crate::Reg<trng0_vid2::TRNG0_VID2_SPEC>;
#[doc = "TRNG0 Version ID Register (LS)"]
pub mod trng0_vid2;
