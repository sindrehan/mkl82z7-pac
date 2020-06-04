#[doc = "Reader of register SECKEY0"]
pub type R = crate::R<u32, super::SECKEY0>;
#[doc = "Reader of field `SECKEY`"]
pub type SECKEY_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Key 31:0"]
    #[inline(always)]
    pub fn seckey(&self) -> SECKEY_R {
        SECKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
