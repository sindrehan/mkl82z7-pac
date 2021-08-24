#[doc = "Register `TRNG0_SBLIM` reader"]
pub struct R(crate::R<TRNG0_TRNG0_SBLIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_SBLIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_SBLIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_SBLIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_SBLIM` writer"]
pub struct W(crate::W<TRNG0_TRNG0_SBLIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_TRNG0_SBLIM_SPEC>;
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
impl From<crate::W<TRNG0_TRNG0_SBLIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_TRNG0_SBLIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB_LIM` reader - Sparse Bit Limit"]
pub struct SB_LIM_R(crate::FieldReader<u16, u16>);
impl SB_LIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        SB_LIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SB_LIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SB_LIM` writer - Sparse Bit Limit"]
pub struct SB_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Sparse Bit Limit"]
    #[inline(always)]
    pub fn sb_lim(&self) -> SB_LIM_R {
        SB_LIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sparse Bit Limit"]
    #[inline(always)]
    pub fn sb_lim(&mut self) -> SB_LIM_W {
        SB_LIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Sparse Bit Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_sblim](index.html) module"]
pub struct TRNG0_TRNG0_SBLIM_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_SBLIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_sblim::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_SBLIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_trng0_sblim::W](W) writer structure"]
impl crate::Writable for TRNG0_TRNG0_SBLIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_SBLIM to value 0x3f"]
impl crate::Resettable for TRNG0_TRNG0_SBLIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
