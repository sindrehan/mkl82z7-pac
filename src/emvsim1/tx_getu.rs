#[doc = "Reader of register TX_GETU"]
pub type R = crate::R<u32, super::TX_GETU>;
#[doc = "Writer for register TX_GETU"]
pub type W = crate::W<u32, super::TX_GETU>;
#[doc = "Register TX_GETU `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_GETU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmitter Guard Time Value in ETU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GETU_A {
    #[doc = "0: no additional ETUs inserted (default)"]
    _0 = 0,
    #[doc = "1: 1 additional ETU inserted"]
    _1 = 1,
}
impl From<GETU_A> for u8 {
    #[inline(always)]
    fn from(variant: GETU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GETU`"]
pub type GETU_R = crate::R<u8, GETU_A>;
impl GETU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GETU_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GETU_A::_0),
            1 => Val(GETU_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GETU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GETU_A::_1
    }
}
#[doc = "Write proxy for field `GETU`"]
pub struct GETU_W<'a> {
    w: &'a mut W,
}
impl<'a> GETU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GETU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no additional ETUs inserted (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GETU_A::_0)
    }
    #[doc = "1 additional ETU inserted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GETU_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmitter Guard Time Value in ETU"]
    #[inline(always)]
    pub fn getu(&self) -> GETU_R {
        GETU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmitter Guard Time Value in ETU"]
    #[inline(always)]
    pub fn getu(&mut self) -> GETU_W {
        GETU_W { w: self }
    }
}
