#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TBSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TRBFLR {
    bits: u8,
}
impl TRBFLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRCTRR {
    bits: u16,
}
impl TRCTRR {
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
    #[doc = "Bits 8:12 - TX Buffer Fill Level"]
    #[inline]
    pub fn trbfl(&self) -> TRBFLR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRBFLR { bits }
    }
    #[doc = "Bits 16:31 - Transmit Counter"]
    #[inline]
    pub fn trctr(&self) -> TRCTRR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TRCTRR { bits }
    }
}
