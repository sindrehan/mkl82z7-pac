#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl From<crate::W<C2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACBFUP` reader - DAC Buffer Upper Limit"]
pub struct DACBFUP_R(crate::FieldReader<u8, u8>);
impl DACBFUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DACBFUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACBFUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBFUP` writer - DAC Buffer Upper Limit"]
pub struct DACBFUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Field `DACBFRP` reader - DAC Buffer Read Pointer"]
pub struct DACBFRP_R(crate::FieldReader<u8, u8>);
impl DACBFRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DACBFRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACBFRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBFRP` writer - DAC Buffer Read Pointer"]
pub struct DACBFRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u8 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DAC Buffer Upper Limit"]
    #[inline(always)]
    pub fn dacbfup(&self) -> DACBFUP_R {
        DACBFUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DAC Buffer Read Pointer"]
    #[inline(always)]
    pub fn dacbfrp(&self) -> DACBFRP_R {
        DACBFRP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DAC Buffer Upper Limit"]
    #[inline(always)]
    pub fn dacbfup(&mut self) -> DACBFUP_W {
        DACBFUP_W { w: self }
    }
    #[doc = "Bits 4:7 - DAC Buffer Read Pointer"]
    #[inline(always)]
    pub fn dacbfrp(&mut self) -> DACBFRP_W {
        DACBFRP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2 to value 0x0f"]
impl crate::Resettable for C2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
