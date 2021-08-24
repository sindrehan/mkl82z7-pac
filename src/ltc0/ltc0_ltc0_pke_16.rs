#[doc = "Register `LTC0_PKE_16` reader"]
pub struct R(crate::R<LTC0_LTC0_PKE_16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_LTC0_PKE_16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_LTC0_PKE_16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_LTC0_PKE_16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_PKE_16` writer"]
pub struct W(crate::W<LTC0_LTC0_PKE_16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_LTC0_PKE_16_SPEC>;
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
impl From<crate::W<LTC0_LTC0_PKE_16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_LTC0_PKE_16_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC PKHA E 16 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ltc0_pke_16](index.html) module"]
pub struct LTC0_LTC0_PKE_16_SPEC;
impl crate::RegisterSpec for LTC0_LTC0_PKE_16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_ltc0_pke_16::R](R) reader structure"]
impl crate::Readable for LTC0_LTC0_PKE_16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_ltc0_pke_16::W](W) writer structure"]
impl crate::Writable for LTC0_LTC0_PKE_16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_PKE_16 to value 0"]
impl crate::Resettable for LTC0_LTC0_PKE_16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
