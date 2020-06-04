#[doc = "Reader of register BUF1IND"]
pub type R = crate::R<u32, super::BUF1IND>;
#[doc = "Writer for register BUF1IND"]
pub type W = crate::W<u32, super::BUF1IND>;
#[doc = "Register BUF1IND `reset()`'s with value 0"]
impl crate::ResetValue for super::BUF1IND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPINDX1`"]
pub type TPINDX1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TPINDX1`"]
pub struct TPINDX1_W<'a> {
    w: &'a mut W,
}
impl<'a> TPINDX1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Top index of buffer 1."]
    #[inline(always)]
    pub fn tpindx1(&self) -> TPINDX1_R {
        TPINDX1_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Top index of buffer 1."]
    #[inline(always)]
    pub fn tpindx1(&mut self) -> TPINDX1_W {
        TPINDX1_W { w: self }
    }
}
