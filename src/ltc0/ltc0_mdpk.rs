#[doc = "Reader of register LTC0_MDPK"]
pub type R = crate::R<u32, super::LTC0_MDPK>;
#[doc = "Writer for register LTC0_MDPK"]
pub type W = crate::W<u32, super::LTC0_MDPK>;
#[doc = "Register LTC0_MDPK `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_MDPK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKHA_MODE_LS`"]
pub type PKHA_MODE_LS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKHA_MODE_LS`"]
pub struct PKHA_MODE_LS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_MODE_LS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PKHA_MODE_MS`"]
pub type PKHA_MODE_MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PKHA_MODE_MS`"]
pub struct PKHA_MODE_MS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_MODE_MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Algorithm. This field specifies which algorithm is being selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALG_A {
    #[doc = "8: PKHA"]
    _1000 = 8,
}
impl From<ALG_A> for u8 {
    #[inline(always)]
    fn from(variant: ALG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALG`"]
pub type ALG_R = crate::R<u8, ALG_A>;
impl ALG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALG_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(ALG_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ALG_A::_1000
    }
}
#[doc = "Write proxy for field `ALG`"]
pub struct ALG_W<'a> {
    w: &'a mut W,
}
impl<'a> ALG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PKHA"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ALG_A::_1000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PKHA_MODE least significant 12 bits"]
    #[inline(always)]
    pub fn pkha_mode_ls(&self) -> PKHA_MODE_LS_R {
        PKHA_MODE_LS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - PKHA_MODE most-significant 4 bits"]
    #[inline(always)]
    pub fn pkha_mode_ms(&self) -> PKHA_MODE_MS_R {
        PKHA_MODE_MS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline(always)]
    pub fn alg(&self) -> ALG_R {
        ALG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - PKHA_MODE least significant 12 bits"]
    #[inline(always)]
    pub fn pkha_mode_ls(&mut self) -> PKHA_MODE_LS_W {
        PKHA_MODE_LS_W { w: self }
    }
    #[doc = "Bits 16:19 - PKHA_MODE most-significant 4 bits"]
    #[inline(always)]
    pub fn pkha_mode_ms(&mut self) -> PKHA_MODE_MS_W {
        PKHA_MODE_MS_W { w: self }
    }
    #[doc = "Bits 20:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline(always)]
    pub fn alg(&mut self) -> ALG_W {
        ALG_W { w: self }
    }
}
