#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `IC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR {
    #[doc = "Direction convention transfers enabled (default)"]
    _0,
    #[doc = "Inverse convention transfers enabled"]
    _1,
}
impl ICR {
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
            ICR::_0 => false,
            ICR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICR {
        match value {
            false => ICR::_0,
            true => ICR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ICR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ICR::_1
    }
}
#[doc = "Possible values of the field `ICM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICMR {
    #[doc = "Initial Character Mode disabled"]
    _0,
    #[doc = "Initial Character Mode enabled (default)"]
    _1,
}
impl ICMR {
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
            ICMR::_0 => false,
            ICMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICMR {
        match value {
            false => ICMR::_0,
            true => ICMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ICMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ICMR::_1
    }
}
#[doc = "Possible values of the field `ANACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACKR {
    #[doc = "NACK generation on errors disabled"]
    _0,
    #[doc = "NACK generation on errors enabled (default)"]
    _1,
}
impl ANACKR {
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
            ANACKR::_0 => false,
            ANACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANACKR {
        match value {
            false => ANACKR::_0,
            true => ANACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ANACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ANACKR::_1
    }
}
#[doc = "Possible values of the field `ONACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONACKR {
    #[doc = "NACK generation on overrun is disabled (default)"]
    _0,
    #[doc = "NACK generation on overrun is enabled"]
    _1,
}
impl ONACKR {
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
            ONACKR::_0 => false,
            ONACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONACKR {
        match value {
            false => ONACKR::_0,
            true => ONACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ONACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ONACKR::_1
    }
}
#[doc = "Possible values of the field `KILL_CLOCKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KILL_CLOCKSR {
    #[doc = "EMV SIM input clock enabled (default)"]
    _0,
    #[doc = "EMV SIM input clock is disabled"]
    _1,
}
impl KILL_CLOCKSR {
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
            KILL_CLOCKSR::_0 => false,
            KILL_CLOCKSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KILL_CLOCKSR {
        match value {
            false => KILL_CLOCKSR::_0,
            true => KILL_CLOCKSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == KILL_CLOCKSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == KILL_CLOCKSR::_1
    }
}
#[doc = "Possible values of the field `DOZE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZE_ENR {
    #[doc = "DOZE instruction will gate all internal EMV SIM clocks as well as the Smart Card clock when the transmit FIFO is empty (default)"]
    _0,
    #[doc = "DOZE instruction has no effect on EMV SIM module"]
    _1,
}
impl DOZE_ENR {
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
            DOZE_ENR::_0 => false,
            DOZE_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZE_ENR {
        match value {
            false => DOZE_ENR::_0,
            true => DOZE_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DOZE_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DOZE_ENR::_1
    }
}
#[doc = "Possible values of the field `STOP_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_ENR {
    #[doc = "STOP instruction shuts down all EMV SIM clocks (default)"]
    _0,
    #[doc = "STOP instruction shuts down all clocks except for the Smart Card Clock (SCK) (clock provided to Smart Card)"]
    _1,
}
impl STOP_ENR {
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
            STOP_ENR::_0 => false,
            STOP_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOP_ENR {
        match value {
            false => STOP_ENR::_0,
            true => STOP_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STOP_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STOP_ENR::_1
    }
}
#[doc = "Possible values of the field `RCV_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCV_ENR {
    #[doc = "EMV SIM Receiver disabled (default)"]
    _0,
    #[doc = "EMV SIM Receiver enabled"]
    _1,
}
impl RCV_ENR {
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
            RCV_ENR::_0 => false,
            RCV_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCV_ENR {
        match value {
            false => RCV_ENR::_0,
            true => RCV_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCV_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCV_ENR::_1
    }
}
#[doc = "Possible values of the field `XMT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XMT_ENR {
    #[doc = "EMV SIM Transmitter disabled (default)"]
    _0,
    #[doc = "EMV SIM Transmitter enabled"]
    _1,
}
impl XMT_ENR {
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
            XMT_ENR::_0 => false,
            XMT_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XMT_ENR {
        match value {
            false => XMT_ENR::_0,
            true => XMT_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == XMT_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == XMT_ENR::_1
    }
}
#[doc = "Possible values of the field `RCVR_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCVR_11R {
    #[doc = "Receiver configured for 12 ETU operation mode (default)"]
    _0,
    #[doc = "Receiver configured for 11 ETU operation mode"]
    _1,
}
impl RCVR_11R {
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
            RCVR_11R::_0 => false,
            RCVR_11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCVR_11R {
        match value {
            false => RCVR_11R::_0,
            true => RCVR_11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCVR_11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCVR_11R::_1
    }
}
#[doc = "Possible values of the field `RX_DMA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DMA_ENR {
    #[doc = "No DMA Read Request asserted for Receiver (default)"]
    _0,
    #[doc = "DMA Read Request asserted for Receiver"]
    _1,
}
impl RX_DMA_ENR {
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
            RX_DMA_ENR::_0 => false,
            RX_DMA_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DMA_ENR {
        match value {
            false => RX_DMA_ENR::_0,
            true => RX_DMA_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DMA_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DMA_ENR::_1
    }
}
#[doc = "Possible values of the field `TX_DMA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DMA_ENR {
    #[doc = "No DMA Write Request asserted for Transmitter (default)"]
    _0,
    #[doc = "DMA Write Request asserted for Transmitter"]
    _1,
}
impl TX_DMA_ENR {
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
            TX_DMA_ENR::_0 => false,
            TX_DMA_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_DMA_ENR {
        match value {
            false => TX_DMA_ENR::_0,
            true => TX_DMA_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TX_DMA_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TX_DMA_ENR::_1
    }
}
#[doc = "Possible values of the field `INV_CRC_VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_CRC_VALR {
    #[doc = "Bits in CRC Output value will not be inverted."]
    _0,
    #[doc = "Bits in CRC Output value will be inverted. (default)"]
    _1,
}
impl INV_CRC_VALR {
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
            INV_CRC_VALR::_0 => false,
            INV_CRC_VALR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INV_CRC_VALR {
        match value {
            false => INV_CRC_VALR::_0,
            true => INV_CRC_VALR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INV_CRC_VALR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INV_CRC_VALR::_1
    }
}
#[doc = "Possible values of the field `CRC_OUT_FLIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_OUT_FLIPR {
    #[doc = "Bits within the CRC output bytes will not be reversed i.e. 15:0 will remain 15:0 (default)"]
    _0,
    #[doc = "Bits within the CRC output bytes will be reversed i.e. 15:0 will become {8:15,0:7}"]
    _1,
}
impl CRC_OUT_FLIPR {
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
            CRC_OUT_FLIPR::_0 => false,
            CRC_OUT_FLIPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_OUT_FLIPR {
        match value {
            false => CRC_OUT_FLIPR::_0,
            true => CRC_OUT_FLIPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_OUT_FLIPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_OUT_FLIPR::_1
    }
}
#[doc = "Possible values of the field `CRC_IN_FLIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_IN_FLIPR {
    #[doc = "Bits in the input byte will not be reversed (i.e. 7:0 will remain 7:0) before the CRC calculation (default)"]
    _0,
    #[doc = "Bits in the input byte will be reversed (i.e. 7:0 will become 0:7) before CRC calculation"]
    _1,
}
impl CRC_IN_FLIPR {
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
            CRC_IN_FLIPR::_0 => false,
            CRC_IN_FLIPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_IN_FLIPR {
        match value {
            false => CRC_IN_FLIPR::_0,
            true => CRC_IN_FLIPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_IN_FLIPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_IN_FLIPR::_1
    }
}
#[doc = "Possible values of the field `CWT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWT_ENR {
    #[doc = "Character Wait time Counter is disabled (default)"]
    _0,
    #[doc = "Character Wait time counter is enabled"]
    _1,
}
impl CWT_ENR {
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
            CWT_ENR::_0 => false,
            CWT_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CWT_ENR {
        match value {
            false => CWT_ENR::_0,
            true => CWT_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CWT_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CWT_ENR::_1
    }
}
#[doc = "Possible values of the field `LRC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC_ENR {
    #[doc = "8-bit Linear Redundancy Checking disabled (default)"]
    _0,
    #[doc = "8-bit Linear Redundancy Checking enabled"]
    _1,
}
impl LRC_ENR {
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
            LRC_ENR::_0 => false,
            LRC_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRC_ENR {
        match value {
            false => LRC_ENR::_0,
            true => LRC_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LRC_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LRC_ENR::_1
    }
}
#[doc = "Possible values of the field `CRC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_ENR {
    #[doc = "16-bit Cyclic Redundancy Checking disabled (default)"]
    _0,
    #[doc = "16-bit Cyclic Redundancy Checking enabled"]
    _1,
}
impl CRC_ENR {
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
            CRC_ENR::_0 => false,
            CRC_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_ENR {
        match value {
            false => CRC_ENR::_0,
            true => CRC_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_ENR::_1
    }
}
#[doc = "Possible values of the field `XMT_CRC_LRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XMT_CRC_LRCR {
    #[doc = "No CRC or LRC value is transmitted (default)"]
    _0,
    #[doc = "Transmit LRC or CRC info when FIFO empties (whichever is enabled)"]
    _1,
}
impl XMT_CRC_LRCR {
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
            XMT_CRC_LRCR::_0 => false,
            XMT_CRC_LRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XMT_CRC_LRCR {
        match value {
            false => XMT_CRC_LRCR::_0,
            true => XMT_CRC_LRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == XMT_CRC_LRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == XMT_CRC_LRCR::_1
    }
}
#[doc = "Possible values of the field `BWT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWT_ENR {
    #[doc = "Disable BWT, BGT Counters (default)"]
    _0,
    #[doc = "Enable BWT, BGT Counters"]
    _1,
}
impl BWT_ENR {
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
            BWT_ENR::_0 => false,
            BWT_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWT_ENR {
        match value {
            false => BWT_ENR::_0,
            true => BWT_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWT_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWT_ENR::_1
    }
}
#[doc = "Values that can be written to the field `IC`"]
pub enum ICW {
    #[doc = "Direction convention transfers enabled (default)"]
    _0,
    #[doc = "Inverse convention transfers enabled"]
    _1,
}
impl ICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICW::_0 => false,
            ICW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICW<'a> {
    w: &'a mut W,
}
impl<'a> _ICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Direction convention transfers enabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICW::_0)
    }
    #[doc = "Inverse convention transfers enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICW::_1)
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
#[doc = "Values that can be written to the field `ICM`"]
pub enum ICMW {
    #[doc = "Initial Character Mode disabled"]
    _0,
    #[doc = "Initial Character Mode enabled (default)"]
    _1,
}
impl ICMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICMW::_0 => false,
            ICMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICMW<'a> {
    w: &'a mut W,
}
impl<'a> _ICMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Initial Character Mode disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICMW::_0)
    }
    #[doc = "Initial Character Mode enabled (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICMW::_1)
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
#[doc = "Values that can be written to the field `ANACK`"]
pub enum ANACKW {
    #[doc = "NACK generation on errors disabled"]
    _0,
    #[doc = "NACK generation on errors enabled (default)"]
    _1,
}
impl ANACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANACKW::_0 => false,
            ANACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ANACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NACK generation on errors disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANACKW::_0)
    }
    #[doc = "NACK generation on errors enabled (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANACKW::_1)
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
#[doc = "Values that can be written to the field `ONACK`"]
pub enum ONACKW {
    #[doc = "NACK generation on overrun is disabled (default)"]
    _0,
    #[doc = "NACK generation on overrun is enabled"]
    _1,
}
impl ONACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONACKW::_0 => false,
            ONACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ONACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NACK generation on overrun is disabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONACKW::_0)
    }
    #[doc = "NACK generation on overrun is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONACKW::_1)
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
#[doc = "Values that can be written to the field `FLSH_RX`"]
pub enum FLSH_RXW {
    #[doc = "EMV SIM Receiver normal operation (default)"]
    _0,
    #[doc = "EMV SIM Receiver held in Reset"]
    _1,
}
impl FLSH_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLSH_RXW::_0 => false,
            FLSH_RXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLSH_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _FLSH_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLSH_RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EMV SIM Receiver normal operation (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLSH_RXW::_0)
    }
    #[doc = "EMV SIM Receiver held in Reset"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLSH_RXW::_1)
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
#[doc = "Values that can be written to the field `FLSH_TX`"]
pub enum FLSH_TXW {
    #[doc = "EMV SIM Transmitter normal operation (default)"]
    _0,
    #[doc = "EMV SIM Transmitter held in Reset"]
    _1,
}
impl FLSH_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLSH_TXW::_0 => false,
            FLSH_TXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLSH_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _FLSH_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLSH_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EMV SIM Transmitter normal operation (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLSH_TXW::_0)
    }
    #[doc = "EMV SIM Transmitter held in Reset"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLSH_TXW::_1)
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
#[doc = "Values that can be written to the field `SW_RST`"]
pub enum SW_RSTW {
    #[doc = "EMV SIM Normal operation (default)"]
    _0,
    #[doc = "EMV SIM held in Reset"]
    _1,
}
impl SW_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SW_RSTW::_0 => false,
            SW_RSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SW_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SW_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EMV SIM Normal operation (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SW_RSTW::_0)
    }
    #[doc = "EMV SIM held in Reset"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SW_RSTW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KILL_CLOCKS`"]
