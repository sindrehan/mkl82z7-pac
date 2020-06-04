#[doc = "Reader of register SHIFTEIEN"]
pub type R = crate::R<u32, super::SHIFTEIEN>;
#[doc = "Writer for register SHIFTEIEN"]
pub type W = crate::W<u32, super::SHIFTEIEN>;
#[doc = "Register SHIFTEIEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTEIEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shifter Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEIE_A {
    #[doc = "0: Shifter Error Flag interrupt disabled"]
    _0 = 0,
    #[doc = "1: Shifter Error Flag interrupt enabled"]
    _1 = 1,
}
impl From<SEIE_A> for u8 {
    #[inline(always)]
    fn from(variant: SEIE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEIE`"]
pub type SEIE_R = crate::R<u8, SEIE_A>;
impl SEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEIE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEIE_A::_0),
            1 => Val(SEIE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEIE_A::_1
    }
}
#[doc = "Write proxy for field `SEIE`"]
pub struct SEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shifter Error Flag interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEIE_A::_0)
    }
    #[doc = "Shifter Error Flag interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEIE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W {
        SEIE_W { w: self }
    }
}
