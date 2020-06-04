#[doc = "Reader of register SFA1AD"]
pub type R = crate::R<u32, super::SFA1AD>;
#[doc = "Writer for register SFA1AD"]
pub type W = crate::W<u32, super::SFA1AD>;
#[doc = "Register SFA1AD `reset()`'s with value 0x6fff_fc00"]
impl crate::ResetValue for super::SFA1AD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6fff_fc00
    }
}
#[doc = "Reader of field `TPADA1`"]
pub type TPADA1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TPADA1`"]
pub struct TPADA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TPADA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Top address for Serial Flash A1. In effect, TPADxx is the first location of the next memory."]
    #[inline(always)]
    pub fn tpada1(&self) -> TPADA1_R {
        TPADA1_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Top address for Serial Flash A1. In effect, TPADxx is the first location of the next memory."]
    #[inline(always)]
    pub fn tpada1(&mut self) -> TPADA1_W {
        TPADA1_W { w: self }
    }
}
