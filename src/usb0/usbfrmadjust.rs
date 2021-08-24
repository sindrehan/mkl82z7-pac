#[doc = "Register `USBFRMADJUST` reader"]
pub struct R(crate::R<USBFRMADJUST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBFRMADJUST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBFRMADJUST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBFRMADJUST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBFRMADJUST` writer"]
pub struct W(crate::W<USBFRMADJUST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBFRMADJUST_SPEC>;
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
impl From<crate::W<USBFRMADJUST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBFRMADJUST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADJ` reader - Frame Adjustment"]
pub struct ADJ_R(crate::FieldReader<u8, u8>);
impl ADJ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADJ` writer - Frame Adjustment"]
pub struct ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Frame Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Adjustment"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W {
        ADJ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Adjust Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbfrmadjust](index.html) module"]
pub struct USBFRMADJUST_SPEC;
impl crate::RegisterSpec for USBFRMADJUST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbfrmadjust::R](R) reader structure"]
impl crate::Readable for USBFRMADJUST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbfrmadjust::W](W) writer structure"]
impl crate::Writable for USBFRMADJUST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBFRMADJUST to value 0"]
impl crate::Resettable for USBFRMADJUST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
