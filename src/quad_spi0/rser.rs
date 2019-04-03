#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSER {
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
#[doc = "Possible values of the field `TFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFIER {
    #[doc = "No TFF interrupt will be generated"]
    _0,
    #[doc = "TFF interrupt will be generated"]
    _1,
}
impl TFIER {
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
            TFIER::_0 => false,
            TFIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFIER {
        match value {
            false => TFIER::_0,
            true => TFIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFIER::_1
    }
}
#[doc = "Possible values of the field `IPGEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGEIER {
    #[doc = "No IPGEF interrupt will be generated"]
    _0,
    #[doc = "IPGEF interrupt will be generated"]
    _1,
}
impl IPGEIER {
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
            IPGEIER::_0 => false,
            IPGEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPGEIER {
        match value {
            false => IPGEIER::_0,
            true => IPGEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IPGEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IPGEIER::_1
    }
}
#[doc = "Possible values of the field `IPIEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPIEIER {
    #[doc = "No IPIEF interrupt will be generated"]
    _0,
    #[doc = "IPIEF interrupt will be generated"]
    _1,
}
impl IPIEIER {
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
            IPIEIER::_0 => false,
            IPIEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPIEIER {
        match value {
            false => IPIEIER::_0,
            true => IPIEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IPIEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IPIEIER::_1
    }
}
#[doc = "Possible values of the field `IPAEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPAEIER {
    #[doc = "No IPAEF interrupt will be generated"]
    _0,
    #[doc = "IPAEF interrupt will be generated"]
    _1,
}
impl IPAEIER {
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
            IPAEIER::_0 => false,
            IPAEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPAEIER {
        match value {
            false => IPAEIER::_0,
            true => IPAEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IPAEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IPAEIER::_1
    }
}
#[doc = "Possible values of the field `IUEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IUEIER {
    #[doc = "No IUEF interrupt will be generated"]
    _0,
    #[doc = "IUEF interrupt will be generated"]
    _1,
}
impl IUEIER {
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
            IUEIER::_0 => false,
            IUEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IUEIER {
        match value {
            false => IUEIER::_0,
            true => IUEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IUEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IUEIER::_1
    }
}
#[doc = "Possible values of the field `ABOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABOIER {
    #[doc = "No ABOF interrupt will be generated"]
    _0,
    #[doc = "ABOF interrupt will be generated"]
    _1,
}
impl ABOIER {
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
            ABOIER::_0 => false,
            ABOIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABOIER {
        match value {
            false => ABOIER::_0,
            true => ABOIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ABOIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ABOIER::_1
    }
}
#[doc = "Possible values of the field `AIBSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIBSIER {
    #[doc = "No AIBSEF interrupt will be generated"]
    _0,
    #[doc = "AIBSEF interrupt will be generated"]
    _1,
}
impl AIBSIER {
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
            AIBSIER::_0 => false,
            AIBSIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIBSIER {
        match value {
            false => AIBSIER::_0,
            true => AIBSIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AIBSIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AIBSIER::_1
    }
}
#[doc = "Possible values of the field `AITIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AITIER {
    #[doc = "No AITEF interrupt will be generated"]
    _0,
    #[doc = "AITEF interrupt will be generated"]
    _1,
}
impl AITIER {
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
            AITIER::_0 => false,
            AITIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AITIER {
        match value {
            false => AITIER::_0,
            true => AITIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AITIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AITIER::_1
    }
}
#[doc = "Possible values of the field `ABSEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABSEIER {
    #[doc = "No ABSEF interrupt will be generated"]
    _0,
    #[doc = "ABSEF interrupt will be generated"]
    _1,
}
impl ABSEIER {
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
            ABSEIER::_0 => false,
            ABSEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABSEIER {
        match value {
            false => ABSEIER::_0,
            true => ABSEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ABSEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ABSEIER::_1
    }
}
#[doc = "Possible values of the field `RBDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBDIER {
    #[doc = "No RBDF interrupt will be generated"]
    _0,
    #[doc = "RBDF Interrupt will be generated"]
    _1,
}
impl RBDIER {
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
            RBDIER::_0 => false,
            RBDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBDIER {
        match value {
            false => RBDIER::_0,
            true => RBDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RBDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RBDIER::_1
    }
}
#[doc = "Possible values of the field `RBOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBOIER {
    #[doc = "No RBOF interrupt will be generated"]
    _0,
    #[doc = "RBOF interrupt will be generated"]
    _1,
}
impl RBOIER {
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
            RBOIER::_0 => false,
            RBOIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBOIER {
        match value {
            false => RBOIER::_0,
            true => RBOIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RBOIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RBOIER::_1
    }
}
#[doc = "Possible values of the field `RBDDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBDDER {
    #[doc = "No DMA request will be generated"]
    _0,
    #[doc = "DMA request will be generated"]
    _1,
}
impl RBDDER {
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
            RBDDER::_0 => false,
            RBDDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBDDER {
        match value {
            false => RBDDER::_0,
            true => RBDDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RBDDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RBDDER::_1
    }
}
#[doc = "Possible values of the field `ILLINIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILLINIER {
    #[doc = "No ILLINE interrupt will be generated"]
    _0,
    #[doc = "ILLINE interrupt will be generated"]
    _1,
}
impl ILLINIER {
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
            ILLINIER::_0 => false,
            ILLINIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILLINIER {
        match value {
            false => ILLINIER::_0,
            true => ILLINIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ILLINIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ILLINIER::_1
    }
}
#[doc = "Possible values of the field `TBFDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBFDER {
    #[doc = "No DMA request will be generated"]
    _0,
    #[doc = "DMA request will be generated"]
    _1,
}
impl TBFDER {
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
            TBFDER::_0 => false,
            TBFDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBFDER {
        match value {
            false => TBFDER::_0,
            true => TBFDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TBFDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TBFDER::_1
    }
}
#[doc = "Possible values of the field `TBUIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBUIER {
    #[doc = "No TBUF interrupt will be generated"]
    _0,
    #[doc = "TBUF interrupt will be generated"]
    _1,
}
impl TBUIER {
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
            TBUIER::_0 => false,
            TBUIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBUIER {
        match value {
            false => TBUIER::_0,
            true => TBUIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TBUIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TBUIER::_1
    }
}
#[doc = "Possible values of the field `TBFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBFIER {
    #[doc = "No TBFF interrupt will be generated"]
    _0,
    #[doc = "TBFF interrupt will be generated"]
    _1,
}
impl TBFIER {
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
            TBFIER::_0 => false,
            TBFIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBFIER {
        match value {
            false => TBFIER::_0,
            true => TBFIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TBFIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TBFIER::_1
    }
}
#[doc = "Possible values of the field `DLPFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLPFIER {
    #[doc = "No DLPFF interrupt will be generated"]
    _0,
    #[doc = "DLPFF interrupt will be generated"]
    _1,
}
impl DLPFIER {
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
            DLPFIER::_0 => false,
            DLPFIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLPFIER {
        match value {
            false => DLPFIER::_0,
            true => DLPFIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DLPFIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DLPFIER::_1
    }
}
#[doc = "Values that can be written to the field `TFIE`"]
pub enum TFIEW {
    #[doc = "No TFF interrupt will be generated"]
    _0,
    #[doc = "TFF interrupt will be generated"]
    _1,
}
impl TFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFIEW::_0 => false,
            TFIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No TFF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFIEW::_0)
    }
    #[doc = "TFF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFIEW::_1)
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
#[doc = "Values that can be written to the field `IPGEIE`"]
pub enum IPGEIEW {
    #[doc = "No IPGEF interrupt will be generated"]
    _0,
    #[doc = "IPGEF interrupt will be generated"]
    _1,
}
impl IPGEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPGEIEW::_0 => false,
            IPGEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPGEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IPGEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPGEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No IPGEF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPGEIEW::_0)
    }
    #[doc = "IPGEF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPGEIEW::_1)
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
#[doc = "Values that can be written to the field `IPIEIE`"]
pub enum IPIEIEW {
    #[doc = "No IPIEF interrupt will be generated"]
    _0,
    #[doc = "IPIEF interrupt will be generated"]
    _1,
}
impl IPIEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPIEIEW::_0 => false,
            IPIEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPIEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IPIEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPIEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No IPIEF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPIEIEW::_0)
    }
    #[doc = "IPIEF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPIEIEW::_1)
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
#[doc = "Values that can be written to the field `IPAEIE`"]
pub enum IPAEIEW {
    #[doc = "No IPAEF interrupt will be generated"]
    _0,
    #[doc = "IPAEF interrupt will be generated"]
    _1,
}
impl IPAEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPAEIEW::_0 => false,
            IPAEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPAEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IPAEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPAEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No IPAEF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPAEIEW::_0)
    }
    #[doc = "IPAEF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPAEIEW::_1)
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
#[doc = "Values that can be written to the field `IUEIE`"]
pub enum IUEIEW {
    #[doc = "No IUEF interrupt will be generated"]
    _0,
    #[doc = "IUEF interrupt will be generated"]
    _1,
}
impl IUEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IUEIEW::_0 => false,
            IUEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IUEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IUEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IUEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No IUEF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IUEIEW::_0)
    }
    #[doc = "IUEF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IUEIEW::_1)
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
#[doc = "Values that can be written to the field `ABOIE`"]
pub enum ABOIEW {
    #[doc = "No ABOF interrupt will be generated"]
    _0,
    #[doc = "ABOF interrupt will be generated"]
    _1,
}
impl ABOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABOIEW::_0 => false,
            ABOIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABOIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No ABOF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABOIEW::_0)
    }
    #[doc = "ABOF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABOIEW::_1)
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
#[doc = "Values that can be written to the field `AIBSIE`"]
pub enum AIBSIEW {
    #[doc = "No AIBSEF interrupt will be generated"]
    _0,
    #[doc = "AIBSEF interrupt will be generated"]
    _1,
}
impl AIBSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIBSIEW::_0 => false,
            AIBSIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIBSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AIBSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIBSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No AIBSEF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIBSIEW::_0)
    }
    #[doc = "AIBSEF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIBSIEW::_1)
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
#[doc = "Values that can be written to the field `AITIE`"]
pub enum AITIEW {
    #[doc = "No AITEF interrupt will be generated"]
    _0,
    #[doc = "AITEF interrupt will be generated"]
    _1,
}
impl AITIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AITIEW::_0 => false,
            AITIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AITIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AITIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AITIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No AITEF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AITIEW::_0)
    }
    #[doc = "AITEF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AITIEW::_1)
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
#[doc = "Values that can be written to the field `ABSEIE`"]
pub enum ABSEIEW {
    #[doc = "No ABSEF interrupt will be generated"]
    _0,
    #[doc = "ABSEF interrupt will be generated"]
    _1,
}
impl ABSEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABSEIEW::_0 => false,
            ABSEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABSEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABSEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABSEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No ABSEF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABSEIEW::_0)
    }
    #[doc = "ABSEF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABSEIEW::_1)
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
#[doc = "Values that can be written to the field `RBDIE`"]
pub enum RBDIEW {
    #[doc = "No RBDF interrupt will be generated"]
    _0,
    #[doc = "RBDF Interrupt will be generated"]
    _1,
}
impl RBDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBDIEW::_0 => false,
            RBDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No RBDF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBDIEW::_0)
    }
    #[doc = "RBDF Interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBDIEW::_1)
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
#[doc = "Values that can be written to the field `RBOIE`"]
pub enum RBOIEW {
    #[doc = "No RBOF interrupt will be generated"]
    _0,
    #[doc = "RBOF interrupt will be generated"]
    _1,
}
impl RBOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBOIEW::_0 => false,
            RBOIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBOIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No RBOF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBOIEW::_0)
    }
    #[doc = "RBOF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBOIEW::_1)
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
#[doc = "Values that can be written to the field `RBDDE`"]
pub enum RBDDEW {
    #[doc = "No DMA request will be generated"]
    _0,
    #[doc = "DMA request will be generated"]
    _1,
}
impl RBDDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBDDEW::_0 => false,
            RBDDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBDDEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBDDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBDDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DMA request will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBDDEW::_0)
    }
    #[doc = "DMA request will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBDDEW::_1)
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
#[doc = "Values that can be written to the field `ILLINIE`"]
pub enum ILLINIEW {
    #[doc = "No ILLINE interrupt will be generated"]
    _0,
    #[doc = "ILLINE interrupt will be generated"]
    _1,
}
impl ILLINIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ILLINIEW::_0 => false,
            ILLINIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ILLINIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ILLINIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ILLINIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No ILLINE interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILLINIEW::_0)
    }
    #[doc = "ILLINE interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILLINIEW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TBFDE`"]
