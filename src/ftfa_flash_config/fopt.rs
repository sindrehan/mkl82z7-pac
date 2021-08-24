#[doc = "Register `FOPT` reader"]
pub struct R(crate::R<FOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT_A {
    #[doc = "0: Low-power boot"]
    _00 = 0,
    #[doc = "1: Normal boot"]
    _01 = 1,
}
impl From<LPBOOT_A> for bool {
    #[inline(always)]
    fn from(variant: LPBOOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBOOT` reader - no description available"]
pub struct LPBOOT_R(crate::FieldReader<bool, LPBOOT_A>);
impl LPBOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPBOOT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBOOT_A {
        match self.bits {
            false => LPBOOT_A::_00,
            true => LPBOOT_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LPBOOT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LPBOOT_A::_01
    }
}
impl core::ops::Deref for LPBOOT_R {
    type Target = crate::FieldReader<bool, LPBOOT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTPIN_OPT_A {
    #[doc = "0: Force Boot from ROM if BOOTCFG0 asserted, where BOOTCFG0 is the boot config function which is muxed with NMI pin"]
    _00 = 0,
    #[doc = "1: Boot source configured by FOPT (BOOTSRC_SEL) bits"]
    _01 = 1,
}
impl From<BOOTPIN_OPT_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTPIN_OPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTPIN_OPT` reader - no description available"]
pub struct BOOTPIN_OPT_R(crate::FieldReader<bool, BOOTPIN_OPT_A>);
impl BOOTPIN_OPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOTPIN_OPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTPIN_OPT_A {
        match self.bits {
            false => BOOTPIN_OPT_A::_00,
            true => BOOTPIN_OPT_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == BOOTPIN_OPT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == BOOTPIN_OPT_A::_01
    }
}
impl core::ops::Deref for BOOTPIN_OPT_R {
    type Target = crate::FieldReader<bool, BOOTPIN_OPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_DIS_A {
    #[doc = "0: NMI interrupts are always blocked"]
    _00 = 0,
    #[doc = "1: NMI_b pin/interrupts reset default to enabled"]
    _01 = 1,
}
impl From<NMI_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: NMI_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMI_DIS` reader - no description available"]
pub struct NMI_DIS_R(crate::FieldReader<bool, NMI_DIS_A>);
impl NMI_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMI_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMI_DIS_A {
        match self.bits {
            false => NMI_DIS_A::_00,
            true => NMI_DIS_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == NMI_DIS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == NMI_DIS_A::_01
    }
}
impl core::ops::Deref for NMI_DIS_R {
    type Target = crate::FieldReader<bool, NMI_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_INIT_A {
    #[doc = "0: Slower initialization"]
    _00 = 0,
    #[doc = "1: Fast Initialization"]
    _01 = 1,
}
impl From<FAST_INIT_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST_INIT` reader - no description available"]
pub struct FAST_INIT_R(crate::FieldReader<bool, FAST_INIT_A>);
impl FAST_INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAST_INIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_INIT_A {
        match self.bits {
            false => FAST_INIT_A::_00,
            true => FAST_INIT_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FAST_INIT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FAST_INIT_A::_01
    }
}
impl core::ops::Deref for FAST_INIT_R {
    type Target = crate::FieldReader<bool, FAST_INIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boot source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOTSRC_SEL_A {
    #[doc = "0: Boot from Flash"]
    _00 = 0,
    #[doc = "2: Boot from ROM, configure QSPI0, and enter boot loader mode."]
    _10 = 2,
    #[doc = "3: Boot from ROM and enter boot loader mode."]
    _11 = 3,
}
impl From<BOOTSRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTSRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BOOTSRC_SEL` reader - Boot source selection"]
pub struct BOOTSRC_SEL_R(crate::FieldReader<u8, BOOTSRC_SEL_A>);
impl BOOTSRC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOOTSRC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOTSRC_SEL_A> {
        match self.bits {
            0 => Some(BOOTSRC_SEL_A::_00),
            2 => Some(BOOTSRC_SEL_A::_10),
            3 => Some(BOOTSRC_SEL_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == BOOTSRC_SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == BOOTSRC_SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == BOOTSRC_SEL_A::_11
    }
}
impl core::ops::Deref for BOOTSRC_SEL_R {
    type Target = crate::FieldReader<u8, BOOTSRC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn lpboot(&self) -> LPBOOT_R {
        LPBOOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bootpin_opt(&self) -> BOOTPIN_OPT_R {
        BOOTPIN_OPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nmi_dis(&self) -> NMI_DIS_R {
        NMI_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn fast_init(&self) -> FAST_INIT_R {
        FAST_INIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Boot source selection"]
    #[inline(always)]
    pub fn bootsrc_sel(&self) -> BOOTSRC_SEL_R {
        BOOTSRC_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "Non-volatile Flash Option Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fopt](index.html) module"]
pub struct FOPT_SPEC;
impl crate::RegisterSpec for FOPT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fopt::R](R) reader structure"]
impl crate::Readable for FOPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FOPT to value 0x3d"]
impl crate::Resettable for FOPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3d
    }
}
