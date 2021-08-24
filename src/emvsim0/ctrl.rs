#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Inverse Convention\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC_A {
    #[doc = "0: Direction convention transfers enabled (default)"]
    _0 = 0,
    #[doc = "1: Inverse convention transfers enabled"]
    _1 = 1,
}
impl From<IC_A> for bool {
    #[inline(always)]
    fn from(variant: IC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC` reader - Inverse Convention"]
pub struct IC_R(crate::FieldReader<bool, IC_A>);
impl IC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_A {
        match self.bits {
            false => IC_A::_0,
            true => IC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IC_A::_1
    }
}
impl core::ops::Deref for IC_R {
    type Target = crate::FieldReader<bool, IC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC` writer - Inverse Convention"]
pub struct IC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Direction convention transfers enabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IC_A::_0)
    }
    #[doc = "Inverse convention transfers enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IC_A::_1)
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
#[doc = "Initial Character Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICM_A {
    #[doc = "0: Initial Character Mode disabled"]
    _0 = 0,
    #[doc = "1: Initial Character Mode enabled (default)"]
    _1 = 1,
}
impl From<ICM_A> for bool {
    #[inline(always)]
    fn from(variant: ICM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICM` reader - Initial Character Mode"]
pub struct ICM_R(crate::FieldReader<bool, ICM_A>);
impl ICM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICM_A {
        match self.bits {
            false => ICM_A::_0,
            true => ICM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ICM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ICM_A::_1
    }
}
impl core::ops::Deref for ICM_R {
    type Target = crate::FieldReader<bool, ICM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICM` writer - Initial Character Mode"]
pub struct ICM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Initial Character Mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICM_A::_0)
    }
    #[doc = "Initial Character Mode enabled (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICM_A::_1)
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
#[doc = "Auto NACK Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACK_A {
    #[doc = "0: NACK generation on errors disabled"]
    _0 = 0,
    #[doc = "1: NACK generation on errors enabled (default)"]
    _1 = 1,
}
impl From<ANACK_A> for bool {
    #[inline(always)]
    fn from(variant: ANACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACK` reader - Auto NACK Enable"]
pub struct ANACK_R(crate::FieldReader<bool, ANACK_A>);
impl ANACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANACK_A {
        match self.bits {
            false => ANACK_A::_0,
            true => ANACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ANACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ANACK_A::_1
    }
}
impl core::ops::Deref for ANACK_R {
    type Target = crate::FieldReader<bool, ANACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANACK` writer - Auto NACK Enable"]
pub struct ANACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NACK generation on errors disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANACK_A::_0)
    }
    #[doc = "NACK generation on errors enabled (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANACK_A::_1)
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
#[doc = "Overrun NACK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONACK_A {
    #[doc = "0: NACK generation on overrun is disabled (default)"]
    _0 = 0,
    #[doc = "1: NACK generation on overrun is enabled"]
    _1 = 1,
}
impl From<ONACK_A> for bool {
    #[inline(always)]
    fn from(variant: ONACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONACK` reader - Overrun NACK Enable"]
pub struct ONACK_R(crate::FieldReader<bool, ONACK_A>);
impl ONACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONACK_A {
        match self.bits {
            false => ONACK_A::_0,
            true => ONACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ONACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ONACK_A::_1
    }
}
impl core::ops::Deref for ONACK_R {
    type Target = crate::FieldReader<bool, ONACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONACK` writer - Overrun NACK Enable"]
pub struct ONACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ONACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NACK generation on overrun is disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONACK_A::_0)
    }
    #[doc = "NACK generation on overrun is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONACK_A::_1)
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
#[doc = "Flush Receiver Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLSH_RX_AW {
    #[doc = "0: EMV SIM Receiver normal operation (default)"]
    _0 = 0,
    #[doc = "1: EMV SIM Receiver held in Reset"]
    _1 = 1,
}
impl From<FLSH_RX_AW> for bool {
    #[inline(always)]
    fn from(variant: FLSH_RX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLSH_RX` writer - Flush Receiver Bit"]
pub struct FLSH_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> FLSH_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLSH_RX_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EMV SIM Receiver normal operation (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLSH_RX_AW::_0)
    }
    #[doc = "EMV SIM Receiver held in Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLSH_RX_AW::_1)
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
#[doc = "Flush Transmitter Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLSH_TX_AW {
    #[doc = "0: EMV SIM Transmitter normal operation (default)"]
    _0 = 0,
    #[doc = "1: EMV SIM Transmitter held in Reset"]
    _1 = 1,
}
impl From<FLSH_TX_AW> for bool {
    #[inline(always)]
    fn from(variant: FLSH_TX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLSH_TX` writer - Flush Transmitter Bit"]
pub struct FLSH_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> FLSH_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLSH_TX_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EMV SIM Transmitter normal operation (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLSH_TX_AW::_0)
    }
    #[doc = "EMV SIM Transmitter held in Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLSH_TX_AW::_1)
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
#[doc = "Software Reset Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_AW {
    #[doc = "0: EMV SIM Normal operation (default)"]
    _0 = 0,
    #[doc = "1: EMV SIM held in Reset"]
    _1 = 1,
}
impl From<SW_RST_AW> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RST` writer - Software Reset Bit"]
pub struct SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_RST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EMV SIM Normal operation (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SW_RST_AW::_0)
    }
    #[doc = "EMV SIM held in Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SW_RST_AW::_1)
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
#[doc = "Kill all internal clocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KILL_CLOCKS_A {
    #[doc = "0: EMV SIM input clock enabled (default)"]
    _0 = 0,
    #[doc = "1: EMV SIM input clock is disabled"]
    _1 = 1,
}
impl From<KILL_CLOCKS_A> for bool {
    #[inline(always)]
    fn from(variant: KILL_CLOCKS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KILL_CLOCKS` reader - Kill all internal clocks"]
