#[doc = "Reader of register TX_THD"]
pub type R = crate::R<u32, super::TX_THD>;
#[doc = "Writer for register TX_THD"]
pub type W = crate::W<u32, super::TX_THD>;
#[doc = "Register TX_THD `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::TX_THD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TDT`"]
pub type TDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDT`"]
pub struct TDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Transmitter NACK Threshold Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TNCK_THD_A {
    #[doc = "0: TNTE will never be set; retransmission after NACK reception is disabled."]
    _0 = 0,
    #[doc = "1: TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
    _1 = 1,
}
impl From<TNCK_THD_A> for u8 {
    #[inline(always)]
    fn from(variant: TNCK_THD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TNCK_THD`"]
pub type TNCK_THD_R = crate::R<u8, TNCK_THD_A>;
impl TNCK_THD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TNCK_THD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TNCK_THD_A::_0),
            1 => Val(TNCK_THD_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TNCK_THD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TNCK_THD_A::_1
    }
}
#[doc = "Write proxy for field `TNCK_THD`"]
pub struct TNCK_THD_W<'a> {
    w: &'a mut W,
}
impl<'a> TNCK_THD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TNCK_THD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TNTE will never be set; retransmission after NACK reception is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TNCK_THD_A::_0)
    }
    #[doc = "TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TNCK_THD_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmitter Data Threshold Value"]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transmitter NACK Threshold Value"]
    #[inline(always)]
    pub fn tnck_thd(&self) -> TNCK_THD_R {
        TNCK_THD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Data Threshold Value"]
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W {
        TDT_W { w: self }
    }
    #[doc = "Bits 8:11 - Transmitter NACK Threshold Value"]
    #[inline(always)]
    pub fn tnck_thd(&mut self) -> TNCK_THD_W {
        TNCK_THD_W { w: self }
    }
}
