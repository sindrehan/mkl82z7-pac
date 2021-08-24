#[doc = "Register `TRNG0_PKRRNG` reader"]
pub struct R(crate::R<TRNG0_PKRRNG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_PKRRNG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_PKRRNG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_PKRRNG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_PKRRNG` writer"]
pub struct W(crate::W<TRNG0_PKRRNG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_PKRRNG_SPEC>;
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
impl From<crate::W<TRNG0_PKRRNG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_PKRRNG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKR_RNG` reader - Poker Range"]
pub struct PKR_RNG_R(crate::FieldReader<u16, u16>);
impl PKR_RNG_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKR_RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_RNG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKR_RNG` writer - Poker Range"]
pub struct PKR_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PKR_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Poker Range"]
    #[inline(always)]
    pub fn pkr_rng(&self) -> PKR_RNG_R {
        PKR_RNG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Poker Range"]
    #[inline(always)]
    pub fn pkr_rng(&mut self) -> PKR_RNG_W {
        PKR_RNG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Poker Range Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrrng](index.html) module"]
pub struct TRNG0_PKRRNG_SPEC;
impl crate::RegisterSpec for TRNG0_PKRRNG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_pkrrng::R](R) reader structure"]
impl crate::Readable for TRNG0_PKRRNG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_pkrrng::W](W) writer structure"]
impl crate::Writable for TRNG0_PKRRNG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_PKRRNG to value 0x09a3"]
impl crate::Resettable for TRNG0_PKRRNG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x09a3
    }
}
