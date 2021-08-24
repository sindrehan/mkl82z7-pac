#[doc = "Register `LTC0_PKN1_12` reader"]
pub struct R(crate::R<LTC0_LTC0_PKN1_12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_LTC0_PKN1_12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_LTC0_PKN1_12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_LTC0_PKN1_12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_PKN1_12` writer"]
pub struct W(crate::W<LTC0_LTC0_PKN1_12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_LTC0_PKN1_12_SPEC>;
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
impl From<crate::W<LTC0_LTC0_PKN1_12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_LTC0_PKN1_12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKHA_N1` reader - N1 VALUE"]
pub struct PKHA_N1_R(crate::FieldReader<u32, u32>);
impl PKHA_N1_R {
    pub(crate) fn new(bits: u32) -> Self {
        PKHA_N1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKHA_N1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKHA_N1` writer - N1 VALUE"]
pub struct PKHA_N1_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_N1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - N1 VALUE"]
    #[inline(always)]
    pub fn pkha_n1(&self) -> PKHA_N1_R {
        PKHA_N1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - N1 VALUE"]
    #[inline(always)]
    pub fn pkha_n1(&mut self) -> PKHA_N1_W {
        PKHA_N1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC PKHA N1 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ltc0_pkn1_12](index.html) module"]
pub struct LTC0_LTC0_PKN1_12_SPEC;
impl crate::RegisterSpec for LTC0_LTC0_PKN1_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_ltc0_pkn1_12::R](R) reader structure"]
impl crate::Readable for LTC0_LTC0_PKN1_12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_ltc0_pkn1_12::W](W) writer structure"]
impl crate::Writable for LTC0_LTC0_PKN1_12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_PKN1_12 to value 0"]
impl crate::Resettable for LTC0_LTC0_PKN1_12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
