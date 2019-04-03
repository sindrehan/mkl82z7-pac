#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CH_VEC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct VECNR {
    bits: u16,
}
impl VECNR {
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
    #[doc = "Bits 2:13 - Vector Number"]
    #[inline]
    pub fn vecn(&self) -> VECNR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECNR { bits }
    }
}
