#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEEN_A {
    #[doc = "0: Internal TPM counter continues in Doze mode."]
    _0 = 0,
    #[doc = "1: Internal TPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    _1 = 1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEEN` reader - Doze Enable"]
pub struct DOZEEN_R(crate::FieldReader<bool, DOZEEN_A>);
impl DOZEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOZEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::_0,
            true => DOZEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOZEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOZEEN_A::_1
    }
}
impl core::ops::Deref for DOZEEN_R {
    type Target = crate::FieldReader<bool, DOZEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZEEN` writer - Doze Enable"]
pub struct DOZEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal TPM counter continues in Doze mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZEEN_A::_0)
    }
    #[doc = "Internal TPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Debug Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBGMODE_A {
    #[doc = "0: TPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    _00 = 0,
    #[doc = "3: TPM counter continues in debug mode."]
    _11 = 3,
}
impl From<DBGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DBGMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DBGMODE` reader - Debug Mode"]
pub struct DBGMODE_R(crate::FieldReader<u8, DBGMODE_A>);
impl DBGMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBGMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBGMODE_A> {
        match self.bits {
            0 => Some(DBGMODE_A::_00),
            3 => Some(DBGMODE_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == DBGMODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == DBGMODE_A::_11
    }
}
impl core::ops::Deref for DBGMODE_R {
    type Target = crate::FieldReader<u8, DBGMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGMODE` writer - Debug Mode"]
pub struct DBGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DBGMODE_A::_00)
    }
    #[doc = "TPM counter continues in debug mode."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DBGMODE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Global Time Base Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBSYNC_A {
    #[doc = "0: Global timebase synchronization disabled."]
    _0 = 0,
    #[doc = "1: Global timebase synchronization enabled."]
    _1 = 1,
}
impl From<GTBSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: GTBSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTBSYNC` reader - Global Time Base Synchronization"]
pub struct GTBSYNC_R(crate::FieldReader<bool, GTBSYNC_A>);
impl GTBSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTBSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBSYNC_A {
        match self.bits {
            false => GTBSYNC_A::_0,
            true => GTBSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GTBSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GTBSYNC_A::_1
    }
}
impl core::ops::Deref for GTBSYNC_R {
    type Target = crate::FieldReader<bool, GTBSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTBSYNC` writer - Global Time Base Synchronization"]
pub struct GTBSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Global timebase synchronization disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBSYNC_A::_0)
    }
    #[doc = "Global timebase synchronization enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBSYNC_A::_1)
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
#[doc = "Global time base enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEEN_A {
    #[doc = "0: All channels use the internally generated TPM counter as their timebase"]
    _0 = 0,
    #[doc = "1: All channels use an externally generated global timebase as their timebase"]
    _1 = 1,
}
impl From<GTBEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTBEEN` reader - Global time base enable"]
pub struct GTBEEN_R(crate::FieldReader<bool, GTBEEN_A>);
impl GTBEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTBEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEEN_A {
        match self.bits {
            false => GTBEEN_A::_0,
            true => GTBEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GTBEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GTBEEN_A::_1
    }
}
impl core::ops::Deref for GTBEEN_R {
    type Target = crate::FieldReader<bool, GTBEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTBEEN` writer - Global time base enable"]
pub struct GTBEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTBEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All channels use the internally generated TPM counter as their timebase"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEEN_A::_0)
    }
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEEN_A::_1)
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
#[doc = "Counter Start on Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOT_A {
    #[doc = "0: TPM counter starts to increment immediately, once it is enabled."]
    _0 = 0,
    #[doc = "1: TPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    _1 = 1,
}
impl From<CSOT_A> for bool {
    #[inline(always)]
    fn from(variant: CSOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSOT` reader - Counter Start on Trigger"]
pub struct CSOT_R(crate::FieldReader<bool, CSOT_A>);
impl CSOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSOT_A {
        match self.bits {
            false => CSOT_A::_0,
            true => CSOT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CSOT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CSOT_A::_1
    }
}
impl core::ops::Deref for CSOT_R {
    type Target = crate::FieldReader<bool, CSOT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOT` writer - Counter Start on Trigger"]
pub struct CSOT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TPM counter starts to increment immediately, once it is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSOT_A::_0)
    }
    #[doc = "TPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSOT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Counter Stop On Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSOO_A {
    #[doc = "0: TPM counter continues incrementing or decrementing after overflow"]
    _0 = 0,
    #[doc = "1: TPM counter stops incrementing or decrementing after overflow."]
    _1 = 1,
}
impl From<CSOO_A> for bool {
    #[inline(always)]
    fn from(variant: CSOO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSOO` reader - Counter Stop On Overflow"]
pub struct CSOO_R(crate::FieldReader<bool, CSOO_A>);
impl CSOO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSOO_A {
        match self.bits {
            false => CSOO_A::_0,
            true => CSOO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CSOO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CSOO_A::_1
    }
}
impl core::ops::Deref for CSOO_R {
    type Target = crate::FieldReader<bool, CSOO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOO` writer - Counter Stop On Overflow"]
pub struct CSOO_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSOO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TPM counter continues incrementing or decrementing after overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSOO_A::_0)
    }
    #[doc = "TPM counter stops incrementing or decrementing after overflow."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSOO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Counter Reload On Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROT_A {
    #[doc = "0: Counter is not reloaded due to a rising edge on the selected input trigger"]
    _0 = 0,
    #[doc = "1: Counter is reloaded when a rising edge is detected on the selected input trigger"]
    _1 = 1,
}
impl From<CROT_A> for bool {
    #[inline(always)]
    fn from(variant: CROT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROT` reader - Counter Reload On Trigger"]
pub struct CROT_R(crate::FieldReader<bool, CROT_A>);
impl CROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CROT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CROT_A {
        match self.bits {
            false => CROT_A::_0,
            true => CROT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CROT_A::_1
    }
}
impl core::ops::Deref for CROT_R {
    type Target = crate::FieldReader<bool, CROT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CROT` writer - Counter Reload On Trigger"]
pub struct CROT_W<'a> {
    w: &'a mut W,
}
impl<'a> CROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CROT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CROT_A::_0)
    }
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CROT_A::_1)
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
#[doc = "Field `CPOT` reader - Counter Pause On Trigger"]
pub struct CPOT_R(crate::FieldReader<bool, bool>);
impl CPOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOT` writer - Counter Pause On Trigger"]
pub struct CPOT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Trigger Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPOL_A {
    #[doc = "0: Trigger is active high."]
    _0 = 0,
    #[doc = "1: Trigger is active low."]
    _1 = 1,
}
impl From<TRGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRGPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGPOL` reader - Trigger Polarity"]
pub struct TRGPOL_R(crate::FieldReader<bool, TRGPOL_A>);
impl TRGPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGPOL_A {
        match self.bits {
            false => TRGPOL_A::_0,
            true => TRGPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGPOL_A::_1
    }
}
impl core::ops::Deref for TRGPOL_R {
    type Target = crate::FieldReader<bool, TRGPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGPOL` writer - Trigger Polarity"]
pub struct TRGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGPOL_A::_0)
    }
    #[doc = "Trigger is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSRC_A {
    #[doc = "0: Trigger source selected by TRGSEL is external."]
    _0 = 0,
    #[doc = "1: Trigger source selected by TRGSEL is internal (channel pin input capture)."]
    _1 = 1,
}
impl From<TRGSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TRGSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSRC` reader - Trigger Source"]
pub struct TRGSRC_R(crate::FieldReader<bool, TRGSRC_A>);
impl TRGSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSRC_A {
        match self.bits {
            false => TRGSRC_A::_0,
            true => TRGSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSRC_A::_1
    }
}
impl core::ops::Deref for TRGSRC_R {
    type Target = crate::FieldReader<bool, TRGSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSRC` writer - Trigger Source"]
pub struct TRGSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger source selected by TRGSEL is external."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSRC_A::_0)
    }
    #[doc = "Trigger source selected by TRGSEL is internal (channel pin input capture)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "1: Channel 0 pin input capture"]
    _0001 = 1,
    #[doc = "2: Channel 1 pin input capture"]
    _0010 = 2,
    #[doc = "3: Channel 0 or Channel 1 pin input capture"]
    _0011 = 3,
    #[doc = "4: Channel 2 pin input capture"]
    _0100 = 4,
    #[doc = "5: Channel 0 or Channel 2 pin input capture"]
    _0101 = 5,
    #[doc = "6: Channel 1 or Channel 2 pin input capture"]
    _0110 = 6,
    #[doc = "7: Channel 0 or Channel 1 or Channel 2 pin input capture"]
    _0111 = 7,
    #[doc = "8: Channel 3 pin input capture"]
    _1000 = 8,
    #[doc = "9: Channel 0 or Channel 3 pin input capture"]
    _1001 = 9,
    #[doc = "10: Channel 1 or Channel 3 pin input capture"]
    _1010 = 10,
    #[doc = "11: Channel 0 or Channel 1 or Channel 3 pin input capture"]
    _1011 = 11,
    #[doc = "12: Channel 2 or Channel 3 pin input capture"]
    _1100 = 12,
    #[doc = "13: Channel 0 or Channel 2 or Channel 3 pin input capture"]
    _1101 = 13,
    #[doc = "14: Channel 1 or Channel 2 or Channel 3 pin input capture"]
    _1110 = 14,
    #[doc = "15: Channel 0 or Channel 1 or Channel 2 or Channel 3 pin input capture"]
    _1111 = 15,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Select"]
pub struct TRGSEL_R(crate::FieldReader<u8, TRGSEL_A>);
impl TRGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSEL_A> {
        match self.bits {
            1 => Some(TRGSEL_A::_0001),
            2 => Some(TRGSEL_A::_0010),
            3 => Some(TRGSEL_A::_0011),
            4 => Some(TRGSEL_A::_0100),
            5 => Some(TRGSEL_A::_0101),
            6 => Some(TRGSEL_A::_0110),
            7 => Some(TRGSEL_A::_0111),
            8 => Some(TRGSEL_A::_1000),
            9 => Some(TRGSEL_A::_1001),
            10 => Some(TRGSEL_A::_1010),
            11 => Some(TRGSEL_A::_1011),
            12 => Some(TRGSEL_A::_1100),
            13 => Some(TRGSEL_A::_1101),
            14 => Some(TRGSEL_A::_1110),
            15 => Some(TRGSEL_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == TRGSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == TRGSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == TRGSEL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == TRGSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == TRGSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == TRGSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == TRGSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == TRGSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == TRGSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == TRGSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == TRGSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == TRGSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == TRGSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == TRGSEL_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == TRGSEL_A::_1111
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, TRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Select"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0 pin input capture"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0001)
    }
    #[doc = "Channel 1 pin input capture"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0010)
    }
    #[doc = "Channel 0 or Channel 1 pin input capture"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0011)
    }
    #[doc = "Channel 2 pin input capture"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0100)
    }
    #[doc = "Channel 0 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0101)
    }
    #[doc = "Channel 1 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0110)
    }
    #[doc = "Channel 0 or Channel 1 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0111)
    }
    #[doc = "Channel 3 pin input capture"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1000)
    }
    #[doc = "Channel 0 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1001)
    }
    #[doc = "Channel 1 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1010)
    }
    #[doc = "Channel 0 or Channel 1 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1011)
    }
    #[doc = "Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1100)
    }
    #[doc = "Channel 0 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1101)
    }
    #[doc = "Channel 1 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1110)
    }
    #[doc = "Channel 0 or Channel 1 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn dbgmode(&self) -> DBGMODE_R {
        DBGMODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Global Time Base Synchronization"]
    #[inline(always)]
    pub fn gtbsync(&self) -> GTBSYNC_R {
        GTBSYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline(always)]
    pub fn gtbeen(&self) -> GTBEEN_R {
        GTBEEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline(always)]
    pub fn csot(&self) -> CSOT_R {
        CSOT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline(always)]
    pub fn csoo(&self) -> CSOO_R {
        CSOO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline(always)]
    pub fn crot(&self) -> CROT_R {
        CROT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Counter Pause On Trigger"]
    #[inline(always)]
    pub fn cpot(&self) -> CPOT_R {
        CPOT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&self) -> TRGPOL_R {
        TRGPOL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&mut self) -> DOZEEN_W {
        DOZEEN_W { w: self }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn dbgmode(&mut self) -> DBGMODE_W {
        DBGMODE_W { w: self }
    }
    #[doc = "Bit 8 - Global Time Base Synchronization"]
    #[inline(always)]
    pub fn gtbsync(&mut self) -> GTBSYNC_W {
        GTBSYNC_W { w: self }
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline(always)]
    pub fn gtbeen(&mut self) -> GTBEEN_W {
        GTBEEN_W { w: self }
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline(always)]
    pub fn csot(&mut self) -> CSOT_W {
        CSOT_W { w: self }
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline(always)]
    pub fn csoo(&mut self) -> CSOO_W {
        CSOO_W { w: self }
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline(always)]
    pub fn crot(&mut self) -> CROT_W {
        CROT_W { w: self }
    }
    #[doc = "Bit 19 - Counter Pause On Trigger"]
    #[inline(always)]
    pub fn cpot(&mut self) -> CPOT_W {
        CPOT_W { w: self }
    }
    #[doc = "Bit 22 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&mut self) -> TRGPOL_W {
        TRGPOL_W { w: self }
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W {
        TRGSRC_W { w: self }
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
