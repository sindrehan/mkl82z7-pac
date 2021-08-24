#[doc = "Register `CRCHU` reader"]
pub struct R(crate::R<CRCHU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCHU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCHU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCHU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCHU` writer"]
pub struct W(crate::W<CRCHU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCHU_SPEC>;
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
impl From<crate::W<CRCHU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCHU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCHU` reader - CRCHU stores the fourth 8 bits of the 32 bit CRC"]
pub struct CRCHU_R(crate::FieldReader<u8, u8>);
impl CRCHU_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCHU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCHU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCHU` writer - CRCHU stores the fourth 8 bits of the 32 bit CRC"]
pub struct CRCHU_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCHU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchu(&self) -> CRCHU_R {
        CRCHU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchu(&mut self) -> CRCHU_W {
        CRCHU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_CRCHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crchu](index.html) module"]
pub struct CRCHU_SPEC;
impl crate::RegisterSpec for CRCHU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crchu::R](R) reader structure"]
impl crate::Readable for CRCHU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crchu::W](W) writer structure"]
impl crate::Writable for CRCHU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCHU to value 0xff"]
impl crate::Resettable for CRCHU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
