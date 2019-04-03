#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
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
#[doc = "Possible values of the field `SOFDYNTHLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFDYNTHLDR {
    #[doc = "SOF_TOK interrupt is set when byte times SOF threshold is reached."]
    _0,
    #[doc = "SOF_TOK interrupt is set when 8 byte times SOF threshold is reached or overstepped."]
    _1,
}
impl SOFDYNTHLDR {
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
            SOFDYNTHLDR::_0 => false,
            SOFDYNTHLDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFDYNTHLDR {
        match value {
            false => SOFDYNTHLDR::_0,
            true => SOFDYNTHLDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOFDYNTHLDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOFDYNTHLDR::_1
    }
}
#[doc = "Possible values of the field `SOFBUSSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFBUSSETR {
    #[doc = "SOF_TOK interrupt is set according to SOF threshold value."]
    _0,
    #[doc = "SOF_TOK interrupt is set when SOF counter reaches 0."]
    _1,
}
impl SOFBUSSETR {
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
            SOFBUSSETR::_0 => false,
            SOFBUSSETR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFBUSSETR {
        match value {
            false => SOFBUSSETR::_0,
            true => SOFBUSSETR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOFBUSSETR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOFBUSSETR::_1
    }
}
#[doc = "Possible values of the field `OWNERRISODIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWNERRISODISR {
    #[doc = "OWN error detect for ISO IN / ISO OUT is not disabled."]
    _0,
    #[doc = "OWN error detect for ISO IN / ISO OUT is disabled."]
    _1,
}
impl OWNERRISODISR {
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
            OWNERRISODISR::_0 => false,
            OWNERRISODISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OWNERRISODISR {
        match value {
            false => OWNERRISODISR::_0,
            true => OWNERRISODISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OWNERRISODISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OWNERRISODISR::_1
    }
}
#[doc = "Possible values of the field `VREDG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREDG_ENR {
    #[doc = "VREGIN rising edge interrupt disabled."]
    _0,
    #[doc = "VREGIN rising edge interrupt enabled."]
    _1,
}
impl VREDG_ENR {
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
            VREDG_ENR::_0 => false,
            VREDG_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VREDG_ENR {
        match value {
            false => VREDG_ENR::_0,
            true => VREDG_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VREDG_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VREDG_ENR::_1
    }
}
#[doc = "Possible values of the field `VFEDG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VFEDG_ENR {
    #[doc = "VREGIN falling edge interrupt disabled."]
    _0,
    #[doc = "VREGIN falling edge interrupt enabled."]
    _1,
}
impl VFEDG_ENR {
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
            VFEDG_ENR::_0 => false,
            VFEDG_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VFEDG_ENR {
        match value {
            false => VFEDG_ENR::_0,
            true => VFEDG_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VFEDG_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VFEDG_ENR::_1
    }
}
#[doc = "Possible values of the field `STL_ADJ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STL_ADJ_ENR {
    #[doc = "If USB_ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint will be stalled"]
    _0,
    #[doc = "If USB_ENDPTn\\[END_STALL\\] = 1, the USB_STALL_xx_DIS registers control which directions for the associated endpoint will be stalled."]
    _1,
}
impl STL_ADJ_ENR {
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
            STL_ADJ_ENR::_0 => false,
            STL_ADJ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STL_ADJ_ENR {
        match value {
            false => STL_ADJ_ENR::_0,
            true => STL_ADJ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STL_ADJ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STL_ADJ_ENR::_1
    }
}
#[doc = "Values that can be written to the field `SOFDYNTHLD`"]
pub enum SOFDYNTHLDW {
    #[doc = "SOF_TOK interrupt is set when byte times SOF threshold is reached."]
    _0,
    #[doc = "SOF_TOK interrupt is set when 8 byte times SOF threshold is reached or overstepped."]
    _1,
}
impl SOFDYNTHLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFDYNTHLDW::_0 => false,
            SOFDYNTHLDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFDYNTHLDW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFDYNTHLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFDYNTHLDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOF_TOK interrupt is set when byte times SOF threshold is reached."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFDYNTHLDW::_0)
    }
    #[doc = "SOF_TOK interrupt is set when 8 byte times SOF threshold is reached or overstepped."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFDYNTHLDW::_1)
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
#[doc = "Values that can be written to the field `SOFBUSSET`"]
pub enum SOFBUSSETW {
    #[doc = "SOF_TOK interrupt is set according to SOF threshold value."]
    _0,
    #[doc = "SOF_TOK interrupt is set when SOF counter reaches 0."]
    _1,
}
impl SOFBUSSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFBUSSETW::_0 => false,
            SOFBUSSETW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFBUSSETW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFBUSSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFBUSSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOF_TOK interrupt is set according to SOF threshold value."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFBUSSETW::_0)
    }
    #[doc = "SOF_TOK interrupt is set when SOF counter reaches 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFBUSSETW::_1)
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
#[doc = "Values that can be written to the field `OWNERRISODIS`"]
pub enum OWNERRISODISW {
    #[doc = "OWN error detect for ISO IN / ISO OUT is not disabled."]
    _0,
    #[doc = "OWN error detect for ISO IN / ISO OUT is disabled."]
    _1,
}
impl OWNERRISODISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OWNERRISODISW::_0 => false,
            OWNERRISODISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OWNERRISODISW<'a> {
    w: &'a mut W,
}
impl<'a> _OWNERRISODISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OWNERRISODISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OWN error detect for ISO IN / ISO OUT is not disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OWNERRISODISW::_0)
    }
    #[doc = "OWN error detect for ISO IN / ISO OUT is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OWNERRISODISW::_1)
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
#[doc = "Values that can be written to the field `VREDG_EN`"]
pub enum VREDG_ENW {
    #[doc = "VREGIN rising edge interrupt disabled."]
    _0,
    #[doc = "VREGIN rising edge interrupt enabled."]
    _1,
}
impl VREDG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VREDG_ENW::_0 => false,
            VREDG_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VREDG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VREDG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VREDG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VREGIN rising edge interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREDG_ENW::_0)
    }
    #[doc = "VREGIN rising edge interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREDG_ENW::_1)
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
#[doc = "Values that can be written to the field `VFEDG_EN`"]
pub enum VFEDG_ENW {
    #[doc = "VREGIN falling edge interrupt disabled."]
    _0,
    #[doc = "VREGIN falling edge interrupt enabled."]
    _1,
}
impl VFEDG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VFEDG_ENW::_0 => false,
            VFEDG_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VFEDG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VFEDG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VFEDG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VREGIN falling edge interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VFEDG_ENW::_0)
    }
    #[doc = "VREGIN falling edge interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VFEDG_ENW::_1)
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
#[doc = "Values that can be written to the field `STL_ADJ_EN`"]
pub enum STL_ADJ_ENW {
    #[doc = "If USB_ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint will be stalled"]
    _0,
    #[doc = "If USB_ENDPTn\\[END_STALL\\] = 1, the USB_STALL_xx_DIS registers control which directions for the associated endpoint will be stalled."]
    _1,
}
impl STL_ADJ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STL_ADJ_ENW::_0 => false,
            STL_ADJ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STL_ADJ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _STL_ADJ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STL_ADJ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If USB_ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint will be stalled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STL_ADJ_ENW::_0)
    }
    #[doc = "If USB_ENDPTn\\[END_STALL\\] = 1, the USB_STALL_xx_DIS registers control which directions for the associated endpoint will be stalled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STL_ADJ_ENW::_1)
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
    #[doc = "Bit 0 - Dynamic SOF Threshold Compare mode"]
    #[inline]
    pub fn sofdynthld(&self) -> SOFDYNTHLDR {
        SOFDYNTHLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - SOF_TOK Interrupt Generation Mode Select"]
    #[inline]
    pub fn sofbusset(&self) -> SOFBUSSETR {
        SOFBUSSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - OWN Error Detect for ISO IN / ISO OUT Disable"]
    #[inline]
    pub fn ownerrisodis(&self) -> OWNERRISODISR {
        OWNERRISODISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Enable"]
    #[inline]
    pub fn vredg_en(&self) -> VREDG_ENR {
        VREDG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Enable"]
    #[inline]
    pub fn vfedg_en(&self) -> VFEDG_ENR {
        VFEDG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - USB Peripheral mode Stall Adjust Enable"]
    #[inline]
    pub fn stl_adj_en(&self) -> STL_ADJ_ENR {
        STL_ADJ_ENR::_from({
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
    #[doc = "Bit 0 - Dynamic SOF Threshold Compare mode"]
    #[inline]
    pub fn sofdynthld(&mut self) -> _SOFDYNTHLDW {
        _SOFDYNTHLDW { w: self }
    }
    #[doc = "Bit 1 - SOF_TOK Interrupt Generation Mode Select"]
    #[inline]
    pub fn sofbusset(&mut self) -> _SOFBUSSETW {
        _SOFBUSSETW { w: self }
    }
    #[doc = "Bit 2 - OWN Error Detect for ISO IN / ISO OUT Disable"]
    #[inline]
    pub fn ownerrisodis(&mut self) -> _OWNERRISODISW {
        _OWNERRISODISW { w: self }
    }
    #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Enable"]
    #[inline]
    pub fn vredg_en(&mut self) -> _VREDG_ENW {
        _VREDG_ENW { w: self }
    }
    #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Enable"]
    #[inline]
    pub fn vfedg_en(&mut self) -> _VFEDG_ENW {
        _VFEDG_ENW { w: self }
    }
    #[doc = "Bit 7 - USB Peripheral mode Stall Adjust Enable"]
    #[inline]
    pub fn stl_adj_en(&mut self) -> _STL_ADJ_ENW {
        _STL_ADJ_ENW { w: self }
    }
}
