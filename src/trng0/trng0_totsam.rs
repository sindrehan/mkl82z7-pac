#[doc = "Reader of register TRNG0_TOTSAM"]
pub type R = crate::R<u32, super::TRNG0_TOTSAM>;
#[doc = "Reader of field `TOT_SAM`"]
pub type TOT_SAM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - Total Samples"]
    #[inline(always)]
    pub fn tot_sam(&self) -> TOT_SAM_R {
        TOT_SAM_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
