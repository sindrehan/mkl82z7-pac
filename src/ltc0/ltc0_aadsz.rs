#[doc = "Reader of register LTC0_AADSZ"]
pub type R = crate::R<u32, super::LTC0_AADSZ>;
#[doc = "Writer for register LTC0_AADSZ"]
pub type W = crate::W<u32, super::LTC0_AADSZ>;
#[doc = "Register LTC0_AADSZ `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_AADSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AADSZ`"]
pub type AADSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AADSZ`"]
pub struct AADSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> AADSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `AL`"]
pub type AL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AL`"]
pub struct AL_W<'a> {
    w: &'a mut W,
}
impl<'a> AL_W<'a> {
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
    #[doc = "Bits 0:3 - AAD size in Bytes, mod 16."]
    #[inline(always)]
    pub fn aadsz(&self) -> AADSZ_R {
        AADSZ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - AAD Last. Only AAD data will be written into the Input FIFO."]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - AAD size in Bytes, mod 16."]
    #[inline(always)]
    pub fn aadsz(&mut self) -> AADSZ_W {
        AADSZ_W { w: self }
    }
    #[doc = "Bit 31 - AAD Last. Only AAD data will be written into the Input FIFO."]
    #[inline(always)]
    pub fn al(&mut self) -> AL_W {
        AL_W { w: self }
    }
}
