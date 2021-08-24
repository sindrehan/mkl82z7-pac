#[doc = "Register `TIMIEN` reader"]
pub struct R(crate::R<TIMIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMIEN` writer"]
pub struct W(crate::W<TIMIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMIEN_SPEC>;
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
impl From<crate::W<TIMIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Status Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TEIE_A {
    #[doc = "0: Timer Status Flag interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Timer Status Flag interrupt is enabled"]
    _1 = 1,
}
impl From<TEIE_A> for u8 {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TEIE` reader - Timer Status Interrupt Enable"]
pub struct TEIE_R(crate::FieldReader<u8, TEIE_A>);
impl TEIE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEIE_A> {
        match self.bits {
            0 => Some(TEIE_A::_0),
            1 => Some(TEIE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TEIE_A::_1
    }
}
impl core::ops::Deref for TEIE_R {
    type Target = crate::FieldReader<u8, TEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIE` writer - Timer Status Interrupt Enable"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer Status Flag interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEIE_A::_0)
    }
    #[doc = "Timer Status Flag interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEIE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer Status Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Status Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timien](index.html) module"]
pub struct TIMIEN_SPEC;
impl crate::RegisterSpec for TIMIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timien::R](R) reader structure"]
impl crate::Readable for TIMIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timien::W](W) writer structure"]
impl crate::Writable for TIMIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMIEN to value 0"]
impl crate::Resettable for TIMIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
