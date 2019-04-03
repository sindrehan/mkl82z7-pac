#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RBCT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct WMRKR {
    bits: u8,
}
impl WMRKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXBRD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBRDR {
    #[doc = "RX Buffer content is read using the AHB Bus registers QSPI_ARDB0 to QSPI_ARDB15. For details, refer to Exclusive Access to Serial Flash for AHB Commands."]
    _0,
    #[doc = "RX Buffer content is read using the IP Bus registers QSPI_RBDR0 to QSPI_RBDR15."]
    _1,
}
impl RXBRDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RXBRDR::_0 => false,
            RXBRDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXBRDR {
        match value {
            false => RXBRDR::_0,
            true => RXBRDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXBRDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXBRDR::_1
    }
}
#[doc = r" Proxy"]
pub struct _WMRKW<'a> {
    w: &'a mut W,
}
impl<'a> _WMRKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXBRD`"]
pub enum RXBRDW {
    #[doc = "RX Buffer content is read using the AHB Bus registers QSPI_ARDB0 to QSPI_ARDB15. For details, refer to Exclusive Access to Serial Flash for AHB Commands."]
    _0,
    #[doc = "RX Buffer content is read using the IP Bus registers QSPI_RBDR0 to QSPI_RBDR15."]
    _1,
}
impl RXBRDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXBRDW::_0 => false,
            RXBRDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXBRDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBRDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXBRDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX Buffer content is read using the AHB Bus registers QSPI_ARDB0 to QSPI_ARDB15. For details, refer to Exclusive Access to Serial Flash for AHB Commands."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBRDW::_0)
    }
    #[doc = "RX Buffer content is read using the IP Bus registers QSPI_RBDR0 to QSPI_RBDR15."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBRDW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - RX Buffer Watermark"]
    #[inline]
    pub fn wmrk(&self) -> WMRKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WMRKR { bits }
    }
    #[doc = "Bit 8 - RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout."]
    #[inline]
    pub fn rxbrd(&self) -> RXBRDR {
        RXBRDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - RX Buffer Watermark"]
    #[inline]
    pub fn wmrk(&mut self) -> _WMRKW {
        _WMRKW { w: self }
    }
    #[doc = "Bit 8 - RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout."]
    #[inline]
    pub fn rxbrd(&mut self) -> _RXBRDW {
        _RXBRDW { w: self }
    }
}
