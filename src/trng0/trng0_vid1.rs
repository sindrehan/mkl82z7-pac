#[doc = "Register `TRNG0_VID1` reader"]
pub struct R(crate::R<TRNG0_VID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_VID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_VID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_VID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Shows the Freescale IP's Minor revision of the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_MIN_REV_A {
    #[doc = "0: Minor revision number for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_MIN_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_MIN_REV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRNG0_MIN_REV` reader - Shows the Freescale IP's Minor revision of the TRNG."]
pub struct TRNG0_MIN_REV_R(crate::FieldReader<u8, TRNG0_MIN_REV_A>);
impl TRNG0_MIN_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRNG0_MIN_REV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRNG0_MIN_REV_A> {
        match self.bits {
            0 => Some(TRNG0_MIN_REV_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        **self == TRNG0_MIN_REV_A::_0X00
    }
}
impl core::ops::Deref for TRNG0_MIN_REV_R {
    type Target = crate::FieldReader<u8, TRNG0_MIN_REV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows the Freescale IP's Major revision of the TRNG.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_MAJ_REV_A {
    #[doc = "1: Major revision number for TRNG."]
    _0X01 = 1,
}
impl From<TRNG0_MAJ_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_MAJ_REV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRNG0_MAJ_REV` reader - Shows the Freescale IP's Major revision of the TRNG."]
pub struct TRNG0_MAJ_REV_R(crate::FieldReader<u8, TRNG0_MAJ_REV_A>);
impl TRNG0_MAJ_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRNG0_MAJ_REV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRNG0_MAJ_REV_A> {
        match self.bits {
            1 => Some(TRNG0_MAJ_REV_A::_0X01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        **self == TRNG0_MAJ_REV_A::_0X01
    }
}
impl core::ops::Deref for TRNG0_MAJ_REV_R {
    type Target = crate::FieldReader<u8, TRNG0_MAJ_REV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNG0_IP_ID` reader - Shows the Freescale IP ID."]
pub struct TRNG0_IP_ID_R(crate::FieldReader<u16, u16>);
impl TRNG0_IP_ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        TRNG0_IP_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNG0_IP_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the Freescale IP's Minor revision of the TRNG."]
    #[inline(always)]
    pub fn trng0_min_rev(&self) -> TRNG0_MIN_REV_R {
        TRNG0_MIN_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the Freescale IP's Major revision of the TRNG."]
    #[inline(always)]
    pub fn trng0_maj_rev(&self) -> TRNG0_MAJ_REV_R {
        TRNG0_MAJ_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Shows the Freescale IP ID."]
    #[inline(always)]
    pub fn trng0_ip_id(&self) -> TRNG0_IP_ID_R {
        TRNG0_IP_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TRNG0 Version ID Register (MS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_vid1](index.html) module"]
pub struct TRNG0_VID1_SPEC;
impl crate::RegisterSpec for TRNG0_VID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_vid1::R](R) reader structure"]
impl crate::Readable for TRNG0_VID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_VID1 to value 0x0030_0100"]
impl crate::Resettable for TRNG0_VID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0030_0100
    }
}
