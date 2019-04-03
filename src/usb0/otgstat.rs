#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::OTGSTAT {
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
#[doc = "Possible values of the field `LINESTATESTABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATESTABLER {
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    _0,
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    _1,
}
impl LINESTATESTABLER {
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
            LINESTATESTABLER::_0 => false,
            LINESTATESTABLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINESTATESTABLER {
        match value {
            false => LINESTATESTABLER::_0,
            true => LINESTATESTABLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LINESTATESTABLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LINESTATESTABLER::_1
    }
}
#[doc = r" Value of the field"]
pub struct ONEMSECENR {
    bits: bool,
}
impl ONEMSECENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `LINESTATESTABLE`"]
pub enum LINESTATESTABLEW {
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    _0,
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    _1,
}
impl LINESTATESTABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINESTATESTABLEW::_0 => false,
            LINESTATESTABLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINESTATESTABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _LINESTATESTABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINESTATESTABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATESTABLEW::_0)
    }
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATESTABLEW::_1)
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
#[doc = r" Proxy"]
pub struct _ONEMSECENW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEMSECENW<'a> {
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
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
    #[inline]
    pub fn linestatestable(&self) -> LINESTATESTABLER {
        LINESTATESTABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline]
    pub fn onemsecen(&self) -> ONEMSECENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        ONEMSECENR { bits }
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
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
    #[inline]
    pub fn linestatestable(&mut self) -> _LINESTATESTABLEW {
        _LINESTATESTABLEW { w: self }
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline]
    pub fn onemsecen(&mut self) -> _ONEMSECENW {
        _ONEMSECENW { w: self }
    }
}
