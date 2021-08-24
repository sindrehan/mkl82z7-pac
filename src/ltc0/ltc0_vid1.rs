#[doc = "Register `LTC0_VID1` reader"]
pub struct R(crate::R<LTC0_VID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_VID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_VID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_VID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIN_REV` reader - Minor revision number."]
pub struct MIN_REV_R(crate::FieldReader<u8, u8>);
impl MIN_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIN_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJ_REV` reader - Major revision number."]
pub struct MAJ_REV_R(crate::FieldReader<u8, u8>);
impl MAJ_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAJ_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJ_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_ID` reader - ID(0x0038)."]
pub struct IP_ID_R(crate::FieldReader<u16, u16>);
impl IP_ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        IP_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Minor revision number."]
    #[inline(always)]
    pub fn min_rev(&self) -> MIN_REV_R {
        MIN_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Major revision number."]
    #[inline(always)]
    pub fn maj_rev(&self) -> MAJ_REV_R {
        MAJ_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - ID(0x0038)."]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "LTC Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_vid1](index.html) module"]
pub struct LTC0_VID1_SPEC;
impl crate::RegisterSpec for LTC0_VID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_vid1::R](R) reader structure"]
impl crate::Readable for LTC0_VID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTC0_VID1 to value 0x0034_0100"]
impl crate::Resettable for LTC0_VID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0034_0100
    }
}
