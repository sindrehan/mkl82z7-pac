#[doc = "Reader of register CH%s_VEC"]
pub type R = crate::R<u32, super::CH_VEC>;
#[doc = "Reader of field `VECN`"]
pub type VECN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 2:13 - Vector Number"]
    #[inline(always)]
    pub fn vecn(&self) -> VECN_R {
        VECN_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
}
