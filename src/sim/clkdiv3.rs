#[doc = "Register `CLKDIV3` reader"]
pub struct R(crate::R<CLKDIV3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV3` writer"]
pub struct W(crate::W<CLKDIV3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV3_SPEC>;
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
impl From<crate::W<CLKDIV3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLFLLFRAC` reader - PLLFLL clock divider fraction"]
pub struct PLLFLLFRAC_R(crate::FieldReader<bool, bool>);
impl PLLFLLFRAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLFLLFRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLFLLFRAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLFLLFRAC` writer - PLLFLL clock divider fraction"]
pub struct PLLFLLFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLFRAC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PLLFLLDIV` reader - PLLFLL clock divider divisor"]
pub struct PLLFLLDIV_R(crate::FieldReader<u8, u8>);
impl PLLFLLDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLFLLDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLFLLDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLFLLDIV` writer - PLLFLL clock divider divisor"]
pub struct PLLFLLDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLLFLL clock divider fraction"]
    #[inline(always)]
    pub fn pllfllfrac(&self) -> PLLFLLFRAC_R {
        PLLFLLFRAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - PLLFLL clock divider divisor"]
    #[inline(always)]
    pub fn pllflldiv(&self) -> PLLFLLDIV_R {
        PLLFLLDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLLFLL clock divider fraction"]
    #[inline(always)]
    pub fn pllfllfrac(&mut self) -> PLLFLLFRAC_W {
        PLLFLLFRAC_W { w: self }
    }
    #[doc = "Bits 1:3 - PLLFLL clock divider divisor"]
    #[inline(always)]
    pub fn pllflldiv(&mut self) -> PLLFLLDIV_W {
        PLLFLLDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Divider Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv3](index.html) module"]
pub struct CLKDIV3_SPEC;
impl crate::RegisterSpec for CLKDIV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv3::R](R) reader structure"]
impl crate::Readable for CLKDIV3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv3::W](W) writer structure"]
impl crate::Writable for CLKDIV3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV3 to value 0"]
impl crate::Resettable for CLKDIV3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
