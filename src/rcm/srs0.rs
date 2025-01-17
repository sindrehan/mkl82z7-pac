#[doc = "Register `SRS0` reader"]
pub struct R(crate::R<SRS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Low Leakage Wakeup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_A {
    #[doc = "0: Reset not caused by LLWU module wakeup source"]
    _0 = 0,
    #[doc = "1: Reset caused by LLWU module wakeup source"]
    _1 = 1,
}
impl From<WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP` reader - Low Leakage Wakeup Reset"]
pub struct WAKEUP_R(crate::FieldReader<bool, WAKEUP_A>);
impl WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_A {
        match self.bits {
            false => WAKEUP_A::_0,
            true => WAKEUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WAKEUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WAKEUP_A::_1
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, WAKEUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVD_A {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    _0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    _1 = 1,
}
impl From<LVD_A> for bool {
    #[inline(always)]
    fn from(variant: LVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD` reader - Low-Voltage Detect Reset"]
pub struct LVD_R(crate::FieldReader<bool, LVD_A>);
impl LVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD_A {
        match self.bits {
            false => LVD_A::_0,
            true => LVD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVD_A::_1
    }
}
impl core::ops::Deref for LVD_R {
    type Target = crate::FieldReader<bool, LVD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Loss-of-Clock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOC_A {
    #[doc = "0: Reset not caused by a loss of external clock."]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of external clock."]
    _1 = 1,
}
impl From<LOC_A> for bool {
    #[inline(always)]
    fn from(variant: LOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOC` reader - Loss-of-Clock Reset"]
pub struct LOC_R(crate::FieldReader<bool, LOC_A>);
impl LOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOC_A {
        match self.bits {
            false => LOC_A::_0,
            true => LOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOC_A::_1
    }
}
impl core::ops::Deref for LOC_R {
    type Target = crate::FieldReader<bool, LOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Loss-of-Lock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOL_A {
    #[doc = "0: Reset not caused by a loss of lock in the PLL"]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of lock in the PLL"]
    _1 = 1,
}
impl From<LOL_A> for bool {
    #[inline(always)]
    fn from(variant: LOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOL` reader - Loss-of-Lock Reset"]
pub struct LOL_R(crate::FieldReader<bool, LOL_A>);
impl LOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOL_A {
        match self.bits {
            false => LOL_A::_0,
            true => LOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOL_A::_1
    }
}
impl core::ops::Deref for LOL_R {
    type Target = crate::FieldReader<bool, LOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_A {
    #[doc = "0: Reset not caused by watchdog timeout"]
    _0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    _1 = 1,
}
impl From<WDOG_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG` reader - Watchdog"]
pub struct WDOG_R(crate::FieldReader<bool, WDOG_A>);
impl WDOG_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_A {
        match self.bits {
            false => WDOG_A::_0,
            true => WDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WDOG_A::_1
    }
}
impl core::ops::Deref for WDOG_R {
    type Target = crate::FieldReader<bool, WDOG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    _0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    _1 = 1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN` reader - External Reset Pin"]
pub struct PIN_R(crate::FieldReader<bool, PIN_A>);
impl PIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            false => PIN_A::_0,
            true => PIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIN_A::_1
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<bool, PIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: Reset not caused by POR"]
    _0 = 0,
    #[doc = "1: Reset caused by POR"]
    _1 = 1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POR` reader - Power-On Reset"]
pub struct POR_R(crate::FieldReader<bool, POR_A>);
impl POR_R {
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::_0,
            true => POR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POR_A::_1
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, POR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn lvd(&self) -> LVD_R {
        LVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn lol(&self) -> LOL_R {
        LOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watchdog"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Reset Pin"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power-On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "System Reset Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs0](index.html) module"]
pub struct SRS0_SPEC;
impl crate::RegisterSpec for SRS0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [srs0::R](R) reader structure"]
impl crate::Readable for SRS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRS0 to value 0x82"]
impl crate::Resettable for SRS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x82
    }
}
