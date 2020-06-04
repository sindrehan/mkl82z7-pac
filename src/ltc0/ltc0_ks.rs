#[doc = "Reader of register LTC0_KS"]
pub type R = crate::R<u32, super::LTC0_KS>;
#[doc = "Writer for register LTC0_KS"]
pub type W = crate::W<u32, super::LTC0_KS>;
#[doc = "Register LTC0_KS `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_KS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KS`"]
pub type KS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KS`"]
pub struct KS_W<'a> {
    w: &'a mut W,
}
impl<'a> KS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Key Size. This is the size of a Key measured in bytes"]
    #[inline(always)]
    pub fn ks(&self) -> KS_R {
        KS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Key Size. This is the size of a Key measured in bytes"]
    #[inline(always)]
    pub fn ks(&mut self) -> KS_W {
        KS_W { w: self }
    }
}
