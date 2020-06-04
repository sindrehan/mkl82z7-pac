#[doc = "Reader of register LTC0_ESTA"]
pub type R = crate::R<u32, super::LTC0_ESTA>;
#[doc = "Error ID 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERRID1_A {
    #[doc = "1: Mode Error"]
    _0001 = 1,
    #[doc = "2: Data Size Error, including PKHA N Register Size Error"]
    _0010 = 2,
    #[doc = "3: Key Size Error, including PKHA E Register Size Error"]
    _0011 = 3,
    #[doc = "4: PKHA A Register Size Error"]
    _0100 = 4,
    #[doc = "5: PKHA B Register Size Error"]
    _0101 = 5,
    #[doc = "6: Data Arrived out of Sequence Error"]
    _0110 = 6,
    #[doc = "7: PKHA Divide by Zero Error"]
    _0111 = 7,
    #[doc = "8: PKHA Modulus Even Error"]
    _1000 = 8,
    #[doc = "9: DES Key Parity Error"]
    _1001 = 9,
    #[doc = "10: ICV Check Failed"]
    _1010 = 10,
    #[doc = "11: Internal Hardware Failure"]
    _1011 = 11,
    #[doc = "12: CCM AAD Size Error (either 1. AAD flag in B0 =1 and no AAD type provided, 2. AAD flag in B0 = 0 and AAD povided, or 3. AAD flag in B0 =1 and not enough AAD provided - expecting more based on AAD size.)"]
    _1100 = 12,
    #[doc = "15: Invalid Crypto Engine Selected"]
    _1111 = 15,
}
impl From<ERRID1_A> for u8 {
    #[inline(always)]
    fn from(variant: ERRID1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ERRID1`"]
pub type ERRID1_R = crate::R<u8, ERRID1_A>;
impl ERRID1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ERRID1_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ERRID1_A::_0001),
            2 => Val(ERRID1_A::_0010),
            3 => Val(ERRID1_A::_0011),
            4 => Val(ERRID1_A::_0100),
            5 => Val(ERRID1_A::_0101),
            6 => Val(ERRID1_A::_0110),
            7 => Val(ERRID1_A::_0111),
            8 => Val(ERRID1_A::_1000),
            9 => Val(ERRID1_A::_1001),
            10 => Val(ERRID1_A::_1010),
            11 => Val(ERRID1_A::_1011),
            12 => Val(ERRID1_A::_1100),
            15 => Val(ERRID1_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ERRID1_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == ERRID1_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == ERRID1_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == ERRID1_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == ERRID1_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == ERRID1_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == ERRID1_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ERRID1_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == ERRID1_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == ERRID1_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == ERRID1_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == ERRID1_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == ERRID1_A::_1111
    }
}
#[doc = "algorithms. The algorithms field indicates which algorithm is asserting an error. Others reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL1_A {
    #[doc = "0: LTC General Error"]
    _0000 = 0,
    #[doc = "1: AES"]
    _0001 = 1,
    #[doc = "2: DES"]
    _0010 = 2,
    #[doc = "4: MDHA"]
    _0100 = 4,
    #[doc = "8: Public Key"]
    _1000 = 8,
}
impl From<CL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL1`"]
pub type CL1_R = crate::R<u8, CL1_A>;
impl CL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL1_A::_0000),
            1 => Val(CL1_A::_0001),
            2 => Val(CL1_A::_0010),
            4 => Val(CL1_A::_0100),
            8 => Val(CL1_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CL1_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CL1_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CL1_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CL1_A::_0100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CL1_A::_1000
    }
}
impl R {
    #[doc = "Bits 0:3 - Error ID 1"]
    #[inline(always)]
    pub fn errid1(&self) -> ERRID1_R {
        ERRID1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - algorithms. The algorithms field indicates which algorithm is asserting an error. Others reserved"]
    #[inline(always)]
    pub fn cl1(&self) -> CL1_R {
        CL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
