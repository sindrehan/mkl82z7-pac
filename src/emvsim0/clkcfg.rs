#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKCFG {
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
#[doc = r" Value of the field"]
pub struct CLK_PRSCR {
    bits: u8,
}
impl CLK_PRSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `GPCNT1_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT1_CLK_SELR {
    #[doc = "Disabled / Reset (default)"]
    _00,
    #[doc = "Card Clock"]
    _01,
    #[doc = "Receive Clock"]
    _10,
    #[doc = "ETU Clock (transmit clock)"]
    _11,
}
impl GPCNT1_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPCNT1_CLK_SELR::_00 => 0,
            GPCNT1_CLK_SELR::_01 => 1,
            GPCNT1_CLK_SELR::_10 => 2,
            GPCNT1_CLK_SELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPCNT1_CLK_SELR {
        match value {
            0 => GPCNT1_CLK_SELR::_00,
            1 => GPCNT1_CLK_SELR::_01,
            2 => GPCNT1_CLK_SELR::_10,
            3 => GPCNT1_CLK_SELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == GPCNT1_CLK_SELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == GPCNT1_CLK_SELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == GPCNT1_CLK_SELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == GPCNT1_CLK_SELR::_11
    }
}
#[doc = "Possible values of the field `GPCNT0_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT0_CLK_SELR {
    #[doc = "Disabled / Reset (default)"]
    _00,
    #[doc = "Card Clock"]
    _01,
    #[doc = "Receive Clock"]
    _10,
    #[doc = "ETU Clock (transmit clock)"]
    _11,
}
impl GPCNT0_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPCNT0_CLK_SELR::_00 => 0,
            GPCNT0_CLK_SELR::_01 => 1,
            GPCNT0_CLK_SELR::_10 => 2,
            GPCNT0_CLK_SELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPCNT0_CLK_SELR {
        match value {
            0 => GPCNT0_CLK_SELR::_00,
            1 => GPCNT0_CLK_SELR::_01,
            2 => GPCNT0_CLK_SELR::_10,
            3 => GPCNT0_CLK_SELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == GPCNT0_CLK_SELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == GPCNT0_CLK_SELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == GPCNT0_CLK_SELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == GPCNT0_CLK_SELR::_11
    }
}
#[doc = r" Proxy"]
pub struct _CLK_PRSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_PRSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPCNT1_CLK_SEL`"]
pub enum GPCNT1_CLK_SELW {
    #[doc = "Disabled / Reset (default)"]
    _00,
    #[doc = "Card Clock"]
    _01,
    #[doc = "Receive Clock"]
    _10,
    #[doc = "ETU Clock (transmit clock)"]
    _11,
}
impl GPCNT1_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPCNT1_CLK_SELW::_00 => 0,
            GPCNT1_CLK_SELW::_01 => 1,
            GPCNT1_CLK_SELW::_10 => 2,
            GPCNT1_CLK_SELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPCNT1_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _GPCNT1_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPCNT1_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled / Reset (default)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SELW::_00)
    }
    #[doc = "Card Clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SELW::_01)
    }
    #[doc = "Receive Clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SELW::_10)
    }
    #[doc = "ETU Clock (transmit clock)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPCNT0_CLK_SEL`"]
pub enum GPCNT0_CLK_SELW {
    #[doc = "Disabled / Reset (default)"]
    _00,
    #[doc = "Card Clock"]
    _01,
    #[doc = "Receive Clock"]
    _10,
    #[doc = "ETU Clock (transmit clock)"]
    _11,
}
impl GPCNT0_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPCNT0_CLK_SELW::_00 => 0,
            GPCNT0_CLK_SELW::_01 => 1,
            GPCNT0_CLK_SELW::_10 => 2,
            GPCNT0_CLK_SELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPCNT0_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _GPCNT0_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPCNT0_CLK_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled / Reset (default)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SELW::_00)
    }
    #[doc = "Card Clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SELW::_01)
    }
    #[doc = "Receive Clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SELW::_10)
    }
    #[doc = "ETU Clock (transmit clock)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:7 - Clock Prescaler Value"]
    #[inline]
    pub fn clk_prsc(&self) -> CLK_PRSCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLK_PRSCR { bits }
    }
    #[doc = "Bits 8:9 - General Purpose Counter 1 Clock Select"]
    #[inline]
    pub fn gpcnt1_clk_sel(&self) -> GPCNT1_CLK_SELR {
        GPCNT1_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - General Purpose Counter 0 Clock Select"]
    #[inline]
    pub fn gpcnt0_clk_sel(&self) -> GPCNT0_CLK_SELR {
        GPCNT0_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:7 - Clock Prescaler Value"]
    #[inline]
    pub fn clk_prsc(&mut self) -> _CLK_PRSCW {
        _CLK_PRSCW { w: self }
    }
    #[doc = "Bits 8:9 - General Purpose Counter 1 Clock Select"]
    #[inline]
    pub fn gpcnt1_clk_sel(&mut self) -> _GPCNT1_CLK_SELW {
        _GPCNT1_CLK_SELW { w: self }
    }
    #[doc = "Bits 10:11 - General Purpose Counter 0 Clock Select"]
    #[inline]
    pub fn gpcnt0_clk_sel(&mut self) -> _GPCNT0_CLK_SELW {
        _GPCNT0_CLK_SELW { w: self }
    }
}