pub enum KILL_CLOCKSW {
    #[doc = "EMV SIM input clock enabled (default)"]
    _0,
    #[doc = "EMV SIM input clock is disabled"]
    _1,
}
impl KILL_CLOCKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KILL_CLOCKSW::_0 => false,
            KILL_CLOCKSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KILL_CLOCKSW<'a> {
    w: &'a mut W,
}
impl<'a> _KILL_CLOCKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KILL_CLOCKSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EMV SIM input clock enabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(KILL_CLOCKSW::_0)
    }
    #[doc = "EMV SIM input clock is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(KILL_CLOCKSW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOZE_EN`"]
pub enum DOZE_ENW {
    #[doc = "DOZE instruction will gate all internal EMV SIM clocks as well as the Smart Card clock when the transmit FIFO is empty (default)"]
    _0,
    #[doc = "DOZE instruction has no effect on EMV SIM module"]
    _1,
}
impl DOZE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZE_ENW::_0 => false,
            DOZE_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DOZE instruction will gate all internal EMV SIM clocks as well as the Smart Card clock when the transmit FIFO is empty (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZE_ENW::_0)
    }
    #[doc = "DOZE instruction has no effect on EMV SIM module"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZE_ENW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOP_EN`"]
pub enum STOP_ENW {
    #[doc = "STOP instruction shuts down all EMV SIM clocks (default)"]
    _0,
    #[doc = "STOP instruction shuts down all clocks except for the Smart Card Clock (SCK) (clock provided to Smart Card)"]
    _1,
}
impl STOP_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOP_ENW::_0 => false,
            STOP_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOP_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STOP instruction shuts down all EMV SIM clocks (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOP_ENW::_0)
    }
    #[doc = "STOP instruction shuts down all clocks except for the Smart Card Clock (SCK) (clock provided to Smart Card)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOP_ENW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCV_EN`"]
