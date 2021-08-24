#[doc = "Register `DIVISOR` reader"]
pub struct R(crate::R<DIVISOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVISOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVISOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVISOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVISOR` writer"]
pub struct W(crate::W<DIVISOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVISOR_SPEC>;
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
impl From<crate::W<DIVISOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVISOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVISOR_VALUE` reader - Divisor (F/D) Value"]
pub struct DIVISOR_VALUE_R(crate::FieldReader<u16, u16>);
impl DIVISOR_VALUE_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVISOR_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVISOR_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVISOR_VALUE` writer - Divisor (F/D) Value"]
pub struct DIVISOR_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVISOR_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Divisor (F/D) Value"]
    #[inline(always)]
    pub fn divisor_value(&self) -> DIVISOR_VALUE_R {
        DIVISOR_VALUE_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Divisor (F/D) Value"]
    #[inline(always)]
    pub fn divisor_value(&mut self) -> DIVISOR_VALUE_W {
        DIVISOR_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divisor](index.html) module"]
pub struct DIVISOR_SPEC;
impl crate::RegisterSpec for DIVISOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divisor::R](R) reader structure"]
impl crate::Readable for DIVISOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divisor::W](W) writer structure"]
impl crate::Writable for DIVISOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIVISOR to value 0x0174"]
impl crate::Resettable for DIVISOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0174
    }
}
