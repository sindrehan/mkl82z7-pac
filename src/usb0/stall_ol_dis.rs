#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STALL_OL_DIS {
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
#[doc = "Possible values of the field `STALL_O_DIS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS0R {
    #[doc = "Endpoint 0 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 0 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS0R {
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
            STALL_O_DIS0R::_0 => false,
            STALL_O_DIS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_O_DIS0R {
        match value {
            false => STALL_O_DIS0R::_0,
            true => STALL_O_DIS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS0R::_1
    }
}
#[doc = "Possible values of the field `STALL_O_DIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS1R {
    #[doc = "Endpoint 1 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 1 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS1R {
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
            STALL_O_DIS1R::_0 => false,
            STALL_O_DIS1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_O_DIS1R {
        match value {
            false => STALL_O_DIS1R::_0,
            true => STALL_O_DIS1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS1R::_1
    }
}
#[doc = "Possible values of the field `STALL_O_DIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS2R {
    #[doc = "Endpoint 2 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 2 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS2R {
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
            STALL_O_DIS2R::_0 => false,
            STALL_O_DIS2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_O_DIS2R {
        match value {
            false => STALL_O_DIS2R::_0,
            true => STALL_O_DIS2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS2R::_1
    }
}
#[doc = "Possible values of the field `STALL_O_DIS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS3R {
    #[doc = "Endpoint 3 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 3 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS3R {
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
            STALL_O_DIS3R::_0 => false,
            STALL_O_DIS3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_O_DIS3R {
        match value {
            false => STALL_O_DIS3R::_0,
            true => STALL_O_DIS3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS3R::_1
    }
}
#[doc = "Possible values of the field `STALL_O_DIS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS4R {
    #[doc = "Endpoint 4 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 4 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS4R {
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
            STALL_O_DIS4R::_0 => false,
            STALL_O_DIS4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_O_DIS4R {
        match value {
            false => STALL_O_DIS4R::_0,
            true => STALL_O_DIS4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS4R::_1
    }
}
#[doc = "Possible values of the field `STALL_O_DIS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS5R {
    #[doc = "Endpoint 5 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 5 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS5R {
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
            STALL_O_DIS5R::_0 => false,
            STALL_O_DIS5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_O_DIS5R {
        match value {
            false => STALL_O_DIS5R::_0,
            true => STALL_O_DIS5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS5R::_1
    }
}
#[doc = "Possible values of the field `STALL_O_DIS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS6R {
    #[doc = "Endpoint 6 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 6 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS6R {
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
            STALL_O_DIS6R::_0 => false,
            STALL_O_DIS6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_O_DIS6R {
        match value {
            false => STALL_O_DIS6R::_0,
            true => STALL_O_DIS6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS6R::_1
    }
}
#[doc = "Possible values of the field `STALL_O_DIS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS7R {
    #[doc = "Endpoint 7 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 7 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS7R {
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
            STALL_O_DIS7R::_0 => false,
            STALL_O_DIS7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_O_DIS7R {
        match value {
            false => STALL_O_DIS7R::_0,
            true => STALL_O_DIS7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS7R::_1
    }
}
#[doc = "Values that can be written to the field `STALL_O_DIS0`"]
pub enum STALL_O_DIS0W {
    #[doc = "Endpoint 0 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 0 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_O_DIS0W::_0 => false,
            STALL_O_DIS0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_O_DIS0W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_O_DIS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_O_DIS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 0 OUT direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS0W::_0)
    }
    #[doc = "Endpoint 0 OUT direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS0W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STALL_O_DIS1`"]
pub enum STALL_O_DIS1W {
    #[doc = "Endpoint 1 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 1 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_O_DIS1W::_0 => false,
            STALL_O_DIS1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_O_DIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_O_DIS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_O_DIS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 1 OUT direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS1W::_0)
    }
    #[doc = "Endpoint 1 OUT direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS1W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STALL_O_DIS2`"]
pub enum STALL_O_DIS2W {
    #[doc = "Endpoint 2 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 2 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_O_DIS2W::_0 => false,
            STALL_O_DIS2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_O_DIS2W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_O_DIS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_O_DIS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 2 OUT direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS2W::_0)
    }
    #[doc = "Endpoint 2 OUT direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS2W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STALL_O_DIS3`"]
pub enum STALL_O_DIS3W {
    #[doc = "Endpoint 3 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 3 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_O_DIS3W::_0 => false,
            STALL_O_DIS3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_O_DIS3W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_O_DIS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_O_DIS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 3 OUT direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS3W::_0)
    }
    #[doc = "Endpoint 3 OUT direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS3W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STALL_O_DIS4`"]
