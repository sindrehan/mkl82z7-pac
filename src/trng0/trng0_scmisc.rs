#[doc = "Register `TRNG0_SCMISC` reader"]
pub struct R(crate::R<TRNG0_SCMISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_SCMISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_SCMISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_SCMISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_SCMISC` writer"]
pub struct W(crate::W<TRNG0_SCMISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_SCMISC_SPEC>;
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
impl From<crate::W<TRNG0_SCMISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_SCMISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LRUN_MAX` reader - LONG RUN MAX LIMIT"]
pub struct LRUN_MAX_R(crate::FieldReader<u8, u8>);
impl LRUN_MAX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LRUN_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LRUN_MAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRUN_MAX` writer - LONG RUN MAX LIMIT"]
pub struct LRUN_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> LRUN_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RTY_CT` reader - RETRY COUNT"]
pub struct RTY_CT_R(crate::FieldReader<u8, u8>);
impl RTY_CT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTY_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTY_CT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTY_CT` writer - RETRY COUNT"]
pub struct RTY_CT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTY_CT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LONG RUN MAX LIMIT"]
    #[inline(always)]
    pub fn lrun_max(&self) -> LRUN_MAX_R {
        LRUN_MAX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn rty_ct(&self) -> RTY_CT_R {
        RTY_CT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LONG RUN MAX LIMIT"]
    #[inline(always)]
    pub fn lrun_max(&mut self) -> LRUN_MAX_W {
        LRUN_MAX_W { w: self }
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn rty_ct(&mut self) -> RTY_CT_W {
        RTY_CT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Statistical Check Miscellaneous Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_scmisc](index.html) module"]
pub struct TRNG0_SCMISC_SPEC;
impl crate::RegisterSpec for TRNG0_SCMISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_scmisc::R](R) reader structure"]
impl crate::Readable for TRNG0_SCMISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_scmisc::W](W) writer structure"]
impl crate::Writable for TRNG0_SCMISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_SCMISC to value 0x0001_0022"]
impl crate::Resettable for TRNG0_SCMISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0022
    }
}
