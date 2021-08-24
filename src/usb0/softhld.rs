#[doc = "Register `SOFTHLD` reader"]
pub struct R(crate::R<SOFTHLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTHLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTHLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTHLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTHLD` writer"]
pub struct W(crate::W<SOFTHLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTHLD_SPEC>;
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
impl From<crate::W<SOFTHLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTHLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Represents the SOF count threshold in byte times when SOFDYNTHLD=0 or 8 byte times when SOFDYNTHLD=1"]
pub struct CNT_R(crate::FieldReader<u8, u8>);
impl CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - Represents the SOF count threshold in byte times when SOFDYNTHLD=0 or 8 byte times when SOFDYNTHLD=1"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Represents the SOF count threshold in byte times when SOFDYNTHLD=0 or 8 byte times when SOFDYNTHLD=1"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Represents the SOF count threshold in byte times when SOFDYNTHLD=0 or 8 byte times when SOFDYNTHLD=1"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SOF Threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softhld](index.html) module"]
pub struct SOFTHLD_SPEC;
impl crate::RegisterSpec for SOFTHLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [softhld::R](R) reader structure"]
impl crate::Readable for SOFTHLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [softhld::W](W) writer structure"]
impl crate::Writable for SOFTHLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTHLD to value 0"]
impl crate::Resettable for SOFTHLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
