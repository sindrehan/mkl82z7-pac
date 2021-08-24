#[doc = "Register `CLM0` reader"]
pub struct R(crate::R<CLM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLM0` writer"]
pub struct W(crate::W<CLM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLM0_SPEC>;
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
impl From<crate::W<CLM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLM0` reader - Calibration Value"]
pub struct CLM0_R(crate::FieldReader<u8, u8>);
impl CLM0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLM0` writer - Calibration Value"]
pub struct CLM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clm0(&self) -> CLM0_R {
        CLM0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clm0(&mut self) -> CLM0_W {
        CLM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm0](index.html) module"]
pub struct CLM0_SPEC;
impl crate::RegisterSpec for CLM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clm0::R](R) reader structure"]
impl crate::Readable for CLM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clm0::W](W) writer structure"]
impl crate::Writable for CLM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLM0 to value 0x20"]
impl crate::Resettable for CLM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
