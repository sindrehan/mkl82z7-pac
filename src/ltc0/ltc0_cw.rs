#[doc = "Register `LTC0_CW` reader"]
pub struct R(crate::R<LTC0_CW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_CW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_CW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_CW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_CW` writer"]
pub struct W(crate::W<LTC0_CW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_CW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LTC0_CW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_CW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM` writer - Clear the Mode Register. Writing a one to this bit causes the Mode Register to be cleared."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CDS` writer - Clear the Data Size Register"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CICV` writer - Clear the ICV Size Register. Writing a one to this bit causes the ICV Size Register to be cleared."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CCR` writer - Clear the Context Register. Writing a one to this bit causes the Context Register to be cleared."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CKR` writer - Clear the Key Register"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CPKA` writer - Clear the PKHA A Size Register"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CPKB` writer - Clear the PKHA B Size Register"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CPKN` writer - Clear the PKHA N Size Register"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CPKE` writer - Clear the PKHA E Size Register"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `COF` writer - Clear Output FIFO. Writing a 1 to this bit causes the Output FIFO to be cleared."]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CIF` writer - Clear Input FIFO. Writing a 1 to this bit causes the Input Data FIFO."]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC Clear Written Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_cw](index.html) module"]
pub struct LTC0_CW_SPEC;
impl crate::RegisterSpec for LTC0_CW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_cw::R](R) reader structure"]
impl crate::Readable for LTC0_CW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_cw::W](W) writer structure"]
impl crate::Writable for LTC0_CW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_CW to value 0"]
impl crate::Resettable for LTC0_CW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
