#[doc = "Register `PCSR` reader"]
pub struct R(crate::R<PCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSR` writer"]
pub struct W(crate::W<PCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSR_SPEC>;
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
impl From<crate::W<PCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Auto Power Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAPD_A {
    #[doc = "0: Auto power down disabled (default)"]
    _0 = 0,
    #[doc = "1: Auto power down enabled"]
    _1 = 1,
}
impl From<SAPD_A> for bool {
    #[inline(always)]
    fn from(variant: SAPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAPD` reader - Auto Power Down Enable"]
pub struct SAPD_R(crate::FieldReader<bool, SAPD_A>);
impl SAPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAPD_A {
        match self.bits {
            false => SAPD_A::_0,
            true => SAPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SAPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SAPD_A::_1
    }
}
impl core::ops::Deref for SAPD_R {
    type Target = crate::FieldReader<bool, SAPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAPD` writer - Auto Power Down Enable"]
pub struct SAPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Auto power down disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAPD_A::_0)
    }
    #[doc = "Auto power down enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAPD_A::_1)
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
#[doc = "Vcc Enable for Smart Card\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCC_EN_A {
    #[doc = "0: Smart Card Voltage disabled (default)"]
    _0 = 0,
    #[doc = "1: Smart Card Voltage enabled"]
    _1 = 1,
}
impl From<SVCC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SVCC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCC_EN` reader - Vcc Enable for Smart Card"]
pub struct SVCC_EN_R(crate::FieldReader<bool, SVCC_EN_A>);
impl SVCC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVCC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCC_EN_A {
        match self.bits {
            false => SVCC_EN_A::_0,
            true => SVCC_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SVCC_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SVCC_EN_A::_1
    }
}
impl core::ops::Deref for SVCC_EN_R {
    type Target = crate::FieldReader<bool, SVCC_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVCC_EN` writer - Vcc Enable for Smart Card"]
pub struct SVCC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Smart Card Voltage disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVCC_EN_A::_0)
    }
    #[doc = "Smart Card Voltage enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVCC_EN_A::_1)
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
#[doc = "VCC Enable Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCCENP_A {
    #[doc = "0: VCC_EN is active high. Polarity of SVCC_EN is unchanged."]
    _0 = 0,
    #[doc = "1: VCC_EN is active low. Polarity of SVCC_EN is inverted."]
    _1 = 1,
}
impl From<VCCENP_A> for bool {
    #[inline(always)]
    fn from(variant: VCCENP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCCENP` reader - VCC Enable Polarity Control"]
pub struct VCCENP_R(crate::FieldReader<bool, VCCENP_A>);
impl VCCENP_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCCENP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCCENP_A {
        match self.bits {
            false => VCCENP_A::_0,
            true => VCCENP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VCCENP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VCCENP_A::_1
    }
}
impl core::ops::Deref for VCCENP_R {
    type Target = crate::FieldReader<bool, VCCENP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCCENP` writer - VCC Enable Polarity Control"]
pub struct VCCENP_W<'a> {
    w: &'a mut W,
}
impl<'a> VCCENP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCCENP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VCC_EN is active high. Polarity of SVCC_EN is unchanged."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCCENP_A::_0)
    }
    #[doc = "VCC_EN is active low. Polarity of SVCC_EN is inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCCENP_A::_1)
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
#[doc = "Reset to Smart Card\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRST_A {
    #[doc = "0: Smart Card Reset is asserted (default)"]
    _0 = 0,
    #[doc = "1: Smart Card Reset is de-asserted"]
    _1 = 1,
}
impl From<SRST_A> for bool {
    #[inline(always)]
    fn from(variant: SRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRST` reader - Reset to Smart Card"]
pub struct SRST_R(crate::FieldReader<bool, SRST_A>);
impl SRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRST_A {
        match self.bits {
            false => SRST_A::_0,
            true => SRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRST_A::_1
    }
}
impl core::ops::Deref for SRST_R {
    type Target = crate::FieldReader<bool, SRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRST` writer - Reset to Smart Card"]
pub struct SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Smart Card Reset is asserted (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRST_A::_0)
    }
    #[doc = "Smart Card Reset is de-asserted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRST_A::_1)
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
#[doc = "Clock Enable for Smart Card\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCEN_A {
    #[doc = "0: Smart Card Clock Disabled"]
    _0 = 0,
    #[doc = "1: Smart Card Clock Enabled"]
    _1 = 1,
}
impl From<SCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCEN` reader - Clock Enable for Smart Card"]
pub struct SCEN_R(crate::FieldReader<bool, SCEN_A>);
impl SCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCEN_A {
        match self.bits {
            false => SCEN_A::_0,
            true => SCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCEN_A::_1
    }
}
impl core::ops::Deref for SCEN_R {
    type Target = crate::FieldReader<bool, SCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCEN` writer - Clock Enable for Smart Card"]
pub struct SCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Smart Card Clock Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCEN_A::_0)
    }
    #[doc = "Smart Card Clock Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCEN_A::_1)
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
#[doc = "Smart Card Clock Stop Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSP_A {
    #[doc = "0: Clock is logic 0 when stopped by SCEN"]
    _0 = 0,
    #[doc = "1: Clock is logic 1 when stopped by SCEN"]
    _1 = 1,
}
impl From<SCSP_A> for bool {
    #[inline(always)]
    fn from(variant: SCSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCSP` reader - Smart Card Clock Stop Polarity"]
pub struct SCSP_R(crate::FieldReader<bool, SCSP_A>);
impl SCSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCSP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCSP_A {
        match self.bits {
            false => SCSP_A::_0,
            true => SCSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SCSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SCSP_A::_1
    }
}
impl core::ops::Deref for SCSP_R {
    type Target = crate::FieldReader<bool, SCSP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCSP` writer - Smart Card Clock Stop Polarity"]
pub struct SCSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCSP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock is logic 0 when stopped by SCEN"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCSP_A::_0)
    }
    #[doc = "Clock is logic 1 when stopped by SCEN"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCSP_A::_1)
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
#[doc = "Auto Power Down Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPD_A {
    #[doc = "0: No effect (default)"]
    _0 = 0,
    #[doc = "1: Start Auto Powerdown or Power Down is in progress"]
    _1 = 1,
}
impl From<SPD_A> for bool {
    #[inline(always)]
    fn from(variant: SPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPD` reader - Auto Power Down Control"]
pub struct SPD_R(crate::FieldReader<bool, SPD_A>);
impl SPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPD_A {
        match self.bits {
            false => SPD_A::_0,
            true => SPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPD_A::_1
    }
}
impl core::ops::Deref for SPD_R {
    type Target = crate::FieldReader<bool, SPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPD` writer - Auto Power Down Control"]
pub struct SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPD_A::_0)
    }
    #[doc = "Start Auto Powerdown or Power Down is in progress"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Smart Card Presence Detect Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIM_A {
    #[doc = "0: SIM presence detect interrupt is enabled"]
    _0 = 0,
    #[doc = "1: SIM presence detect interrupt is masked (default)"]
    _1 = 1,
}
impl From<SPDIM_A> for bool {
    #[inline(always)]
    fn from(variant: SPDIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPDIM` reader - Smart Card Presence Detect Interrupt Mask"]
pub struct SPDIM_R(crate::FieldReader<bool, SPDIM_A>);
impl SPDIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPDIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIM_A {
        match self.bits {
            false => SPDIM_A::_0,
            true => SPDIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPDIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPDIM_A::_1
    }
}
impl core::ops::Deref for SPDIM_R {
    type Target = crate::FieldReader<bool, SPDIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIM` writer - Smart Card Presence Detect Interrupt Mask"]
pub struct SPDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SIM presence detect interrupt is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDIM_A::_0)
    }
    #[doc = "SIM presence detect interrupt is masked (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDIM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Smart Card Presence Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIF_A {
    #[doc = "0: No insertion or removal of Smart Card detected on Port (default)"]
    _0 = 0,
    #[doc = "1: Insertion or removal of Smart Card detected on Port"]
    _1 = 1,
}
impl From<SPDIF_A> for bool {
    #[inline(always)]
    fn from(variant: SPDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPDIF` reader - Smart Card Presence Detect Interrupt Flag"]
pub struct SPDIF_R(crate::FieldReader<bool, SPDIF_A>);
impl SPDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIF_A {
        match self.bits {
            false => SPDIF_A::_0,
            true => SPDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPDIF_A::_1
    }
}
impl core::ops::Deref for SPDIF_R {
    type Target = crate::FieldReader<bool, SPDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIF` writer - Smart Card Presence Detect Interrupt Flag"]
pub struct SPDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No insertion or removal of Smart Card detected on Port (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDIF_A::_0)
    }
    #[doc = "Insertion or removal of Smart Card detected on Port"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Smart Card Presence Detect Pin Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDP_A {
    #[doc = "0: SIM Presence Detect pin is logic low"]
    _0 = 0,
    #[doc = "1: SIM Presence Detectpin is logic high"]
    _1 = 1,
}
impl From<SPDP_A> for bool {
    #[inline(always)]
    fn from(variant: SPDP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPDP` reader - Smart Card Presence Detect Pin Status"]
pub struct SPDP_R(crate::FieldReader<bool, SPDP_A>);
impl SPDP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPDP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDP_A {
        match self.bits {
            false => SPDP_A::_0,
            true => SPDP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPDP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPDP_A::_1
    }
}
impl core::ops::Deref for SPDP_R {
    type Target = crate::FieldReader<bool, SPDP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SIM Presence Detect Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDES_A {
    #[doc = "0: Falling edge on the pin (default)"]
    _0 = 0,
    #[doc = "1: Rising edge on the pin"]
    _1 = 1,
}
impl From<SPDES_A> for bool {
    #[inline(always)]
    fn from(variant: SPDES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPDES` reader - SIM Presence Detect Edge Select"]
pub struct SPDES_R(crate::FieldReader<bool, SPDES_A>);
impl SPDES_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPDES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDES_A {
        match self.bits {
            false => SPDES_A::_0,
            true => SPDES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPDES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPDES_A::_1
    }
}
impl core::ops::Deref for SPDES_R {
    type Target = crate::FieldReader<bool, SPDES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDES` writer - SIM Presence Detect Edge Select"]
pub struct SPDES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge on the pin (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDES_A::_0)
    }
    #[doc = "Rising edge on the pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDES_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto Power Down Enable"]
    #[inline(always)]
    pub fn sapd(&self) -> SAPD_R {
        SAPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Vcc Enable for Smart Card"]
    #[inline(always)]
    pub fn svcc_en(&self) -> SVCC_EN_R {
        SVCC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VCC Enable Polarity Control"]
    #[inline(always)]
    pub fn vccenp(&self) -> VCCENP_R {
        VCCENP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset to Smart Card"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Enable for Smart Card"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Smart Card Clock Stop Polarity"]
    #[inline(always)]
    pub fn scsp(&self) -> SCSP_R {
        SCSP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Auto Power Down Control"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Smart Card Presence Detect Interrupt Mask"]
    #[inline(always)]
    pub fn spdim(&self) -> SPDIM_R {
        SPDIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Smart Card Presence Detect Interrupt Flag"]
    #[inline(always)]
    pub fn spdif(&self) -> SPDIF_R {
        SPDIF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Smart Card Presence Detect Pin Status"]
    #[inline(always)]
    pub fn spdp(&self) -> SPDP_R {
        SPDP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SIM Presence Detect Edge Select"]
    #[inline(always)]
    pub fn spdes(&self) -> SPDES_R {
        SPDES_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Power Down Enable"]
    #[inline(always)]
    pub fn sapd(&mut self) -> SAPD_W {
        SAPD_W { w: self }
    }
    #[doc = "Bit 1 - Vcc Enable for Smart Card"]
    #[inline(always)]
    pub fn svcc_en(&mut self) -> SVCC_EN_W {
        SVCC_EN_W { w: self }
    }
    #[doc = "Bit 2 - VCC Enable Polarity Control"]
    #[inline(always)]
    pub fn vccenp(&mut self) -> VCCENP_W {
        VCCENP_W { w: self }
    }
    #[doc = "Bit 3 - Reset to Smart Card"]
    #[inline(always)]
    pub fn srst(&mut self) -> SRST_W {
        SRST_W { w: self }
    }
    #[doc = "Bit 4 - Clock Enable for Smart Card"]
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W {
        SCEN_W { w: self }
    }
    #[doc = "Bit 5 - Smart Card Clock Stop Polarity"]
    #[inline(always)]
    pub fn scsp(&mut self) -> SCSP_W {
        SCSP_W { w: self }
    }
    #[doc = "Bit 7 - Auto Power Down Control"]
    #[inline(always)]
    pub fn spd(&mut self) -> SPD_W {
        SPD_W { w: self }
    }
    #[doc = "Bit 24 - Smart Card Presence Detect Interrupt Mask"]
    #[inline(always)]
    pub fn spdim(&mut self) -> SPDIM_W {
        SPDIM_W { w: self }
    }
    #[doc = "Bit 25 - Smart Card Presence Detect Interrupt Flag"]
    #[inline(always)]
    pub fn spdif(&mut self) -> SPDIF_W {
        SPDIF_W { w: self }
    }
    #[doc = "Bit 27 - SIM Presence Detect Edge Select"]
    #[inline(always)]
    pub fn spdes(&mut self) -> SPDES_W {
        SPDES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsr](index.html) module"]
pub struct PCSR_SPEC;
impl crate::RegisterSpec for PCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsr::R](R) reader structure"]
impl crate::Readable for PCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsr::W](W) writer structure"]
impl crate::Writable for PCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCSR to value 0x0100_0000"]
impl crate::Resettable for PCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
