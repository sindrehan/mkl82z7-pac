#[doc = "Reader of register LTC0_PKA0_10"]
pub type R = crate::R<u32, super::LTC0_PKA0_10>;
#[doc = "Writer for register LTC0_PKA0_10"]
pub type W = crate::W<u32, super::LTC0_PKA0_10>;
#[doc = "Register LTC0_PKA0_10 `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_PKA0_10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKHA_A0`"]
pub type PKHA_A0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PKHA_A0`"]
pub struct PKHA_A0_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - A0 VALUE"]
    #[inline(always)]
    pub fn pkha_a0(&self) -> PKHA_A0_R {
        PKHA_A0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - A0 VALUE"]
    #[inline(always)]
    pub fn pkha_a0(&mut self) -> PKHA_A0_W {
        PKHA_A0_W { w: self }
    }
}