pub enum TBFDEW {
    #[doc = "No DMA request will be generated"]
    _0,
    #[doc = "DMA request will be generated"]
    _1,
}
impl TBFDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBFDEW::_0 => false,
            TBFDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBFDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBFDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBFDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DMA request will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBFDEW::_0)
    }
    #[doc = "DMA request will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBFDEW::_1)
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
#[doc = "Values that can be written to the field `TBUIE`"]
pub enum TBUIEW {
    #[doc = "No TBUF interrupt will be generated"]
    _0,
    #[doc = "TBUF interrupt will be generated"]
    _1,
}
impl TBUIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBUIEW::_0 => false,
            TBUIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBUIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBUIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBUIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No TBUF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBUIEW::_0)
    }
    #[doc = "TBUF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBUIEW::_1)
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
#[doc = "Values that can be written to the field `TBFIE`"]
pub enum TBFIEW {
    #[doc = "No TBFF interrupt will be generated"]
    _0,
    #[doc = "TBFF interrupt will be generated"]
    _1,
}
impl TBFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBFIEW::_0 => false,
            TBFIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBFIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No TBFF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBFIEW::_0)
    }
    #[doc = "TBFF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBFIEW::_1)
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
#[doc = "Values that can be written to the field `DLPFIE`"]
pub enum DLPFIEW {
    #[doc = "No DLPFF interrupt will be generated"]
    _0,
    #[doc = "DLPFF interrupt will be generated"]
    _1,
}
impl DLPFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLPFIEW::_0 => false,
            DLPFIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLPFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DLPFIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLPFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No DLPFF interrupt will be generated"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLPFIEW::_0)
    }
    #[doc = "DLPFF interrupt will be generated"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLPFIEW::_1)
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
    #[doc = "Bit 0 - Transaction Finished Interrupt Enable"]
    #[inline]
    pub fn tfie(&self) -> TFIER {
        TFIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IP Command Trigger during AHB Grant Error Interrupt Enable"]
    #[inline]
    pub fn ipgeie(&self) -> IPGEIER {
        IPGEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - IP Command Trigger during IP Access Error Interrupt Enable"]
    #[inline]
    pub fn ipieie(&self) -> IPIEIER {
        IPIEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Interrupt Enable"]
    #[inline]
    pub fn ipaeie(&self) -> IPAEIER {
        IPAEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - IP Command Usage Error Interrupt Enable"]
    #[inline]
    pub fn iueie(&self) -> IUEIER {
        IUEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Interrupt Enable"]
    #[inline]
    pub fn aboie(&self) -> ABOIER {
        ABOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Interrupt Enable"]
    #[inline]
    pub fn aibsie(&self) -> AIBSIER {
        AIBSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - AHB Illegal transaction interrupt enable."]
    #[inline]
    pub fn aitie(&self) -> AITIER {
        AITIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR"]
    #[inline]
    pub fn abseie(&self) -> ABSEIER {
        ABSEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain"]
    #[inline]
    pub fn rbdie(&self) -> RBDIER {
        RBDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - RX Buffer Overflow Interrupt Enable"]
    #[inline]
    pub fn rboie(&self) -> RBOIER {
        RBOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain"]
    #[inline]
    pub fn rbdde(&self) -> RBDDER {
        RBDDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR"]
    #[inline]
    pub fn illinie(&self) -> ILLINIER {
        ILLINIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - TX Buffer Fill DMA Enable"]
    #[inline]
    pub fn tbfde(&self) -> TBFDER {
        TBFDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - TX Buffer Underrun Interrupt Enable"]
    #[inline]
    pub fn tbuie(&self) -> TBUIER {
        TBUIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - TX Buffer Fill Interrupt Enable"]
    #[inline]
    pub fn tbfie(&self) -> TBFIER {
        TBFIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Data Learning Pattern Failure Interrupt enable . Triggered by DLPFF flag in QSPI_FR register"]
    #[inline]
    pub fn dlpfie(&self) -> DLPFIER {
        DLPFIER::_from({
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
    #[doc = "Bit 0 - Transaction Finished Interrupt Enable"]
    #[inline]
    pub fn tfie(&mut self) -> _TFIEW {
        _TFIEW { w: self }
    }
    #[doc = "Bit 4 - IP Command Trigger during AHB Grant Error Interrupt Enable"]
    #[inline]
    pub fn ipgeie(&mut self) -> _IPGEIEW {
        _IPGEIEW { w: self }
    }
    #[doc = "Bit 6 - IP Command Trigger during IP Access Error Interrupt Enable"]
    #[inline]
    pub fn ipieie(&mut self) -> _IPIEIEW {
        _IPIEIEW { w: self }
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Interrupt Enable"]
    #[inline]
    pub fn ipaeie(&mut self) -> _IPAEIEW {
        _IPAEIEW { w: self }
    }
    #[doc = "Bit 11 - IP Command Usage Error Interrupt Enable"]
    #[inline]
    pub fn iueie(&mut self) -> _IUEIEW {
        _IUEIEW { w: self }
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Interrupt Enable"]
    #[inline]
    pub fn aboie(&mut self) -> _ABOIEW {
        _ABOIEW { w: self }
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Interrupt Enable"]
    #[inline]
    pub fn aibsie(&mut self) -> _AIBSIEW {
        _AIBSIEW { w: self }
    }
    #[doc = "Bit 14 - AHB Illegal transaction interrupt enable."]
    #[inline]
    pub fn aitie(&mut self) -> _AITIEW {
        _AITIEW { w: self }
    }
    #[doc = "Bit 15 - AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR"]
    #[inline]
    pub fn abseie(&mut self) -> _ABSEIEW {
        _ABSEIEW { w: self }
    }
    #[doc = "Bit 16 - RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain"]
    #[inline]
    pub fn rbdie(&mut self) -> _RBDIEW {
        _RBDIEW { w: self }
    }
    #[doc = "Bit 17 - RX Buffer Overflow Interrupt Enable"]
    #[inline]
    pub fn rboie(&mut self) -> _RBOIEW {
        _RBOIEW { w: self }
    }
    #[doc = "Bit 21 - RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain"]
    #[inline]
    pub fn rbdde(&mut self) -> _RBDDEW {
        _RBDDEW { w: self }
    }
    #[doc = "Bit 23 - Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR"]
    #[inline]
    pub fn illinie(&mut self) -> _ILLINIEW {
        _ILLINIEW { w: self }
    }
    #[doc = "Bit 25 - TX Buffer Fill DMA Enable"]
    #[inline]
    pub fn tbfde(&mut self) -> _TBFDEW {
        _TBFDEW { w: self }
    }
    #[doc = "Bit 26 - TX Buffer Underrun Interrupt Enable"]
    #[inline]
    pub fn tbuie(&mut self) -> _TBUIEW {
        _TBUIEW { w: self }
    }
    #[doc = "Bit 27 - TX Buffer Fill Interrupt Enable"]
    #[inline]
    pub fn tbfie(&mut self) -> _TBFIEW {
        _TBFIEW { w: self }
    }
    #[doc = "Bit 31 - Data Learning Pattern Failure Interrupt enable . Triggered by DLPFF flag in QSPI_FR register"]
    #[inline]
    pub fn dlpfie(&mut self) -> _DLPFIEW {
        _DLPFIEW { w: self }
    }
}
