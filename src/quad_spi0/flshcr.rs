#[doc = "Reader of register FLSHCR"]
pub type R = crate::R<u32, super::FLSHCR>;
#[doc = "Writer for register FLSHCR"]
pub type W = crate::W<u32, super::FLSHCR>;
#[doc = "Register FLSHCR `reset()`'s with value 0x0303"]
impl crate::ResetValue for super::FLSHCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0303
    }
}
#[doc = "Reader of field `TCSS`"]
pub type TCSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCSS`"]
pub struct TCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TCSH`"]
pub type TCSH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCSH`"]
pub struct TCSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Serial flash data in hold time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TDH_A {
    #[doc = "0: Data aligned with the posedge of Internal reference clock of QuadSPI"]
    _00 = 0,
    #[doc = "1: Data aligned with 2x serial flash half clock"]
    _01 = 1,
    #[doc = "2: Data aligned with 4x serial flash half clock"]
    _10 = 2,
}
impl From<TDH_A> for u8 {
    #[inline(always)]
    fn from(variant: TDH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TDH`"]
pub type TDH_R = crate::R<u8, TDH_A>;
impl TDH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TDH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TDH_A::_00),
            1 => Val(TDH_A::_01),
            2 => Val(TDH_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TDH_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TDH_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TDH_A::_10
    }
}
#[doc = "Write proxy for field `TDH`"]
pub struct TDH_W<'a> {
    w: &'a mut W,
}
impl<'a> TDH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data aligned with the posedge of Internal reference clock of QuadSPI"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TDH_A::_00)
    }
    #[doc = "Data aligned with 2x serial flash half clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TDH_A::_01)
    }
    #[doc = "Data aligned with 4x serial flash half clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TDH_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Serial flash CS setup time in terms of serial flash clock cycles"]
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Serial flash CS hold time in terms of serial flash clock cycles"]
    #[inline(always)]
    pub fn tcsh(&self) -> TCSH_R {
        TCSH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Serial flash data in hold time"]
    #[inline(always)]
    pub fn tdh(&self) -> TDH_R {
        TDH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial flash CS setup time in terms of serial flash clock cycles"]
    #[inline(always)]
    pub fn tcss(&mut self) -> TCSS_W {
        TCSS_W { w: self }
    }
    #[doc = "Bits 8:11 - Serial flash CS hold time in terms of serial flash clock cycles"]
    #[inline(always)]
    pub fn tcsh(&mut self) -> TCSH_W {
        TCSH_W { w: self }
    }
    #[doc = "Bits 16:17 - Serial flash data in hold time"]
    #[inline(always)]
    pub fn tdh(&mut self) -> TDH_W {
        TDH_W { w: self }
    }
}
