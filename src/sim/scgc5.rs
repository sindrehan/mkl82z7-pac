#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCGC5 {
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
#[doc = "Possible values of the field `LPTMR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTMR0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPTMR0R {
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
            LPTMR0R::_0 => false,
            LPTMR0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPTMR0R {
        match value {
            false => LPTMR0R::_0,
            true => LPTMR0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPTMR0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPTMR0R::_1
    }
}
#[doc = "Possible values of the field `SECREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECREGR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SECREGR {
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
            SECREGR::_0 => false,
            SECREGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SECREGR {
        match value {
            false => SECREGR::_0,
            true => SECREGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SECREGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SECREGR::_1
    }
}
#[doc = "Possible values of the field `LPTMR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTMR1R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPTMR1R {
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
            LPTMR1R::_0 => false,
            LPTMR1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPTMR1R {
        match value {
            false => LPTMR1R::_0,
            true => LPTMR1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPTMR1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPTMR1R::_1
    }
}
#[doc = "Possible values of the field `TSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIR {
    #[doc = "Access disabled"]
    _0,
    #[doc = "Access enabled"]
    _1,
}
impl TSIR {
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
            TSIR::_0 => false,
            TSIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIR {
        match value {
            false => TSIR::_0,
            true => TSIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSIR::_1
    }
}
#[doc = "Possible values of the field `PTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTAR {
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
            PTAR::_0 => false,
            PTAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTAR {
        match value {
            false => PTAR::_0,
            true => PTAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PTAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PTAR::_1
    }
}
#[doc = "Possible values of the field `PTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTBR {
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
            PTBR::_0 => false,
            PTBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTBR {
        match value {
            false => PTBR::_0,
            true => PTBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PTBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PTBR::_1
    }
}
#[doc = "Possible values of the field `PTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTCR {
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
            PTCR::_0 => false,
            PTCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTCR {
        match value {
            false => PTCR::_0,
            true => PTCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PTCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PTCR::_1
    }
}
#[doc = "Possible values of the field `PTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTDR {
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
            PTDR::_0 => false,
            PTDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTDR {
        match value {
            false => PTDR::_0,
            true => PTDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PTDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PTDR::_1
    }
}
#[doc = "Possible values of the field `PTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTER {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTER {
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
            PTER::_0 => false,
            PTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTER {
        match value {
            false => PTER::_0,
            true => PTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PTER::_1
    }
}
#[doc = "Possible values of the field `EMVSIM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMVSIM0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl EMVSIM0R {
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
            EMVSIM0R::_0 => false,
            EMVSIM0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMVSIM0R {
        match value {
            false => EMVSIM0R::_0,
            true => EMVSIM0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EMVSIM0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EMVSIM0R::_1
    }
}
#[doc = "Possible values of the field `EMVSIM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMVSIM1R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl EMVSIM1R {
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
            EMVSIM1R::_0 => false,
            EMVSIM1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMVSIM1R {
        match value {
            false => EMVSIM1R::_0,
            true => EMVSIM1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EMVSIM1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EMVSIM1R::_1
    }
}
#[doc = "Possible values of the field `LTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTCR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LTCR {
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
            LTCR::_0 => false,
            LTCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTCR {
        match value {
            false => LTCR::_0,
            true => LTCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LTCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LTCR::_1
    }
}
#[doc = "Possible values of the field `LPUART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPUART0R {
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
            LPUART0R::_0 => false,
            LPUART0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART0R {
        match value {
            false => LPUART0R::_0,
            true => LPUART0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPUART0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPUART0R::_1
    }
}
#[doc = "Possible values of the field `LPUART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPUART1R {
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
            LPUART1R::_0 => false,
            LPUART1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART1R {
        match value {
            false => LPUART1R::_0,
            true => LPUART1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPUART1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPUART1R::_1
    }
}
#[doc = "Possible values of the field `LPUART2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPUART2R {
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
            LPUART2R::_0 => false,
            LPUART2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPUART2R {
        match value {
            false => LPUART2R::_0,
            true => LPUART2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPUART2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPUART2R::_1
    }
}
#[doc = "Possible values of the field `QSPI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPI0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl QSPI0R {
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
            QSPI0R::_0 => false,
            QSPI0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QSPI0R {
        match value {
            false => QSPI0R::_0,
            true => QSPI0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == QSPI0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == QSPI0R::_1
    }
}
#[doc = "Possible values of the field `FLEXIO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl FLEXIO0R {
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
            FLEXIO0R::_0 => false,
            FLEXIO0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXIO0R {
        match value {
            false => FLEXIO0R::_0,
            true => FLEXIO0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLEXIO0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLEXIO0R::_1
    }
}
#[doc = "Values that can be written to the field `LPTMR0`"]
pub enum LPTMR0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPTMR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPTMR0W::_0 => false,
            LPTMR0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPTMR0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPTMR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPTMR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPTMR0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPTMR0W::_1)
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
#[doc = "Values that can be written to the field `SECREG`"]
pub enum SECREGW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SECREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SECREGW::_0 => false,
            SECREGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SECREGW<'a> {
    w: &'a mut W,
}
impl<'a> _SECREGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SECREGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECREGW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECREGW::_1)
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
#[doc = "Values that can be written to the field `LPTMR1`"]
pub enum LPTMR1W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPTMR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPTMR1W::_0 => false,
            LPTMR1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPTMR1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPTMR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPTMR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPTMR1W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPTMR1W::_1)
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
#[doc = "Values that can be written to the field `TSI`"]
pub enum TSIW {
    #[doc = "Access disabled"]
    _0,
    #[doc = "Access enabled"]
    _1,
}
impl TSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIW::_0 => false,
            TSIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Access disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIW::_0)
    }
    #[doc = "Access enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIW::_1)
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
#[doc = "Values that can be written to the field `PTA`"]
pub enum PTAW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTAW::_0 => false,
            PTAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTAW<'a> {
    w: &'a mut W,
}
impl<'a> _PTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAW::_1)
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
#[doc = "Values that can be written to the field `PTB`"]
pub enum PTBW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTBW::_0 => false,
            PTBW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTBW<'a> {
    w: &'a mut W,
}
impl<'a> _PTBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBW::_1)
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
#[doc = "Values that can be written to the field `PTC`"]
pub enum PTCW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCW::_0 => false,
            PTCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCW<'a> {
    w: &'a mut W,
}
impl<'a> _PTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCW::_1)
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
#[doc = "Values that can be written to the field `PTD`"]
pub enum PTDW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTDW::_0 => false,
            PTDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTDW<'a> {
    w: &'a mut W,
}
impl<'a> _PTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDW::_1)
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
#[doc = "Values that can be written to the field `PTE`"]
pub enum PTEW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTEW::_0 => false,
            PTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTEW<'a> {
    w: &'a mut W,
}
impl<'a> _PTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEW::_1)
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
#[doc = "Values that can be written to the field `EMVSIM0`"]
pub enum EMVSIM0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl EMVSIM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMVSIM0W::_0 => false,
            EMVSIM0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMVSIM0W<'a> {
    w: &'a mut W,
}
impl<'a> _EMVSIM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMVSIM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMVSIM0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMVSIM0W::_1)
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
#[doc = "Values that can be written to the field `EMVSIM1`"]
pub enum EMVSIM1W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl EMVSIM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMVSIM1W::_0 => false,
            EMVSIM1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMVSIM1W<'a> {
    w: &'a mut W,
}
impl<'a> _EMVSIM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMVSIM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMVSIM1W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMVSIM1W::_1)
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
#[doc = "Values that can be written to the field `LTC`"]
pub enum LTCW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LTCW::_0 => false,
            LTCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LTCW<'a> {
    w: &'a mut W,
}
impl<'a> _LTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LTCW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LTCW::_1)
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
#[doc = "Values that can be written to the field `LPUART0`"]
pub enum LPUART0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPUART0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART0W::_0 => false,
            LPUART0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART0W::_1)
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
#[doc = "Values that can be written to the field `LPUART1`"]
pub enum LPUART1W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPUART1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART1W::_0 => false,
            LPUART1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART1W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART1W::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUART2`"]
pub enum LPUART2W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl LPUART2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPUART2W::_0 => false,
            LPUART2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUART2W<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUART2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART2W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART2W::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QSPI0`"]
