#[doc = "Reader of register LTC0_MD"]
pub type R = crate::R<u32, super::LTC0_MD>;
#[doc = "Writer for register LTC0_MD"]
pub type W = crate::W<u32, super::LTC0_MD>;
#[doc = "Register LTC0_MD `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_MD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Encrypt/Decrypt. This bit selects encryption or decryption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENC_A {
    #[doc = "0: Decrypt."]
    _0 = 0,
    #[doc = "1: Encrypt."]
    _1 = 1,
}
impl From<ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENC`"]
pub type ENC_R = crate::R<bool, ENC_A>;
impl ENC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENC_A {
        match self.bits {
            false => ENC_A::_0,
            true => ENC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENC_A::_1
    }
}
#[doc = "Write proxy for field `ENC`"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Decrypt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENC_A::_0)
    }
    #[doc = "Encrypt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENC_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ICV_TEST`"]
pub type ICV_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICV_TEST`"]
pub struct ICV_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICV_TEST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Algorithm State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AS_A {
    #[doc = "0: Update"]
    _00 = 0,
    #[doc = "1: Initialize"]
    _01 = 1,
    #[doc = "2: Finalize"]
    _10 = 2,
    #[doc = "3: Initialize/Finalize"]
    _11 = 3,
}
impl From<AS_A> for u8 {
    #[inline(always)]
    fn from(variant: AS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AS`"]
pub type AS_R = crate::R<u8, AS_A>;
impl AS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AS_A {
        match self.bits {
            0 => AS_A::_00,
            1 => AS_A::_01,
            2 => AS_A::_10,
            3 => AS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AS_A::_11
    }
}
#[doc = "Write proxy for field `AS`"]
pub struct AS_W<'a> {
    w: &'a mut W,
}
impl<'a> AS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Update"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AS_A::_00)
    }
    #[doc = "Initialize"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AS_A::_01)
    }
    #[doc = "Finalize"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AS_A::_10)
    }
    #[doc = "Initialize/Finalize"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `AAI`"]
pub type AAI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AAI`"]
pub struct AAI_W<'a> {
    w: &'a mut W,
}
impl<'a> AAI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u32) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Algorithm. This field specifies which algorithm is being selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALG_A {
    #[doc = "16: AES"]
    _00010000 = 16,
    #[doc = "32: DES"]
    _00100000 = 32,
    #[doc = "33: 3DES"]
    _00100001 = 33,
    #[doc = "65: MDHA - SHA-1"]
    _01000001 = 65,
    #[doc = "66: MDHA - SHA-224"]
    _01000010 = 66,
    #[doc = "67: MDHA - SHA-256"]
    _01000011 = 67,
}
impl From<ALG_A> for u8 {
    #[inline(always)]
    fn from(variant: ALG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALG`"]
pub type ALG_R = crate::R<u8, ALG_A>;
impl ALG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALG_A> {
        use crate::Variant::*;
        match self.bits {
            16 => Val(ALG_A::_00010000),
            32 => Val(ALG_A::_00100000),
            33 => Val(ALG_A::_00100001),
            65 => Val(ALG_A::_01000001),
            66 => Val(ALG_A::_01000010),
            67 => Val(ALG_A::_01000011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00010000`"]
    #[inline(always)]
    pub fn is_00010000(&self) -> bool {
        *self == ALG_A::_00010000
    }
    #[doc = "Checks if the value of the field is `_00100000`"]
    #[inline(always)]
    pub fn is_00100000(&self) -> bool {
        *self == ALG_A::_00100000
    }
    #[doc = "Checks if the value of the field is `_00100001`"]
    #[inline(always)]
    pub fn is_00100001(&self) -> bool {
        *self == ALG_A::_00100001
    }
    #[doc = "Checks if the value of the field is `_01000001`"]
    #[inline(always)]
    pub fn is_01000001(&self) -> bool {
        *self == ALG_A::_01000001
    }
    #[doc = "Checks if the value of the field is `_01000010`"]
    #[inline(always)]
    pub fn is_01000010(&self) -> bool {
        *self == ALG_A::_01000010
    }
    #[doc = "Checks if the value of the field is `_01000011`"]
    #[inline(always)]
    pub fn is_01000011(&self) -> bool {
        *self == ALG_A::_01000011
    }
}
#[doc = "Write proxy for field `ALG`"]
pub struct ALG_W<'a> {
    w: &'a mut W,
}
impl<'a> ALG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AES"]
    #[inline(always)]
    pub fn _00010000(self) -> &'a mut W {
        self.variant(ALG_A::_00010000)
    }
    #[doc = "DES"]
    #[inline(always)]
    pub fn _00100000(self) -> &'a mut W {
        self.variant(ALG_A::_00100000)
    }
    #[doc = "3DES"]
    #[inline(always)]
    pub fn _00100001(self) -> &'a mut W {
        self.variant(ALG_A::_00100001)
    }
    #[doc = "MDHA - SHA-1"]
    #[inline(always)]
    pub fn _01000001(self) -> &'a mut W {
        self.variant(ALG_A::_01000001)
    }
    #[doc = "MDHA - SHA-224"]
    #[inline(always)]
    pub fn _01000010(self) -> &'a mut W {
        self.variant(ALG_A::_01000010)
    }
    #[doc = "MDHA - SHA-256"]
    #[inline(always)]
    pub fn _01000011(self) -> &'a mut W {
        self.variant(ALG_A::_01000011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Encrypt/Decrypt. This bit selects encryption or decryption."]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ICV Checking / Test AES fault detection"]
    #[inline(always)]
    pub fn icv_test(&self) -> ICV_TEST_R {
        ICV_TEST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Algorithm State"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:12 - Additional Algorithm information"]
    #[inline(always)]
    pub fn aai(&self) -> AAI_R {
        AAI_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline(always)]
    pub fn alg(&self) -> ALG_R {
        ALG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Encrypt/Decrypt. This bit selects encryption or decryption."]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bit 1 - ICV Checking / Test AES fault detection"]
    #[inline(always)]
    pub fn icv_test(&mut self) -> ICV_TEST_W {
        ICV_TEST_W { w: self }
    }
    #[doc = "Bits 2:3 - Algorithm State"]
    #[inline(always)]
    pub fn as_(&mut self) -> AS_W {
        AS_W { w: self }
    }
    #[doc = "Bits 4:12 - Additional Algorithm information"]
    #[inline(always)]
    pub fn aai(&mut self) -> AAI_W {
        AAI_W { w: self }
    }
    #[doc = "Bits 16:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline(always)]
    pub fn alg(&mut self) -> ALG_W {
        ALG_W { w: self }
    }
}
