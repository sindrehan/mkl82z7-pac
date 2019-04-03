#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT_MASK {
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
#[doc = "Possible values of the field `RDT_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDT_IMR {
    #[doc = "RDTF interrupt enabled"]
    _0,
    #[doc = "RDTF interrupt masked (default)"]
    _1,
}
impl RDT_IMR {
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
            RDT_IMR::_0 => false,
            RDT_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDT_IMR {
        match value {
            false => RDT_IMR::_0,
            true => RDT_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDT_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDT_IMR::_1
    }
}
#[doc = "Possible values of the field `TC_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_IMR {
    #[doc = "TCF interrupt enabled"]
    _0,
    #[doc = "TCF interrupt masked (default)"]
    _1,
}
impl TC_IMR {
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
            TC_IMR::_0 => false,
            TC_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC_IMR {
        match value {
            false => TC_IMR::_0,
            true => TC_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TC_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TC_IMR::_1
    }
}
#[doc = "Possible values of the field `RFO_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFO_IMR {
    #[doc = "RFO interrupt enabled"]
    _0,
    #[doc = "RFO interrupt masked (default)"]
    _1,
}
impl RFO_IMR {
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
            RFO_IMR::_0 => false,
            RFO_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFO_IMR {
        match value {
            false => RFO_IMR::_0,
            true => RFO_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFO_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFO_IMR::_1
    }
}
#[doc = "Possible values of the field `ETC_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETC_IMR {
    #[doc = "ETC interrupt enabled"]
    _0,
    #[doc = "ETC interrupt masked (default)"]
    _1,
}
impl ETC_IMR {
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
            ETC_IMR::_0 => false,
            ETC_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETC_IMR {
        match value {
            false => ETC_IMR::_0,
            true => ETC_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ETC_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ETC_IMR::_1
    }
}
#[doc = "Possible values of the field `TFE_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_IMR {
    #[doc = "TFE interrupt enabled"]
    _0,
    #[doc = "TFE interrupt masked (default)"]
    _1,
}
impl TFE_IMR {
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
            TFE_IMR::_0 => false,
            TFE_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFE_IMR {
        match value {
            false => TFE_IMR::_0,
            true => TFE_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFE_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFE_IMR::_1
    }
}
#[doc = "Possible values of the field `TNACK_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNACK_IMR {
    #[doc = "TNTE interrupt enabled"]
    _0,
    #[doc = "TNTE interrupt masked (default)"]
    _1,
}
impl TNACK_IMR {
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
            TNACK_IMR::_0 => false,
            TNACK_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TNACK_IMR {
        match value {
            false => TNACK_IMR::_0,
            true => TNACK_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TNACK_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TNACK_IMR::_1
    }
}
#[doc = "Possible values of the field `TFF_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFF_IMR {
    #[doc = "TFF interrupt enabled"]
    _0,
    #[doc = "TFF interrupt masked (default)"]
    _1,
}
impl TFF_IMR {
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
            TFF_IMR::_0 => false,
            TFF_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFF_IMR {
        match value {
            false => TFF_IMR::_0,
            true => TFF_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFF_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFF_IMR::_1
    }
}
#[doc = "Possible values of the field `TDT_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDT_IMR {
    #[doc = "TDTF interrupt enabled"]
    _0,
    #[doc = "TDTF interrupt masked (default)"]
    _1,
}
impl TDT_IMR {
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
            TDT_IMR::_0 => false,
            TDT_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDT_IMR {
        match value {
            false => TDT_IMR::_0,
            true => TDT_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDT_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDT_IMR::_1
    }
}
#[doc = "Possible values of the field `GPCNT0_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT0_IMR {
    #[doc = "GPCNT0_TO interrupt enabled"]
    _0,
    #[doc = "GPCNT0_TO interrupt masked (default)"]
    _1,
}
impl GPCNT0_IMR {
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
            GPCNT0_IMR::_0 => false,
            GPCNT0_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPCNT0_IMR {
        match value {
            false => GPCNT0_IMR::_0,
            true => GPCNT0_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GPCNT0_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GPCNT0_IMR::_1
    }
}
#[doc = "Possible values of the field `CWT_ERR_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWT_ERR_IMR {
    #[doc = "CWT_ERR interrupt enabled"]
    _0,
    #[doc = "CWT_ERR interrupt masked (default)"]
    _1,
}
impl CWT_ERR_IMR {
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
            CWT_ERR_IMR::_0 => false,
            CWT_ERR_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CWT_ERR_IMR {
        match value {
            false => CWT_ERR_IMR::_0,
            true => CWT_ERR_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CWT_ERR_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CWT_ERR_IMR::_1
    }
}
#[doc = "Possible values of the field `RNACK_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNACK_IMR {
    #[doc = "RTE interrupt enabled"]
    _0,
    #[doc = "RTE interrupt masked (default)"]
    _1,
}
impl RNACK_IMR {
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
            RNACK_IMR::_0 => false,
            RNACK_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RNACK_IMR {
        match value {
            false => RNACK_IMR::_0,
            true => RNACK_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RNACK_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RNACK_IMR::_1
    }
}
#[doc = "Possible values of the field `BWT_ERR_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWT_ERR_IMR {
    #[doc = "BWT_ERR interrupt enabled"]
    _0,
    #[doc = "BWT_ERR interrupt masked (default)"]
    _1,
}
impl BWT_ERR_IMR {
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
            BWT_ERR_IMR::_0 => false,
            BWT_ERR_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWT_ERR_IMR {
        match value {
            false => BWT_ERR_IMR::_0,
            true => BWT_ERR_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWT_ERR_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWT_ERR_IMR::_1
    }
}
#[doc = "Possible values of the field `BGT_ERR_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGT_ERR_IMR {
    #[doc = "BGT_ERR interrupt enabled"]
    _0,
    #[doc = "BGT_ERR interrupt masked (default)"]
    _1,
}
impl BGT_ERR_IMR {
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
            BGT_ERR_IMR::_0 => false,
            BGT_ERR_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGT_ERR_IMR {
        match value {
            false => BGT_ERR_IMR::_0,
            true => BGT_ERR_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BGT_ERR_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BGT_ERR_IMR::_1
    }
}
#[doc = "Possible values of the field `GPCNT1_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT1_IMR {
    #[doc = "GPCNT1_TO interrupt enabled"]
    _0,
    #[doc = "GPCNT1_TO interrupt masked (default)"]
    _1,
}
impl GPCNT1_IMR {
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
            GPCNT1_IMR::_0 => false,
            GPCNT1_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPCNT1_IMR {
        match value {
            false => GPCNT1_IMR::_0,
            true => GPCNT1_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GPCNT1_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GPCNT1_IMR::_1
    }
}
#[doc = "Possible values of the field `RX_DATA_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATA_IMR {
    #[doc = "RX_DATA interrupt enabled"]
    _0,
    #[doc = "RX_DATA interrupt masked (default)"]
    _1,
}
impl RX_DATA_IMR {
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
            RX_DATA_IMR::_0 => false,
            RX_DATA_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DATA_IMR {
        match value {
            false => RX_DATA_IMR::_0,
            true => RX_DATA_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DATA_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DATA_IMR::_1
    }
}
#[doc = "Possible values of the field `PEF_IM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEF_IMR {
    #[doc = "PEF interrupt enabled"]
    _0,
    #[doc = "PEF interrupt masked (default)"]
    _1,
}
impl PEF_IMR {
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
            PEF_IMR::_0 => false,
            PEF_IMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEF_IMR {
        match value {
            false => PEF_IMR::_0,
            true => PEF_IMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEF_IMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEF_IMR::_1
    }
}
#[doc = "Values that can be written to the field `RDT_IM`"]
pub enum RDT_IMW {
    #[doc = "RDTF interrupt enabled"]
    _0,
    #[doc = "RDTF interrupt masked (default)"]
    _1,
}
impl RDT_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDT_IMW::_0 => false,
            RDT_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDT_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _RDT_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDT_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RDTF interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDT_IMW::_0)
    }
    #[doc = "RDTF interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDT_IMW::_1)
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
#[doc = "Values that can be written to the field `TC_IM`"]
pub enum TC_IMW {
    #[doc = "TCF interrupt enabled"]
    _0,
    #[doc = "TCF interrupt masked (default)"]
    _1,
}
impl TC_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TC_IMW::_0 => false,
            TC_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _TC_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TCF interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TC_IMW::_0)
    }
    #[doc = "TCF interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TC_IMW::_1)
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
#[doc = "Values that can be written to the field `RFO_IM`"]
pub enum RFO_IMW {
    #[doc = "RFO interrupt enabled"]
    _0,
    #[doc = "RFO interrupt masked (default)"]
    _1,
}
impl RFO_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFO_IMW::_0 => false,
            RFO_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFO_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _RFO_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFO_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RFO interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFO_IMW::_0)
    }
    #[doc = "RFO interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFO_IMW::_1)
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
#[doc = "Values that can be written to the field `ETC_IM`"]
pub enum ETC_IMW {
    #[doc = "ETC interrupt enabled"]
    _0,
    #[doc = "ETC interrupt masked (default)"]
    _1,
}
impl ETC_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETC_IMW::_0 => false,
            ETC_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETC_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _ETC_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETC_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ETC interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETC_IMW::_0)
    }
    #[doc = "ETC interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETC_IMW::_1)
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
#[doc = "Values that can be written to the field `TFE_IM`"]
pub enum TFE_IMW {
    #[doc = "TFE interrupt enabled"]
    _0,
    #[doc = "TFE interrupt masked (default)"]
    _1,
}
impl TFE_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFE_IMW::_0 => false,
            TFE_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFE_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _TFE_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFE_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TFE interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFE_IMW::_0)
    }
    #[doc = "TFE interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFE_IMW::_1)
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
#[doc = "Values that can be written to the field `TNACK_IM`"]
pub enum TNACK_IMW {
    #[doc = "TNTE interrupt enabled"]
    _0,
    #[doc = "TNTE interrupt masked (default)"]
    _1,
}
impl TNACK_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TNACK_IMW::_0 => false,
            TNACK_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNACK_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _TNACK_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNACK_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TNTE interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TNACK_IMW::_0)
    }
    #[doc = "TNTE interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TNACK_IMW::_1)
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
#[doc = "Values that can be written to the field `TFF_IM`"]
pub enum TFF_IMW {
    #[doc = "TFF interrupt enabled"]
    _0,
    #[doc = "TFF interrupt masked (default)"]
    _1,
}
impl TFF_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFF_IMW::_0 => false,
            TFF_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFF_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _TFF_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFF_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TFF interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFF_IMW::_0)
    }
    #[doc = "TFF interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFF_IMW::_1)
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
#[doc = "Values that can be written to the field `TDT_IM`"]
pub enum TDT_IMW {
    #[doc = "TDTF interrupt enabled"]
    _0,
    #[doc = "TDTF interrupt masked (default)"]
    _1,
}
impl TDT_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDT_IMW::_0 => false,
            TDT_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDT_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _TDT_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDT_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TDTF interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDT_IMW::_0)
    }
    #[doc = "TDTF interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDT_IMW::_1)
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
#[doc = "Values that can be written to the field `GPCNT0_IM`"]
pub enum GPCNT0_IMW {
    #[doc = "GPCNT0_TO interrupt enabled"]
    _0,
    #[doc = "GPCNT0_TO interrupt masked (default)"]
    _1,
}
impl GPCNT0_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPCNT0_IMW::_0 => false,
            GPCNT0_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPCNT0_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _GPCNT0_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPCNT0_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPCNT0_TO interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPCNT0_IMW::_0)
    }
    #[doc = "GPCNT0_TO interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPCNT0_IMW::_1)
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
#[doc = "Values that can be written to the field `CWT_ERR_IM`"]
pub enum CWT_ERR_IMW {
    #[doc = "CWT_ERR interrupt enabled"]
    _0,
    #[doc = "CWT_ERR interrupt masked (default)"]
    _1,
}
impl CWT_ERR_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CWT_ERR_IMW::_0 => false,
            CWT_ERR_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CWT_ERR_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _CWT_ERR_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CWT_ERR_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CWT_ERR interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWT_ERR_IMW::_0)
    }
    #[doc = "CWT_ERR interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWT_ERR_IMW::_1)
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
#[doc = "Values that can be written to the field `RNACK_IM`"]
pub enum RNACK_IMW {
    #[doc = "RTE interrupt enabled"]
    _0,
    #[doc = "RTE interrupt masked (default)"]
    _1,
}
impl RNACK_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RNACK_IMW::_0 => false,
            RNACK_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RNACK_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _RNACK_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNACK_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTE interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNACK_IMW::_0)
    }
    #[doc = "RTE interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RNACK_IMW::_1)
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
#[doc = "Values that can be written to the field `BWT_ERR_IM`"]
pub enum BWT_ERR_IMW {
    #[doc = "BWT_ERR interrupt enabled"]
    _0,
    #[doc = "BWT_ERR interrupt masked (default)"]
    _1,
}
impl BWT_ERR_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWT_ERR_IMW::_0 => false,
            BWT_ERR_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWT_ERR_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _BWT_ERR_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWT_ERR_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BWT_ERR interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWT_ERR_IMW::_0)
    }
    #[doc = "BWT_ERR interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWT_ERR_IMW::_1)
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
#[doc = "Values that can be written to the field `BGT_ERR_IM`"]
pub enum BGT_ERR_IMW {
    #[doc = "BGT_ERR interrupt enabled"]
    _0,
    #[doc = "BGT_ERR interrupt masked (default)"]
    _1,
}
impl BGT_ERR_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGT_ERR_IMW::_0 => false,
            BGT_ERR_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGT_ERR_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _BGT_ERR_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGT_ERR_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BGT_ERR interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGT_ERR_IMW::_0)
    }
    #[doc = "BGT_ERR interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGT_ERR_IMW::_1)
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
#[doc = "Values that can be written to the field `GPCNT1_IM`"]
pub enum GPCNT1_IMW {
    #[doc = "GPCNT1_TO interrupt enabled"]
    _0,
    #[doc = "GPCNT1_TO interrupt masked (default)"]
    _1,
}
impl GPCNT1_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPCNT1_IMW::_0 => false,
            GPCNT1_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPCNT1_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _GPCNT1_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPCNT1_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPCNT1_TO interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPCNT1_IMW::_0)
    }
    #[doc = "GPCNT1_TO interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPCNT1_IMW::_1)
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
#[doc = "Values that can be written to the field `RX_DATA_IM`"]
pub enum RX_DATA_IMW {
    #[doc = "RX_DATA interrupt enabled"]
    _0,
    #[doc = "RX_DATA interrupt masked (default)"]
    _1,
}
impl RX_DATA_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_DATA_IMW::_0 => false,
            RX_DATA_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DATA_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DATA_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DATA_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX_DATA interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DATA_IMW::_0)
    }
    #[doc = "RX_DATA interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DATA_IMW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEF_IM`"]
