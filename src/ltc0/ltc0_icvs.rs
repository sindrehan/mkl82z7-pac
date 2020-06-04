#[doc = "Reader of register LTC0_ICVS"]
pub type R = crate::R<u32, super::LTC0_ICVS>;
#[doc = "Writer for register LTC0_ICVS"]
pub type W = crate::W<u32, super::LTC0_ICVS>;
#[doc = "Register LTC0_ICVS `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_ICVS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICVS`"]
pub type ICVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICVS`"]
pub struct ICVS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ICV Size, in Bytes."]
    #[inline(always)]
    pub fn icvs(&self) -> ICVS_R {
        ICVS_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ICV Size, in Bytes."]
    #[inline(always)]
    pub fn icvs(&mut self) -> ICVS_W {
        ICVS_W { w: self }
    }
}
