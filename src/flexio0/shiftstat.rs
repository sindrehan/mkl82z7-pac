#[doc = "Reader of register SHIFTSTAT"]
pub type R = crate::R<u32, super::SHIFTSTAT>;
#[doc = "Writer for register SHIFTSTAT"]
pub type W = crate::W<u32, super::SHIFTSTAT>;
#[doc = "Register SHIFTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shifter Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSF_A {
    #[doc = "0: Status flag is clear"]
    _0 = 0,
    #[doc = "1: Status flag is set"]
    _1 = 1,
}
impl From<SSF_A> for u8 {
    #[inline(always)]
    fn from(variant: SSF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSF`"]
pub type SSF_R = crate::R<u8, SSF_A>;
impl SSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSF_A::_0),
            1 => Val(SSF_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSF_A::_1
    }
}
#[doc = "Write proxy for field `SSF`"]
pub struct SSF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Status flag is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSF_A::_0)
    }
    #[doc = "Status flag is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&mut self) -> SSF_W {
        SSF_W { w: self }
    }
}
