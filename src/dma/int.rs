#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT {
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
#[doc = "Possible values of the field `INT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT0R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT0R {
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
            INT0R::_0 => false,
            INT0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT0R {
        match value {
            false => INT0R::_0,
            true => INT0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT0R::_1
    }
}
#[doc = "Possible values of the field `INT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT1R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT1R {
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
            INT1R::_0 => false,
            INT1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT1R {
        match value {
            false => INT1R::_0,
            true => INT1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT1R::_1
    }
}
#[doc = "Possible values of the field `INT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT2R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT2R {
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
            INT2R::_0 => false,
            INT2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT2R {
        match value {
            false => INT2R::_0,
            true => INT2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT2R::_1
    }
}
#[doc = "Possible values of the field `INT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT3R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT3R {
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
            INT3R::_0 => false,
            INT3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT3R {
        match value {
            false => INT3R::_0,
            true => INT3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT3R::_1
    }
}
#[doc = "Possible values of the field `INT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT4R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT4R {
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
            INT4R::_0 => false,
            INT4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT4R {
        match value {
            false => INT4R::_0,
            true => INT4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT4R::_1
    }
}
#[doc = "Possible values of the field `INT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT5R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT5R {
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
            INT5R::_0 => false,
            INT5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT5R {
        match value {
            false => INT5R::_0,
            true => INT5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT5R::_1
    }
}
#[doc = "Possible values of the field `INT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT6R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT6R {
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
            INT6R::_0 => false,
            INT6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT6R {
        match value {
            false => INT6R::_0,
            true => INT6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT6R::_1
    }
}
#[doc = "Possible values of the field `INT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT7R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT7R {
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
            INT7R::_0 => false,
            INT7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT7R {
        match value {
            false => INT7R::_0,
            true => INT7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT7R::_1
    }
}
#[doc = "Values that can be written to the field `INT0`"]
pub enum INT0W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT0W::_0 => false,
            INT0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT0W<'a> {
    w: &'a mut W,
}
impl<'a> _INT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT0W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT0W::_1)
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
#[doc = "Values that can be written to the field `INT1`"]
pub enum INT1W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT1W::_0 => false,
            INT1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT1W<'a> {
    w: &'a mut W,
}
impl<'a> _INT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT1W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT1W::_1)
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
#[doc = "Values that can be written to the field `INT2`"]
pub enum INT2W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT2W::_0 => false,
            INT2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT2W<'a> {
    w: &'a mut W,
}
impl<'a> _INT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT2W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT2W::_1)
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
#[doc = "Values that can be written to the field `INT3`"]
pub enum INT3W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT3W::_0 => false,
            INT3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT3W<'a> {
    w: &'a mut W,
}
impl<'a> _INT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT3W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT3W::_1)
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
#[doc = "Values that can be written to the field `INT4`"]
pub enum INT4W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT4W::_0 => false,
            INT4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT4W<'a> {
    w: &'a mut W,
}
impl<'a> _INT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT4W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT4W::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT5`"]
pub enum INT5W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT5W::_0 => false,
            INT5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT5W<'a> {
    w: &'a mut W,
}
impl<'a> _INT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT5W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT5W::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT6`"]
pub enum INT6W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT6W::_0 => false,
            INT6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT6W<'a> {
    w: &'a mut W,
}
impl<'a> _INT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT6W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT6W::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT7`"]
pub enum INT7W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT7W::_0 => false,
            INT7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT7W<'a> {
    w: &'a mut W,
}
impl<'a> _INT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT7W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT7W::_1)
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
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline]
    pub fn int0(&self) -> INT0R {
        INT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline]
    pub fn int1(&self) -> INT1R {
        INT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline]
    pub fn int2(&self) -> INT2R {
        INT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline]
    pub fn int3(&self) -> INT3R {
        INT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline]
    pub fn int4(&self) -> INT4R {
        INT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline]
    pub fn int5(&self) -> INT5R {
        INT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline]
    pub fn int6(&self) -> INT6R {
        INT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline]
    pub fn int7(&self) -> INT7R {
        INT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline]
    pub fn int0(&mut self) -> _INT0W {
        _INT0W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline]
    pub fn int1(&mut self) -> _INT1W {
        _INT1W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline]
    pub fn int2(&mut self) -> _INT2W {
        _INT2W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline]
    pub fn int3(&mut self) -> _INT3W {
        _INT3W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline]
    pub fn int4(&mut self) -> _INT4W {
        _INT4W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline]
    pub fn int5(&mut self) -> _INT5W {
        _INT5W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline]
    pub fn int6(&mut self) -> _INT6W {
        _INT6W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline]
    pub fn int7(&mut self) -> _INT7W {
        _INT7W { w: self }
    }
}
