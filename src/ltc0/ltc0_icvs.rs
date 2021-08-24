#[doc = "Register `LTC0_ICVS` reader"]
pub struct R(crate::R<LTC0_ICVS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_ICVS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_ICVS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_ICVS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_ICVS` writer"]
pub struct W(crate::W<LTC0_ICVS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_ICVS_SPEC>;
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
impl From<crate::W<LTC0_ICVS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_ICVS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICVS` reader - ICV Size, in Bytes."]
pub struct ICVS_R(crate::FieldReader<u8, u8>);
impl ICVS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ICVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICVS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICVS` writer - ICV Size, in Bytes."]
pub struct ICVS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ICV Size, in Bytes."]
    #[inline(always)]
    pub fn icvs(&self) -> ICVS_R {
        ICVS_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ICV Size, in Bytes."]
    #[inline(always)]
    pub fn icvs(&mut self) -> ICVS_W {
        ICVS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC ICV Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_icvs](index.html) module"]
pub struct LTC0_ICVS_SPEC;
impl crate::RegisterSpec for LTC0_ICVS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_icvs::R](R) reader structure"]
impl crate::Readable for LTC0_ICVS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_icvs::W](W) writer structure"]
impl crate::Writable for LTC0_ICVS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_ICVS to value 0"]
impl crate::Resettable for LTC0_ICVS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
