#[doc = "Register `TXFR%s` reader"]
pub struct R(crate::R<TXFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXDATA` reader - Transmit Data"]
pub struct TXDATA_R(crate::FieldReader<u16, u16>);
impl TXDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCMD_TXDATA` reader - Transmit Command or Transmit Data"]
pub struct TXCMD_TXDATA_R(crate::FieldReader<u16, u16>);
impl TXCMD_TXDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXCMD_TXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCMD_TXDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit Command or Transmit Data"]
    #[inline(always)]
    pub fn txcmd_txdata(&self) -> TXCMD_TXDATA_R {
        TXCMD_TXDATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Transmit FIFO Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfr](index.html) module"]
pub struct TXFR_SPEC;
impl crate::RegisterSpec for TXFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfr::R](R) reader structure"]
impl crate::Readable for TXFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXFR%s to value 0"]
impl crate::Resettable for TXFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
