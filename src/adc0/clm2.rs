#[doc = "Register `CLM2` reader"]
pub struct R(crate::R<CLM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLM2` writer"]
pub struct W(crate::W<CLM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLM2_SPEC>;
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
impl From<crate::W<CLM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLM2` reader - Calibration Value"]
pub struct CLM2_R(crate::FieldReader<u8, u8>);
impl CLM2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLM2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLM2` writer - Calibration Value"]
pub struct CLM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn clm2(&self) -> CLM2_R {
        CLM2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn clm2(&mut self) -> CLM2_W {
        CLM2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm2](index.html) module"]
pub struct CLM2_SPEC;
impl crate::RegisterSpec for CLM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clm2::R](R) reader structure"]
impl crate::Readable for CLM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clm2::W](W) writer structure"]
impl crate::Writable for CLM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLM2 to value 0x80"]
impl crate::Resettable for CLM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
