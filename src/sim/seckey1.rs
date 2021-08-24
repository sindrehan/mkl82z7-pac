#[doc = "Register `SECKEY1` reader"]
pub struct R(crate::R<SECKEY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECKEY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECKEY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECKEY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECKEY` reader - Secure Key 31:0"]
pub struct SECKEY_R(crate::FieldReader<u32, u32>);
impl SECKEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        SECKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECKEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Secure Key 31:0"]
    #[inline(always)]
    pub fn seckey(&self) -> SECKEY_R {
        SECKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Secure Key Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seckey1](index.html) module"]
pub struct SECKEY1_SPEC;
impl crate::RegisterSpec for SECKEY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seckey1::R](R) reader structure"]
impl crate::Readable for SECKEY1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECKEY1 to value 0"]
impl crate::Resettable for SECKEY1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
