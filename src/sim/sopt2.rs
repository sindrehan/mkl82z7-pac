#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT2 {
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
#[doc = "Possible values of the field `RTCCLKOUTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTSR {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl RTCCLKOUTSR {
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
            RTCCLKOUTSR::_0 => false,
            RTCCLKOUTSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCCLKOUTSR {
        match value {
            false => RTCCLKOUTSR::_0,
            true => RTCCLKOUTSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKOUTSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKOUTSR::_1
    }
}
#[doc = "Possible values of the field `CLKOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTR {
    #[doc = "Flash clock"]
    _010,
    #[doc = "LPO clock (1 kHz)"]
    _011,
    #[doc = "MCGIRCLK"]
    _100,
    #[doc = "RTC 32.768kHz clock"]
    _101,
    #[doc = "OSCERCLK0"]
    _110,
    #[doc = "IRC 48 MHz clock"]
    _111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTR::_010 => 2,
            CLKOUTR::_011 => 3,
            CLKOUTR::_100 => 4,
            CLKOUTR::_101 => 5,
            CLKOUTR::_110 => 6,
            CLKOUTR::_111 => 7,
            CLKOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTR {
        match value {
            2 => CLKOUTR::_010,
            3 => CLKOUTR::_011,
            4 => CLKOUTR::_100,
            5 => CLKOUTR::_101,
            6 => CLKOUTR::_110,
            7 => CLKOUTR::_111,
            i => CLKOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTR::_111
    }
}
#[doc = "Possible values of the field `PLLFLLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLFLLSELR {
    #[doc = "MCGFLLCLK clock"]
    _00,
    #[doc = "MCGPLLCLK clock"]
    _01,
    #[doc = "IRC48 MHz clock"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLLFLLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLFLLSELR::_00 => 0,
            PLLFLLSELR::_01 => 1,
            PLLFLLSELR::_11 => 3,
            PLLFLLSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLFLLSELR {
        match value {
            0 => PLLFLLSELR::_00,
            1 => PLLFLLSELR::_01,
            3 => PLLFLLSELR::_11,
            i => PLLFLLSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PLLFLLSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PLLFLLSELR::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PLLFLLSELR::_11
    }
}
#[doc = "Possible values of the field `USBSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRCR {
    #[doc = "External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    _1,
}
impl USBSRCR {
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
            USBSRCR::_0 => false,
            USBSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBSRCR {
        match value {
            false => USBSRCR::_0,
            true => USBSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBSRCR::_1
    }
}
#[doc = "Possible values of the field `FLEXIOSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIOSRCR {
    #[doc = "System clock"]
    _00,
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl FLEXIOSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLEXIOSRCR::_00 => 0,
            FLEXIOSRCR::_01 => 1,
            FLEXIOSRCR::_10 => 2,
            FLEXIOSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLEXIOSRCR {
        match value {
            0 => FLEXIOSRCR::_00,
            1 => FLEXIOSRCR::_01,
            2 => FLEXIOSRCR::_10,
            3 => FLEXIOSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FLEXIOSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FLEXIOSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FLEXIOSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FLEXIOSRCR::_11
    }
}
#[doc = "Possible values of the field `TPMSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPMSRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl TPMSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPMSRCR::_00 => 0,
            TPMSRCR::_01 => 1,
            TPMSRCR::_10 => 2,
            TPMSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPMSRCR {
        match value {
            0 => TPMSRCR::_00,
            1 => TPMSRCR::_01,
            2 => TPMSRCR::_10,
            3 => TPMSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TPMSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TPMSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TPMSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TPMSRCR::_11
    }
}
#[doc = "Possible values of the field `LPUARTSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUARTSRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\] and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl LPUARTSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPUARTSRCR::_00 => 0,
            LPUARTSRCR::_01 => 1,
            LPUARTSRCR::_10 => 2,
            LPUARTSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPUARTSRCR {
        match value {
            0 => LPUARTSRCR::_00,
            1 => LPUARTSRCR::_01,
            2 => LPUARTSRCR::_10,
            3 => LPUARTSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPUARTSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPUARTSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LPUARTSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LPUARTSRCR::_11
    }
}
#[doc = "Possible values of the field `EMVSIMSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMVSIMSRCR {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\] and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl EMVSIMSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMVSIMSRCR::_00 => 0,
            EMVSIMSRCR::_01 => 1,
            EMVSIMSRCR::_10 => 2,
            EMVSIMSRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMVSIMSRCR {
        match value {
            0 => EMVSIMSRCR::_00,
            1 => EMVSIMSRCR::_01,
            2 => EMVSIMSRCR::_10,
            3 => EMVSIMSRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == EMVSIMSRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == EMVSIMSRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == EMVSIMSRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == EMVSIMSRCR::_11
    }
}
#[doc = "Values that can be written to the field `RTCCLKOUTS`"]
pub enum RTCCLKOUTSW {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0,
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1,
}
impl RTCCLKOUTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCCLKOUTSW::_0 => false,
            RTCCLKOUTSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCCLKOUTSW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCCLKOUTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCCLKOUTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSW::_0)
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSW::_1)
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
#[doc = "Values that can be written to the field `CLKOUT`"]
pub enum CLKOUTW {
    #[doc = "Flash clock"]
    _010,
    #[doc = "LPO clock (1 kHz)"]
    _011,
    #[doc = "MCGIRCLK"]
    _100,
    #[doc = "RTC 32.768kHz clock"]
    _101,
    #[doc = "OSCERCLK0"]
    _110,
    #[doc = "IRC 48 MHz clock"]
    _111,
}
impl CLKOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTW::_010 => 2,
            CLKOUTW::_011 => 3,
            CLKOUTW::_100 => 4,
            CLKOUTW::_101 => 5,
            CLKOUTW::_110 => 6,
            CLKOUTW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash clock"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTW::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTW::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTW::_100)
    }
    #[doc = "RTC 32.768kHz clock"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTW::_101)
    }
    #[doc = "OSCERCLK0"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTW::_110)
    }
    #[doc = "IRC 48 MHz clock"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLFLLSEL`"]
