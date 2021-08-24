#[doc = "Register `BDTPAGE2` reader"]
pub struct R(crate::R<BDTPAGE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTPAGE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTPAGE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTPAGE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTPAGE2` writer"]
pub struct W(crate::W<BDTPAGE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTPAGE2_SPEC>;
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
impl From<crate::W<BDTPAGE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTPAGE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDTBA` reader - Provides address bits 23 through 16 of the BDT base address that defines the location of Buffer Descriptor Table resides in system memory"]
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
#[doc = "Field `BDTBA` writer - Provides address bits 23 through 16 of the BDT base address that defines the location of Buffer Descriptor Table resides in system memory"]
pub struct BDTBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BDTBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Provides address bits 23 through 16 of the BDT base address that defines the location of Buffer Descriptor Table resides in system memory"]
    #[inline(always)]
    pub fn bdtba(&self) -> BDTBA_R {
        BDTBA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Provides address bits 23 through 16 of the BDT base address that defines the location of Buffer Descriptor Table resides in system memory"]
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
#[doc = "BDT Page Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtpage2](index.html) module"]
pub struct BDTPAGE2_SPEC;
impl crate::RegisterSpec for BDTPAGE2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bdtpage2::R](R) reader structure"]
impl crate::Readable for BDTPAGE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtpage2::W](W) writer structure"]
impl crate::Writable for BDTPAGE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDTPAGE2 to value 0"]
impl crate::Resettable for BDTPAGE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
