#[doc = "Reader of register BUF2CR"]
pub type R = crate::R<u32, super::BUF2CR>;
#[doc = "Writer for register BUF2CR"]
pub type W = crate::W<u32, super::BUF2CR>;
#[doc = "Register BUF2CR `reset()`'s with value 0x04"]
impl crate::ResetValue for super::BUF2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `MSTRID`"]
pub type MSTRID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSTRID`"]
pub struct MSTRID_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTRID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADATSZ`"]
pub type ADATSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADATSZ`"]
pub struct ADATSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADATSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Master ID"]
    #[inline(always)]
    pub fn mstrid(&self) -> MSTRID_R {
        MSTRID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - AHB data transfer size"]
    #[inline(always)]
    pub fn adatsz(&self) -> ADATSZ_R {
        ADATSZ_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Master ID"]
    #[inline(always)]
    pub fn mstrid(&mut self) -> MSTRID_W {
        MSTRID_W { w: self }
    }
    #[doc = "Bits 8:14 - AHB data transfer size"]
    #[inline(always)]
    pub fn adatsz(&mut self) -> ADATSZ_W {
        ADATSZ_W { w: self }
    }
}
