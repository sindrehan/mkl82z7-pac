#[doc = "Reader of register LTC0_OFIFO"]
pub type R = crate::R<u32, super::LTC0_OFIFO>;
#[doc = "Reader of field `OFIFO`"]
pub type OFIFO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output FIFO"]
    #[inline(always)]
    pub fn ofifo(&self) -> OFIFO_R {
        OFIFO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
