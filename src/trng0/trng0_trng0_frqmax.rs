#[doc = "Register `TRNG0_FRQMAX` reader"]
pub struct R(crate::R<TRNG0_TRNG0_FRQMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_FRQMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_FRQMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_FRQMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_FRQMAX` writer"]
pub struct W(crate::W<TRNG0_TRNG0_FRQMAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_TRNG0_FRQMAX_SPEC>;
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
impl From<crate::W<TRNG0_TRNG0_FRQMAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_TRNG0_FRQMAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRQ_MAX` reader - Frequency Counter Maximum Limit"]
pub struct FRQ_MAX_R(crate::FieldReader<u32, u32>);
impl FRQ_MAX_R {
    pub(crate) fn new(bits: u32) -> Self {
        FRQ_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRQ_MAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRQ_MAX` writer - Frequency Counter Maximum Limit"]
pub struct FRQ_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQ_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Frequency Counter Maximum Limit"]
    #[inline(always)]
    pub fn frq_max(&self) -> FRQ_MAX_R {
        FRQ_MAX_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Frequency Counter Maximum Limit"]
    #[inline(always)]
    pub fn frq_max(&mut self) -> FRQ_MAX_W {
        FRQ_MAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Frequency Count Maximum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_frqmax](index.html) module"]
pub struct TRNG0_TRNG0_FRQMAX_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_FRQMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_frqmax::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_FRQMAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_trng0_frqmax::W](W) writer structure"]
impl crate::Writable for TRNG0_TRNG0_FRQMAX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_FRQMAX to value 0x6400"]
impl crate::Resettable for TRNG0_TRNG0_FRQMAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6400
    }
}
