#[doc = "Register `TRNG0_SCR6PL` reader"]
pub struct R(crate::R<TRNG0_TRNG0_SCR6PL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_SCR6PL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_SCR6PL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_SCR6PL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_SCR6PL` writer"]
pub struct W(crate::W<TRNG0_TRNG0_SCR6PL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_TRNG0_SCR6PL_SPEC>;
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
impl From<crate::W<TRNG0_TRNG0_SCR6PL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_TRNG0_SCR6PL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN6P_MAX` reader - Run Length 6+ Maximum Limit"]
pub struct RUN6P_MAX_R(crate::FieldReader<u16, u16>);
impl RUN6P_MAX_R {
    pub(crate) fn new(bits: u16) -> Self {
        RUN6P_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUN6P_MAX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUN6P_MAX` writer - Run Length 6+ Maximum Limit"]
pub struct RUN6P_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN6P_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `RUN6P_RNG` reader - Run Length 6+ Range"]
pub struct RUN6P_RNG_R(crate::FieldReader<u16, u16>);
impl RUN6P_RNG_R {
    pub(crate) fn new(bits: u16) -> Self {
        RUN6P_RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUN6P_RNG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUN6P_RNG` writer - Run Length 6+ Range"]
pub struct RUN6P_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN6P_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Run Length 6+ Maximum Limit"]
    #[inline(always)]
    pub fn run6p_max(&self) -> RUN6P_MAX_R {
        RUN6P_MAX_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Run Length 6+ Range"]
    #[inline(always)]
    pub fn run6p_rng(&self) -> RUN6P_RNG_R {
        RUN6P_RNG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Run Length 6+ Maximum Limit"]
    #[inline(always)]
    pub fn run6p_max(&mut self) -> RUN6P_MAX_W {
        RUN6P_MAX_W { w: self }
    }
    #[doc = "Bits 16:26 - Run Length 6+ Range"]
    #[inline(always)]
    pub fn run6p_rng(&mut self) -> RUN6P_RNG_W {
        RUN6P_RNG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Statistical Check Run Length 6+ Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_scr6pl](index.html) module"]
pub struct TRNG0_TRNG0_SCR6PL_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_SCR6PL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_scr6pl::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_SCR6PL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_trng0_scr6pl::W](W) writer structure"]
impl crate::Writable for TRNG0_TRNG0_SCR6PL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_SCR6PL to value 0x002e_002f"]
impl crate::Resettable for TRNG0_TRNG0_SCR6PL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x002e_002f
    }
}
