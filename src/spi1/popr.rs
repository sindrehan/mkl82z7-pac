#[doc = "Register `POPR` reader"]
pub struct R(crate::R<POPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Received Data"]
pub struct RXDATA_R(crate::FieldReader<u32, u32>);
impl RXDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Received Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "POP RX FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [popr](index.html) module"]
pub struct POPR_SPEC;
impl crate::RegisterSpec for POPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [popr::R](R) reader structure"]
impl crate::Readable for POPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets POPR to value 0"]
impl crate::Resettable for POPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