pub enum RCV_ENW {
    #[doc = "EMV SIM Receiver disabled (default)"]
    _0,
    #[doc = "EMV SIM Receiver enabled"]
    _1,
}
impl RCV_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCV_ENW::_0 => false,
            RCV_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCV_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RCV_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCV_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EMV SIM Receiver disabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCV_ENW::_0)
    }
    #[doc = "EMV SIM Receiver enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCV_ENW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XMT_EN`"]
pub enum XMT_ENW {
    #[doc = "EMV SIM Transmitter disabled (default)"]
    _0,
    #[doc = "EMV SIM Transmitter enabled"]
    _1,
}
impl XMT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XMT_ENW::_0 => false,
            XMT_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XMT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _XMT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XMT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EMV SIM Transmitter disabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(XMT_ENW::_0)
    }
    #[doc = "EMV SIM Transmitter enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(XMT_ENW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCVR_11`"]
pub enum RCVR_11W {
    #[doc = "Receiver configured for 12 ETU operation mode (default)"]
    _0,
    #[doc = "Receiver configured for 11 ETU operation mode"]
    _1,
}
impl RCVR_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCVR_11W::_0 => false,
            RCVR_11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCVR_11W<'a> {
    w: &'a mut W,
}
impl<'a> _RCVR_11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCVR_11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver configured for 12 ETU operation mode (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCVR_11W::_0)
    }
    #[doc = "Receiver configured for 11 ETU operation mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCVR_11W::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_DMA_EN`"]