pub enum PLLFLLSELW {
    #[doc = "MCGFLLCLK clock"]
    _00,
    #[doc = "MCGPLLCLK clock"]
    _01,
    #[doc = "IRC48 MHz clock"]
    _11,
}
impl PLLFLLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLFLLSELW::_00 => 0,
            PLLFLLSELW::_01 => 1,
            PLLFLLSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLFLLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLFLLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLFLLSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_00)
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_01)
    }
    #[doc = "IRC48 MHz clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLLFLLSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBSRC`"]
pub enum USBSRCW {
    #[doc = "External bypass clock (USB_CLKIN)."]
    _0,
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    _1,
}
impl USBSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBSRCW::_0 => false,
            USBSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRCW::_0)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRCW::_1)
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
#[doc = "Values that can be written to the field `FLEXIOSRC`"]
pub enum FLEXIOSRCW {
    #[doc = "System clock"]
    _00,
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl FLEXIOSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLEXIOSRCW::_00 => 0,
            FLEXIOSRCW::_01 => 1,
            FLEXIOSRCW::_10 => 2,
            FLEXIOSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXIOSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXIOSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXIOSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "System clock"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLEXIOSRCW::_00)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLEXIOSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLEXIOSRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLEXIOSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPMSRC`"]
pub enum TPMSRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl TPMSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPMSRCW::_00 => 0,
            TPMSRCW::_01 => 1,
            TPMSRCW::_10 => 2,
            TPMSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPMSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPMSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPMSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPMSRCW::_00)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPMSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPMSRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPMSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPUARTSRC`"]
pub enum LPUARTSRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\] and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl LPUARTSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPUARTSRCW::_00 => 0,
            LPUARTSRCW::_01 => 1,
            LPUARTSRCW::_10 => 2,
            LPUARTSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPUARTSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUARTSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPUARTSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUARTSRCW::_00)
    }
    #[doc = "MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\] and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUARTSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUARTSRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LPUARTSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMVSIMSRC`"]
pub enum EMVSIMSRCW {
    #[doc = "Clock disabled"]
    _00,
    #[doc = "MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\] and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    _01,
    #[doc = "OSCERCLK clock"]
    _10,
    #[doc = "MCGIRCLK clock"]
    _11,
}
impl EMVSIMSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMVSIMSRCW::_00 => 0,
            EMVSIMSRCW::_01 => 1,
            EMVSIMSRCW::_10 => 2,
            EMVSIMSRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMVSIMSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMVSIMSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMVSIMSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMVSIMSRCW::_00)
    }
    #[doc = "MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\] and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(EMVSIMSRCW::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(EMVSIMSRCW::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(EMVSIMSRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline]
    pub fn rtcclkouts(&self) -> RTCCLKOUTSR {
        RTCCLKOUTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkout(&self) -> CLKOUTR {
        CLKOUTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline]
    pub fn pllfllsel(&self) -> PLLFLLSELR {
        PLLFLLSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline]
    pub fn usbsrc(&self) -> USBSRCR {
        USBSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - FlexIO Module Clock Source Select"]
    #[inline]
    pub fn flexiosrc(&self) -> FLEXIOSRCR {
        FLEXIOSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline]
    pub fn tpmsrc(&self) -> TPMSRCR {
        TPMSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - LPUART clock source select"]
    #[inline]
    pub fn lpuartsrc(&self) -> LPUARTSRCR {
        LPUARTSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - EMVSIM Module Clock Source Select"]
    #[inline]
    pub fn emvsimsrc(&self) -> EMVSIMSRCR {
        EMVSIMSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline]
    pub fn rtcclkouts(&mut self) -> _RTCCLKOUTSW {
        _RTCCLKOUTSW { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline]
    pub fn clkout(&mut self) -> _CLKOUTW {
        _CLKOUTW { w: self }
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline]
    pub fn pllfllsel(&mut self) -> _PLLFLLSELW {
        _PLLFLLSELW { w: self }
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline]
    pub fn usbsrc(&mut self) -> _USBSRCW {
        _USBSRCW { w: self }
    }
    #[doc = "Bits 22:23 - FlexIO Module Clock Source Select"]
    #[inline]
    pub fn flexiosrc(&mut self) -> _FLEXIOSRCW {
        _FLEXIOSRCW { w: self }
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline]
    pub fn tpmsrc(&mut self) -> _TPMSRCW {
        _TPMSRCW { w: self }
    }
    #[doc = "Bits 26:27 - LPUART clock source select"]
    #[inline]
    pub fn lpuartsrc(&mut self) -> _LPUARTSRCW {
        _LPUARTSRCW { w: self }
    }
    #[doc = "Bits 30:31 - EMVSIM Module Clock Source Select"]
    #[inline]
    pub fn emvsimsrc(&mut self) -> _EMVSIMSRCW {
        _EMVSIMSRCW { w: self }
    }
}
