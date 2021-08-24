#[doc = "Register `BDTPAGE1` reader"]
pub struct R(crate::R<BDTPAGE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTPAGE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTPAGE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTPAGE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTPAGE1` writer"]
pub struct W(crate::W<BDTPAGE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTPAGE1_SPEC>;
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
impl From<crate::W<BDTPAGE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTPAGE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDTBA` reader - Provides address bits 15 through 9 of the BDT base address."]
pub struct BDTBA_R(crate::FieldReader<u8, u8>);
impl BDTBA_R {
    pub(crate) fn new(bits: u8) -> Self {
        BDTBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDTBA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDTBA` writer - Provides address bits 15 through 9 of the BDT base address."]
pub struct BDTBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BDTBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u8 & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Provides address bits 15 through 9 of the BDT base address."]
    #[inline(always)]
    pub fn bdtba(&self) -> BDTBA_R {
        BDTBA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Provides address bits 15 through 9 of the BDT base address."]
    #[inline(always)]
    pub fn bdtba(&mut self) -> BDTBA_W {
        BDTBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BDT Page register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtpage1](index.html) module"]
pub struct BDTPAGE1_SPEC;
impl crate::RegisterSpec for BDTPAGE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bdtpage1::R](R) reader structure"]
impl crate::Readable for BDTPAGE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtpage1::W](W) writer structure"]
impl crate::Writable for BDTPAGE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDTPAGE1 to value 0"]
impl crate::Resettable for BDTPAGE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
