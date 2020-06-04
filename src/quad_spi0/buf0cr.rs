#[doc = "Reader of register BUF0CR"]
pub type R = crate::R<u32, super::BUF0CR>;
#[doc = "Writer for register BUF0CR"]
pub type W = crate::W<u32, super::BUF0CR>;
#[doc = "Register BUF0CR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::BUF0CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
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
#[doc = "Reader of field `HP_EN`"]
pub type HP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HP_EN`"]
pub struct HP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_EN_W<'a> {
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
    #[doc = "Bit 31 - High Priority Enable"]
    #[inline(always)]
    pub fn hp_en(&self) -> HP_EN_R {
        HP_EN_R::new(((self.bits >> 31) & 0x01) != 0)
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
    #[doc = "Bit 31 - High Priority Enable"]
    #[inline(always)]
    pub fn hp_en(&mut self) -> HP_EN_W {
        HP_EN_W { w: self }
    }
}
