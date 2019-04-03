#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT5 {
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
#[doc = "Possible values of the field `LPUART0TXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0TXSRCR {
    #[doc = "LPUART0_TX pin"]
    _00,
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPUART0TXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPUART0TXSRCR::_00 => 0,
            LPUART0TXSRCR::_01 => 1,
            LPUART0TXSRCR::_10 => 2,
            LPUART0TXSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPUART0TXSRCR {
        match value {
            0 => LPUART0TXSRCR::_00,
            1 => LPUART0TXSRCR::_01,
            2 => LPUART0TXSRCR::_10,
            i => LPUART0TXSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPUART0TXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPUART0TXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LPUART0TXSRCR::_10
    }
}
#[doc = "Possible values of the field `LPUART0RXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0RXSRCR {
    #[doc = "LPUART0_RX pin"]
    _00,
    #[doc = "CMP0"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPUART0RXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPUART0RXSRCR::_00 => 0,
            LPUART0RXSRCR::_01 => 1,
            LPUART0RXSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPUART0RXSRCR {
        match value {
            0 => LPUART0RXSRCR::_00,
            1 => LPUART0RXSRCR::_01,
            i => LPUART0RXSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPUART0RXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPUART0RXSRCR::_01
    }
}
#[doc = "Possible values of the field `LPUART1TXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1TXSRCR {
    #[doc = "LPUART1_TX pin"]
    _00,
    #[doc = "LPUART1_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "LPUART1_TX pin modulated with TPM2 channel 0 output"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPUART1TXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPUART1TXSRCR::_00 => 0,
            LPUART1TXSRCR::_01 => 1,
            LPUART1TXSRCR::_10 => 2,
            LPUART1TXSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPUART1TXSRCR {
        match value {
            0 => LPUART1TXSRCR::_00,
            1 => LPUART1TXSRCR::_01,
            2 => LPUART1TXSRCR::_10,
            i => LPUART1TXSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPUART1TXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPUART1TXSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LPUART1TXSRCR::_10
    }
}
#[doc = "Possible values of the field `LPUART1RXSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1RXSRCR {
    #[doc = "LPUART1_RX pin"]
    _00,
    #[doc = "CMP0"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPUART1RXSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPUART1RXSRCR::_00 => 0,
            LPUART1RXSRCR::_01 => 1,
            LPUART1RXSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPUART1RXSRCR {
        match value {
            0 => LPUART1RXSRCR::_00,
            1 => LPUART1RXSRCR::_01,
            i => LPUART1RXSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPUART1RXSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPUART1RXSRCR::_01
    }
}
#[doc = "Values that can be written to the field `LPUART0TXSRC`"]
pub enum LPUART0TXSRCW {
    #[doc = "LPUART0_TX pin"]
    _00,
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    _10,
}
impl LPUART0TXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPUART0TXSRCW::_00 => 0,
            LPUART0TXSRCW::_01 => 1,
            LPUART0TXSRCW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART0TXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART0TXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART0TXSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LPUART0_TX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0TXSRCW::_00)
    }
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0TXSRCW::_01)
    }
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART0TXSRCW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART0RXSRC`"]
pub enum LPUART0RXSRCW {
    #[doc = "LPUART0_RX pin"]
    _00,
    #[doc = "CMP0"]
    _01,
}
impl LPUART0RXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPUART0RXSRCW::_00 => 0,
            LPUART0RXSRCW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART0RXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART0RXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART0RXSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LPUART0_RX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0RXSRCW::_00)
    }
    #[doc = "CMP0"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0RXSRCW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART1TXSRC`"]
pub enum LPUART1TXSRCW {
    #[doc = "LPUART1_TX pin"]
    _00,
    #[doc = "LPUART1_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "LPUART1_TX pin modulated with TPM2 channel 0 output"]
    _10,
}
impl LPUART1TXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPUART1TXSRCW::_00 => 0,
            LPUART1TXSRCW::_01 => 1,
            LPUART1TXSRCW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART1TXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART1TXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART1TXSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LPUART1_TX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART1TXSRCW::_00)
    }
    #[doc = "LPUART1_TX pin modulated with TPM1 channel 0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART1TXSRCW::_01)
    }
    #[doc = "LPUART1_TX pin modulated with TPM2 channel 0 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART1TXSRCW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART1RXSRC`"]
pub enum LPUART1RXSRCW {
    #[doc = "LPUART1_RX pin"]
    _00,
    #[doc = "CMP0"]
    _01,
}
impl LPUART1RXSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPUART1RXSRCW::_00 => 0,
            LPUART1RXSRCW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART1RXSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART1RXSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART1RXSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LPUART1_RX pin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART1RXSRCW::_00)
    }
    #[doc = "CMP0"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART1RXSRCW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:17 - LPUART0 transmit data source select"]
    #[inline]
    pub fn lpuart0txsrc(&self) -> LPUART0TXSRCR {
        LPUART0TXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - LPUART 0 receive data source select"]
    #[inline]
    pub fn lpuart0rxsrc(&self) -> LPUART0RXSRCR {
        LPUART0RXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - LPUART1 transmit data source select"]
    #[inline]
    pub fn lpuart1txsrc(&self) -> LPUART1TXSRCR {
        LPUART1TXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - LPUART1 receive data source select"]
    #[inline]
    pub fn lpuart1rxsrc(&self) -> LPUART1RXSRCR {
        LPUART1RXSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:17 - LPUART0 transmit data source select"]
    #[inline]
    pub fn lpuart0txsrc(&mut self) -> _LPUART0TXSRCW {
        _LPUART0TXSRCW { w: self }
    }
    #[doc = "Bits 18:19 - LPUART 0 receive data source select"]
    #[inline]
    pub fn lpuart0rxsrc(&mut self) -> _LPUART0RXSRCW {
        _LPUART0RXSRCW { w: self }
    }
    #[doc = "Bits 20:21 - LPUART1 transmit data source select"]
    #[inline]
    pub fn lpuart1txsrc(&mut self) -> _LPUART1TXSRCW {
        _LPUART1TXSRCW { w: self }
    }
    #[doc = "Bits 22:23 - LPUART1 receive data source select"]
    #[inline]
    pub fn lpuart1rxsrc(&mut self) -> _LPUART1RXSRCW {
        _LPUART1RXSRCW { w: self }
    }
}
