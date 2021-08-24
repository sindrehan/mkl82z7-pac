#[doc = "Register `OBSERVE` reader"]
pub struct R(crate::R<OBSERVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBSERVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBSERVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBSERVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Provides observability of the D- Pulldown enable at the USB transceiver.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMPD_A {
    #[doc = "0: D- pulldown disabled."]
    _0 = 0,
    #[doc = "1: D- pulldown enabled."]
    _1 = 1,
}
impl From<DMPD_A> for bool {
    #[inline(always)]
    fn from(variant: DMPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMPD` reader - Provides observability of the D- Pulldown enable at the USB transceiver."]
pub struct DMPD_R(crate::FieldReader<bool, DMPD_A>);
impl DMPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMPD_A {
        match self.bits {
            false => DMPD_A::_0,
            true => DMPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMPD_A::_1
    }
}
impl core::ops::Deref for DMPD_R {
    type Target = crate::FieldReader<bool, DMPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Provides observability of the D+ Pulldown enable at the USB transceiver.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPD_A {
    #[doc = "0: D+ pulldown disabled."]
    _0 = 0,
    #[doc = "1: D+ pulldown enabled."]
    _1 = 1,
}
impl From<DPPD_A> for bool {
    #[inline(always)]
    fn from(variant: DPPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPD` reader - Provides observability of the D+ Pulldown enable at the USB transceiver."]
pub struct DPPD_R(crate::FieldReader<bool, DPPD_A>);
impl DPPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPD_A {
        match self.bits {
            false => DPPD_A::_0,
            true => DPPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DPPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DPPD_A::_1
    }
}
impl core::ops::Deref for DPPD_R {
    type Target = crate::FieldReader<bool, DPPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Provides observability of the D+ Pullup enable at the USB transceiver.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPU_A {
    #[doc = "0: D+ pullup disabled."]
    _0 = 0,
    #[doc = "1: D+ pullup enabled."]
    _1 = 1,
}
impl From<DPPU_A> for bool {
    #[inline(always)]
    fn from(variant: DPPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPU` reader - Provides observability of the D+ Pullup enable at the USB transceiver."]
pub struct DPPU_R(crate::FieldReader<bool, DPPU_A>);
impl DPPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPU_A {
        match self.bits {
            false => DPPU_A::_0,
            true => DPPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DPPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DPPU_A::_1
    }
}
impl core::ops::Deref for DPPU_R {
    type Target = crate::FieldReader<bool, DPPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Provides observability of the D- Pulldown enable at the USB transceiver."]
    #[inline(always)]
    pub fn dmpd(&self) -> DMPD_R {
        DMPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Provides observability of the D+ Pulldown enable at the USB transceiver."]
    #[inline(always)]
    pub fn dppd(&self) -> DPPD_R {
        DPPD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Provides observability of the D+ Pullup enable at the USB transceiver."]
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "USB OTG Observe register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [observe](index.html) module"]
pub struct OBSERVE_SPEC;
impl crate::RegisterSpec for OBSERVE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [observe::R](R) reader structure"]
impl crate::Readable for OBSERVE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBSERVE to value 0x50"]
impl crate::Resettable for OBSERVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x50
    }
}
