#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SPNDST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SUSPNDR {
    bits: bool,
}
impl SUSPNDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SPDBUFR {
    bits: u8,
}
impl SPDBUFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATLFTR {
    bits: u8,
}
impl DATLFTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When set, it signifies that a sequence is in suspended state"]
    #[inline]
    pub fn suspnd(&self) -> SUSPNDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUSPNDR { bits }
    }
    #[doc = "Bits 6:7 - Suspended Buffer: Provides the suspended buffer number. Valid only when SUSPND is set to 1'b1"]
    #[inline]
    pub fn spdbuf(&self) -> SPDBUFR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPDBUFR { bits }
    }
    #[doc = "Bits 9:14 - Data left: Provides information about the amount of data left to be read in the suspended sequence"]
    #[inline]
    pub fn datlft(&self) -> DATLFTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATLFTR { bits }
    }
}
