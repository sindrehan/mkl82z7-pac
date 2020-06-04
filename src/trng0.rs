#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRNG0 Miscellaneous Control Register"]
    pub trng0_mctl: TRNG0_MCTL,
    #[doc = "0x04 - TRNG0 Statistical Check Miscellaneous Register"]
    pub trng0_scmisc: TRNG0_SCMISC,
    #[doc = "0x08 - TRNG0 Poker Range Register"]
    pub trng0_pkrrng: TRNG0_PKRRNG,
    _reserved_3_trng0: [u8; 4usize],
    #[doc = "0x10 - TRNG0 Seed Control Register"]
    pub trng0_sdctl: TRNG0_SDCTL,
    _reserved_5_trng0: [u8; 4usize],
    #[doc = "0x18 - TRNG0 Frequency Count Minimum Limit Register"]
    pub trng0_frqmin: TRNG0_FRQMIN,
    _reserved_7_trng0: [u8; 4usize],
    _reserved_8_trng0: [u8; 4usize],
    _reserved_9_trng0_scr: [u8; 4usize],
    _reserved_10_trng0_scr: [u8; 4usize],
    _reserved_11_trng0_scr: [u8; 4usize],
    _reserved_12_trng0_scr: [u8; 4usize],
    _reserved_13_trng0_scr: [u8; 4usize],
    _reserved_14_trng0_scr: [u8; 4usize],
    #[doc = "0x3c - TRNG0 Status Register"]
    pub trng0_status: TRNG0_STATUS,
    #[doc = "0x40 - RNG TRNG Entropy Read Register"]
    pub trng0_ent0: TRNG0_ENT0,
    #[doc = "0x44 - RNG TRNG Entropy Read Register"]
    pub trng0_ent1: TRNG0_ENT1,
    #[doc = "0x48 - RNG TRNG Entropy Read Register"]
    pub trng0_ent2: TRNG0_ENT2,
    #[doc = "0x4c - RNG TRNG Entropy Read Register"]
    pub trng0_ent3: TRNG0_ENT3,
    #[doc = "0x50 - RNG TRNG Entropy Read Register"]
    pub trng0_ent4: TRNG0_ENT4,
    #[doc = "0x54 - RNG TRNG Entropy Read Register"]
    pub trng0_ent5: TRNG0_ENT5,
    #[doc = "0x58 - RNG TRNG Entropy Read Register"]
    pub trng0_ent6: TRNG0_ENT6,
    #[doc = "0x5c - RNG TRNG Entropy Read Register"]
    pub trng0_ent7: TRNG0_ENT7,
    #[doc = "0x60 - RNG TRNG Entropy Read Register"]
    pub trng0_ent8: TRNG0_ENT8,
    #[doc = "0x64 - RNG TRNG Entropy Read Register"]
    pub trng0_ent9: TRNG0_ENT9,
    #[doc = "0x68 - RNG TRNG Entropy Read Register"]
    pub trng0_ent10: TRNG0_ENT10,
    #[doc = "0x6c - RNG TRNG Entropy Read Register"]
    pub trng0_ent11: TRNG0_ENT11,
    #[doc = "0x70 - RNG TRNG Entropy Read Register"]
    pub trng0_ent12: TRNG0_ENT12,
    #[doc = "0x74 - RNG TRNG Entropy Read Register"]
    pub trng0_ent13: TRNG0_ENT13,
    #[doc = "0x78 - RNG TRNG Entropy Read Register"]
    pub trng0_ent14: TRNG0_ENT14,
    #[doc = "0x7c - RNG TRNG Entropy Read Register"]
    pub trng0_ent15: TRNG0_ENT15,
    #[doc = "0x80 - TRNG0 Statistical Check Poker Count 1 and 0 Register"]
    pub trng0_pkrcnt10: TRNG0_PKRCNT10,
    #[doc = "0x84 - TRNG0 Statistical Check Poker Count 3 and 2 Register"]
    pub trng0_pkrcnt32: TRNG0_PKRCNT32,
    #[doc = "0x88 - TRNG0 Statistical Check Poker Count 5 and 4 Register"]
    pub trng0_pkrcnt54: TRNG0_PKRCNT54,
    #[doc = "0x8c - TRNG0 Statistical Check Poker Count 7 and 6 Register"]
    pub trng0_pkrcnt76: TRNG0_PKRCNT76,
    #[doc = "0x90 - TRNG0 Statistical Check Poker Count 9 and 8 Register"]
    pub trng0_pkrcnt98: TRNG0_PKRCNT98,
    #[doc = "0x94 - TRNG0 Statistical Check Poker Count B and A Register"]
    pub trng0_pkrcntba: TRNG0_PKRCNTBA,
    #[doc = "0x98 - TRNG0 Statistical Check Poker Count D and C Register"]
    pub trng0_pkrcntdc: TRNG0_PKRCNTDC,
    #[doc = "0x9c - TRNG0 Statistical Check Poker Count F and E Register"]
    pub trng0_pkrcntfe: TRNG0_PKRCNTFE,
    #[doc = "0xa0 - TRNG0 Security Configuration Register"]
    pub trng0_sec_cfg: TRNG0_SEC_CFG,
    #[doc = "0xa4 - TRNG0 Interrupt Control Register"]
    pub trng0_int_ctrl: TRNG0_INT_CTRL,
    #[doc = "0xa8 - TRNG0 Mask Register"]
    pub trng0_int_mask: TRNG0_INT_MASK,
    #[doc = "0xac - TRNG0 Interrupt Status Register"]
    pub trng0_int_status: TRNG0_INT_STATUS,
    _reserved44: [u8; 64usize],
    #[doc = "0xf0 - TRNG0 Version ID Register (MS)"]
    pub trng0_vid1: TRNG0_VID1,
    #[doc = "0xf4 - TRNG0 Version ID Register (LS)"]
    pub trng0_vid2: TRNG0_VID2,
}
impl RegisterBlock {
    #[doc = "0x0c - TRNG0 Poker Square Calculation Result Register"]
    #[inline(always)]
    pub fn trng0_pkrsq(&self) -> &TRNG0_PKRSQ {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const TRNG0_PKRSQ) }
    }
    #[doc = "0x0c - TRNG0 Poker Square Calculation Result Register"]
    #[inline(always)]
    pub fn trng0_pkrsq_mut(&self) -> &mut TRNG0_PKRSQ {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut TRNG0_PKRSQ) }
    }
    #[doc = "0x0c - TRNG0 Poker Maximum Limit Register"]
    #[inline(always)]
    pub fn trng0_pkrmax(&self) -> &TRNG0_PKRMAX {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const TRNG0_PKRMAX) }
    }
    #[doc = "0x0c - TRNG0 Poker Maximum Limit Register"]
    #[inline(always)]
    pub fn trng0_pkrmax_mut(&self) -> &mut TRNG0_PKRMAX {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut TRNG0_PKRMAX) }
    }
    #[doc = "0x14 - TRNG0 Total Samples Register"]
    #[inline(always)]
    pub fn trng0_totsam(&self) -> &TRNG0_TOTSAM {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const TRNG0_TOTSAM) }
    }
    #[doc = "0x14 - TRNG0 Total Samples Register"]
    #[inline(always)]
    pub fn trng0_totsam_mut(&self) -> &mut TRNG0_TOTSAM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut TRNG0_TOTSAM) }
    }
    #[doc = "0x14 - TRNG0 Sparse Bit Limit Register"]
    #[inline(always)]
    pub fn trng0_sblim(&self) -> &TRNG0_SBLIM {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const TRNG0_SBLIM) }
    }
    #[doc = "0x14 - TRNG0 Sparse Bit Limit Register"]
    #[inline(always)]
    pub fn trng0_sblim_mut(&self) -> &mut TRNG0_SBLIM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut TRNG0_SBLIM) }
    }
    #[doc = "0x1c - TRNG0 Frequency Count Maximum Limit Register"]
    #[inline(always)]
    pub fn trng0_frqmax(&self) -> &TRNG0_FRQMAX {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const TRNG0_FRQMAX) }
    }
    #[doc = "0x1c - TRNG0 Frequency Count Maximum Limit Register"]
    #[inline(always)]
    pub fn trng0_frqmax_mut(&self) -> &mut TRNG0_FRQMAX {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut TRNG0_FRQMAX) }
    }
    #[doc = "0x1c - TRNG0 Frequency Count Register"]
    #[inline(always)]
    pub fn trng0_frqcnt(&self) -> &TRNG0_FRQCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const TRNG0_FRQCNT) }
    }
    #[doc = "0x1c - TRNG0 Frequency Count Register"]
    #[inline(always)]
    pub fn trng0_frqcnt_mut(&self) -> &mut TRNG0_FRQCNT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut TRNG0_FRQCNT) }
    }
    #[doc = "0x20 - TRNG0 Statistical Check Monobit Limit Register"]
    #[inline(always)]
    pub fn trng0_scml(&self) -> &TRNG0_SCML {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const TRNG0_SCML) }
    }
    #[doc = "0x20 - TRNG0 Statistical Check Monobit Limit Register"]
    #[inline(always)]
    pub fn trng0_scml_mut(&self) -> &mut TRNG0_SCML {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut TRNG0_SCML) }
    }
    #[doc = "0x20 - TRNG0 Statistical Check Monobit Count Register"]
    #[inline(always)]
    pub fn trng0_scmc(&self) -> &TRNG0_SCMC {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const TRNG0_SCMC) }
    }
    #[doc = "0x20 - TRNG0 Statistical Check Monobit Count Register"]
    #[inline(always)]
    pub fn trng0_scmc_mut(&self) -> &mut TRNG0_SCMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut TRNG0_SCMC) }
    }
    #[doc = "0x24 - TRNG0 Statistical Check Run Length 1 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr1l(&self) -> &TRNG0_SCR1L {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const TRNG0_SCR1L) }
    }
    #[doc = "0x24 - TRNG0 Statistical Check Run Length 1 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr1l_mut(&self) -> &mut TRNG0_SCR1L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut TRNG0_SCR1L) }
    }
    #[doc = "0x24 - TRNG0 Statistical Check Run Length 1 Count Register"]
    #[inline(always)]
    pub fn trng0_scr1c(&self) -> &TRNG0_SCR1C {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const TRNG0_SCR1C) }
    }
    #[doc = "0x24 - TRNG0 Statistical Check Run Length 1 Count Register"]
    #[inline(always)]
    pub fn trng0_scr1c_mut(&self) -> &mut TRNG0_SCR1C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut TRNG0_SCR1C) }
    }
    #[doc = "0x28 - TRNG0 Statistical Check Run Length 2 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr2l(&self) -> &TRNG0_SCR2L {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const TRNG0_SCR2L) }
    }
    #[doc = "0x28 - TRNG0 Statistical Check Run Length 2 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr2l_mut(&self) -> &mut TRNG0_SCR2L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut TRNG0_SCR2L) }
    }
    #[doc = "0x28 - TRNG0 Statistical Check Run Length 2 Count Register"]
    #[inline(always)]
    pub fn trng0_scr2c(&self) -> &TRNG0_SCR2C {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const TRNG0_SCR2C) }
    }
    #[doc = "0x28 - TRNG0 Statistical Check Run Length 2 Count Register"]
    #[inline(always)]
    pub fn trng0_scr2c_mut(&self) -> &mut TRNG0_SCR2C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut TRNG0_SCR2C) }
    }
    #[doc = "0x2c - TRNG0 Statistical Check Run Length 3 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr3l(&self) -> &TRNG0_SCR3L {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const TRNG0_SCR3L) }
    }
    #[doc = "0x2c - TRNG0 Statistical Check Run Length 3 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr3l_mut(&self) -> &mut TRNG0_SCR3L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut TRNG0_SCR3L) }
    }
    #[doc = "0x2c - TRNG0 Statistical Check Run Length 3 Count Register"]
    #[inline(always)]
    pub fn trng0_scr3c(&self) -> &TRNG0_SCR3C {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const TRNG0_SCR3C) }
    }
    #[doc = "0x2c - TRNG0 Statistical Check Run Length 3 Count Register"]
    #[inline(always)]
    pub fn trng0_scr3c_mut(&self) -> &mut TRNG0_SCR3C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut TRNG0_SCR3C) }
    }
    #[doc = "0x30 - TRNG0 Statistical Check Run Length 4 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr4l(&self) -> &TRNG0_SCR4L {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const TRNG0_SCR4L) }
    }
    #[doc = "0x30 - TRNG0 Statistical Check Run Length 4 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr4l_mut(&self) -> &mut TRNG0_SCR4L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut TRNG0_SCR4L) }
    }
    #[doc = "0x30 - TRNG0 Statistical Check Run Length 4 Count Register"]
    #[inline(always)]
    pub fn trng0_scr4c(&self) -> &TRNG0_SCR4C {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const TRNG0_SCR4C) }
    }
    #[doc = "0x30 - TRNG0 Statistical Check Run Length 4 Count Register"]
    #[inline(always)]
    pub fn trng0_scr4c_mut(&self) -> &mut TRNG0_SCR4C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut TRNG0_SCR4C) }
    }
    #[doc = "0x34 - TRNG0 Statistical Check Run Length 5 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr5l(&self) -> &TRNG0_SCR5L {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const TRNG0_SCR5L) }
    }
    #[doc = "0x34 - TRNG0 Statistical Check Run Length 5 Limit Register"]
    #[inline(always)]
    pub fn trng0_scr5l_mut(&self) -> &mut TRNG0_SCR5L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut TRNG0_SCR5L) }
    }
    #[doc = "0x34 - TRNG0 Statistical Check Run Length 5 Count Register"]
    #[inline(always)]
    pub fn trng0_scr5c(&self) -> &TRNG0_SCR5C {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const TRNG0_SCR5C) }
    }
    #[doc = "0x34 - TRNG0 Statistical Check Run Length 5 Count Register"]
    #[inline(always)]
    pub fn trng0_scr5c_mut(&self) -> &mut TRNG0_SCR5C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut TRNG0_SCR5C) }
    }
    #[doc = "0x38 - TRNG0 Statistical Check Run Length 6+ Limit Register"]
    #[inline(always)]
    pub fn trng0_scr6pl(&self) -> &TRNG0_SCR6PL {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const TRNG0_SCR6PL) }
    }
    #[doc = "0x38 - TRNG0 Statistical Check Run Length 6+ Limit Register"]
    #[inline(always)]
    pub fn trng0_scr6pl_mut(&self) -> &mut TRNG0_SCR6PL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut TRNG0_SCR6PL) }
    }
    #[doc = "0x38 - TRNG0 Statistical Check Run Length 6+ Count Register"]
    #[inline(always)]
    pub fn trng0_scr6pc(&self) -> &TRNG0_SCR6PC {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const TRNG0_SCR6PC) }
    }
    #[doc = "0x38 - TRNG0 Statistical Check Run Length 6+ Count Register"]
    #[inline(always)]
    pub fn trng0_scr6pc_mut(&self) -> &mut TRNG0_SCR6PC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut TRNG0_SCR6PC) }
    }
}
#[doc = "TRNG0 Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_mctl](trng0_mctl) module"]
pub type TRNG0_MCTL = crate::Reg<u32, _TRNG0_MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_MCTL;
#[doc = "`read()` method returns [trng0_mctl::R](trng0_mctl::R) reader structure"]
impl crate::Readable for TRNG0_MCTL {}
#[doc = "`write(|w| ..)` method takes [trng0_mctl::W](trng0_mctl::W) writer structure"]
impl crate::Writable for TRNG0_MCTL {}
#[doc = "TRNG0 Miscellaneous Control Register"]
pub mod trng0_mctl;
#[doc = "TRNG0 Statistical Check Miscellaneous Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scmisc](trng0_scmisc) module"]
pub type TRNG0_SCMISC = crate::Reg<u32, _TRNG0_SCMISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCMISC;
#[doc = "`read()` method returns [trng0_scmisc::R](trng0_scmisc::R) reader structure"]
impl crate::Readable for TRNG0_SCMISC {}
#[doc = "`write(|w| ..)` method takes [trng0_scmisc::W](trng0_scmisc::W) writer structure"]
impl crate::Writable for TRNG0_SCMISC {}
#[doc = "TRNG0 Statistical Check Miscellaneous Register"]
pub mod trng0_scmisc;
#[doc = "TRNG0 Poker Range Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrrng](trng0_pkrrng) module"]
pub type TRNG0_PKRRNG = crate::Reg<u32, _TRNG0_PKRRNG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRRNG;
#[doc = "`read()` method returns [trng0_pkrrng::R](trng0_pkrrng::R) reader structure"]
impl crate::Readable for TRNG0_PKRRNG {}
#[doc = "`write(|w| ..)` method takes [trng0_pkrrng::W](trng0_pkrrng::W) writer structure"]
impl crate::Writable for TRNG0_PKRRNG {}
#[doc = "TRNG0 Poker Range Register"]
pub mod trng0_pkrrng;
#[doc = "TRNG0 Poker Maximum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrmax](trng0_pkrmax) module"]
pub type TRNG0_PKRMAX = crate::Reg<u32, _TRNG0_PKRMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRMAX;
#[doc = "`read()` method returns [trng0_pkrmax::R](trng0_pkrmax::R) reader structure"]
impl crate::Readable for TRNG0_PKRMAX {}
#[doc = "`write(|w| ..)` method takes [trng0_pkrmax::W](trng0_pkrmax::W) writer structure"]
impl crate::Writable for TRNG0_PKRMAX {}
#[doc = "TRNG0 Poker Maximum Limit Register"]
pub mod trng0_pkrmax;
#[doc = "TRNG0 Poker Square Calculation Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrsq](trng0_pkrsq) module"]
pub type TRNG0_PKRSQ = crate::Reg<u32, _TRNG0_PKRSQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRSQ;
#[doc = "`read()` method returns [trng0_pkrsq::R](trng0_pkrsq::R) reader structure"]
impl crate::Readable for TRNG0_PKRSQ {}
#[doc = "TRNG0 Poker Square Calculation Result Register"]
pub mod trng0_pkrsq;
#[doc = "TRNG0 Seed Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_sdctl](trng0_sdctl) module"]
pub type TRNG0_SDCTL = crate::Reg<u32, _TRNG0_SDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SDCTL;
#[doc = "`read()` method returns [trng0_sdctl::R](trng0_sdctl::R) reader structure"]
impl crate::Readable for TRNG0_SDCTL {}
#[doc = "`write(|w| ..)` method takes [trng0_sdctl::W](trng0_sdctl::W) writer structure"]
impl crate::Writable for TRNG0_SDCTL {}
#[doc = "TRNG0 Seed Control Register"]
pub mod trng0_sdctl;
#[doc = "TRNG0 Sparse Bit Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_sblim](trng0_sblim) module"]
pub type TRNG0_SBLIM = crate::Reg<u32, _TRNG0_SBLIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SBLIM;
#[doc = "`read()` method returns [trng0_sblim::R](trng0_sblim::R) reader structure"]
impl crate::Readable for TRNG0_SBLIM {}
#[doc = "`write(|w| ..)` method takes [trng0_sblim::W](trng0_sblim::W) writer structure"]
impl crate::Writable for TRNG0_SBLIM {}
#[doc = "TRNG0 Sparse Bit Limit Register"]
pub mod trng0_sblim;
#[doc = "TRNG0 Total Samples Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_totsam](trng0_totsam) module"]
pub type TRNG0_TOTSAM = crate::Reg<u32, _TRNG0_TOTSAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_TOTSAM;
#[doc = "`read()` method returns [trng0_totsam::R](trng0_totsam::R) reader structure"]
impl crate::Readable for TRNG0_TOTSAM {}
#[doc = "TRNG0 Total Samples Register"]
pub mod trng0_totsam;
#[doc = "TRNG0 Frequency Count Minimum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_frqmin](trng0_frqmin) module"]
pub type TRNG0_FRQMIN = crate::Reg<u32, _TRNG0_FRQMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_FRQMIN;
#[doc = "`read()` method returns [trng0_frqmin::R](trng0_frqmin::R) reader structure"]
impl crate::Readable for TRNG0_FRQMIN {}
#[doc = "`write(|w| ..)` method takes [trng0_frqmin::W](trng0_frqmin::W) writer structure"]
impl crate::Writable for TRNG0_FRQMIN {}
#[doc = "TRNG0 Frequency Count Minimum Limit Register"]
pub mod trng0_frqmin;
#[doc = "TRNG0 Frequency Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_frqcnt](trng0_frqcnt) module"]
pub type TRNG0_FRQCNT = crate::Reg<u32, _TRNG0_FRQCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_FRQCNT;
#[doc = "`read()` method returns [trng0_frqcnt::R](trng0_frqcnt::R) reader structure"]
impl crate::Readable for TRNG0_FRQCNT {}
#[doc = "TRNG0 Frequency Count Register"]
pub mod trng0_frqcnt;
#[doc = "TRNG0 Frequency Count Maximum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_frqmax](trng0_frqmax) module"]
pub type TRNG0_FRQMAX = crate::Reg<u32, _TRNG0_FRQMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_FRQMAX;
#[doc = "`read()` method returns [trng0_frqmax::R](trng0_frqmax::R) reader structure"]
impl crate::Readable for TRNG0_FRQMAX {}
#[doc = "`write(|w| ..)` method takes [trng0_frqmax::W](trng0_frqmax::W) writer structure"]
impl crate::Writable for TRNG0_FRQMAX {}
#[doc = "TRNG0 Frequency Count Maximum Limit Register"]
pub mod trng0_frqmax;
#[doc = "TRNG0 Statistical Check Monobit Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scmc](trng0_scmc) module"]
pub type TRNG0_SCMC = crate::Reg<u32, _TRNG0_SCMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCMC;
#[doc = "`read()` method returns [trng0_scmc::R](trng0_scmc::R) reader structure"]
impl crate::Readable for TRNG0_SCMC {}
#[doc = "TRNG0 Statistical Check Monobit Count Register"]
pub mod trng0_scmc;
#[doc = "TRNG0 Statistical Check Monobit Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scml](trng0_scml) module"]
pub type TRNG0_SCML = crate::Reg<u32, _TRNG0_SCML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCML;
#[doc = "`read()` method returns [trng0_scml::R](trng0_scml::R) reader structure"]
impl crate::Readable for TRNG0_SCML {}
#[doc = "`write(|w| ..)` method takes [trng0_scml::W](trng0_scml::W) writer structure"]
impl crate::Writable for TRNG0_SCML {}
#[doc = "TRNG0 Statistical Check Monobit Limit Register"]
pub mod trng0_scml;
#[doc = "TRNG0 Statistical Check Run Length 1 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr1c](trng0_scr1c) module"]
pub type TRNG0_SCR1C = crate::Reg<u32, _TRNG0_SCR1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR1C;
#[doc = "`read()` method returns [trng0_scr1c::R](trng0_scr1c::R) reader structure"]
impl crate::Readable for TRNG0_SCR1C {}
#[doc = "TRNG0 Statistical Check Run Length 1 Count Register"]
pub mod trng0_scr1c;
#[doc = "TRNG0 Statistical Check Run Length 1 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr1l](trng0_scr1l) module"]
pub type TRNG0_SCR1L = crate::Reg<u32, _TRNG0_SCR1L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR1L;
#[doc = "`read()` method returns [trng0_scr1l::R](trng0_scr1l::R) reader structure"]
impl crate::Readable for TRNG0_SCR1L {}
#[doc = "`write(|w| ..)` method takes [trng0_scr1l::W](trng0_scr1l::W) writer structure"]
impl crate::Writable for TRNG0_SCR1L {}
#[doc = "TRNG0 Statistical Check Run Length 1 Limit Register"]
pub mod trng0_scr1l;
#[doc = "TRNG0 Statistical Check Run Length 2 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr2c](trng0_scr2c) module"]
pub type TRNG0_SCR2C = crate::Reg<u32, _TRNG0_SCR2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR2C;
#[doc = "`read()` method returns [trng0_scr2c::R](trng0_scr2c::R) reader structure"]
impl crate::Readable for TRNG0_SCR2C {}
#[doc = "TRNG0 Statistical Check Run Length 2 Count Register"]
pub mod trng0_scr2c;
#[doc = "TRNG0 Statistical Check Run Length 2 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr2l](trng0_scr2l) module"]
pub type TRNG0_SCR2L = crate::Reg<u32, _TRNG0_SCR2L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR2L;
#[doc = "`read()` method returns [trng0_scr2l::R](trng0_scr2l::R) reader structure"]
impl crate::Readable for TRNG0_SCR2L {}
#[doc = "`write(|w| ..)` method takes [trng0_scr2l::W](trng0_scr2l::W) writer structure"]
impl crate::Writable for TRNG0_SCR2L {}
#[doc = "TRNG0 Statistical Check Run Length 2 Limit Register"]
pub mod trng0_scr2l;
#[doc = "TRNG0 Statistical Check Run Length 3 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr3c](trng0_scr3c) module"]
pub type TRNG0_SCR3C = crate::Reg<u32, _TRNG0_SCR3C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR3C;
#[doc = "`read()` method returns [trng0_scr3c::R](trng0_scr3c::R) reader structure"]
impl crate::Readable for TRNG0_SCR3C {}
#[doc = "TRNG0 Statistical Check Run Length 3 Count Register"]
pub mod trng0_scr3c;
#[doc = "TRNG0 Statistical Check Run Length 3 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr3l](trng0_scr3l) module"]
pub type TRNG0_SCR3L = crate::Reg<u32, _TRNG0_SCR3L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR3L;
#[doc = "`read()` method returns [trng0_scr3l::R](trng0_scr3l::R) reader structure"]
impl crate::Readable for TRNG0_SCR3L {}
#[doc = "`write(|w| ..)` method takes [trng0_scr3l::W](trng0_scr3l::W) writer structure"]
impl crate::Writable for TRNG0_SCR3L {}
#[doc = "TRNG0 Statistical Check Run Length 3 Limit Register"]
pub mod trng0_scr3l;
#[doc = "TRNG0 Statistical Check Run Length 4 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr4c](trng0_scr4c) module"]
pub type TRNG0_SCR4C = crate::Reg<u32, _TRNG0_SCR4C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR4C;
#[doc = "`read()` method returns [trng0_scr4c::R](trng0_scr4c::R) reader structure"]
impl crate::Readable for TRNG0_SCR4C {}
#[doc = "TRNG0 Statistical Check Run Length 4 Count Register"]
pub mod trng0_scr4c;
#[doc = "TRNG0 Statistical Check Run Length 4 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr4l](trng0_scr4l) module"]
pub type TRNG0_SCR4L = crate::Reg<u32, _TRNG0_SCR4L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR4L;
#[doc = "`read()` method returns [trng0_scr4l::R](trng0_scr4l::R) reader structure"]
impl crate::Readable for TRNG0_SCR4L {}
#[doc = "`write(|w| ..)` method takes [trng0_scr4l::W](trng0_scr4l::W) writer structure"]
impl crate::Writable for TRNG0_SCR4L {}
#[doc = "TRNG0 Statistical Check Run Length 4 Limit Register"]
pub mod trng0_scr4l;
#[doc = "TRNG0 Statistical Check Run Length 5 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr5c](trng0_scr5c) module"]
pub type TRNG0_SCR5C = crate::Reg<u32, _TRNG0_SCR5C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR5C;
#[doc = "`read()` method returns [trng0_scr5c::R](trng0_scr5c::R) reader structure"]
impl crate::Readable for TRNG0_SCR5C {}
#[doc = "TRNG0 Statistical Check Run Length 5 Count Register"]
pub mod trng0_scr5c;
#[doc = "TRNG0 Statistical Check Run Length 5 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr5l](trng0_scr5l) module"]
pub type TRNG0_SCR5L = crate::Reg<u32, _TRNG0_SCR5L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR5L;
#[doc = "`read()` method returns [trng0_scr5l::R](trng0_scr5l::R) reader structure"]
impl crate::Readable for TRNG0_SCR5L {}
#[doc = "`write(|w| ..)` method takes [trng0_scr5l::W](trng0_scr5l::W) writer structure"]
impl crate::Writable for TRNG0_SCR5L {}
#[doc = "TRNG0 Statistical Check Run Length 5 Limit Register"]
pub mod trng0_scr5l;
#[doc = "TRNG0 Statistical Check Run Length 6+ Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr6pc](trng0_scr6pc) module"]
pub type TRNG0_SCR6PC = crate::Reg<u32, _TRNG0_SCR6PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR6PC;
#[doc = "`read()` method returns [trng0_scr6pc::R](trng0_scr6pc::R) reader structure"]
impl crate::Readable for TRNG0_SCR6PC {}
#[doc = "TRNG0 Statistical Check Run Length 6+ Count Register"]
pub mod trng0_scr6pc;
#[doc = "TRNG0 Statistical Check Run Length 6+ Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scr6pl](trng0_scr6pl) module"]
pub type TRNG0_SCR6PL = crate::Reg<u32, _TRNG0_SCR6PL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SCR6PL;
#[doc = "`read()` method returns [trng0_scr6pl::R](trng0_scr6pl::R) reader structure"]
impl crate::Readable for TRNG0_SCR6PL {}
#[doc = "`write(|w| ..)` method takes [trng0_scr6pl::W](trng0_scr6pl::W) writer structure"]
impl crate::Writable for TRNG0_SCR6PL {}
#[doc = "TRNG0 Statistical Check Run Length 6+ Limit Register"]
pub mod trng0_scr6pl;
#[doc = "TRNG0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_status](trng0_status) module"]
pub type TRNG0_STATUS = crate::Reg<u32, _TRNG0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_STATUS;
#[doc = "`read()` method returns [trng0_status::R](trng0_status::R) reader structure"]
impl crate::Readable for TRNG0_STATUS {}
#[doc = "TRNG0 Status Register"]
pub mod trng0_status;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent0](trng0_ent0) module"]
pub type TRNG0_ENT0 = crate::Reg<u32, _TRNG0_ENT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT0;
#[doc = "`read()` method returns [trng0_ent0::R](trng0_ent0::R) reader structure"]
impl crate::Readable for TRNG0_ENT0 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent0;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent1](trng0_ent1) module"]
pub type TRNG0_ENT1 = crate::Reg<u32, _TRNG0_ENT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT1;
#[doc = "`read()` method returns [trng0_ent1::R](trng0_ent1::R) reader structure"]
impl crate::Readable for TRNG0_ENT1 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent1;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent2](trng0_ent2) module"]
pub type TRNG0_ENT2 = crate::Reg<u32, _TRNG0_ENT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT2;
#[doc = "`read()` method returns [trng0_ent2::R](trng0_ent2::R) reader structure"]
impl crate::Readable for TRNG0_ENT2 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent2;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent3](trng0_ent3) module"]
pub type TRNG0_ENT3 = crate::Reg<u32, _TRNG0_ENT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT3;
#[doc = "`read()` method returns [trng0_ent3::R](trng0_ent3::R) reader structure"]
impl crate::Readable for TRNG0_ENT3 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent3;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent4](trng0_ent4) module"]
pub type TRNG0_ENT4 = crate::Reg<u32, _TRNG0_ENT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT4;
#[doc = "`read()` method returns [trng0_ent4::R](trng0_ent4::R) reader structure"]
impl crate::Readable for TRNG0_ENT4 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent4;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent5](trng0_ent5) module"]
pub type TRNG0_ENT5 = crate::Reg<u32, _TRNG0_ENT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT5;
#[doc = "`read()` method returns [trng0_ent5::R](trng0_ent5::R) reader structure"]
impl crate::Readable for TRNG0_ENT5 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent5;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent6](trng0_ent6) module"]
pub type TRNG0_ENT6 = crate::Reg<u32, _TRNG0_ENT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT6;
#[doc = "`read()` method returns [trng0_ent6::R](trng0_ent6::R) reader structure"]
impl crate::Readable for TRNG0_ENT6 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent6;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent7](trng0_ent7) module"]
pub type TRNG0_ENT7 = crate::Reg<u32, _TRNG0_ENT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT7;
#[doc = "`read()` method returns [trng0_ent7::R](trng0_ent7::R) reader structure"]
impl crate::Readable for TRNG0_ENT7 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent7;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent8](trng0_ent8) module"]
pub type TRNG0_ENT8 = crate::Reg<u32, _TRNG0_ENT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT8;
#[doc = "`read()` method returns [trng0_ent8::R](trng0_ent8::R) reader structure"]
impl crate::Readable for TRNG0_ENT8 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent8;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent9](trng0_ent9) module"]
pub type TRNG0_ENT9 = crate::Reg<u32, _TRNG0_ENT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT9;
#[doc = "`read()` method returns [trng0_ent9::R](trng0_ent9::R) reader structure"]
impl crate::Readable for TRNG0_ENT9 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent9;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent10](trng0_ent10) module"]
pub type TRNG0_ENT10 = crate::Reg<u32, _TRNG0_ENT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT10;
#[doc = "`read()` method returns [trng0_ent10::R](trng0_ent10::R) reader structure"]
impl crate::Readable for TRNG0_ENT10 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent10;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent11](trng0_ent11) module"]
pub type TRNG0_ENT11 = crate::Reg<u32, _TRNG0_ENT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT11;
#[doc = "`read()` method returns [trng0_ent11::R](trng0_ent11::R) reader structure"]
impl crate::Readable for TRNG0_ENT11 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent11;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent12](trng0_ent12) module"]
pub type TRNG0_ENT12 = crate::Reg<u32, _TRNG0_ENT12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT12;
#[doc = "`read()` method returns [trng0_ent12::R](trng0_ent12::R) reader structure"]
impl crate::Readable for TRNG0_ENT12 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent12;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent13](trng0_ent13) module"]
pub type TRNG0_ENT13 = crate::Reg<u32, _TRNG0_ENT13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT13;
#[doc = "`read()` method returns [trng0_ent13::R](trng0_ent13::R) reader structure"]
impl crate::Readable for TRNG0_ENT13 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent13;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent14](trng0_ent14) module"]
pub type TRNG0_ENT14 = crate::Reg<u32, _TRNG0_ENT14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT14;
#[doc = "`read()` method returns [trng0_ent14::R](trng0_ent14::R) reader structure"]
impl crate::Readable for TRNG0_ENT14 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent14;
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent15](trng0_ent15) module"]
pub type TRNG0_ENT15 = crate::Reg<u32, _TRNG0_ENT15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_ENT15;
#[doc = "`read()` method returns [trng0_ent15::R](trng0_ent15::R) reader structure"]
impl crate::Readable for TRNG0_ENT15 {}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent15;
#[doc = "TRNG0 Statistical Check Poker Count 1 and 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcnt10](trng0_pkrcnt10) module"]
pub type TRNG0_PKRCNT10 = crate::Reg<u32, _TRNG0_PKRCNT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRCNT10;
#[doc = "`read()` method returns [trng0_pkrcnt10::R](trng0_pkrcnt10::R) reader structure"]
impl crate::Readable for TRNG0_PKRCNT10 {}
#[doc = "TRNG0 Statistical Check Poker Count 1 and 0 Register"]
pub mod trng0_pkrcnt10;
#[doc = "TRNG0 Statistical Check Poker Count 3 and 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcnt32](trng0_pkrcnt32) module"]
pub type TRNG0_PKRCNT32 = crate::Reg<u32, _TRNG0_PKRCNT32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRCNT32;
#[doc = "`read()` method returns [trng0_pkrcnt32::R](trng0_pkrcnt32::R) reader structure"]
impl crate::Readable for TRNG0_PKRCNT32 {}
#[doc = "TRNG0 Statistical Check Poker Count 3 and 2 Register"]
pub mod trng0_pkrcnt32;
#[doc = "TRNG0 Statistical Check Poker Count 5 and 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcnt54](trng0_pkrcnt54) module"]
pub type TRNG0_PKRCNT54 = crate::Reg<u32, _TRNG0_PKRCNT54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRCNT54;
#[doc = "`read()` method returns [trng0_pkrcnt54::R](trng0_pkrcnt54::R) reader structure"]
impl crate::Readable for TRNG0_PKRCNT54 {}
#[doc = "TRNG0 Statistical Check Poker Count 5 and 4 Register"]
pub mod trng0_pkrcnt54;
#[doc = "TRNG0 Statistical Check Poker Count 7 and 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcnt76](trng0_pkrcnt76) module"]
pub type TRNG0_PKRCNT76 = crate::Reg<u32, _TRNG0_PKRCNT76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRCNT76;
#[doc = "`read()` method returns [trng0_pkrcnt76::R](trng0_pkrcnt76::R) reader structure"]
impl crate::Readable for TRNG0_PKRCNT76 {}
#[doc = "TRNG0 Statistical Check Poker Count 7 and 6 Register"]
pub mod trng0_pkrcnt76;
#[doc = "TRNG0 Statistical Check Poker Count 9 and 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcnt98](trng0_pkrcnt98) module"]
pub type TRNG0_PKRCNT98 = crate::Reg<u32, _TRNG0_PKRCNT98>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRCNT98;
#[doc = "`read()` method returns [trng0_pkrcnt98::R](trng0_pkrcnt98::R) reader structure"]
impl crate::Readable for TRNG0_PKRCNT98 {}
#[doc = "TRNG0 Statistical Check Poker Count 9 and 8 Register"]
pub mod trng0_pkrcnt98;
#[doc = "TRNG0 Statistical Check Poker Count B and A Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcntba](trng0_pkrcntba) module"]
pub type TRNG0_PKRCNTBA = crate::Reg<u32, _TRNG0_PKRCNTBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRCNTBA;
#[doc = "`read()` method returns [trng0_pkrcntba::R](trng0_pkrcntba::R) reader structure"]
impl crate::Readable for TRNG0_PKRCNTBA {}
#[doc = "TRNG0 Statistical Check Poker Count B and A Register"]
pub mod trng0_pkrcntba;
#[doc = "TRNG0 Statistical Check Poker Count D and C Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcntdc](trng0_pkrcntdc) module"]
pub type TRNG0_PKRCNTDC = crate::Reg<u32, _TRNG0_PKRCNTDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRCNTDC;
#[doc = "`read()` method returns [trng0_pkrcntdc::R](trng0_pkrcntdc::R) reader structure"]
impl crate::Readable for TRNG0_PKRCNTDC {}
#[doc = "TRNG0 Statistical Check Poker Count D and C Register"]
pub mod trng0_pkrcntdc;
#[doc = "TRNG0 Statistical Check Poker Count F and E Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcntfe](trng0_pkrcntfe) module"]
pub type TRNG0_PKRCNTFE = crate::Reg<u32, _TRNG0_PKRCNTFE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_PKRCNTFE;
#[doc = "`read()` method returns [trng0_pkrcntfe::R](trng0_pkrcntfe::R) reader structure"]
impl crate::Readable for TRNG0_PKRCNTFE {}
#[doc = "TRNG0 Statistical Check Poker Count F and E Register"]
pub mod trng0_pkrcntfe;
#[doc = "TRNG0 Security Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_sec_cfg](trng0_sec_cfg) module"]
pub type TRNG0_SEC_CFG = crate::Reg<u32, _TRNG0_SEC_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_SEC_CFG;
#[doc = "`read()` method returns [trng0_sec_cfg::R](trng0_sec_cfg::R) reader structure"]
impl crate::Readable for TRNG0_SEC_CFG {}
#[doc = "`write(|w| ..)` method takes [trng0_sec_cfg::W](trng0_sec_cfg::W) writer structure"]
impl crate::Writable for TRNG0_SEC_CFG {}
#[doc = "TRNG0 Security Configuration Register"]
pub mod trng0_sec_cfg;
#[doc = "TRNG0 Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_int_ctrl](trng0_int_ctrl) module"]
pub type TRNG0_INT_CTRL = crate::Reg<u32, _TRNG0_INT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_INT_CTRL;
#[doc = "`read()` method returns [trng0_int_ctrl::R](trng0_int_ctrl::R) reader structure"]
impl crate::Readable for TRNG0_INT_CTRL {}
#[doc = "`write(|w| ..)` method takes [trng0_int_ctrl::W](trng0_int_ctrl::W) writer structure"]
impl crate::Writable for TRNG0_INT_CTRL {}
#[doc = "TRNG0 Interrupt Control Register"]
pub mod trng0_int_ctrl;
#[doc = "TRNG0 Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_int_mask](trng0_int_mask) module"]
pub type TRNG0_INT_MASK = crate::Reg<u32, _TRNG0_INT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_INT_MASK;
#[doc = "`read()` method returns [trng0_int_mask::R](trng0_int_mask::R) reader structure"]
impl crate::Readable for TRNG0_INT_MASK {}
#[doc = "`write(|w| ..)` method takes [trng0_int_mask::W](trng0_int_mask::W) writer structure"]
impl crate::Writable for TRNG0_INT_MASK {}
#[doc = "TRNG0 Mask Register"]
pub mod trng0_int_mask;
#[doc = "TRNG0 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_int_status](trng0_int_status) module"]
pub type TRNG0_INT_STATUS = crate::Reg<u32, _TRNG0_INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_INT_STATUS;
#[doc = "`read()` method returns [trng0_int_status::R](trng0_int_status::R) reader structure"]
impl crate::Readable for TRNG0_INT_STATUS {}
#[doc = "`write(|w| ..)` method takes [trng0_int_status::W](trng0_int_status::W) writer structure"]
impl crate::Writable for TRNG0_INT_STATUS {}
#[doc = "TRNG0 Interrupt Status Register"]
pub mod trng0_int_status;
#[doc = "TRNG0 Version ID Register (MS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_vid1](trng0_vid1) module"]
pub type TRNG0_VID1 = crate::Reg<u32, _TRNG0_VID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_VID1;
#[doc = "`read()` method returns [trng0_vid1::R](trng0_vid1::R) reader structure"]
impl crate::Readable for TRNG0_VID1 {}
#[doc = "TRNG0 Version ID Register (MS)"]
pub mod trng0_vid1;
#[doc = "TRNG0 Version ID Register (LS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_vid2](trng0_vid2) module"]
pub type TRNG0_VID2 = crate::Reg<u32, _TRNG0_VID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG0_VID2;
#[doc = "`read()` method returns [trng0_vid2::R](trng0_vid2::R) reader structure"]
impl crate::Readable for TRNG0_VID2 {}
#[doc = "TRNG0 Version ID Register (LS)"]
pub mod trng0_vid2;
