#[doc = "Reader of register SOPT5"]
pub type R = crate::R<u32, super::SOPT5>;
#[doc = "Writer for register SOPT5"]
pub type W = crate::W<u32, super::SOPT5>;
#[doc = "Register SOPT5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPUART0 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART0TXSRC_A {
    #[doc = "0: LPUART0_TX pin"]
    _00 = 0,
    #[doc = "1: LPUART0_TX pin modulated with TPM1 channel 0 output"]
    _01 = 1,
    #[doc = "2: LPUART0_TX pin modulated with TPM2 channel 0 output"]
    _10 = 2,
}
impl From<LPUART0TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART0TXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPUART0TXSRC`"]
pub type LPUART0TXSRC_R = crate::R<u8, LPUART0TXSRC_A>;
impl LPUART0TXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPUART0TXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPUART0TXSRC_A::_00),
            1 => Val(LPUART0TXSRC_A::_01),
            2 => Val(LPUART0TXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPUART0TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPUART0TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPUART0TXSRC_A::_10
    }
}
#[doc = "Write proxy for field `LPUART0TXSRC`"]
pub struct LPUART0TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART0TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART0TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART0_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_00)
    }
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_01)
    }
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "LPUART 0 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART0RXSRC_A {
    #[doc = "0: LPUART0_RX pin"]
    _00 = 0,
    #[doc = "1: CMP0"]
    _01 = 1,
}
impl From<LPUART0RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART0RXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPUART0RXSRC`"]
pub type LPUART0RXSRC_R = crate::R<u8, LPUART0RXSRC_A>;
impl LPUART0RXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPUART0RXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPUART0RXSRC_A::_00),
            1 => Val(LPUART0RXSRC_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPUART0RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPUART0RXSRC_A::_01
    }
}
#[doc = "Write proxy for field `LPUART0RXSRC`"]
pub struct LPUART0RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART0RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART0RXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART0_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0RXSRC_A::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0RXSRC_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "LPUART1 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART1TXSRC_A {
    #[doc = "0: LPUART1_TX pin"]
    _00 = 0,
    #[doc = "1: LPUART1_TX pin modulated with TPM1 channel 0 output"]
    _01 = 1,
    #[doc = "2: LPUART1_TX pin modulated with TPM2 channel 0 output"]
    _10 = 2,
}
impl From<LPUART1TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1TXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPUART1TXSRC`"]
pub type LPUART1TXSRC_R = crate::R<u8, LPUART1TXSRC_A>;
impl LPUART1TXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPUART1TXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPUART1TXSRC_A::_00),
            1 => Val(LPUART1TXSRC_A::_01),
            2 => Val(LPUART1TXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPUART1TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPUART1TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPUART1TXSRC_A::_10
    }
}
#[doc = "Write proxy for field `LPUART1TXSRC`"]
pub struct LPUART1TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART1_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART1TXSRC_A::_00)
    }
    #[doc = "LPUART1_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART1TXSRC_A::_01)
    }
    #[doc = "LPUART1_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART1TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "LPUART1 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART1RXSRC_A {
    #[doc = "0: LPUART1_RX pin"]
    _00 = 0,
    #[doc = "1: CMP0"]
    _01 = 1,
}
impl From<LPUART1RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1RXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPUART1RXSRC`"]
pub type LPUART1RXSRC_R = crate::R<u8, LPUART1RXSRC_A>;
impl LPUART1RXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPUART1RXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPUART1RXSRC_A::_00),
            1 => Val(LPUART1RXSRC_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPUART1RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPUART1RXSRC_A::_01
    }
}
#[doc = "Write proxy for field `LPUART1RXSRC`"]
pub struct LPUART1RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1RXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART1_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART1RXSRC_A::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART1RXSRC_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - LPUART0 transmit data source select"]
    #[inline(always)]
    pub fn lpuart0txsrc(&self) -> LPUART0TXSRC_R {
        LPUART0TXSRC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - LPUART 0 receive data source select"]
    #[inline(always)]
    pub fn lpuart0rxsrc(&self) -> LPUART0RXSRC_R {
        LPUART0RXSRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - LPUART1 transmit data source select"]
    #[inline(always)]
    pub fn lpuart1txsrc(&self) -> LPUART1TXSRC_R {
        LPUART1TXSRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - LPUART1 receive data source select"]
    #[inline(always)]
    pub fn lpuart1rxsrc(&self) -> LPUART1RXSRC_R {
        LPUART1RXSRC_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - LPUART0 transmit data source select"]
    #[inline(always)]
    pub fn lpuart0txsrc(&mut self) -> LPUART0TXSRC_W {
        LPUART0TXSRC_W { w: self }
    }
    #[doc = "Bits 18:19 - LPUART 0 receive data source select"]
    #[inline(always)]
    pub fn lpuart0rxsrc(&mut self) -> LPUART0RXSRC_W {
        LPUART0RXSRC_W { w: self }
    }
    #[doc = "Bits 20:21 - LPUART1 transmit data source select"]
    #[inline(always)]
    pub fn lpuart1txsrc(&mut self) -> LPUART1TXSRC_W {
        LPUART1TXSRC_W { w: self }
    }
    #[doc = "Bits 22:23 - LPUART1 receive data source select"]
    #[inline(always)]
    pub fn lpuart1rxsrc(&mut self) -> LPUART1RXSRC_W {
        LPUART1RXSRC_W { w: self }
    }
}
