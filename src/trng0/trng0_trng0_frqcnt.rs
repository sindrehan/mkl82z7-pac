#[doc = "Register `TRNG0_FRQCNT` reader"]
pub struct R(crate::R<TRNG0_TRNG0_FRQCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_FRQCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_FRQCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_FRQCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRQ_CT` reader - Frequency Count"]
pub struct FRQ_CT_R(crate::FieldReader<u32, u32>);
impl FRQ_CT_R {
    pub(crate) fn new(bits: u32) -> Self {
        FRQ_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRQ_CT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:21 - Frequency Count"]
    #[inline(always)]
    pub fn frq_ct(&self) -> FRQ_CT_R {
        FRQ_CT_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
#[doc = "TRNG0 Frequency Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_frqcnt](index.html) module"]
pub struct TRNG0_TRNG0_FRQCNT_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_FRQCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_frqcnt::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_FRQCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_FRQCNT to value 0"]
impl crate::Resettable for TRNG0_TRNG0_FRQCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
