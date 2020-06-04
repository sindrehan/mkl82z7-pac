#[doc = "Reader of register SHIFTERR"]
pub type R = crate::R<u32, super::SHIFTERR>;
#[doc = "Writer for register SHIFTERR"]
pub type W = crate::W<u32, super::SHIFTERR>;
#[doc = "Register SHIFTERR `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTERR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shifter Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEF_A {
    #[doc = "0: Shifter Error Flag is clear"]
    _0 = 0,
    #[doc = "1: Shifter Error Flag is set"]
    _1 = 1,
}
impl From<SEF_A> for u8 {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEF`"]
pub type SEF_R = crate::R<u8, SEF_A>;
impl SEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEF_A::_0),
            1 => Val(SEF_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEF_A::_1
    }
}
#[doc = "Write proxy for field `SEF`"]
pub struct SEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shifter Error Flag is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEF_A::_0)
    }
    #[doc = "Shifter Error Flag is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W {
        SEF_W { w: self }
    }
}
