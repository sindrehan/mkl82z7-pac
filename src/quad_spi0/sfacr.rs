#[doc = "Reader of register SFACR"]
pub type R = crate::R<u32, super::SFACR>;
#[doc = "Writer for register SFACR"]
pub type W = crate::W<u32, super::SFACR>;
#[doc = "Register SFACR `reset()`'s with value 0"]
impl crate::ResetValue for super::SFACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAS`"]
pub type CAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAS`"]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Word Addressable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WA_A {
    #[doc = "0: Byte addressable serial flash mode."]
    _0 = 0,
    #[doc = "1: Word (2 byte) addressable serial flash mode."]
    _1 = 1,
}
impl From<WA_A> for bool {
    #[inline(always)]
    fn from(variant: WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WA`"]
pub type WA_R = crate::R<bool, WA_A>;
impl WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WA_A {
        match self.bits {
            false => WA_A::_0,
            true => WA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WA_A::_1
    }
}
#[doc = "Write proxy for field `WA`"]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Byte addressable serial flash mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WA_A::_0)
    }
    #[doc = "Word (2 byte) addressable serial flash mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Column Address Space"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Word Addressable"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Column Address Space"]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
    #[doc = "Bit 16 - Word Addressable"]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
}
