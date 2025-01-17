#[doc = "Register `RSTCNT` reader"]
pub struct R(crate::R<RSTCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCNT` writer"]
pub struct W(crate::W<RSTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCNT_SPEC>;
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
impl From<crate::W<RSTCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTCNT` reader - Counts the number of times the watchdog resets the system"]
pub struct RSTCNT_R(crate::FieldReader<u16, u16>);
impl RSTCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RSTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTCNT` writer - Counts the number of times the watchdog resets the system"]
pub struct RSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counts the number of times the watchdog resets the system"]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counts the number of times the watchdog resets the system"]
    #[inline(always)]
    pub fn rstcnt(&mut self) -> RSTCNT_W {
        RSTCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Reset Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcnt](index.html) module"]
pub struct RSTCNT_SPEC;
impl crate::RegisterSpec for RSTCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rstcnt::R](R) reader structure"]
impl crate::Readable for RSTCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcnt::W](W) writer structure"]
impl crate::Writable for RSTCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCNT to value 0"]
impl crate::Resettable for RSTCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
