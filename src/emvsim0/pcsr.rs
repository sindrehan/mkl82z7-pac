#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCSR {
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
#[doc = "Possible values of the field `SAPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAPDR {
    #[doc = "Auto power down disabled (default)"]
    _0,
    #[doc = "Auto power down enabled"]
    _1,
}
impl SAPDR {
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
            SAPDR::_0 => false,
            SAPDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAPDR {
        match value {
            false => SAPDR::_0,
            true => SAPDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SAPDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAPDR::_1
    }
}
#[doc = "Possible values of the field `SVCC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCC_ENR {
    #[doc = "Smart Card Voltage disabled (default)"]
    _0,
    #[doc = "Smart Card Voltage enabled"]
    _1,
}
impl SVCC_ENR {
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
            SVCC_ENR::_0 => false,
            SVCC_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVCC_ENR {
        match value {
            false => SVCC_ENR::_0,
            true => SVCC_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SVCC_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SVCC_ENR::_1
    }
}
#[doc = "Possible values of the field `VCCENP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCCENPR {
    #[doc = "VCC_EN is active high. Polarity of SVCC_EN is unchanged."]
    _0,
    #[doc = "VCC_EN is active low. Polarity of SVCC_EN is inverted."]
    _1,
}
impl VCCENPR {
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
            VCCENPR::_0 => false,
            VCCENPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VCCENPR {
        match value {
            false => VCCENPR::_0,
            true => VCCENPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VCCENPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VCCENPR::_1
    }
}
#[doc = "Possible values of the field `SRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSTR {
    #[doc = "Smart Card Reset is asserted (default)"]
    _0,
    #[doc = "Smart Card Reset is de-asserted"]
    _1,
}
impl SRSTR {
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
            SRSTR::_0 => false,
            SRSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRSTR {
        match value {
            false => SRSTR::_0,
            true => SRSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRSTR::_1
    }
}
#[doc = "Possible values of the field `SCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCENR {
    #[doc = "Smart Card Clock Disabled"]
    _0,
    #[doc = "Smart Card Clock Enabled"]
    _1,
}
impl SCENR {
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
            SCENR::_0 => false,
            SCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCENR {
        match value {
            false => SCENR::_0,
            true => SCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SCENR::_1
    }
}
#[doc = "Possible values of the field `SCSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSPR {
    #[doc = "Clock is logic 0 when stopped by SCEN"]
    _0,
    #[doc = "Clock is logic 1 when stopped by SCEN"]
    _1,
}
impl SCSPR {
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
            SCSPR::_0 => false,
            SCSPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCSPR {
        match value {
            false => SCSPR::_0,
            true => SCSPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SCSPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SCSPR::_1
    }
}
#[doc = "Possible values of the field `SPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDR {
    #[doc = "No effect (default)"]
    _0,
    #[doc = "Start Auto Powerdown or Power Down is in progress"]
    _1,
}
impl SPDR {
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
            SPDR::_0 => false,
            SPDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPDR {
        match value {
            false => SPDR::_0,
            true => SPDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPDR::_1
    }
}
#[doc = "Possible values of the field `SPDIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIMR {
    #[doc = "SIM presence detect interrupt is enabled"]
    _0,
    #[doc = "SIM presence detect interrupt is masked (default)"]
    _1,
}
impl SPDIMR {
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
            SPDIMR::_0 => false,
            SPDIMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPDIMR {
        match value {
            false => SPDIMR::_0,
            true => SPDIMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPDIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPDIMR::_1
    }
}
#[doc = "Possible values of the field `SPDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIFR {
    #[doc = "No insertion or removal of Smart Card detected on Port (default)"]
    _0,
    #[doc = "Insertion or removal of Smart Card detected on Port"]
    _1,
}
impl SPDIFR {
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
            SPDIFR::_0 => false,
            SPDIFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPDIFR {
        match value {
            false => SPDIFR::_0,
            true => SPDIFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPDIFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPDIFR::_1
    }
}
#[doc = "Possible values of the field `SPDP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDPR {
    #[doc = "SIM Presence Detect pin is logic low"]
    _0,
    #[doc = "SIM Presence Detectpin is logic high"]
    _1,
}
impl SPDPR {
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
            SPDPR::_0 => false,
            SPDPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPDPR {
        match value {
            false => SPDPR::_0,
            true => SPDPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPDPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPDPR::_1
    }
}
#[doc = "Possible values of the field `SPDES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDESR {
    #[doc = "Falling edge on the pin (default)"]
    _0,
    #[doc = "Rising edge on the pin"]
    _1,
}
impl SPDESR {
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
            SPDESR::_0 => false,
            SPDESR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPDESR {
        match value {
            false => SPDESR::_0,
            true => SPDESR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPDESR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPDESR::_1
    }
}
#[doc = "Values that can be written to the field `SAPD`"]
pub enum SAPDW {
    #[doc = "Auto power down disabled (default)"]
    _0,
    #[doc = "Auto power down enabled"]
    _1,
}
impl SAPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAPDW::_0 => false,
            SAPDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAPDW<'a> {
    w: &'a mut W,
}
impl<'a> _SAPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto power down disabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAPDW::_0)
    }
    #[doc = "Auto power down enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAPDW::_1)
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
#[doc = "Values that can be written to the field `SVCC_EN`"]
pub enum SVCC_ENW {
    #[doc = "Smart Card Voltage disabled (default)"]
    _0,
    #[doc = "Smart Card Voltage enabled"]
    _1,
}
impl SVCC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SVCC_ENW::_0 => false,
            SVCC_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVCC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SVCC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVCC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Smart Card Voltage disabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVCC_ENW::_0)
    }
    #[doc = "Smart Card Voltage enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVCC_ENW::_1)
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
#[doc = "Values that can be written to the field `VCCENP`"]
pub enum VCCENPW {
    #[doc = "VCC_EN is active high. Polarity of SVCC_EN is unchanged."]
    _0,
    #[doc = "VCC_EN is active low. Polarity of SVCC_EN is inverted."]
    _1,
}
impl VCCENPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VCCENPW::_0 => false,
            VCCENPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VCCENPW<'a> {
    w: &'a mut W,
}
impl<'a> _VCCENPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VCCENPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VCC_EN is active high. Polarity of SVCC_EN is unchanged."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCCENPW::_0)
    }
    #[doc = "VCC_EN is active low. Polarity of SVCC_EN is inverted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCCENPW::_1)
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
#[doc = "Values that can be written to the field `SRST`"]
pub enum SRSTW {
    #[doc = "Smart Card Reset is asserted (default)"]
    _0,
    #[doc = "Smart Card Reset is de-asserted"]
    _1,
}
impl SRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRSTW::_0 => false,
            SRSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Smart Card Reset is asserted (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRSTW::_0)
    }
    #[doc = "Smart Card Reset is de-asserted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRSTW::_1)
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
#[doc = "Values that can be written to the field `SCEN`"]
pub enum SCENW {
    #[doc = "Smart Card Clock Disabled"]
    _0,
    #[doc = "Smart Card Clock Enabled"]
    _1,
}
impl SCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCENW::_0 => false,
            SCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Smart Card Clock Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCENW::_0)
    }
    #[doc = "Smart Card Clock Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCENW::_1)
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
#[doc = "Values that can be written to the field `SCSP`"]
pub enum SCSPW {
    #[doc = "Clock is logic 0 when stopped by SCEN"]
    _0,
    #[doc = "Clock is logic 1 when stopped by SCEN"]
    _1,
}
impl SCSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCSPW::_0 => false,
            SCSPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCSPW<'a> {
    w: &'a mut W,
}
impl<'a> _SCSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock is logic 0 when stopped by SCEN"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCSPW::_0)
    }
    #[doc = "Clock is logic 1 when stopped by SCEN"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCSPW::_1)
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
#[doc = "Values that can be written to the field `SPD`"]
pub enum SPDW {
    #[doc = "No effect (default)"]
    _0,
    #[doc = "Start Auto Powerdown or Power Down is in progress"]
    _1,
}
impl SPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPDW::_0 => false,
            SPDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDW::_0)
    }
    #[doc = "Start Auto Powerdown or Power Down is in progress"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDW::_1)
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
#[doc = "Values that can be written to the field `SPDIM`"]
pub enum SPDIMW {
    #[doc = "SIM presence detect interrupt is enabled"]
    _0,
    #[doc = "SIM presence detect interrupt is masked (default)"]
    _1,
}
impl SPDIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPDIMW::_0 => false,
            SPDIMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SIM presence detect interrupt is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDIMW::_0)
    }
    #[doc = "SIM presence detect interrupt is masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDIMW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPDIF`"]
