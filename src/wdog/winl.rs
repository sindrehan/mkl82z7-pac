#[doc = "Register `WINL` reader"]
pub struct R(crate::R<WINL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINL` writer"]
pub struct W(crate::W<WINL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINL_SPEC>;
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
impl From<crate::W<WINL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINLOW` reader - Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
pub struct WINLOW_R(crate::FieldReader<u16, u16>);
impl WINLOW_R {
    pub(crate) fn new(bits: u16) -> Self {
        WINLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINLOW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINLOW` writer - Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
pub struct WINLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline(always)]
    pub fn winlow(&self) -> WINLOW_R {
        WINLOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline(always)]
    pub fn winlow(&mut self) -> WINLOW_W {
        WINLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Window Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winl](index.html) module"]
pub struct WINL_SPEC;
impl crate::RegisterSpec for WINL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [winl::R](R) reader structure"]
impl crate::Readable for WINL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winl::W](W) writer structure"]
impl crate::Writable for WINL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINL to value 0x10"]
impl crate::Resettable for WINL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
