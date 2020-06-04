#[doc = "Reader of register BUF2IND"]
pub type R = crate::R<u32, super::BUF2IND>;
#[doc = "Writer for register BUF2IND"]
pub type W = crate::W<u32, super::BUF2IND>;
#[doc = "Register BUF2IND `reset()`'s with value 0"]
impl crate::ResetValue for super::BUF2IND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPINDX2`"]
pub type TPINDX2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TPINDX2`"]
pub struct TPINDX2_W<'a> {
    w: &'a mut W,
}
impl<'a> TPINDX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Top index of buffer 2."]
    #[inline(always)]
    pub fn tpindx2(&self) -> TPINDX2_R {
        TPINDX2_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Top index of buffer 2."]
    #[inline(always)]
    pub fn tpindx2(&mut self) -> TPINDX2_W {
        TPINDX2_W { w: self }
    }
}
