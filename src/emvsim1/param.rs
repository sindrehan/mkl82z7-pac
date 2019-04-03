#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PARAM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RX_FIFO_DEPTHR {
    bits: u8,
}
impl RX_FIFO_DEPTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TX_FIFO_DEPTHR {
    bits: u8,
}
impl TX_FIFO_DEPTHR {
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
    #[doc = "Bits 0:7 - Receive FIFO Depth"]
    #[inline]
    pub fn rx_fifo_depth(&self) -> RX_FIFO_DEPTHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_FIFO_DEPTHR { bits }
    }
    #[doc = "Bits 8:15 - Transmit FIFO Depth"]
    #[inline]
    pub fn tx_fifo_depth(&self) -> TX_FIFO_DEPTHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TX_FIFO_DEPTHR { bits }
    }
}
