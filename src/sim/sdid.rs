#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SDID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PINID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINIDR {
    #[doc = "64-pin"]
    _0101,
    #[doc = "80-pin"]
    _0110,
    #[doc = "100-pin"]
    _1000,
    #[doc = "121-pin"]
    _1001,
    #[doc = "Custom pinout (WLCSP)"]
    _1011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PINIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINIDR::_0101 => 5,
            PINIDR::_0110 => 6,
            PINIDR::_1000 => 8,
            PINIDR::_1001 => 9,
            PINIDR::_1011 => 11,
            PINIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINIDR {
        match value {
            5 => PINIDR::_0101,
            6 => PINIDR::_0110,
            8 => PINIDR::_1000,
            9 => PINIDR::_1001,
            11 => PINIDR::_1011,
            i => PINIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PINIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PINIDR::_0110
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PINIDR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == PINIDR::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == PINIDR::_1011
    }
}
#[doc = r" Value of the field"]
pub struct FAMIDR {
    bits: u8,
}
impl FAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIEIDR {
    bits: u8,
}
impl DIEIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REVIDR {
    bits: u8,
}
impl REVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SUBFAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBFAMIDR {
    #[doc = "KLx0 Subfamily"]
    _0000,
    #[doc = "KLx1 Subfamily"]
    _0001,
    #[doc = "KLx2 Subfamily"]
    _0010,
    #[doc = "KLx3 Subfamily"]
    _0011,
    #[doc = "KLx4 Subfamily"]
    _0100,
    #[doc = "KLx5 Subfamily"]
    _0101,
    #[doc = "KLx6 Subfamily"]
    _0110,
    #[doc = "KLx7 Subfamily"]
    _0111,
    #[doc = "KLx8 Subfamily"]
    _1000,
    #[doc = "KLx9 Subfamily"]
    _1001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SUBFAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUBFAMIDR::_0000 => 0,
            SUBFAMIDR::_0001 => 1,
            SUBFAMIDR::_0010 => 2,
            SUBFAMIDR::_0011 => 3,
            SUBFAMIDR::_0100 => 4,
            SUBFAMIDR::_0101 => 5,
            SUBFAMIDR::_0110 => 6,
            SUBFAMIDR::_0111 => 7,
            SUBFAMIDR::_1000 => 8,
            SUBFAMIDR::_1001 => 9,
            SUBFAMIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUBFAMIDR {
        match value {
            0 => SUBFAMIDR::_0000,
            1 => SUBFAMIDR::_0001,
            2 => SUBFAMIDR::_0010,
            3 => SUBFAMIDR::_0011,
            4 => SUBFAMIDR::_0100,
            5 => SUBFAMIDR::_0101,
            6 => SUBFAMIDR::_0110,
            7 => SUBFAMIDR::_0111,
            8 => SUBFAMIDR::_1000,
            9 => SUBFAMIDR::_1001,
            i => SUBFAMIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == SUBFAMIDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == SUBFAMIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == SUBFAMIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == SUBFAMIDR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == SUBFAMIDR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == SUBFAMIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == SUBFAMIDR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == SUBFAMIDR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == SUBFAMIDR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == SUBFAMIDR::_1001
    }
}
#[doc = "Possible values of the field `FAMILYID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMILYIDR {
    #[doc = "KL0x Family"]
    _0000,
    #[doc = "KL1x Family"]
    _0001,
    #[doc = "KL2x Family"]
    _0010,
    #[doc = "KL3x Family)"]
    _0011,
    #[doc = "KL4x Family)"]
    _0100,
    #[doc = "KL6x Family"]
    _0110,
    #[doc = "KL7x Family"]
    _0111,
    #[doc = "KL8x Family"]
    _1001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FAMILYIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAMILYIDR::_0000 => 0,
            FAMILYIDR::_0001 => 1,
            FAMILYIDR::_0010 => 2,
            FAMILYIDR::_0011 => 3,
            FAMILYIDR::_0100 => 4,
            FAMILYIDR::_0110 => 6,
            FAMILYIDR::_0111 => 7,
            FAMILYIDR::_1001 => 9,
            FAMILYIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAMILYIDR {
        match value {
            0 => FAMILYIDR::_0000,
            1 => FAMILYIDR::_0001,
            2 => FAMILYIDR::_0010,
            3 => FAMILYIDR::_0011,
            4 => FAMILYIDR::_0100,
            6 => FAMILYIDR::_0110,
            7 => FAMILYIDR::_0111,
            9 => FAMILYIDR::_1001,
            i => FAMILYIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == FAMILYIDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == FAMILYIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == FAMILYIDR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == FAMILYIDR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == FAMILYIDR::_0100
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == FAMILYIDR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == FAMILYIDR::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == FAMILYIDR::_1001
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline]
    pub fn pinid(&self) -> PINIDR {
        PINIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Kinetis family ID"]
    #[inline]
    pub fn famid(&self) -> FAMIDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FAMIDR { bits }
    }
    #[doc = "Bits 7:11 - Device die number"]
    #[inline]
    pub fn dieid(&self) -> DIEIDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIEIDR { bits }
    }
    #[doc = "Bits 12:15 - Device Revision Number"]
    #[inline]
    pub fn revid(&self) -> REVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVIDR { bits }
    }
    #[doc = "Bits 24:27 - Kinetis Sub-Family ID"]
    #[inline]
    pub fn subfamid(&self) -> SUBFAMIDR {
        SUBFAMIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Kinetis L family ID"]
    #[inline]
    pub fn familyid(&self) -> FAMILYIDR {
        FAMILYIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