pub enum STALL_O_DIS4W {
    #[doc = "Endpoint 4 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 4 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_O_DIS4W::_0 => false,
            STALL_O_DIS4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_O_DIS4W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_O_DIS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_O_DIS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 4 OUT direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS4W::_0)
    }
    #[doc = "Endpoint 4 OUT direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS4W::_1)
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
#[doc = "Values that can be written to the field `STALL_O_DIS5`"]
pub enum STALL_O_DIS5W {
    #[doc = "Endpoint 5 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 5 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_O_DIS5W::_0 => false,
            STALL_O_DIS5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_O_DIS5W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_O_DIS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_O_DIS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 5 OUT direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS5W::_0)
    }
    #[doc = "Endpoint 5 OUT direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS5W::_1)
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
#[doc = "Values that can be written to the field `STALL_O_DIS6`"]
pub enum STALL_O_DIS6W {
    #[doc = "Endpoint 6 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 6 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_O_DIS6W::_0 => false,
            STALL_O_DIS6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_O_DIS6W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_O_DIS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_O_DIS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 6 OUT direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS6W::_0)
    }
    #[doc = "Endpoint 6 OUT direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS6W::_1)
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
#[doc = "Values that can be written to the field `STALL_O_DIS7`"]
pub enum STALL_O_DIS7W {
    #[doc = "Endpoint 7 OUT direction stall is enabled."]
    _0,
    #[doc = "Endpoint 7 OUT direction stall is disabled."]
    _1,
}
impl STALL_O_DIS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_O_DIS7W::_0 => false,
            STALL_O_DIS7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_O_DIS7W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_O_DIS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_O_DIS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 7 OUT direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS7W::_0)
    }
    #[doc = "Endpoint 7 OUT direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS7W::_1)
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
    #[doc = "Bit 0 - Disable endpoint 0 OUT direction."]
    #[inline]
    pub fn stall_o_dis0(&self) -> STALL_O_DIS0R {
        STALL_O_DIS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Disable endpoint 1 OUT direction."]
    #[inline]
    pub fn stall_o_dis1(&self) -> STALL_O_DIS1R {
        STALL_O_DIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Disable endpoint 2 OUT direction."]
    #[inline]
    pub fn stall_o_dis2(&self) -> STALL_O_DIS2R {
        STALL_O_DIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Disable endpoint 3 OUT direction."]
    #[inline]
    pub fn stall_o_dis3(&self) -> STALL_O_DIS3R {
        STALL_O_DIS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Disable endpoint 4 OUT direction."]
    #[inline]
    pub fn stall_o_dis4(&self) -> STALL_O_DIS4R {
        STALL_O_DIS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Disable endpoint 5 OUT direction."]
    #[inline]
    pub fn stall_o_dis5(&self) -> STALL_O_DIS5R {
        STALL_O_DIS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Disable endpoint 6 OUT direction."]
    #[inline]
    pub fn stall_o_dis6(&self) -> STALL_O_DIS6R {
        STALL_O_DIS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Disable endpoint 7 OUT direction."]
    #[inline]
    pub fn stall_o_dis7(&self) -> STALL_O_DIS7R {
        STALL_O_DIS7R::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Disable endpoint 0 OUT direction."]
    #[inline]
    pub fn stall_o_dis0(&mut self) -> _STALL_O_DIS0W {
        _STALL_O_DIS0W { w: self }
    }
    #[doc = "Bit 1 - Disable endpoint 1 OUT direction."]
    #[inline]
    pub fn stall_o_dis1(&mut self) -> _STALL_O_DIS1W {
        _STALL_O_DIS1W { w: self }
    }
    #[doc = "Bit 2 - Disable endpoint 2 OUT direction."]
    #[inline]
    pub fn stall_o_dis2(&mut self) -> _STALL_O_DIS2W {
        _STALL_O_DIS2W { w: self }
    }
    #[doc = "Bit 3 - Disable endpoint 3 OUT direction."]
    #[inline]
    pub fn stall_o_dis3(&mut self) -> _STALL_O_DIS3W {
        _STALL_O_DIS3W { w: self }
    }
    #[doc = "Bit 4 - Disable endpoint 4 OUT direction."]
    #[inline]
    pub fn stall_o_dis4(&mut self) -> _STALL_O_DIS4W {
        _STALL_O_DIS4W { w: self }
    }
    #[doc = "Bit 5 - Disable endpoint 5 OUT direction."]
    #[inline]
    pub fn stall_o_dis5(&mut self) -> _STALL_O_DIS5W {
        _STALL_O_DIS5W { w: self }
    }
    #[doc = "Bit 6 - Disable endpoint 6 OUT direction."]
    #[inline]
    pub fn stall_o_dis6(&mut self) -> _STALL_O_DIS6W {
        _STALL_O_DIS6W { w: self }
    }
    #[doc = "Bit 7 - Disable endpoint 7 OUT direction."]
    #[inline]
    pub fn stall_o_dis7(&mut self) -> _STALL_O_DIS7W {
        _STALL_O_DIS7W { w: self }
    }
}
