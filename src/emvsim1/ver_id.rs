#[doc = "Register `VER_ID` reader"]
pub struct R(crate::R<VER_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VER_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VER_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VER_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VER` reader - Version ID of the module"]
pub struct VER_R(crate::FieldReader<u32, u32>);
impl VER_R {
    pub(crate) fn new(bits: u32) -> Self {
        VER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Version ID of the module"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ver_id](index.html) module"]
pub struct VER_ID_SPEC;
impl crate::RegisterSpec for VER_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ver_id::R](R) reader structure"]
impl crate::Readable for VER_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VER_ID to value 0"]
impl crate::Resettable for VER_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
