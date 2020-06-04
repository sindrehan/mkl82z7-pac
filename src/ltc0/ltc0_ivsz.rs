#[doc = "Reader of register LTC0_IVSZ"]
pub type R = crate::R<u32, super::LTC0_IVSZ>;
#[doc = "Writer for register LTC0_IVSZ"]
pub type W = crate::W<u32, super::LTC0_IVSZ>;
#[doc = "Register LTC0_IVSZ `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_IVSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IVSZ`"]
pub type IVSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IVSZ`"]
pub struct IVSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IVSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `IL`"]
pub type IL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IL`"]
pub struct IL_W<'a> {
    w: &'a mut W,
}
impl<'a> IL_W<'a> {
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
    #[doc = "Bits 0:3 - IV size in Bytes, mod 16."]
    #[inline(always)]
    pub fn ivsz(&self) -> IVSZ_R {
        IVSZ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - IV Last. Only IV data will be written into the Input FIFO."]
    #[inline(always)]
    pub fn il(&self) -> IL_R {
        IL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - IV size in Bytes, mod 16."]
    #[inline(always)]
    pub fn ivsz(&mut self) -> IVSZ_W {
        IVSZ_W { w: self }
    }
    #[doc = "Bit 31 - IV Last. Only IV data will be written into the Input FIFO."]
    #[inline(always)]
    pub fn il(&mut self) -> IL_W {
        IL_W { w: self }
    }
}
