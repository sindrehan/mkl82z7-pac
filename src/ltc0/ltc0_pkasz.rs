#[doc = "Reader of register LTC0_PKASZ"]
pub type R = crate::R<u32, super::LTC0_PKASZ>;
#[doc = "Writer for register LTC0_PKASZ"]
pub type W = crate::W<u32, super::LTC0_PKASZ>;
#[doc = "Register LTC0_PKASZ `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_PKASZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKASZ`"]
pub type PKASZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKASZ`"]
pub struct PKASZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PKASZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PKHA A Size. This is the size of the numeric value, in bytes, contained within the PKHA A Register."]
    #[inline(always)]
    pub fn pkasz(&self) -> PKASZ_R {
        PKASZ_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PKHA A Size. This is the size of the numeric value, in bytes, contained within the PKHA A Register."]
    #[inline(always)]
    pub fn pkasz(&mut self) -> PKASZ_W {
        PKASZ_W { w: self }
    }
}
