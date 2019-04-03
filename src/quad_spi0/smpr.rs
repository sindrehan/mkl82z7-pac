#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMPR {
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
#[doc = "Possible values of the field `HSENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSENAR {
    #[doc = "Disable divide by 2 of serial flash clock for half speed commands"]
    _0,
    #[doc = "Enable divide by 2 of serial flash clock for half speed commands"]
    _1,
}
impl HSENAR {
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
            HSENAR::_0 => false,
            HSENAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSENAR {
        match value {
            false => HSENAR::_0,
            true => HSENAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HSENAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HSENAR::_1
    }
}
#[doc = "Possible values of the field `HSPHS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPHSR {
    #[doc = "Select sampling at non-inverted clock"]
    _0,
    #[doc = "Select sampling at inverted clock"]
    _1,
}
impl HSPHSR {
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
            HSPHSR::_0 => false,
            HSPHSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSPHSR {
        match value {
            false => HSPHSR::_0,
            true => HSPHSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HSPHSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HSPHSR::_1
    }
}
#[doc = "Possible values of the field `HSDLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSDLYR {
    #[doc = "One clock cycle delay"]
    _0,
    #[doc = "Two clock cycle delay"]
    _1,
}
impl HSDLYR {
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
            HSDLYR::_0 => false,
            HSDLYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSDLYR {
        match value {
            false => HSDLYR::_0,
            true => HSDLYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HSDLYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HSDLYR::_1
    }
}
#[doc = "Possible values of the field `FSPHS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPHSR {
    #[doc = "Select sampling at non-inverted clock"]
    _0,
    #[doc = "Select sampling at inverted clock. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    _1,
}
impl FSPHSR {
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
            FSPHSR::_0 => false,
            FSPHSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSPHSR {
        match value {
            false => FSPHSR::_0,
            true => FSPHSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSPHSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSPHSR::_1
    }
}
#[doc = "Possible values of the field `FSDLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSDLYR {
    #[doc = "One clock cycle delay"]
    _0,
    #[doc = "Two clock cycles delay. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    _1,
}
impl FSDLYR {
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
            FSDLYR::_0 => false,
            FSDLYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSDLYR {
        match value {
            false => FSDLYR::_0,
            true => FSDLYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSDLYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSDLYR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DDRSMPR {
    bits: u8,
}
impl DDRSMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `HSENA`"]
pub enum HSENAW {
    #[doc = "Disable divide by 2 of serial flash clock for half speed commands"]
    _0,
    #[doc = "Enable divide by 2 of serial flash clock for half speed commands"]
    _1,
}
impl HSENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSENAW::_0 => false,
            HSENAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSENAW<'a> {
    w: &'a mut W,
}
impl<'a> _HSENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable divide by 2 of serial flash clock for half speed commands"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSENAW::_0)
    }
    #[doc = "Enable divide by 2 of serial flash clock for half speed commands"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSENAW::_1)
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
#[doc = "Values that can be written to the field `HSPHS`"]
pub enum HSPHSW {
    #[doc = "Select sampling at non-inverted clock"]
    _0,
    #[doc = "Select sampling at inverted clock"]
    _1,
}
impl HSPHSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSPHSW::_0 => false,
            HSPHSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSPHSW<'a> {
    w: &'a mut W,
}
impl<'a> _HSPHSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSPHSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select sampling at non-inverted clock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSPHSW::_0)
    }
    #[doc = "Select sampling at inverted clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSPHSW::_1)
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
#[doc = "Values that can be written to the field `HSDLY`"]
pub enum HSDLYW {
    #[doc = "One clock cycle delay"]
    _0,
    #[doc = "Two clock cycle delay"]
    _1,
}
impl HSDLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSDLYW::_0 => false,
            HSDLYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _HSDLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSDLYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One clock cycle delay"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSDLYW::_0)
    }
    #[doc = "Two clock cycle delay"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSDLYW::_1)
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
#[doc = "Values that can be written to the field `FSPHS`"]
pub enum FSPHSW {
    #[doc = "Select sampling at non-inverted clock"]
    _0,
    #[doc = "Select sampling at inverted clock. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    _1,
}
impl FSPHSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSPHSW::_0 => false,
            FSPHSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSPHSW<'a> {
    w: &'a mut W,
}
impl<'a> _FSPHSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSPHSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select sampling at non-inverted clock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSPHSW::_0)
    }
    #[doc = "Select sampling at inverted clock. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSPHSW::_1)
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
#[doc = "Values that can be written to the field `FSDLY`"]
pub enum FSDLYW {
    #[doc = "One clock cycle delay"]
    _0,
    #[doc = "Two clock cycles delay. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    _1,
}
impl FSDLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSDLYW::_0 => false,
            FSDLYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSDLYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One clock cycle delay"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSDLYW::_0)
    }
    #[doc = "Two clock cycles delay. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSDLYW::_1)
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
#[doc = r" Proxy"]
pub struct _DDRSMPW<'a> {
    w: &'a mut W,
}
impl<'a> _DDRSMPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Half Speed serial flash clock Enable"]
    #[inline]
    pub fn hsena(&self) -> HSENAR {
        HSENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Half Speed Phase selection for SDR instructions."]
    #[inline]
    pub fn hsphs(&self) -> HSPHSR {
        HSPHSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Half Speed Delay selection for SDR instructions."]
    #[inline]
    pub fn hsdly(&self) -> HSDLYR {
        HSDLYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Full Speed Phase selection for SDR instructions."]
    #[inline]
    pub fn fsphs(&self) -> FSPHSR {
        FSPHSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands."]
    #[inline]
    pub fn fsdly(&self) -> FSDLYR {
        FSDLYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - DDR Sampling point"]
    #[inline]
    pub fn ddrsmp(&self) -> DDRSMPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DDRSMPR { bits }
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
    #[doc = "Bit 0 - Half Speed serial flash clock Enable"]
    #[inline]
    pub fn hsena(&mut self) -> _HSENAW {
        _HSENAW { w: self }
    }
    #[doc = "Bit 1 - Half Speed Phase selection for SDR instructions."]
    #[inline]
    pub fn hsphs(&mut self) -> _HSPHSW {
        _HSPHSW { w: self }
    }
    #[doc = "Bit 2 - Half Speed Delay selection for SDR instructions."]
    #[inline]
    pub fn hsdly(&mut self) -> _HSDLYW {
        _HSDLYW { w: self }
    }
    #[doc = "Bit 5 - Full Speed Phase selection for SDR instructions."]
    #[inline]
    pub fn fsphs(&mut self) -> _FSPHSW {
        _FSPHSW { w: self }
    }
    #[doc = "Bit 6 - Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands."]
    #[inline]
    pub fn fsdly(&mut self) -> _FSDLYW {
        _FSDLYW { w: self }
    }
    #[doc = "Bits 16:18 - DDR Sampling point"]
    #[inline]
    pub fn ddrsmp(&mut self) -> _DDRSMPW {
        _DDRSMPW { w: self }
    }
}
