#[doc = "Reader of register SFB2AD"]
pub type R = crate::R<u32, super::SFB2AD>;
#[doc = "Writer for register SFB2AD"]
pub type W = crate::W<u32, super::SFB2AD>;
#[doc = "Register SFB2AD `reset()`'s with value 0x6fff_fc00"]
impl crate::ResetValue for super::SFB2AD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6fff_fc00
    }
}
#[doc = "Reader of field `TPADB2`"]
pub type TPADB2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TPADB2`"]
pub struct TPADB2_W<'a> {
    w: &'a mut W,
}
impl<'a> TPADB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Top address for Serial Flash B2. In effect, TPxxAD is the first location of the next memory."]
    #[inline(always)]
    pub fn tpadb2(&self) -> TPADB2_R {
        TPADB2_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Top address for Serial Flash B2. In effect, TPxxAD is the first location of the next memory."]
    #[inline(always)]
    pub fn tpadb2(&mut self) -> TPADB2_W {
        TPADB2_W { w: self }
    }
}
