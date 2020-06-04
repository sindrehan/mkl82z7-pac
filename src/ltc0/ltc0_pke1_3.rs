#[doc = "Reader of register LTC0_PKE1_3"]
pub type R = crate::R<u32, super::LTC0_PKE1_3>;
#[doc = "Writer for register LTC0_PKE1_3"]
pub type W = crate::W<u32, super::LTC0_PKE1_3>;
#[doc = "Register LTC0_PKE1_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_PKE1_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKHA_E1`"]
pub type PKHA_E1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PKHA_E1`"]
pub struct PKHA_E1_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_E1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - E1 VALUE"]
    #[inline(always)]
    pub fn pkha_e1(&self) -> PKHA_E1_R {
        PKHA_E1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - E1 VALUE"]
    #[inline(always)]
    pub fn pkha_e1(&mut self) -> PKHA_E1_W {
        PKHA_E1_W { w: self }
    }
}
