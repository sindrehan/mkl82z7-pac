#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT1 {
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
#[doc = "Possible values of the field `RAMSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMSIZER {
    #[doc = "8 KB"]
    _0001,
    #[doc = "16 KB"]
    _0011,
    #[doc = "24 KB"]
    _0100,
    #[doc = "32 KB"]
    _0101,
    #[doc = "48 KB"]
    _0110,
    #[doc = "64 KB"]
    _0111,
    #[doc = "96 KB"]
    _1000,
    #[doc = "128 KB"]
    _1001,
    #[doc = "256 KB"]
    _1011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMSIZER::_0001 => 1,
            RAMSIZER::_0011 => 3,
            RAMSIZER::_0100 => 4,
            RAMSIZER::_0101 => 5,
            RAMSIZER::_0110 => 6,
            RAMSIZER::_0111 => 7,
            RAMSIZER::_1000 => 8,
            RAMSIZER::_1001 => 9,
            RAMSIZER::_1011 => 11,
            RAMSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMSIZER {
        match value {
            1 => RAMSIZER::_0001,
            3 => RAMSIZER::_0011,
            4 => RAMSIZER::_0100,
            5 => RAMSIZER::_0101,
            6 => RAMSIZER::_0110,
            7 => RAMSIZER::_0111,
            8 => RAMSIZER::_1000,
            9 => RAMSIZER::_1001,
            11 => RAMSIZER::_1011,
            i => RAMSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == RAMSIZER::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == RAMSIZER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == RAMSIZER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == RAMSIZER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == RAMSIZER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == RAMSIZER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == RAMSIZER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == RAMSIZER::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == RAMSIZER::_1011
    }
}
#[doc = "Possible values of the field `OSC32KSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC32KSELR {
    #[doc = "System oscillator (OSC32KCLK)"]
    _00,
    #[doc = "RTC oscillator"]
    _10,
    #[doc = "LPO 1 kHz"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSC32KSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSC32KSELR::_00 => 0,
            OSC32KSELR::_10 => 2,
            OSC32KSELR::_11 => 3,
            OSC32KSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSC32KSELR {
        match value {
            0 => OSC32KSELR::_00,
            2 => OSC32KSELR::_10,
            3 => OSC32KSELR::_11,
            i => OSC32KSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == OSC32KSELR::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == OSC32KSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == OSC32KSELR::_11
    }
}
#[doc = "Values that can be written to the field `OSC32KSEL`"]
pub enum OSC32KSELW {
    #[doc = "System oscillator (OSC32KCLK)"]
    _00,
    #[doc = "RTC oscillator"]
    _10,
    #[doc = "LPO 1 kHz"]
    _11,
}
impl OSC32KSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSC32KSELW::_00 => 0,
            OSC32KSELW::_10 => 2,
            OSC32KSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSC32KSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC32KSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC32KSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "System oscillator (OSC32KCLK)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KSELW::_00)
    }
    #[doc = "RTC oscillator"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC32KSELW::_10)
    }
    #[doc = "LPO 1 kHz"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC32KSELW::_11)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 12:15 - System RAM Size"]
    #[inline]
    pub fn ramsize(&self) -> RAMSIZER {
        RAMSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - 32K Oscillator Clock Select"]
    #[inline]
    pub fn osc32ksel(&self) -> OSC32KSELR {
        OSC32KSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
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
    #[doc = "Bits 18:19 - 32K Oscillator Clock Select"]
    #[inline]
    pub fn osc32ksel(&mut self) -> _OSC32KSELW {
        _OSC32KSELW { w: self }
    }
}
