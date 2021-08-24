#[doc = "Register `C8` reader"]
pub struct R(crate::R<C8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C8` writer"]
pub struct W(crate::W<C8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C8_SPEC>;
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
impl From<crate::W<C8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC Loss of Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCS1_A {
    #[doc = "0: Loss of RTC has not occur."]
    _0 = 0,
    #[doc = "1: Loss of RTC has occur"]
    _1 = 1,
}
impl From<LOCS1_A> for bool {
    #[inline(always)]
    fn from(variant: LOCS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCS1` reader - RTC Loss of Clock Status"]
pub struct LOCS1_R(crate::FieldReader<bool, LOCS1_A>);
impl LOCS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCS1_A {
        match self.bits {
            false => LOCS1_A::_0,
            true => LOCS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOCS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOCS1_A::_1
    }
}
impl core::ops::Deref for LOCS1_R {
    type Target = crate::FieldReader<bool, LOCS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCS1` writer - RTC Loss of Clock Status"]
pub struct LOCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Loss of RTC has not occur."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCS1_A::_0)
    }
    #[doc = "Loss of RTC has occur"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCS1_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Clock Monitor Enable1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME1_A {
    #[doc = "0: External clock monitor is disabled for RTC clock."]
    _0 = 0,
    #[doc = "1: External clock monitor is enabled for RTC clock."]
    _1 = 1,
}
impl From<CME1_A> for bool {
    #[inline(always)]
    fn from(variant: CME1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CME1` reader - Clock Monitor Enable1"]
pub struct CME1_R(crate::FieldReader<bool, CME1_A>);
impl CME1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CME1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CME1_A {
        match self.bits {
            false => CME1_A::_0,
            true => CME1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CME1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CME1_A::_1
    }
}
impl core::ops::Deref for CME1_R {
    type Target = crate::FieldReader<bool, CME1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CME1` writer - Clock Monitor Enable1"]
pub struct CME1_W<'a> {
    w: &'a mut W,
}
impl<'a> CME1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External clock monitor is disabled for RTC clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME1_A::_0)
    }
    #[doc = "External clock monitor is enabled for RTC clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME1_A::_1)
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
#[doc = "PLL Loss of Lock Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLRE_A {
    #[doc = "0: Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    _0 = 0,
    #[doc = "1: Generate a reset request on a PLL loss of lock indication."]
    _1 = 1,
}
impl From<LOLRE_A> for bool {
    #[inline(always)]
    fn from(variant: LOLRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOLRE` reader - PLL Loss of Lock Reset Enable"]
pub struct LOLRE_R(crate::FieldReader<bool, LOLRE_A>);
impl LOLRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOLRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLRE_A {
        match self.bits {
            false => LOLRE_A::_0,
            true => LOLRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOLRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOLRE_A::_1
    }
}
impl core::ops::Deref for LOLRE_R {
    type Target = crate::FieldReader<bool, LOLRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOLRE` writer - PLL Loss of Lock Reset Enable"]
pub struct LOLRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLRE_A::_0)
    }
    #[doc = "Generate a reset request on a PLL loss of lock indication."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLRE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRE1_A {
    #[doc = "0: Interrupt request is generated on a loss of RTC external reference clock."]
    _0 = 0,
    #[doc = "1: Generate a reset request on a loss of RTC external reference clock"]
    _1 = 1,
}
impl From<LOCRE1_A> for bool {
    #[inline(always)]
    fn from(variant: LOCRE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCRE1` reader - Loss of Clock Reset Enable"]
pub struct LOCRE1_R(crate::FieldReader<bool, LOCRE1_A>);
impl LOCRE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCRE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCRE1_A {
        match self.bits {
            false => LOCRE1_A::_0,
            true => LOCRE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOCRE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOCRE1_A::_1
    }
}
impl core::ops::Deref for LOCRE1_R {
    type Target = crate::FieldReader<bool, LOCRE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCRE1` writer - Loss of Clock Reset Enable"]
pub struct LOCRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCRE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCRE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request is generated on a loss of RTC external reference clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCRE1_A::_0)
    }
    #[doc = "Generate a reset request on a loss of RTC external reference clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCRE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Loss of Clock Status"]
    #[inline(always)]
    pub fn locs1(&self) -> LOCS1_R {
        LOCS1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Monitor Enable1"]
    #[inline(always)]
    pub fn cme1(&self) -> CME1_R {
        CME1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Loss of Lock Reset Enable"]
    #[inline(always)]
    pub fn lolre(&self) -> LOLRE_R {
        LOLRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre1(&self) -> LOCRE1_R {
        LOCRE1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Loss of Clock Status"]
    #[inline(always)]
    pub fn locs1(&mut self) -> LOCS1_W {
        LOCS1_W { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable1"]
    #[inline(always)]
    pub fn cme1(&mut self) -> CME1_W {
        CME1_W { w: self }
    }
    #[doc = "Bit 6 - PLL Loss of Lock Reset Enable"]
    #[inline(always)]
    pub fn lolre(&mut self) -> LOLRE_W {
        LOLRE_W { w: self }
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre1(&mut self) -> LOCRE1_W {
        LOCRE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8](index.html) module"]
pub struct C8_SPEC;
impl crate::RegisterSpec for C8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c8::R](R) reader structure"]
impl crate::Readable for C8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c8::W](W) writer structure"]
impl crate::Writable for C8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C8 to value 0x80"]
impl crate::Resettable for C8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
