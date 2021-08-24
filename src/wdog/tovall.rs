#[doc = "Register `TOVALL` reader"]
pub struct R(crate::R<TOVALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOVALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOVALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOVALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOVALL` writer"]
pub struct W(crate::W<TOVALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOVALL_SPEC>;
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
impl From<crate::W<TOVALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOVALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOVALLOW` reader - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
pub struct TOVALLOW_R(crate::FieldReader<u16, u16>);
impl TOVALLOW_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOVALLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOVALLOW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVALLOW` writer - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
pub struct TOVALLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    pub fn tovallow(&self) -> TOVALLOW_R {
        TOVALLOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    pub fn tovallow(&mut self) -> TOVALLOW_W {
        TOVALLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Time-out Value Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tovall](index.html) module"]
pub struct TOVALL_SPEC;
impl crate::RegisterSpec for TOVALL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tovall::R](R) reader structure"]
impl crate::Readable for TOVALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tovall::W](W) writer structure"]
impl crate::Writable for TOVALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOVALL to value 0x4b4c"]
impl crate::Resettable for TOVALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4b4c
    }
}
