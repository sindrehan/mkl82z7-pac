#[doc = "Register `LTC0_PKESZ` reader"]
pub struct R(crate::R<LTC0_PKESZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_PKESZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_PKESZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_PKESZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_PKESZ` writer"]
pub struct W(crate::W<LTC0_PKESZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_PKESZ_SPEC>;
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
impl From<crate::W<LTC0_PKESZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_PKESZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKESZ` reader - PKHA E Size. This is the size of the numeric value, in bytes, contained within the PKHA E Register."]
pub struct PKESZ_R(crate::FieldReader<u16, u16>);
impl PKESZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKESZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKESZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKESZ` writer - PKHA E Size. This is the size of the numeric value, in bytes, contained within the PKHA E Register."]
pub struct PKESZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PKESZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PKHA E Size. This is the size of the numeric value, in bytes, contained within the PKHA E Register."]
    #[inline(always)]
    pub fn pkesz(&self) -> PKESZ_R {
        PKESZ_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PKHA E Size. This is the size of the numeric value, in bytes, contained within the PKHA E Register."]
    #[inline(always)]
    pub fn pkesz(&mut self) -> PKESZ_W {
        PKESZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC PKHA E Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkesz](index.html) module"]
pub struct LTC0_PKESZ_SPEC;
impl crate::RegisterSpec for LTC0_PKESZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_pkesz::R](R) reader structure"]
impl crate::Readable for LTC0_PKESZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_pkesz::W](W) writer structure"]
impl crate::Writable for LTC0_PKESZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_PKESZ to value 0"]
impl crate::Resettable for LTC0_PKESZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
