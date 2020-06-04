#[doc = "Reader of register USBCTRL"]
pub type R = crate::R<u8, super::USBCTRL>;
#[doc = "Writer for register USBCTRL"]
pub type W = crate::W<u8, super::USBCTRL>;
#[doc = "Register USBCTRL `reset()`'s with value 0xc0"]
impl crate::ResetValue for super::USBCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc0
    }
}
#[doc = "Selects USB signals to be used as UART signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTSEL_A {
    #[doc = "0: USB signals not used as UART signals."]
    _0 = 0,
    #[doc = "1: USB signals used as UART signals."]
    _1 = 1,
}
impl From<UARTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: UARTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UARTSEL`"]
pub type UARTSEL_R = crate::R<bool, UARTSEL_A>;
impl UARTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTSEL_A {
        match self.bits {
            false => UARTSEL_A::_0,
            true => UARTSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UARTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UARTSEL_A::_1
    }
}
#[doc = "Write proxy for field `UARTSEL`"]
pub struct UARTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB signals not used as UART signals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTSEL_A::_0)
    }
    #[doc = "USB signals used as UART signals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTSEL_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "UART Signal Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTCHLS_A {
    #[doc = "0: USB DP/DM signals used as UART TX/RX."]
    _0 = 0,
    #[doc = "1: USB DP/DM signals used as UART RX/TX."]
    _1 = 1,
}
impl From<UARTCHLS_A> for bool {
    #[inline(always)]
    fn from(variant: UARTCHLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UARTCHLS`"]
pub type UARTCHLS_R = crate::R<bool, UARTCHLS_A>;
impl UARTCHLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTCHLS_A {
        match self.bits {
            false => UARTCHLS_A::_0,
            true => UARTCHLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UARTCHLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UARTCHLS_A::_1
    }
}
#[doc = "Write proxy for field `UARTCHLS`"]
pub struct UARTCHLS_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTCHLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTCHLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB DP/DM signals used as UART TX/RX."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTCHLS_A::_0)
    }
    #[doc = "USB DP/DM signals used as UART RX/TX."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTCHLS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Enables the weak pulldowns on the USB transceiver.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDE_A {
    #[doc = "0: Weak pulldowns are disabled on D+ and D-."]
    _0 = 0,
    #[doc = "1: Weak pulldowns are enabled on D+ and D-."]
    _1 = 1,
}
impl From<PDE_A> for bool {
    #[inline(always)]
    fn from(variant: PDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDE`"]
pub type PDE_R = crate::R<bool, PDE_A>;
impl PDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDE_A {
        match self.bits {
            false => PDE_A::_0,
            true => PDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDE_A::_1
    }
}
#[doc = "Write proxy for field `PDE`"]
pub struct PDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Weak pulldowns are disabled on D+ and D-."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDE_A::_0)
    }
    #[doc = "Weak pulldowns are enabled on D+ and D-."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Places the USB transceiver into the suspend state.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSP_A {
    #[doc = "0: USB transceiver is not in suspend state."]
    _0 = 0,
    #[doc = "1: USB transceiver is in suspend state."]
    _1 = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, SUSP_A>;
impl SUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::_0,
            true => SUSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUSP_A::_1
    }
}
#[doc = "Write proxy for field `SUSP`"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB transceiver is not in suspend state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUSP_A::_0)
    }
    #[doc = "USB transceiver is in suspend state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUSP_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Selects USB signals to be used as UART signals."]
    #[inline(always)]
    pub fn uartsel(&self) -> UARTSEL_R {
        UARTSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART Signal Channel Select"]
    #[inline(always)]
    pub fn uartchls(&self) -> UARTCHLS_R {
        UARTCHLS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the weak pulldowns on the USB transceiver."]
    #[inline(always)]
    pub fn pde(&self) -> PDE_R {
        PDE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Places the USB transceiver into the suspend state."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Selects USB signals to be used as UART signals."]
    #[inline(always)]
    pub fn uartsel(&mut self) -> UARTSEL_W {
        UARTSEL_W { w: self }
    }
    #[doc = "Bit 5 - UART Signal Channel Select"]
    #[inline(always)]
    pub fn uartchls(&mut self) -> UARTCHLS_W {
        UARTCHLS_W { w: self }
    }
    #[doc = "Bit 6 - Enables the weak pulldowns on the USB transceiver."]
    #[inline(always)]
    pub fn pde(&mut self) -> PDE_W {
        PDE_W { w: self }
    }
    #[doc = "Bit 7 - Places the USB transceiver into the suspend state."]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
}
