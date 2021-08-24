#[doc = "Register `CWT_VAL` reader"]
pub struct R(crate::R<CWT_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWT_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWT_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWT_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWT_VAL` writer"]
pub struct W(crate::W<CWT_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWT_VAL_SPEC>;
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
impl From<crate::W<CWT_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWT_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWT` reader - Character Wait Time Value"]
pub struct CWT_R(crate::FieldReader<u16, u16>);
impl CWT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWT` writer - Character Wait Time Value"]
pub struct CWT_W<'a> {
    w: &'a mut W,
}
impl<'a> CWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Character Wait Time Value"]
    #[inline(always)]
    pub fn cwt(&self) -> CWT_R {
        CWT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Character Wait Time Value"]
    #[inline(always)]
    pub fn cwt(&mut self) -> CWT_W {
        CWT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Character Wait Time Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwt_val](index.html) module"]
pub struct CWT_VAL_SPEC;
impl crate::RegisterSpec for CWT_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwt_val::R](R) reader structure"]
impl crate::Readable for CWT_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwt_val::W](W) writer structure"]
impl crate::Writable for CWT_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWT_VAL to value 0xffff"]
impl crate::Resettable for CWT_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
