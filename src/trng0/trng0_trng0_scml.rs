#[doc = "Register `TRNG0_SCML` reader"]
pub struct R(crate::R<TRNG0_TRNG0_SCML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_SCML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_SCML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_SCML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_SCML` writer"]
pub struct W(crate::W<TRNG0_TRNG0_SCML_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_TRNG0_SCML_SPEC>;
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
impl From<crate::W<TRNG0_TRNG0_SCML_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_TRNG0_SCML_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MONO_MAX` reader - Monobit Maximum Limit"]
pub struct MONO_MAX_R(crate::FieldReader<u16, u16>);
impl MONO_MAX_R {
    pub(crate) fn new(bits: u16) -> Self {
        MONO_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONO_MAX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONO_MAX` writer - Monobit Maximum Limit"]
pub struct MONO_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MONO_RNG` reader - Monobit Range"]
pub struct MONO_RNG_R(crate::FieldReader<u16, u16>);
impl MONO_RNG_R {
    pub(crate) fn new(bits: u16) -> Self {
        MONO_RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONO_RNG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONO_RNG` writer - Monobit Range"]
pub struct MONO_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Monobit Maximum Limit"]
    #[inline(always)]
    pub fn mono_max(&self) -> MONO_MAX_R {
        MONO_MAX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Monobit Range"]
    #[inline(always)]
    pub fn mono_rng(&self) -> MONO_RNG_R {
        MONO_RNG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Monobit Maximum Limit"]
    #[inline(always)]
    pub fn mono_max(&mut self) -> MONO_MAX_W {
        MONO_MAX_W { w: self }
    }
    #[doc = "Bits 16:31 - Monobit Range"]
    #[inline(always)]
    pub fn mono_rng(&mut self) -> MONO_RNG_W {
        MONO_RNG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Statistical Check Monobit Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_scml](index.html) module"]
pub struct TRNG0_TRNG0_SCML_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_SCML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_scml::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_SCML_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_trng0_scml::W](W) writer structure"]
impl crate::Writable for TRNG0_TRNG0_SCML_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_SCML to value 0x010c_0568"]
impl crate::Resettable for TRNG0_TRNG0_SCML_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x010c_0568
    }
}
