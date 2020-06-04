#[doc = "Reader of register BFGENCR"]
pub type R = crate::R<u32, super::BFGENCR>;
#[doc = "Writer for register BFGENCR"]
pub type W = crate::W<u32, super::BFGENCR>;
#[doc = "Register BFGENCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BFGENCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
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
impl R {
    #[doc = "Bits 12:15 - Points to a sequence in the Look-up-table"]
    #[inline(always)]
    pub fn seqid(&self) -> SEQID_R {
        SEQID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - When set, a transaction to two serial flash devices is triggered in parallel mode"]
    #[inline(always)]
    pub fn par_en(&self) -> PAR_EN_R {
        PAR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:15 - Points to a sequence in the Look-up-table"]
    #[inline(always)]
    pub fn seqid(&mut self) -> SEQID_W {
        SEQID_W { w: self }
    }
    #[doc = "Bit 16 - When set, a transaction to two serial flash devices is triggered in parallel mode"]
    #[inline(always)]
    pub fn par_en(&mut self) -> PAR_EN_W {
        PAR_EN_W { w: self }
    }
}
