#[doc = "Register `STCTRLL` reader"]
pub struct R(crate::R<STCTRLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCTRLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCTRLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCTRLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCTRLL` writer"]
pub struct W(crate::W<STCTRLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCTRLL_SPEC>;
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
impl From<crate::W<STCTRLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCTRLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTFLG` reader - Interrupt flag"]
pub struct INTFLG_R(crate::FieldReader<bool, bool>);
impl INTFLG_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTFLG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTFLG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTFLG` writer - Interrupt flag"]
pub struct INTFLG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFLG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Interrupt flag"]
    #[inline(always)]
    pub fn intflg(&self) -> INTFLG_R {
        INTFLG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Interrupt flag"]
    #[inline(always)]
    pub fn intflg(&mut self) -> INTFLG_W {
        INTFLG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Status and Control Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stctrll](index.html) module"]
pub struct STCTRLL_SPEC;
impl crate::RegisterSpec for STCTRLL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stctrll::R](R) reader structure"]
impl crate::Readable for STCTRLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stctrll::W](W) writer structure"]
impl crate::Writable for STCTRLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STCTRLL to value 0x01"]
impl crate::Resettable for STCTRLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
