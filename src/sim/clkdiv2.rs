#[doc = "Register `CLKDIV2` reader"]
pub struct R(crate::R<CLKDIV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV2` writer"]
pub struct W(crate::W<CLKDIV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV2_SPEC>;
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
impl From<crate::W<CLKDIV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBFRAC` reader - USB clock divider fraction"]
pub struct USBFRAC_R(crate::FieldReader<bool, bool>);
impl USBFRAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBFRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBFRAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFRAC` writer - USB clock divider fraction"]
pub struct USBFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFRAC_W<'a> {
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
#[doc = "Field `USBDIV` reader - USB clock divider divisor"]
pub struct USBDIV_R(crate::FieldReader<u8, u8>);
impl USBDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDIV` writer - USB clock divider divisor"]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB clock divider fraction"]
    #[inline(always)]
    pub fn usbfrac(&self) -> USBFRAC_R {
        USBFRAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - USB clock divider divisor"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB clock divider fraction"]
    #[inline(always)]
    pub fn usbfrac(&mut self) -> USBFRAC_W {
        USBFRAC_W { w: self }
    }
    #[doc = "Bits 1:3 - USB clock divider divisor"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Divider Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv2](index.html) module"]
pub struct CLKDIV2_SPEC;
impl crate::RegisterSpec for CLKDIV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv2::R](R) reader structure"]
impl crate::Readable for CLKDIV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv2::W](W) writer structure"]
impl crate::Writable for CLKDIV2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKDIV2 to value 0"]
impl crate::Resettable for CLKDIV2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
