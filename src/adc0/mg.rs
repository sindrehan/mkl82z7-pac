#[doc = "Register `MG` reader"]
pub struct R(crate::R<MG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MG` writer"]
pub struct W(crate::W<MG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MG_SPEC>;
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
impl From<crate::W<MG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MG` reader - Minus-Side Gain"]
pub struct MG_R(crate::FieldReader<u16, u16>);
impl MG_R {
    pub(crate) fn new(bits: u16) -> Self {
        MG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MG` writer - Minus-Side Gain"]
pub struct MG_W<'a> {
    w: &'a mut W,
}
impl<'a> MG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Minus-Side Gain"]
    #[inline(always)]
    pub fn mg(&self) -> MG_R {
        MG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minus-Side Gain"]
    #[inline(always)]
    pub fn mg(&mut self) -> MG_W {
        MG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mg](index.html) module"]
pub struct MG_SPEC;
impl crate::RegisterSpec for MG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mg::R](R) reader structure"]
impl crate::Readable for MG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mg::W](W) writer structure"]
impl crate::Writable for MG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MG to value 0x8200"]
impl crate::Resettable for MG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8200
    }
}
