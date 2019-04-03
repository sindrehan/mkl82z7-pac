#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::OTGICR {
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
#[doc = "Possible values of the field `LINESTATEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATEENR {
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    _0,
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    _1,
}
impl LINESTATEENR {
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
            LINESTATEENR::_0 => false,
            LINESTATEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINESTATEENR {
        match value {
            false => LINESTATEENR::_0,
            true => LINESTATEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LINESTATEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LINESTATEENR::_1
    }
}
#[doc = "Possible values of the field `ONEMSECEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEMSECENR {
    #[doc = "Diables the 1ms timer interrupt."]
    _0,
    #[doc = "Enables the 1ms timer interrupt."]
    _1,
}
impl ONEMSECENR {
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
            ONEMSECENR::_0 => false,
            ONEMSECENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONEMSECENR {
        match value {
            false => ONEMSECENR::_0,
            true => ONEMSECENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ONEMSECENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ONEMSECENR::_1
    }
}
#[doc = "Values that can be written to the field `LINESTATEEN`"]
pub enum LINESTATEENW {
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    _0,
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    _1,
}
impl LINESTATEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINESTATEENW::_0 => false,
            LINESTATEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINESTATEENW<'a> {
    w: &'a mut W,
}
impl<'a> _LINESTATEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINESTATEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATEENW::_0)
    }
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATEENW::_1)
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
#[doc = "Values that can be written to the field `ONEMSECEN`"]
pub enum ONEMSECENW {
    #[doc = "Diables the 1ms timer interrupt."]
    _0,
    #[doc = "Enables the 1ms timer interrupt."]
    _1,
}
impl ONEMSECENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONEMSECENW::_0 => false,
            ONEMSECENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONEMSECENW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEMSECENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONEMSECENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Diables the 1ms timer interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONEMSECENW::_0)
    }
    #[doc = "Enables the 1ms timer interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONEMSECENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline]
    pub fn linestateen(&self) -> LINESTATEENR {
        LINESTATEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline]
    pub fn onemsecen(&self) -> ONEMSECENR {
        ONEMSECENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline]
    pub fn linestateen(&mut self) -> _LINESTATEENW {
        _LINESTATEENW { w: self }
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline]
    pub fn onemsecen(&mut self) -> _ONEMSECENW {
        _ONEMSECENW { w: self }
    }
}
