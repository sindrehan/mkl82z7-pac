#[doc = "Reader of register SPNDST"]
pub type R = crate::R<u32, super::SPNDST>;
#[doc = "Reader of field `SUSPND`"]
pub type SUSPND_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPDBUF`"]
pub type SPDBUF_R = crate::R<u8, u8>;
#[doc = "Reader of field `DATLFT`"]
pub type DATLFT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - When set, it signifies that a sequence is in suspended state"]
    #[inline(always)]
    pub fn suspnd(&self) -> SUSPND_R {
        SUSPND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Suspended Buffer: Provides the suspended buffer number. Valid only when SUSPND is set to 1'b1"]
    #[inline(always)]
    pub fn spdbuf(&self) -> SPDBUF_R {
        SPDBUF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 9:14 - Data left: Provides information about the amount of data left to be read in the suspended sequence"]
    #[inline(always)]
    pub fn datlft(&self) -> DATLFT_R {
        DATLFT_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
