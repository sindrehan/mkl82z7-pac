#[doc = "Reader of register TRNG0_ENT4"]
pub type R = crate::R<u32, super::TRNG0_ENT4>;
#[doc = "Reader of field `ENT`"]
pub type ENT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Entropy Value"]
    #[inline(always)]
    pub fn ent(&self) -> ENT_R {
        ENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
