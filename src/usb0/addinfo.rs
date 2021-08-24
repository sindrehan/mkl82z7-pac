#[doc = "Register `ADDINFO` reader"]
pub struct R(crate::R<ADDINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IEHOST` reader - This bit is set if host mode is enabled."]
pub struct IEHOST_R(crate::FieldReader<bool, bool>);
impl IEHOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEHOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEHOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit is set if host mode is enabled."]
    #[inline(always)]
    pub fn iehost(&self) -> IEHOST_R {
        IEHOST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Peripheral Additional Info register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addinfo](index.html) module"]
pub struct ADDINFO_SPEC;
impl crate::RegisterSpec for ADDINFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [addinfo::R](R) reader structure"]
impl crate::Readable for ADDINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDINFO to value 0x01"]
impl crate::Resettable for ADDINFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
