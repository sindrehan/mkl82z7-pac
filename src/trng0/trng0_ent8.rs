#[doc = "Register `TRNG0_ENT8` reader"]
pub struct R(crate::R<TRNG0_ENT8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_ENT8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_ENT8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_ENT8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENT` reader - Entropy Value"]
pub struct ENT_R(crate::FieldReader<u32, u32>);
impl ENT_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Entropy Value"]
    #[inline(always)]
    pub fn ent(&self) -> ENT_R {
        ENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RNG TRNG Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_ent8](index.html) module"]
pub struct TRNG0_ENT8_SPEC;
impl crate::RegisterSpec for TRNG0_ENT8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_ent8::R](R) reader structure"]
impl crate::Readable for TRNG0_ENT8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_ENT8 to value 0"]
impl crate::Resettable for TRNG0_ENT8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
