#[doc = "Reader of register BUF0IND"]
pub type R = crate::R<u32, super::BUF0IND>;
#[doc = "Writer for register BUF0IND"]
pub type W = crate::W<u32, super::BUF0IND>;
#[doc = "Register BUF0IND `reset()`'s with value 0"]
impl crate::ResetValue for super::BUF0IND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPINDX0`"]
pub type TPINDX0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TPINDX0`"]
pub struct TPINDX0_W<'a> {
    w: &'a mut W,
}
impl<'a> TPINDX0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Top index of buffer 0."]
    #[inline(always)]
    pub fn tpindx0(&self) -> TPINDX0_R {
        TPINDX0_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Top index of buffer 0."]
    #[inline(always)]
    pub fn tpindx0(&mut self) -> TPINDX0_W {
        TPINDX0_W { w: self }
    }
}
