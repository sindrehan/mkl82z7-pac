#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::USBCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `UARTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTSELR {
    #[doc = "USB signals not used as UART signals."]
    _0,
    #[doc = "USB signals used as UART signals."]
    _1,
}
impl UARTSELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UARTSELR::_0 => false,
            UARTSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UARTSELR {
        match value {
            false => UARTSELR::_0,
            true => UARTSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UARTSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UARTSELR::_1
    }
}
#[doc = "Possible values of the field `UARTCHLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTCHLSR {
    #[doc = "USB DP/DM signals used as UART TX/RX."]
    _0,
    #[doc = "USB DP/DM signals used as UART RX/TX."]
    _1,
}
impl UARTCHLSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UARTCHLSR::_0 => false,
            UARTCHLSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UARTCHLSR {
        match value {
            false => UARTCHLSR::_0,
            true => UARTCHLSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UARTCHLSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UARTCHLSR::_1
    }
}
#[doc = "Possible values of the field `PDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDER {
    #[doc = "Weak pulldowns are disabled on D+ and D-."]
    _0,
    #[doc = "Weak pulldowns are enabled on D+ and D-."]
    _1,
}
impl PDER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PDER::_0 => false,
            PDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDER {
        match value {
            false => PDER::_0,
            true => PDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDER::_1
    }
}
#[doc = "Possible values of the field `SUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPR {
    #[doc = "USB transceiver is not in suspend state."]
    _0,
    #[doc = "USB transceiver is in suspend state."]
    _1,
}
impl SUSPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SUSPR::_0 => false,
            SUSPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPR {
        match value {
            false => SUSPR::_0,
            true => SUSPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SUSPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SUSPR::_1
    }
}
#[doc = "Values that can be written to the field `UARTSEL`"]
pub enum UARTSELW {
    #[doc = "USB signals not used as UART signals."]
    _0,
    #[doc = "USB signals used as UART signals."]
    _1,
}
impl UARTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UARTSELW::_0 => false,
            UARTSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UARTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _UARTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UARTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB signals not used as UART signals."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTSELW::_0)
    }
    #[doc = "USB signals used as UART signals."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTSELW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UARTCHLS`"]
pub enum UARTCHLSW {
    #[doc = "USB DP/DM signals used as UART TX/RX."]
    _0,
    #[doc = "USB DP/DM signals used as UART RX/TX."]
    _1,
}
impl UARTCHLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UARTCHLSW::_0 => false,
            UARTCHLSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UARTCHLSW<'a> {
    w: &'a mut W,
}
impl<'a> _UARTCHLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UARTCHLSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB DP/DM signals used as UART TX/RX."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTCHLSW::_0)
    }
    #[doc = "USB DP/DM signals used as UART RX/TX."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTCHLSW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDE`"]
pub enum PDEW {
    #[doc = "Weak pulldowns are disabled on D+ and D-."]
    _0,
    #[doc = "Weak pulldowns are enabled on D+ and D-."]
    _1,
}
impl PDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDEW::_0 => false,
            PDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Weak pulldowns are disabled on D+ and D-."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDEW::_0)
    }
    #[doc = "Weak pulldowns are enabled on D+ and D-."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDEW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUSP`"]
pub enum SUSPW {
    #[doc = "USB transceiver is not in suspend state."]
    _0,
    #[doc = "USB transceiver is in suspend state."]
    _1,
}
impl SUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSPW::_0 => false,
            SUSPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB transceiver is not in suspend state."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUSPW::_0)
    }
    #[doc = "USB transceiver is in suspend state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUSPW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 4 - Selects USB signals to be used as UART signals."]
    #[inline]
    pub fn uartsel(&self) -> UARTSELR {
        UARTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - UART Signal Channel Select"]
    #[inline]
    pub fn uartchls(&self) -> UARTCHLSR {
        UARTCHLSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Enables the weak pulldowns on the USB transceiver."]
    #[inline]
    pub fn pde(&self) -> PDER {
        PDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Places the USB transceiver into the suspend state."]
    #[inline]
    pub fn susp(&self) -> SUSPR {
        SUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Selects USB signals to be used as UART signals."]
    #[inline]
    pub fn uartsel(&mut self) -> _UARTSELW {
        _UARTSELW { w: self }
    }
    #[doc = "Bit 5 - UART Signal Channel Select"]
    #[inline]
    pub fn uartchls(&mut self) -> _UARTCHLSW {
        _UARTCHLSW { w: self }
    }
    #[doc = "Bit 6 - Enables the weak pulldowns on the USB transceiver."]
    #[inline]
    pub fn pde(&mut self) -> _PDEW {
        _PDEW { w: self }
    }
    #[doc = "Bit 7 - Places the USB transceiver into the suspend state."]
    #[inline]
    pub fn susp(&mut self) -> _SUSPW {
        _SUSPW { w: self }
    }
}
