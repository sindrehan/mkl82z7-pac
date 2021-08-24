#[doc = "Register `FACSS` reader"]
pub struct R(crate::R<FACSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FACSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FACSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FACSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SGSIZE` reader - Segment Size"]
pub struct SGSIZE_R(crate::FieldReader<u8, u8>);
impl SGSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SGSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Segment Size"]
    #[inline(always)]
    pub fn sgsize(&self) -> SGSIZE_R {
        SGSIZE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Flash Access Segment Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [facss](index.html) module"]
pub struct FACSS_SPEC;
impl crate::RegisterSpec for FACSS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [facss::R](R) reader structure"]
impl crate::Readable for FACSS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FACSS to value 0"]
impl crate::Resettable for FACSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
