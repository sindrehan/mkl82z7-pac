#[doc = "Reader of register LTC0_CW"]
pub type R = crate::R<u32, super::LTC0_CW>;
#[doc = "Writer for register LTC0_CW"]
pub type W = crate::W<u32, super::LTC0_CW>;
#[doc = "Register LTC0_CW `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_CW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
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
#[doc = "Write proxy for field `CDS`"]
pub struct CDS_W<'a> {
    w: &'a mut W,
}
impl<'a> CDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `CICV`"]
pub struct CICV_W<'a> {
    w: &'a mut W,
}
impl<'a> CICV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `CCR`"]
pub struct CCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `CKR`"]
pub struct CKR_W<'a> {
    w: &'a mut W,
}
impl<'a> CKR_W<'a> {
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
#[doc = "Write proxy for field `CPKA`"]
pub struct CPKA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPKA_W<'a> {
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
#[doc = "Write proxy for field `CPKB`"]
pub struct CPKB_W<'a> {
    w: &'a mut W,
}
impl<'a> CPKB_W<'a> {
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
#[doc = "Write proxy for field `CPKN`"]
pub struct CPKN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPKN_W<'a> {
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
#[doc = "Write proxy for field `CPKE`"]
pub struct CPKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPKE_W<'a> {
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
#[doc = "Write proxy for field `COF`"]
pub struct COF_W<'a> {
    w: &'a mut W,
}
impl<'a> COF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `CIF`"]
pub struct CIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CIF_W<'a> {
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
impl R {}
impl W {
    #[doc = "Bit 0 - Clear the Mode Register. Writing a one to this bit causes the Mode Register to be cleared."]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Bit 2 - Clear the Data Size Register"]
    #[inline(always)]
    pub fn cds(&mut self) -> CDS_W {
        CDS_W { w: self }
    }
    #[doc = "Bit 3 - Clear the ICV Size Register. Writing a one to this bit causes the ICV Size Register to be cleared."]
    #[inline(always)]
    pub fn cicv(&mut self) -> CICV_W {
        CICV_W { w: self }
    }
    #[doc = "Bit 5 - Clear the Context Register. Writing a one to this bit causes the Context Register to be cleared."]
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W {
        CCR_W { w: self }
    }
    #[doc = "Bit 6 - Clear the Key Register"]
    #[inline(always)]
    pub fn ckr(&mut self) -> CKR_W {
        CKR_W { w: self }
    }
    #[doc = "Bit 12 - Clear the PKHA A Size Register"]
    #[inline(always)]
    pub fn cpka(&mut self) -> CPKA_W {
        CPKA_W { w: self }
    }
    #[doc = "Bit 13 - Clear the PKHA B Size Register"]
    #[inline(always)]
    pub fn cpkb(&mut self) -> CPKB_W {
        CPKB_W { w: self }
    }
    #[doc = "Bit 14 - Clear the PKHA N Size Register"]
    #[inline(always)]
    pub fn cpkn(&mut self) -> CPKN_W {
        CPKN_W { w: self }
    }
    #[doc = "Bit 15 - Clear the PKHA E Size Register"]
    #[inline(always)]
    pub fn cpke(&mut self) -> CPKE_W {
        CPKE_W { w: self }
    }
    #[doc = "Bit 30 - Clear Output FIFO. Writing a 1 to this bit causes the Output FIFO to be cleared."]
    #[inline(always)]
    pub fn cof(&mut self) -> COF_W {
        COF_W { w: self }
    }
    #[doc = "Bit 31 - Clear Input FIFO. Writing a 1 to this bit causes the Input Data FIFO."]
    #[inline(always)]
    pub fn cif(&mut self) -> CIF_W {
        CIF_W { w: self }
    }
}
