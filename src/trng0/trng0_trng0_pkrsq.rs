#[doc = "Register `TRNG0_PKRSQ` reader"]
pub struct R(crate::R<TRNG0_TRNG0_PKRSQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_PKRSQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_PKRSQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_PKRSQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_SQ` reader - Poker Square Calculation Result"]
pub struct PKR_SQ_R(crate::FieldReader<u32, u32>);
impl PKR_SQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        PKR_SQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_SQ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Poker Square Calculation Result"]
    #[inline(always)]
    pub fn pkr_sq(&self) -> PKR_SQ_R {
        PKR_SQ_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "TRNG0 Poker Square Calculation Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_pkrsq](index.html) module"]
pub struct TRNG0_TRNG0_PKRSQ_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_PKRSQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_pkrsq::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_PKRSQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_PKRSQ to value 0"]
impl crate::Resettable for TRNG0_TRNG0_PKRSQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
