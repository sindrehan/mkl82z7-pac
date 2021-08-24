#[doc = "Register `LTC0_FIFOSTA` reader"]
pub struct R(crate::R<LTC0_FIFOSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_FIFOSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_FIFOSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_FIFOSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IFL` reader - Input FIFO Level. These bits indicate the current number of entries in the Input FIFO."]
pub struct IFL_R(crate::FieldReader<u8, u8>);
impl IFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFF` reader - Input FIFO Full. The Input FIFO is full and should not be written to."]
pub struct IFF_R(crate::FieldReader<bool, bool>);
impl IFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFL` reader - Output FIFO Level. These bits indicate the current number of entries in the Output FIFO."]
pub struct OFL_R(crate::FieldReader<u8, u8>);
impl OFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF` reader - Output FIFO Full. The Output FIFO is full and should not be written to."]
pub struct OFF_R(crate::FieldReader<bool, bool>);
impl OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Input FIFO Level. These bits indicate the current number of entries in the Input FIFO."]
    #[inline(always)]
    pub fn ifl(&self) -> IFL_R {
        IFL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Input FIFO Full. The Input FIFO is full and should not be written to."]
    #[inline(always)]
    pub fn iff(&self) -> IFF_R {
        IFF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Output FIFO Level. These bits indicate the current number of entries in the Output FIFO."]
    #[inline(always)]
    pub fn ofl(&self) -> OFL_R {
        OFL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Output FIFO Full. The Output FIFO is full and should not be written to."]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "LTC FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_fifosta](index.html) module"]
pub struct LTC0_FIFOSTA_SPEC;
impl crate::RegisterSpec for LTC0_FIFOSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_fifosta::R](R) reader structure"]
impl crate::Readable for LTC0_FIFOSTA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTC0_FIFOSTA to value 0"]
impl crate::Resettable for LTC0_FIFOSTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
