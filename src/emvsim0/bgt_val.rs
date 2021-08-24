#[doc = "Register `BGT_VAL` reader"]
pub struct R(crate::R<BGT_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGT_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGT_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGT_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGT_VAL` writer"]
pub struct W(crate::W<BGT_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGT_VAL_SPEC>;
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
impl From<crate::W<BGT_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGT_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGT` reader - Block Guard Time Value"]
pub struct BGT_R(crate::FieldReader<u16, u16>);
impl BGT_R {
    pub(crate) fn new(bits: u16) -> Self {
        BGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGT` writer - Block Guard Time Value"]
pub struct BGT_W<'a> {
    w: &'a mut W,
}
impl<'a> BGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Block Guard Time Value"]
    #[inline(always)]
    pub fn bgt(&self) -> BGT_R {
        BGT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block Guard Time Value"]
    #[inline(always)]
    pub fn bgt(&mut self) -> BGT_W {
        BGT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Guard Time Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgt_val](index.html) module"]
pub struct BGT_VAL_SPEC;
impl crate::RegisterSpec for BGT_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgt_val::R](R) reader structure"]
impl crate::Readable for BGT_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgt_val::W](W) writer structure"]
impl crate::Writable for BGT_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGT_VAL to value 0"]
impl crate::Resettable for BGT_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
