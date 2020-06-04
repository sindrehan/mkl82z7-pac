#[doc = "Reader of register TBDR"]
pub type R = crate::R<u32, super::TBDR>;
#[doc = "Writer for register TBDR"]
pub type W = crate::W<u32, super::TBDR>;
#[doc = "Register TBDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDATA`"]
pub type TXDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TXDATA`"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TX Data On write access the data is written into the next available entry of the TX Buffer and the QPSI_TBSR\\[TRBFL\\]
field is updated accordingly"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TX Data On write access the data is written into the next available entry of the TX Buffer and the QPSI_TBSR\\[TRBFL\\]
field is updated accordingly"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
}
