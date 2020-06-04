#[doc = "Reader of register TRNG0_VID1"]
pub type R = crate::R<u32, super::TRNG0_VID1>;
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
#[doc = "Reader of field `TRNG0_MIN_REV`"]
pub type TRNG0_MIN_REV_R = crate::R<u8, TRNG0_MIN_REV_A>;
impl TRNG0_MIN_REV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRNG0_MIN_REV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRNG0_MIN_REV_A::_0X00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_MIN_REV_A::_0X00
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
#[doc = "Reader of field `TRNG0_MAJ_REV`"]
pub type TRNG0_MAJ_REV_R = crate::R<u8, TRNG0_MAJ_REV_A>;
impl TRNG0_MAJ_REV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRNG0_MAJ_REV_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TRNG0_MAJ_REV_A::_0X01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == TRNG0_MAJ_REV_A::_0X01
    }
}
#[doc = "Reader of field `TRNG0_IP_ID`"]
pub type TRNG0_IP_ID_R = crate::R<u16, u16>;
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
