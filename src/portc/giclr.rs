#[doc = "Writer for register GICLR"]
pub type W = crate::W<u32, super::GICLR>;
#[doc = "Register GICLR `reset()`'s with value 0"]
impl crate::ResetValue for super::GICLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Global Interrupt Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GIWE_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GIWE_AW> for u16 {
    #[inline(always)]
    fn from(variant: GIWE_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `GIWE`"]
pub struct GIWE_W<'a> {
    w: &'a mut W,
}
impl<'a> GIWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIWE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GIWE_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GIWE_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Write proxy for field `GIWD`"]
pub struct GIWD_W<'a> {
    w: &'a mut W,
}
impl<'a> GIWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Global Interrupt Write Enable"]
    #[inline(always)]
    pub fn giwe(&mut self) -> GIWE_W {
        GIWE_W { w: self }
    }
    #[doc = "Bits 16:31 - Global Interrupt Write Data"]
    #[inline(always)]
    pub fn giwd(&mut self) -> GIWD_W {
        GIWD_W { w: self }
    }
}
