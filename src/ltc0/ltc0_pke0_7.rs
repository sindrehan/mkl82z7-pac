#[doc = "Reader of register LTC0_PKE0_7"]
pub type R = crate::R<u32, super::LTC0_PKE0_7>;
#[doc = "Writer for register LTC0_PKE0_7"]
pub type W = crate::W<u32, super::LTC0_PKE0_7>;
#[doc = "Register LTC0_PKE0_7 `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_PKE0_7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKHA_E0`"]
pub type PKHA_E0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PKHA_E0`"]
pub struct PKHA_E0_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_E0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - E0 VALUE"]
    #[inline(always)]
    pub fn pkha_e0(&self) -> PKHA_E0_R {
        PKHA_E0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - E0 VALUE"]
    #[inline(always)]
    pub fn pkha_e0(&mut self) -> PKHA_E0_W {
        PKHA_E0_W { w: self }
    }
}
