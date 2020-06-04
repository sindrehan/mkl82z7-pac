#[doc = "Reader of register DLPR"]
pub type R = crate::R<u32, super::DLPR>;
#[doc = "Writer for register DLPR"]
pub type W = crate::W<u32, super::DLPR>;
#[doc = "Register DLPR `reset()`'s with value 0xaa55_3443"]
impl crate::ResetValue for super::DLPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xaa55_3443
    }
}
#[doc = "Reader of field `DLPV`"]
pub type DLPV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DLPV`"]
pub struct DLPV_W<'a> {
    w: &'a mut W,
}
impl<'a> DLPV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Learning Pattern Value: This value is used for data learning in DDR and DQS mode"]
    #[inline(always)]
    pub fn dlpv(&self) -> DLPV_R {
        DLPV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Learning Pattern Value: This value is used for data learning in DDR and DQS mode"]
    #[inline(always)]
    pub fn dlpv(&mut self) -> DLPV_W {
        DLPV_W { w: self }
    }
}
