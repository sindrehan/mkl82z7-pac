#[doc = "Register `CRCH` reader"]
pub struct R(crate::R<CRC_CRCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CRCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_CRCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_CRCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCH` writer"]
pub struct W(crate::W<CRC_CRCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CRCH_SPEC>;
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
impl From<crate::W<CRC_CRCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_CRCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCH` reader - CRCH stores the high 16 bits of the 16/32 bit CRC"]
pub struct CRCH_R(crate::FieldReader<u16, u16>);
impl CRCH_R {
    pub(crate) fn new(bits: u16) -> Self {
        CRCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCH` writer - CRCH stores the high 16 bits of the 16/32 bit CRC"]
pub struct CRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRCH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn crch(&self) -> CRCH_R {
        CRCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRCH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn crch(&mut self) -> CRCH_W {
        CRCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_CRCH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_crch](index.html) module"]
pub struct CRC_CRCH_SPEC;
impl crate::RegisterSpec for CRC_CRCH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc_crch::R](R) reader structure"]
impl crate::Readable for CRC_CRCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_crch::W](W) writer structure"]
impl crate::Writable for CRC_CRCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCH to value 0xffff"]
impl crate::Resettable for CRC_CRCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
