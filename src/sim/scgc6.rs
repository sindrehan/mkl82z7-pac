#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCGC6 {
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
#[doc = "Possible values of the field `NVM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVMR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl NVMR {
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
            NVMR::_0 => false,
            NVMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NVMR {
        match value {
            false => NVMR::_0,
            true => NVMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NVMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NVMR::_1
    }
}
#[doc = "Possible values of the field `DMACHMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMACHMUXR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl DMACHMUXR {
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
            DMACHMUXR::_0 => false,
            DMACHMUXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMACHMUXR {
        match value {
            false => DMACHMUXR::_0,
            true => DMACHMUXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMACHMUXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMACHMUXR::_1
    }
}
#[doc = "Possible values of the field `INTMUX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTMUX0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl INTMUX0R {
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
            INTMUX0R::_0 => false,
            INTMUX0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTMUX0R {
        match value {
            false => INTMUX0R::_0,
            true => INTMUX0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INTMUX0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INTMUX0R::_1
    }
}
#[doc = "Possible values of the field `TRNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNGR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl TRNGR {
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
            TRNGR::_0 => false,
            TRNGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRNGR {
        match value {
            false => TRNGR::_0,
            true => TRNGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRNGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRNGR::_1
    }
}
#[doc = "Possible values of the field `SPI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SPI0R {
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
            SPI0R::_0 => false,
            SPI0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI0R {
        match value {
            false => SPI0R::_0,
            true => SPI0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPI0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPI0R::_1
    }
}
#[doc = "Possible values of the field `SPI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SPI1R {
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
            SPI1R::_0 => false,
            SPI1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI1R {
        match value {
            false => SPI1R::_0,
            true => SPI1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPI1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPI1R::_1
    }
}
#[doc = "Possible values of the field `CRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CRCR {
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
            CRCR::_0 => false,
            CRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCR {
        match value {
            false => CRCR::_0,
            true => CRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRCR::_1
    }
}
#[doc = "Possible values of the field `PIT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PIT0R {
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
            PIT0R::_0 => false,
            PIT0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIT0R {
        match value {
            false => PIT0R::_0,
            true => PIT0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PIT0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PIT0R::_1
    }
}
#[doc = "Possible values of the field `TPM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl TPM0R {
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
            TPM0R::_0 => false,
            TPM0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM0R {
        match value {
            false => TPM0R::_0,
            true => TPM0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM0R::_1
    }
}
#[doc = "Possible values of the field `TPM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl TPM1R {
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
            TPM1R::_0 => false,
            TPM1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM1R {
        match value {
            false => TPM1R::_0,
            true => TPM1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM1R::_1
    }
}
#[doc = "Possible values of the field `TPM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl TPM2R {
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
            TPM2R::_0 => false,
            TPM2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM2R {
        match value {
            false => TPM2R::_0,
            true => TPM2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM2R::_1
    }
}
#[doc = "Possible values of the field `ADC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl ADC0R {
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
            ADC0R::_0 => false,
            ADC0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0R {
        match value {
            false => ADC0R::_0,
            true => ADC0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADC0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADC0R::_1
    }
}
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl RTCR {
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
            RTCR::_0 => false,
            RTCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCR {
        match value {
            false => RTCR::_0,
            true => RTCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTCR::_1
    }
}
#[doc = "Possible values of the field `RTC_RF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_RFR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl RTC_RFR {
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
            RTC_RFR::_0 => false,
            RTC_RFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_RFR {
        match value {
            false => RTC_RFR::_0,
            true => RTC_RFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTC_RFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTC_RFR::_1
    }
}
#[doc = "Possible values of the field `DAC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl DAC0R {
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
            DAC0R::_0 => false,
            DAC0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAC0R {
        match value {
            false => DAC0R::_0,
            true => DAC0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DAC0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DAC0R::_1
    }
}
#[doc = "Values that can be written to the field `NVM`"]
pub enum NVMW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl NVMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NVMW::_0 => false,
            NVMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NVMW<'a> {
    w: &'a mut W,
}
impl<'a> _NVMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NVMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NVMW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NVMW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMACHMUX`"]
pub enum DMACHMUXW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl DMACHMUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMACHMUXW::_0 => false,
            DMACHMUXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMACHMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMACHMUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMACHMUXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMACHMUXW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMACHMUXW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTMUX0`"]
pub enum INTMUX0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl INTMUX0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTMUX0W::_0 => false,
            INTMUX0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX0W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTMUX0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTMUX0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTMUX0W::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRNG`"]
pub enum TRNGW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl TRNGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRNGW::_0 => false,
            TRNGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRNGW<'a> {
    w: &'a mut W,
}
impl<'a> _TRNGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRNGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRNGW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRNGW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI0`"]
pub enum SPI0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SPI0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI0W::_0 => false,
            SPI0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI0W<'a> {
    w: &'a mut W,
}
impl<'a> _SPI0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0W::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI1`"]
pub enum SPI1W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SPI1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI1W::_0 => false,
            SPI1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI1W<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1W::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRC`"]
pub enum CRCW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCW::_0 => false,
            CRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIT0`"]
