#[doc = "Reader of register SFAR"]
pub type R = crate::R<u32, super::SFAR>;
#[doc = "Writer for register SFAR"]
pub type W = crate::W<u32, super::SFAR>;
#[doc = "Register SFAR `reset()`'s with value 0"]
impl crate::ResetValue for super::SFAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SFADR`"]
pub type SFADR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SFADR`"]
pub struct SFADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SFADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Serial Flash Address. The register content is used as byte address for all following IP Commands."]
    #[inline(always)]
    pub fn sfadr(&self) -> SFADR_R {
        SFADR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Serial Flash Address. The register content is used as byte address for all following IP Commands."]
    #[inline(always)]
    pub fn sfadr(&mut self) -> SFADR_W {
        SFADR_W { w: self }
    }
}
