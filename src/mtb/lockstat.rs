#[doc = "Reader of register LOCKSTAT"]
pub type R = crate::R<u32, super::LOCKSTAT>;
#[doc = "Reader of field `LOCKSTAT`"]
pub type LOCKSTAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - LOCKSTAT"]
    #[inline(always)]
    pub fn lockstat(&self) -> LOCKSTAT_R {
        LOCKSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
