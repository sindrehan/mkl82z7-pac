#[doc = "Register `BFGENCR` reader"]
pub struct R(crate::R<BFGENCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFGENCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFGENCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFGENCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFGENCR` writer"]
pub struct W(crate::W<BFGENCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFGENCR_SPEC>;
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
impl From<crate::W<BFGENCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFGENCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQID` reader - Points to a sequence in the Look-up-table"]
pub struct SEQID_R(crate::FieldReader<u8, u8>);
impl SEQID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEQID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQID` writer - Points to a sequence in the Look-up-table"]
pub struct SEQID_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PAR_EN` reader - When set, a transaction to two serial flash devices is triggered in parallel mode"]
pub struct PAR_EN_R(crate::FieldReader<bool, bool>);
impl PAR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAR_EN` writer - When set, a transaction to two serial flash devices is triggered in parallel mode"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer Generic Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfgencr](index.html) module"]
pub struct BFGENCR_SPEC;
impl crate::RegisterSpec for BFGENCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bfgencr::R](R) reader structure"]
impl crate::Readable for BFGENCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfgencr::W](W) writer structure"]
impl crate::Writable for BFGENCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BFGENCR to value 0"]
impl crate::Resettable for BFGENCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
