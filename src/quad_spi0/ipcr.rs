#[doc = "Reader of register IPCR"]
pub type R = crate::R<u32, super::IPCR>;
#[doc = "Writer for register IPCR"]
pub type W = crate::W<u32, super::IPCR>;
#[doc = "Register IPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDATSZ`"]
pub type IDATSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IDATSZ`"]
pub struct IDATSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IDATSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PAR_EN`"]
pub type PAR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAR_EN`"]
pub struct PAR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SEQID`"]
pub type SEQID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQID`"]
pub struct SEQID_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IP data transfer size. Defines the data transfer size in bytes of the IP command."]
    #[inline(always)]
    pub fn idatsz(&self) -> IDATSZ_R {
        IDATSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - When set, a transaction to two serial flash devices is triggered in parallel mode"]
    #[inline(always)]
    pub fn par_en(&self) -> PAR_EN_R {
        PAR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Points to a sequence in the Look-up table"]
    #[inline(always)]
    pub fn seqid(&self) -> SEQID_R {
        SEQID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - IP data transfer size. Defines the data transfer size in bytes of the IP command."]
    #[inline(always)]
    pub fn idatsz(&mut self) -> IDATSZ_W {
        IDATSZ_W { w: self }
    }
    #[doc = "Bit 16 - When set, a transaction to two serial flash devices is triggered in parallel mode"]
    #[inline(always)]
    pub fn par_en(&mut self) -> PAR_EN_W {
        PAR_EN_W { w: self }
    }
    #[doc = "Bits 24:27 - Points to a sequence in the Look-up table"]
    #[inline(always)]
    pub fn seqid(&mut self) -> SEQID_W {
        SEQID_W { w: self }
    }
}
