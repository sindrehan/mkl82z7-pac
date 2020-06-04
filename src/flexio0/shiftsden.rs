#[doc = "Reader of register SHIFTSDEN"]
pub type R = crate::R<u32, super::SHIFTSDEN>;
#[doc = "Writer for register SHIFTSDEN"]
pub type W = crate::W<u32, super::SHIFTSDEN>;
#[doc = "Register SHIFTSDEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTSDEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shifter Status DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSDE_A {
    #[doc = "0: Shifter Status Flag DMA request is disabled"]
    _0 = 0,
    #[doc = "1: Shifter Status Flag DMA request is enabled"]
    _1 = 1,
}
impl From<SSDE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSDE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSDE`"]
pub type SSDE_R = crate::R<u8, SSDE_A>;
impl SSDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSDE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSDE_A::_0),
            1 => Val(SSDE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSDE_A::_1
    }
}
#[doc = "Write proxy for field `SSDE`"]
pub struct SSDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shifter Status Flag DMA request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSDE_A::_0)
    }
    #[doc = "Shifter Status Flag DMA request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSDE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SSDE_R {
        SSDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub fn ssde(&mut self) -> SSDE_W {
        SSDE_W { w: self }
    }
}
