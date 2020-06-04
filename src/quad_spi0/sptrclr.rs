#[doc = "Reader of register SPTRCLR"]
pub type R = crate::R<u32, super::SPTRCLR>;
#[doc = "Writer for register SPTRCLR"]
pub type W = crate::W<u32, super::SPTRCLR>;
#[doc = "Register SPTRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPTRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `BFPTRC`"]
pub struct BFPTRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BFPTRC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `IPPTRC`"]
pub struct IPPTRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IPPTRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bit 0 - Buffer Pointer Clear: 1: Clears the sequence pointer for AHB accesses as defined in QuadSPI_BFGENCR"]
    #[inline(always)]
    pub fn bfptrc(&mut self) -> BFPTRC_W {
        BFPTRC_W { w: self }
    }
    #[doc = "Bit 8 - IP Pointer Clear: 1: Clears the sequence pointer for IP accesses as defined in QuadSPI_IPCR This is a self-clearing field"]
    #[inline(always)]
    pub fn ipptrc(&mut self) -> IPPTRC_W {
        IPPTRC_W { w: self }
    }
}