pub enum RX_DMA_ENW {
    #[doc = "No DMA Read Request asserted for Receiver (default)"]
    _0,
    #[doc = "DMA Read Request asserted for Receiver"]
    _1,
}
impl RX_DMA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_DMA_ENW::_0 => false,
            RX_DMA_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DMA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DMA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DMA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DMA Read Request asserted for Receiver (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DMA_ENW::_0)
    }
    #[doc = "DMA Read Request asserted for Receiver"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DMA_ENW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_DMA_EN`"]
pub enum TX_DMA_ENW {
    #[doc = "No DMA Write Request asserted for Transmitter (default)"]
    _0,
    #[doc = "DMA Write Request asserted for Transmitter"]
    _1,
}
impl TX_DMA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_DMA_ENW::_0 => false,
            TX_DMA_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DMA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DMA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DMA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DMA Write Request asserted for Transmitter (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_DMA_ENW::_0)
    }
    #[doc = "DMA Write Request asserted for Transmitter"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_DMA_ENW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INV_CRC_VAL`"]
pub enum INV_CRC_VALW {
    #[doc = "Bits in CRC Output value will not be inverted."]
    _0,
    #[doc = "Bits in CRC Output value will be inverted. (default)"]
    _1,
}
impl INV_CRC_VALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INV_CRC_VALW::_0 => false,
            INV_CRC_VALW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INV_CRC_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _INV_CRC_VALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INV_CRC_VALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bits in CRC Output value will not be inverted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_CRC_VALW::_0)
    }
    #[doc = "Bits in CRC Output value will be inverted. (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_CRC_VALW::_1)
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
#[doc = "Values that can be written to the field `CRC_OUT_FLIP`"]
pub enum CRC_OUT_FLIPW {
    #[doc = "Bits within the CRC output bytes will not be reversed i.e. 15:0 will remain 15:0 (default)"]
    _0,
    #[doc = "Bits within the CRC output bytes will be reversed i.e. 15:0 will become {8:15,0:7}"]
    _1,
}
impl CRC_OUT_FLIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_OUT_FLIPW::_0 => false,
            CRC_OUT_FLIPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_OUT_FLIPW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_OUT_FLIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_OUT_FLIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bits within the CRC output bytes will not be reversed i.e. 15:0 will remain 15:0 (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_OUT_FLIPW::_0)
    }
    #[doc = "Bits within the CRC output bytes will be reversed i.e. 15:0 will become {8:15,0:7}"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_OUT_FLIPW::_1)
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
#[doc = "Values that can be written to the field `CRC_IN_FLIP`"]
pub enum CRC_IN_FLIPW {
    #[doc = "Bits in the input byte will not be reversed (i.e. 7:0 will remain 7:0) before the CRC calculation (default)"]
    _0,
    #[doc = "Bits in the input byte will be reversed (i.e. 7:0 will become 0:7) before CRC calculation"]
    _1,
}
impl CRC_IN_FLIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_IN_FLIPW::_0 => false,
            CRC_IN_FLIPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_IN_FLIPW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_IN_FLIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_IN_FLIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bits in the input byte will not be reversed (i.e. 7:0 will remain 7:0) before the CRC calculation (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_IN_FLIPW::_0)
    }
    #[doc = "Bits in the input byte will be reversed (i.e. 7:0 will become 0:7) before CRC calculation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_IN_FLIPW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CWT_EN`"]
