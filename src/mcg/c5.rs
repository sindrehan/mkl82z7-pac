#[doc = "Register `C5` reader"]
pub struct R(crate::R<C5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C5` writer"]
pub struct W(crate::W<C5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C5_SPEC>;
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
impl From<crate::W<C5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRDIV` reader - PLL External Reference Divider"]
pub struct PRDIV_R(crate::FieldReader<u8, u8>);
impl PRDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRDIV` writer - PLL External Reference Divider"]
pub struct PRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "PLL Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTEN_A {
    #[doc = "0: MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    _1 = 1,
}
impl From<PLLSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTEN` reader - PLL Stop Enable"]
pub struct PLLSTEN_R(crate::FieldReader<bool, PLLSTEN_A>);
impl PLLSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTEN_A {
        match self.bits {
            false => PLLSTEN_A::_0,
            true => PLLSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PLLSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PLLSTEN_A::_1
    }
}
impl core::ops::Deref for PLLSTEN_R {
    type Target = crate::FieldReader<bool, PLLSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSTEN` writer - PLL Stop Enable"]
pub struct PLLSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTEN_A::_0)
    }
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTEN_A::_1)
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
#[doc = "PLL Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCLKEN_A {
    #[doc = "0: MCGPLLCLK is inactive."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK is active."]
    _1 = 1,
}
impl From<PLLCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLCLKEN` reader - PLL Clock Enable"]
pub struct PLLCLKEN_R(crate::FieldReader<bool, PLLCLKEN_A>);
impl PLLCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLCLKEN_A {
        match self.bits {
            false => PLLCLKEN_A::_0,
            true => PLLCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PLLCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PLLCLKEN_A::_1
    }
}
impl core::ops::Deref for PLLCLKEN_R {
    type Target = crate::FieldReader<bool, PLLCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLCLKEN` writer - PLL Clock Enable"]
pub struct PLLCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MCGPLLCLK is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCLKEN_A::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCLKEN_A::_1)
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
impl R {
    #[doc = "Bits 0:2 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv(&self) -> PRDIV_R {
        PRDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten(&self) -> PLLSTEN_R {
        PLLSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken(&self) -> PLLCLKEN_R {
        PLLCLKEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv(&mut self) -> PRDIV_W {
        PRDIV_W { w: self }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten(&mut self) -> PLLSTEN_W {
        PLLSTEN_W { w: self }
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken(&mut self) -> PLLCLKEN_W {
        PLLCLKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5](index.html) module"]
pub struct C5_SPEC;
impl crate::RegisterSpec for C5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c5::R](R) reader structure"]
impl crate::Readable for C5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c5::W](W) writer structure"]
impl crate::Writable for C5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C5 to value 0"]
impl crate::Resettable for C5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
