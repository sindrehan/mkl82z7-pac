#[doc = "Register `LTC0_CTX_8` reader"]
pub struct R(crate::R<LTC0_CTX_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_CTX_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_CTX_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_CTX_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_CTX_8` writer"]
pub struct W(crate::W<LTC0_CTX_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_CTX_8_SPEC>;
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
impl From<crate::W<LTC0_CTX_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_CTX_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTX` reader - CTX"]
pub struct CTX_R(crate::FieldReader<u32, u32>);
impl CTX_R {
    pub(crate) fn new(bits: u32) -> Self {
        CTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTX` writer - CTX"]
pub struct CTX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CTX"]
    #[inline(always)]
    pub fn ctx(&self) -> CTX_R {
        CTX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CTX"]
    #[inline(always)]
    pub fn ctx(&mut self) -> CTX_W {
        CTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_8](index.html) module"]
pub struct LTC0_CTX_8_SPEC;
impl crate::RegisterSpec for LTC0_CTX_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_ctx_8::R](R) reader structure"]
impl crate::Readable for LTC0_CTX_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_8::W](W) writer structure"]
impl crate::Writable for LTC0_CTX_8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_CTX_8 to value 0"]
impl crate::Resettable for LTC0_CTX_8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
