#[doc = "Register `LTC0_CHAVID` reader"]
pub struct R(crate::R<LTC0_CHAVID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_CHAVID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_CHAVID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_CHAVID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AESREV` reader - AES Revision Number"]
pub struct AESREV_R(crate::FieldReader<u8, u8>);
impl AESREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        AESREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESVID` reader - AES Version ID"]
pub struct AESVID_R(crate::FieldReader<u8, u8>);
impl AESVID_R {
    pub(crate) fn new(bits: u8) -> Self {
        AESVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESVID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESREV` reader - DES Revision Number"]
pub struct DESREV_R(crate::FieldReader<u8, u8>);
impl DESREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        DESREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESVID` reader - DES Version ID(0x0). 0000 - High-performance DESA 0001 - Low-performance DESA"]
pub struct DESVID_R(crate::FieldReader<u8, u8>);
impl DESVID_R {
    pub(crate) fn new(bits: u8) -> Self {
        DESVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESVID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKHAREV` reader - PK Revision Number"]
pub struct PKHAREV_R(crate::FieldReader<u8, u8>);
impl PKHAREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKHAREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKHAREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKHAVID` reader - PK Version ID 0001 - 32-bit PKHA-SD 0010 - 64-bit PKHA-SD 0011 - 128-bit PKHA-SD 0100 - 16-bit PKHA-SD"]
pub struct PKHAVID_R(crate::FieldReader<u8, u8>);
impl PKHAVID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKHAVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKHAVID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDHAREV` reader - MDHA Revision Number"]
pub struct MDHAREV_R(crate::FieldReader<u8, u8>);
impl MDHAREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MDHAREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDHAREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDHAVID` reader - MDHA Hashing Version ID"]
pub struct MDHAVID_R(crate::FieldReader<u8, u8>);
impl MDHAVID_R {
    pub(crate) fn new(bits: u8) -> Self {
        MDHAVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDHAVID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - AES Revision Number"]
    #[inline(always)]
    pub fn aesrev(&self) -> AESREV_R {
        AESREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AES Version ID"]
    #[inline(always)]
    pub fn aesvid(&self) -> AESVID_R {
        AESVID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DES Revision Number"]
    #[inline(always)]
    pub fn desrev(&self) -> DESREV_R {
        DESREV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DES Version ID(0x0). 0000 - High-performance DESA 0001 - Low-performance DESA"]
    #[inline(always)]
    pub fn desvid(&self) -> DESVID_R {
        DESVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PK Revision Number"]
    #[inline(always)]
    pub fn pkharev(&self) -> PKHAREV_R {
        PKHAREV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PK Version ID 0001 - 32-bit PKHA-SD 0010 - 64-bit PKHA-SD 0011 - 128-bit PKHA-SD 0100 - 16-bit PKHA-SD"]
    #[inline(always)]
    pub fn pkhavid(&self) -> PKHAVID_R {
        PKHAVID_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - MDHA Revision Number"]
    #[inline(always)]
    pub fn mdharev(&self) -> MDHAREV_R {
        MDHAREV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - MDHA Hashing Version ID"]
    #[inline(always)]
    pub fn mdhavid(&self) -> MDHAVID_R {
        MDHAVID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "LTC CHA Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_chavid](index.html) module"]
pub struct LTC0_CHAVID_SPEC;
impl crate::RegisterSpec for LTC0_CHAVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_chavid::R](R) reader structure"]
impl crate::Readable for LTC0_CHAVID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTC0_CHAVID to value 0x4044_0251"]
impl crate::Resettable for LTC0_CHAVID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4044_0251
    }
}
