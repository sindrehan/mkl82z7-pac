#[doc = "Register `TCD%s_SADDR` reader"]
pub struct R(crate::R<TCD_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD%s_SADDR` writer"]
pub struct W(crate::W<TCD_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD_SADDR_SPEC>;
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
impl From<crate::W<TCD_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDR` reader - Source Address"]
pub struct SADDR_R(crate::FieldReader<u32, u32>);
impl SADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        SADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR` writer - Source Address"]
pub struct SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Source Address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W {
        SADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_saddr](index.html) module"]
pub struct TCD_SADDR_SPEC;
impl crate::RegisterSpec for TCD_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcd_saddr::R](R) reader structure"]
impl crate::Readable for TCD_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd_saddr::W](W) writer structure"]
impl crate::Writable for TCD_SADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD%s_SADDR to value 0"]
impl crate::Resettable for TCD_SADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
