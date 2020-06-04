#[doc = "Writer for register LTC0_DPAMS"]
pub type W = crate::W<u32, super::LTC0_DPAMS>;
#[doc = "Register LTC0_DPAMS `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_DPAMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DPAMS`"]
pub struct DPAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPAMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Differential Power Analysis Mask Seed"]
    #[inline(always)]
    pub fn dpams(&mut self) -> DPAMS_W {
        DPAMS_W { w: self }
    }
}
