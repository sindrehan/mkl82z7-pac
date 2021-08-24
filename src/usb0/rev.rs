#[doc = "Register `REV` reader"]
pub struct R(crate::R<REV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REV` reader - Revision"]
pub struct REV_R(crate::FieldReader<u8, u8>);
impl REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Revision"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral Revision register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rev](index.html) module"]
pub struct REV_SPEC;
impl crate::RegisterSpec for REV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rev::R](R) reader structure"]
impl crate::Readable for REV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REV to value 0x33"]
impl crate::Resettable for REV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x33
    }
}
