#[doc = "Register `TMROUTL` reader"]
pub struct R(crate::R<TMROUTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMROUTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMROUTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMROUTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMROUTL` writer"]
pub struct W(crate::W<TMROUTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMROUTL_SPEC>;
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
impl From<crate::W<TMROUTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMROUTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEROUTLOW` reader - Shows the value of the lower 16 bits of the watchdog timer."]
pub struct TIMEROUTLOW_R(crate::FieldReader<u16, u16>);
impl TIMEROUTLOW_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIMEROUTLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEROUTLOW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEROUTLOW` writer - Shows the value of the lower 16 bits of the watchdog timer."]
pub struct TIMEROUTLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEROUTLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shows the value of the lower 16 bits of the watchdog timer."]
    #[inline(always)]
    pub fn timeroutlow(&self) -> TIMEROUTLOW_R {
        TIMEROUTLOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shows the value of the lower 16 bits of the watchdog timer."]
    #[inline(always)]
    pub fn timeroutlow(&mut self) -> TIMEROUTLOW_W {
        TIMEROUTLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Output Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmroutl](index.html) module"]
pub struct TMROUTL_SPEC;
impl crate::RegisterSpec for TMROUTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tmroutl::R](R) reader structure"]
impl crate::Readable for TMROUTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmroutl::W](W) writer structure"]
impl crate::Writable for TMROUTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMROUTL to value 0"]
impl crate::Resettable for TMROUTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
