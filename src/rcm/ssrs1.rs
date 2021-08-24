#[doc = "Register `SSRS1` reader"]
pub struct R(crate::R<SSRS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSRS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSRS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSRS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSRS1` writer"]
pub struct W(crate::W<SSRS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSRS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SSRS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSRS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sticky Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOCKUP_A {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    _0 = 0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    _1 = 1,
}
impl From<SLOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: SLOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLOCKUP` reader - Sticky Core Lockup"]
pub struct SLOCKUP_R(crate::FieldReader<bool, SLOCKUP_A>);
impl SLOCKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOCKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOCKUP_A {
        match self.bits {
            false => SLOCKUP_A::_0,
            true => SLOCKUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLOCKUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLOCKUP_A::_1
    }
}
impl core::ops::Deref for SLOCKUP_R {
    type Target = crate::FieldReader<bool, SLOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOCKUP` writer - Sticky Core Lockup"]
pub struct SLOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOCKUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOCKUP_A::_0)
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOCKUP_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Sticky Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSW_A {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    _0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    _1 = 1,
}
impl From<SSW_A> for bool {
    #[inline(always)]
    fn from(variant: SSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSW` reader - Sticky Software"]
pub struct SSW_R(crate::FieldReader<bool, SSW_A>);
impl SSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSW_A {
        match self.bits {
            false => SSW_A::_0,
            true => SSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSW_A::_1
    }
}
impl core::ops::Deref for SSW_R {
    type Target = crate::FieldReader<bool, SSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSW` writer - Sticky Software"]
pub struct SSW_W<'a> {
    w: &'a mut W,
}
impl<'a> SSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSW_A::_0)
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSW_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Sticky MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMDM_AP_A {
    #[doc = "0: Reset not caused by host debugger system setting of the System Reset Request bit"]
    _0 = 0,
    #[doc = "1: Reset caused by host debugger system setting of the System Reset Request bit"]
    _1 = 1,
}
impl From<SMDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: SMDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMDM_AP` reader - Sticky MDM-AP System Reset Request"]
pub struct SMDM_AP_R(crate::FieldReader<bool, SMDM_AP_A>);
impl SMDM_AP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMDM_AP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMDM_AP_A {
        match self.bits {
            false => SMDM_AP_A::_0,
            true => SMDM_AP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMDM_AP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMDM_AP_A::_1
    }
}
impl core::ops::Deref for SMDM_AP_R {
    type Target = crate::FieldReader<bool, SMDM_AP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMDM_AP` writer - Sticky MDM-AP System Reset Request"]
pub struct SMDM_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDM_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMDM_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMDM_AP_A::_0)
    }
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMDM_AP_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Sticky Stop Mode Acknowledge Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACKERR_A {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0 = 0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1 = 1,
}
impl From<SSACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SSACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSACKERR` reader - Sticky Stop Mode Acknowledge Error Reset"]
pub struct SSACKERR_R(crate::FieldReader<bool, SSACKERR_A>);
impl SSACKERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSACKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACKERR_A {
        match self.bits {
            false => SSACKERR_A::_0,
            true => SSACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSACKERR_A::_1
    }
}
impl core::ops::Deref for SSACKERR_R {
    type Target = crate::FieldReader<bool, SSACKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSACKERR` writer - Sticky Stop Mode Acknowledge Error Reset"]
pub struct SSACKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACKERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACKERR_A::_0)
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACKERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&self) -> SLOCKUP_R {
        SLOCKUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&self) -> SSW_R {
        SSW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&self) -> SMDM_AP_R {
        SMDM_AP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sticky Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn ssackerr(&self) -> SSACKERR_R {
        SSACKERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&mut self) -> SLOCKUP_W {
        SLOCKUP_W { w: self }
    }
    #[doc = "Bit 2 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&mut self) -> SSW_W {
        SSW_W { w: self }
    }
    #[doc = "Bit 3 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&mut self) -> SMDM_AP_W {
        SMDM_AP_W { w: self }
    }
    #[doc = "Bit 5 - Sticky Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn ssackerr(&mut self) -> SSACKERR_W {
        SSACKERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sticky System Reset Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrs1](index.html) module"]
pub struct SSRS1_SPEC;
impl crate::RegisterSpec for SSRS1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ssrs1::R](R) reader structure"]
impl crate::Readable for SSRS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssrs1::W](W) writer structure"]
impl crate::Writable for SSRS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSRS1 to value 0"]
impl crate::Resettable for SSRS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