pub enum QSPI0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl QSPI0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QSPI0W::_0 => false,
            QSPI0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QSPI0W<'a> {
    w: &'a mut W,
}
impl<'a> _QSPI0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QSPI0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(QSPI0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(QSPI0W::_1)
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
#[doc = "Values that can be written to the field `FLEXIO0`"]
pub enum FLEXIO0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl FLEXIO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXIO0W::_0 => false,
            FLEXIO0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIO0W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLEXIO0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLEXIO0W::_1)
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
    #[doc = "Bit 0 - LPTMR0 Clock Gate Control"]
    #[inline]
    pub fn lptmr0(&self) -> LPTMR0R {
        LPTMR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - SECREG Clock Gate Control"]
    #[inline]
    pub fn secreg(&self) -> SECREGR {
        SECREGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - LPTMR1 Clock Gate Control"]
    #[inline]
    pub fn lptmr1(&self) -> LPTMR1R {
        LPTMR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - TSI Access Control"]
    #[inline]
    pub fn tsi(&self) -> TSIR {
        TSIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PTA Clock Gate Control"]
    #[inline]
    pub fn pta(&self) -> PTAR {
        PTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PTB Clock Gate Control"]
    #[inline]
    pub fn ptb(&self) -> PTBR {
        PTBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PTC Clock Gate Control"]
    #[inline]
    pub fn ptc(&self) -> PTCR {
        PTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PTD Clock Gate Control"]
    #[inline]
    pub fn ptd(&self) -> PTDR {
        PTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - PTE Clock Gate Control"]
    #[inline]
    pub fn pte(&self) -> PTER {
        PTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - EMVSIM0 Clock Gate Control"]
    #[inline]
    pub fn emvsim0(&self) -> EMVSIM0R {
        EMVSIM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - EMVSIM1 Clock Gate Control"]
    #[inline]
    pub fn emvsim1(&self) -> EMVSIM1R {
        EMVSIM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - LTC Clock Gate Control"]
    #[inline]
    pub fn ltc(&self) -> LTCR {
        LTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - LPUART0 Clock Gate Control"]
    #[inline]
    pub fn lpuart0(&self) -> LPUART0R {
        LPUART0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - LPUART1 Clock Gate Control"]
    #[inline]
    pub fn lpuart1(&self) -> LPUART1R {
        LPUART1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - LPUART2 Clock Gate Control"]
    #[inline]
    pub fn lpuart2(&self) -> LPUART2R {
        LPUART2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - QSPI0 Clock Gate Control"]
    #[inline]
    pub fn qspi0(&self) -> QSPI0R {
        QSPI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - FLEXIO0 Clock Gate Control"]
    #[inline]
    pub fn flexio0(&self) -> FLEXIO0R {
        FLEXIO0R::_from({
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
        W { bits: 262530 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LPTMR0 Clock Gate Control"]
    #[inline]
    pub fn lptmr0(&mut self) -> _LPTMR0W {
        _LPTMR0W { w: self }
    }
    #[doc = "Bit 3 - SECREG Clock Gate Control"]
    #[inline]
    pub fn secreg(&mut self) -> _SECREGW {
        _SECREGW { w: self }
    }
    #[doc = "Bit 4 - LPTMR1 Clock Gate Control"]
    #[inline]
    pub fn lptmr1(&mut self) -> _LPTMR1W {
        _LPTMR1W { w: self }
    }
    #[doc = "Bit 5 - TSI Access Control"]
    #[inline]
    pub fn tsi(&mut self) -> _TSIW {
        _TSIW { w: self }
    }
    #[doc = "Bit 9 - PTA Clock Gate Control"]
    #[inline]
    pub fn pta(&mut self) -> _PTAW {
        _PTAW { w: self }
    }
    #[doc = "Bit 10 - PTB Clock Gate Control"]
    #[inline]
    pub fn ptb(&mut self) -> _PTBW {
        _PTBW { w: self }
    }
    #[doc = "Bit 11 - PTC Clock Gate Control"]
    #[inline]
    pub fn ptc(&mut self) -> _PTCW {
        _PTCW { w: self }
    }
    #[doc = "Bit 12 - PTD Clock Gate Control"]
    #[inline]
    pub fn ptd(&mut self) -> _PTDW {
        _PTDW { w: self }
    }
    #[doc = "Bit 13 - PTE Clock Gate Control"]
    #[inline]
    pub fn pte(&mut self) -> _PTEW {
        _PTEW { w: self }
    }
    #[doc = "Bit 14 - EMVSIM0 Clock Gate Control"]
    #[inline]
    pub fn emvsim0(&mut self) -> _EMVSIM0W {
        _EMVSIM0W { w: self }
    }
    #[doc = "Bit 15 - EMVSIM1 Clock Gate Control"]
    #[inline]
    pub fn emvsim1(&mut self) -> _EMVSIM1W {
        _EMVSIM1W { w: self }
    }
    #[doc = "Bit 17 - LTC Clock Gate Control"]
    #[inline]
    pub fn ltc(&mut self) -> _LTCW {
        _LTCW { w: self }
    }
    #[doc = "Bit 20 - LPUART0 Clock Gate Control"]
    #[inline]
    pub fn lpuart0(&mut self) -> _LPUART0W {
        _LPUART0W { w: self }
    }
    #[doc = "Bit 21 - LPUART1 Clock Gate Control"]
    #[inline]
    pub fn lpuart1(&mut self) -> _LPUART1W {
        _LPUART1W { w: self }
    }
    #[doc = "Bit 22 - LPUART2 Clock Gate Control"]
    #[inline]
    pub fn lpuart2(&mut self) -> _LPUART2W {
        _LPUART2W { w: self }
    }
    #[doc = "Bit 26 - QSPI0 Clock Gate Control"]
    #[inline]
    pub fn qspi0(&mut self) -> _QSPI0W {
        _QSPI0W { w: self }
    }
    #[doc = "Bit 31 - FLEXIO0 Clock Gate Control"]
    #[inline]
    pub fn flexio0(&mut self) -> _FLEXIO0W {
        _FLEXIO0W { w: self }
    }
}
