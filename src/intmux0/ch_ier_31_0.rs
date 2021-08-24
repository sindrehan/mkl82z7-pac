#[doc = "Register `CH%s_IER_31_0` reader"]
pub struct R(crate::R<CH_IER_31_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_IER_31_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_IER_31_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_IER_31_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_IER_31_0` writer"]
pub struct W(crate::W<CH_IER_31_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_IER_31_0_SPEC>;
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
impl From<crate::W<CH_IER_31_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_IER_31_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum INTE_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<INTE_A> for u32 {
    #[inline(always)]
    fn from(variant: INTE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTE` reader - Interrupt Enable"]
pub struct INTE_R(crate::FieldReader<u32, INTE_A>);
impl INTE_R {
    pub(crate) fn new(bits: u32) -> Self {
        INTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTE_A> {
        match self.bits {
            0 => Some(INTE_A::_0),
            1 => Some(INTE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTE_A::_1
    }
}
impl core::ops::Deref for INTE_R {
    type Target = crate::FieldReader<u32, INTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTE` writer - Interrupt Enable"]
pub struct INTE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTE_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Enable"]
    #[inline(always)]
    pub fn inte(&self) -> INTE_R {
        INTE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Enable"]
    #[inline(always)]
    pub fn inte(&mut self) -> INTE_W {
        INTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ier_31_0](index.html) module"]
pub struct CH_IER_31_0_SPEC;
impl crate::RegisterSpec for CH_IER_31_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_ier_31_0::R](R) reader structure"]
impl crate::Readable for CH_IER_31_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_ier_31_0::W](W) writer structure"]
impl crate::Writable for CH_IER_31_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_IER_31_0 to value 0"]
impl crate::Resettable for CH_IER_31_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
