#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISCCTRL {
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
#[doc = "Possible values of the field `DMAINTSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTSEL0R {
    #[doc = "DMA0 channel 4 is not available in vector 16."]
    _0,
    #[doc = "DMA0 channel 4 is available in vector 16."]
    _1,
}
impl DMAINTSEL0R {
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
            DMAINTSEL0R::_0 => false,
            DMAINTSEL0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAINTSEL0R {
        match value {
            false => DMAINTSEL0R::_0,
            true => DMAINTSEL0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAINTSEL0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAINTSEL0R::_1
    }
}
#[doc = "Possible values of the field `DMAINTSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTSEL1R {
    #[doc = "DMA0 channel 5 is not available in vector 17."]
    _0,
    #[doc = "DMA0 channel 5 is available in vector 17."]
    _1,
}
impl DMAINTSEL1R {
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
            DMAINTSEL1R::_0 => false,
            DMAINTSEL1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAINTSEL1R {
        match value {
            false => DMAINTSEL1R::_0,
            true => DMAINTSEL1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAINTSEL1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAINTSEL1R::_1
    }
}
#[doc = "Possible values of the field `DMAINTSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTSEL2R {
    #[doc = "DMA0 channel 6 is not available in vector 18."]
    _0,
    #[doc = "DMA0 channel 6 is available in vector 18."]
    _1,
}
impl DMAINTSEL2R {
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
            DMAINTSEL2R::_0 => false,
            DMAINTSEL2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAINTSEL2R {
        match value {
            false => DMAINTSEL2R::_0,
            true => DMAINTSEL2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAINTSEL2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAINTSEL2R::_1
    }
}
#[doc = "Possible values of the field `DMAINTSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTSEL3R {
    #[doc = "DMA0 channel 7 is not available in vector 19."]
    _0,
    #[doc = "DMA0 channel 7 is available in vector 19."]
    _1,
}
impl DMAINTSEL3R {
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
            DMAINTSEL3R::_0 => false,
            DMAINTSEL3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAINTSEL3R {
        match value {
            false => DMAINTSEL3R::_0,
            true => DMAINTSEL3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAINTSEL3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAINTSEL3R::_1
    }
}
#[doc = "Possible values of the field `LTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTCENR {
    #[doc = "LTC is not available."]
    _0,
    #[doc = "LTC is available."]
    _1,
}
impl LTCENR {
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
            LTCENR::_0 => false,
            LTCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTCENR {
        match value {
            false => LTCENR::_0,
            true => LTCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LTCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LTCENR::_1
    }
}
#[doc = "Values that can be written to the field `DMAINTSEL0`"]
pub enum DMAINTSEL0W {
    #[doc = "DMA0 channel 4 is not available in vector 16."]
    _0,
    #[doc = "DMA0 channel 4 is available in vector 16."]
    _1,
}
impl DMAINTSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAINTSEL0W::_0 => false,
            DMAINTSEL0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAINTSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAINTSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAINTSEL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA0 channel 4 is not available in vector 16."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAINTSEL0W::_0)
    }
    #[doc = "DMA0 channel 4 is available in vector 16."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAINTSEL0W::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAINTSEL1`"]
pub enum DMAINTSEL1W {
    #[doc = "DMA0 channel 5 is not available in vector 17."]
    _0,
    #[doc = "DMA0 channel 5 is available in vector 17."]
    _1,
}
impl DMAINTSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAINTSEL1W::_0 => false,
            DMAINTSEL1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAINTSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAINTSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAINTSEL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA0 channel 5 is not available in vector 17."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAINTSEL1W::_0)
    }
    #[doc = "DMA0 channel 5 is available in vector 17."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAINTSEL1W::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAINTSEL2`"]
pub enum DMAINTSEL2W {
    #[doc = "DMA0 channel 6 is not available in vector 18."]
    _0,
    #[doc = "DMA0 channel 6 is available in vector 18."]
    _1,
}
impl DMAINTSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAINTSEL2W::_0 => false,
            DMAINTSEL2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAINTSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAINTSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAINTSEL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA0 channel 6 is not available in vector 18."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAINTSEL2W::_0)
    }
    #[doc = "DMA0 channel 6 is available in vector 18."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAINTSEL2W::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAINTSEL3`"]
pub enum DMAINTSEL3W {
    #[doc = "DMA0 channel 7 is not available in vector 19."]
    _0,
    #[doc = "DMA0 channel 7 is available in vector 19."]
    _1,
}
impl DMAINTSEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAINTSEL3W::_0 => false,
            DMAINTSEL3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAINTSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAINTSEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAINTSEL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA0 channel 7 is not available in vector 19."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAINTSEL3W::_0)
    }
    #[doc = "DMA0 channel 7 is available in vector 19."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAINTSEL3W::_1)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - DMA Channel Interrupts Select 0"]
    #[inline]
    pub fn dmaintsel0(&self) -> DMAINTSEL0R {
        DMAINTSEL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DMA Channel Interrupts Select 1"]
    #[inline]
    pub fn dmaintsel1(&self) -> DMAINTSEL1R {
        DMAINTSEL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DMA Channel Interrupts Select 2"]
    #[inline]
    pub fn dmaintsel2(&self) -> DMAINTSEL2R {
        DMAINTSEL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DMA Channel Interrupts Select 3"]
    #[inline]
    pub fn dmaintsel3(&self) -> DMAINTSEL3R {
        DMAINTSEL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - LTC Status"]
    #[inline]
    pub fn ltcen(&self) -> LTCENR {
        LTCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65536 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DMA Channel Interrupts Select 0"]
    #[inline]
    pub fn dmaintsel0(&mut self) -> _DMAINTSEL0W {
        _DMAINTSEL0W { w: self }
    }
    #[doc = "Bit 1 - DMA Channel Interrupts Select 1"]
    #[inline]
    pub fn dmaintsel1(&mut self) -> _DMAINTSEL1W {
        _DMAINTSEL1W { w: self }
    }
    #[doc = "Bit 2 - DMA Channel Interrupts Select 2"]
    #[inline]
    pub fn dmaintsel2(&mut self) -> _DMAINTSEL2W {
        _DMAINTSEL2W { w: self }
    }
    #[doc = "Bit 3 - DMA Channel Interrupts Select 3"]
    #[inline]
    pub fn dmaintsel3(&mut self) -> _DMAINTSEL3W {
        _DMAINTSEL3W { w: self }
    }
}
