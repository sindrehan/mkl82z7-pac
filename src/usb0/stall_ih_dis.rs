#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STALL_IH_DIS {
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
#[doc = "Possible values of the field `STALL_I_DIS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS8R {
    #[doc = "Endpoint 8 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 8 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS8R {
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
            STALL_I_DIS8R::_0 => false,
            STALL_I_DIS8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_I_DIS8R {
        match value {
            false => STALL_I_DIS8R::_0,
            true => STALL_I_DIS8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_I_DIS8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_I_DIS8R::_1
    }
}
#[doc = "Possible values of the field `STALL_I_DIS9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS9R {
    #[doc = "Endpoint 9 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 9 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS9R {
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
            STALL_I_DIS9R::_0 => false,
            STALL_I_DIS9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_I_DIS9R {
        match value {
            false => STALL_I_DIS9R::_0,
            true => STALL_I_DIS9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_I_DIS9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_I_DIS9R::_1
    }
}
#[doc = "Possible values of the field `STALL_I_DIS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS10R {
    #[doc = "Endpoint 10 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 10 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS10R {
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
            STALL_I_DIS10R::_0 => false,
            STALL_I_DIS10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_I_DIS10R {
        match value {
            false => STALL_I_DIS10R::_0,
            true => STALL_I_DIS10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_I_DIS10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_I_DIS10R::_1
    }
}
#[doc = "Possible values of the field `STALL_I_DIS11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS11R {
    #[doc = "Endpoint 11 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 11 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS11R {
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
            STALL_I_DIS11R::_0 => false,
            STALL_I_DIS11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_I_DIS11R {
        match value {
            false => STALL_I_DIS11R::_0,
            true => STALL_I_DIS11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_I_DIS11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_I_DIS11R::_1
    }
}
#[doc = "Possible values of the field `STALL_I_DIS12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS12R {
    #[doc = "Endpoint 12 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 12 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS12R {
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
            STALL_I_DIS12R::_0 => false,
            STALL_I_DIS12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_I_DIS12R {
        match value {
            false => STALL_I_DIS12R::_0,
            true => STALL_I_DIS12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_I_DIS12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_I_DIS12R::_1
    }
}
#[doc = "Possible values of the field `STALL_I_DIS13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS13R {
    #[doc = "Endpoint 13 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 13 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS13R {
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
            STALL_I_DIS13R::_0 => false,
            STALL_I_DIS13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_I_DIS13R {
        match value {
            false => STALL_I_DIS13R::_0,
            true => STALL_I_DIS13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_I_DIS13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_I_DIS13R::_1
    }
}
#[doc = "Possible values of the field `STALL_I_DIS14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS14R {
    #[doc = "Endpoint 14 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 14 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS14R {
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
            STALL_I_DIS14R::_0 => false,
            STALL_I_DIS14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_I_DIS14R {
        match value {
            false => STALL_I_DIS14R::_0,
            true => STALL_I_DIS14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_I_DIS14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_I_DIS14R::_1
    }
}
#[doc = "Possible values of the field `STALL_I_DIS15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_I_DIS15R {
    #[doc = "Endpoint 15 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 15 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS15R {
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
            STALL_I_DIS15R::_0 => false,
            STALL_I_DIS15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALL_I_DIS15R {
        match value {
            false => STALL_I_DIS15R::_0,
            true => STALL_I_DIS15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STALL_I_DIS15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STALL_I_DIS15R::_1
    }
}
#[doc = "Values that can be written to the field `STALL_I_DIS8`"]
pub enum STALL_I_DIS8W {
    #[doc = "Endpoint 8 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 8 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_I_DIS8W::_0 => false,
            STALL_I_DIS8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_I_DIS8W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_I_DIS8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_I_DIS8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 8 IN direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_I_DIS8W::_0)
    }
    #[doc = "Endpoint 8 IN direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_I_DIS8W::_1)
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
#[doc = "Values that can be written to the field `STALL_I_DIS9`"]
pub enum STALL_I_DIS9W {
    #[doc = "Endpoint 9 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 9 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_I_DIS9W::_0 => false,
            STALL_I_DIS9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_I_DIS9W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_I_DIS9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_I_DIS9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 9 IN direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_I_DIS9W::_0)
    }
    #[doc = "Endpoint 9 IN direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_I_DIS9W::_1)
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
#[doc = "Values that can be written to the field `STALL_I_DIS10`"]
pub enum STALL_I_DIS10W {
    #[doc = "Endpoint 10 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 10 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_I_DIS10W::_0 => false,
            STALL_I_DIS10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_I_DIS10W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_I_DIS10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_I_DIS10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 10 IN direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_I_DIS10W::_0)
    }
    #[doc = "Endpoint 10 IN direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_I_DIS10W::_1)
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
#[doc = "Values that can be written to the field `STALL_I_DIS11`"]
pub enum STALL_I_DIS11W {
    #[doc = "Endpoint 11 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 11 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_I_DIS11W::_0 => false,
            STALL_I_DIS11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_I_DIS11W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_I_DIS11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_I_DIS11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 11 IN direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_I_DIS11W::_0)
    }
    #[doc = "Endpoint 11 IN direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_I_DIS11W::_1)
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
#[doc = "Values that can be written to the field `STALL_I_DIS12`"]
pub enum STALL_I_DIS12W {
    #[doc = "Endpoint 12 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 12 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_I_DIS12W::_0 => false,
            STALL_I_DIS12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_I_DIS12W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_I_DIS12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_I_DIS12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 12 IN direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_I_DIS12W::_0)
    }
    #[doc = "Endpoint 12 IN direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_I_DIS12W::_1)
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
#[doc = "Values that can be written to the field `STALL_I_DIS13`"]
pub enum STALL_I_DIS13W {
    #[doc = "Endpoint 13 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 13 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_I_DIS13W::_0 => false,
            STALL_I_DIS13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_I_DIS13W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_I_DIS13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_I_DIS13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 13 IN direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_I_DIS13W::_0)
    }
    #[doc = "Endpoint 13 IN direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_I_DIS13W::_1)
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
#[doc = "Values that can be written to the field `STALL_I_DIS14`"]
pub enum STALL_I_DIS14W {
    #[doc = "Endpoint 14 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 14 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_I_DIS14W::_0 => false,
            STALL_I_DIS14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_I_DIS14W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_I_DIS14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_I_DIS14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 14 IN direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_I_DIS14W::_0)
    }
    #[doc = "Endpoint 14 IN direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_I_DIS14W::_1)
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
#[doc = "Values that can be written to the field `STALL_I_DIS15`"]
pub enum STALL_I_DIS15W {
    #[doc = "Endpoint 15 IN direction stall is enabled."]
    _0,
    #[doc = "Endpoint 15 IN direction stall is disabled."]
    _1,
}
impl STALL_I_DIS15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALL_I_DIS15W::_0 => false,
            STALL_I_DIS15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALL_I_DIS15W<'a> {
    w: &'a mut W,
}
impl<'a> _STALL_I_DIS15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALL_I_DIS15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Endpoint 15 IN direction stall is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_I_DIS15W::_0)
    }
    #[doc = "Endpoint 15 IN direction stall is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_I_DIS15W::_1)
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
    #[doc = "Bit 0 - Disable endpoint 8 IN direction."]
    #[inline]
    pub fn stall_i_dis8(&self) -> STALL_I_DIS8R {
        STALL_I_DIS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Disable endpoint 9 IN direction."]
    #[inline]
    pub fn stall_i_dis9(&self) -> STALL_I_DIS9R {
        STALL_I_DIS9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Disable endpoint 10 IN direction."]
    #[inline]
    pub fn stall_i_dis10(&self) -> STALL_I_DIS10R {
        STALL_I_DIS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Disable endpoint 11 IN direction."]
    #[inline]
    pub fn stall_i_dis11(&self) -> STALL_I_DIS11R {
        STALL_I_DIS11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Disable endpoint 12 IN direction."]
    #[inline]
    pub fn stall_i_dis12(&self) -> STALL_I_DIS12R {
        STALL_I_DIS12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Disable endpoint 13 IN direction."]
    #[inline]
    pub fn stall_i_dis13(&self) -> STALL_I_DIS13R {
        STALL_I_DIS13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Disable endpoint 14 IN direction."]
    #[inline]
    pub fn stall_i_dis14(&self) -> STALL_I_DIS14R {
        STALL_I_DIS14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Disable endpoint 15 IN direction."]
    #[inline]
    pub fn stall_i_dis15(&self) -> STALL_I_DIS15R {
        STALL_I_DIS15R::_from({
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
    #[doc = "Bit 0 - Disable endpoint 8 IN direction."]
    #[inline]
    pub fn stall_i_dis8(&mut self) -> _STALL_I_DIS8W {
        _STALL_I_DIS8W { w: self }
    }
    #[doc = "Bit 1 - Disable endpoint 9 IN direction."]
    #[inline]
    pub fn stall_i_dis9(&mut self) -> _STALL_I_DIS9W {
        _STALL_I_DIS9W { w: self }
    }
    #[doc = "Bit 2 - Disable endpoint 10 IN direction."]
    #[inline]
    pub fn stall_i_dis10(&mut self) -> _STALL_I_DIS10W {
        _STALL_I_DIS10W { w: self }
    }
    #[doc = "Bit 3 - Disable endpoint 11 IN direction."]
    #[inline]
    pub fn stall_i_dis11(&mut self) -> _STALL_I_DIS11W {
        _STALL_I_DIS11W { w: self }
    }
    #[doc = "Bit 4 - Disable endpoint 12 IN direction."]
    #[inline]
    pub fn stall_i_dis12(&mut self) -> _STALL_I_DIS12W {
        _STALL_I_DIS12W { w: self }
    }
    #[doc = "Bit 5 - Disable endpoint 13 IN direction."]
    #[inline]
    pub fn stall_i_dis13(&mut self) -> _STALL_I_DIS13W {
        _STALL_I_DIS13W { w: self }
    }
    #[doc = "Bit 6 - Disable endpoint 14 IN direction."]
    #[inline]
    pub fn stall_i_dis14(&mut self) -> _STALL_I_DIS14W {
        _STALL_I_DIS14W { w: self }
    }
    #[doc = "Bit 7 - Disable endpoint 15 IN direction."]
    #[inline]
    pub fn stall_i_dis15(&mut self) -> _STALL_I_DIS15W {
        _STALL_I_DIS15W { w: self }
    }
}
