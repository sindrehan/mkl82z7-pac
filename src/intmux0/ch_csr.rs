#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH_CSR {
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
#[doc = "Possible values of the field `RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTR {
    #[doc = "No operation."]
    _0,
    #[doc = "Perform a software reset on this channel."]
    _1,
}
impl RSTR {
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
            RSTR::_0 => false,
            RSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTR {
        match value {
            false => RSTR::_0,
            true => RSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSTR::_1
    }
}
#[doc = "Possible values of the field `AND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANDR {
    #[doc = "Logic OR all enabled interrupt inputs."]
    _0,
    #[doc = "Logic AND all enabled interrupt inputs."]
    _1,
}
impl ANDR {
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
            ANDR::_0 => false,
            ANDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANDR {
        match value {
            false => ANDR::_0,
            true => ANDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ANDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ANDR::_1
    }
}
#[doc = "Possible values of the field `IRQN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQNR {
    #[doc = "32 interrupt inputs"]
    _00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IRQNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IRQNR::_00 => 0,
            IRQNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IRQNR {
        match value {
            0 => IRQNR::_00,
            i => IRQNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == IRQNR::_00
    }
}
#[doc = r" Value of the field"]
pub struct CHINR {
    bits: u8,
}
impl CHINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IRQP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQPR {
    #[doc = "No interrupt is pending."]
    _0,
    #[doc = "The interrupt output of this channel is pending."]
    _1,
}
impl IRQPR {
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
            IRQPR::_0 => false,
            IRQPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQPR {
        match value {
            false => IRQPR::_0,
            true => IRQPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRQPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRQPR::_1
    }
}
#[doc = "Values that can be written to the field `RST`"]
pub enum RSTW {
    #[doc = "No operation."]
    _0,
    #[doc = "Perform a software reset on this channel."]
    _1,
}
impl RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTW::_0 => false,
            RSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTW::_0)
    }
    #[doc = "Perform a software reset on this channel."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTW::_1)
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
#[doc = "Values that can be written to the field `AND`"]
pub enum ANDW {
    #[doc = "Logic OR all enabled interrupt inputs."]
    _0,
    #[doc = "Logic AND all enabled interrupt inputs."]
    _1,
}
impl ANDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANDW::_0 => false,
            ANDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANDW<'a> {
    w: &'a mut W,
}
impl<'a> _ANDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic OR all enabled interrupt inputs."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANDW::_0)
    }
    #[doc = "Logic AND all enabled interrupt inputs."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANDW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Logic AND"]
    #[inline]
    pub fn and(&self) -> ANDR {
        ANDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Channel Input Number"]
    #[inline]
    pub fn irqn(&self) -> IRQNR {
        IRQNR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Channel Instance Number"]
    #[inline]
    pub fn chin(&self) -> CHINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHINR { bits }
    }
    #[doc = "Bit 31 - Channel Interrupt Request Pending"]
    #[inline]
    pub fn irqp(&self) -> IRQPR {
        IRQPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bit 1 - Logic AND"]
    #[inline]
    pub fn and(&mut self) -> _ANDW {
        _ANDW { w: self }
    }
}
