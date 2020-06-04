#[doc = "Reader of register BUF3CR"]
pub type R = crate::R<u32, super::BUF3CR>;
#[doc = "Writer for register BUF3CR"]
pub type W = crate::W<u32, super::BUF3CR>;
#[doc = "Register BUF3CR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::BUF3CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
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
#[doc = "Reader of field `ALLMST`"]
pub type ALLMST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLMST`"]
pub struct ALLMST_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLMST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
    #[doc = "Bit 31 - All master enable"]
    #[inline(always)]
    pub fn allmst(&self) -> ALLMST_R {
        ALLMST_R::new(((self.bits >> 31) & 0x01) != 0)
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
    #[doc = "Bit 31 - All master enable"]
    #[inline(always)]
    pub fn allmst(&mut self) -> ALLMST_W {
        ALLMST_W { w: self }
    }
}
