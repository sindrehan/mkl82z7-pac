#[doc = "Reader of register SFA2AD"]
pub type R = crate::R<u32, super::SFA2AD>;
#[doc = "Writer for register SFA2AD"]
pub type W = crate::W<u32, super::SFA2AD>;
#[doc = "Register SFA2AD `reset()`'s with value 0x6fff_fc00"]
impl crate::ResetValue for super::SFA2AD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6fff_fc00
    }
}
#[doc = "Reader of field `TPADA2`"]
pub type TPADA2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TPADA2`"]
pub struct TPADA2_W<'a> {
    w: &'a mut W,
}
impl<'a> TPADA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Top address for Serial Flash A2. In effect, TPxxAD is the first location of the next memory."]
    #[inline(always)]
    pub fn tpada2(&self) -> TPADA2_R {
        TPADA2_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Top address for Serial Flash A2. In effect, TPxxAD is the first location of the next memory."]
    #[inline(always)]
    pub fn tpada2(&mut self) -> TPADA2_W {
        TPADA2_W { w: self }
    }
}
