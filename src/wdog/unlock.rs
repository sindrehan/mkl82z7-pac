#[doc = "Register `UNLOCK` reader"]
pub struct R(crate::R<UNLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNLOCK` writer"]
pub struct W(crate::W<UNLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNLOCK_SPEC>;
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
impl From<crate::W<UNLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOGUNLOCK` reader - Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
pub struct WDOGUNLOCK_R(crate::FieldReader<u16, u16>);
impl WDOGUNLOCK_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDOGUNLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGUNLOCK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGUNLOCK` writer - Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
pub struct WDOGUNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGUNLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
    #[inline(always)]
    pub fn wdogunlock(&self) -> WDOGUNLOCK_R {
        WDOGUNLOCK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
    #[inline(always)]
    pub fn wdogunlock(&mut self) -> WDOGUNLOCK_W {
        WDOGUNLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Unlock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlock](index.html) module"]
pub struct UNLOCK_SPEC;
impl crate::RegisterSpec for UNLOCK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [unlock::R](R) reader structure"]
impl crate::Readable for UNLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unlock::W](W) writer structure"]
impl crate::Writable for UNLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UNLOCK to value 0xd928"]
impl crate::Resettable for UNLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xd928
    }
}
