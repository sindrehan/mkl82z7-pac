#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA` writer"]
pub struct W(crate::W<DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_SPEC>;
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
impl From<crate::W<DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R0T0` reader - Read receive data buffer 0 or write transmit data buffer 0."]
pub struct R0T0_R(crate::FieldReader<bool, bool>);
impl R0T0_R {
    pub(crate) fn new(bits: bool) -> Self {
        R0T0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R0T0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0T0` writer - Read receive data buffer 0 or write transmit data buffer 0."]
pub struct R0T0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0T0_W<'a> {
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
#[doc = "Field `R1T1` reader - Read receive data buffer 1 or write transmit data buffer 1."]
pub struct R1T1_R(crate::FieldReader<bool, bool>);
impl R1T1_R {
    pub(crate) fn new(bits: bool) -> Self {
        R1T1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R1T1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R1T1` writer - Read receive data buffer 1 or write transmit data buffer 1."]
pub struct R1T1_W<'a> {
    w: &'a mut W,
}
impl<'a> R1T1_W<'a> {
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
#[doc = "Field `R2T2` reader - Read receive data buffer 2 or write transmit data buffer 2."]
pub struct R2T2_R(crate::FieldReader<bool, bool>);
impl R2T2_R {
    pub(crate) fn new(bits: bool) -> Self {
        R2T2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R2T2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R2T2` writer - Read receive data buffer 2 or write transmit data buffer 2."]
pub struct R2T2_W<'a> {
    w: &'a mut W,
}
impl<'a> R2T2_W<'a> {
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
#[doc = "Field `R3T3` reader - Read receive data buffer 3 or write transmit data buffer 3."]
pub struct R3T3_R(crate::FieldReader<bool, bool>);
impl R3T3_R {
    pub(crate) fn new(bits: bool) -> Self {
        R3T3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R3T3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R3T3` writer - Read receive data buffer 3 or write transmit data buffer 3."]
pub struct R3T3_W<'a> {
    w: &'a mut W,
}
impl<'a> R3T3_W<'a> {
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
#[doc = "Field `R4T4` reader - Read receive data buffer 4 or write transmit data buffer 4."]
pub struct R4T4_R(crate::FieldReader<bool, bool>);
impl R4T4_R {
    pub(crate) fn new(bits: bool) -> Self {
        R4T4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R4T4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R4T4` writer - Read receive data buffer 4 or write transmit data buffer 4."]
pub struct R4T4_W<'a> {
    w: &'a mut W,
}
impl<'a> R4T4_W<'a> {
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
#[doc = "Field `R5T5` reader - Read receive data buffer 5 or write transmit data buffer 5."]
pub struct R5T5_R(crate::FieldReader<bool, bool>);
impl R5T5_R {
    pub(crate) fn new(bits: bool) -> Self {
        R5T5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R5T5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R5T5` writer - Read receive data buffer 5 or write transmit data buffer 5."]
pub struct R5T5_W<'a> {
    w: &'a mut W,
}
impl<'a> R5T5_W<'a> {
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
#[doc = "Field `R6T6` reader - Read receive data buffer 6 or write transmit data buffer 6."]
pub struct R6T6_R(crate::FieldReader<bool, bool>);
impl R6T6_R {
    pub(crate) fn new(bits: bool) -> Self {
        R6T6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R6T6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R6T6` writer - Read receive data buffer 6 or write transmit data buffer 6."]
pub struct R6T6_W<'a> {
    w: &'a mut W,
}
impl<'a> R6T6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `R7T7` reader - Read receive data buffer 7 or write transmit data buffer 7."]
pub struct R7T7_R(crate::FieldReader<bool, bool>);
impl R7T7_R {
    pub(crate) fn new(bits: bool) -> Self {
        R7T7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R7T7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R7T7` writer - Read receive data buffer 7 or write transmit data buffer 7."]
pub struct R7T7_W<'a> {
    w: &'a mut W,
}
impl<'a> R7T7_W<'a> {
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
#[doc = "Field `R8T8` reader - Read receive data buffer 8 or write transmit data buffer 8."]
pub struct R8T8_R(crate::FieldReader<bool, bool>);
impl R8T8_R {
    pub(crate) fn new(bits: bool) -> Self {
        R8T8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R8T8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R8T8` writer - Read receive data buffer 8 or write transmit data buffer 8."]
pub struct R8T8_W<'a> {
    w: &'a mut W,
}
impl<'a> R8T8_W<'a> {
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
#[doc = "Field `R9T9` reader - Read receive data buffer 9 or write transmit data buffer 9."]
pub struct R9T9_R(crate::FieldReader<bool, bool>);
impl R9T9_R {
    pub(crate) fn new(bits: bool) -> Self {
        R9T9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R9T9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R9T9` writer - Read receive data buffer 9 or write transmit data buffer 9."]
pub struct R9T9_W<'a> {
    w: &'a mut W,
}
impl<'a> R9T9_W<'a> {
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
#[doc = "Idle Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLINE_A {
    #[doc = "0: Receiver was not idle before receiving this character."]
    _0 = 0,
    #[doc = "1: Receiver was idle before receiving this character."]
    _1 = 1,
}
impl From<IDLINE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLINE` reader - Idle Line"]
pub struct IDLINE_R(crate::FieldReader<bool, IDLINE_A>);
impl IDLINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLINE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLINE_A {
        match self.bits {
            false => IDLINE_A::_0,
            true => IDLINE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDLINE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDLINE_A::_1
    }
}
impl core::ops::Deref for IDLINE_R {
    type Target = crate::FieldReader<bool, IDLINE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPT_A {
    #[doc = "0: Receive buffer contains valid data."]
    _0 = 0,
    #[doc = "1: Receive buffer is empty, data returned on read is not valid."]
    _1 = 1,
}
impl From<RXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPT` reader - Receive Buffer Empty"]
pub struct RXEMPT_R(crate::FieldReader<bool, RXEMPT_A>);
impl RXEMPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEMPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPT_A {
        match self.bits {
            false => RXEMPT_A::_0,
            true => RXEMPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXEMPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXEMPT_A::_1
    }
}
impl core::ops::Deref for RXEMPT_R {
    type Target = crate::FieldReader<bool, RXEMPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Frame Error / Transmit Special Character\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRETSC_A {
    #[doc = "0: The dataword was received without a frame error on read, transmit a normal character on write."]
    _0 = 0,
    #[doc = "1: The dataword was received with a frame error, transmit an idle or break character on transmit."]
    _1 = 1,
}
impl From<FRETSC_A> for bool {
    #[inline(always)]
    fn from(variant: FRETSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRETSC` reader - Frame Error / Transmit Special Character"]
pub struct FRETSC_R(crate::FieldReader<bool, FRETSC_A>);
impl FRETSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRETSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRETSC_A {
        match self.bits {
            false => FRETSC_A::_0,
            true => FRETSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRETSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRETSC_A::_1
    }
}
impl core::ops::Deref for FRETSC_R {
    type Target = crate::FieldReader<bool, FRETSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRETSC` writer - Frame Error / Transmit Special Character"]
pub struct FRETSC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRETSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRETSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The dataword was received without a frame error on read, transmit a normal character on write."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRETSC_A::_0)
    }
    #[doc = "The dataword was received with a frame error, transmit an idle or break character on transmit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRETSC_A::_1)
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
#[doc = "The current received dataword contained in DATA\\[R9:R0\\]
was received with a parity error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYE_A {
    #[doc = "0: The dataword was received without a parity error."]
    _0 = 0,
    #[doc = "1: The dataword was received with a parity error."]
    _1 = 1,
}
impl From<PARITYE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITYE` reader - The current received dataword contained in DATA\\[R9:R0\\]
was received with a parity error."]
pub struct PARITYE_R(crate::FieldReader<bool, PARITYE_A>);
impl PARITYE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITYE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYE_A {
        match self.bits {
            false => PARITYE_A::_0,
            true => PARITYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PARITYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PARITYE_A::_1
    }
}
impl core::ops::Deref for PARITYE_R {
    type Target = crate::FieldReader<bool, PARITYE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "The current received dataword contained in DATA\\[R9:R0\\]
was received with noise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOISY_A {
    #[doc = "0: The dataword was received without noise."]
    _0 = 0,
    #[doc = "1: The data was received with noise."]
    _1 = 1,
}
impl From<NOISY_A> for bool {
    #[inline(always)]
    fn from(variant: NOISY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOISY` reader - The current received dataword contained in DATA\\[R9:R0\\]
was received with noise."]
pub struct NOISY_R(crate::FieldReader<bool, NOISY_A>);
impl NOISY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOISY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOISY_A {
        match self.bits {
            false => NOISY_A::_0,
            true => NOISY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NOISY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NOISY_A::_1
    }
}
impl core::ops::Deref for NOISY_R {
    type Target = crate::FieldReader<bool, NOISY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read receive data buffer 0 or write transmit data buffer 0."]
    #[inline(always)]
    pub fn r0t0(&self) -> R0T0_R {
        R0T0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read receive data buffer 1 or write transmit data buffer 1."]
    #[inline(always)]
    pub fn r1t1(&self) -> R1T1_R {
        R1T1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read receive data buffer 2 or write transmit data buffer 2."]
    #[inline(always)]
    pub fn r2t2(&self) -> R2T2_R {
        R2T2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read receive data buffer 3 or write transmit data buffer 3."]
    #[inline(always)]
    pub fn r3t3(&self) -> R3T3_R {
        R3T3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read receive data buffer 4 or write transmit data buffer 4."]
    #[inline(always)]
    pub fn r4t4(&self) -> R4T4_R {
        R4T4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read receive data buffer 5 or write transmit data buffer 5."]
    #[inline(always)]
    pub fn r5t5(&self) -> R5T5_R {
        R5T5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read receive data buffer 6 or write transmit data buffer 6."]
    #[inline(always)]
    pub fn r6t6(&self) -> R6T6_R {
        R6T6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read receive data buffer 7 or write transmit data buffer 7."]
    #[inline(always)]
    pub fn r7t7(&self) -> R7T7_R {
        R7T7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Read receive data buffer 8 or write transmit data buffer 8."]
    #[inline(always)]
    pub fn r8t8(&self) -> R8T8_R {
        R8T8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read receive data buffer 9 or write transmit data buffer 9."]
    #[inline(always)]
    pub fn r9t9(&self) -> R9T9_R {
        R9T9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Idle Line"]
    #[inline(always)]
    pub fn idline(&self) -> IDLINE_R {
        IDLINE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RXEMPT_R {
        RXEMPT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline(always)]
    pub fn fretsc(&self) -> FRETSC_R {
        FRETSC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The current received dataword contained in DATA\\[R9:R0\\]
was received with a parity error."]
    #[inline(always)]
    pub fn paritye(&self) -> PARITYE_R {
        PARITYE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The current received dataword contained in DATA\\[R9:R0\\]
was received with noise."]
    #[inline(always)]
    pub fn noisy(&self) -> NOISY_R {
        NOISY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read receive data buffer 0 or write transmit data buffer 0."]
    #[inline(always)]
    pub fn r0t0(&mut self) -> R0T0_W {
        R0T0_W { w: self }
    }
    #[doc = "Bit 1 - Read receive data buffer 1 or write transmit data buffer 1."]
    #[inline(always)]
    pub fn r1t1(&mut self) -> R1T1_W {
        R1T1_W { w: self }
    }
    #[doc = "Bit 2 - Read receive data buffer 2 or write transmit data buffer 2."]
    #[inline(always)]
    pub fn r2t2(&mut self) -> R2T2_W {
        R2T2_W { w: self }
    }
    #[doc = "Bit 3 - Read receive data buffer 3 or write transmit data buffer 3."]
    #[inline(always)]
    pub fn r3t3(&mut self) -> R3T3_W {
        R3T3_W { w: self }
    }
    #[doc = "Bit 4 - Read receive data buffer 4 or write transmit data buffer 4."]
    #[inline(always)]
    pub fn r4t4(&mut self) -> R4T4_W {
        R4T4_W { w: self }
    }
    #[doc = "Bit 5 - Read receive data buffer 5 or write transmit data buffer 5."]
    #[inline(always)]
    pub fn r5t5(&mut self) -> R5T5_W {
        R5T5_W { w: self }
    }
    #[doc = "Bit 6 - Read receive data buffer 6 or write transmit data buffer 6."]
    #[inline(always)]
    pub fn r6t6(&mut self) -> R6T6_W {
        R6T6_W { w: self }
    }
    #[doc = "Bit 7 - Read receive data buffer 7 or write transmit data buffer 7."]
    #[inline(always)]
    pub fn r7t7(&mut self) -> R7T7_W {
        R7T7_W { w: self }
    }
    #[doc = "Bit 8 - Read receive data buffer 8 or write transmit data buffer 8."]
    #[inline(always)]
    pub fn r8t8(&mut self) -> R8T8_W {
        R8T8_W { w: self }
    }
    #[doc = "Bit 9 - Read receive data buffer 9 or write transmit data buffer 9."]
    #[inline(always)]
    pub fn r9t9(&mut self) -> R9T9_W {
        R9T9_W { w: self }
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline(always)]
    pub fn fretsc(&mut self) -> FRETSC_W {
        FRETSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data::W](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA to value 0x1000"]
impl crate::Resettable for DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
