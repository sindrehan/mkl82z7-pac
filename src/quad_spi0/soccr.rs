#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOCCR {
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
#[doc = "Possible values of the field `QSPISRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPISRCR {
    #[doc = "Core/system clock"]
    _000,
    #[doc = "MCGFLL clock"]
    _001,
    #[doc = "MCGPLL clock"]
    _010,
    #[doc = "MCGPLL 2x clock (DDR mode specific)"]
    _011,
    #[doc = "IRC48M clock"]
    _100,
    #[doc = "OSCERCLK clock"]
    _101,
    #[doc = "MCGIRCLK clock"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl QSPISRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QSPISRCR::_000 => 0,
            QSPISRCR::_001 => 1,
            QSPISRCR::_010 => 2,
            QSPISRCR::_011 => 3,
            QSPISRCR::_100 => 4,
            QSPISRCR::_101 => 5,
            QSPISRCR::_110 => 6,
            QSPISRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QSPISRCR {
        match value {
            0 => QSPISRCR::_000,
            1 => QSPISRCR::_001,
            2 => QSPISRCR::_010,
            3 => QSPISRCR::_011,
            4 => QSPISRCR::_100,
            5 => QSPISRCR::_101,
            6 => QSPISRCR::_110,
            i => QSPISRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == QSPISRCR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == QSPISRCR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == QSPISRCR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == QSPISRCR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == QSPISRCR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == QSPISRCR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == QSPISRCR::_110
    }
}
#[doc = "Possible values of the field `DQSLPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSLPENR {
    #[doc = "DQS loop back is disabled"]
    _0,
    #[doc = "DQS loop back is enabled"]
    _1,
}
impl DQSLPENR {
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
            DQSLPENR::_0 => false,
            DQSLPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DQSLPENR {
        match value {
            false => DQSLPENR::_0,
            true => DQSLPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DQSLPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DQSLPENR::_1
    }
}
#[doc = "Possible values of the field `DQSPADLPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSPADLPENR {
    #[doc = "DQS loop back from DQS pad is disabled"]
    _0,
    #[doc = "DQS loop back from DQS pad is enabled"]
    _1,
}
impl DQSPADLPENR {
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
            DQSPADLPENR::_0 => false,
            DQSPADLPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DQSPADLPENR {
        match value {
            false => DQSPADLPENR::_0,
            true => DQSPADLPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DQSPADLPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DQSPADLPENR::_1
    }
}
#[doc = "Possible values of the field `DQSPHASEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSPHASELR {
    #[doc = "No phase shift"]
    _00,
    #[doc = "Select 45 degree phase shift"]
    _01,
    #[doc = "Select 90 degree phase shift"]
    _10,
    #[doc = "Select 135 degree phase shift"]
    _11,
}
impl DQSPHASELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DQSPHASELR::_00 => 0,
            DQSPHASELR::_01 => 1,
            DQSPHASELR::_10 => 2,
            DQSPHASELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DQSPHASELR {
        match value {
            0 => DQSPHASELR::_00,
            1 => DQSPHASELR::_01,
            2 => DQSPHASELR::_10,
            3 => DQSPHASELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DQSPHASELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DQSPHASELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DQSPHASELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DQSPHASELR::_11
    }
}
#[doc = "Possible values of the field `DQSINVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSINVSELR {
    #[doc = "Use 1x internal reference clock for the DQS generation"]
    _0,
    #[doc = "Use inverse 1x internal reference clock for the DQS generation"]
    _1,
}
impl DQSINVSELR {
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
            DQSINVSELR::_0 => false,
            DQSINVSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DQSINVSELR {
        match value {
            false => DQSINVSELR::_0,
            true => DQSINVSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DQSINVSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DQSINVSELR::_1
    }
}
#[doc = "Possible values of the field `CK2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CK2ENR {
    #[doc = "CK2 flash clock is disabled"]
    _0,
    #[doc = "CK2 flash clock is enabled"]
    _1,
}
impl CK2ENR {
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
            CK2ENR::_0 => false,
            CK2ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CK2ENR {
        match value {
            false => CK2ENR::_0,
            true => CK2ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CK2ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CK2ENR::_1
    }
}
#[doc = "Possible values of the field `DIFFCKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFCKENR {
    #[doc = "Differential flash clock is disabled"]
    _0,
    #[doc = "Differential flash clock is enabled"]
    _1,
}
impl DIFFCKENR {
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
            DIFFCKENR::_0 => false,
            DIFFCKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIFFCKENR {
        match value {
            false => DIFFCKENR::_0,
            true => DIFFCKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DIFFCKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DIFFCKENR::_1
    }
}
#[doc = "Possible values of the field `OCTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCTENR {
    #[doc = "QSPI0B_DATAx pins are assigned to QSPI Port B"]
    _0,
    #[doc = "QSPI0B_DATAx pins are assigned to QSPI Port A"]
    _1,
}
impl OCTENR {
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
            OCTENR::_0 => false,
            OCTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCTENR {
        match value {
            false => OCTENR::_0,
            true => OCTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OCTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OCTENR::_1
    }
}
#[doc = "Possible values of the field `DLYTAPSELA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLYTAPSELAR {
    #[doc = "Select 1 delay chain tap"]
    _000000,
    #[doc = "Select 2 delay chain tap"]
    _000001,
    #[doc = "Select 3 delay chain tap"]
    _0000010,
    #[doc = "Select 4 delay chain tap"]
    _0000011,
    #[doc = "Select 5 delay chain tap"]
    _00000100,
    #[doc = "Select 6 delay chain tap"]
    _00000101,
    #[doc = "Select 7 delay chain tap"]
    _00000110,
    #[doc = "Select 8 delay chain tap"]
    _00000111,
    #[doc = "Select 9 delay chain tap"]
    _000001000,
    #[doc = "Select 10 delay chain tap"]
    _000001001,
    #[doc = "Select 11 delay chain tap"]
    _000001010,
    #[doc = "Select 12 delay chain tap"]
    _000001011,
    #[doc = "Select 13 delay chain tap"]
    _000001100,
    #[doc = "Select 14 delay chain tap"]
    _000001101,
    #[doc = "Select 15 delay chain tap"]
    _000001110,
    #[doc = "Select 16 delay chain tap"]
    _000001111,
    #[doc = "Select 17 delay chain tap"]
    _0000010000,
    #[doc = "Select 18 delay chain tap"]
    _0000010001,
    #[doc = "Select 19 delay chain tap"]
    _0000010010,
    #[doc = "Select 20 delay chain tap"]
    _0000010011,
    #[doc = "Select 21 delay chain tap"]
    _0000010100,
    #[doc = "Select 22 delay chain tap"]
    _0000010101,
    #[doc = "Select 23 delay chain tap"]
    _0000010110,
    #[doc = "Select 24 delay chain tap"]
    _0000010111,
    #[doc = "Select 25 delay chain tap"]
    _0000011000,
    #[doc = "Select 26 delay chain tap"]
    _0000011001,
    #[doc = "Select 27 delay chain tap"]
    _0000011010,
    #[doc = "Select 28 delay chain tap"]
    _0000011011,
    #[doc = "Select 29 delay chain tap"]
    _0000011100,
    #[doc = "Select 30 delay chain tap"]
    _0000011101,
    #[doc = "Select 31 delay chain tap"]
    _0000011110,
    #[doc = "Select 32 delay chain tap"]
    _0000011111,
    #[doc = "Select 33 delay chain tap"]
    _00000100000,
    #[doc = "Select 34 delay chain tap"]
    _00000100001,
    #[doc = "Select 35 delay chain tap"]
    _00000100010,
    #[doc = "Select 36 delay chain tap"]
    _00000100011,
    #[doc = "Select 37 delay chain tap"]
    _00000100100,
    #[doc = "Select 38 delay chain tap"]
    _00000100101,
    #[doc = "Select 39 delay chain tap"]
    _00000100110,
    #[doc = "Select 40 delay chain tap"]
    _00000100111,
    #[doc = "Select 41 delay chain tap"]
    _00000101000,
    #[doc = "Select 42 delay chain tap"]
    _00000101001,
    #[doc = "Select 43 delay chain tap"]
    _00000101010,
    #[doc = "Select 44 delay chain tap"]
    _00000101011,
    #[doc = "Select 45 delay chain tap"]
    _00000101100,
    #[doc = "Select 46 delay chain tap"]
    _00000101101,
    #[doc = "Select 47 delay chain tap"]
    _00000101110,
    #[doc = "Select 48 delay chain tap"]
    _00000101111,
    #[doc = "Select 49 delay chain tap"]
    _00000110000,
    #[doc = "Select 50 delay chain tap"]
    _00000110001,
    #[doc = "Select 51 delay chain tap"]
    _00000110010,
    #[doc = "Select 52 delay chain tap"]
    _00000110011,
    #[doc = "Select 53 delay chain tap"]
    _00000110100,
    #[doc = "Select 54 delay chain tap"]
    _00000110101,
    #[doc = "Select 55 delay chain tap"]
    _00000110110,
    #[doc = "Select 56 delay chain tap"]
    _00000110111,
    #[doc = "Select 57 delay chain tap"]
    _00000111000,
    #[doc = "Select 58 delay chain tap"]
    _00000111001,
    #[doc = "Select 59 delay chain tap"]
    _00000111010,
    #[doc = "Select 60 delay chain tap"]
    _00000111011,
    #[doc = "Select 61 delay chain tap"]
    _00000111100,
    #[doc = "Select 62 delay chain tap"]
    _00000111101,
    #[doc = "Select 63 delay chain tap"]
    _00000111110,
    #[doc = "Select 64 delay chain tap"]
    _00000111111,
}
impl DLYTAPSELAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DLYTAPSELAR::_000000 => 0,
            DLYTAPSELAR::_000001 => 1,
            DLYTAPSELAR::_0000010 => 2,
            DLYTAPSELAR::_0000011 => 3,
            DLYTAPSELAR::_00000100 => 4,
            DLYTAPSELAR::_00000101 => 5,
            DLYTAPSELAR::_00000110 => 6,
            DLYTAPSELAR::_00000111 => 7,
            DLYTAPSELAR::_000001000 => 8,
            DLYTAPSELAR::_000001001 => 9,
            DLYTAPSELAR::_000001010 => 10,
            DLYTAPSELAR::_000001011 => 11,
            DLYTAPSELAR::_000001100 => 12,
            DLYTAPSELAR::_000001101 => 13,
            DLYTAPSELAR::_000001110 => 14,
            DLYTAPSELAR::_000001111 => 15,
            DLYTAPSELAR::_0000010000 => 16,
            DLYTAPSELAR::_0000010001 => 17,
            DLYTAPSELAR::_0000010010 => 18,
            DLYTAPSELAR::_0000010011 => 19,
            DLYTAPSELAR::_0000010100 => 20,
            DLYTAPSELAR::_0000010101 => 21,
            DLYTAPSELAR::_0000010110 => 22,
            DLYTAPSELAR::_0000010111 => 23,
            DLYTAPSELAR::_0000011000 => 24,
            DLYTAPSELAR::_0000011001 => 25,
            DLYTAPSELAR::_0000011010 => 26,
            DLYTAPSELAR::_0000011011 => 27,
            DLYTAPSELAR::_0000011100 => 28,
            DLYTAPSELAR::_0000011101 => 29,
            DLYTAPSELAR::_0000011110 => 30,
            DLYTAPSELAR::_0000011111 => 31,
            DLYTAPSELAR::_00000100000 => 32,
            DLYTAPSELAR::_00000100001 => 33,
            DLYTAPSELAR::_00000100010 => 34,
            DLYTAPSELAR::_00000100011 => 35,
            DLYTAPSELAR::_00000100100 => 36,
            DLYTAPSELAR::_00000100101 => 37,
            DLYTAPSELAR::_00000100110 => 38,
            DLYTAPSELAR::_00000100111 => 39,
            DLYTAPSELAR::_00000101000 => 40,
            DLYTAPSELAR::_00000101001 => 41,
            DLYTAPSELAR::_00000101010 => 42,
            DLYTAPSELAR::_00000101011 => 43,
            DLYTAPSELAR::_00000101100 => 44,
            DLYTAPSELAR::_00000101101 => 45,
            DLYTAPSELAR::_00000101110 => 46,
            DLYTAPSELAR::_00000101111 => 47,
            DLYTAPSELAR::_00000110000 => 48,
            DLYTAPSELAR::_00000110001 => 49,
            DLYTAPSELAR::_00000110010 => 50,
            DLYTAPSELAR::_00000110011 => 51,
            DLYTAPSELAR::_00000110100 => 52,
            DLYTAPSELAR::_00000110101 => 53,
            DLYTAPSELAR::_00000110110 => 54,
            DLYTAPSELAR::_00000110111 => 55,
            DLYTAPSELAR::_00000111000 => 56,
            DLYTAPSELAR::_00000111001 => 57,
            DLYTAPSELAR::_00000111010 => 58,
            DLYTAPSELAR::_00000111011 => 59,
            DLYTAPSELAR::_00000111100 => 60,
            DLYTAPSELAR::_00000111101 => 61,
            DLYTAPSELAR::_00000111110 => 62,
            DLYTAPSELAR::_00000111111 => 63,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DLYTAPSELAR {
        match value {
            0 => DLYTAPSELAR::_000000,
            1 => DLYTAPSELAR::_000001,
            2 => DLYTAPSELAR::_0000010,
            3 => DLYTAPSELAR::_0000011,
            4 => DLYTAPSELAR::_00000100,
            5 => DLYTAPSELAR::_00000101,
            6 => DLYTAPSELAR::_00000110,
            7 => DLYTAPSELAR::_00000111,
            8 => DLYTAPSELAR::_000001000,
            9 => DLYTAPSELAR::_000001001,
            10 => DLYTAPSELAR::_000001010,
            11 => DLYTAPSELAR::_000001011,
            12 => DLYTAPSELAR::_000001100,
            13 => DLYTAPSELAR::_000001101,
            14 => DLYTAPSELAR::_000001110,
            15 => DLYTAPSELAR::_000001111,
            16 => DLYTAPSELAR::_0000010000,
            17 => DLYTAPSELAR::_0000010001,
            18 => DLYTAPSELAR::_0000010010,
            19 => DLYTAPSELAR::_0000010011,
            20 => DLYTAPSELAR::_0000010100,
            21 => DLYTAPSELAR::_0000010101,
            22 => DLYTAPSELAR::_0000010110,
            23 => DLYTAPSELAR::_0000010111,
            24 => DLYTAPSELAR::_0000011000,
            25 => DLYTAPSELAR::_0000011001,
            26 => DLYTAPSELAR::_0000011010,
            27 => DLYTAPSELAR::_0000011011,
            28 => DLYTAPSELAR::_0000011100,
            29 => DLYTAPSELAR::_0000011101,
            30 => DLYTAPSELAR::_0000011110,
            31 => DLYTAPSELAR::_0000011111,
            32 => DLYTAPSELAR::_00000100000,
            33 => DLYTAPSELAR::_00000100001,
            34 => DLYTAPSELAR::_00000100010,
            35 => DLYTAPSELAR::_00000100011,
            36 => DLYTAPSELAR::_00000100100,
            37 => DLYTAPSELAR::_00000100101,
            38 => DLYTAPSELAR::_00000100110,
            39 => DLYTAPSELAR::_00000100111,
            40 => DLYTAPSELAR::_00000101000,
            41 => DLYTAPSELAR::_00000101001,
            42 => DLYTAPSELAR::_00000101010,
            43 => DLYTAPSELAR::_00000101011,
            44 => DLYTAPSELAR::_00000101100,
            45 => DLYTAPSELAR::_00000101101,
            46 => DLYTAPSELAR::_00000101110,
            47 => DLYTAPSELAR::_00000101111,
            48 => DLYTAPSELAR::_00000110000,
            49 => DLYTAPSELAR::_00000110001,
            50 => DLYTAPSELAR::_00000110010,
            51 => DLYTAPSELAR::_00000110011,
            52 => DLYTAPSELAR::_00000110100,
            53 => DLYTAPSELAR::_00000110101,
            54 => DLYTAPSELAR::_00000110110,
            55 => DLYTAPSELAR::_00000110111,
            56 => DLYTAPSELAR::_00000111000,
            57 => DLYTAPSELAR::_00000111001,
            58 => DLYTAPSELAR::_00000111010,
            59 => DLYTAPSELAR::_00000111011,
            60 => DLYTAPSELAR::_00000111100,
            61 => DLYTAPSELAR::_00000111101,
            62 => DLYTAPSELAR::_00000111110,
            63 => DLYTAPSELAR::_00000111111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline]
    pub fn is_000000(&self) -> bool {
        *self == DLYTAPSELAR::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline]
    pub fn is_000001(&self) -> bool {
        *self == DLYTAPSELAR::_000001
    }
    #[doc = "Checks if the value of the field is `_0000010`"]
    #[inline]
    pub fn is_0000010(&self) -> bool {
        *self == DLYTAPSELAR::_0000010
    }
    #[doc = "Checks if the value of the field is `_0000011`"]
    #[inline]
    pub fn is_0000011(&self) -> bool {
        *self == DLYTAPSELAR::_0000011
    }
    #[doc = "Checks if the value of the field is `_00000100`"]
    #[inline]
    pub fn is_00000100(&self) -> bool {
        *self == DLYTAPSELAR::_00000100
    }
    #[doc = "Checks if the value of the field is `_00000101`"]
    #[inline]
    pub fn is_00000101(&self) -> bool {
        *self == DLYTAPSELAR::_00000101
    }
    #[doc = "Checks if the value of the field is `_00000110`"]
    #[inline]
    pub fn is_00000110(&self) -> bool {
        *self == DLYTAPSELAR::_00000110
    }
    #[doc = "Checks if the value of the field is `_00000111`"]
    #[inline]
    pub fn is_00000111(&self) -> bool {
        *self == DLYTAPSELAR::_00000111
    }
    #[doc = "Checks if the value of the field is `_000001000`"]
    #[inline]
    pub fn is_000001000(&self) -> bool {
        *self == DLYTAPSELAR::_000001000
    }
    #[doc = "Checks if the value of the field is `_000001001`"]
    #[inline]
    pub fn is_000001001(&self) -> bool {
        *self == DLYTAPSELAR::_000001001
    }
    #[doc = "Checks if the value of the field is `_000001010`"]
    #[inline]
    pub fn is_000001010(&self) -> bool {
        *self == DLYTAPSELAR::_000001010
    }
    #[doc = "Checks if the value of the field is `_000001011`"]
    #[inline]
    pub fn is_000001011(&self) -> bool {
        *self == DLYTAPSELAR::_000001011
    }
    #[doc = "Checks if the value of the field is `_000001100`"]
    #[inline]
    pub fn is_000001100(&self) -> bool {
        *self == DLYTAPSELAR::_000001100
    }
    #[doc = "Checks if the value of the field is `_000001101`"]
    #[inline]
    pub fn is_000001101(&self) -> bool {
        *self == DLYTAPSELAR::_000001101
    }
    #[doc = "Checks if the value of the field is `_000001110`"]
    #[inline]
    pub fn is_000001110(&self) -> bool {
        *self == DLYTAPSELAR::_000001110
    }
    #[doc = "Checks if the value of the field is `_000001111`"]
    #[inline]
    pub fn is_000001111(&self) -> bool {
        *self == DLYTAPSELAR::_000001111
    }
    #[doc = "Checks if the value of the field is `_0000010000`"]
    #[inline]
    pub fn is_0000010000(&self) -> bool {
        *self == DLYTAPSELAR::_0000010000
    }
    #[doc = "Checks if the value of the field is `_0000010001`"]
    #[inline]
    pub fn is_0000010001(&self) -> bool {
        *self == DLYTAPSELAR::_0000010001
    }
    #[doc = "Checks if the value of the field is `_0000010010`"]
    #[inline]
    pub fn is_0000010010(&self) -> bool {
        *self == DLYTAPSELAR::_0000010010
    }
    #[doc = "Checks if the value of the field is `_0000010011`"]
    #[inline]
    pub fn is_0000010011(&self) -> bool {
        *self == DLYTAPSELAR::_0000010011
    }
    #[doc = "Checks if the value of the field is `_0000010100`"]
    #[inline]
    pub fn is_0000010100(&self) -> bool {
        *self == DLYTAPSELAR::_0000010100
    }
    #[doc = "Checks if the value of the field is `_0000010101`"]
    #[inline]
    pub fn is_0000010101(&self) -> bool {
        *self == DLYTAPSELAR::_0000010101
    }
    #[doc = "Checks if the value of the field is `_0000010110`"]
    #[inline]
    pub fn is_0000010110(&self) -> bool {
        *self == DLYTAPSELAR::_0000010110
    }
    #[doc = "Checks if the value of the field is `_0000010111`"]
    #[inline]
    pub fn is_0000010111(&self) -> bool {
        *self == DLYTAPSELAR::_0000010111
    }
    #[doc = "Checks if the value of the field is `_0000011000`"]
    #[inline]
    pub fn is_0000011000(&self) -> bool {
        *self == DLYTAPSELAR::_0000011000
    }
    #[doc = "Checks if the value of the field is `_0000011001`"]
    #[inline]
    pub fn is_0000011001(&self) -> bool {
        *self == DLYTAPSELAR::_0000011001
    }
    #[doc = "Checks if the value of the field is `_0000011010`"]
    #[inline]
    pub fn is_0000011010(&self) -> bool {
        *self == DLYTAPSELAR::_0000011010
    }
    #[doc = "Checks if the value of the field is `_0000011011`"]
    #[inline]
    pub fn is_0000011011(&self) -> bool {
        *self == DLYTAPSELAR::_0000011011
    }
    #[doc = "Checks if the value of the field is `_0000011100`"]
    #[inline]
    pub fn is_0000011100(&self) -> bool {
        *self == DLYTAPSELAR::_0000011100
    }
    #[doc = "Checks if the value of the field is `_0000011101`"]
    #[inline]
    pub fn is_0000011101(&self) -> bool {
        *self == DLYTAPSELAR::_0000011101
    }
    #[doc = "Checks if the value of the field is `_0000011110`"]
    #[inline]
    pub fn is_0000011110(&self) -> bool {
        *self == DLYTAPSELAR::_0000011110
    }
    #[doc = "Checks if the value of the field is `_0000011111`"]
    #[inline]
    pub fn is_0000011111(&self) -> bool {
        *self == DLYTAPSELAR::_0000011111
    }
    #[doc = "Checks if the value of the field is `_00000100000`"]
    #[inline]
    pub fn is_00000100000(&self) -> bool {
        *self == DLYTAPSELAR::_00000100000
    }
    #[doc = "Checks if the value of the field is `_00000100001`"]
    #[inline]
    pub fn is_00000100001(&self) -> bool {
        *self == DLYTAPSELAR::_00000100001
    }
    #[doc = "Checks if the value of the field is `_00000100010`"]
    #[inline]
    pub fn is_00000100010(&self) -> bool {
        *self == DLYTAPSELAR::_00000100010
    }
    #[doc = "Checks if the value of the field is `_00000100011`"]
    #[inline]
    pub fn is_00000100011(&self) -> bool {
        *self == DLYTAPSELAR::_00000100011
    }
    #[doc = "Checks if the value of the field is `_00000100100`"]
    #[inline]
    pub fn is_00000100100(&self) -> bool {
        *self == DLYTAPSELAR::_00000100100
    }
    #[doc = "Checks if the value of the field is `_00000100101`"]
    #[inline]
    pub fn is_00000100101(&self) -> bool {
        *self == DLYTAPSELAR::_00000100101
    }
    #[doc = "Checks if the value of the field is `_00000100110`"]
    #[inline]
    pub fn is_00000100110(&self) -> bool {
        *self == DLYTAPSELAR::_00000100110
    }
    #[doc = "Checks if the value of the field is `_00000100111`"]
    #[inline]
    pub fn is_00000100111(&self) -> bool {
        *self == DLYTAPSELAR::_00000100111
    }
    #[doc = "Checks if the value of the field is `_00000101000`"]
    #[inline]
    pub fn is_00000101000(&self) -> bool {
        *self == DLYTAPSELAR::_00000101000
    }
    #[doc = "Checks if the value of the field is `_00000101001`"]
    #[inline]
    pub fn is_00000101001(&self) -> bool {
        *self == DLYTAPSELAR::_00000101001
    }
    #[doc = "Checks if the value of the field is `_00000101010`"]
    #[inline]
    pub fn is_00000101010(&self) -> bool {
        *self == DLYTAPSELAR::_00000101010
    }
    #[doc = "Checks if the value of the field is `_00000101011`"]
    #[inline]
    pub fn is_00000101011(&self) -> bool {
        *self == DLYTAPSELAR::_00000101011
    }
    #[doc = "Checks if the value of the field is `_00000101100`"]
    #[inline]
    pub fn is_00000101100(&self) -> bool {
        *self == DLYTAPSELAR::_00000101100
    }
    #[doc = "Checks if the value of the field is `_00000101101`"]
    #[inline]
    pub fn is_00000101101(&self) -> bool {
        *self == DLYTAPSELAR::_00000101101
    }
    #[doc = "Checks if the value of the field is `_00000101110`"]
    #[inline]
    pub fn is_00000101110(&self) -> bool {
        *self == DLYTAPSELAR::_00000101110
    }
    #[doc = "Checks if the value of the field is `_00000101111`"]
    #[inline]
    pub fn is_00000101111(&self) -> bool {
        *self == DLYTAPSELAR::_00000101111
    }
    #[doc = "Checks if the value of the field is `_00000110000`"]
    #[inline]
    pub fn is_00000110000(&self) -> bool {
        *self == DLYTAPSELAR::_00000110000
    }
    #[doc = "Checks if the value of the field is `_00000110001`"]
    #[inline]
    pub fn is_00000110001(&self) -> bool {
        *self == DLYTAPSELAR::_00000110001
    }
    #[doc = "Checks if the value of the field is `_00000110010`"]
    #[inline]
    pub fn is_00000110010(&self) -> bool {
        *self == DLYTAPSELAR::_00000110010
    }
    #[doc = "Checks if the value of the field is `_00000110011`"]
    #[inline]
    pub fn is_00000110011(&self) -> bool {
        *self == DLYTAPSELAR::_00000110011
    }
    #[doc = "Checks if the value of the field is `_00000110100`"]
    #[inline]
    pub fn is_00000110100(&self) -> bool {
        *self == DLYTAPSELAR::_00000110100
    }
    #[doc = "Checks if the value of the field is `_00000110101`"]
    #[inline]
    pub fn is_00000110101(&self) -> bool {
        *self == DLYTAPSELAR::_00000110101
    }
    #[doc = "Checks if the value of the field is `_00000110110`"]
    #[inline]
    pub fn is_00000110110(&self) -> bool {
        *self == DLYTAPSELAR::_00000110110
    }
    #[doc = "Checks if the value of the field is `_00000110111`"]
    #[inline]
    pub fn is_00000110111(&self) -> bool {
        *self == DLYTAPSELAR::_00000110111
    }
    #[doc = "Checks if the value of the field is `_00000111000`"]
    #[inline]
    pub fn is_00000111000(&self) -> bool {
        *self == DLYTAPSELAR::_00000111000
    }
    #[doc = "Checks if the value of the field is `_00000111001`"]
    #[inline]
    pub fn is_00000111001(&self) -> bool {
        *self == DLYTAPSELAR::_00000111001
    }
    #[doc = "Checks if the value of the field is `_00000111010`"]
    #[inline]
    pub fn is_00000111010(&self) -> bool {
        *self == DLYTAPSELAR::_00000111010
    }
    #[doc = "Checks if the value of the field is `_00000111011`"]
    #[inline]
    pub fn is_00000111011(&self) -> bool {
        *self == DLYTAPSELAR::_00000111011
    }
    #[doc = "Checks if the value of the field is `_00000111100`"]
    #[inline]
    pub fn is_00000111100(&self) -> bool {
        *self == DLYTAPSELAR::_00000111100
    }
    #[doc = "Checks if the value of the field is `_00000111101`"]
    #[inline]
    pub fn is_00000111101(&self) -> bool {
        *self == DLYTAPSELAR::_00000111101
    }
    #[doc = "Checks if the value of the field is `_00000111110`"]
    #[inline]
    pub fn is_00000111110(&self) -> bool {
        *self == DLYTAPSELAR::_00000111110
    }
    #[doc = "Checks if the value of the field is `_00000111111`"]
    #[inline]
    pub fn is_00000111111(&self) -> bool {
        *self == DLYTAPSELAR::_00000111111
    }
}
#[doc = "Possible values of the field `DLYTAPSELB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLYTAPSELBR {
    #[doc = "Select 1 delay chain tap"]
    _000000,
    #[doc = "Select 2 delay chain tap"]
    _000001,
    #[doc = "Select 3 delay chain tap"]
    _0000010,
    #[doc = "Select 4 delay chain tap"]
    _0000011,
    #[doc = "Select 5 delay chain tap"]
    _00000100,
    #[doc = "Select 6 delay chain tap"]
    _00000101,
    #[doc = "Select 7 delay chain tap"]
    _00000110,
    #[doc = "Select 8 delay chain tap"]
    _00000111,
    #[doc = "Select 9 delay chain tap"]
    _000001000,
    #[doc = "Select 10 delay chain tap"]
    _000001001,
    #[doc = "Select 11 delay chain tap"]
    _000001010,
    #[doc = "Select 12 delay chain tap"]
    _000001011,
    #[doc = "Select 13 delay chain tap"]
    _000001100,
    #[doc = "Select 14 delay chain tap"]
    _000001101,
    #[doc = "Select 15 delay chain tap"]
    _000001110,
    #[doc = "Select 16 delay chain tap"]
    _000001111,
    #[doc = "Select 17 delay chain tap"]
    _0000010000,
    #[doc = "Select 18 delay chain tap"]
    _0000010001,
    #[doc = "Select 19 delay chain tap"]
    _0000010010,
    #[doc = "Select 20 delay chain tap"]
    _0000010011,
    #[doc = "Select 21 delay chain tap"]
    _0000010100,
    #[doc = "Select 22 delay chain tap"]
    _0000010101,
    #[doc = "Select 23 delay chain tap"]
    _0000010110,
    #[doc = "Select 24 delay chain tap"]
    _0000010111,
    #[doc = "Select 25 delay chain tap"]
    _0000011000,
    #[doc = "Select 26 delay chain tap"]
    _0000011001,
    #[doc = "Select 27 delay chain tap"]
    _0000011010,
    #[doc = "Select 28 delay chain tap"]
    _0000011011,
    #[doc = "Select 29 delay chain tap"]
    _0000011100,
    #[doc = "Select 30 delay chain tap"]
    _0000011101,
    #[doc = "Select 31 delay chain tap"]
    _0000011110,
    #[doc = "Select 32 delay chain tap"]
    _0000011111,
    #[doc = "Select 33 delay chain tap"]
    _00000100000,
    #[doc = "Select 34 delay chain tap"]
    _00000100001,
    #[doc = "Select 35 delay chain tap"]
    _00000100010,
    #[doc = "Select 36 delay chain tap"]
    _00000100011,
    #[doc = "Select 37 delay chain tap"]
    _00000100100,
    #[doc = "Select 38 delay chain tap"]
    _00000100101,
    #[doc = "Select 39 delay chain tap"]
    _00000100110,
    #[doc = "Select 40 delay chain tap"]
    _00000100111,
    #[doc = "Select 41 delay chain tap"]
    _00000101000,
    #[doc = "Select 42 delay chain tap"]
    _00000101001,
    #[doc = "Select 43 delay chain tap"]
    _00000101010,
    #[doc = "Select 44 delay chain tap"]
    _00000101011,
    #[doc = "Select 45 delay chain tap"]
    _00000101100,
    #[doc = "Select 46 delay chain tap"]
    _00000101101,
    #[doc = "Select 47 delay chain tap"]
    _00000101110,
    #[doc = "Select 48 delay chain tap"]
    _00000101111,
    #[doc = "Select 49 delay chain tap"]
    _00000110000,
    #[doc = "Select 50 delay chain tap"]
    _00000110001,
    #[doc = "Select 51 delay chain tap"]
    _00000110010,
    #[doc = "Select 52 delay chain tap"]
    _00000110011,
    #[doc = "Select 53 delay chain tap"]
    _00000110100,
    #[doc = "Select 54 delay chain tap"]
    _00000110101,
    #[doc = "Select 55 delay chain tap"]
    _00000110110,
    #[doc = "Select 56 delay chain tap"]
    _00000110111,
    #[doc = "Select 57 delay chain tap"]
    _00000111000,
    #[doc = "Select 58 delay chain tap"]
    _00000111001,
    #[doc = "Select 59 delay chain tap"]
    _00000111010,
    #[doc = "Select 60 delay chain tap"]
    _00000111011,
    #[doc = "Select 61 delay chain tap"]
    _00000111100,
    #[doc = "Select 62 delay chain tap"]
    _00000111101,
    #[doc = "Select 63 delay chain tap"]
    _00000111110,
    #[doc = "Select 64 delay chain tap"]
    _00000111111,
}
impl DLYTAPSELBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DLYTAPSELBR::_000000 => 0,
            DLYTAPSELBR::_000001 => 1,
            DLYTAPSELBR::_0000010 => 2,
            DLYTAPSELBR::_0000011 => 3,
            DLYTAPSELBR::_00000100 => 4,
            DLYTAPSELBR::_00000101 => 5,
            DLYTAPSELBR::_00000110 => 6,
            DLYTAPSELBR::_00000111 => 7,
            DLYTAPSELBR::_000001000 => 8,
            DLYTAPSELBR::_000001001 => 9,
            DLYTAPSELBR::_000001010 => 10,
            DLYTAPSELBR::_000001011 => 11,
            DLYTAPSELBR::_000001100 => 12,
            DLYTAPSELBR::_000001101 => 13,
            DLYTAPSELBR::_000001110 => 14,
            DLYTAPSELBR::_000001111 => 15,
            DLYTAPSELBR::_0000010000 => 16,
            DLYTAPSELBR::_0000010001 => 17,
            DLYTAPSELBR::_0000010010 => 18,
            DLYTAPSELBR::_0000010011 => 19,
            DLYTAPSELBR::_0000010100 => 20,
            DLYTAPSELBR::_0000010101 => 21,
            DLYTAPSELBR::_0000010110 => 22,
            DLYTAPSELBR::_0000010111 => 23,
            DLYTAPSELBR::_0000011000 => 24,
            DLYTAPSELBR::_0000011001 => 25,
            DLYTAPSELBR::_0000011010 => 26,
            DLYTAPSELBR::_0000011011 => 27,
            DLYTAPSELBR::_0000011100 => 28,
            DLYTAPSELBR::_0000011101 => 29,
            DLYTAPSELBR::_0000011110 => 30,
            DLYTAPSELBR::_0000011111 => 31,
            DLYTAPSELBR::_00000100000 => 32,
            DLYTAPSELBR::_00000100001 => 33,
            DLYTAPSELBR::_00000100010 => 34,
            DLYTAPSELBR::_00000100011 => 35,
            DLYTAPSELBR::_00000100100 => 36,
            DLYTAPSELBR::_00000100101 => 37,
            DLYTAPSELBR::_00000100110 => 38,
            DLYTAPSELBR::_00000100111 => 39,
            DLYTAPSELBR::_00000101000 => 40,
            DLYTAPSELBR::_00000101001 => 41,
            DLYTAPSELBR::_00000101010 => 42,
            DLYTAPSELBR::_00000101011 => 43,
            DLYTAPSELBR::_00000101100 => 44,
            DLYTAPSELBR::_00000101101 => 45,
            DLYTAPSELBR::_00000101110 => 46,
            DLYTAPSELBR::_00000101111 => 47,
            DLYTAPSELBR::_00000110000 => 48,
            DLYTAPSELBR::_00000110001 => 49,
            DLYTAPSELBR::_00000110010 => 50,
            DLYTAPSELBR::_00000110011 => 51,
            DLYTAPSELBR::_00000110100 => 52,
            DLYTAPSELBR::_00000110101 => 53,
            DLYTAPSELBR::_00000110110 => 54,
            DLYTAPSELBR::_00000110111 => 55,
            DLYTAPSELBR::_00000111000 => 56,
            DLYTAPSELBR::_00000111001 => 57,
            DLYTAPSELBR::_00000111010 => 58,
            DLYTAPSELBR::_00000111011 => 59,
            DLYTAPSELBR::_00000111100 => 60,
            DLYTAPSELBR::_00000111101 => 61,
            DLYTAPSELBR::_00000111110 => 62,
            DLYTAPSELBR::_00000111111 => 63,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DLYTAPSELBR {
        match value {
            0 => DLYTAPSELBR::_000000,
            1 => DLYTAPSELBR::_000001,
            2 => DLYTAPSELBR::_0000010,
            3 => DLYTAPSELBR::_0000011,
            4 => DLYTAPSELBR::_00000100,
            5 => DLYTAPSELBR::_00000101,
            6 => DLYTAPSELBR::_00000110,
            7 => DLYTAPSELBR::_00000111,
            8 => DLYTAPSELBR::_000001000,
            9 => DLYTAPSELBR::_000001001,
            10 => DLYTAPSELBR::_000001010,
            11 => DLYTAPSELBR::_000001011,
            12 => DLYTAPSELBR::_000001100,
            13 => DLYTAPSELBR::_000001101,
            14 => DLYTAPSELBR::_000001110,
            15 => DLYTAPSELBR::_000001111,
            16 => DLYTAPSELBR::_0000010000,
            17 => DLYTAPSELBR::_0000010001,
            18 => DLYTAPSELBR::_0000010010,
            19 => DLYTAPSELBR::_0000010011,
            20 => DLYTAPSELBR::_0000010100,
            21 => DLYTAPSELBR::_0000010101,
            22 => DLYTAPSELBR::_0000010110,
            23 => DLYTAPSELBR::_0000010111,
            24 => DLYTAPSELBR::_0000011000,
            25 => DLYTAPSELBR::_0000011001,
            26 => DLYTAPSELBR::_0000011010,
            27 => DLYTAPSELBR::_0000011011,
            28 => DLYTAPSELBR::_0000011100,
            29 => DLYTAPSELBR::_0000011101,
            30 => DLYTAPSELBR::_0000011110,
            31 => DLYTAPSELBR::_0000011111,
            32 => DLYTAPSELBR::_00000100000,
            33 => DLYTAPSELBR::_00000100001,
            34 => DLYTAPSELBR::_00000100010,
            35 => DLYTAPSELBR::_00000100011,
            36 => DLYTAPSELBR::_00000100100,
            37 => DLYTAPSELBR::_00000100101,
            38 => DLYTAPSELBR::_00000100110,
            39 => DLYTAPSELBR::_00000100111,
            40 => DLYTAPSELBR::_00000101000,
            41 => DLYTAPSELBR::_00000101001,
            42 => DLYTAPSELBR::_00000101010,
            43 => DLYTAPSELBR::_00000101011,
            44 => DLYTAPSELBR::_00000101100,
            45 => DLYTAPSELBR::_00000101101,
            46 => DLYTAPSELBR::_00000101110,
            47 => DLYTAPSELBR::_00000101111,
            48 => DLYTAPSELBR::_00000110000,
            49 => DLYTAPSELBR::_00000110001,
            50 => DLYTAPSELBR::_00000110010,
            51 => DLYTAPSELBR::_00000110011,
            52 => DLYTAPSELBR::_00000110100,
            53 => DLYTAPSELBR::_00000110101,
            54 => DLYTAPSELBR::_00000110110,
            55 => DLYTAPSELBR::_00000110111,
            56 => DLYTAPSELBR::_00000111000,
            57 => DLYTAPSELBR::_00000111001,
            58 => DLYTAPSELBR::_00000111010,
            59 => DLYTAPSELBR::_00000111011,
            60 => DLYTAPSELBR::_00000111100,
            61 => DLYTAPSELBR::_00000111101,
            62 => DLYTAPSELBR::_00000111110,
            63 => DLYTAPSELBR::_00000111111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline]
    pub fn is_000000(&self) -> bool {
        *self == DLYTAPSELBR::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline]
    pub fn is_000001(&self) -> bool {
        *self == DLYTAPSELBR::_000001
    }
    #[doc = "Checks if the value of the field is `_0000010`"]
    #[inline]
    pub fn is_0000010(&self) -> bool {
        *self == DLYTAPSELBR::_0000010
    }
    #[doc = "Checks if the value of the field is `_0000011`"]
    #[inline]
    pub fn is_0000011(&self) -> bool {
        *self == DLYTAPSELBR::_0000011
    }
    #[doc = "Checks if the value of the field is `_00000100`"]
    #[inline]
    pub fn is_00000100(&self) -> bool {
        *self == DLYTAPSELBR::_00000100
    }
    #[doc = "Checks if the value of the field is `_00000101`"]
    #[inline]
    pub fn is_00000101(&self) -> bool {
        *self == DLYTAPSELBR::_00000101
    }
    #[doc = "Checks if the value of the field is `_00000110`"]
    #[inline]
    pub fn is_00000110(&self) -> bool {
        *self == DLYTAPSELBR::_00000110
    }
    #[doc = "Checks if the value of the field is `_00000111`"]
    #[inline]
    pub fn is_00000111(&self) -> bool {
        *self == DLYTAPSELBR::_00000111
    }
    #[doc = "Checks if the value of the field is `_000001000`"]
    #[inline]
    pub fn is_000001000(&self) -> bool {
        *self == DLYTAPSELBR::_000001000
    }
    #[doc = "Checks if the value of the field is `_000001001`"]
    #[inline]
    pub fn is_000001001(&self) -> bool {
        *self == DLYTAPSELBR::_000001001
    }
    #[doc = "Checks if the value of the field is `_000001010`"]
    #[inline]
    pub fn is_000001010(&self) -> bool {
        *self == DLYTAPSELBR::_000001010
    }
    #[doc = "Checks if the value of the field is `_000001011`"]
    #[inline]
    pub fn is_000001011(&self) -> bool {
        *self == DLYTAPSELBR::_000001011
    }
    #[doc = "Checks if the value of the field is `_000001100`"]
    #[inline]
    pub fn is_000001100(&self) -> bool {
        *self == DLYTAPSELBR::_000001100
    }
    #[doc = "Checks if the value of the field is `_000001101`"]
    #[inline]
    pub fn is_000001101(&self) -> bool {
        *self == DLYTAPSELBR::_000001101
    }
    #[doc = "Checks if the value of the field is `_000001110`"]
    #[inline]
    pub fn is_000001110(&self) -> bool {
        *self == DLYTAPSELBR::_000001110
    }
    #[doc = "Checks if the value of the field is `_000001111`"]
    #[inline]
    pub fn is_000001111(&self) -> bool {
        *self == DLYTAPSELBR::_000001111
    }
    #[doc = "Checks if the value of the field is `_0000010000`"]
    #[inline]
    pub fn is_0000010000(&self) -> bool {
        *self == DLYTAPSELBR::_0000010000
    }
    #[doc = "Checks if the value of the field is `_0000010001`"]
    #[inline]
    pub fn is_0000010001(&self) -> bool {
        *self == DLYTAPSELBR::_0000010001
    }
    #[doc = "Checks if the value of the field is `_0000010010`"]
    #[inline]
    pub fn is_0000010010(&self) -> bool {
        *self == DLYTAPSELBR::_0000010010
    }
    #[doc = "Checks if the value of the field is `_0000010011`"]
    #[inline]
    pub fn is_0000010011(&self) -> bool {
        *self == DLYTAPSELBR::_0000010011
    }
    #[doc = "Checks if the value of the field is `_0000010100`"]
    #[inline]
    pub fn is_0000010100(&self) -> bool {
        *self == DLYTAPSELBR::_0000010100
    }
    #[doc = "Checks if the value of the field is `_0000010101`"]
    #[inline]
    pub fn is_0000010101(&self) -> bool {
        *self == DLYTAPSELBR::_0000010101
    }
    #[doc = "Checks if the value of the field is `_0000010110`"]
    #[inline]
    pub fn is_0000010110(&self) -> bool {
        *self == DLYTAPSELBR::_0000010110
    }
    #[doc = "Checks if the value of the field is `_0000010111`"]
    #[inline]
    pub fn is_0000010111(&self) -> bool {
        *self == DLYTAPSELBR::_0000010111
    }
    #[doc = "Checks if the value of the field is `_0000011000`"]
    #[inline]
    pub fn is_0000011000(&self) -> bool {
        *self == DLYTAPSELBR::_0000011000
    }
    #[doc = "Checks if the value of the field is `_0000011001`"]
    #[inline]
    pub fn is_0000011001(&self) -> bool {
        *self == DLYTAPSELBR::_0000011001
    }
    #[doc = "Checks if the value of the field is `_0000011010`"]
    #[inline]
    pub fn is_0000011010(&self) -> bool {
        *self == DLYTAPSELBR::_0000011010
    }
    #[doc = "Checks if the value of the field is `_0000011011`"]
    #[inline]
    pub fn is_0000011011(&self) -> bool {
        *self == DLYTAPSELBR::_0000011011
    }
    #[doc = "Checks if the value of the field is `_0000011100`"]
    #[inline]
    pub fn is_0000011100(&self) -> bool {
        *self == DLYTAPSELBR::_0000011100
    }
    #[doc = "Checks if the value of the field is `_0000011101`"]
    #[inline]
    pub fn is_0000011101(&self) -> bool {
        *self == DLYTAPSELBR::_0000011101
    }
    #[doc = "Checks if the value of the field is `_0000011110`"]
    #[inline]
    pub fn is_0000011110(&self) -> bool {
        *self == DLYTAPSELBR::_0000011110
    }
    #[doc = "Checks if the value of the field is `_0000011111`"]
    #[inline]
    pub fn is_0000011111(&self) -> bool {
        *self == DLYTAPSELBR::_0000011111
    }
    #[doc = "Checks if the value of the field is `_00000100000`"]
    #[inline]
    pub fn is_00000100000(&self) -> bool {
        *self == DLYTAPSELBR::_00000100000
    }
    #[doc = "Checks if the value of the field is `_00000100001`"]
    #[inline]
    pub fn is_00000100001(&self) -> bool {
        *self == DLYTAPSELBR::_00000100001
    }
    #[doc = "Checks if the value of the field is `_00000100010`"]
    #[inline]
    pub fn is_00000100010(&self) -> bool {
        *self == DLYTAPSELBR::_00000100010
    }
    #[doc = "Checks if the value of the field is `_00000100011`"]
    #[inline]
    pub fn is_00000100011(&self) -> bool {
        *self == DLYTAPSELBR::_00000100011
    }
    #[doc = "Checks if the value of the field is `_00000100100`"]
    #[inline]
    pub fn is_00000100100(&self) -> bool {
        *self == DLYTAPSELBR::_00000100100
    }
    #[doc = "Checks if the value of the field is `_00000100101`"]
    #[inline]
    pub fn is_00000100101(&self) -> bool {
        *self == DLYTAPSELBR::_00000100101
    }
    #[doc = "Checks if the value of the field is `_00000100110`"]
    #[inline]
    pub fn is_00000100110(&self) -> bool {
        *self == DLYTAPSELBR::_00000100110
    }
    #[doc = "Checks if the value of the field is `_00000100111`"]
    #[inline]
    pub fn is_00000100111(&self) -> bool {
        *self == DLYTAPSELBR::_00000100111
    }
    #[doc = "Checks if the value of the field is `_00000101000`"]
    #[inline]
    pub fn is_00000101000(&self) -> bool {
        *self == DLYTAPSELBR::_00000101000
    }
    #[doc = "Checks if the value of the field is `_00000101001`"]
    #[inline]
    pub fn is_00000101001(&self) -> bool {
        *self == DLYTAPSELBR::_00000101001
    }
    #[doc = "Checks if the value of the field is `_00000101010`"]
    #[inline]
    pub fn is_00000101010(&self) -> bool {
        *self == DLYTAPSELBR::_00000101010
    }
    #[doc = "Checks if the value of the field is `_00000101011`"]
    #[inline]
    pub fn is_00000101011(&self) -> bool {
        *self == DLYTAPSELBR::_00000101011
    }
    #[doc = "Checks if the value of the field is `_00000101100`"]
    #[inline]
    pub fn is_00000101100(&self) -> bool {
        *self == DLYTAPSELBR::_00000101100
    }
    #[doc = "Checks if the value of the field is `_00000101101`"]
    #[inline]
    pub fn is_00000101101(&self) -> bool {
        *self == DLYTAPSELBR::_00000101101
    }
    #[doc = "Checks if the value of the field is `_00000101110`"]
    #[inline]
    pub fn is_00000101110(&self) -> bool {
        *self == DLYTAPSELBR::_00000101110
    }
    #[doc = "Checks if the value of the field is `_00000101111`"]
    #[inline]
    pub fn is_00000101111(&self) -> bool {
        *self == DLYTAPSELBR::_00000101111
    }
    #[doc = "Checks if the value of the field is `_00000110000`"]
    #[inline]
    pub fn is_00000110000(&self) -> bool {
        *self == DLYTAPSELBR::_00000110000
    }
    #[doc = "Checks if the value of the field is `_00000110001`"]
    #[inline]
    pub fn is_00000110001(&self) -> bool {
        *self == DLYTAPSELBR::_00000110001
    }
    #[doc = "Checks if the value of the field is `_00000110010`"]
    #[inline]
    pub fn is_00000110010(&self) -> bool {
        *self == DLYTAPSELBR::_00000110010
    }
    #[doc = "Checks if the value of the field is `_00000110011`"]
    #[inline]
    pub fn is_00000110011(&self) -> bool {
        *self == DLYTAPSELBR::_00000110011
    }
    #[doc = "Checks if the value of the field is `_00000110100`"]
    #[inline]
    pub fn is_00000110100(&self) -> bool {
        *self == DLYTAPSELBR::_00000110100
    }
    #[doc = "Checks if the value of the field is `_00000110101`"]
    #[inline]
    pub fn is_00000110101(&self) -> bool {
        *self == DLYTAPSELBR::_00000110101
    }
    #[doc = "Checks if the value of the field is `_00000110110`"]
    #[inline]
    pub fn is_00000110110(&self) -> bool {
        *self == DLYTAPSELBR::_00000110110
    }
    #[doc = "Checks if the value of the field is `_00000110111`"]
    #[inline]
    pub fn is_00000110111(&self) -> bool {
        *self == DLYTAPSELBR::_00000110111
    }
    #[doc = "Checks if the value of the field is `_00000111000`"]
    #[inline]
    pub fn is_00000111000(&self) -> bool {
        *self == DLYTAPSELBR::_00000111000
    }
    #[doc = "Checks if the value of the field is `_00000111001`"]
    #[inline]
    pub fn is_00000111001(&self) -> bool {
        *self == DLYTAPSELBR::_00000111001
    }
    #[doc = "Checks if the value of the field is `_00000111010`"]
    #[inline]
    pub fn is_00000111010(&self) -> bool {
        *self == DLYTAPSELBR::_00000111010
    }
    #[doc = "Checks if the value of the field is `_00000111011`"]
    #[inline]
    pub fn is_00000111011(&self) -> bool {
        *self == DLYTAPSELBR::_00000111011
    }
    #[doc = "Checks if the value of the field is `_00000111100`"]
    #[inline]
    pub fn is_00000111100(&self) -> bool {
        *self == DLYTAPSELBR::_00000111100
    }
    #[doc = "Checks if the value of the field is `_00000111101`"]
    #[inline]
    pub fn is_00000111101(&self) -> bool {
        *self == DLYTAPSELBR::_00000111101
    }
    #[doc = "Checks if the value of the field is `_00000111110`"]
    #[inline]
    pub fn is_00000111110(&self) -> bool {
        *self == DLYTAPSELBR::_00000111110
    }
    #[doc = "Checks if the value of the field is `_00000111111`"]
    #[inline]
    pub fn is_00000111111(&self) -> bool {
        *self == DLYTAPSELBR::_00000111111
    }
}
#[doc = "Values that can be written to the field `QSPISRC`"]
pub enum QSPISRCW {
    #[doc = "Core/system clock"]
    _000,
    #[doc = "MCGFLL clock"]
    _001,
    #[doc = "MCGPLL clock"]
    _010,
    #[doc = "MCGPLL 2x clock (DDR mode specific)"]
    _011,
    #[doc = "IRC48M clock"]
    _100,
    #[doc = "OSCERCLK clock"]
    _101,
    #[doc = "MCGIRCLK clock"]
    _110,
}
impl QSPISRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QSPISRCW::_000 => 0,
            QSPISRCW::_001 => 1,
            QSPISRCW::_010 => 2,
            QSPISRCW::_011 => 3,
            QSPISRCW::_100 => 4,
            QSPISRCW::_101 => 5,
            QSPISRCW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QSPISRCW<'a> {
    w: &'a mut W,
}
impl<'a> _QSPISRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QSPISRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Core/system clock"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(QSPISRCW::_000)
    }
    #[doc = "MCGFLL clock"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(QSPISRCW::_001)
    }
    #[doc = "MCGPLL clock"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(QSPISRCW::_010)
    }
    #[doc = "MCGPLL 2x clock (DDR mode specific)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(QSPISRCW::_011)
    }
    #[doc = "IRC48M clock"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(QSPISRCW::_100)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(QSPISRCW::_101)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(QSPISRCW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DQSLPEN`"]
