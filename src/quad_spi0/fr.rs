#[doc = "Reader of register FR"]
pub type R = crate::R<u32, super::FR>;
#[doc = "Writer for register FR"]
pub type W = crate::W<u32, super::FR>;
#[doc = "Register FR `reset()`'s with value 0x0800_0000"]
impl crate::ResetValue for super::FR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800_0000
    }
}
#[doc = "Reader of field `TFF`"]
pub type TFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFF`"]
pub struct TFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFF_W<'a> {
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
#[doc = "Reader of field `IPGEF`"]
pub type IPGEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPGEF`"]
pub struct IPGEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `IPIEF`"]
pub type IPIEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPIEF`"]
pub struct IPIEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IPIEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IPAEF`"]
pub type IPAEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPAEF`"]
pub struct IPAEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IPAEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IUEF`"]
pub type IUEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IUEF`"]
pub struct IUEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IUEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ABOF`"]
pub type ABOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABOF`"]
pub struct ABOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `AIBSEF`"]
pub type AIBSEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIBSEF`"]
pub struct AIBSEF_W<'a> {
    w: &'a mut W,
}
impl<'a> AIBSEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `AITEF`"]
pub type AITEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AITEF`"]
pub struct AITEF_W<'a> {
    w: &'a mut W,
}
impl<'a> AITEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ABSEF`"]
pub type ABSEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABSEF`"]
pub struct ABSEF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABSEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RBDF`"]
pub type RBDF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBDF`"]
pub struct RBDF_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDF_W<'a> {
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
#[doc = "Reader of field `RBOF`"]
pub type RBOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBOF`"]
pub struct RBOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RBOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ILLINE`"]
pub type ILLINE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ILLINE`"]
pub struct ILLINE_W<'a> {
    w: &'a mut W,
}
impl<'a> ILLINE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TBUF`"]
pub type TBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBUF`"]
pub struct TBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TBFF`"]
pub type TBFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBFF`"]
pub struct TBFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DLPFF`"]
pub type DLPFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLPFF`"]
pub struct DLPFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DLPFF_W<'a> {
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
    #[doc = "Bit 0 - IP Command Transaction Finished Flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - IP Command Trigger during AHB Grant Error Flag"]
    #[inline(always)]
    pub fn ipgef(&self) -> IPGEF_R {
        IPGEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IP Command Trigger could not be executed Error Flag"]
    #[inline(always)]
    pub fn ipief(&self) -> IPIEF_R {
        IPIEF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Flag"]
    #[inline(always)]
    pub fn ipaef(&self) -> IPAEF_R {
        IPAEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IP Command Usage Error Flag"]
    #[inline(always)]
    pub fn iuef(&self) -> IUEF_R {
        IUEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Flag"]
    #[inline(always)]
    pub fn abof(&self) -> ABOF_R {
        ABOF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Error Flag"]
    #[inline(always)]
    pub fn aibsef(&self) -> AIBSEF_R {
        AIBSEF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AHB Illegal transaction error flag"]
    #[inline(always)]
    pub fn aitef(&self) -> AITEF_R {
        AITEF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AHB Sequence Error Flag"]
    #[inline(always)]
    pub fn absef(&self) -> ABSEF_R {
        ABSEF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX Buffer Drain Flag"]
    #[inline(always)]
    pub fn rbdf(&self) -> RBDF_R {
        RBDF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX Buffer Overflow Flag"]
    #[inline(always)]
    pub fn rbof(&self) -> RBOF_R {
        RBOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Illegal Instruction Error Flag"]
    #[inline(always)]
    pub fn illine(&self) -> ILLINE_R {
        ILLINE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TX Buffer Underrun Flag"]
    #[inline(always)]
    pub fn tbuf(&self) -> TBUF_R {
        TBUF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TX Buffer Fill Flag"]
    #[inline(always)]
    pub fn tbff(&self) -> TBFF_R {
        TBFF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Data Learning Pattern Failure Flag"]
    #[inline(always)]
    pub fn dlpff(&self) -> DLPFF_R {
        DLPFF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP Command Transaction Finished Flag"]
    #[inline(always)]
    pub fn tff(&mut self) -> TFF_W {
        TFF_W { w: self }
    }
    #[doc = "Bit 4 - IP Command Trigger during AHB Grant Error Flag"]
    #[inline(always)]
    pub fn ipgef(&mut self) -> IPGEF_W {
        IPGEF_W { w: self }
    }
    #[doc = "Bit 6 - IP Command Trigger could not be executed Error Flag"]
    #[inline(always)]
    pub fn ipief(&mut self) -> IPIEF_W {
        IPIEF_W { w: self }
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Flag"]
    #[inline(always)]
    pub fn ipaef(&mut self) -> IPAEF_W {
        IPAEF_W { w: self }
    }
    #[doc = "Bit 11 - IP Command Usage Error Flag"]
    #[inline(always)]
    pub fn iuef(&mut self) -> IUEF_W {
        IUEF_W { w: self }
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Flag"]
    #[inline(always)]
    pub fn abof(&mut self) -> ABOF_W {
        ABOF_W { w: self }
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Error Flag"]
    #[inline(always)]
    pub fn aibsef(&mut self) -> AIBSEF_W {
        AIBSEF_W { w: self }
    }
    #[doc = "Bit 14 - AHB Illegal transaction error flag"]
    #[inline(always)]
    pub fn aitef(&mut self) -> AITEF_W {
        AITEF_W { w: self }
    }
    #[doc = "Bit 15 - AHB Sequence Error Flag"]
    #[inline(always)]
    pub fn absef(&mut self) -> ABSEF_W {
        ABSEF_W { w: self }
    }
    #[doc = "Bit 16 - RX Buffer Drain Flag"]
    #[inline(always)]
    pub fn rbdf(&mut self) -> RBDF_W {
        RBDF_W { w: self }
    }
    #[doc = "Bit 17 - RX Buffer Overflow Flag"]
    #[inline(always)]
    pub fn rbof(&mut self) -> RBOF_W {
        RBOF_W { w: self }
    }
    #[doc = "Bit 23 - Illegal Instruction Error Flag"]
    #[inline(always)]
    pub fn illine(&mut self) -> ILLINE_W {
        ILLINE_W { w: self }
    }
    #[doc = "Bit 26 - TX Buffer Underrun Flag"]
    #[inline(always)]
    pub fn tbuf(&mut self) -> TBUF_W {
        TBUF_W { w: self }
    }
    #[doc = "Bit 27 - TX Buffer Fill Flag"]
    #[inline(always)]
    pub fn tbff(&mut self) -> TBFF_W {
        TBFF_W { w: self }
    }
    #[doc = "Bit 31 - Data Learning Pattern Failure Flag"]
    #[inline(always)]
    pub fn dlpff(&mut self) -> DLPFF_W {
        DLPFF_W { w: self }
    }
}
