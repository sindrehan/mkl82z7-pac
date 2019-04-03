#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::HVDSC1 {
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
#[doc = "Possible values of the field `HVDV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVDVR {
    #[doc = "Low trip point selected (V HVD = V HVDL )"]
    _0,
    #[doc = "High trip point selected (V HVD = V HVDH )"]
    _1,
}
impl HVDVR {
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
            HVDVR::_0 => false,
            HVDVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HVDVR {
        match value {
            false => HVDVR::_0,
            true => HVDVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HVDVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HVDVR::_1
    }
}
#[doc = "Possible values of the field `HVDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVDRER {
    #[doc = "HVDF does not generate hardware resets"]
    _0,
    #[doc = "Force an MCU reset when HVDF = 1"]
    _1,
}
impl HVDRER {
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
            HVDRER::_0 => false,
            HVDRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HVDRER {
        match value {
            false => HVDRER::_0,
            true => HVDRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HVDRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HVDRER::_1
    }
}
#[doc = "Possible values of the field `HVDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVDIER {
    #[doc = "Hardware interrupt disabled (use polling)"]
    _0,
    #[doc = "Request a hardware interrupt when HVDF = 1"]
    _1,
}
impl HVDIER {
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
            HVDIER::_0 => false,
            HVDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HVDIER {
        match value {
            false => HVDIER::_0,
            true => HVDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HVDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HVDIER::_1
    }
}
#[doc = "Possible values of the field `HVDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVDFR {
    #[doc = "High-voltage event not detected"]
    _0,
    #[doc = "High-voltage event detected"]
    _1,
}
impl HVDFR {
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
            HVDFR::_0 => false,
            HVDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HVDFR {
        match value {
            false => HVDFR::_0,
            true => HVDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HVDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HVDFR::_1
    }
}
#[doc = "Values that can be written to the field `HVDV`"]
pub enum HVDVW {
    #[doc = "Low trip point selected (V HVD = V HVDL )"]
    _0,
    #[doc = "High trip point selected (V HVD = V HVDH )"]
    _1,
}
impl HVDVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HVDVW::_0 => false,
            HVDVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HVDVW<'a> {
    w: &'a mut W,
}
impl<'a> _HVDVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HVDVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low trip point selected (V HVD = V HVDL )"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HVDVW::_0)
    }
    #[doc = "High trip point selected (V HVD = V HVDH )"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HVDVW::_1)
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
#[doc = "Values that can be written to the field `HVDRE`"]
pub enum HVDREW {
    #[doc = "HVDF does not generate hardware resets"]
    _0,
    #[doc = "Force an MCU reset when HVDF = 1"]
    _1,
}
impl HVDREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HVDREW::_0 => false,
            HVDREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HVDREW<'a> {
    w: &'a mut W,
}
impl<'a> _HVDREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HVDREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HVDF does not generate hardware resets"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HVDREW::_0)
    }
    #[doc = "Force an MCU reset when HVDF = 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HVDREW::_1)
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
#[doc = "Values that can be written to the field `HVDIE`"]
pub enum HVDIEW {
    #[doc = "Hardware interrupt disabled (use polling)"]
    _0,
    #[doc = "Request a hardware interrupt when HVDF = 1"]
    _1,
}
impl HVDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HVDIEW::_0 => false,
            HVDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HVDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HVDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HVDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HVDIEW::_0)
    }
    #[doc = "Request a hardware interrupt when HVDF = 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HVDIEW::_1)
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
pub struct _HVDACKW<'a> {
    w: &'a mut W,
}
impl<'a> _HVDACKW<'a> {
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
    #[doc = "Bit 0 - High-Voltage Detect Voltage Select"]
    #[inline]
    pub fn hvdv(&self) -> HVDVR {
        HVDVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - High-Voltage Detect Reset Enable"]
    #[inline]
    pub fn hvdre(&self) -> HVDRER {
        HVDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - High-Voltage Detect Interrupt Enable"]
    #[inline]
    pub fn hvdie(&self) -> HVDIER {
        HVDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - High-Voltage Detect Flag"]
    #[inline]
    pub fn hvdf(&self) -> HVDFR {
        HVDFR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - High-Voltage Detect Voltage Select"]
    #[inline]
    pub fn hvdv(&mut self) -> _HVDVW {
        _HVDVW { w: self }
    }
    #[doc = "Bit 4 - High-Voltage Detect Reset Enable"]
    #[inline]
    pub fn hvdre(&mut self) -> _HVDREW {
        _HVDREW { w: self }
    }
    #[doc = "Bit 5 - High-Voltage Detect Interrupt Enable"]
    #[inline]
    pub fn hvdie(&mut self) -> _HVDIEW {
        _HVDIEW { w: self }
    }
    #[doc = "Bit 6 - High-Voltage Detect Acknowledge"]
    #[inline]
    pub fn hvdack(&mut self) -> _HVDACKW {
        _HVDACKW { w: self }
    }
}
