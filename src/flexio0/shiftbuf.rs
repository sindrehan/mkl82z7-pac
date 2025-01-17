#[doc = "Register `SHIFTBUF%s` reader"]
pub struct R(crate::R<SHIFTBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUF%s` writer"]
pub struct W(crate::W<SHIFTBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUF_SPEC>;
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
impl From<crate::W<SHIFTBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUF` reader - Shift Buffer"]
pub struct SHIFTBUF_R(crate::FieldReader<u32, u32>);
impl SHIFTBUF_R {
    pub(crate) fn new(bits: u32) -> Self {
        SHIFTBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFTBUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFTBUF` writer - Shift Buffer"]
pub struct SHIFTBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbuf(&self) -> SHIFTBUF_R {
        SHIFTBUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbuf(&mut self) -> SHIFTBUF_W {
        SHIFTBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbuf](index.html) module"]
pub struct SHIFTBUF_SPEC;
impl crate::RegisterSpec for SHIFTBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbuf::R](R) reader structure"]
impl crate::Readable for SHIFTBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbuf::W](W) writer structure"]
impl crate::Writable for SHIFTBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTBUF%s to value 0"]
impl crate::Resettable for SHIFTBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
