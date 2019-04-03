#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LTC0_CHAVID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct AESREVR {
    bits: u8,
}
impl AESREVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AESVIDR {
    bits: u8,
}
impl AESVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DESREVR {
    bits: u8,
}
impl DESREVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DESVIDR {
    bits: u8,
}
impl DESVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKHAREVR {
    bits: u8,
}
impl PKHAREVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKHAVIDR {
    bits: u8,
}
impl PKHAVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MDHAREVR {
    bits: u8,
}
impl MDHAREVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MDHAVIDR {
    bits: u8,
}
impl MDHAVIDR {
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
    #[doc = "Bits 0:3 - AES Revision Number"]
    #[inline]
    pub fn aesrev(&self) -> AESREVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AESREVR { bits }
    }
    #[doc = "Bits 4:7 - AES Version ID"]
    #[inline]
    pub fn aesvid(&self) -> AESVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AESVIDR { bits }
    }
    #[doc = "Bits 8:11 - DES Revision Number"]
    #[inline]
    pub fn desrev(&self) -> DESREVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DESREVR { bits }
    }
    #[doc = "Bits 12:15 - DES Version ID(0x0). 0000 - High-performance DESA 0001 - Low-performance DESA"]
    #[inline]
    pub fn desvid(&self) -> DESVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DESVIDR { bits }
    }
    #[doc = "Bits 16:19 - PK Revision Number"]
    #[inline]
    pub fn pkharev(&self) -> PKHAREVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PKHAREVR { bits }
    }
    #[doc = "Bits 20:23 - PK Version ID 0001 - 32-bit PKHA-SD 0010 - 64-bit PKHA-SD 0011 - 128-bit PKHA-SD 0100 - 16-bit PKHA-SD"]
    #[inline]
    pub fn pkhavid(&self) -> PKHAVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PKHAVIDR { bits }
    }
    #[doc = "Bits 24:27 - MDHA Revision Number"]
    #[inline]
    pub fn mdharev(&self) -> MDHAREVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MDHAREVR { bits }
    }
    #[doc = "Bits 28:31 - MDHA Hashing Version ID"]
    #[inline]
    pub fn mdhavid(&self) -> MDHAVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MDHAVIDR { bits }
    }
}