pub enum CWT_ENW {
    #[doc = "Character Wait time Counter is disabled (default)"]
    _0,
    #[doc = "Character Wait time counter is enabled"]
    _1,
}
impl CWT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CWT_ENW::_0 => false,
            CWT_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CWT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CWT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CWT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Character Wait time Counter is disabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWT_ENW::_0)
    }
    #[doc = "Character Wait time counter is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWT_ENW::_1)
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
#[doc = "Values that can be written to the field `LRC_EN`"]
pub enum LRC_ENW {
    #[doc = "8-bit Linear Redundancy Checking disabled (default)"]
    _0,
    #[doc = "8-bit Linear Redundancy Checking enabled"]
    _1,
}
impl LRC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRC_ENW::_0 => false,
            LRC_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LRC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit Linear Redundancy Checking disabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRC_ENW::_0)
    }
    #[doc = "8-bit Linear Redundancy Checking enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRC_ENW::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRC_EN`"]
pub enum CRC_ENW {
    #[doc = "16-bit Cyclic Redundancy Checking disabled (default)"]
    _0,
    #[doc = "16-bit Cyclic Redundancy Checking enabled"]
    _1,
}
impl CRC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_ENW::_0 => false,
            CRC_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "16-bit Cyclic Redundancy Checking disabled (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_ENW::_0)
    }
    #[doc = "16-bit Cyclic Redundancy Checking enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_ENW::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XMT_CRC_LRC`"]
