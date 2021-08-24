#[doc = "Register `CRCHL` reader"]
pub struct R(crate::R<CRC_CRCHL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CRCHL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_CRCHL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_CRCHL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCHL` writer"]
pub struct W(crate::W<CRC_CRCHL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CRCHL_SPEC>;
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
impl From<crate::W<CRC_CRCHL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_CRCHL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCHL` reader - CRCHL stores the third 8 bits of the 32 bit CRC"]
pub struct CRCHL_R(crate::FieldReader<u8, u8>);
impl CRCHL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCHL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCHL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCHL` writer - CRCHL stores the third 8 bits of the 32 bit CRC"]
pub struct CRCHL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCHL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchl(&self) -> CRCHL_R {
        CRCHL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchl(&mut self) -> CRCHL_W {
        CRCHL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_CRCHL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_crchl](index.html) module"]
pub struct CRC_CRCHL_SPEC;
impl crate::RegisterSpec for CRC_CRCHL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crc_crchl::R](R) reader structure"]
impl crate::Readable for CRC_CRCHL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_crchl::W](W) writer structure"]
impl crate::Writable for CRC_CRCHL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCHL to value 0xff"]
impl crate::Resettable for CRC_CRCHL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
