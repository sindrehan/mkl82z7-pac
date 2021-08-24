#[doc = "Register `FRMNUML` reader"]
pub struct R(crate::R<FRMNUML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMNUML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMNUML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMNUML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRMNUML` writer"]
pub struct W(crate::W<FRMNUML_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRMNUML_SPEC>;
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
impl From<crate::W<FRMNUML_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRMNUML_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRM` reader - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
pub struct FRM_R(crate::FieldReader<u8, u8>);
impl FRM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRM` writer - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
pub struct FRM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline(always)]
    pub fn frm(&self) -> FRM_R {
        FRM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline(always)]
    pub fn frm(&mut self) -> FRM_W {
        FRM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Number register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmnuml](index.html) module"]
pub struct FRMNUML_SPEC;
impl crate::RegisterSpec for FRMNUML_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [frmnuml::R](R) reader structure"]
impl crate::Readable for FRMNUML_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frmnuml::W](W) writer structure"]
impl crate::Writable for FRMNUML_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRMNUML to value 0"]
impl crate::Resettable for FRMNUML_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
