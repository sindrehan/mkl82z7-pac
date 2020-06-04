#[doc = "Reader of register LTC0_FIFOSTA"]
pub type R = crate::R<u32, super::LTC0_FIFOSTA>;
#[doc = "Reader of field `IFL`"]
pub type IFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `IFF`"]
pub type IFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OFL`"]
pub type OFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `OFF`"]
pub type OFF_R = crate::R<bool, bool>;
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