pub enum PIT0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl PIT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIT0W::_0 => false,
            PIT0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PIT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT0W::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM0`"]
pub enum TPM0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl TPM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM0W::_0 => false,
            TPM0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM0W<'a> {
    w: &'a mut W,
}
impl<'a> _TPM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM0W::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM1`"]
pub enum TPM1W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl TPM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM1W::_0 => false,
            TPM1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM1W<'a> {
    w: &'a mut W,
}
impl<'a> _TPM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1W::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM2`"]
pub enum TPM2W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl TPM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM2W::_0 => false,
            TPM2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM2W<'a> {
    w: &'a mut W,
}
impl<'a> _TPM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM2W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM2W::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC0`"]
pub enum ADC0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl ADC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0W::_0 => false,
            ADC0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0W::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC`"]
pub enum RTCW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCW::_0 => false,
            RTCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCW::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_RF`"]
pub enum RTC_RFW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl RTC_RFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_RFW::_0 => false,
            RTC_RFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_RFW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_RFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_RFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTC_RFW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTC_RFW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DAC0`"]
pub enum DAC0W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl DAC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAC0W::_0 => false,
            DAC0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAC0W<'a> {
    w: &'a mut W,
}
impl<'a> _DAC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC0W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC0W::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - NVM Clock Gate Control"]
    #[inline]
    pub fn nvm(&self) -> NVMR {
        NVMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DMACHMUX Clock Gate Control"]
    #[inline]
    pub fn dmachmux(&self) -> DMACHMUXR {
        DMACHMUXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - INTMUX0 Clock Gate Control"]
    #[inline]
    pub fn intmux0(&self) -> INTMUX0R {
        INTMUX0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - TRNG Clock Gate Control"]
    #[inline]
    pub fn trng(&self) -> TRNGR {
        TRNGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline]
    pub fn spi0(&self) -> SPI0R {
        SPI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SPI1 Clock Gate Control"]
    #[inline]
    pub fn spi1(&self) -> SPI1R {
        SPI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline]
    pub fn crc(&self) -> CRCR {
        CRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - PIT0 Clock Gate Control"]
    #[inline]
    pub fn pit0(&self) -> PIT0R {
        PIT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - TPM0 Clock Gate Control"]
    #[inline]
    pub fn tpm0(&self) -> TPM0R {
        TPM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - TPM1 Clock Gate Control"]
    #[inline]
    pub fn tpm1(&self) -> TPM1R {
        TPM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - TPM2 Clock Gate Control"]
    #[inline]
    pub fn tpm2(&self) -> TPM2R {
        TPM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline]
    pub fn adc0(&self) -> ADC0R {
        ADC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - RTC Clock Gate Control"]
    #[inline]
    pub fn rtc(&self) -> RTCR {
        RTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - RTC_RF Clock Gate Control"]
    #[inline]
    pub fn rtc_rf(&self) -> RTC_RFR {
        RTC_RFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline]
    pub fn dac0(&self) -> DAC0R {
        DAC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - NVM Clock Gate Control"]
    #[inline]
    pub fn nvm(&mut self) -> _NVMW {
        _NVMW { w: self }
    }
    #[doc = "Bit 1 - DMACHMUX Clock Gate Control"]
    #[inline]
    pub fn dmachmux(&mut self) -> _DMACHMUXW {
        _DMACHMUXW { w: self }
    }
    #[doc = "Bit 4 - INTMUX0 Clock Gate Control"]
    #[inline]
    pub fn intmux0(&mut self) -> _INTMUX0W {
        _INTMUX0W { w: self }
    }
    #[doc = "Bit 5 - TRNG Clock Gate Control"]
    #[inline]
    pub fn trng(&mut self) -> _TRNGW {
        _TRNGW { w: self }
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline]
    pub fn spi0(&mut self) -> _SPI0W {
        _SPI0W { w: self }
    }
    #[doc = "Bit 13 - SPI1 Clock Gate Control"]
    #[inline]
    pub fn spi1(&mut self) -> _SPI1W {
        _SPI1W { w: self }
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline]
    pub fn crc(&mut self) -> _CRCW {
        _CRCW { w: self }
    }
    #[doc = "Bit 23 - PIT0 Clock Gate Control"]
    #[inline]
    pub fn pit0(&mut self) -> _PIT0W {
        _PIT0W { w: self }
    }
    #[doc = "Bit 24 - TPM0 Clock Gate Control"]
    #[inline]
    pub fn tpm0(&mut self) -> _TPM0W {
        _TPM0W { w: self }
    }
    #[doc = "Bit 25 - TPM1 Clock Gate Control"]
    #[inline]
    pub fn tpm1(&mut self) -> _TPM1W {
        _TPM1W { w: self }
    }
    #[doc = "Bit 26 - TPM2 Clock Gate Control"]
    #[inline]
    pub fn tpm2(&mut self) -> _TPM2W {
        _TPM2W { w: self }
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline]
    pub fn adc0(&mut self) -> _ADC0W {
        _ADC0W { w: self }
    }
    #[doc = "Bit 29 - RTC Clock Gate Control"]
    #[inline]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
    #[doc = "Bit 30 - RTC_RF Clock Gate Control"]
    #[inline]
    pub fn rtc_rf(&mut self) -> _RTC_RFW {
        _RTC_RFW { w: self }
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline]
    pub fn dac0(&mut self) -> _DAC0W {
        _DAC0W { w: self }
    }
}
