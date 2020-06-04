#[doc = "Reader of register RBSR"]
pub type R = crate::R<u32, super::RBSR>;
#[doc = "Reader of field `RDBFL`"]
pub type RDBFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RDCTR`"]
pub type RDCTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 8:12 - RX Buffer Fill Level"]
    #[inline(always)]
    pub fn rdbfl(&self) -> RDBFL_R {
        RDBFL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Read Counter"]
    #[inline(always)]
    pub fn rdctr(&self) -> RDCTR_R {
        RDCTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
