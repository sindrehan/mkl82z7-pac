#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Resets all RTC registers except for the SWR bit and the RTC_WAR and RTC_RAR registers . The SWR bit is cleared by VBAT POR and by software explicitly clearing it."]
    _1 = 1,
}
impl From<SWR_A> for bool {
    #[inline(always)]
    fn from(variant: SWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWR` reader - Software Reset"]
pub struct SWR_R(crate::FieldReader<bool, SWR_A>);
impl SWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWR_A {
        match self.bits {
            false => SWR_A::_0,
            true => SWR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWR_A::_1
    }
}
impl core::ops::Deref for SWR_R {
    type Target = crate::FieldReader<bool, SWR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWR` writer - Software Reset"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWR_A::_0)
    }
    #[doc = "Resets all RTC registers except for the SWR bit and the RTC_WAR and RTC_RAR registers . The SWR bit is cleared by VBAT POR and by software explicitly clearing it."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWR_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Wakeup Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPE_A {
    #[doc = "0: Wakeup pin is disabled."]
    _0 = 0,
    #[doc = "1: Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    _1 = 1,
}
impl From<WPE_A> for bool {
    #[inline(always)]
    fn from(variant: WPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPE` reader - Wakeup Pin Enable"]
pub struct WPE_R(crate::FieldReader<bool, WPE_A>);
impl WPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPE_A {
        match self.bits {
            false => WPE_A::_0,
            true => WPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WPE_A::_1
    }
}
impl core::ops::Deref for WPE_R {
    type Target = crate::FieldReader<bool, WPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPE` writer - Wakeup Pin Enable"]
pub struct WPE_W<'a> {
    w: &'a mut W,
}
impl<'a> WPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wakeup pin is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPE_A::_0)
    }
    #[doc = "Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Supervisor Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUP_A {
    #[doc = "0: Non-supervisor mode write accesses are not supported and generate a bus error."]
    _0 = 0,
    #[doc = "1: Non-supervisor mode write accesses are supported."]
    _1 = 1,
}
impl From<SUP_A> for bool {
    #[inline(always)]
    fn from(variant: SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUP` reader - Supervisor Access"]
pub struct SUP_R(crate::FieldReader<bool, SUP_A>);
impl SUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUP_A {
        match self.bits {
            false => SUP_A::_0,
            true => SUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SUP_A::_1
    }
}
impl core::ops::Deref for SUP_R {
    type Target = crate::FieldReader<bool, SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUP` writer - Supervisor Access"]
pub struct SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUP_A::_0)
    }
    #[doc = "Non-supervisor mode write accesses are supported."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UM_A {
    #[doc = "0: Registers cannot be written when locked."]
    _0 = 0,
    #[doc = "1: Registers can be written when locked under limited conditions."]
    _1 = 1,
}
impl From<UM_A> for bool {
    #[inline(always)]
    fn from(variant: UM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UM` reader - Update Mode"]
pub struct UM_R(crate::FieldReader<bool, UM_A>);
impl UM_R {
    pub(crate) fn new(bits: bool) -> Self {
        UM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UM_A {
        match self.bits {
            false => UM_A::_0,
            true => UM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UM_A::_1
    }
}
impl core::ops::Deref for UM_R {
    type Target = crate::FieldReader<bool, UM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UM` writer - Update Mode"]
pub struct UM_W<'a> {
    w: &'a mut W,
}
impl<'a> UM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Registers cannot be written when locked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UM_A::_0)
    }
    #[doc = "Registers can be written when locked under limited conditions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Wakeup Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPS_A {
    #[doc = "0: Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    _0 = 0,
    #[doc = "1: Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    _1 = 1,
}
impl From<WPS_A> for bool {
    #[inline(always)]
    fn from(variant: WPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPS` reader - Wakeup Pin Select"]
pub struct WPS_R(crate::FieldReader<bool, WPS_A>);
impl WPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPS_A {
        match self.bits {
            false => WPS_A::_0,
            true => WPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WPS_A::_1
    }
}
impl core::ops::Deref for WPS_R {
    type Target = crate::FieldReader<bool, WPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPS` writer - Wakeup Pin Select"]
pub struct WPS_W<'a> {
    w: &'a mut W,
}
impl<'a> WPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPS_A::_0)
    }
    #[doc = "Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPS_A::_1)
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
#[doc = "Oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCE_A {
    #[doc = "0: 32.768 kHz oscillator is disabled."]
    _0 = 0,
    #[doc = "1: 32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    _1 = 1,
}
impl From<OSCE_A> for bool {
    #[inline(always)]
    fn from(variant: OSCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCE` reader - Oscillator Enable"]
pub struct OSCE_R(crate::FieldReader<bool, OSCE_A>);
impl OSCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCE_A {
        match self.bits {
            false => OSCE_A::_0,
            true => OSCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OSCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OSCE_A::_1
    }
}
impl core::ops::Deref for OSCE_R {
    type Target = crate::FieldReader<bool, OSCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCE` writer - Oscillator Enable"]
pub struct OSCE_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "32.768 kHz oscillator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSCE_A::_0)
    }
    #[doc = "32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO_A {
    #[doc = "0: The 32 kHz clock is output to other peripherals."]
    _0 = 0,
    #[doc = "1: The 32 kHz clock is not output to other peripherals."]
    _1 = 1,
}
impl From<CLKO_A> for bool {
    #[inline(always)]
    fn from(variant: CLKO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKO` reader - Clock Output"]
pub struct CLKO_R(crate::FieldReader<bool, CLKO_A>);
impl CLKO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO_A {
        match self.bits {
            false => CLKO_A::_0,
            true => CLKO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKO_A::_1
    }
}
impl core::ops::Deref for CLKO_R {
    type Target = crate::FieldReader<bool, CLKO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKO` writer - Clock Output"]
pub struct CLKO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The 32 kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKO_A::_0)
    }
    #[doc = "The 32 kHz clock is not output to other peripherals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Oscillator 16pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC16P_A {
    #[doc = "0: Disable the load."]
    _0 = 0,
    #[doc = "1: Enable the additional load."]
    _1 = 1,
}
impl From<SC16P_A> for bool {
    #[inline(always)]
    fn from(variant: SC16P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC16P` reader - Oscillator 16pF Load Configure"]
pub struct SC16P_R(crate::FieldReader<bool, SC16P_A>);
impl SC16P_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC16P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC16P_A {
        match self.bits {
            false => SC16P_A::_0,
            true => SC16P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SC16P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SC16P_A::_1
    }
}
impl core::ops::Deref for SC16P_R {
    type Target = crate::FieldReader<bool, SC16P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC16P` writer - Oscillator 16pF Load Configure"]
pub struct SC16P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC16P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC16P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC16P_A::_0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC16P_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Oscillator 8pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC8P_A {
    #[doc = "0: Disable the load."]
    _0 = 0,
    #[doc = "1: Enable the additional load."]
    _1 = 1,
}
impl From<SC8P_A> for bool {
    #[inline(always)]
    fn from(variant: SC8P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC8P` reader - Oscillator 8pF Load Configure"]
pub struct SC8P_R(crate::FieldReader<bool, SC8P_A>);
impl SC8P_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC8P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC8P_A {
        match self.bits {
            false => SC8P_A::_0,
            true => SC8P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SC8P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SC8P_A::_1
    }
}
impl core::ops::Deref for SC8P_R {
    type Target = crate::FieldReader<bool, SC8P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC8P` writer - Oscillator 8pF Load Configure"]
pub struct SC8P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC8P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC8P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC8P_A::_0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC8P_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Oscillator 4pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC4P_A {
    #[doc = "0: Disable the load."]
    _0 = 0,
    #[doc = "1: Enable the additional load."]
    _1 = 1,
}
impl From<SC4P_A> for bool {
    #[inline(always)]
    fn from(variant: SC4P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC4P` reader - Oscillator 4pF Load Configure"]
pub struct SC4P_R(crate::FieldReader<bool, SC4P_A>);
impl SC4P_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC4P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC4P_A {
        match self.bits {
            false => SC4P_A::_0,
            true => SC4P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SC4P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SC4P_A::_1
    }
}
impl core::ops::Deref for SC4P_R {
    type Target = crate::FieldReader<bool, SC4P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC4P` writer - Oscillator 4pF Load Configure"]
pub struct SC4P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC4P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC4P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC4P_A::_0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC4P_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Oscillator 2pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC2P_A {
    #[doc = "0: Disable the load."]
    _0 = 0,
    #[doc = "1: Enable the additional load."]
    _1 = 1,
}
impl From<SC2P_A> for bool {
    #[inline(always)]
    fn from(variant: SC2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC2P` reader - Oscillator 2pF Load Configure"]
pub struct SC2P_R(crate::FieldReader<bool, SC2P_A>);
impl SC2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC2P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC2P_A {
        match self.bits {
            false => SC2P_A::_0,
            true => SC2P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SC2P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SC2P_A::_1
    }
}
impl core::ops::Deref for SC2P_R {
    type Target = crate::FieldReader<bool, SC2P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC2P` writer - Oscillator 2pF Load Configure"]
pub struct SC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC2P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC2P_A::_0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC2P_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Pin Enable"]
    #[inline(always)]
    pub fn wpe(&self) -> WPE_R {
        WPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&self) -> SUP_R {
        SUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&self) -> UM_R {
        UM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Pin Select"]
    #[inline(always)]
    pub fn wps(&self) -> WPS_R {
        WPS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Oscillator Enable"]
    #[inline(always)]
    pub fn osce(&self) -> OSCE_R {
        OSCE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
    #[inline(always)]
    pub fn sc16p(&self) -> SC16P_R {
        SC16P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
    #[inline(always)]
    pub fn sc8p(&self) -> SC8P_R {
        SC8P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
    #[inline(always)]
    pub fn sc4p(&self) -> SC4P_R {
        SC4P_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
    #[inline(always)]
    pub fn sc2p(&self) -> SC2P_R {
        SC2P_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Pin Enable"]
    #[inline(always)]
    pub fn wpe(&mut self) -> WPE_W {
        WPE_W { w: self }
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&mut self) -> SUP_W {
        SUP_W { w: self }
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&mut self) -> UM_W {
        UM_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Pin Select"]
    #[inline(always)]
    pub fn wps(&mut self) -> WPS_W {
        WPS_W { w: self }
    }
    #[doc = "Bit 8 - Oscillator Enable"]
    #[inline(always)]
    pub fn osce(&mut self) -> OSCE_W {
        OSCE_W { w: self }
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline(always)]
    pub fn clko(&mut self) -> CLKO_W {
        CLKO_W { w: self }
    }
    #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
    #[inline(always)]
    pub fn sc16p(&mut self) -> SC16P_W {
        SC16P_W { w: self }
    }
    #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
    #[inline(always)]
    pub fn sc8p(&mut self) -> SC8P_W {
        SC8P_W { w: self }
    }
    #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
    #[inline(always)]
    pub fn sc4p(&mut self) -> SC4P_W {
        SC4P_W { w: self }
    }
    #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
    #[inline(always)]
    pub fn sc2p(&mut self) -> SC2P_W {
        SC2P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
