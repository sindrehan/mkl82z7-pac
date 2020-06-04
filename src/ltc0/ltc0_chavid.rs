#[doc = "Reader of register LTC0_CHAVID"]
pub type R = crate::R<u32, super::LTC0_CHAVID>;
#[doc = "Reader of field `AESREV`"]
pub type AESREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `AESVID`"]
pub type AESVID_R = crate::R<u8, u8>;
#[doc = "Reader of field `DESREV`"]
pub type DESREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `DESVID`"]
pub type DESVID_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKHAREV`"]
pub type PKHAREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKHAVID`"]
pub type PKHAVID_R = crate::R<u8, u8>;
#[doc = "Reader of field `MDHAREV`"]
pub type MDHAREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MDHAVID`"]
pub type MDHAVID_R = crate::R<u8, u8>;
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