pub enum PEF_IMW {
    #[doc = "PEF interrupt enabled"]
    _0,
    #[doc = "PEF interrupt masked (default)"]
    _1,
}
impl PEF_IMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEF_IMW::_0 => false,
            PEF_IMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEF_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _PEF_IMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEF_IMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PEF interrupt enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEF_IMW::_0)
    }
    #[doc = "PEF interrupt masked (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEF_IMW::_1)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Receive Data Threshold Interrupt Mask"]
    #[inline]
    pub fn rdt_im(&self) -> RDT_IMR {
        RDT_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Mask"]
    #[inline]
    pub fn tc_im(&self) -> TC_IMR {
        TC_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Mask"]
    #[inline]
    pub fn rfo_im(&self) -> RFO_IMR {
        RFO_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Early Transmit Complete Interrupt Mask"]
    #[inline]
    pub fn etc_im(&self) -> ETC_IMR {
        ETC_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Transmit FIFO Empty Interrupt Mask"]
    #[inline]
    pub fn tfe_im(&self) -> TFE_IMR {
        TFE_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Transmit NACK Threshold Interrupt Mask"]
    #[inline]
    pub fn tnack_im(&self) -> TNACK_IMR {
        TNACK_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmit FIFO Full Interrupt Mask"]
    #[inline]
    pub fn tff_im(&self) -> TFF_IMR {
        TFF_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transmit Data Threshold Interrupt Mask"]
    #[inline]
    pub fn tdt_im(&self) -> TDT_IMR {
        TDT_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - General Purpose Timer 0 Timeout Interrupt Mask"]
    #[inline]
    pub fn gpcnt0_im(&self) -> GPCNT0_IMR {
        GPCNT0_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Character Wait Time Error Interrupt Mask"]
    #[inline]
    pub fn cwt_err_im(&self) -> CWT_ERR_IMR {
        CWT_ERR_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Receiver NACK Threshold Interrupt Mask"]
    #[inline]
    pub fn rnack_im(&self) -> RNACK_IMR {
        RNACK_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Block Wait Time Error Interrupt Mask"]
    #[inline]
    pub fn bwt_err_im(&self) -> BWT_ERR_IMR {
        BWT_ERR_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Block Guard Time Error Interrupt"]
    #[inline]
    pub fn bgt_err_im(&self) -> BGT_ERR_IMR {
        BGT_ERR_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - General Purpose Counter 1 Timeout Interrupt Mask"]
    #[inline]
    pub fn gpcnt1_im(&self) -> GPCNT1_IMR {
        GPCNT1_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive Data Interrupt Mask"]
    #[inline]
    pub fn rx_data_im(&self) -> RX_DATA_IMR {
        RX_DATA_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Parity Error Interrupt Mask"]
    #[inline]
    pub fn pef_im(&self) -> PEF_IMR {
        PEF_IMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Receive Data Threshold Interrupt Mask"]
    #[inline]
    pub fn rdt_im(&mut self) -> _RDT_IMW {
        _RDT_IMW { w: self }
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Mask"]
    #[inline]
    pub fn tc_im(&mut self) -> _TC_IMW {
        _TC_IMW { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO Overflow Interrupt Mask"]
    #[inline]
    pub fn rfo_im(&mut self) -> _RFO_IMW {
        _RFO_IMW { w: self }
    }
    #[doc = "Bit 3 - Early Transmit Complete Interrupt Mask"]
    #[inline]
    pub fn etc_im(&mut self) -> _ETC_IMW {
        _ETC_IMW { w: self }
    }
    #[doc = "Bit 4 - Transmit FIFO Empty Interrupt Mask"]
    #[inline]
    pub fn tfe_im(&mut self) -> _TFE_IMW {
        _TFE_IMW { w: self }
    }
    #[doc = "Bit 5 - Transmit NACK Threshold Interrupt Mask"]
    #[inline]
    pub fn tnack_im(&mut self) -> _TNACK_IMW {
        _TNACK_IMW { w: self }
    }
    #[doc = "Bit 6 - Transmit FIFO Full Interrupt Mask"]
    #[inline]
    pub fn tff_im(&mut self) -> _TFF_IMW {
        _TFF_IMW { w: self }
    }
    #[doc = "Bit 7 - Transmit Data Threshold Interrupt Mask"]
    #[inline]
    pub fn tdt_im(&mut self) -> _TDT_IMW {
        _TDT_IMW { w: self }
    }
    #[doc = "Bit 8 - General Purpose Timer 0 Timeout Interrupt Mask"]
    #[inline]
    pub fn gpcnt0_im(&mut self) -> _GPCNT0_IMW {
        _GPCNT0_IMW { w: self }
    }
    #[doc = "Bit 9 - Character Wait Time Error Interrupt Mask"]
    #[inline]
    pub fn cwt_err_im(&mut self) -> _CWT_ERR_IMW {
        _CWT_ERR_IMW { w: self }
    }
    #[doc = "Bit 10 - Receiver NACK Threshold Interrupt Mask"]
    #[inline]
    pub fn rnack_im(&mut self) -> _RNACK_IMW {
        _RNACK_IMW { w: self }
    }
    #[doc = "Bit 11 - Block Wait Time Error Interrupt Mask"]
    #[inline]
    pub fn bwt_err_im(&mut self) -> _BWT_ERR_IMW {
        _BWT_ERR_IMW { w: self }
    }
    #[doc = "Bit 12 - Block Guard Time Error Interrupt"]
    #[inline]
    pub fn bgt_err_im(&mut self) -> _BGT_ERR_IMW {
        _BGT_ERR_IMW { w: self }
    }
    #[doc = "Bit 13 - General Purpose Counter 1 Timeout Interrupt Mask"]
    #[inline]
    pub fn gpcnt1_im(&mut self) -> _GPCNT1_IMW {
        _GPCNT1_IMW { w: self }
    }
    #[doc = "Bit 14 - Receive Data Interrupt Mask"]
    #[inline]
    pub fn rx_data_im(&mut self) -> _RX_DATA_IMW {
        _RX_DATA_IMW { w: self }
    }
    #[doc = "Bit 15 - Parity Error Interrupt Mask"]
    #[inline]
    pub fn pef_im(&mut self) -> _PEF_IMW {
        _PEF_IMW { w: self }
    }
}
