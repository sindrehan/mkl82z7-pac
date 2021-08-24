#[doc = "Register `LTC0_ESTA` reader"]
pub struct R(crate::R<LTC0_ESTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_ESTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_ESTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_ESTA_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `ERRID1` reader - Error ID 1"]
pub struct ERRID1_R(crate::FieldReader<u8, ERRID1_A>);
impl ERRID1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ERRID1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERRID1_A> {
        match self.bits {
            1 => Some(ERRID1_A::_0001),
            2 => Some(ERRID1_A::_0010),
            3 => Some(ERRID1_A::_0011),
            4 => Some(ERRID1_A::_0100),
            5 => Some(ERRID1_A::_0101),
            6 => Some(ERRID1_A::_0110),
            7 => Some(ERRID1_A::_0111),
            8 => Some(ERRID1_A::_1000),
            9 => Some(ERRID1_A::_1001),
            10 => Some(ERRID1_A::_1010),
            11 => Some(ERRID1_A::_1011),
            12 => Some(ERRID1_A::_1100),
            15 => Some(ERRID1_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == ERRID1_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == ERRID1_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == ERRID1_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == ERRID1_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == ERRID1_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == ERRID1_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == ERRID1_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == ERRID1_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == ERRID1_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == ERRID1_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == ERRID1_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == ERRID1_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == ERRID1_A::_1111
    }
}
impl core::ops::Deref for ERRID1_R {
    type Target = crate::FieldReader<u8, ERRID1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CL1` reader - algorithms. The algorithms field indicates which algorithm is asserting an error. Others reserved"]
pub struct CL1_R(crate::FieldReader<u8, CL1_A>);
impl CL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CL1_A> {
        match self.bits {
            0 => Some(CL1_A::_0000),
            1 => Some(CL1_A::_0001),
            2 => Some(CL1_A::_0010),
            4 => Some(CL1_A::_0100),
            8 => Some(CL1_A::_1000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == CL1_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == CL1_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == CL1_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == CL1_A::_0100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == CL1_A::_1000
    }
}
impl core::ops::Deref for CL1_R {
    type Target = crate::FieldReader<u8, CL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "LTC Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_esta](index.html) module"]
pub struct LTC0_ESTA_SPEC;
impl crate::RegisterSpec for LTC0_ESTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_esta::R](R) reader structure"]
impl crate::Readable for LTC0_ESTA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTC0_ESTA to value 0"]
impl crate::Resettable for LTC0_ESTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
