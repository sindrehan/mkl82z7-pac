#[doc = "Reader of register SDID"]
pub type R = crate::R<u32, super::SDID>;
#[doc = "Pincount identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINID_A {
    #[doc = "5: 64-pin"]
    _0101 = 5,
    #[doc = "6: 80-pin"]
    _0110 = 6,
    #[doc = "8: 100-pin"]
    _1000 = 8,
    #[doc = "9: 121-pin"]
    _1001 = 9,
    #[doc = "11: Custom pinout (WLCSP)"]
    _1011 = 11,
}
impl From<PINID_A> for u8 {
    #[inline(always)]
    fn from(variant: PINID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINID`"]
pub type PINID_R = crate::R<u8, PINID_A>;
impl PINID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINID_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(PINID_A::_0101),
            6 => Val(PINID_A::_0110),
            8 => Val(PINID_A::_1000),
            9 => Val(PINID_A::_1001),
            11 => Val(PINID_A::_1011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PINID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PINID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PINID_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PINID_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PINID_A::_1011
    }
}
#[doc = "Reader of field `FAMID`"]
pub type FAMID_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIEID`"]
pub type DIEID_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVID`"]
pub type REVID_R = crate::R<u8, u8>;
#[doc = "Kinetis Sub-Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBFAMID_A {
    #[doc = "0: KLx0 Subfamily"]
    _0000 = 0,
    #[doc = "1: KLx1 Subfamily"]
    _0001 = 1,
    #[doc = "2: KLx2 Subfamily"]
    _0010 = 2,
    #[doc = "3: KLx3 Subfamily"]
    _0011 = 3,
    #[doc = "4: KLx4 Subfamily"]
    _0100 = 4,
    #[doc = "5: KLx5 Subfamily"]
    _0101 = 5,
    #[doc = "6: KLx6 Subfamily"]
    _0110 = 6,
    #[doc = "7: KLx7 Subfamily"]
    _0111 = 7,
    #[doc = "8: KLx8 Subfamily"]
    _1000 = 8,
    #[doc = "9: KLx9 Subfamily"]
    _1001 = 9,
}
impl From<SUBFAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBFAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBFAMID`"]
pub type SUBFAMID_R = crate::R<u8, SUBFAMID_A>;
impl SUBFAMID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBFAMID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBFAMID_A::_0000),
            1 => Val(SUBFAMID_A::_0001),
            2 => Val(SUBFAMID_A::_0010),
            3 => Val(SUBFAMID_A::_0011),
            4 => Val(SUBFAMID_A::_0100),
            5 => Val(SUBFAMID_A::_0101),
            6 => Val(SUBFAMID_A::_0110),
            7 => Val(SUBFAMID_A::_0111),
            8 => Val(SUBFAMID_A::_1000),
            9 => Val(SUBFAMID_A::_1001),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SUBFAMID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SUBFAMID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SUBFAMID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SUBFAMID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SUBFAMID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SUBFAMID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SUBFAMID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == SUBFAMID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SUBFAMID_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == SUBFAMID_A::_1001
    }
}
#[doc = "Kinetis L family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAMILYID_A {
    #[doc = "0: KL0x Family"]
    _0000 = 0,
    #[doc = "1: KL1x Family"]
    _0001 = 1,
    #[doc = "2: KL2x Family"]
    _0010 = 2,
    #[doc = "3: KL3x Family)"]
    _0011 = 3,
    #[doc = "4: KL4x Family)"]
    _0100 = 4,
    #[doc = "6: KL6x Family"]
    _0110 = 6,
    #[doc = "7: KL7x Family"]
    _0111 = 7,
    #[doc = "9: KL8x Family"]
    _1001 = 9,
}
impl From<FAMILYID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMILYID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FAMILYID`"]
pub type FAMILYID_R = crate::R<u8, FAMILYID_A>;
impl FAMILYID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAMILYID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAMILYID_A::_0000),
            1 => Val(FAMILYID_A::_0001),
            2 => Val(FAMILYID_A::_0010),
            3 => Val(FAMILYID_A::_0011),
            4 => Val(FAMILYID_A::_0100),
            6 => Val(FAMILYID_A::_0110),
            7 => Val(FAMILYID_A::_0111),
            9 => Val(FAMILYID_A::_1001),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == FAMILYID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == FAMILYID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == FAMILYID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == FAMILYID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == FAMILYID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == FAMILYID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == FAMILYID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == FAMILYID_A::_1001
    }
}
impl R {
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PINID_R {
        PINID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Kinetis family ID"]
    #[inline(always)]
    pub fn famid(&self) -> FAMID_R {
        FAMID_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:11 - Device die number"]
    #[inline(always)]
    pub fn dieid(&self) -> DIEID_R {
        DIEID_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Device Revision Number"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Kinetis Sub-Family ID"]
    #[inline(always)]
    pub fn subfamid(&self) -> SUBFAMID_R {
        SUBFAMID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Kinetis L family ID"]
    #[inline(always)]
    pub fn familyid(&self) -> FAMILYID_R {
        FAMILYID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
