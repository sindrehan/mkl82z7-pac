#[doc = "Register `REFRESH` reader"]
pub struct R(crate::R<REFRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFRESH` writer"]
pub struct W(crate::W<REFRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFRESH_SPEC>;
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
impl From<crate::W<REFRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOGREFRESH` reader - Watchdog refresh register"]
pub struct WDOGREFRESH_R(crate::FieldReader<u16, u16>);
impl WDOGREFRESH_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDOGREFRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGREFRESH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGREFRESH` writer - Watchdog refresh register"]
pub struct WDOGREFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGREFRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog refresh register"]
    #[inline(always)]
    pub fn wdogrefresh(&self) -> WDOGREFRESH_R {
        WDOGREFRESH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog refresh register"]
    #[inline(always)]
    pub fn wdogrefresh(&mut self) -> WDOGREFRESH_W {
        WDOGREFRESH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Refresh register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refresh](index.html) module"]
pub struct REFRESH_SPEC;
impl crate::RegisterSpec for REFRESH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [refresh::R](R) reader structure"]
impl crate::Readable for REFRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refresh::W](W) writer structure"]
impl crate::Writable for REFRESH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFRESH to value 0xb480"]
impl crate::Resettable for REFRESH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb480
    }
}
