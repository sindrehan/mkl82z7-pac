#[doc = "Reader of register SFB1AD"]
pub type R = crate::R<u32, super::SFB1AD>;
#[doc = "Writer for register SFB1AD"]
pub type W = crate::W<u32, super::SFB1AD>;
#[doc = "Register SFB1AD `reset()`'s with value 0x6fff_fc00"]
impl crate::ResetValue for super::SFB1AD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6fff_fc00
    }
}
#[doc = "Reader of field `TPADB1`"]
pub type TPADB1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TPADB1`"]
pub struct TPADB1_W<'a> {
    w: &'a mut W,
}
impl<'a> TPADB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Top address for Serial Flash B1.In effect, TPxxAD is the first location of the next memory."]
    #[inline(always)]
    pub fn tpadb1(&self) -> TPADB1_R {
        TPADB1_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Top address for Serial Flash B1.In effect, TPxxAD is the first location of the next memory."]
    #[inline(always)]
    pub fn tpadb1(&mut self) -> TPADB1_W {
        TPADB1_W { w: self }
    }
}
