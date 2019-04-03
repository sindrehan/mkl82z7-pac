#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TX_STATUS {
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
#[doc = "Possible values of the field `TNTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNTER {
    #[doc = "Transmit NACK threshold has not been reached (default)"]
    _0,
    #[doc = "Transmit NACK threshold reached; transmitter frozen"]
    _1,
}
impl TNTER {
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
            TNTER::_0 => false,
            TNTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNTER {
        match value {
            false => TNTER::_0,
            true => TNTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TNTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TNTER::_1
    }
}
#[doc = "Possible values of the field `TFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFER {
    #[doc = "Transmit FIFO is not empty"]
    _0,
    #[doc = "Transmit FIFO is empty (default)"]
    _1,
}
impl TFER {
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
            TFER::_0 => false,
            TFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFER {
        match value {
            false => TFER::_0,
            true => TFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFER::_1
    }
}
#[doc = "Possible values of the field `ETCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETCFR {
    #[doc = "Transmit pending or in progress"]
    _0,
    #[doc = "Transmit complete (default)"]
    _1,
}
impl ETCFR {
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
            ETCFR::_0 => false,
            ETCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETCFR {
        match value {
            false => ETCFR::_0,
            true => ETCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ETCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ETCFR::_1
    }
}
#[doc = "Possible values of the field `TCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCFR {
    #[doc = "Transmit pending or in progress"]
    _0,
    #[doc = "Transmit complete (default)"]
    _1,
}
impl TCFR {
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
            TCFR::_0 => false,
            TCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCFR {
        match value {
            false => TCFR::_0,
            true => TCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCFR::_1
    }
}
#[doc = "Possible values of the field `TFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFR {
    #[doc = "Transmit FIFO Full condition has not occurred (default)"]
    _0,
    #[doc = "A Transmit FIFO Full condition has occurred"]
    _1,
}
impl TFFR {
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
            TFFR::_0 => false,
            TFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFFR {
        match value {
            false => TFFR::_0,
            true => TFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFFR::_1
    }
}
#[doc = "Possible values of the field `TDTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDTFR {
    #[doc = "Number of bytes in FIFO is greater than TDT\\[3:0\\], or bit has been cleared"]
    _0,
    #[doc = "Number of bytes in FIFO is less than or equal to TDT\\[3:0\\] (default)"]
    _1,
}
impl TDTFR {
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
            TDTFR::_0 => false,
            TDTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDTFR {
        match value {
            false => TDTFR::_0,
            true => TDTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDTFR::_1
    }
}
#[doc = "Possible values of the field `GPCNT0_TO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT0_TOR {
    #[doc = "GPCNT0_VAL time not reached, or bit has been cleared. (default)"]
    _0,
    #[doc = "General Purpose counter has reached the GPCNT0_VAL value"]
    _1,
}
impl GPCNT0_TOR {
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
            GPCNT0_TOR::_0 => false,
            GPCNT0_TOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPCNT0_TOR {
        match value {
            false => GPCNT0_TOR::_0,
            true => GPCNT0_TOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GPCNT0_TOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GPCNT0_TOR::_1
    }
}
#[doc = "Possible values of the field `GPCNT1_TO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT1_TOR {
    #[doc = "GPCNT1_VAL time not reached, or bit has been cleared. (default)"]
    _0,
    #[doc = "General Purpose counter has reached the GPCNT1_VAL value"]
    _1,
}
impl GPCNT1_TOR {
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
            GPCNT1_TOR::_0 => false,
            GPCNT1_TOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPCNT1_TOR {
        match value {
            false => GPCNT1_TOR::_0,
            true => GPCNT1_TOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GPCNT1_TOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GPCNT1_TOR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TX_RPTRR {
    bits: u8,
}
impl TX_RPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TX_CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CNTR {
    #[doc = "FIFO is emtpy"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TX_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_CNTR::_0 => 0,
            TX_CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_CNTR {
        match value {
            0 => TX_CNTR::_0,
            i => TX_CNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_CNTR::_0
    }
}
#[doc = "Values that can be written to the field `TNTE`"]
pub enum TNTEW {
    #[doc = "Transmit NACK threshold has not been reached (default)"]
    _0,
    #[doc = "Transmit NACK threshold reached; transmitter frozen"]
    _1,
}
impl TNTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNTEW::_0 => false,
            TNTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNTEW<'a> {
    w: &'a mut W,
}
impl<'a> _TNTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit NACK threshold has not been reached (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TNTEW::_0)
    }
    #[doc = "Transmit NACK threshold reached; transmitter frozen"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TNTEW::_1)
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
#[doc = "Values that can be written to the field `TFE`"]
pub enum TFEW {
    #[doc = "Transmit FIFO is not empty"]
    _0,
    #[doc = "Transmit FIFO is empty (default)"]
    _1,
}
impl TFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFEW::_0 => false,
            TFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO is not empty"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFEW::_0)
    }
    #[doc = "Transmit FIFO is empty (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFEW::_1)
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
#[doc = "Values that can be written to the field `ETCF`"]
pub enum ETCFW {
    #[doc = "Transmit pending or in progress"]
    _0,
    #[doc = "Transmit complete (default)"]
    _1,
}
impl ETCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETCFW::_0 => false,
            ETCFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETCFW<'a> {
    w: &'a mut W,
}
impl<'a> _ETCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit pending or in progress"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETCFW::_0)
    }
    #[doc = "Transmit complete (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETCFW::_1)
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
#[doc = "Values that can be written to the field `TCF`"]
pub enum TCFW {
    #[doc = "Transmit pending or in progress"]
    _0,
    #[doc = "Transmit complete (default)"]
    _1,
}
impl TCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCFW::_0 => false,
            TCFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit pending or in progress"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFW::_0)
    }
    #[doc = "Transmit complete (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFW::_1)
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
#[doc = "Values that can be written to the field `TFF`"]
pub enum TFFW {
    #[doc = "Transmit FIFO Full condition has not occurred (default)"]
    _0,
    #[doc = "A Transmit FIFO Full condition has occurred"]
    _1,
}
impl TFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFFW::_0 => false,
            TFFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFFW<'a> {
    w: &'a mut W,
}
impl<'a> _TFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO Full condition has not occurred (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFW::_0)
    }
    #[doc = "A Transmit FIFO Full condition has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFW::_1)
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
#[doc = "Values that can be written to the field `GPCNT0_TO`"]
pub enum GPCNT0_TOW {
    #[doc = "GPCNT0_VAL time not reached, or bit has been cleared. (default)"]
    _0,
    #[doc = "General Purpose counter has reached the GPCNT0_VAL value"]
    _1,
}
impl GPCNT0_TOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPCNT0_TOW::_0 => false,
            GPCNT0_TOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPCNT0_TOW<'a> {
    w: &'a mut W,
}
impl<'a> _GPCNT0_TOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPCNT0_TOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPCNT0_VAL time not reached, or bit has been cleared. (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPCNT0_TOW::_0)
    }
    #[doc = "General Purpose counter has reached the GPCNT0_VAL value"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPCNT0_TOW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPCNT1_TO`"]
