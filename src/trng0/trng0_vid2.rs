#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRNG0_VID2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TRNG0_CONFIG_OPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG0_CONFIG_OPTR {
    #[doc = "TRNG_CONFIG_OPT for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRNG0_CONFIG_OPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRNG0_CONFIG_OPTR::_0X00 => 0,
            TRNG0_CONFIG_OPTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRNG0_CONFIG_OPTR {
        match value {
            0 => TRNG0_CONFIG_OPTR::_0X00,
            i => TRNG0_CONFIG_OPTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_CONFIG_OPTR::_0X00
    }
}
#[doc = "Possible values of the field `TRNG0_ECO_REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG0_ECO_REVR {
    #[doc = "TRNG_ECO_REV for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRNG0_ECO_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRNG0_ECO_REVR::_0X00 => 0,
            TRNG0_ECO_REVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRNG0_ECO_REVR {
        match value {
            0 => TRNG0_ECO_REVR::_0X00,
            i => TRNG0_ECO_REVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_ECO_REVR::_0X00
    }
}
#[doc = "Possible values of the field `TRNG0_INTG_OPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG0_INTG_OPTR {
    #[doc = "INTG_OPT for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRNG0_INTG_OPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRNG0_INTG_OPTR::_0X00 => 0,
            TRNG0_INTG_OPTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRNG0_INTG_OPTR {
        match value {
            0 => TRNG0_INTG_OPTR::_0X00,
            i => TRNG0_INTG_OPTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_INTG_OPTR::_0X00
    }
}
#[doc = "Possible values of the field `TRNG0_ERA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG0_ERAR {
    #[doc = "COMPILE_OPT for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRNG0_ERAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRNG0_ERAR::_0X00 => 0,
            TRNG0_ERAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRNG0_ERAR {
        match value {
            0 => TRNG0_ERAR::_0X00,
            i => TRNG0_ERAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_ERAR::_0X00
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Shows the Freescale IP's Configuaration options for the TRNG."]
    #[inline]
    pub fn trng0_config_opt(&self) -> TRNG0_CONFIG_OPTR {
        TRNG0_CONFIG_OPTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Shows the Freescale IP's ECO revision of the TRNG."]
    #[inline]
    pub fn trng0_eco_rev(&self) -> TRNG0_ECO_REVR {
        TRNG0_ECO_REVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Shows the Freescale integration options for the TRNG."]
    #[inline]
    pub fn trng0_intg_opt(&self) -> TRNG0_INTG_OPTR {
        TRNG0_INTG_OPTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Shows the Freescale compile options for the TRNG."]
    #[inline]
    pub fn trng0_era(&self) -> TRNG0_ERAR {
        TRNG0_ERAR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
