#[doc = "Register `LTC0_KS` reader"]
pub struct R(crate::R<LTC0_KS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_KS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_KS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_KS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_KS` writer"]
pub struct W(crate::W<LTC0_KS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_KS_SPEC>;
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
impl From<crate::W<LTC0_KS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_KS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KS` reader - Key Size. This is the size of a Key measured in bytes"]
pub struct KS_R(crate::FieldReader<u8, u8>);
impl KS_R {
    pub(crate) fn new(bits: u8) -> Self {
        KS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KS` writer - Key Size. This is the size of a Key measured in bytes"]
pub struct KS_W<'a> {
    w: &'a mut W,
}
impl<'a> KS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Key Size. This is the size of a Key measured in bytes"]
    #[inline(always)]
    pub fn ks(&self) -> KS_R {
        KS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Key Size. This is the size of a Key measured in bytes"]
    #[inline(always)]
    pub fn ks(&mut self) -> KS_W {
        KS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC Key Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ks](index.html) module"]
pub struct LTC0_KS_SPEC;
impl crate::RegisterSpec for LTC0_KS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_ks::R](R) reader structure"]
impl crate::Readable for LTC0_KS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_ks::W](W) writer structure"]
impl crate::Writable for LTC0_KS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_KS to value 0"]
impl crate::Resettable for LTC0_KS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
