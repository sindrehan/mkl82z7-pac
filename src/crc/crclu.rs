#[doc = "Register `CRCLU` reader"]
pub struct R(crate::R<CRCLU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCLU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCLU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCLU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCLU` writer"]
pub struct W(crate::W<CRCLU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCLU_SPEC>;
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
impl From<crate::W<CRCLU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCLU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCLU` reader - CRCLL stores the second 8 bits of the 32 bit CRC"]
pub struct CRCLU_R(crate::FieldReader<u8, u8>);
impl CRCLU_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCLU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCLU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCLU` writer - CRCLL stores the second 8 bits of the 32 bit CRC"]
pub struct CRCLU_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCLU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crclu(&self) -> CRCLU_R {
        CRCLU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crclu(&mut self) -> CRCLU_W {
        CRCLU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_CRCLU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crclu](index.html) module"]
pub struct CRCLU_SPEC;
impl crate::RegisterSpec for CRCLU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crclu::R](R) reader structure"]
impl crate::Readable for CRCLU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crclu::W](W) writer structure"]
impl crate::Writable for CRCLU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCLU to value 0xff"]
impl crate::Resettable for CRCLU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
