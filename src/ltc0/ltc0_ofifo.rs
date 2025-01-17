#[doc = "Register `LTC0_OFIFO` reader"]
pub struct R(crate::R<LTC0_OFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_OFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_OFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_OFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFIFO` reader - Output FIFO"]
pub struct OFIFO_R(crate::FieldReader<u32, u32>);
impl OFIFO_R {
    pub(crate) fn new(bits: u32) -> Self {
        OFIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFIFO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Output FIFO"]
    #[inline(always)]
    pub fn ofifo(&self) -> OFIFO_R {
        OFIFO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "LTC Output Data FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ofifo](index.html) module"]
pub struct LTC0_OFIFO_SPEC;
impl crate::RegisterSpec for LTC0_OFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_ofifo::R](R) reader structure"]
impl crate::Readable for LTC0_OFIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTC0_OFIFO to value 0"]
impl crate::Resettable for LTC0_OFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
