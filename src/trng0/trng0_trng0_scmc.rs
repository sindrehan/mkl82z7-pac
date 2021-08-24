#[doc = "Register `TRNG0_SCMC` reader"]
pub struct R(crate::R<TRNG0_TRNG0_SCMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_SCMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_SCMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_SCMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MONO_CT` reader - Monobit Count"]
pub struct MONO_CT_R(crate::FieldReader<u16, u16>);
impl MONO_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        MONO_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONO_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Monobit Count"]
    #[inline(always)]
    pub fn mono_ct(&self) -> MONO_CT_R {
        MONO_CT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TRNG0 Statistical Check Monobit Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_scmc](index.html) module"]
pub struct TRNG0_TRNG0_SCMC_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_SCMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_scmc::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_SCMC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_SCMC to value 0"]
impl crate::Resettable for TRNG0_TRNG0_SCMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
