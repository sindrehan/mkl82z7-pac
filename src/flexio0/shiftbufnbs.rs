#[doc = "Register `SHIFTBUFNBS%s` reader"]
pub struct R(crate::R<SHIFTBUFNBS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFNBS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFNBS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFNBS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFNBS%s` writer"]
pub struct W(crate::W<SHIFTBUFNBS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFNBS_SPEC>;
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
impl From<crate::W<SHIFTBUFNBS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFNBS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFNBS` reader - Shift Buffer"]
pub struct SHIFTBUFNBS_R(crate::FieldReader<u32, u32>);
impl SHIFTBUFNBS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SHIFTBUFNBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFTBUFNBS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFTBUFNBS` writer - Shift Buffer"]
pub struct SHIFTBUFNBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFNBS_W<'a> {
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
    pub fn shiftbufnbs(&self) -> SHIFTBUFNBS_R {
        SHIFTBUFNBS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufnbs(&mut self) -> SHIFTBUFNBS_W {
        SHIFTBUFNBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Nibble Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufnbs](index.html) module"]
pub struct SHIFTBUFNBS_SPEC;
impl crate::RegisterSpec for SHIFTBUFNBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufnbs::R](R) reader structure"]
impl crate::Readable for SHIFTBUFNBS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufnbs::W](W) writer structure"]
impl crate::Writable for SHIFTBUFNBS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTBUFNBS%s to value 0"]
impl crate::Resettable for SHIFTBUFNBS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
