#[doc = "Register `C6` reader"]
pub struct R(crate::R<C6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C6` writer"]
pub struct W(crate::W<C6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C6_SPEC>;
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
impl From<crate::W<C6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDIV` reader - VCO Divider"]
pub struct VDIV_R(crate::FieldReader<u8, u8>);
impl VDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDIV` writer - VCO Divider"]
pub struct VDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> VDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u8 & 0x1f);
        self.w
    }
}
#[doc = "Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME0_A {
    #[doc = "0: External clock monitor is disabled for OSC0."]
    _0 = 0,
    #[doc = "1: External clock monitor is enabled for OSC0."]
    _1 = 1,
}
impl From<CME0_A> for bool {
    #[inline(always)]
    fn from(variant: CME0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CME0` reader - Clock Monitor Enable"]
pub struct CME0_R(crate::FieldReader<bool, CME0_A>);
impl CME0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CME0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CME0_A {
        match self.bits {
            false => CME0_A::_0,
            true => CME0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CME0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CME0_A::_1
    }
}
impl core::ops::Deref for CME0_R {
    type Target = crate::FieldReader<bool, CME0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CME0` writer - Clock Monitor Enable"]
pub struct CME0_W<'a> {
    w: &'a mut W,
}
impl<'a> CME0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External clock monitor is disabled for OSC0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME0_A::_0)
    }
    #[doc = "External clock monitor is enabled for OSC0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME0_A::_1)
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
#[doc = "PLL Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLS_A {
    #[doc = "0: FLL is selected."]
    _0 = 0,
    #[doc = "1: PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 8-16 MHz prior to setting the PLLS bit)."]
    _1 = 1,
}
impl From<PLLS_A> for bool {
    #[inline(always)]
    fn from(variant: PLLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLS` reader - PLL Select"]
pub struct PLLS_R(crate::FieldReader<bool, PLLS_A>);
impl PLLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLS_A {
        match self.bits {
            false => PLLS_A::_0,
            true => PLLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PLLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PLLS_A::_1
    }
}
impl core::ops::Deref for PLLS_R {
    type Target = crate::FieldReader<bool, PLLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLS` writer - PLL Select"]
pub struct PLLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FLL is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLS_A::_0)
    }
    #[doc = "PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 8-16 MHz prior to setting the PLLS bit)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLS_A::_1)
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
#[doc = "Loss of Lock Interrrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLIE0_A {
    #[doc = "0: No interrupt request is generated on loss of lock."]
    _0 = 0,
    #[doc = "1: Generate an interrupt request on loss of lock."]
    _1 = 1,
}
impl From<LOLIE0_A> for bool {
    #[inline(always)]
    fn from(variant: LOLIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOLIE0` reader - Loss of Lock Interrrupt Enable"]
pub struct LOLIE0_R(crate::FieldReader<bool, LOLIE0_A>);
impl LOLIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOLIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLIE0_A {
        match self.bits {
            false => LOLIE0_A::_0,
            true => LOLIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOLIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOLIE0_A::_1
    }
}
impl core::ops::Deref for LOLIE0_R {
    type Target = crate::FieldReader<bool, LOLIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOLIE0` writer - Loss of Lock Interrrupt Enable"]
pub struct LOLIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt request is generated on loss of lock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLIE0_A::_0)
    }
    #[doc = "Generate an interrupt request on loss of lock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLIE0_A::_1)
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
    #[doc = "Bits 0:4 - VCO Divider"]
    #[inline(always)]
    pub fn vdiv(&self) -> VDIV_R {
        VDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme0(&self) -> CME0_R {
        CME0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline(always)]
    pub fn plls(&self) -> PLLS_R {
        PLLS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline(always)]
    pub fn lolie0(&self) -> LOLIE0_R {
        LOLIE0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - VCO Divider"]
    #[inline(always)]
    pub fn vdiv(&mut self) -> VDIV_W {
        VDIV_W { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme0(&mut self) -> CME0_W {
        CME0_W { w: self }
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline(always)]
    pub fn plls(&mut self) -> PLLS_W {
        PLLS_W { w: self }
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline(always)]
    pub fn lolie0(&mut self) -> LOLIE0_W {
        LOLIE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6](index.html) module"]
pub struct C6_SPEC;
impl crate::RegisterSpec for C6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c6::R](R) reader structure"]
impl crate::Readable for C6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c6::W](W) writer structure"]
impl crate::Writable for C6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C6 to value 0"]
impl crate::Resettable for C6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
