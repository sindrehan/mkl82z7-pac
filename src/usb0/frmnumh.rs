#[doc = "Register `FRMNUMH` reader"]
pub struct R(crate::R<FRMNUMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMNUMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMNUMH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMNUMH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRMNUMH` writer"]
pub struct W(crate::W<FRMNUMH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRMNUMH_SPEC>;
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
impl From<crate::W<FRMNUMH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRMNUMH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRM` reader - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
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
#[doc = "Field `FRM` writer - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
pub struct FRM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline(always)]
    pub fn frm(&self) -> FRM_R {
        FRM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
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
#[doc = "Frame Number register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmnumh](index.html) module"]
pub struct FRMNUMH_SPEC;
impl crate::RegisterSpec for FRMNUMH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [frmnumh::R](R) reader structure"]
impl crate::Readable for FRMNUMH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frmnumh::W](W) writer structure"]
impl crate::Writable for FRMNUMH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRMNUMH to value 0"]
impl crate::Resettable for FRMNUMH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
