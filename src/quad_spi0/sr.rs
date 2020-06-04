#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `IP_ACC`"]
pub type IP_ACC_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB_ACC`"]
pub type AHB_ACC_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHBGNT`"]
pub type AHBGNT_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHBTRN`"]
pub type AHBTRN_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB0NE`"]
pub type AHB0NE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB1NE`"]
pub type AHB1NE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB2NE`"]
pub type AHB2NE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB3NE`"]
pub type AHB3NE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB0FUL`"]
pub type AHB0FUL_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB1FUL`"]
pub type AHB1FUL_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB2FUL`"]
pub type AHB2FUL_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB3FUL`"]
pub type AHB3FUL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXWE`"]
pub type RXWE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDMA`"]
pub type RXDMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEDA`"]
pub type TXEDA_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXWA`"]
pub type TXWA_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDMA`"]
pub type TXDMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFULL`"]
pub type TXFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DLPSMP`"]
pub type DLPSMP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Module Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IP Access. Asserted when transaction currently executed was initiated by IP bus."]
    #[inline(always)]
    pub fn ip_acc(&self) -> IP_ACC_R {
        IP_ACC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Access. Asserted when the transaction currently executed was initiated by AHB bus."]
    #[inline(always)]
    pub fn ahb_acc(&self) -> AHB_ACC_R {
        AHB_ACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AHB Command priority Granted: Asserted when another module has been granted priority of AHB Commands against IP Commands"]
    #[inline(always)]
    pub fn ahbgnt(&self) -> AHBGNT_R {
        AHBGNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AHB Access Transaction pending"]
    #[inline(always)]
    pub fn ahbtrn(&self) -> AHBTRN_R {
        AHBTRN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AHB 0 Buffer Not Empty. Asserted when AHB 0 buffer contains data."]
    #[inline(always)]
    pub fn ahb0ne(&self) -> AHB0NE_R {
        AHB0NE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AHB 1 Buffer Not Empty. Asserted when AHB 1 buffer contains data."]
    #[inline(always)]
    pub fn ahb1ne(&self) -> AHB1NE_R {
        AHB1NE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AHB 2 Buffer Not Empty. Asserted when AHB 2 buffer contains data."]
    #[inline(always)]
    pub fn ahb2ne(&self) -> AHB2NE_R {
        AHB2NE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AHB 3 Buffer Not Empty. Asserted when AHB 3 buffer contains data."]
    #[inline(always)]
    pub fn ahb3ne(&self) -> AHB3NE_R {
        AHB3NE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AHB 0 Buffer Full. Asserted when AHB 0 buffer is full."]
    #[inline(always)]
    pub fn ahb0ful(&self) -> AHB0FUL_R {
        AHB0FUL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AHB 1 Buffer Full. Asserted when AHB 1 buffer is full."]
    #[inline(always)]
    pub fn ahb1ful(&self) -> AHB1FUL_R {
        AHB1FUL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AHB 2 Buffer Full. Asserted when AHB 2 buffer is full."]
    #[inline(always)]
    pub fn ahb2ful(&self) -> AHB2FUL_R {
        AHB2FUL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AHB 3 Buffer Full. Asserted when AHB 3 buffer is full."]
    #[inline(always)]
    pub fn ahb3ful(&self) -> AHB3FUL_R {
        AHB3FUL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX Buffer Watermark Exceeded"]
    #[inline(always)]
    pub fn rxwe(&self) -> RXWE_R {
        RXWE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - RX Buffer DMA. Asserted when RX Buffer read out via DMA is active i.e DMA is requested or running."]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Tx Buffer Enough Data Available"]
    #[inline(always)]
    pub fn txeda(&self) -> TXEDA_R {
        TXEDA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TX Buffer watermark Available"]
    #[inline(always)]
    pub fn txwa(&self) -> TXWA_R {
        TXWA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TXDMA"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TX Buffer Full. Asserted when no more data can be stored."]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - Data learning pattern sampling point"]
    #[inline(always)]
    pub fn dlpsmp(&self) -> DLPSMP_R {
        DLPSMP_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
