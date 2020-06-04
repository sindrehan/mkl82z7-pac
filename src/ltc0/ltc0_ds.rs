#[doc = "Reader of register LTC0_DS"]
pub type R = crate::R<u32, super::LTC0_DS>;
#[doc = "Writer for register LTC0_DS"]
pub type W = crate::W<u32, super::LTC0_DS>;
#[doc = "Register LTC0_DS `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_DS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Data Size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data Size"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
}
