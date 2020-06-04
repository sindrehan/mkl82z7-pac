#[doc = "Reader of register SHIFTSIEN"]
pub type R = crate::R<u32, super::SHIFTSIEN>;
#[doc = "Writer for register SHIFTSIEN"]
pub type W = crate::W<u32, super::SHIFTSIEN>;
#[doc = "Register SHIFTSIEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTSIEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shifter Status Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSIE_A {
    #[doc = "0: Shifter Status Flag interrupt disabled"]
    _0 = 0,
    #[doc = "1: Shifter Status Flag interrupt enabled"]
    _1 = 1,
}
impl From<SSIE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSIE`"]
pub type SSIE_R = crate::R<u8, SSIE_A>;
impl SSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSIE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSIE_A::_0),
            1 => Val(SSIE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSIE_A::_1
    }
}
#[doc = "Write proxy for field `SSIE`"]
pub struct SSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shifter Status Flag interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIE_A::_0)
    }
    #[doc = "Shifter Status Flag interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&mut self) -> SSIE_W {
        SSIE_W { w: self }
    }
}
