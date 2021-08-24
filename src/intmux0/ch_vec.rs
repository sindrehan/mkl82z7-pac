#[doc = "Register `CH%s_VEC` reader"]
pub struct R(crate::R<CH_VEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_VEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_VEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_VEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VECN` reader - Vector Number"]
pub struct VECN_R(crate::FieldReader<u16, u16>);
impl VECN_R {
    pub(crate) fn new(bits: u16) -> Self {
        VECN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 2:13 - Vector Number"]
    #[inline(always)]
    pub fn vecn(&self) -> VECN_R {
        VECN_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
}
#[doc = "Channel n Vector Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_vec](index.html) module"]
pub struct CH_VEC_SPEC;
impl crate::RegisterSpec for CH_VEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_vec::R](R) reader structure"]
impl crate::Readable for CH_VEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%s_VEC to value 0"]
impl crate::Resettable for CH_VEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