pub enum DQSLPENW {
    #[doc = "DQS loop back is disabled"]
    _0,
    #[doc = "DQS loop back is enabled"]
    _1,
}
impl DQSLPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DQSLPENW::_0 => false,
            DQSLPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DQSLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DQSLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DQSLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DQS loop back is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQSLPENW::_0)
    }
    #[doc = "DQS loop back is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQSLPENW::_1)
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
#[doc = "Values that can be written to the field `DQSPADLPEN`"]
pub enum DQSPADLPENW {
    #[doc = "DQS loop back from DQS pad is disabled"]
    _0,
    #[doc = "DQS loop back from DQS pad is enabled"]
    _1,
}
impl DQSPADLPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DQSPADLPENW::_0 => false,
            DQSPADLPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DQSPADLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _DQSPADLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DQSPADLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DQS loop back from DQS pad is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQSPADLPENW::_0)
    }
    #[doc = "DQS loop back from DQS pad is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQSPADLPENW::_1)
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
#[doc = "Values that can be written to the field `DQSPHASEL`"]
pub enum DQSPHASELW {
    #[doc = "No phase shift"]
    _00,
    #[doc = "Select 45 degree phase shift"]
    _01,
    #[doc = "Select 90 degree phase shift"]
    _10,
    #[doc = "Select 135 degree phase shift"]
    _11,
}
impl DQSPHASELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DQSPHASELW::_00 => 0,
            DQSPHASELW::_01 => 1,
            DQSPHASELW::_10 => 2,
            DQSPHASELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DQSPHASELW<'a> {
    w: &'a mut W,
}
impl<'a> _DQSPHASELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DQSPHASELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No phase shift"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DQSPHASELW::_00)
    }
    #[doc = "Select 45 degree phase shift"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DQSPHASELW::_01)
    }
    #[doc = "Select 90 degree phase shift"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DQSPHASELW::_10)
    }
    #[doc = "Select 135 degree phase shift"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DQSPHASELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DQSINVSEL`"]
