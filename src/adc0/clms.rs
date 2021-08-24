#[doc = "Register `CLMS` reader"]
pub struct R(crate::R<CLMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLMS` writer"]
pub struct W(crate::W<CLMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLMS_SPEC>;
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
impl From<crate::W<CLMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLMS` reader - Calibration Value"]
pub struct CLMS_R(crate::FieldReader<u8, u8>);
impl CLMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLMS` writer - Calibration Value"]
pub struct CLMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clms(&self) -> CLMS_R {
        CLMS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clms(&mut self) -> CLMS_W {
        CLMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clms](index.html) module"]
pub struct CLMS_SPEC;
impl crate::RegisterSpec for CLMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clms::R](R) reader structure"]
impl crate::Readable for CLMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clms::W](W) writer structure"]
impl crate::Writable for CLMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLMS to value 0x20"]
impl crate::Resettable for CLMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
