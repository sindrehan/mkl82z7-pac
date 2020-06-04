#[doc = "Reader of register TBSR"]
pub type R = crate::R<u32, super::TBSR>;
#[doc = "Reader of field `TRBFL`"]
pub type TRBFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRCTR`"]
pub type TRCTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 8:12 - TX Buffer Fill Level"]
    #[inline(always)]
    pub fn trbfl(&self) -> TRBFL_R {
        TRBFL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Transmit Counter"]
    #[inline(always)]
    pub fn trctr(&self) -> TRCTR_R {
        TRCTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
