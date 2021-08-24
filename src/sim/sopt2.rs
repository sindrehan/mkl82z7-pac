#[doc = "Register `SOPT2` reader"]
pub struct R(crate::R<SOPT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT2` writer"]
pub struct W(crate::W<SOPT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SOPT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC clock out select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTS_A {
    #[doc = "0: RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0 = 0,
    #[doc = "1: RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1 = 1,
}
impl From<RTCCLKOUTS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCLKOUTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCLKOUTS` reader - RTC clock out select"]
pub struct RTCCLKOUTS_R(crate::FieldReader<bool, RTCCLKOUTS_A>);
impl RTCCLKOUTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCLKOUTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKOUTS_A {
        match self.bits {
            false => RTCCLKOUTS_A::_0,
            true => RTCCLKOUTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTCCLKOUTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTCCLKOUTS_A::_1
    }
}
impl core::ops::Deref for RTCCLKOUTS_R {
    type Target = crate::FieldReader<bool, RTCCLKOUTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCLKOUTS` writer - RTC clock out select"]
pub struct RTCCLKOUTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKOUTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCLKOUTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTS_A::_0)
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUT_A {
    #[doc = "2: Flash clock"]
    _010 = 2,
    #[doc = "3: LPO clock (1 kHz)"]
    _011 = 3,
    #[doc = "4: MCGIRCLK"]
    _100 = 4,
    #[doc = "5: RTC 32.768kHz clock"]
    _101 = 5,
    #[doc = "6: OSCERCLK0"]
    _110 = 6,
    #[doc = "7: IRC 48 MHz clock"]
    _111 = 7,
}
impl From<CLKOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUT` reader - CLKOUT select"]
pub struct CLKOUT_R(crate::FieldReader<u8, CLKOUT_A>);
impl CLKOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUT_A> {
        match self.bits {
            2 => Some(CLKOUT_A::_010),
            3 => Some(CLKOUT_A::_011),
            4 => Some(CLKOUT_A::_100),
            5 => Some(CLKOUT_A::_101),
            6 => Some(CLKOUT_A::_110),
            7 => Some(CLKOUT_A::_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == CLKOUT_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == CLKOUT_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == CLKOUT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == CLKOUT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == CLKOUT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == CLKOUT_A::_111
    }
}
impl core::ops::Deref for CLKOUT_R {
    type Target = crate::FieldReader<u8, CLKOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUT` writer - CLKOUT select"]
pub struct CLKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUT_A::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUT_A::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUT_A::_100)
    }
    #[doc = "RTC 32.768kHz clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUT_A::_101)
    }
    #[doc = "OSCERCLK0"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUT_A::_110)
    }
    #[doc = "IRC 48 MHz clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUT_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "PLL/FLL clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLFLLSEL_A {
    #[doc = "0: MCGFLLCLK clock"]
    _00 = 0,
    #[doc = "1: MCGPLLCLK clock"]
    _01 = 1,
    #[doc = "3: IRC48 MHz clock"]
    _11 = 3,
}
impl From<PLLFLLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLFLLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLFLLSEL` reader - PLL/FLL clock select"]
pub struct PLLFLLSEL_R(crate::FieldReader<u8, PLLFLLSEL_A>);
impl PLLFLLSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLFLLSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLFLLSEL_A> {
        match self.bits {
            0 => Some(PLLFLLSEL_A::_00),
            1 => Some(PLLFLLSEL_A::_01),
            3 => Some(PLLFLLSEL_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == PLLFLLSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == PLLFLLSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PLLFLLSEL_A::_11
    }
}
impl core::ops::Deref for PLLFLLSEL_R {
    type Target = crate::FieldReader<u8, PLLFLLSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLFLLSEL` writer - PLL/FLL clock select"]
pub struct PLLFLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLFLLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_00)
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_01)
    }
    #[doc = "IRC48 MHz clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "USB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRC_A {
    #[doc = "0: External bypass clock (USB_CLKIN)."]
    _0 = 0,
    #[doc = "1: MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    _1 = 1,
}
impl From<USBSRC_A> for bool {
    #[inline(always)]
    fn from(variant: USBSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSRC` reader - USB clock source select"]
pub struct USBSRC_R(crate::FieldReader<bool, USBSRC_A>);
impl USBSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSRC_A {
        match self.bits {
            false => USBSRC_A::_0,
            true => USBSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBSRC_A::_1
    }
}
impl core::ops::Deref for USBSRC_R {
    type Target = crate::FieldReader<bool, USBSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSRC` writer - USB clock source select"]
pub struct USBSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRC_A::_0)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "FlexIO Module Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXIOSRC_A {
    #[doc = "0: System clock"]
    _00 = 0,
    #[doc = "1: MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    _11 = 3,
}
impl From<FLEXIOSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXIOSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLEXIOSRC` reader - FlexIO Module Clock Source Select"]
pub struct FLEXIOSRC_R(crate::FieldReader<u8, FLEXIOSRC_A>);
impl FLEXIOSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLEXIOSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIOSRC_A {
        match self.bits {
            0 => FLEXIOSRC_A::_00,
            1 => FLEXIOSRC_A::_01,
            2 => FLEXIOSRC_A::_10,
            3 => FLEXIOSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FLEXIOSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FLEXIOSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FLEXIOSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FLEXIOSRC_A::_11
    }
}
impl core::ops::Deref for FLEXIOSRC_R {
    type Target = crate::FieldReader<u8, FLEXIOSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXIOSRC` writer - FlexIO Module Clock Source Select"]
pub struct FLEXIOSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIOSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIOSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "System clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FLEXIOSRC_A::_00)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FLEXIOSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FLEXIOSRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FLEXIOSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "TPM clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPMSRC_A {
    #[doc = "0: Clock disabled"]
    _00 = 0,
    #[doc = "1: MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    _11 = 3,
}
impl From<TPMSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TPMSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TPMSRC` reader - TPM clock source select"]
pub struct TPMSRC_R(crate::FieldReader<u8, TPMSRC_A>);
impl TPMSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPMSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPMSRC_A {
        match self.bits {
            0 => TPMSRC_A::_00,
            1 => TPMSRC_A::_01,
            2 => TPMSRC_A::_10,
            3 => TPMSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == TPMSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == TPMSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TPMSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TPMSRC_A::_11
    }
}
impl core::ops::Deref for TPMSRC_R {
    type Target = crate::FieldReader<u8, TPMSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPMSRC` writer - TPM clock source select"]
pub struct TPMSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPMSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPMSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPMSRC_A::_00)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPMSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPMSRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPMSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "LPUART clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUARTSRC_A {
    #[doc = "0: Clock disabled"]
    _00 = 0,
    #[doc = "1: MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\]
and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    _11 = 3,
}
impl From<LPUARTSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUARTSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPUARTSRC` reader - LPUART clock source select"]
pub struct LPUARTSRC_R(crate::FieldReader<u8, LPUARTSRC_A>);
impl LPUARTSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUARTSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUARTSRC_A {
        match self.bits {
            0 => LPUARTSRC_A::_00,
            1 => LPUARTSRC_A::_01,
            2 => LPUARTSRC_A::_10,
            3 => LPUARTSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LPUARTSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LPUARTSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == LPUARTSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == LPUARTSRC_A::_11
    }
}
impl core::ops::Deref for LPUARTSRC_R {
    type Target = crate::FieldReader<u8, LPUARTSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUARTSRC` writer - LPUART clock source select"]
pub struct LPUARTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUARTSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUARTSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_00)
    }
    #[doc = "MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\]
and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "EMVSIM Module Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMVSIMSRC_A {
    #[doc = "0: Clock disabled"]
    _00 = 0,
    #[doc = "1: MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\]
and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    _11 = 3,
}
impl From<EMVSIMSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: EMVSIMSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMVSIMSRC` reader - EMVSIM Module Clock Source Select"]
pub struct EMVSIMSRC_R(crate::FieldReader<u8, EMVSIMSRC_A>);
impl EMVSIMSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMVSIMSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMVSIMSRC_A {
        match self.bits {
            0 => EMVSIMSRC_A::_00,
            1 => EMVSIMSRC_A::_01,
            2 => EMVSIMSRC_A::_10,
            3 => EMVSIMSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == EMVSIMSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == EMVSIMSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == EMVSIMSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == EMVSIMSRC_A::_11
    }
}
impl core::ops::Deref for EMVSIMSRC_R {
    type Target = crate::FieldReader<u8, EMVSIMSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMVSIMSRC` writer - EMVSIM Module Clock Source Select"]
pub struct EMVSIMSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMVSIMSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMVSIMSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EMVSIMSRC_A::_00)
    }
    #[doc = "MCGFLLCLK ,MCGPLLCLK, or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the PLLFLLCLK fractional divider as configured by SIM_CLKDIV3\\[PLLFLLFRAC\\]
and SIM_CLKDIV3\\[PLLFLLDIV\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EMVSIMSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EMVSIMSRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EMVSIMSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkouts(&self) -> RTCCLKOUTS_R {
        RTCCLKOUTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkout(&self) -> CLKOUT_R {
        CLKOUT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&self) -> PLLFLLSEL_R {
        PLLFLLSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&self) -> USBSRC_R {
        USBSRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - FlexIO Module Clock Source Select"]
    #[inline(always)]
    pub fn flexiosrc(&self) -> FLEXIOSRC_R {
        FLEXIOSRC_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline(always)]
    pub fn tpmsrc(&self) -> TPMSRC_R {
        TPMSRC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - LPUART clock source select"]
    #[inline(always)]
    pub fn lpuartsrc(&self) -> LPUARTSRC_R {
        LPUARTSRC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - EMVSIM Module Clock Source Select"]
    #[inline(always)]
    pub fn emvsimsrc(&self) -> EMVSIMSRC_R {
        EMVSIMSRC_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkouts(&mut self) -> RTCCLKOUTS_W {
        RTCCLKOUTS_W { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkout(&mut self) -> CLKOUT_W {
        CLKOUT_W { w: self }
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&mut self) -> PLLFLLSEL_W {
        PLLFLLSEL_W { w: self }
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&mut self) -> USBSRC_W {
        USBSRC_W { w: self }
    }
    #[doc = "Bits 22:23 - FlexIO Module Clock Source Select"]
    #[inline(always)]
    pub fn flexiosrc(&mut self) -> FLEXIOSRC_W {
        FLEXIOSRC_W { w: self }
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline(always)]
    pub fn tpmsrc(&mut self) -> TPMSRC_W {
        TPMSRC_W { w: self }
    }
    #[doc = "Bits 26:27 - LPUART clock source select"]
    #[inline(always)]
    pub fn lpuartsrc(&mut self) -> LPUARTSRC_W {
        LPUARTSRC_W { w: self }
    }
    #[doc = "Bits 30:31 - EMVSIM Module Clock Source Select"]
    #[inline(always)]
    pub fn emvsimsrc(&mut self) -> EMVSIMSRC_W {
        EMVSIMSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt2](index.html) module"]
pub struct SOPT2_SPEC;
impl crate::RegisterSpec for SOPT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt2::R](R) reader structure"]
impl crate::Readable for SOPT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt2::W](W) writer structure"]
impl crate::Writable for SOPT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOPT2 to value 0"]
impl crate::Resettable for SOPT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
