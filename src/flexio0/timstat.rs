#[doc = "Reader of register TIMSTAT"]
pub type R = crate::R<u32, super::TIMSTAT>;
#[doc = "Writer for register TIMSTAT"]
pub type W = crate::W<u32, super::TIMSTAT>;
#[doc = "Register TIMSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer Status Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSF_A {
    #[doc = "0: Timer Status Flag is clear"]
    _0 = 0,
    #[doc = "1: Timer Status Flag is set"]
    _1 = 1,
}
impl From<TSF_A> for u8 {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSF`"]
pub type TSF_R = crate::R<u8, TSF_A>;
impl TSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSF_A::_0),
            1 => Val(TSF_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSF_A::_1
    }
}
#[doc = "Write proxy for field `TSF`"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer Status Flag is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSF_A::_0)
    }
    #[doc = "Timer Status Flag is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer Status Flags"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Status Flags"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
}
