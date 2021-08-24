#[doc = "Register `LTC0_VID2` reader"]
pub struct R(crate::R<LTC0_VID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_VID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_VID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_VID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECO_REV` reader - ECO revision number."]
pub struct ECO_REV_R(crate::FieldReader<u8, u8>);
impl ECO_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECO_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECO_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARCH_ERA` reader - Architectural ERA."]
pub struct ARCH_ERA_R(crate::FieldReader<u8, u8>);
impl ARCH_ERA_R {
    pub(crate) fn new(bits: u8) -> Self {
        ARCH_ERA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARCH_ERA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - ECO revision number."]
    #[inline(always)]
    pub fn eco_rev(&self) -> ECO_REV_R {
        ECO_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Architectural ERA."]
    #[inline(always)]
    pub fn arch_era(&self) -> ARCH_ERA_R {
        ARCH_ERA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "LTC Version ID 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_vid2](index.html) module"]
pub struct LTC0_VID2_SPEC;
impl crate::RegisterSpec for LTC0_VID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_vid2::R](R) reader structure"]
impl crate::Readable for LTC0_VID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTC0_VID2 to value 0x0101"]
impl crate::Resettable for LTC0_VID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