pub struct KILL_CLOCKS_R(crate::FieldReader<bool, KILL_CLOCKS_A>);
impl KILL_CLOCKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        KILL_CLOCKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KILL_CLOCKS_A {
        match self.bits {
            false => KILL_CLOCKS_A::_0,
            true => KILL_CLOCKS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == KILL_CLOCKS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == KILL_CLOCKS_A::_1
    }
}
impl core::ops::Deref for KILL_CLOCKS_R {
    type Target = crate::FieldReader<bool, KILL_CLOCKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KILL_CLOCKS` writer - Kill all internal clocks"]
pub struct KILL_CLOCKS_W<'a> {
    w: &'a mut W,
}
impl<'a> KILL_CLOCKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KILL_CLOCKS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EMV SIM input clock enabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KILL_CLOCKS_A::_0)
    }
    #[doc = "EMV SIM input clock is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KILL_CLOCKS_A::_1)
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
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZE_EN_A {
    #[doc = "0: DOZE instruction will gate all internal EMV SIM clocks as well as the Smart Card clock when the transmit FIFO is empty (default)"]
    _0 = 0,
    #[doc = "1: DOZE instruction has no effect on EMV SIM module"]
    _1 = 1,
}
impl From<DOZE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZE_EN` reader - Doze Enable"]
pub struct DOZE_EN_R(crate::FieldReader<bool, DOZE_EN_A>);
impl DOZE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOZE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZE_EN_A {
        match self.bits {
            false => DOZE_EN_A::_0,
            true => DOZE_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOZE_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOZE_EN_A::_1
    }
}
impl core::ops::Deref for DOZE_EN_R {
    type Target = crate::FieldReader<bool, DOZE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZE_EN` writer - Doze Enable"]
pub struct DOZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DOZE instruction will gate all internal EMV SIM clocks as well as the Smart Card clock when the transmit FIFO is empty (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZE_EN_A::_0)
    }
    #[doc = "DOZE instruction has no effect on EMV SIM module"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZE_EN_A::_1)
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
#[doc = "STOP Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_EN_A {
    #[doc = "0: STOP instruction shuts down all EMV SIM clocks (default)"]
    _0 = 0,
    #[doc = "1: STOP instruction shuts down all clocks except for the Smart Card Clock (SCK) (clock provided to Smart Card)"]
    _1 = 1,
}
impl From<STOP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_EN` reader - STOP Enable"]
pub struct STOP_EN_R(crate::FieldReader<bool, STOP_EN_A>);
impl STOP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_EN_A {
        match self.bits {
            false => STOP_EN_A::_0,
            true => STOP_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STOP_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STOP_EN_A::_1
    }
}
impl core::ops::Deref for STOP_EN_R {
    type Target = crate::FieldReader<bool, STOP_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_EN` writer - STOP Enable"]
pub struct STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "STOP instruction shuts down all EMV SIM clocks (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOP_EN_A::_0)
    }
    #[doc = "STOP instruction shuts down all clocks except for the Smart Card Clock (SCK) (clock provided to Smart Card)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOP_EN_A::_1)
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
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCV_EN_A {
    #[doc = "0: EMV SIM Receiver disabled (default)"]
    _0 = 0,
    #[doc = "1: EMV SIM Receiver enabled"]
    _1 = 1,
}
impl From<RCV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RCV_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCV_EN` reader - Receiver Enable"]
pub struct RCV_EN_R(crate::FieldReader<bool, RCV_EN_A>);
impl RCV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCV_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCV_EN_A {
        match self.bits {
            false => RCV_EN_A::_0,
            true => RCV_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCV_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCV_EN_A::_1
    }
}
impl core::ops::Deref for RCV_EN_R {
    type Target = crate::FieldReader<bool, RCV_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV_EN` writer - Receiver Enable"]
pub struct RCV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCV_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCV_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EMV SIM Receiver disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCV_EN_A::_0)
    }
    #[doc = "EMV SIM Receiver enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCV_EN_A::_1)
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
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XMT_EN_A {
    #[doc = "0: EMV SIM Transmitter disabled (default)"]
    _0 = 0,
    #[doc = "1: EMV SIM Transmitter enabled"]
    _1 = 1,
}
impl From<XMT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: XMT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XMT_EN` reader - Transmitter Enable"]
pub struct XMT_EN_R(crate::FieldReader<bool, XMT_EN_A>);
impl XMT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XMT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XMT_EN_A {
        match self.bits {
            false => XMT_EN_A::_0,
            true => XMT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == XMT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == XMT_EN_A::_1
    }
}
impl core::ops::Deref for XMT_EN_R {
    type Target = crate::FieldReader<bool, XMT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XMT_EN` writer - Transmitter Enable"]
pub struct XMT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XMT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XMT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EMV SIM Transmitter disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(XMT_EN_A::_0)
    }
    #[doc = "EMV SIM Transmitter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(XMT_EN_A::_1)
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
#[doc = "Receiver 11 ETU Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCVR_11_A {
    #[doc = "0: Receiver configured for 12 ETU operation mode (default)"]
    _0 = 0,
    #[doc = "1: Receiver configured for 11 ETU operation mode"]
    _1 = 1,
}
impl From<RCVR_11_A> for bool {
    #[inline(always)]
    fn from(variant: RCVR_11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCVR_11` reader - Receiver 11 ETU Mode Enable"]
pub struct RCVR_11_R(crate::FieldReader<bool, RCVR_11_A>);
impl RCVR_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCVR_11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCVR_11_A {
        match self.bits {
            false => RCVR_11_A::_0,
            true => RCVR_11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCVR_11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCVR_11_A::_1
    }
}
impl core::ops::Deref for RCVR_11_R {
    type Target = crate::FieldReader<bool, RCVR_11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCVR_11` writer - Receiver 11 ETU Mode Enable"]
pub struct RCVR_11_W<'a> {
    w: &'a mut W,
}
impl<'a> RCVR_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCVR_11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receiver configured for 12 ETU operation mode (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCVR_11_A::_0)
    }
    #[doc = "Receiver configured for 11 ETU operation mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCVR_11_A::_1)
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
#[doc = "Receive DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DMA_EN_A {
    #[doc = "0: No DMA Read Request asserted for Receiver (default)"]
    _0 = 0,
    #[doc = "1: DMA Read Request asserted for Receiver"]
    _1 = 1,
}
impl From<RX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DMA_EN` reader - Receive DMA Enable"]
pub struct RX_DMA_EN_R(crate::FieldReader<bool, RX_DMA_EN_A>);
impl RX_DMA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DMA_EN_A {
        match self.bits {
            false => RX_DMA_EN_A::_0,
            true => RX_DMA_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RX_DMA_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RX_DMA_EN_A::_1
    }
}
impl core::ops::Deref for RX_DMA_EN_R {
    type Target = crate::FieldReader<bool, RX_DMA_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DMA_EN` writer - Receive DMA Enable"]
pub struct RX_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA Read Request asserted for Receiver (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DMA_EN_A::_0)
    }
    #[doc = "DMA Read Request asserted for Receiver"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DMA_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Transmit DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DMA_EN_A {
    #[doc = "0: No DMA Write Request asserted for Transmitter (default)"]
    _0 = 0,
    #[doc = "1: DMA Write Request asserted for Transmitter"]
    _1 = 1,
}
impl From<TX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_DMA_EN` reader - Transmit DMA Enable"]
pub struct TX_DMA_EN_R(crate::FieldReader<bool, TX_DMA_EN_A>);
impl TX_DMA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DMA_EN_A {
        match self.bits {
            false => TX_DMA_EN_A::_0,
            true => TX_DMA_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TX_DMA_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TX_DMA_EN_A::_1
    }
}
impl core::ops::Deref for TX_DMA_EN_R {
    type Target = crate::FieldReader<bool, TX_DMA_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA_EN` writer - Transmit DMA Enable"]
pub struct TX_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA Write Request asserted for Transmitter (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_DMA_EN_A::_0)
    }
    #[doc = "DMA Write Request asserted for Transmitter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_DMA_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Invert bits in the CRC Output Value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_CRC_VAL_A {
    #[doc = "0: Bits in CRC Output value will not be inverted."]
    _0 = 0,
    #[doc = "1: Bits in CRC Output value will be inverted. (default)"]
    _1 = 1,
}
impl From<INV_CRC_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: INV_CRC_VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV_CRC_VAL` reader - Invert bits in the CRC Output Value"]
pub struct INV_CRC_VAL_R(crate::FieldReader<bool, INV_CRC_VAL_A>);
impl INV_CRC_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        INV_CRC_VAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_CRC_VAL_A {
        match self.bits {
            false => INV_CRC_VAL_A::_0,
            true => INV_CRC_VAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INV_CRC_VAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INV_CRC_VAL_A::_1
    }
}
impl core::ops::Deref for INV_CRC_VAL_R {
    type Target = crate::FieldReader<bool, INV_CRC_VAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_CRC_VAL` writer - Invert bits in the CRC Output Value"]
pub struct INV_CRC_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_CRC_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_CRC_VAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bits in CRC Output value will not be inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_CRC_VAL_A::_0)
    }
    #[doc = "Bits in CRC Output value will be inverted. (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_CRC_VAL_A::_1)
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
#[doc = "CRC Output Value Bit Reversal or Flip\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_OUT_FLIP_A {
    #[doc = "0: Bits within the CRC output bytes will not be reversed i.e. 15:0 will remain 15:0 (default)"]
    _0 = 0,
    #[doc = "1: Bits within the CRC output bytes will be reversed i.e. 15:0 will become {8:15,0:7}"]
    _1 = 1,
}
impl From<CRC_OUT_FLIP_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_OUT_FLIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_OUT_FLIP` reader - CRC Output Value Bit Reversal or Flip"]
pub struct CRC_OUT_FLIP_R(crate::FieldReader<bool, CRC_OUT_FLIP_A>);
impl CRC_OUT_FLIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_OUT_FLIP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_OUT_FLIP_A {
        match self.bits {
            false => CRC_OUT_FLIP_A::_0,
            true => CRC_OUT_FLIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRC_OUT_FLIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRC_OUT_FLIP_A::_1
    }
}
impl core::ops::Deref for CRC_OUT_FLIP_R {
    type Target = crate::FieldReader<bool, CRC_OUT_FLIP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_OUT_FLIP` writer - CRC Output Value Bit Reversal or Flip"]
pub struct CRC_OUT_FLIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_OUT_FLIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_OUT_FLIP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bits within the CRC output bytes will not be reversed i.e. 15:0 will remain 15:0 (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_OUT_FLIP_A::_0)
    }
    #[doc = "Bits within the CRC output bytes will be reversed i.e. 15:0 will become {8:15,0:7}"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_OUT_FLIP_A::_1)
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
#[doc = "CRC Input Byte's Bit Reversal or Flip Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_IN_FLIP_A {
    #[doc = "0: Bits in the input byte will not be reversed (i.e. 7:0 will remain 7:0) before the CRC calculation (default)"]
    _0 = 0,
    #[doc = "1: Bits in the input byte will be reversed (i.e. 7:0 will become 0:7) before CRC calculation"]
    _1 = 1,
}
impl From<CRC_IN_FLIP_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_IN_FLIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_IN_FLIP` reader - CRC Input Byte's Bit Reversal or Flip Control"]
pub struct CRC_IN_FLIP_R(crate::FieldReader<bool, CRC_IN_FLIP_A>);
impl CRC_IN_FLIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_IN_FLIP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_IN_FLIP_A {
        match self.bits {
            false => CRC_IN_FLIP_A::_0,
            true => CRC_IN_FLIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRC_IN_FLIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRC_IN_FLIP_A::_1
    }
}
impl core::ops::Deref for CRC_IN_FLIP_R {
    type Target = crate::FieldReader<bool, CRC_IN_FLIP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_IN_FLIP` writer - CRC Input Byte's Bit Reversal or Flip Control"]
pub struct CRC_IN_FLIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_IN_FLIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_IN_FLIP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bits in the input byte will not be reversed (i.e. 7:0 will remain 7:0) before the CRC calculation (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_IN_FLIP_A::_0)
    }
    #[doc = "Bits in the input byte will be reversed (i.e. 7:0 will become 0:7) before CRC calculation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_IN_FLIP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Character Wait Time Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWT_EN_A {
    #[doc = "0: Character Wait time Counter is disabled (default)"]
    _0 = 0,
    #[doc = "1: Character Wait time counter is enabled"]
    _1 = 1,
}
impl From<CWT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CWT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWT_EN` reader - Character Wait Time Counter Enable"]
pub struct CWT_EN_R(crate::FieldReader<bool, CWT_EN_A>);
impl CWT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CWT_EN_A {
        match self.bits {
            false => CWT_EN_A::_0,
            true => CWT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CWT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CWT_EN_A::_1
    }
}
impl core::ops::Deref for CWT_EN_R {
    type Target = crate::FieldReader<bool, CWT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWT_EN` writer - Character Wait Time Counter Enable"]
pub struct CWT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CWT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Character Wait time Counter is disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWT_EN_A::_0)
    }
    #[doc = "Character Wait time counter is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWT_EN_A::_1)
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
#[doc = "LRC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC_EN_A {
    #[doc = "0: 8-bit Linear Redundancy Checking disabled (default)"]
    _0 = 0,
    #[doc = "1: 8-bit Linear Redundancy Checking enabled"]
    _1 = 1,
}
impl From<LRC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LRC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRC_EN` reader - LRC Enable"]
pub struct LRC_EN_R(crate::FieldReader<bool, LRC_EN_A>);
impl LRC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRC_EN_A {
        match self.bits {
            false => LRC_EN_A::_0,
            true => LRC_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LRC_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LRC_EN_A::_1
    }
}
impl core::ops::Deref for LRC_EN_R {
    type Target = crate::FieldReader<bool, LRC_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRC_EN` writer - LRC Enable"]
pub struct LRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LRC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "8-bit Linear Redundancy Checking disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRC_EN_A::_0)
    }
    #[doc = "8-bit Linear Redundancy Checking enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRC_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "CRC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_EN_A {
    #[doc = "0: 16-bit Cyclic Redundancy Checking disabled (default)"]
    _0 = 0,
    #[doc = "1: 16-bit Cyclic Redundancy Checking enabled"]
    _1 = 1,
}
impl From<CRC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_EN` reader - CRC Enable"]
pub struct CRC_EN_R(crate::FieldReader<bool, CRC_EN_A>);
impl CRC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_EN_A {
        match self.bits {
            false => CRC_EN_A::_0,
            true => CRC_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRC_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRC_EN_A::_1
    }
}
impl core::ops::Deref for CRC_EN_R {
    type Target = crate::FieldReader<bool, CRC_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_EN` writer - CRC Enable"]
pub struct CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "16-bit Cyclic Redundancy Checking disabled (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_EN_A::_0)
    }
    #[doc = "16-bit Cyclic Redundancy Checking enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Transmit CRC or LRC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XMT_CRC_LRC_A {
    #[doc = "0: No CRC or LRC value is transmitted (default)"]
    _0 = 0,
    #[doc = "1: Transmit LRC or CRC info when FIFO empties (whichever is enabled)"]
    _1 = 1,
}
impl From<XMT_CRC_LRC_A> for bool {
    #[inline(always)]
    fn from(variant: XMT_CRC_LRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XMT_CRC_LRC` reader - Transmit CRC or LRC Enable"]
pub struct XMT_CRC_LRC_R(crate::FieldReader<bool, XMT_CRC_LRC_A>);
impl XMT_CRC_LRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        XMT_CRC_LRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XMT_CRC_LRC_A {
        match self.bits {
            false => XMT_CRC_LRC_A::_0,
            true => XMT_CRC_LRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == XMT_CRC_LRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == XMT_CRC_LRC_A::_1
    }
}
impl core::ops::Deref for XMT_CRC_LRC_R {
    type Target = crate::FieldReader<bool, XMT_CRC_LRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XMT_CRC_LRC` writer - Transmit CRC or LRC Enable"]
pub struct XMT_CRC_LRC_W<'a> {
    w: &'a mut W,
}
impl<'a> XMT_CRC_LRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XMT_CRC_LRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No CRC or LRC value is transmitted (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(XMT_CRC_LRC_A::_0)
    }
    #[doc = "Transmit LRC or CRC info when FIFO empties (whichever is enabled)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(XMT_CRC_LRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Block Wait Time Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWT_EN_A {
    #[doc = "0: Disable BWT, BGT Counters (default)"]
    _0 = 0,
    #[doc = "1: Enable BWT, BGT Counters"]
    _1 = 1,
}
impl From<BWT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BWT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWT_EN` reader - Block Wait Time Counter Enable"]
pub struct BWT_EN_R(crate::FieldReader<bool, BWT_EN_A>);
impl BWT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWT_EN_A {
        match self.bits {
            false => BWT_EN_A::_0,
            true => BWT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BWT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BWT_EN_A::_1
    }
}
impl core::ops::Deref for BWT_EN_R {
    type Target = crate::FieldReader<bool, BWT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWT_EN` writer - Block Wait Time Counter Enable"]
pub struct BWT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BWT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable BWT, BGT Counters (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWT_EN_A::_0)
    }
    #[doc = "Enable BWT, BGT Counters"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWT_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Inverse Convention"]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Initial Character Mode"]
    #[inline(always)]
    pub fn icm(&self) -> ICM_R {
        ICM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto NACK Enable"]
    #[inline(always)]
    pub fn anack(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun NACK Enable"]
    #[inline(always)]
    pub fn onack(&self) -> ONACK_R {
        ONACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Kill all internal clocks"]
    #[inline(always)]
    pub fn kill_clocks(&self) -> KILL_CLOCKS_R {
        KILL_CLOCKS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Doze Enable"]
    #[inline(always)]
    pub fn doze_en(&self) -> DOZE_EN_R {
        DOZE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - STOP Enable"]
    #[inline(always)]
    pub fn stop_en(&self) -> STOP_EN_R {
        STOP_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receiver Enable"]
    #[inline(always)]
    pub fn rcv_en(&self) -> RCV_EN_R {
        RCV_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmitter Enable"]
    #[inline(always)]
    pub fn xmt_en(&self) -> XMT_EN_R {
        XMT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Receiver 11 ETU Mode Enable"]
    #[inline(always)]
    pub fn rcvr_11(&self) -> RCVR_11_R {
        RCVR_11_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive DMA Enable"]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Invert bits in the CRC Output Value"]
    #[inline(always)]
    pub fn inv_crc_val(&self) -> INV_CRC_VAL_R {
        INV_CRC_VAL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CRC Output Value Bit Reversal or Flip"]
    #[inline(always)]
    pub fn crc_out_flip(&self) -> CRC_OUT_FLIP_R {
        CRC_OUT_FLIP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CRC Input Byte's Bit Reversal or Flip Control"]
    #[inline(always)]
    pub fn crc_in_flip(&self) -> CRC_IN_FLIP_R {
        CRC_IN_FLIP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Character Wait Time Counter Enable"]
    #[inline(always)]
    pub fn cwt_en(&self) -> CWT_EN_R {
        CWT_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LRC Enable"]
    #[inline(always)]
    pub fn lrc_en(&self) -> LRC_EN_R {
        LRC_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - CRC Enable"]
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmit CRC or LRC Enable"]
    #[inline(always)]
    pub fn xmt_crc_lrc(&self) -> XMT_CRC_LRC_R {
        XMT_CRC_LRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Block Wait Time Counter Enable"]
    #[inline(always)]
    pub fn bwt_en(&self) -> BWT_EN_R {
        BWT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Inverse Convention"]
    #[inline(always)]
    pub fn ic(&mut self) -> IC_W {
        IC_W { w: self }
    }
    #[doc = "Bit 1 - Initial Character Mode"]
    #[inline(always)]
    pub fn icm(&mut self) -> ICM_W {
        ICM_W { w: self }
    }
    #[doc = "Bit 2 - Auto NACK Enable"]
    #[inline(always)]
    pub fn anack(&mut self) -> ANACK_W {
        ANACK_W { w: self }
    }
    #[doc = "Bit 3 - Overrun NACK Enable"]
    #[inline(always)]
    pub fn onack(&mut self) -> ONACK_W {
        ONACK_W { w: self }
    }
    #[doc = "Bit 8 - Flush Receiver Bit"]
    #[inline(always)]
    pub fn flsh_rx(&mut self) -> FLSH_RX_W {
        FLSH_RX_W { w: self }
    }
    #[doc = "Bit 9 - Flush Transmitter Bit"]
    #[inline(always)]
    pub fn flsh_tx(&mut self) -> FLSH_TX_W {
        FLSH_TX_W { w: self }
    }
    #[doc = "Bit 10 - Software Reset Bit"]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SW_RST_W {
        SW_RST_W { w: self }
    }
    #[doc = "Bit 11 - Kill all internal clocks"]
    #[inline(always)]
    pub fn kill_clocks(&mut self) -> KILL_CLOCKS_W {
        KILL_CLOCKS_W { w: self }
    }
    #[doc = "Bit 12 - Doze Enable"]
    #[inline(always)]
    pub fn doze_en(&mut self) -> DOZE_EN_W {
        DOZE_EN_W { w: self }
    }
    #[doc = "Bit 13 - STOP Enable"]
    #[inline(always)]
    pub fn stop_en(&mut self) -> STOP_EN_W {
        STOP_EN_W { w: self }
    }
    #[doc = "Bit 16 - Receiver Enable"]
    #[inline(always)]
    pub fn rcv_en(&mut self) -> RCV_EN_W {
        RCV_EN_W { w: self }
    }
    #[doc = "Bit 17 - Transmitter Enable"]
    #[inline(always)]
    pub fn xmt_en(&mut self) -> XMT_EN_W {
        XMT_EN_W { w: self }
    }
    #[doc = "Bit 18 - Receiver 11 ETU Mode Enable"]
    #[inline(always)]
    pub fn rcvr_11(&mut self) -> RCVR_11_W {
        RCVR_11_W { w: self }
    }
    #[doc = "Bit 19 - Receive DMA Enable"]
    #[inline(always)]
    pub fn rx_dma_en(&mut self) -> RX_DMA_EN_W {
        RX_DMA_EN_W { w: self }
    }
    #[doc = "Bit 20 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W {
        TX_DMA_EN_W { w: self }
    }
    #[doc = "Bit 24 - Invert bits in the CRC Output Value"]
    #[inline(always)]
    pub fn inv_crc_val(&mut self) -> INV_CRC_VAL_W {
        INV_CRC_VAL_W { w: self }
    }
    #[doc = "Bit 25 - CRC Output Value Bit Reversal or Flip"]
    #[inline(always)]
    pub fn crc_out_flip(&mut self) -> CRC_OUT_FLIP_W {
        CRC_OUT_FLIP_W { w: self }
    }
    #[doc = "Bit 26 - CRC Input Byte's Bit Reversal or Flip Control"]
    #[inline(always)]
    pub fn crc_in_flip(&mut self) -> CRC_IN_FLIP_W {
        CRC_IN_FLIP_W { w: self }
    }
    #[doc = "Bit 27 - Character Wait Time Counter Enable"]
    #[inline(always)]
    pub fn cwt_en(&mut self) -> CWT_EN_W {
        CWT_EN_W { w: self }
    }
    #[doc = "Bit 28 - LRC Enable"]
    #[inline(always)]
    pub fn lrc_en(&mut self) -> LRC_EN_W {
        LRC_EN_W { w: self }
    }
    #[doc = "Bit 29 - CRC Enable"]
    #[inline(always)]
    pub fn crc_en(&mut self) -> CRC_EN_W {
        CRC_EN_W { w: self }
    }
    #[doc = "Bit 30 - Transmit CRC or LRC Enable"]
    #[inline(always)]
    pub fn xmt_crc_lrc(&mut self) -> XMT_CRC_LRC_W {
        XMT_CRC_LRC_W { w: self }
    }
    #[doc = "Bit 31 - Block Wait Time Counter Enable"]
    #[inline(always)]
    pub fn bwt_en(&mut self) -> BWT_EN_W {
        BWT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0100_0006"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0006
    }
}
