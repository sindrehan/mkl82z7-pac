#[doc = "Reader of register LTC0_CTX_13"]
pub type R = crate::R<u32, super::LTC0_CTX_13>;
#[doc = "Writer for register LTC0_CTX_13"]
pub type W = crate::W<u32, super::LTC0_CTX_13>;
#[doc = "Register LTC0_CTX_13 `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_CTX_13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTX`"]
pub type CTX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CTX`"]
pub struct CTX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CTX"]
    #[inline(always)]
    pub fn ctx(&self) -> CTX_R {
        CTX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CTX"]
    #[inline(always)]
    pub fn ctx(&mut self) -> CTX_W {
        CTX_W { w: self }
    }
}