pub enum GPCNT1_TOW {
    #[doc = "GPCNT1_VAL time not reached, or bit has been cleared. (default)"]
    _0,
    #[doc = "General Purpose counter has reached the GPCNT1_VAL value"]
    _1,
}
impl GPCNT1_TOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPCNT1_TOW::_0 => false,
            GPCNT1_TOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPCNT1_TOW<'a> {
    w: &'a mut W,
}
impl<'a> _GPCNT1_TOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPCNT1_TOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPCNT1_VAL time not reached, or bit has been cleared. (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPCNT1_TOW::_0)
    }
    #[doc = "General Purpose counter has reached the GPCNT1_VAL value"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPCNT1_TOW::_1)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Transmit NACK Threshold Error Flag"]
    #[inline]
    pub fn tnte(&self) -> TNTER {
        TNTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit FIFO Empty Flag"]
    #[inline]
    pub fn tfe(&self) -> TFER {
        TFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Early Transmit Complete Flag"]
    #[inline]
    pub fn etcf(&self) -> ETCFR {
        ETCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Transmit Complete Flag"]
    #[inline]
    pub fn tcf(&self) -> TCFR {
        TCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmit FIFO Full Flag"]
    #[inline]
    pub fn tff(&self) -> TFFR {
        TFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transmit Data Threshold Flag"]
    #[inline]
    pub fn tdtf(&self) -> TDTFR {
        TDTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - General Purpose Counter 0 Timeout Flag"]
    #[inline]
    pub fn gpcnt0_to(&self) -> GPCNT0_TOR {
        GPCNT0_TOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - General Purpose Counter 1 Timeout Flag"]
    #[inline]
    pub fn gpcnt1_to(&self) -> GPCNT1_TOR {
        GPCNT1_TOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Transmit FIFO Read Pointer"]
    #[inline]
    pub fn tx_rptr(&self) -> TX_RPTRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TX_RPTRR { bits }
    }
    #[doc = "Bits 24:31 - Transmit FIFO Byte Count"]
    #[inline]
    pub fn tx_cnt(&self) -> TX_CNTR {
        TX_CNTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 184 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit NACK Threshold Error Flag"]
    #[inline]
    pub fn tnte(&mut self) -> _TNTEW {
        _TNTEW { w: self }
    }
    #[doc = "Bit 3 - Transmit FIFO Empty Flag"]
    #[inline]
    pub fn tfe(&mut self) -> _TFEW {
        _TFEW { w: self }
    }
    #[doc = "Bit 4 - Early Transmit Complete Flag"]
    #[inline]
    pub fn etcf(&mut self) -> _ETCFW {
        _ETCFW { w: self }
    }
    #[doc = "Bit 5 - Transmit Complete Flag"]
    #[inline]
    pub fn tcf(&mut self) -> _TCFW {
        _TCFW { w: self }
    }
    #[doc = "Bit 6 - Transmit FIFO Full Flag"]
    #[inline]
    pub fn tff(&mut self) -> _TFFW {
        _TFFW { w: self }
    }
    #[doc = "Bit 8 - General Purpose Counter 0 Timeout Flag"]
    #[inline]
    pub fn gpcnt0_to(&mut self) -> _GPCNT0_TOW {
        _GPCNT0_TOW { w: self }
    }
    #[doc = "Bit 9 - General Purpose Counter 1 Timeout Flag"]
    #[inline]
    pub fn gpcnt1_to(&mut self) -> _GPCNT1_TOW {
        _GPCNT1_TOW { w: self }
    }
}
