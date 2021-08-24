#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_FIFO_DEPTH` reader - Receive FIFO Depth"]
pub struct RX_FIFO_DEPTH_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_DEPTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_DEPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_DEPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_DEPTH` reader - Transmit FIFO Depth"]
pub struct TX_FIFO_DEPTH_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_DEPTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_DEPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_DEPTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Depth"]
    #[inline(always)]
    pub fn rx_fifo_depth(&self) -> RX_FIFO_DEPTH_R {
        RX_FIFO_DEPTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit FIFO Depth"]
    #[inline(always)]
    pub fn tx_fifo_depth(&self) -> TX_FIFO_DEPTH_R {
        TX_FIFO_DEPTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0x1010"]
impl crate::Resettable for PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1010
    }
}
