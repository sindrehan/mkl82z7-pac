#[doc = "Register `TRNG0_TOTSAM` reader"]
pub struct R(crate::R<TRNG0_TRNG0_TOTSAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_TOTSAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_TOTSAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_TOTSAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOT_SAM` reader - Total Samples"]
pub struct TOT_SAM_R(crate::FieldReader<u32, u32>);
impl TOT_SAM_R {
    pub(crate) fn new(bits: u32) -> Self {
        TOT_SAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOT_SAM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19 - Total Samples"]
    #[inline(always)]
    pub fn tot_sam(&self) -> TOT_SAM_R {
        TOT_SAM_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "TRNG0 Total Samples Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_totsam](index.html) module"]
pub struct TRNG0_TRNG0_TOTSAM_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_TOTSAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_totsam::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_TOTSAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_TOTSAM to value 0"]
impl crate::Resettable for TRNG0_TRNG0_TOTSAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
