#[doc = "Reader of register LTC0_PKBSZ"]
pub type R = crate::R<u32, super::LTC0_PKBSZ>;
#[doc = "Writer for register LTC0_PKBSZ"]
pub type W = crate::W<u32, super::LTC0_PKBSZ>;
#[doc = "Register LTC0_PKBSZ `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_PKBSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKBSZ`"]
pub type PKBSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKBSZ`"]
pub struct PKBSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PKBSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PKHA B Size. This is the size of the numeric value, in bytes, contained within the PKHA B Register."]
    #[inline(always)]
    pub fn pkbsz(&self) -> PKBSZ_R {
        PKBSZ_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PKHA B Size. This is the size of the numeric value, in bytes, contained within the PKHA B Register."]
    #[inline(always)]
    pub fn pkbsz(&mut self) -> PKBSZ_W {
        PKBSZ_W { w: self }
    }
}
