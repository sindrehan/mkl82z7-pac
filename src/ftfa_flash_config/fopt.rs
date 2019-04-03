#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FOPT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `LPBOOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOTR {
    #[doc = "Low-power boot"]
    _00,
    #[doc = "Normal boot"]
    _01,
}
impl LPBOOTR {
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
            LPBOOTR::_00 => false,
            LPBOOTR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPBOOTR {
        match value {
            false => LPBOOTR::_00,
            true => LPBOOTR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPBOOTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPBOOTR::_01
    }
}
#[doc = "Possible values of the field `BOOTPIN_OPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTPIN_OPTR {
    #[doc = "Force Boot from ROM if BOOTCFG0 asserted, where BOOTCFG0 is the boot config function which is muxed with NMI pin"]
    _00,
    #[doc = "Boot source configured by FOPT (BOOTSRC_SEL) bits"]
    _01,
}
impl BOOTPIN_OPTR {
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
            BOOTPIN_OPTR::_00 => false,
            BOOTPIN_OPTR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOTPIN_OPTR {
        match value {
            false => BOOTPIN_OPTR::_00,
            true => BOOTPIN_OPTR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == BOOTPIN_OPTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == BOOTPIN_OPTR::_01
    }
}
#[doc = "Possible values of the field `NMI_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_DISR {
    #[doc = "NMI interrupts are always blocked"]
    _00,
    #[doc = "NMI_b pin/interrupts reset default to enabled"]
    _01,
}
impl NMI_DISR {
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
            NMI_DISR::_00 => false,
            NMI_DISR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMI_DISR {
        match value {
            false => NMI_DISR::_00,
            true => NMI_DISR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == NMI_DISR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == NMI_DISR::_01
    }
}
#[doc = "Possible values of the field `FAST_INIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_INITR {
    #[doc = "Slower initialization"]
    _00,
    #[doc = "Fast Initialization"]
    _01,
}
impl FAST_INITR {
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
            FAST_INITR::_00 => false,
            FAST_INITR::_01 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAST_INITR {
        match value {
            false => FAST_INITR::_00,
            true => FAST_INITR::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FAST_INITR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FAST_INITR::_01
    }
}
#[doc = "Possible values of the field `BOOTSRC_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTSRC_SELR {
    #[doc = "Boot from Flash"]
    _00,
    #[doc = "Boot from ROM, configure QSPI0, and enter boot loader mode."]
    _10,
    #[doc = "Boot from ROM and enter boot loader mode."]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BOOTSRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BOOTSRC_SELR::_00 => 0,
            BOOTSRC_SELR::_10 => 2,
            BOOTSRC_SELR::_11 => 3,
            BOOTSRC_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BOOTSRC_SELR {
        match value {
            0 => BOOTSRC_SELR::_00,
            2 => BOOTSRC_SELR::_10,
            3 => BOOTSRC_SELR::_11,
            i => BOOTSRC_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == BOOTSRC_SELR::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == BOOTSRC_SELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == BOOTSRC_SELR::_11
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn lpboot(&self) -> LPBOOTR {
        LPBOOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn bootpin_opt(&self) -> BOOTPIN_OPTR {
        BOOTPIN_OPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn nmi_dis(&self) -> NMI_DISR {
        NMI_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - no description available"]
    #[inline]
    pub fn fast_init(&self) -> FAST_INITR {
        FAST_INITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 6:7 - Boot source selection"]
    #[inline]
    pub fn bootsrc_sel(&self) -> BOOTSRC_SELR {
        BOOTSRC_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
