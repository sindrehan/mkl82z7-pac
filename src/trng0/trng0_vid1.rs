#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRNG0_VID1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TRNG0_MIN_REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG0_MIN_REVR {
    #[doc = "Minor revision number for TRNG."]
    _0X00,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRNG0_MIN_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRNG0_MIN_REVR::_0X00 => 0,
            TRNG0_MIN_REVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRNG0_MIN_REVR {
        match value {
            0 => TRNG0_MIN_REVR::_0X00,
            i => TRNG0_MIN_REVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_MIN_REVR::_0X00
    }
}
#[doc = "Possible values of the field `TRNG0_MAJ_REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG0_MAJ_REVR {
    #[doc = "Major revision number for TRNG."]
    _0X01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRNG0_MAJ_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRNG0_MAJ_REVR::_0X01 => 1,
            TRNG0_MAJ_REVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRNG0_MAJ_REVR {
        match value {
            1 => TRNG0_MAJ_REVR::_0X01,
            i => TRNG0_MAJ_REVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline]
    pub fn is_0x01(&self) -> bool {
        *self == TRNG0_MAJ_REVR::_0X01
    }
}
#[doc = r" Value of the field"]
pub struct TRNG0_IP_IDR {
    bits: u16,
}
impl TRNG0_IP_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Shows the Freescale IP's Minor revision of the TRNG."]
    #[inline]
    pub fn trng0_min_rev(&self) -> TRNG0_MIN_REVR {
        TRNG0_MIN_REVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Shows the Freescale IP's Major revision of the TRNG."]
    #[inline]
    pub fn trng0_maj_rev(&self) -> TRNG0_MAJ_REVR {
        TRNG0_MAJ_REVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:31 - Shows the Freescale IP ID."]
    #[inline]
    pub fn trng0_ip_id(&self) -> TRNG0_IP_IDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TRNG0_IP_IDR { bits }
    }
}
