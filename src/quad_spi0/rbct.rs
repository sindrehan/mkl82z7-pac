#[doc = "Reader of register RBCT"]
pub type R = crate::R<u32, super::RBCT>;
#[doc = "Writer for register RBCT"]
pub type W = crate::W<u32, super::RBCT>;
#[doc = "Register RBCT `reset()`'s with value 0"]
impl crate::ResetValue for super::RBCT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WMRK`"]
pub type WMRK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WMRK`"]
pub struct WMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> WMRK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBRD_A {
    #[doc = "0: RX Buffer content is read using the AHB Bus registers QSPI_ARDB0 to QSPI_ARDB15. For details, refer to Exclusive Access to Serial Flash for AHB Commands."]
    _0 = 0,
    #[doc = "1: RX Buffer content is read using the IP Bus registers QSPI_RBDR0 to QSPI_RBDR15."]
    _1 = 1,
}
impl From<RXBRD_A> for bool {
    #[inline(always)]
    fn from(variant: RXBRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXBRD`"]
pub type RXBRD_R = crate::R<bool, RXBRD_A>;
impl RXBRD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBRD_A {
        match self.bits {
            false => RXBRD_A::_0,
            true => RXBRD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXBRD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXBRD_A::_1
    }
}
#[doc = "Write proxy for field `RXBRD`"]
pub struct RXBRD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXBRD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RX Buffer content is read using the AHB Bus registers QSPI_ARDB0 to QSPI_ARDB15. For details, refer to Exclusive Access to Serial Flash for AHB Commands."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBRD_A::_0)
    }
    #[doc = "RX Buffer content is read using the IP Bus registers QSPI_RBDR0 to QSPI_RBDR15."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBRD_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RX Buffer Watermark"]
    #[inline(always)]
    pub fn wmrk(&self) -> WMRK_R {
        WMRK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout."]
    #[inline(always)]
    pub fn rxbrd(&self) -> RXBRD_R {
        RXBRD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - RX Buffer Watermark"]
    #[inline(always)]
    pub fn wmrk(&mut self) -> WMRK_W {
        WMRK_W { w: self }
    }
    #[doc = "Bit 8 - RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout."]
    #[inline(always)]
    pub fn rxbrd(&mut self) -> RXBRD_W {
        RXBRD_W { w: self }
    }
}