pub enum XMT_CRC_LRCW {
    #[doc = "No CRC or LRC value is transmitted (default)"]
    _0,
    #[doc = "Transmit LRC or CRC info when FIFO empties (whichever is enabled)"]
    _1,
}
impl XMT_CRC_LRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XMT_CRC_LRCW::_0 => false,
            XMT_CRC_LRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XMT_CRC_LRCW<'a> {
    w: &'a mut W,
}
impl<'a> _XMT_CRC_LRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XMT_CRC_LRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No CRC or LRC value is transmitted (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(XMT_CRC_LRCW::_0)
    }
    #[doc = "Transmit LRC or CRC info when FIFO empties (whichever is enabled)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(XMT_CRC_LRCW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWT_EN`"]
pub enum BWT_ENW {
    #[doc = "Disable BWT, BGT Counters (default)"]
    _0,
    #[doc = "Enable BWT, BGT Counters"]
    _1,
}
impl BWT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWT_ENW::_0 => false,
            BWT_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BWT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable BWT, BGT Counters (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWT_ENW::_0)
    }
    #[doc = "Enable BWT, BGT Counters"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWT_ENW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Inverse Convention"]
    #[inline]
    pub fn ic(&self) -> ICR {
        ICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Initial Character Mode"]
    #[inline]
    pub fn icm(&self) -> ICMR {
        ICMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Auto NACK Enable"]
    #[inline]
    pub fn anack(&self) -> ANACKR {
        ANACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Overrun NACK Enable"]
    #[inline]
    pub fn onack(&self) -> ONACKR {
        ONACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Kill all internal clocks"]
    #[inline]
    pub fn kill_clocks(&self) -> KILL_CLOCKSR {
        KILL_CLOCKSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Doze Enable"]
    #[inline]
    pub fn doze_en(&self) -> DOZE_ENR {
        DOZE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - STOP Enable"]
    #[inline]
    pub fn stop_en(&self) -> STOP_ENR {
        STOP_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Receiver Enable"]
    #[inline]
    pub fn rcv_en(&self) -> RCV_ENR {
        RCV_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Transmitter Enable"]
    #[inline]
    pub fn xmt_en(&self) -> XMT_ENR {
        XMT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Receiver 11 ETU Mode Enable"]
    #[inline]
    pub fn rcvr_11(&self) -> RCVR_11R {
        RCVR_11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Receive DMA Enable"]
    #[inline]
    pub fn rx_dma_en(&self) -> RX_DMA_ENR {
        RX_DMA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Transmit DMA Enable"]
    #[inline]
    pub fn tx_dma_en(&self) -> TX_DMA_ENR {
        TX_DMA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Invert bits in the CRC Output Value"]
    #[inline]
    pub fn inv_crc_val(&self) -> INV_CRC_VALR {
        INV_CRC_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CRC Output Value Bit Reversal or Flip"]
    #[inline]
    pub fn crc_out_flip(&self) -> CRC_OUT_FLIPR {
        CRC_OUT_FLIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - CRC Input Byte's Bit Reversal or Flip Control"]
    #[inline]
    pub fn crc_in_flip(&self) -> CRC_IN_FLIPR {
        CRC_IN_FLIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Character Wait Time Counter Enable"]
    #[inline]
    pub fn cwt_en(&self) -> CWT_ENR {
        CWT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - LRC Enable"]
    #[inline]
    pub fn lrc_en(&self) -> LRC_ENR {
        LRC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - CRC Enable"]
    #[inline]
    pub fn crc_en(&self) -> CRC_ENR {
        CRC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Transmit CRC or LRC Enable"]
    #[inline]
    pub fn xmt_crc_lrc(&self) -> XMT_CRC_LRCR {
        XMT_CRC_LRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Block Wait Time Counter Enable"]
    #[inline]
    pub fn bwt_en(&self) -> BWT_ENR {
        BWT_ENR::_from({
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
        W { bits: 16777222 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Inverse Convention"]
    #[inline]
    pub fn ic(&mut self) -> _ICW {
        _ICW { w: self }
    }
    #[doc = "Bit 1 - Initial Character Mode"]
    #[inline]
    pub fn icm(&mut self) -> _ICMW {
        _ICMW { w: self }
    }
    #[doc = "Bit 2 - Auto NACK Enable"]
    #[inline]
    pub fn anack(&mut self) -> _ANACKW {
        _ANACKW { w: self }
    }
    #[doc = "Bit 3 - Overrun NACK Enable"]
    #[inline]
    pub fn onack(&mut self) -> _ONACKW {
        _ONACKW { w: self }
    }
    #[doc = "Bit 8 - Flush Receiver Bit"]
    #[inline]
    pub fn flsh_rx(&mut self) -> _FLSH_RXW {
        _FLSH_RXW { w: self }
    }
    #[doc = "Bit 9 - Flush Transmitter Bit"]
    #[inline]
    pub fn flsh_tx(&mut self) -> _FLSH_TXW {
        _FLSH_TXW { w: self }
    }
    #[doc = "Bit 10 - Software Reset Bit"]
    #[inline]
    pub fn sw_rst(&mut self) -> _SW_RSTW {
        _SW_RSTW { w: self }
    }
    #[doc = "Bit 11 - Kill all internal clocks"]
    #[inline]
    pub fn kill_clocks(&mut self) -> _KILL_CLOCKSW {
        _KILL_CLOCKSW { w: self }
    }
    #[doc = "Bit 12 - Doze Enable"]
    #[inline]
    pub fn doze_en(&mut self) -> _DOZE_ENW {
        _DOZE_ENW { w: self }
    }
    #[doc = "Bit 13 - STOP Enable"]
    #[inline]
    pub fn stop_en(&mut self) -> _STOP_ENW {
        _STOP_ENW { w: self }
    }
    #[doc = "Bit 16 - Receiver Enable"]
    #[inline]
    pub fn rcv_en(&mut self) -> _RCV_ENW {
        _RCV_ENW { w: self }
    }
    #[doc = "Bit 17 - Transmitter Enable"]
    #[inline]
    pub fn xmt_en(&mut self) -> _XMT_ENW {
        _XMT_ENW { w: self }
    }
    #[doc = "Bit 18 - Receiver 11 ETU Mode Enable"]
    #[inline]
    pub fn rcvr_11(&mut self) -> _RCVR_11W {
        _RCVR_11W { w: self }
    }
    #[doc = "Bit 19 - Receive DMA Enable"]
    #[inline]
    pub fn rx_dma_en(&mut self) -> _RX_DMA_ENW {
        _RX_DMA_ENW { w: self }
    }
    #[doc = "Bit 20 - Transmit DMA Enable"]
    #[inline]
    pub fn tx_dma_en(&mut self) -> _TX_DMA_ENW {
        _TX_DMA_ENW { w: self }
    }
    #[doc = "Bit 24 - Invert bits in the CRC Output Value"]
    #[inline]
    pub fn inv_crc_val(&mut self) -> _INV_CRC_VALW {
        _INV_CRC_VALW { w: self }
    }
    #[doc = "Bit 25 - CRC Output Value Bit Reversal or Flip"]
    #[inline]
    pub fn crc_out_flip(&mut self) -> _CRC_OUT_FLIPW {
        _CRC_OUT_FLIPW { w: self }
    }
    #[doc = "Bit 26 - CRC Input Byte's Bit Reversal or Flip Control"]
    #[inline]
    pub fn crc_in_flip(&mut self) -> _CRC_IN_FLIPW {
        _CRC_IN_FLIPW { w: self }
    }
    #[doc = "Bit 27 - Character Wait Time Counter Enable"]
    #[inline]
    pub fn cwt_en(&mut self) -> _CWT_ENW {
        _CWT_ENW { w: self }
    }
    #[doc = "Bit 28 - LRC Enable"]
    #[inline]
    pub fn lrc_en(&mut self) -> _LRC_ENW {
        _LRC_ENW { w: self }
    }
    #[doc = "Bit 29 - CRC Enable"]
    #[inline]
    pub fn crc_en(&mut self) -> _CRC_ENW {
        _CRC_ENW { w: self }
    }
    #[doc = "Bit 30 - Transmit CRC or LRC Enable"]
    #[inline]
    pub fn xmt_crc_lrc(&mut self) -> _XMT_CRC_LRCW {
        _XMT_CRC_LRCW { w: self }
    }
    #[doc = "Bit 31 - Block Wait Time Counter Enable"]
    #[inline]
    pub fn bwt_en(&mut self) -> _BWT_ENW {
        _BWT_ENW { w: self }
    }
}