pub enum DQSINVSELW {
    #[doc = "Use 1x internal reference clock for the DQS generation"]
    _0,
    #[doc = "Use inverse 1x internal reference clock for the DQS generation"]
    _1,
}
impl DQSINVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DQSINVSELW::_0 => false,
            DQSINVSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DQSINVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DQSINVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DQSINVSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use 1x internal reference clock for the DQS generation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQSINVSELW::_0)
    }
    #[doc = "Use inverse 1x internal reference clock for the DQS generation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQSINVSELW::_1)
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
#[doc = "Values that can be written to the field `CK2EN`"]
pub enum CK2ENW {
    #[doc = "CK2 flash clock is disabled"]
    _0,
    #[doc = "CK2 flash clock is enabled"]
    _1,
}
impl CK2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CK2ENW::_0 => false,
            CK2ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CK2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CK2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CK2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CK2 flash clock is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CK2ENW::_0)
    }
    #[doc = "CK2 flash clock is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CK2ENW::_1)
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
#[doc = "Values that can be written to the field `DIFFCKEN`"]
pub enum DIFFCKENW {
    #[doc = "Differential flash clock is disabled"]
    _0,
    #[doc = "Differential flash clock is enabled"]
    _1,
}
impl DIFFCKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIFFCKENW::_0 => false,
            DIFFCKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIFFCKENW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFCKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIFFCKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Differential flash clock is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIFFCKENW::_0)
    }
    #[doc = "Differential flash clock is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIFFCKENW::_1)
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
#[doc = "Values that can be written to the field `OCTEN`"]
pub enum OCTENW {
    #[doc = "QSPI0B_DATAx pins are assigned to QSPI Port B"]
    _0,
    #[doc = "QSPI0B_DATAx pins are assigned to QSPI Port A"]
    _1,
}
impl OCTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCTENW::_0 => false,
            OCTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCTENW<'a> {
    w: &'a mut W,
}
impl<'a> _OCTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "QSPI0B_DATAx pins are assigned to QSPI Port B"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCTENW::_0)
    }
    #[doc = "QSPI0B_DATAx pins are assigned to QSPI Port A"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCTENW::_1)
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
#[doc = "Values that can be written to the field `DLYTAPSELA`"]
pub enum DLYTAPSELAW {
    #[doc = "Select 1 delay chain tap"]
    _000000,
    #[doc = "Select 2 delay chain tap"]
    _000001,
    #[doc = "Select 3 delay chain tap"]
    _0000010,
    #[doc = "Select 4 delay chain tap"]
    _0000011,
    #[doc = "Select 5 delay chain tap"]
    _00000100,
    #[doc = "Select 6 delay chain tap"]
    _00000101,
    #[doc = "Select 7 delay chain tap"]
    _00000110,
    #[doc = "Select 8 delay chain tap"]
    _00000111,
    #[doc = "Select 9 delay chain tap"]
    _000001000,
    #[doc = "Select 10 delay chain tap"]
    _000001001,
    #[doc = "Select 11 delay chain tap"]
    _000001010,
    #[doc = "Select 12 delay chain tap"]
    _000001011,
    #[doc = "Select 13 delay chain tap"]
    _000001100,
    #[doc = "Select 14 delay chain tap"]
    _000001101,
    #[doc = "Select 15 delay chain tap"]
    _000001110,
    #[doc = "Select 16 delay chain tap"]
    _000001111,
    #[doc = "Select 17 delay chain tap"]
    _0000010000,
    #[doc = "Select 18 delay chain tap"]
    _0000010001,
    #[doc = "Select 19 delay chain tap"]
    _0000010010,
    #[doc = "Select 20 delay chain tap"]
    _0000010011,
    #[doc = "Select 21 delay chain tap"]
    _0000010100,
    #[doc = "Select 22 delay chain tap"]
    _0000010101,
    #[doc = "Select 23 delay chain tap"]
    _0000010110,
    #[doc = "Select 24 delay chain tap"]
    _0000010111,
    #[doc = "Select 25 delay chain tap"]
    _0000011000,
    #[doc = "Select 26 delay chain tap"]
    _0000011001,
    #[doc = "Select 27 delay chain tap"]
    _0000011010,
    #[doc = "Select 28 delay chain tap"]
    _0000011011,
    #[doc = "Select 29 delay chain tap"]
    _0000011100,
    #[doc = "Select 30 delay chain tap"]
    _0000011101,
    #[doc = "Select 31 delay chain tap"]
    _0000011110,
    #[doc = "Select 32 delay chain tap"]
    _0000011111,
    #[doc = "Select 33 delay chain tap"]
    _00000100000,
    #[doc = "Select 34 delay chain tap"]
    _00000100001,
    #[doc = "Select 35 delay chain tap"]
    _00000100010,
    #[doc = "Select 36 delay chain tap"]
    _00000100011,
    #[doc = "Select 37 delay chain tap"]
    _00000100100,
    #[doc = "Select 38 delay chain tap"]
    _00000100101,
    #[doc = "Select 39 delay chain tap"]
    _00000100110,
    #[doc = "Select 40 delay chain tap"]
    _00000100111,
    #[doc = "Select 41 delay chain tap"]
    _00000101000,
    #[doc = "Select 42 delay chain tap"]
    _00000101001,
    #[doc = "Select 43 delay chain tap"]
    _00000101010,
    #[doc = "Select 44 delay chain tap"]
    _00000101011,
    #[doc = "Select 45 delay chain tap"]
    _00000101100,
    #[doc = "Select 46 delay chain tap"]
    _00000101101,
    #[doc = "Select 47 delay chain tap"]
    _00000101110,
    #[doc = "Select 48 delay chain tap"]
    _00000101111,
    #[doc = "Select 49 delay chain tap"]
    _00000110000,
    #[doc = "Select 50 delay chain tap"]
    _00000110001,
    #[doc = "Select 51 delay chain tap"]
    _00000110010,
    #[doc = "Select 52 delay chain tap"]
    _00000110011,
    #[doc = "Select 53 delay chain tap"]
    _00000110100,
    #[doc = "Select 54 delay chain tap"]
    _00000110101,
    #[doc = "Select 55 delay chain tap"]
    _00000110110,
    #[doc = "Select 56 delay chain tap"]
    _00000110111,
    #[doc = "Select 57 delay chain tap"]
    _00000111000,
    #[doc = "Select 58 delay chain tap"]
    _00000111001,
    #[doc = "Select 59 delay chain tap"]
    _00000111010,
    #[doc = "Select 60 delay chain tap"]
    _00000111011,
    #[doc = "Select 61 delay chain tap"]
    _00000111100,
    #[doc = "Select 62 delay chain tap"]
    _00000111101,
    #[doc = "Select 63 delay chain tap"]
    _00000111110,
    #[doc = "Select 64 delay chain tap"]
    _00000111111,
}
impl DLYTAPSELAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DLYTAPSELAW::_000000 => 0,
            DLYTAPSELAW::_000001 => 1,
            DLYTAPSELAW::_0000010 => 2,
            DLYTAPSELAW::_0000011 => 3,
            DLYTAPSELAW::_00000100 => 4,
            DLYTAPSELAW::_00000101 => 5,
            DLYTAPSELAW::_00000110 => 6,
            DLYTAPSELAW::_00000111 => 7,
            DLYTAPSELAW::_000001000 => 8,
            DLYTAPSELAW::_000001001 => 9,
            DLYTAPSELAW::_000001010 => 10,
            DLYTAPSELAW::_000001011 => 11,
            DLYTAPSELAW::_000001100 => 12,
            DLYTAPSELAW::_000001101 => 13,
            DLYTAPSELAW::_000001110 => 14,
            DLYTAPSELAW::_000001111 => 15,
            DLYTAPSELAW::_0000010000 => 16,
            DLYTAPSELAW::_0000010001 => 17,
            DLYTAPSELAW::_0000010010 => 18,
            DLYTAPSELAW::_0000010011 => 19,
            DLYTAPSELAW::_0000010100 => 20,
            DLYTAPSELAW::_0000010101 => 21,
            DLYTAPSELAW::_0000010110 => 22,
            DLYTAPSELAW::_0000010111 => 23,
            DLYTAPSELAW::_0000011000 => 24,
            DLYTAPSELAW::_0000011001 => 25,
            DLYTAPSELAW::_0000011010 => 26,
            DLYTAPSELAW::_0000011011 => 27,
            DLYTAPSELAW::_0000011100 => 28,
            DLYTAPSELAW::_0000011101 => 29,
            DLYTAPSELAW::_0000011110 => 30,
            DLYTAPSELAW::_0000011111 => 31,
            DLYTAPSELAW::_00000100000 => 32,
            DLYTAPSELAW::_00000100001 => 33,
            DLYTAPSELAW::_00000100010 => 34,
            DLYTAPSELAW::_00000100011 => 35,
            DLYTAPSELAW::_00000100100 => 36,
            DLYTAPSELAW::_00000100101 => 37,
            DLYTAPSELAW::_00000100110 => 38,
            DLYTAPSELAW::_00000100111 => 39,
            DLYTAPSELAW::_00000101000 => 40,
            DLYTAPSELAW::_00000101001 => 41,
            DLYTAPSELAW::_00000101010 => 42,
            DLYTAPSELAW::_00000101011 => 43,
            DLYTAPSELAW::_00000101100 => 44,
            DLYTAPSELAW::_00000101101 => 45,
            DLYTAPSELAW::_00000101110 => 46,
            DLYTAPSELAW::_00000101111 => 47,
            DLYTAPSELAW::_00000110000 => 48,
            DLYTAPSELAW::_00000110001 => 49,
            DLYTAPSELAW::_00000110010 => 50,
            DLYTAPSELAW::_00000110011 => 51,
            DLYTAPSELAW::_00000110100 => 52,
            DLYTAPSELAW::_00000110101 => 53,
            DLYTAPSELAW::_00000110110 => 54,
            DLYTAPSELAW::_00000110111 => 55,
            DLYTAPSELAW::_00000111000 => 56,
            DLYTAPSELAW::_00000111001 => 57,
            DLYTAPSELAW::_00000111010 => 58,
            DLYTAPSELAW::_00000111011 => 59,
            DLYTAPSELAW::_00000111100 => 60,
            DLYTAPSELAW::_00000111101 => 61,
            DLYTAPSELAW::_00000111110 => 62,
            DLYTAPSELAW::_00000111111 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLYTAPSELAW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYTAPSELAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLYTAPSELAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select 1 delay chain tap"]
    #[inline]
    pub fn _000000(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000000)
    }
    #[doc = "Select 2 delay chain tap"]
    #[inline]
    pub fn _000001(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001)
    }
    #[doc = "Select 3 delay chain tap"]
    #[inline]
    pub fn _0000010(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010)
    }
    #[doc = "Select 4 delay chain tap"]
    #[inline]
    pub fn _0000011(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011)
    }
    #[doc = "Select 5 delay chain tap"]
    #[inline]
    pub fn _00000100(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100)
    }
    #[doc = "Select 6 delay chain tap"]
    #[inline]
    pub fn _00000101(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101)
    }
    #[doc = "Select 7 delay chain tap"]
    #[inline]
    pub fn _00000110(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110)
    }
    #[doc = "Select 8 delay chain tap"]
    #[inline]
    pub fn _00000111(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111)
    }
    #[doc = "Select 9 delay chain tap"]
    #[inline]
    pub fn _000001000(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001000)
    }
    #[doc = "Select 10 delay chain tap"]
    #[inline]
    pub fn _000001001(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001001)
    }
    #[doc = "Select 11 delay chain tap"]
    #[inline]
    pub fn _000001010(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001010)
    }
    #[doc = "Select 12 delay chain tap"]
    #[inline]
    pub fn _000001011(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001011)
    }
    #[doc = "Select 13 delay chain tap"]
    #[inline]
    pub fn _000001100(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001100)
    }
    #[doc = "Select 14 delay chain tap"]
    #[inline]
    pub fn _000001101(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001101)
    }
    #[doc = "Select 15 delay chain tap"]
    #[inline]
    pub fn _000001110(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001110)
    }
    #[doc = "Select 16 delay chain tap"]
    #[inline]
    pub fn _000001111(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_000001111)
    }
    #[doc = "Select 17 delay chain tap"]
    #[inline]
    pub fn _0000010000(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010000)
    }
    #[doc = "Select 18 delay chain tap"]
    #[inline]
    pub fn _0000010001(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010001)
    }
    #[doc = "Select 19 delay chain tap"]
    #[inline]
    pub fn _0000010010(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010010)
    }
    #[doc = "Select 20 delay chain tap"]
    #[inline]
    pub fn _0000010011(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010011)
    }
    #[doc = "Select 21 delay chain tap"]
    #[inline]
    pub fn _0000010100(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010100)
    }
    #[doc = "Select 22 delay chain tap"]
    #[inline]
    pub fn _0000010101(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010101)
    }
    #[doc = "Select 23 delay chain tap"]
    #[inline]
    pub fn _0000010110(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010110)
    }
    #[doc = "Select 24 delay chain tap"]
    #[inline]
    pub fn _0000010111(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000010111)
    }
    #[doc = "Select 25 delay chain tap"]
    #[inline]
    pub fn _0000011000(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011000)
    }
    #[doc = "Select 26 delay chain tap"]
    #[inline]
    pub fn _0000011001(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011001)
    }
    #[doc = "Select 27 delay chain tap"]
    #[inline]
    pub fn _0000011010(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011010)
    }
    #[doc = "Select 28 delay chain tap"]
    #[inline]
    pub fn _0000011011(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011011)
    }
    #[doc = "Select 29 delay chain tap"]
    #[inline]
    pub fn _0000011100(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011100)
    }
    #[doc = "Select 30 delay chain tap"]
    #[inline]
    pub fn _0000011101(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011101)
    }
    #[doc = "Select 31 delay chain tap"]
    #[inline]
    pub fn _0000011110(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011110)
    }
    #[doc = "Select 32 delay chain tap"]
    #[inline]
    pub fn _0000011111(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_0000011111)
    }
    #[doc = "Select 33 delay chain tap"]
    #[inline]
    pub fn _00000100000(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100000)
    }
    #[doc = "Select 34 delay chain tap"]
    #[inline]
    pub fn _00000100001(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100001)
    }
    #[doc = "Select 35 delay chain tap"]
    #[inline]
    pub fn _00000100010(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100010)
    }
    #[doc = "Select 36 delay chain tap"]
    #[inline]
    pub fn _00000100011(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100011)
    }
    #[doc = "Select 37 delay chain tap"]
    #[inline]
    pub fn _00000100100(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100100)
    }
    #[doc = "Select 38 delay chain tap"]
    #[inline]
    pub fn _00000100101(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100101)
    }
    #[doc = "Select 39 delay chain tap"]
    #[inline]
    pub fn _00000100110(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100110)
    }
    #[doc = "Select 40 delay chain tap"]
    #[inline]
    pub fn _00000100111(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000100111)
    }
    #[doc = "Select 41 delay chain tap"]
    #[inline]
    pub fn _00000101000(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101000)
    }
    #[doc = "Select 42 delay chain tap"]
    #[inline]
    pub fn _00000101001(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101001)
    }
    #[doc = "Select 43 delay chain tap"]
    #[inline]
    pub fn _00000101010(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101010)
    }
    #[doc = "Select 44 delay chain tap"]
    #[inline]
    pub fn _00000101011(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101011)
    }
    #[doc = "Select 45 delay chain tap"]
    #[inline]
    pub fn _00000101100(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101100)
    }
    #[doc = "Select 46 delay chain tap"]
    #[inline]
    pub fn _00000101101(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101101)
    }
    #[doc = "Select 47 delay chain tap"]
    #[inline]
    pub fn _00000101110(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101110)
    }
    #[doc = "Select 48 delay chain tap"]
    #[inline]
    pub fn _00000101111(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000101111)
    }
    #[doc = "Select 49 delay chain tap"]
    #[inline]
    pub fn _00000110000(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110000)
    }
    #[doc = "Select 50 delay chain tap"]
    #[inline]
    pub fn _00000110001(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110001)
    }
    #[doc = "Select 51 delay chain tap"]
    #[inline]
    pub fn _00000110010(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110010)
    }
    #[doc = "Select 52 delay chain tap"]
    #[inline]
    pub fn _00000110011(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110011)
    }
    #[doc = "Select 53 delay chain tap"]
    #[inline]
    pub fn _00000110100(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110100)
    }
    #[doc = "Select 54 delay chain tap"]
    #[inline]
    pub fn _00000110101(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110101)
    }
    #[doc = "Select 55 delay chain tap"]
    #[inline]
    pub fn _00000110110(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110110)
    }
    #[doc = "Select 56 delay chain tap"]
    #[inline]
    pub fn _00000110111(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000110111)
    }
    #[doc = "Select 57 delay chain tap"]
    #[inline]
    pub fn _00000111000(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111000)
    }
    #[doc = "Select 58 delay chain tap"]
    #[inline]
    pub fn _00000111001(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111001)
    }
    #[doc = "Select 59 delay chain tap"]
    #[inline]
    pub fn _00000111010(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111010)
    }
    #[doc = "Select 60 delay chain tap"]
    #[inline]
    pub fn _00000111011(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111011)
    }
    #[doc = "Select 61 delay chain tap"]
    #[inline]
    pub fn _00000111100(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111100)
    }
    #[doc = "Select 62 delay chain tap"]
    #[inline]
    pub fn _00000111101(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111101)
    }
    #[doc = "Select 63 delay chain tap"]
    #[inline]
    pub fn _00000111110(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111110)
    }
    #[doc = "Select 64 delay chain tap"]
    #[inline]
    pub fn _00000111111(self) -> &'a mut W {
        self.variant(DLYTAPSELAW::_00000111111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DLYTAPSELB`"]
pub enum DLYTAPSELBW {
    #[doc = "Select 1 delay chain tap"]
    _000000,
    #[doc = "Select 2 delay chain tap"]
    _000001,
    #[doc = "Select 3 delay chain tap"]
    _0000010,
    #[doc = "Select 4 delay chain tap"]
    _0000011,
    #[doc = "Select 5 delay chain tap"]
    _00000100,
    #[doc = "Select 6 delay chain tap"]
    _00000101,
    #[doc = "Select 7 delay chain tap"]
    _00000110,
    #[doc = "Select 8 delay chain tap"]
    _00000111,
    #[doc = "Select 9 delay chain tap"]
    _000001000,
    #[doc = "Select 10 delay chain tap"]
    _000001001,
    #[doc = "Select 11 delay chain tap"]
    _000001010,
    #[doc = "Select 12 delay chain tap"]
    _000001011,
    #[doc = "Select 13 delay chain tap"]
    _000001100,
    #[doc = "Select 14 delay chain tap"]
    _000001101,
    #[doc = "Select 15 delay chain tap"]
    _000001110,
    #[doc = "Select 16 delay chain tap"]
    _000001111,
    #[doc = "Select 17 delay chain tap"]
    _0000010000,
    #[doc = "Select 18 delay chain tap"]
    _0000010001,
    #[doc = "Select 19 delay chain tap"]
    _0000010010,
    #[doc = "Select 20 delay chain tap"]
    _0000010011,
    #[doc = "Select 21 delay chain tap"]
    _0000010100,
    #[doc = "Select 22 delay chain tap"]
    _0000010101,
    #[doc = "Select 23 delay chain tap"]
    _0000010110,
    #[doc = "Select 24 delay chain tap"]
    _0000010111,
    #[doc = "Select 25 delay chain tap"]
    _0000011000,
    #[doc = "Select 26 delay chain tap"]
    _0000011001,
    #[doc = "Select 27 delay chain tap"]
    _0000011010,
    #[doc = "Select 28 delay chain tap"]
    _0000011011,
    #[doc = "Select 29 delay chain tap"]
    _0000011100,
    #[doc = "Select 30 delay chain tap"]
    _0000011101,
    #[doc = "Select 31 delay chain tap"]
    _0000011110,
    #[doc = "Select 32 delay chain tap"]
    _0000011111,
    #[doc = "Select 33 delay chain tap"]
    _00000100000,
    #[doc = "Select 34 delay chain tap"]
    _00000100001,
    #[doc = "Select 35 delay chain tap"]
    _00000100010,
    #[doc = "Select 36 delay chain tap"]
    _00000100011,
    #[doc = "Select 37 delay chain tap"]
    _00000100100,
    #[doc = "Select 38 delay chain tap"]
    _00000100101,
    #[doc = "Select 39 delay chain tap"]
    _00000100110,
    #[doc = "Select 40 delay chain tap"]
    _00000100111,
    #[doc = "Select 41 delay chain tap"]
    _00000101000,
    #[doc = "Select 42 delay chain tap"]
    _00000101001,
    #[doc = "Select 43 delay chain tap"]
    _00000101010,
    #[doc = "Select 44 delay chain tap"]
    _00000101011,
    #[doc = "Select 45 delay chain tap"]
    _00000101100,
    #[doc = "Select 46 delay chain tap"]
    _00000101101,
    #[doc = "Select 47 delay chain tap"]
    _00000101110,
    #[doc = "Select 48 delay chain tap"]
    _00000101111,
    #[doc = "Select 49 delay chain tap"]
    _00000110000,
    #[doc = "Select 50 delay chain tap"]
    _00000110001,
    #[doc = "Select 51 delay chain tap"]
    _00000110010,
    #[doc = "Select 52 delay chain tap"]
    _00000110011,
    #[doc = "Select 53 delay chain tap"]
    _00000110100,
    #[doc = "Select 54 delay chain tap"]
    _00000110101,
    #[doc = "Select 55 delay chain tap"]
    _00000110110,
    #[doc = "Select 56 delay chain tap"]
    _00000110111,
    #[doc = "Select 57 delay chain tap"]
    _00000111000,
    #[doc = "Select 58 delay chain tap"]
    _00000111001,
    #[doc = "Select 59 delay chain tap"]
    _00000111010,
    #[doc = "Select 60 delay chain tap"]
    _00000111011,
    #[doc = "Select 61 delay chain tap"]
    _00000111100,
    #[doc = "Select 62 delay chain tap"]
    _00000111101,
    #[doc = "Select 63 delay chain tap"]
    _00000111110,
    #[doc = "Select 64 delay chain tap"]
    _00000111111,
}
impl DLYTAPSELBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DLYTAPSELBW::_000000 => 0,
            DLYTAPSELBW::_000001 => 1,
            DLYTAPSELBW::_0000010 => 2,
            DLYTAPSELBW::_0000011 => 3,
            DLYTAPSELBW::_00000100 => 4,
            DLYTAPSELBW::_00000101 => 5,
            DLYTAPSELBW::_00000110 => 6,
            DLYTAPSELBW::_00000111 => 7,
            DLYTAPSELBW::_000001000 => 8,
            DLYTAPSELBW::_000001001 => 9,
            DLYTAPSELBW::_000001010 => 10,
            DLYTAPSELBW::_000001011 => 11,
            DLYTAPSELBW::_000001100 => 12,
            DLYTAPSELBW::_000001101 => 13,
            DLYTAPSELBW::_000001110 => 14,
            DLYTAPSELBW::_000001111 => 15,
            DLYTAPSELBW::_0000010000 => 16,
            DLYTAPSELBW::_0000010001 => 17,
            DLYTAPSELBW::_0000010010 => 18,
            DLYTAPSELBW::_0000010011 => 19,
            DLYTAPSELBW::_0000010100 => 20,
            DLYTAPSELBW::_0000010101 => 21,
            DLYTAPSELBW::_0000010110 => 22,
            DLYTAPSELBW::_0000010111 => 23,
            DLYTAPSELBW::_0000011000 => 24,
            DLYTAPSELBW::_0000011001 => 25,
            DLYTAPSELBW::_0000011010 => 26,
            DLYTAPSELBW::_0000011011 => 27,
            DLYTAPSELBW::_0000011100 => 28,
            DLYTAPSELBW::_0000011101 => 29,
            DLYTAPSELBW::_0000011110 => 30,
            DLYTAPSELBW::_0000011111 => 31,
            DLYTAPSELBW::_00000100000 => 32,
            DLYTAPSELBW::_00000100001 => 33,
            DLYTAPSELBW::_00000100010 => 34,
            DLYTAPSELBW::_00000100011 => 35,
            DLYTAPSELBW::_00000100100 => 36,
            DLYTAPSELBW::_00000100101 => 37,
            DLYTAPSELBW::_00000100110 => 38,
            DLYTAPSELBW::_00000100111 => 39,
            DLYTAPSELBW::_00000101000 => 40,
            DLYTAPSELBW::_00000101001 => 41,
            DLYTAPSELBW::_00000101010 => 42,
            DLYTAPSELBW::_00000101011 => 43,
            DLYTAPSELBW::_00000101100 => 44,
            DLYTAPSELBW::_00000101101 => 45,
            DLYTAPSELBW::_00000101110 => 46,
            DLYTAPSELBW::_00000101111 => 47,
            DLYTAPSELBW::_00000110000 => 48,
            DLYTAPSELBW::_00000110001 => 49,
            DLYTAPSELBW::_00000110010 => 50,
            DLYTAPSELBW::_00000110011 => 51,
            DLYTAPSELBW::_00000110100 => 52,
            DLYTAPSELBW::_00000110101 => 53,
            DLYTAPSELBW::_00000110110 => 54,
            DLYTAPSELBW::_00000110111 => 55,
            DLYTAPSELBW::_00000111000 => 56,
            DLYTAPSELBW::_00000111001 => 57,
            DLYTAPSELBW::_00000111010 => 58,
            DLYTAPSELBW::_00000111011 => 59,
            DLYTAPSELBW::_00000111100 => 60,
            DLYTAPSELBW::_00000111101 => 61,
            DLYTAPSELBW::_00000111110 => 62,
            DLYTAPSELBW::_00000111111 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLYTAPSELBW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYTAPSELBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLYTAPSELBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select 1 delay chain tap"]
    #[inline]
    pub fn _000000(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000000)
    }
    #[doc = "Select 2 delay chain tap"]
    #[inline]
    pub fn _000001(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001)
    }
    #[doc = "Select 3 delay chain tap"]
    #[inline]
    pub fn _0000010(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010)
    }
    #[doc = "Select 4 delay chain tap"]
    #[inline]
    pub fn _0000011(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011)
    }
    #[doc = "Select 5 delay chain tap"]
    #[inline]
    pub fn _00000100(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100)
    }
    #[doc = "Select 6 delay chain tap"]
    #[inline]
    pub fn _00000101(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101)
    }
    #[doc = "Select 7 delay chain tap"]
    #[inline]
    pub fn _00000110(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110)
    }
    #[doc = "Select 8 delay chain tap"]
    #[inline]
    pub fn _00000111(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111)
    }
    #[doc = "Select 9 delay chain tap"]
    #[inline]
    pub fn _000001000(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001000)
    }
    #[doc = "Select 10 delay chain tap"]
    #[inline]
    pub fn _000001001(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001001)
    }
    #[doc = "Select 11 delay chain tap"]
    #[inline]
    pub fn _000001010(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001010)
    }
    #[doc = "Select 12 delay chain tap"]
    #[inline]
    pub fn _000001011(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001011)
    }
    #[doc = "Select 13 delay chain tap"]
    #[inline]
    pub fn _000001100(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001100)
    }
    #[doc = "Select 14 delay chain tap"]
    #[inline]
    pub fn _000001101(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001101)
    }
    #[doc = "Select 15 delay chain tap"]
    #[inline]
    pub fn _000001110(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001110)
    }
    #[doc = "Select 16 delay chain tap"]
    #[inline]
    pub fn _000001111(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_000001111)
    }
    #[doc = "Select 17 delay chain tap"]
    #[inline]
    pub fn _0000010000(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010000)
    }
    #[doc = "Select 18 delay chain tap"]
    #[inline]
    pub fn _0000010001(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010001)
    }
    #[doc = "Select 19 delay chain tap"]
    #[inline]
    pub fn _0000010010(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010010)
    }
    #[doc = "Select 20 delay chain tap"]
    #[inline]
    pub fn _0000010011(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010011)
    }
    #[doc = "Select 21 delay chain tap"]
    #[inline]
    pub fn _0000010100(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010100)
    }
    #[doc = "Select 22 delay chain tap"]
    #[inline]
    pub fn _0000010101(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010101)
    }
    #[doc = "Select 23 delay chain tap"]
    #[inline]
    pub fn _0000010110(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010110)
    }
    #[doc = "Select 24 delay chain tap"]
    #[inline]
    pub fn _0000010111(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000010111)
    }
    #[doc = "Select 25 delay chain tap"]
    #[inline]
    pub fn _0000011000(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011000)
    }
    #[doc = "Select 26 delay chain tap"]
    #[inline]
    pub fn _0000011001(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011001)
    }
    #[doc = "Select 27 delay chain tap"]
    #[inline]
    pub fn _0000011010(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011010)
    }
    #[doc = "Select 28 delay chain tap"]
    #[inline]
    pub fn _0000011011(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011011)
    }
    #[doc = "Select 29 delay chain tap"]
    #[inline]
    pub fn _0000011100(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011100)
    }
    #[doc = "Select 30 delay chain tap"]
    #[inline]
    pub fn _0000011101(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011101)
    }
    #[doc = "Select 31 delay chain tap"]
    #[inline]
    pub fn _0000011110(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011110)
    }
    #[doc = "Select 32 delay chain tap"]
    #[inline]
    pub fn _0000011111(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_0000011111)
    }
    #[doc = "Select 33 delay chain tap"]
    #[inline]
    pub fn _00000100000(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100000)
    }
    #[doc = "Select 34 delay chain tap"]
    #[inline]
    pub fn _00000100001(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100001)
    }
    #[doc = "Select 35 delay chain tap"]
    #[inline]
    pub fn _00000100010(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100010)
    }
    #[doc = "Select 36 delay chain tap"]
    #[inline]
    pub fn _00000100011(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100011)
    }
    #[doc = "Select 37 delay chain tap"]
    #[inline]
    pub fn _00000100100(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100100)
    }
    #[doc = "Select 38 delay chain tap"]
    #[inline]
    pub fn _00000100101(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100101)
    }
    #[doc = "Select 39 delay chain tap"]
    #[inline]
    pub fn _00000100110(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100110)
    }
    #[doc = "Select 40 delay chain tap"]
    #[inline]
    pub fn _00000100111(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000100111)
    }
    #[doc = "Select 41 delay chain tap"]
    #[inline]
    pub fn _00000101000(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101000)
    }
    #[doc = "Select 42 delay chain tap"]
    #[inline]
    pub fn _00000101001(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101001)
    }
    #[doc = "Select 43 delay chain tap"]
    #[inline]
    pub fn _00000101010(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101010)
    }
    #[doc = "Select 44 delay chain tap"]
    #[inline]
    pub fn _00000101011(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101011)
    }
    #[doc = "Select 45 delay chain tap"]
    #[inline]
    pub fn _00000101100(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101100)
    }
    #[doc = "Select 46 delay chain tap"]
    #[inline]
    pub fn _00000101101(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101101)
    }
    #[doc = "Select 47 delay chain tap"]
    #[inline]
    pub fn _00000101110(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101110)
    }
    #[doc = "Select 48 delay chain tap"]
    #[inline]
    pub fn _00000101111(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000101111)
    }
    #[doc = "Select 49 delay chain tap"]
    #[inline]
    pub fn _00000110000(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110000)
    }
    #[doc = "Select 50 delay chain tap"]
    #[inline]
    pub fn _00000110001(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110001)
    }
    #[doc = "Select 51 delay chain tap"]
    #[inline]
    pub fn _00000110010(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110010)
    }
    #[doc = "Select 52 delay chain tap"]
    #[inline]
    pub fn _00000110011(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110011)
    }
    #[doc = "Select 53 delay chain tap"]
    #[inline]
    pub fn _00000110100(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110100)
    }
    #[doc = "Select 54 delay chain tap"]
    #[inline]
    pub fn _00000110101(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110101)
    }
    #[doc = "Select 55 delay chain tap"]
    #[inline]
    pub fn _00000110110(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110110)
    }
    #[doc = "Select 56 delay chain tap"]
    #[inline]
    pub fn _00000110111(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000110111)
    }
    #[doc = "Select 57 delay chain tap"]
    #[inline]
    pub fn _00000111000(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111000)
    }
    #[doc = "Select 58 delay chain tap"]
    #[inline]
    pub fn _00000111001(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111001)
    }
    #[doc = "Select 59 delay chain tap"]
    #[inline]
    pub fn _00000111010(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111010)
    }
    #[doc = "Select 60 delay chain tap"]
    #[inline]
    pub fn _00000111011(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111011)
    }
    #[doc = "Select 61 delay chain tap"]
    #[inline]
    pub fn _00000111100(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111100)
    }
    #[doc = "Select 62 delay chain tap"]
    #[inline]
    pub fn _00000111101(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111101)
    }
    #[doc = "Select 63 delay chain tap"]
    #[inline]
    pub fn _00000111110(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111110)
    }
    #[doc = "Select 64 delay chain tap"]
    #[inline]
    pub fn _00000111111(self) -> &'a mut W {
        self.variant(DLYTAPSELBW::_00000111111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - QSPI clock source select"]
    #[inline]
    pub fn qspisrc(&self) -> QSPISRCR {
        QSPISRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - When this bit is set the internal generated DQS is selected and looped back to QuadSPI, without going to DQS pad. DQSPADLPEN should be cleared when this bit is set."]
    #[inline]
    pub fn dqslpen(&self) -> DQSLPENR {
        DQSLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - When this bit is set the internal generated DQS will be sent to the DQS pad first and then looped back to QuadSPI. DQSLPEN should be cleared when this bit is set."]
    #[inline]
    pub fn dqspadlpen(&self) -> DQSPADLPENR {
        DQSPADLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Select phase shift for internal DQS generation. These bits are always zero in SDR mode."]
    #[inline]
    pub fn dqsphasel(&self) -> DQSPHASELR {
        DQSPHASELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Select clock source for internal DQS generation"]
    #[inline]
    pub fn dqsinvsel(&self) -> DQSINVSELR {
        DQSINVSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Flash CK2 clock pin enable"]
    #[inline]
    pub fn ck2en(&self) -> CK2ENR {
        CK2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Differential flash clock pins enable"]
    #[inline]
    pub fn diffcken(&self) -> DIFFCKENR {
        DIFFCKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Octal data pins enable"]
    #[inline]
    pub fn octen(&self) -> OCTENR {
        OCTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - Delay chain tap number selection for QSPI Port A DQS"]
    #[inline]
    pub fn dlytapsela(&self) -> DLYTAPSELAR {
        DLYTAPSELAR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - Delay chain tap number selection for QSPI Port B DQS"]
    #[inline]
    pub fn dlytapselb(&self) -> DLYTAPSELBR {
        DLYTAPSELBR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - QSPI clock source select"]
    #[inline]
    pub fn qspisrc(&mut self) -> _QSPISRCW {
        _QSPISRCW { w: self }
    }
    #[doc = "Bit 8 - When this bit is set the internal generated DQS is selected and looped back to QuadSPI, without going to DQS pad. DQSPADLPEN should be cleared when this bit is set."]
    #[inline]
    pub fn dqslpen(&mut self) -> _DQSLPENW {
        _DQSLPENW { w: self }
    }
    #[doc = "Bit 9 - When this bit is set the internal generated DQS will be sent to the DQS pad first and then looped back to QuadSPI. DQSLPEN should be cleared when this bit is set."]
    #[inline]
    pub fn dqspadlpen(&mut self) -> _DQSPADLPENW {
        _DQSPADLPENW { w: self }
    }
    #[doc = "Bits 10:11 - Select phase shift for internal DQS generation. These bits are always zero in SDR mode."]
    #[inline]
    pub fn dqsphasel(&mut self) -> _DQSPHASELW {
        _DQSPHASELW { w: self }
    }
    #[doc = "Bit 12 - Select clock source for internal DQS generation"]
    #[inline]
    pub fn dqsinvsel(&mut self) -> _DQSINVSELW {
        _DQSINVSELW { w: self }
    }
    #[doc = "Bit 13 - Flash CK2 clock pin enable"]
    #[inline]
    pub fn ck2en(&mut self) -> _CK2ENW {
        _CK2ENW { w: self }
    }
    #[doc = "Bit 14 - Differential flash clock pins enable"]
    #[inline]
    pub fn diffcken(&mut self) -> _DIFFCKENW {
        _DIFFCKENW { w: self }
    }
    #[doc = "Bit 15 - Octal data pins enable"]
    #[inline]
    pub fn octen(&mut self) -> _OCTENW {
        _OCTENW { w: self }
    }
    #[doc = "Bits 16:21 - Delay chain tap number selection for QSPI Port A DQS"]
    #[inline]
    pub fn dlytapsela(&mut self) -> _DLYTAPSELAW {
        _DLYTAPSELAW { w: self }
    }
    #[doc = "Bits 24:29 - Delay chain tap number selection for QSPI Port B DQS"]
    #[inline]
    pub fn dlytapselb(&mut self) -> _DLYTAPSELBW {
        _DLYTAPSELBW { w: self }
    }
}
