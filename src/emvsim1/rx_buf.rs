#[doc = "Register `RX_BUF` reader"]
pub struct R(crate::R<RX_BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_BYTE` reader - Receive Data Byte Read"]
pub struct RX_BYTE_R(crate::FieldReader<u8, u8>);
impl RX_BYTE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BYTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Data Byte Read"]
    #[inline(always)]
    pub fn rx_byte(&self) -> RX_BYTE_R {
        RX_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Data Read Buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_buf](index.html) module"]
pub struct RX_BUF_SPEC;
impl crate::RegisterSpec for RX_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_buf::R](R) reader structure"]
impl crate::Readable for RX_BUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_BUF to value 0"]
impl crate::Resettable for RX_BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
