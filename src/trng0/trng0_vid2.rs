#[doc = "Register `TRNG0_VID2` reader"]
pub struct R(crate::R<TRNG0_VID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_VID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_VID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_VID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Shows the Freescale IP's Configuaration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_CONFIG_OPT_A {
    #[doc = "0: TRNG_CONFIG_OPT for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_CONFIG_OPT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_CONFIG_OPT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRNG0_CONFIG_OPT` reader - Shows the Freescale IP's Configuaration options for the TRNG."]
pub struct TRNG0_CONFIG_OPT_R(crate::FieldReader<u8, TRNG0_CONFIG_OPT_A>);
impl TRNG0_CONFIG_OPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRNG0_CONFIG_OPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRNG0_CONFIG_OPT_A> {
        match self.bits {
            0 => Some(TRNG0_CONFIG_OPT_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        **self == TRNG0_CONFIG_OPT_A::_0X00
    }
}
impl core::ops::Deref for TRNG0_CONFIG_OPT_R {
    type Target = crate::FieldReader<u8, TRNG0_CONFIG_OPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows the Freescale IP's ECO revision of the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_ECO_REV_A {
    #[doc = "0: TRNG_ECO_REV for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_ECO_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_ECO_REV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRNG0_ECO_REV` reader - Shows the Freescale IP's ECO revision of the TRNG."]
pub struct TRNG0_ECO_REV_R(crate::FieldReader<u8, TRNG0_ECO_REV_A>);
impl TRNG0_ECO_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRNG0_ECO_REV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRNG0_ECO_REV_A> {
        match self.bits {
            0 => Some(TRNG0_ECO_REV_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        **self == TRNG0_ECO_REV_A::_0X00
    }
}
impl core::ops::Deref for TRNG0_ECO_REV_R {
    type Target = crate::FieldReader<u8, TRNG0_ECO_REV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows the Freescale integration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_INTG_OPT_A {
    #[doc = "0: INTG_OPT for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_INTG_OPT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_INTG_OPT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRNG0_INTG_OPT` reader - Shows the Freescale integration options for the TRNG."]
pub struct TRNG0_INTG_OPT_R(crate::FieldReader<u8, TRNG0_INTG_OPT_A>);
impl TRNG0_INTG_OPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRNG0_INTG_OPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRNG0_INTG_OPT_A> {
        match self.bits {
            0 => Some(TRNG0_INTG_OPT_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        **self == TRNG0_INTG_OPT_A::_0X00
    }
}
impl core::ops::Deref for TRNG0_INTG_OPT_R {
    type Target = crate::FieldReader<u8, TRNG0_INTG_OPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows the Freescale compile options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_ERA_A {
    #[doc = "0: COMPILE_OPT for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_ERA_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_ERA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRNG0_ERA` reader - Shows the Freescale compile options for the TRNG."]
pub struct TRNG0_ERA_R(crate::FieldReader<u8, TRNG0_ERA_A>);
impl TRNG0_ERA_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRNG0_ERA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRNG0_ERA_A> {
        match self.bits {
            0 => Some(TRNG0_ERA_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        **self == TRNG0_ERA_A::_0X00
    }
}
impl core::ops::Deref for TRNG0_ERA_R {
    type Target = crate::FieldReader<u8, TRNG0_ERA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the Freescale IP's Configuaration options for the TRNG."]
    #[inline(always)]
    pub fn trng0_config_opt(&self) -> TRNG0_CONFIG_OPT_R {
        TRNG0_CONFIG_OPT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the Freescale IP's ECO revision of the TRNG."]
    #[inline(always)]
    pub fn trng0_eco_rev(&self) -> TRNG0_ECO_REV_R {
        TRNG0_ECO_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Shows the Freescale integration options for the TRNG."]
    #[inline(always)]
    pub fn trng0_intg_opt(&self) -> TRNG0_INTG_OPT_R {
        TRNG0_INTG_OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Shows the Freescale compile options for the TRNG."]
    #[inline(always)]
    pub fn trng0_era(&self) -> TRNG0_ERA_R {
        TRNG0_ERA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "TRNG0 Version ID Register (LS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_vid2](index.html) module"]
pub struct TRNG0_VID2_SPEC;
impl crate::RegisterSpec for TRNG0_VID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_vid2::R](R) reader structure"]
impl crate::Readable for TRNG0_VID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_VID2 to value 0"]
impl crate::Resettable for TRNG0_VID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