pub enum SPDIFW {
    #[doc = "No insertion or removal of Smart Card detected on Port (default)"]
    _0,
    #[doc = "Insertion or removal of Smart Card detected on Port"]
    _1,
}
impl SPDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPDIFW::_0 => false,
            SPDIFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No insertion or removal of Smart Card detected on Port (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDIFW::_0)
    }
    #[doc = "Insertion or removal of Smart Card detected on Port"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDIFW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPDES`"]
pub enum SPDESW {
    #[doc = "Falling edge on the pin (default)"]
    _0,
    #[doc = "Rising edge on the pin"]
    _1,
}
impl SPDESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPDESW::_0 => false,
            SPDESW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDESW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge on the pin (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDESW::_0)
    }
    #[doc = "Rising edge on the pin"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDESW::_1)
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Auto Power Down Enable"]
    #[inline]
    pub fn sapd(&self) -> SAPDR {
        SAPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Vcc Enable for Smart Card"]
    #[inline]
    pub fn svcc_en(&self) -> SVCC_ENR {
        SVCC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - VCC Enable Polarity Control"]
    #[inline]
    pub fn vccenp(&self) -> VCCENPR {
        VCCENPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Reset to Smart Card"]
    #[inline]
    pub fn srst(&self) -> SRSTR {
        SRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Clock Enable for Smart Card"]
    #[inline]
    pub fn scen(&self) -> SCENR {
        SCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Smart Card Clock Stop Polarity"]
    #[inline]
    pub fn scsp(&self) -> SCSPR {
        SCSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Auto Power Down Control"]
    #[inline]
    pub fn spd(&self) -> SPDR {
        SPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Smart Card Presence Detect Interrupt Mask"]
    #[inline]
    pub fn spdim(&self) -> SPDIMR {
        SPDIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Smart Card Presence Detect Interrupt Flag"]
    #[inline]
    pub fn spdif(&self) -> SPDIFR {
        SPDIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Smart Card Presence Detect Pin Status"]
    #[inline]
    pub fn spdp(&self) -> SPDPR {
        SPDPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - SIM Presence Detect Edge Select"]
    #[inline]
    pub fn spdes(&self) -> SPDESR {
        SPDESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16777216 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Auto Power Down Enable"]
    #[inline]
    pub fn sapd(&mut self) -> _SAPDW {
        _SAPDW { w: self }
    }
    #[doc = "Bit 1 - Vcc Enable for Smart Card"]
    #[inline]
    pub fn svcc_en(&mut self) -> _SVCC_ENW {
        _SVCC_ENW { w: self }
    }
    #[doc = "Bit 2 - VCC Enable Polarity Control"]
    #[inline]
    pub fn vccenp(&mut self) -> _VCCENPW {
        _VCCENPW { w: self }
    }
    #[doc = "Bit 3 - Reset to Smart Card"]
    #[inline]
    pub fn srst(&mut self) -> _SRSTW {
        _SRSTW { w: self }
    }
    #[doc = "Bit 4 - Clock Enable for Smart Card"]
    #[inline]
    pub fn scen(&mut self) -> _SCENW {
        _SCENW { w: self }
    }
    #[doc = "Bit 5 - Smart Card Clock Stop Polarity"]
    #[inline]
    pub fn scsp(&mut self) -> _SCSPW {
        _SCSPW { w: self }
    }
    #[doc = "Bit 7 - Auto Power Down Control"]
    #[inline]
    pub fn spd(&mut self) -> _SPDW {
        _SPDW { w: self }
    }
    #[doc = "Bit 24 - Smart Card Presence Detect Interrupt Mask"]
    #[inline]
    pub fn spdim(&mut self) -> _SPDIMW {
        _SPDIMW { w: self }
    }
    #[doc = "Bit 25 - Smart Card Presence Detect Interrupt Flag"]
    #[inline]
    pub fn spdif(&mut self) -> _SPDIFW {
        _SPDIFW { w: self }
    }
    #[doc = "Bit 27 - SIM Presence Detect Edge Select"]
    #[inline]
    pub fn spdes(&mut self) -> _SPDESW {
        _SPDESW { w: self }
    }
}
