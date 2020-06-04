#[doc = "Reader of register DIVISOR"]
pub type R = crate::R<u32, super::DIVISOR>;
#[doc = "Writer for register DIVISOR"]
pub type W = crate::W<u32, super::DIVISOR>;
#[doc = "Register DIVISOR `reset()`'s with value 0x0174"]
impl crate::ResetValue for super::DIVISOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0174
    }
}
#[doc = "Reader of field `DIVISOR_VALUE`"]
pub type DIVISOR_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVISOR_VALUE`"]
pub struct DIVISOR_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVISOR_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Divisor (F/D) Value"]
    #[inline(always)]
    pub fn divisor_value(&self) -> DIVISOR_VALUE_R {
        DIVISOR_VALUE_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Divisor (F/D) Value"]
    #[inline(always)]
    pub fn divisor_value(&mut self) -> DIVISOR_VALUE_W {
        DIVISOR_VALUE_W { w: self }
    }
}
