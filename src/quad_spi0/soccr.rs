#[doc = "Register `SOCCR` reader"]
pub struct R(crate::R<SOCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOCCR` writer"]
pub struct W(crate::W<SOCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOCCR_SPEC>;
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
impl From<crate::W<SOCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "QSPI clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QSPISRC_A {
    #[doc = "0: Core/system clock"]
    _000 = 0,
    #[doc = "1: MCGFLL clock"]
    _001 = 1,
    #[doc = "2: MCGPLL clock"]
    _010 = 2,
    #[doc = "3: MCGPLL 2x clock (DDR mode specific)"]
    _011 = 3,
    #[doc = "4: IRC48M clock"]
    _100 = 4,
    #[doc = "5: OSCERCLK clock"]
    _101 = 5,
    #[doc = "6: MCGIRCLK clock"]
    _110 = 6,
}
impl From<QSPISRC_A> for u8 {
    #[inline(always)]
    fn from(variant: QSPISRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `QSPISRC` reader - QSPI clock source select"]
pub struct QSPISRC_R(crate::FieldReader<u8, QSPISRC_A>);
impl QSPISRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        QSPISRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<QSPISRC_A> {
        match self.bits {
            0 => Some(QSPISRC_A::_000),
            1 => Some(QSPISRC_A::_001),
            2 => Some(QSPISRC_A::_010),
            3 => Some(QSPISRC_A::_011),
            4 => Some(QSPISRC_A::_100),
            5 => Some(QSPISRC_A::_101),
            6 => Some(QSPISRC_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == QSPISRC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == QSPISRC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == QSPISRC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == QSPISRC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == QSPISRC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == QSPISRC_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == QSPISRC_A::_110
    }
}
impl core::ops::Deref for QSPISRC_R {
    type Target = crate::FieldReader<u8, QSPISRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPISRC` writer - QSPI clock source select"]
pub struct QSPISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPISRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPISRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Core/system clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(QSPISRC_A::_000)
    }
    #[doc = "MCGFLL clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(QSPISRC_A::_001)
    }
    #[doc = "MCGPLL clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(QSPISRC_A::_010)
    }
    #[doc = "MCGPLL 2x clock (DDR mode specific)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(QSPISRC_A::_011)
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(QSPISRC_A::_100)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(QSPISRC_A::_101)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(QSPISRC_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "When this bit is set the internal generated DQS is selected and looped back to QuadSPI, without going to DQS pad. DQSPADLPEN should be cleared when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSLPEN_A {
    #[doc = "0: DQS loop back is disabled"]
    _0 = 0,
    #[doc = "1: DQS loop back is enabled"]
    _1 = 1,
}
impl From<DQSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DQSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DQSLPEN` reader - When this bit is set the internal generated DQS is selected and looped back to QuadSPI, without going to DQS pad. DQSPADLPEN should be cleared when this bit is set."]
pub struct DQSLPEN_R(crate::FieldReader<bool, DQSLPEN_A>);
impl DQSLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSLPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQSLPEN_A {
        match self.bits {
            false => DQSLPEN_A::_0,
            true => DQSLPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DQSLPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DQSLPEN_A::_1
    }
}
impl core::ops::Deref for DQSLPEN_R {
    type Target = crate::FieldReader<bool, DQSLPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSLPEN` writer - When this bit is set the internal generated DQS is selected and looped back to QuadSPI, without going to DQS pad. DQSPADLPEN should be cleared when this bit is set."]
pub struct DQSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQSLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DQS loop back is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQSLPEN_A::_0)
    }
    #[doc = "DQS loop back is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQSLPEN_A::_1)
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
#[doc = "When this bit is set the internal generated DQS will be sent to the DQS pad first and then looped back to QuadSPI. DQSLPEN should be cleared when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSPADLPEN_A {
    #[doc = "0: DQS loop back from DQS pad is disabled"]
    _0 = 0,
    #[doc = "1: DQS loop back from DQS pad is enabled"]
    _1 = 1,
}
impl From<DQSPADLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DQSPADLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DQSPADLPEN` reader - When this bit is set the internal generated DQS will be sent to the DQS pad first and then looped back to QuadSPI. DQSLPEN should be cleared when this bit is set."]
pub struct DQSPADLPEN_R(crate::FieldReader<bool, DQSPADLPEN_A>);
impl DQSPADLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSPADLPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQSPADLPEN_A {
        match self.bits {
            false => DQSPADLPEN_A::_0,
            true => DQSPADLPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DQSPADLPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DQSPADLPEN_A::_1
    }
}
impl core::ops::Deref for DQSPADLPEN_R {
    type Target = crate::FieldReader<bool, DQSPADLPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSPADLPEN` writer - When this bit is set the internal generated DQS will be sent to the DQS pad first and then looped back to QuadSPI. DQSLPEN should be cleared when this bit is set."]
pub struct DQSPADLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSPADLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQSPADLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DQS loop back from DQS pad is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQSPADLPEN_A::_0)
    }
    #[doc = "DQS loop back from DQS pad is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQSPADLPEN_A::_1)
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
#[doc = "Select phase shift for internal DQS generation. These bits are always zero in SDR mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DQSPHASEL_A {
    #[doc = "0: No phase shift"]
    _00 = 0,
    #[doc = "1: Select 45 degree phase shift"]
    _01 = 1,
    #[doc = "2: Select 90 degree phase shift"]
    _10 = 2,
    #[doc = "3: Select 135 degree phase shift"]
    _11 = 3,
}
impl From<DQSPHASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DQSPHASEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DQSPHASEL` reader - Select phase shift for internal DQS generation. These bits are always zero in SDR mode."]
pub struct DQSPHASEL_R(crate::FieldReader<u8, DQSPHASEL_A>);
impl DQSPHASEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQSPHASEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQSPHASEL_A {
        match self.bits {
            0 => DQSPHASEL_A::_00,
            1 => DQSPHASEL_A::_01,
            2 => DQSPHASEL_A::_10,
            3 => DQSPHASEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == DQSPHASEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == DQSPHASEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == DQSPHASEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == DQSPHASEL_A::_11
    }
}
impl core::ops::Deref for DQSPHASEL_R {
    type Target = crate::FieldReader<u8, DQSPHASEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSPHASEL` writer - Select phase shift for internal DQS generation. These bits are always zero in SDR mode."]
pub struct DQSPHASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSPHASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQSPHASEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No phase shift"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DQSPHASEL_A::_00)
    }
    #[doc = "Select 45 degree phase shift"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DQSPHASEL_A::_01)
    }
    #[doc = "Select 90 degree phase shift"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DQSPHASEL_A::_10)
    }
    #[doc = "Select 135 degree phase shift"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DQSPHASEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Select clock source for internal DQS generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSINVSEL_A {
    #[doc = "0: Use 1x internal reference clock for the DQS generation"]
    _0 = 0,
    #[doc = "1: Use inverse 1x internal reference clock for the DQS generation"]
    _1 = 1,
}
impl From<DQSINVSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DQSINVSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DQSINVSEL` reader - Select clock source for internal DQS generation"]
pub struct DQSINVSEL_R(crate::FieldReader<bool, DQSINVSEL_A>);
impl DQSINVSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSINVSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQSINVSEL_A {
        match self.bits {
            false => DQSINVSEL_A::_0,
            true => DQSINVSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DQSINVSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DQSINVSEL_A::_1
    }
}
impl core::ops::Deref for DQSINVSEL_R {
    type Target = crate::FieldReader<bool, DQSINVSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSINVSEL` writer - Select clock source for internal DQS generation"]
pub struct DQSINVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSINVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQSINVSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use 1x internal reference clock for the DQS generation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQSINVSEL_A::_0)
    }
    #[doc = "Use inverse 1x internal reference clock for the DQS generation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQSINVSEL_A::_1)
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
#[doc = "Flash CK2 clock pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CK2EN_A {
    #[doc = "0: CK2 flash clock is disabled"]
    _0 = 0,
    #[doc = "1: CK2 flash clock is enabled"]
    _1 = 1,
}
impl From<CK2EN_A> for bool {
    #[inline(always)]
    fn from(variant: CK2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CK2EN` reader - Flash CK2 clock pin enable"]
pub struct CK2EN_R(crate::FieldReader<bool, CK2EN_A>);
impl CK2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CK2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK2EN_A {
        match self.bits {
            false => CK2EN_A::_0,
            true => CK2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CK2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CK2EN_A::_1
    }
}
impl core::ops::Deref for CK2EN_R {
    type Target = crate::FieldReader<bool, CK2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK2EN` writer - Flash CK2 clock pin enable"]
pub struct CK2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CK2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CK2 flash clock is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CK2EN_A::_0)
    }
    #[doc = "CK2 flash clock is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CK2EN_A::_1)
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
#[doc = "Differential flash clock pins enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFCKEN_A {
    #[doc = "0: Differential flash clock is disabled"]
    _0 = 0,
    #[doc = "1: Differential flash clock is enabled"]
    _1 = 1,
}
impl From<DIFFCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIFFCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFFCKEN` reader - Differential flash clock pins enable"]
pub struct DIFFCKEN_R(crate::FieldReader<bool, DIFFCKEN_A>);
impl DIFFCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIFFCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFFCKEN_A {
        match self.bits {
            false => DIFFCKEN_A::_0,
            true => DIFFCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIFFCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIFFCKEN_A::_1
    }
}
impl core::ops::Deref for DIFFCKEN_R {
    type Target = crate::FieldReader<bool, DIFFCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFFCKEN` writer - Differential flash clock pins enable"]
pub struct DIFFCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFFCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Differential flash clock is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIFFCKEN_A::_0)
    }
    #[doc = "Differential flash clock is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIFFCKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Octal data pins enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCTEN_A {
    #[doc = "0: QSPI0B_DATAx pins are assigned to QSPI Port B"]
    _0 = 0,
    #[doc = "1: QSPI0B_DATAx pins are assigned to QSPI Port A"]
    _1 = 1,
}
impl From<OCTEN_A> for bool {
    #[inline(always)]
    fn from(variant: OCTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTEN` reader - Octal data pins enable"]
pub struct OCTEN_R(crate::FieldReader<bool, OCTEN_A>);
impl OCTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTEN_A {
        match self.bits {
            false => OCTEN_A::_0,
            true => OCTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OCTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OCTEN_A::_1
    }
}
impl core::ops::Deref for OCTEN_R {
    type Target = crate::FieldReader<bool, OCTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCTEN` writer - Octal data pins enable"]
pub struct OCTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "QSPI0B_DATAx pins are assigned to QSPI Port B"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCTEN_A::_0)
    }
    #[doc = "QSPI0B_DATAx pins are assigned to QSPI Port A"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Delay chain tap number selection for QSPI Port A DQS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DLYTAPSELA_A {
    #[doc = "0: Select 1 delay chain tap"]
    _000000 = 0,
    #[doc = "1: Select 2 delay chain tap"]
    _000001 = 1,
    #[doc = "2: Select 3 delay chain tap"]
    _0000010 = 2,
    #[doc = "3: Select 4 delay chain tap"]
    _0000011 = 3,
    #[doc = "4: Select 5 delay chain tap"]
    _00000100 = 4,
    #[doc = "5: Select 6 delay chain tap"]
    _00000101 = 5,
    #[doc = "6: Select 7 delay chain tap"]
    _00000110 = 6,
    #[doc = "7: Select 8 delay chain tap"]
    _00000111 = 7,
    #[doc = "8: Select 9 delay chain tap"]
    _000001000 = 8,
    #[doc = "9: Select 10 delay chain tap"]
    _000001001 = 9,
    #[doc = "10: Select 11 delay chain tap"]
    _000001010 = 10,
    #[doc = "11: Select 12 delay chain tap"]
    _000001011 = 11,
    #[doc = "12: Select 13 delay chain tap"]
    _000001100 = 12,
    #[doc = "13: Select 14 delay chain tap"]
    _000001101 = 13,
    #[doc = "14: Select 15 delay chain tap"]
    _000001110 = 14,
    #[doc = "15: Select 16 delay chain tap"]
    _000001111 = 15,
    #[doc = "16: Select 17 delay chain tap"]
    _0000010000 = 16,
    #[doc = "17: Select 18 delay chain tap"]
    _0000010001 = 17,
    #[doc = "18: Select 19 delay chain tap"]
    _0000010010 = 18,
    #[doc = "19: Select 20 delay chain tap"]
    _0000010011 = 19,
    #[doc = "20: Select 21 delay chain tap"]
    _0000010100 = 20,
    #[doc = "21: Select 22 delay chain tap"]
    _0000010101 = 21,
    #[doc = "22: Select 23 delay chain tap"]
    _0000010110 = 22,
    #[doc = "23: Select 24 delay chain tap"]
    _0000010111 = 23,
    #[doc = "24: Select 25 delay chain tap"]
    _0000011000 = 24,
    #[doc = "25: Select 26 delay chain tap"]
    _0000011001 = 25,
    #[doc = "26: Select 27 delay chain tap"]
    _0000011010 = 26,
    #[doc = "27: Select 28 delay chain tap"]
    _0000011011 = 27,
    #[doc = "28: Select 29 delay chain tap"]
    _0000011100 = 28,
    #[doc = "29: Select 30 delay chain tap"]
    _0000011101 = 29,
    #[doc = "30: Select 31 delay chain tap"]
    _0000011110 = 30,
    #[doc = "31: Select 32 delay chain tap"]
    _0000011111 = 31,
    #[doc = "32: Select 33 delay chain tap"]
    _00000100000 = 32,
    #[doc = "33: Select 34 delay chain tap"]
    _00000100001 = 33,
    #[doc = "34: Select 35 delay chain tap"]
    _00000100010 = 34,
    #[doc = "35: Select 36 delay chain tap"]
    _00000100011 = 35,
    #[doc = "36: Select 37 delay chain tap"]
    _00000100100 = 36,
    #[doc = "37: Select 38 delay chain tap"]
    _00000100101 = 37,
    #[doc = "38: Select 39 delay chain tap"]
    _00000100110 = 38,
    #[doc = "39: Select 40 delay chain tap"]
    _00000100111 = 39,
    #[doc = "40: Select 41 delay chain tap"]
    _00000101000 = 40,
    #[doc = "41: Select 42 delay chain tap"]
    _00000101001 = 41,
    #[doc = "42: Select 43 delay chain tap"]
    _00000101010 = 42,
    #[doc = "43: Select 44 delay chain tap"]
    _00000101011 = 43,
    #[doc = "44: Select 45 delay chain tap"]
    _00000101100 = 44,
    #[doc = "45: Select 46 delay chain tap"]
    _00000101101 = 45,
    #[doc = "46: Select 47 delay chain tap"]
    _00000101110 = 46,
    #[doc = "47: Select 48 delay chain tap"]
    _00000101111 = 47,
    #[doc = "48: Select 49 delay chain tap"]
    _00000110000 = 48,
    #[doc = "49: Select 50 delay chain tap"]
    _00000110001 = 49,
    #[doc = "50: Select 51 delay chain tap"]
    _00000110010 = 50,
    #[doc = "51: Select 52 delay chain tap"]
    _00000110011 = 51,
    #[doc = "52: Select 53 delay chain tap"]
    _00000110100 = 52,
    #[doc = "53: Select 54 delay chain tap"]
    _00000110101 = 53,
    #[doc = "54: Select 55 delay chain tap"]
    _00000110110 = 54,
    #[doc = "55: Select 56 delay chain tap"]
    _00000110111 = 55,
    #[doc = "56: Select 57 delay chain tap"]
    _00000111000 = 56,
    #[doc = "57: Select 58 delay chain tap"]
    _00000111001 = 57,
    #[doc = "58: Select 59 delay chain tap"]
    _00000111010 = 58,
    #[doc = "59: Select 60 delay chain tap"]
    _00000111011 = 59,
    #[doc = "60: Select 61 delay chain tap"]
    _00000111100 = 60,
    #[doc = "61: Select 62 delay chain tap"]
    _00000111101 = 61,
    #[doc = "62: Select 63 delay chain tap"]
    _00000111110 = 62,
    #[doc = "63: Select 64 delay chain tap"]
    _00000111111 = 63,
}
impl From<DLYTAPSELA_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYTAPSELA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DLYTAPSELA` reader - Delay chain tap number selection for QSPI Port A DQS"]
pub struct DLYTAPSELA_R(crate::FieldReader<u8, DLYTAPSELA_A>);
impl DLYTAPSELA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYTAPSELA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYTAPSELA_A {
        match self.bits {
            0 => DLYTAPSELA_A::_000000,
            1 => DLYTAPSELA_A::_000001,
            2 => DLYTAPSELA_A::_0000010,
            3 => DLYTAPSELA_A::_0000011,
            4 => DLYTAPSELA_A::_00000100,
            5 => DLYTAPSELA_A::_00000101,
            6 => DLYTAPSELA_A::_00000110,
            7 => DLYTAPSELA_A::_00000111,
            8 => DLYTAPSELA_A::_000001000,
            9 => DLYTAPSELA_A::_000001001,
            10 => DLYTAPSELA_A::_000001010,
            11 => DLYTAPSELA_A::_000001011,
            12 => DLYTAPSELA_A::_000001100,
            13 => DLYTAPSELA_A::_000001101,
            14 => DLYTAPSELA_A::_000001110,
            15 => DLYTAPSELA_A::_000001111,
            16 => DLYTAPSELA_A::_0000010000,
            17 => DLYTAPSELA_A::_0000010001,
            18 => DLYTAPSELA_A::_0000010010,
            19 => DLYTAPSELA_A::_0000010011,
            20 => DLYTAPSELA_A::_0000010100,
            21 => DLYTAPSELA_A::_0000010101,
            22 => DLYTAPSELA_A::_0000010110,
            23 => DLYTAPSELA_A::_0000010111,
            24 => DLYTAPSELA_A::_0000011000,
            25 => DLYTAPSELA_A::_0000011001,
            26 => DLYTAPSELA_A::_0000011010,
            27 => DLYTAPSELA_A::_0000011011,
            28 => DLYTAPSELA_A::_0000011100,
            29 => DLYTAPSELA_A::_0000011101,
            30 => DLYTAPSELA_A::_0000011110,
            31 => DLYTAPSELA_A::_0000011111,
            32 => DLYTAPSELA_A::_00000100000,
            33 => DLYTAPSELA_A::_00000100001,
            34 => DLYTAPSELA_A::_00000100010,
            35 => DLYTAPSELA_A::_00000100011,
            36 => DLYTAPSELA_A::_00000100100,
            37 => DLYTAPSELA_A::_00000100101,
            38 => DLYTAPSELA_A::_00000100110,
            39 => DLYTAPSELA_A::_00000100111,
            40 => DLYTAPSELA_A::_00000101000,
            41 => DLYTAPSELA_A::_00000101001,
            42 => DLYTAPSELA_A::_00000101010,
            43 => DLYTAPSELA_A::_00000101011,
            44 => DLYTAPSELA_A::_00000101100,
            45 => DLYTAPSELA_A::_00000101101,
            46 => DLYTAPSELA_A::_00000101110,
            47 => DLYTAPSELA_A::_00000101111,
            48 => DLYTAPSELA_A::_00000110000,
            49 => DLYTAPSELA_A::_00000110001,
            50 => DLYTAPSELA_A::_00000110010,
            51 => DLYTAPSELA_A::_00000110011,
            52 => DLYTAPSELA_A::_00000110100,
            53 => DLYTAPSELA_A::_00000110101,
            54 => DLYTAPSELA_A::_00000110110,
            55 => DLYTAPSELA_A::_00000110111,
            56 => DLYTAPSELA_A::_00000111000,
            57 => DLYTAPSELA_A::_00000111001,
            58 => DLYTAPSELA_A::_00000111010,
            59 => DLYTAPSELA_A::_00000111011,
            60 => DLYTAPSELA_A::_00000111100,
            61 => DLYTAPSELA_A::_00000111101,
            62 => DLYTAPSELA_A::_00000111110,
            63 => DLYTAPSELA_A::_00000111111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        **self == DLYTAPSELA_A::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        **self == DLYTAPSELA_A::_000001
    }
    #[doc = "Checks if the value of the field is `_0000010`"]
    #[inline(always)]
    pub fn is_0000010(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010
    }
    #[doc = "Checks if the value of the field is `_0000011`"]
    #[inline(always)]
    pub fn is_0000011(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011
    }
    #[doc = "Checks if the value of the field is `_00000100`"]
    #[inline(always)]
    pub fn is_00000100(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100
    }
    #[doc = "Checks if the value of the field is `_00000101`"]
    #[inline(always)]
    pub fn is_00000101(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101
    }
    #[doc = "Checks if the value of the field is `_00000110`"]
    #[inline(always)]
    pub fn is_00000110(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110
    }
    #[doc = "Checks if the value of the field is `_00000111`"]
    #[inline(always)]
    pub fn is_00000111(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111
    }
    #[doc = "Checks if the value of the field is `_000001000`"]
    #[inline(always)]
    pub fn is_000001000(&self) -> bool {
        **self == DLYTAPSELA_A::_000001000
    }
    #[doc = "Checks if the value of the field is `_000001001`"]
    #[inline(always)]
    pub fn is_000001001(&self) -> bool {
        **self == DLYTAPSELA_A::_000001001
    }
    #[doc = "Checks if the value of the field is `_000001010`"]
    #[inline(always)]
    pub fn is_000001010(&self) -> bool {
        **self == DLYTAPSELA_A::_000001010
    }
    #[doc = "Checks if the value of the field is `_000001011`"]
    #[inline(always)]
    pub fn is_000001011(&self) -> bool {
        **self == DLYTAPSELA_A::_000001011
    }
    #[doc = "Checks if the value of the field is `_000001100`"]
    #[inline(always)]
    pub fn is_000001100(&self) -> bool {
        **self == DLYTAPSELA_A::_000001100
    }
    #[doc = "Checks if the value of the field is `_000001101`"]
    #[inline(always)]
    pub fn is_000001101(&self) -> bool {
        **self == DLYTAPSELA_A::_000001101
    }
    #[doc = "Checks if the value of the field is `_000001110`"]
    #[inline(always)]
    pub fn is_000001110(&self) -> bool {
        **self == DLYTAPSELA_A::_000001110
    }
    #[doc = "Checks if the value of the field is `_000001111`"]
    #[inline(always)]
    pub fn is_000001111(&self) -> bool {
        **self == DLYTAPSELA_A::_000001111
    }
    #[doc = "Checks if the value of the field is `_0000010000`"]
    #[inline(always)]
    pub fn is_0000010000(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010000
    }
    #[doc = "Checks if the value of the field is `_0000010001`"]
    #[inline(always)]
    pub fn is_0000010001(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010001
    }
    #[doc = "Checks if the value of the field is `_0000010010`"]
    #[inline(always)]
    pub fn is_0000010010(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010010
    }
    #[doc = "Checks if the value of the field is `_0000010011`"]
    #[inline(always)]
    pub fn is_0000010011(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010011
    }
    #[doc = "Checks if the value of the field is `_0000010100`"]
    #[inline(always)]
    pub fn is_0000010100(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010100
    }
    #[doc = "Checks if the value of the field is `_0000010101`"]
    #[inline(always)]
    pub fn is_0000010101(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010101
    }
    #[doc = "Checks if the value of the field is `_0000010110`"]
    #[inline(always)]
    pub fn is_0000010110(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010110
    }
    #[doc = "Checks if the value of the field is `_0000010111`"]
    #[inline(always)]
    pub fn is_0000010111(&self) -> bool {
        **self == DLYTAPSELA_A::_0000010111
    }
    #[doc = "Checks if the value of the field is `_0000011000`"]
    #[inline(always)]
    pub fn is_0000011000(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011000
    }
    #[doc = "Checks if the value of the field is `_0000011001`"]
    #[inline(always)]
    pub fn is_0000011001(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011001
    }
    #[doc = "Checks if the value of the field is `_0000011010`"]
    #[inline(always)]
    pub fn is_0000011010(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011010
    }
    #[doc = "Checks if the value of the field is `_0000011011`"]
    #[inline(always)]
    pub fn is_0000011011(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011011
    }
    #[doc = "Checks if the value of the field is `_0000011100`"]
    #[inline(always)]
    pub fn is_0000011100(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011100
    }
    #[doc = "Checks if the value of the field is `_0000011101`"]
    #[inline(always)]
    pub fn is_0000011101(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011101
    }
    #[doc = "Checks if the value of the field is `_0000011110`"]
    #[inline(always)]
    pub fn is_0000011110(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011110
    }
    #[doc = "Checks if the value of the field is `_0000011111`"]
    #[inline(always)]
    pub fn is_0000011111(&self) -> bool {
        **self == DLYTAPSELA_A::_0000011111
    }
    #[doc = "Checks if the value of the field is `_00000100000`"]
    #[inline(always)]
    pub fn is_00000100000(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100000
    }
    #[doc = "Checks if the value of the field is `_00000100001`"]
    #[inline(always)]
    pub fn is_00000100001(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100001
    }
    #[doc = "Checks if the value of the field is `_00000100010`"]
    #[inline(always)]
    pub fn is_00000100010(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100010
    }
    #[doc = "Checks if the value of the field is `_00000100011`"]
    #[inline(always)]
    pub fn is_00000100011(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100011
    }
    #[doc = "Checks if the value of the field is `_00000100100`"]
    #[inline(always)]
    pub fn is_00000100100(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100100
    }
    #[doc = "Checks if the value of the field is `_00000100101`"]
    #[inline(always)]
    pub fn is_00000100101(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100101
    }
    #[doc = "Checks if the value of the field is `_00000100110`"]
    #[inline(always)]
    pub fn is_00000100110(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100110
    }
    #[doc = "Checks if the value of the field is `_00000100111`"]
    #[inline(always)]
    pub fn is_00000100111(&self) -> bool {
        **self == DLYTAPSELA_A::_00000100111
    }
    #[doc = "Checks if the value of the field is `_00000101000`"]
    #[inline(always)]
    pub fn is_00000101000(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101000
    }
    #[doc = "Checks if the value of the field is `_00000101001`"]
    #[inline(always)]
    pub fn is_00000101001(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101001
    }
    #[doc = "Checks if the value of the field is `_00000101010`"]
    #[inline(always)]
    pub fn is_00000101010(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101010
    }
    #[doc = "Checks if the value of the field is `_00000101011`"]
    #[inline(always)]
    pub fn is_00000101011(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101011
    }
    #[doc = "Checks if the value of the field is `_00000101100`"]
    #[inline(always)]
    pub fn is_00000101100(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101100
    }
    #[doc = "Checks if the value of the field is `_00000101101`"]
    #[inline(always)]
    pub fn is_00000101101(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101101
    }
    #[doc = "Checks if the value of the field is `_00000101110`"]
    #[inline(always)]
    pub fn is_00000101110(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101110
    }
    #[doc = "Checks if the value of the field is `_00000101111`"]
    #[inline(always)]
    pub fn is_00000101111(&self) -> bool {
        **self == DLYTAPSELA_A::_00000101111
    }
    #[doc = "Checks if the value of the field is `_00000110000`"]
    #[inline(always)]
    pub fn is_00000110000(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110000
    }
    #[doc = "Checks if the value of the field is `_00000110001`"]
    #[inline(always)]
    pub fn is_00000110001(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110001
    }
    #[doc = "Checks if the value of the field is `_00000110010`"]
    #[inline(always)]
    pub fn is_00000110010(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110010
    }
    #[doc = "Checks if the value of the field is `_00000110011`"]
    #[inline(always)]
    pub fn is_00000110011(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110011
    }
    #[doc = "Checks if the value of the field is `_00000110100`"]
    #[inline(always)]
    pub fn is_00000110100(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110100
    }
    #[doc = "Checks if the value of the field is `_00000110101`"]
    #[inline(always)]
    pub fn is_00000110101(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110101
    }
    #[doc = "Checks if the value of the field is `_00000110110`"]
    #[inline(always)]
    pub fn is_00000110110(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110110
    }
    #[doc = "Checks if the value of the field is `_00000110111`"]
    #[inline(always)]
    pub fn is_00000110111(&self) -> bool {
        **self == DLYTAPSELA_A::_00000110111
    }
    #[doc = "Checks if the value of the field is `_00000111000`"]
    #[inline(always)]
    pub fn is_00000111000(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111000
    }
    #[doc = "Checks if the value of the field is `_00000111001`"]
    #[inline(always)]
    pub fn is_00000111001(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111001
    }
    #[doc = "Checks if the value of the field is `_00000111010`"]
    #[inline(always)]
    pub fn is_00000111010(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111010
    }
    #[doc = "Checks if the value of the field is `_00000111011`"]
    #[inline(always)]
    pub fn is_00000111011(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111011
    }
    #[doc = "Checks if the value of the field is `_00000111100`"]
    #[inline(always)]
    pub fn is_00000111100(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111100
    }
    #[doc = "Checks if the value of the field is `_00000111101`"]
    #[inline(always)]
    pub fn is_00000111101(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111101
    }
    #[doc = "Checks if the value of the field is `_00000111110`"]
    #[inline(always)]
    pub fn is_00000111110(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111110
    }
    #[doc = "Checks if the value of the field is `_00000111111`"]
    #[inline(always)]
    pub fn is_00000111111(&self) -> bool {
        **self == DLYTAPSELA_A::_00000111111
    }
}
impl core::ops::Deref for DLYTAPSELA_R {
    type Target = crate::FieldReader<u8, DLYTAPSELA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYTAPSELA` writer - Delay chain tap number selection for QSPI Port A DQS"]
pub struct DLYTAPSELA_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYTAPSELA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLYTAPSELA_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Select 1 delay chain tap"]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000000)
    }
    #[doc = "Select 2 delay chain tap"]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001)
    }
    #[doc = "Select 3 delay chain tap"]
    #[inline(always)]
    pub fn _0000010(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010)
    }
    #[doc = "Select 4 delay chain tap"]
    #[inline(always)]
    pub fn _0000011(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011)
    }
    #[doc = "Select 5 delay chain tap"]
    #[inline(always)]
    pub fn _00000100(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100)
    }
    #[doc = "Select 6 delay chain tap"]
    #[inline(always)]
    pub fn _00000101(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101)
    }
    #[doc = "Select 7 delay chain tap"]
    #[inline(always)]
    pub fn _00000110(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110)
    }
    #[doc = "Select 8 delay chain tap"]
    #[inline(always)]
    pub fn _00000111(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111)
    }
    #[doc = "Select 9 delay chain tap"]
    #[inline(always)]
    pub fn _000001000(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001000)
    }
    #[doc = "Select 10 delay chain tap"]
    #[inline(always)]
    pub fn _000001001(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001001)
    }
    #[doc = "Select 11 delay chain tap"]
    #[inline(always)]
    pub fn _000001010(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001010)
    }
    #[doc = "Select 12 delay chain tap"]
    #[inline(always)]
    pub fn _000001011(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001011)
    }
    #[doc = "Select 13 delay chain tap"]
    #[inline(always)]
    pub fn _000001100(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001100)
    }
    #[doc = "Select 14 delay chain tap"]
    #[inline(always)]
    pub fn _000001101(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001101)
    }
    #[doc = "Select 15 delay chain tap"]
    #[inline(always)]
    pub fn _000001110(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001110)
    }
    #[doc = "Select 16 delay chain tap"]
    #[inline(always)]
    pub fn _000001111(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_000001111)
    }
    #[doc = "Select 17 delay chain tap"]
    #[inline(always)]
    pub fn _0000010000(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010000)
    }
    #[doc = "Select 18 delay chain tap"]
    #[inline(always)]
    pub fn _0000010001(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010001)
    }
    #[doc = "Select 19 delay chain tap"]
    #[inline(always)]
    pub fn _0000010010(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010010)
    }
    #[doc = "Select 20 delay chain tap"]
    #[inline(always)]
    pub fn _0000010011(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010011)
    }
    #[doc = "Select 21 delay chain tap"]
    #[inline(always)]
    pub fn _0000010100(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010100)
    }
    #[doc = "Select 22 delay chain tap"]
    #[inline(always)]
    pub fn _0000010101(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010101)
    }
    #[doc = "Select 23 delay chain tap"]
    #[inline(always)]
    pub fn _0000010110(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010110)
    }
    #[doc = "Select 24 delay chain tap"]
    #[inline(always)]
    pub fn _0000010111(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000010111)
    }
    #[doc = "Select 25 delay chain tap"]
    #[inline(always)]
    pub fn _0000011000(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011000)
    }
    #[doc = "Select 26 delay chain tap"]
    #[inline(always)]
    pub fn _0000011001(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011001)
    }
    #[doc = "Select 27 delay chain tap"]
    #[inline(always)]
    pub fn _0000011010(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011010)
    }
    #[doc = "Select 28 delay chain tap"]
    #[inline(always)]
    pub fn _0000011011(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011011)
    }
    #[doc = "Select 29 delay chain tap"]
    #[inline(always)]
    pub fn _0000011100(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011100)
    }
    #[doc = "Select 30 delay chain tap"]
    #[inline(always)]
    pub fn _0000011101(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011101)
    }
    #[doc = "Select 31 delay chain tap"]
    #[inline(always)]
    pub fn _0000011110(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011110)
    }
    #[doc = "Select 32 delay chain tap"]
    #[inline(always)]
    pub fn _0000011111(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_0000011111)
    }
    #[doc = "Select 33 delay chain tap"]
    #[inline(always)]
    pub fn _00000100000(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100000)
    }
    #[doc = "Select 34 delay chain tap"]
    #[inline(always)]
    pub fn _00000100001(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100001)
    }
    #[doc = "Select 35 delay chain tap"]
    #[inline(always)]
    pub fn _00000100010(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100010)
    }
    #[doc = "Select 36 delay chain tap"]
    #[inline(always)]
    pub fn _00000100011(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100011)
    }
    #[doc = "Select 37 delay chain tap"]
    #[inline(always)]
    pub fn _00000100100(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100100)
    }
    #[doc = "Select 38 delay chain tap"]
    #[inline(always)]
    pub fn _00000100101(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100101)
    }
    #[doc = "Select 39 delay chain tap"]
    #[inline(always)]
    pub fn _00000100110(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100110)
    }
    #[doc = "Select 40 delay chain tap"]
    #[inline(always)]
    pub fn _00000100111(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000100111)
    }
    #[doc = "Select 41 delay chain tap"]
    #[inline(always)]
    pub fn _00000101000(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101000)
    }
    #[doc = "Select 42 delay chain tap"]
    #[inline(always)]
    pub fn _00000101001(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101001)
    }
    #[doc = "Select 43 delay chain tap"]
    #[inline(always)]
    pub fn _00000101010(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101010)
    }
    #[doc = "Select 44 delay chain tap"]
    #[inline(always)]
    pub fn _00000101011(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101011)
    }
    #[doc = "Select 45 delay chain tap"]
    #[inline(always)]
    pub fn _00000101100(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101100)
    }
    #[doc = "Select 46 delay chain tap"]
    #[inline(always)]
    pub fn _00000101101(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101101)
    }
    #[doc = "Select 47 delay chain tap"]
    #[inline(always)]
    pub fn _00000101110(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101110)
    }
    #[doc = "Select 48 delay chain tap"]
    #[inline(always)]
    pub fn _00000101111(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000101111)
    }
    #[doc = "Select 49 delay chain tap"]
    #[inline(always)]
    pub fn _00000110000(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110000)
    }
    #[doc = "Select 50 delay chain tap"]
    #[inline(always)]
    pub fn _00000110001(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110001)
    }
    #[doc = "Select 51 delay chain tap"]
    #[inline(always)]
    pub fn _00000110010(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110010)
    }
    #[doc = "Select 52 delay chain tap"]
    #[inline(always)]
    pub fn _00000110011(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110011)
    }
    #[doc = "Select 53 delay chain tap"]
    #[inline(always)]
    pub fn _00000110100(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110100)
    }
    #[doc = "Select 54 delay chain tap"]
    #[inline(always)]
    pub fn _00000110101(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110101)
    }
    #[doc = "Select 55 delay chain tap"]
    #[inline(always)]
    pub fn _00000110110(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110110)
    }
    #[doc = "Select 56 delay chain tap"]
    #[inline(always)]
    pub fn _00000110111(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000110111)
    }
    #[doc = "Select 57 delay chain tap"]
    #[inline(always)]
    pub fn _00000111000(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111000)
    }
    #[doc = "Select 58 delay chain tap"]
    #[inline(always)]
    pub fn _00000111001(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111001)
    }
    #[doc = "Select 59 delay chain tap"]
    #[inline(always)]
    pub fn _00000111010(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111010)
    }
    #[doc = "Select 60 delay chain tap"]
    #[inline(always)]
    pub fn _00000111011(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111011)
    }
    #[doc = "Select 61 delay chain tap"]
    #[inline(always)]
    pub fn _00000111100(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111100)
    }
    #[doc = "Select 62 delay chain tap"]
    #[inline(always)]
    pub fn _00000111101(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111101)
    }
    #[doc = "Select 63 delay chain tap"]
    #[inline(always)]
    pub fn _00000111110(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111110)
    }
    #[doc = "Select 64 delay chain tap"]
    #[inline(always)]
    pub fn _00000111111(self) -> &'a mut W {
        self.variant(DLYTAPSELA_A::_00000111111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Delay chain tap number selection for QSPI Port B DQS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DLYTAPSELB_A {
    #[doc = "0: Select 1 delay chain tap"]
    _000000 = 0,
    #[doc = "1: Select 2 delay chain tap"]
    _000001 = 1,
    #[doc = "2: Select 3 delay chain tap"]
    _0000010 = 2,
    #[doc = "3: Select 4 delay chain tap"]
    _0000011 = 3,
    #[doc = "4: Select 5 delay chain tap"]
    _00000100 = 4,
    #[doc = "5: Select 6 delay chain tap"]
    _00000101 = 5,
    #[doc = "6: Select 7 delay chain tap"]
    _00000110 = 6,
    #[doc = "7: Select 8 delay chain tap"]
    _00000111 = 7,
    #[doc = "8: Select 9 delay chain tap"]
    _000001000 = 8,
    #[doc = "9: Select 10 delay chain tap"]
    _000001001 = 9,
    #[doc = "10: Select 11 delay chain tap"]
    _000001010 = 10,
    #[doc = "11: Select 12 delay chain tap"]
    _000001011 = 11,
    #[doc = "12: Select 13 delay chain tap"]
    _000001100 = 12,
    #[doc = "13: Select 14 delay chain tap"]
    _000001101 = 13,
    #[doc = "14: Select 15 delay chain tap"]
    _000001110 = 14,
    #[doc = "15: Select 16 delay chain tap"]
    _000001111 = 15,
    #[doc = "16: Select 17 delay chain tap"]
    _0000010000 = 16,
    #[doc = "17: Select 18 delay chain tap"]
    _0000010001 = 17,
    #[doc = "18: Select 19 delay chain tap"]
    _0000010010 = 18,
    #[doc = "19: Select 20 delay chain tap"]
    _0000010011 = 19,
    #[doc = "20: Select 21 delay chain tap"]
    _0000010100 = 20,
    #[doc = "21: Select 22 delay chain tap"]
    _0000010101 = 21,
    #[doc = "22: Select 23 delay chain tap"]
    _0000010110 = 22,
    #[doc = "23: Select 24 delay chain tap"]
    _0000010111 = 23,
    #[doc = "24: Select 25 delay chain tap"]
    _0000011000 = 24,
    #[doc = "25: Select 26 delay chain tap"]
    _0000011001 = 25,
    #[doc = "26: Select 27 delay chain tap"]
    _0000011010 = 26,
    #[doc = "27: Select 28 delay chain tap"]
    _0000011011 = 27,
    #[doc = "28: Select 29 delay chain tap"]
    _0000011100 = 28,
    #[doc = "29: Select 30 delay chain tap"]
    _0000011101 = 29,
    #[doc = "30: Select 31 delay chain tap"]
    _0000011110 = 30,
    #[doc = "31: Select 32 delay chain tap"]
    _0000011111 = 31,
    #[doc = "32: Select 33 delay chain tap"]
    _00000100000 = 32,
    #[doc = "33: Select 34 delay chain tap"]
    _00000100001 = 33,
    #[doc = "34: Select 35 delay chain tap"]
    _00000100010 = 34,
    #[doc = "35: Select 36 delay chain tap"]
    _00000100011 = 35,
    #[doc = "36: Select 37 delay chain tap"]
    _00000100100 = 36,
    #[doc = "37: Select 38 delay chain tap"]
    _00000100101 = 37,
    #[doc = "38: Select 39 delay chain tap"]
    _00000100110 = 38,
    #[doc = "39: Select 40 delay chain tap"]
    _00000100111 = 39,
    #[doc = "40: Select 41 delay chain tap"]
    _00000101000 = 40,
    #[doc = "41: Select 42 delay chain tap"]
    _00000101001 = 41,
    #[doc = "42: Select 43 delay chain tap"]
    _00000101010 = 42,
    #[doc = "43: Select 44 delay chain tap"]
    _00000101011 = 43,
    #[doc = "44: Select 45 delay chain tap"]
    _00000101100 = 44,
    #[doc = "45: Select 46 delay chain tap"]
    _00000101101 = 45,
    #[doc = "46: Select 47 delay chain tap"]
    _00000101110 = 46,
    #[doc = "47: Select 48 delay chain tap"]
    _00000101111 = 47,
    #[doc = "48: Select 49 delay chain tap"]
    _00000110000 = 48,
    #[doc = "49: Select 50 delay chain tap"]
    _00000110001 = 49,
    #[doc = "50: Select 51 delay chain tap"]
    _00000110010 = 50,
    #[doc = "51: Select 52 delay chain tap"]
    _00000110011 = 51,
    #[doc = "52: Select 53 delay chain tap"]
    _00000110100 = 52,
    #[doc = "53: Select 54 delay chain tap"]
    _00000110101 = 53,
    #[doc = "54: Select 55 delay chain tap"]
    _00000110110 = 54,
    #[doc = "55: Select 56 delay chain tap"]
    _00000110111 = 55,
    #[doc = "56: Select 57 delay chain tap"]
    _00000111000 = 56,
    #[doc = "57: Select 58 delay chain tap"]
    _00000111001 = 57,
    #[doc = "58: Select 59 delay chain tap"]
    _00000111010 = 58,
    #[doc = "59: Select 60 delay chain tap"]
    _00000111011 = 59,
    #[doc = "60: Select 61 delay chain tap"]
    _00000111100 = 60,
    #[doc = "61: Select 62 delay chain tap"]
    _00000111101 = 61,
    #[doc = "62: Select 63 delay chain tap"]
    _00000111110 = 62,
    #[doc = "63: Select 64 delay chain tap"]
    _00000111111 = 63,
}
impl From<DLYTAPSELB_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYTAPSELB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DLYTAPSELB` reader - Delay chain tap number selection for QSPI Port B DQS"]
pub struct DLYTAPSELB_R(crate::FieldReader<u8, DLYTAPSELB_A>);
impl DLYTAPSELB_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYTAPSELB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYTAPSELB_A {
        match self.bits {
            0 => DLYTAPSELB_A::_000000,
            1 => DLYTAPSELB_A::_000001,
            2 => DLYTAPSELB_A::_0000010,
            3 => DLYTAPSELB_A::_0000011,
            4 => DLYTAPSELB_A::_00000100,
            5 => DLYTAPSELB_A::_00000101,
            6 => DLYTAPSELB_A::_00000110,
            7 => DLYTAPSELB_A::_00000111,
            8 => DLYTAPSELB_A::_000001000,
            9 => DLYTAPSELB_A::_000001001,
            10 => DLYTAPSELB_A::_000001010,
            11 => DLYTAPSELB_A::_000001011,
            12 => DLYTAPSELB_A::_000001100,
            13 => DLYTAPSELB_A::_000001101,
            14 => DLYTAPSELB_A::_000001110,
            15 => DLYTAPSELB_A::_000001111,
            16 => DLYTAPSELB_A::_0000010000,
            17 => DLYTAPSELB_A::_0000010001,
            18 => DLYTAPSELB_A::_0000010010,
            19 => DLYTAPSELB_A::_0000010011,
            20 => DLYTAPSELB_A::_0000010100,
            21 => DLYTAPSELB_A::_0000010101,
            22 => DLYTAPSELB_A::_0000010110,
            23 => DLYTAPSELB_A::_0000010111,
            24 => DLYTAPSELB_A::_0000011000,
            25 => DLYTAPSELB_A::_0000011001,
            26 => DLYTAPSELB_A::_0000011010,
            27 => DLYTAPSELB_A::_0000011011,
            28 => DLYTAPSELB_A::_0000011100,
            29 => DLYTAPSELB_A::_0000011101,
            30 => DLYTAPSELB_A::_0000011110,
            31 => DLYTAPSELB_A::_0000011111,
            32 => DLYTAPSELB_A::_00000100000,
            33 => DLYTAPSELB_A::_00000100001,
            34 => DLYTAPSELB_A::_00000100010,
            35 => DLYTAPSELB_A::_00000100011,
            36 => DLYTAPSELB_A::_00000100100,
            37 => DLYTAPSELB_A::_00000100101,
            38 => DLYTAPSELB_A::_00000100110,
            39 => DLYTAPSELB_A::_00000100111,
            40 => DLYTAPSELB_A::_00000101000,
            41 => DLYTAPSELB_A::_00000101001,
            42 => DLYTAPSELB_A::_00000101010,
            43 => DLYTAPSELB_A::_00000101011,
            44 => DLYTAPSELB_A::_00000101100,
            45 => DLYTAPSELB_A::_00000101101,
            46 => DLYTAPSELB_A::_00000101110,
            47 => DLYTAPSELB_A::_00000101111,
            48 => DLYTAPSELB_A::_00000110000,
            49 => DLYTAPSELB_A::_00000110001,
            50 => DLYTAPSELB_A::_00000110010,
            51 => DLYTAPSELB_A::_00000110011,
            52 => DLYTAPSELB_A::_00000110100,
            53 => DLYTAPSELB_A::_00000110101,
            54 => DLYTAPSELB_A::_00000110110,
            55 => DLYTAPSELB_A::_00000110111,
            56 => DLYTAPSELB_A::_00000111000,
            57 => DLYTAPSELB_A::_00000111001,
            58 => DLYTAPSELB_A::_00000111010,
            59 => DLYTAPSELB_A::_00000111011,
            60 => DLYTAPSELB_A::_00000111100,
            61 => DLYTAPSELB_A::_00000111101,
            62 => DLYTAPSELB_A::_00000111110,
            63 => DLYTAPSELB_A::_00000111111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        **self == DLYTAPSELB_A::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline(always)]
    pub fn is_000001(&self) -> bool {
        **self == DLYTAPSELB_A::_000001
    }
    #[doc = "Checks if the value of the field is `_0000010`"]
    #[inline(always)]
    pub fn is_0000010(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010
    }
    #[doc = "Checks if the value of the field is `_0000011`"]
    #[inline(always)]
    pub fn is_0000011(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011
    }
    #[doc = "Checks if the value of the field is `_00000100`"]
    #[inline(always)]
    pub fn is_00000100(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100
    }
    #[doc = "Checks if the value of the field is `_00000101`"]
    #[inline(always)]
    pub fn is_00000101(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101
    }
    #[doc = "Checks if the value of the field is `_00000110`"]
    #[inline(always)]
    pub fn is_00000110(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110
    }
    #[doc = "Checks if the value of the field is `_00000111`"]
    #[inline(always)]
    pub fn is_00000111(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111
    }
    #[doc = "Checks if the value of the field is `_000001000`"]
    #[inline(always)]
    pub fn is_000001000(&self) -> bool {
        **self == DLYTAPSELB_A::_000001000
    }
    #[doc = "Checks if the value of the field is `_000001001`"]
    #[inline(always)]
    pub fn is_000001001(&self) -> bool {
        **self == DLYTAPSELB_A::_000001001
    }
    #[doc = "Checks if the value of the field is `_000001010`"]
    #[inline(always)]
    pub fn is_000001010(&self) -> bool {
        **self == DLYTAPSELB_A::_000001010
    }
    #[doc = "Checks if the value of the field is `_000001011`"]
    #[inline(always)]
    pub fn is_000001011(&self) -> bool {
        **self == DLYTAPSELB_A::_000001011
    }
    #[doc = "Checks if the value of the field is `_000001100`"]
    #[inline(always)]
    pub fn is_000001100(&self) -> bool {
        **self == DLYTAPSELB_A::_000001100
    }
    #[doc = "Checks if the value of the field is `_000001101`"]
    #[inline(always)]
    pub fn is_000001101(&self) -> bool {
        **self == DLYTAPSELB_A::_000001101
    }
    #[doc = "Checks if the value of the field is `_000001110`"]
    #[inline(always)]
    pub fn is_000001110(&self) -> bool {
        **self == DLYTAPSELB_A::_000001110
    }
    #[doc = "Checks if the value of the field is `_000001111`"]
    #[inline(always)]
    pub fn is_000001111(&self) -> bool {
        **self == DLYTAPSELB_A::_000001111
    }
    #[doc = "Checks if the value of the field is `_0000010000`"]
    #[inline(always)]
    pub fn is_0000010000(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010000
    }
    #[doc = "Checks if the value of the field is `_0000010001`"]
    #[inline(always)]
    pub fn is_0000010001(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010001
    }
    #[doc = "Checks if the value of the field is `_0000010010`"]
    #[inline(always)]
    pub fn is_0000010010(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010010
    }
    #[doc = "Checks if the value of the field is `_0000010011`"]
    #[inline(always)]
    pub fn is_0000010011(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010011
    }
    #[doc = "Checks if the value of the field is `_0000010100`"]
    #[inline(always)]
    pub fn is_0000010100(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010100
    }
    #[doc = "Checks if the value of the field is `_0000010101`"]
    #[inline(always)]
    pub fn is_0000010101(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010101
    }
    #[doc = "Checks if the value of the field is `_0000010110`"]
    #[inline(always)]
    pub fn is_0000010110(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010110
    }
    #[doc = "Checks if the value of the field is `_0000010111`"]
    #[inline(always)]
    pub fn is_0000010111(&self) -> bool {
        **self == DLYTAPSELB_A::_0000010111
    }
    #[doc = "Checks if the value of the field is `_0000011000`"]
    #[inline(always)]
    pub fn is_0000011000(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011000
    }
    #[doc = "Checks if the value of the field is `_0000011001`"]
    #[inline(always)]
    pub fn is_0000011001(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011001
    }
    #[doc = "Checks if the value of the field is `_0000011010`"]
    #[inline(always)]
    pub fn is_0000011010(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011010
    }
    #[doc = "Checks if the value of the field is `_0000011011`"]
    #[inline(always)]
    pub fn is_0000011011(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011011
    }
    #[doc = "Checks if the value of the field is `_0000011100`"]
    #[inline(always)]
    pub fn is_0000011100(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011100
    }
    #[doc = "Checks if the value of the field is `_0000011101`"]
    #[inline(always)]
    pub fn is_0000011101(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011101
    }
    #[doc = "Checks if the value of the field is `_0000011110`"]
    #[inline(always)]
    pub fn is_0000011110(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011110
    }
    #[doc = "Checks if the value of the field is `_0000011111`"]
    #[inline(always)]
    pub fn is_0000011111(&self) -> bool {
        **self == DLYTAPSELB_A::_0000011111
    }
    #[doc = "Checks if the value of the field is `_00000100000`"]
    #[inline(always)]
    pub fn is_00000100000(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100000
    }
    #[doc = "Checks if the value of the field is `_00000100001`"]
    #[inline(always)]
    pub fn is_00000100001(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100001
    }
    #[doc = "Checks if the value of the field is `_00000100010`"]
    #[inline(always)]
    pub fn is_00000100010(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100010
    }
    #[doc = "Checks if the value of the field is `_00000100011`"]
    #[inline(always)]
    pub fn is_00000100011(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100011
    }
    #[doc = "Checks if the value of the field is `_00000100100`"]
    #[inline(always)]
    pub fn is_00000100100(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100100
    }
    #[doc = "Checks if the value of the field is `_00000100101`"]
    #[inline(always)]
    pub fn is_00000100101(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100101
    }
    #[doc = "Checks if the value of the field is `_00000100110`"]
    #[inline(always)]
    pub fn is_00000100110(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100110
    }
    #[doc = "Checks if the value of the field is `_00000100111`"]
    #[inline(always)]
    pub fn is_00000100111(&self) -> bool {
        **self == DLYTAPSELB_A::_00000100111
    }
    #[doc = "Checks if the value of the field is `_00000101000`"]
    #[inline(always)]
    pub fn is_00000101000(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101000
    }
    #[doc = "Checks if the value of the field is `_00000101001`"]
    #[inline(always)]
    pub fn is_00000101001(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101001
    }
    #[doc = "Checks if the value of the field is `_00000101010`"]
    #[inline(always)]
    pub fn is_00000101010(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101010
    }
    #[doc = "Checks if the value of the field is `_00000101011`"]
    #[inline(always)]
    pub fn is_00000101011(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101011
    }
    #[doc = "Checks if the value of the field is `_00000101100`"]
    #[inline(always)]
    pub fn is_00000101100(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101100
    }
    #[doc = "Checks if the value of the field is `_00000101101`"]
    #[inline(always)]
    pub fn is_00000101101(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101101
    }
    #[doc = "Checks if the value of the field is `_00000101110`"]
    #[inline(always)]
    pub fn is_00000101110(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101110
    }
    #[doc = "Checks if the value of the field is `_00000101111`"]
    #[inline(always)]
    pub fn is_00000101111(&self) -> bool {
        **self == DLYTAPSELB_A::_00000101111
    }
    #[doc = "Checks if the value of the field is `_00000110000`"]
    #[inline(always)]
    pub fn is_00000110000(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110000
    }
    #[doc = "Checks if the value of the field is `_00000110001`"]
    #[inline(always)]
    pub fn is_00000110001(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110001
    }
    #[doc = "Checks if the value of the field is `_00000110010`"]
    #[inline(always)]
    pub fn is_00000110010(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110010
    }
    #[doc = "Checks if the value of the field is `_00000110011`"]
    #[inline(always)]
    pub fn is_00000110011(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110011
    }
    #[doc = "Checks if the value of the field is `_00000110100`"]
    #[inline(always)]
    pub fn is_00000110100(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110100
    }
    #[doc = "Checks if the value of the field is `_00000110101`"]
    #[inline(always)]
    pub fn is_00000110101(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110101
    }
    #[doc = "Checks if the value of the field is `_00000110110`"]
    #[inline(always)]
    pub fn is_00000110110(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110110
    }
    #[doc = "Checks if the value of the field is `_00000110111`"]
    #[inline(always)]
    pub fn is_00000110111(&self) -> bool {
        **self == DLYTAPSELB_A::_00000110111
    }
    #[doc = "Checks if the value of the field is `_00000111000`"]
    #[inline(always)]
    pub fn is_00000111000(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111000
    }
    #[doc = "Checks if the value of the field is `_00000111001`"]
    #[inline(always)]
    pub fn is_00000111001(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111001
    }
    #[doc = "Checks if the value of the field is `_00000111010`"]
    #[inline(always)]
    pub fn is_00000111010(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111010
    }
    #[doc = "Checks if the value of the field is `_00000111011`"]
    #[inline(always)]
    pub fn is_00000111011(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111011
    }
    #[doc = "Checks if the value of the field is `_00000111100`"]
    #[inline(always)]
    pub fn is_00000111100(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111100
    }
    #[doc = "Checks if the value of the field is `_00000111101`"]
    #[inline(always)]
    pub fn is_00000111101(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111101
    }
    #[doc = "Checks if the value of the field is `_00000111110`"]
    #[inline(always)]
    pub fn is_00000111110(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111110
    }
    #[doc = "Checks if the value of the field is `_00000111111`"]
    #[inline(always)]
    pub fn is_00000111111(&self) -> bool {
        **self == DLYTAPSELB_A::_00000111111
    }
}
impl core::ops::Deref for DLYTAPSELB_R {
    type Target = crate::FieldReader<u8, DLYTAPSELB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYTAPSELB` writer - Delay chain tap number selection for QSPI Port B DQS"]
pub struct DLYTAPSELB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYTAPSELB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLYTAPSELB_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Select 1 delay chain tap"]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000000)
    }
    #[doc = "Select 2 delay chain tap"]
    #[inline(always)]
    pub fn _000001(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001)
    }
    #[doc = "Select 3 delay chain tap"]
    #[inline(always)]
    pub fn _0000010(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010)
    }
    #[doc = "Select 4 delay chain tap"]
    #[inline(always)]
    pub fn _0000011(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011)
    }
    #[doc = "Select 5 delay chain tap"]
    #[inline(always)]
    pub fn _00000100(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100)
    }
    #[doc = "Select 6 delay chain tap"]
    #[inline(always)]
    pub fn _00000101(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101)
    }
    #[doc = "Select 7 delay chain tap"]
    #[inline(always)]
    pub fn _00000110(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110)
    }
    #[doc = "Select 8 delay chain tap"]
    #[inline(always)]
    pub fn _00000111(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111)
    }
    #[doc = "Select 9 delay chain tap"]
    #[inline(always)]
    pub fn _000001000(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001000)
    }
    #[doc = "Select 10 delay chain tap"]
    #[inline(always)]
    pub fn _000001001(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001001)
    }
    #[doc = "Select 11 delay chain tap"]
    #[inline(always)]
    pub fn _000001010(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001010)
    }
    #[doc = "Select 12 delay chain tap"]
    #[inline(always)]
    pub fn _000001011(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001011)
    }
    #[doc = "Select 13 delay chain tap"]
    #[inline(always)]
    pub fn _000001100(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001100)
    }
    #[doc = "Select 14 delay chain tap"]
    #[inline(always)]
    pub fn _000001101(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001101)
    }
    #[doc = "Select 15 delay chain tap"]
    #[inline(always)]
    pub fn _000001110(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001110)
    }
    #[doc = "Select 16 delay chain tap"]
    #[inline(always)]
    pub fn _000001111(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_000001111)
    }
    #[doc = "Select 17 delay chain tap"]
    #[inline(always)]
    pub fn _0000010000(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010000)
    }
    #[doc = "Select 18 delay chain tap"]
    #[inline(always)]
    pub fn _0000010001(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010001)
    }
    #[doc = "Select 19 delay chain tap"]
    #[inline(always)]
    pub fn _0000010010(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010010)
    }
    #[doc = "Select 20 delay chain tap"]
    #[inline(always)]
    pub fn _0000010011(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010011)
    }
    #[doc = "Select 21 delay chain tap"]
    #[inline(always)]
    pub fn _0000010100(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010100)
    }
    #[doc = "Select 22 delay chain tap"]
    #[inline(always)]
    pub fn _0000010101(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010101)
    }
    #[doc = "Select 23 delay chain tap"]
    #[inline(always)]
    pub fn _0000010110(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010110)
    }
    #[doc = "Select 24 delay chain tap"]
    #[inline(always)]
    pub fn _0000010111(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000010111)
    }
    #[doc = "Select 25 delay chain tap"]
    #[inline(always)]
    pub fn _0000011000(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011000)
    }
    #[doc = "Select 26 delay chain tap"]
    #[inline(always)]
    pub fn _0000011001(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011001)
    }
    #[doc = "Select 27 delay chain tap"]
    #[inline(always)]
    pub fn _0000011010(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011010)
    }
    #[doc = "Select 28 delay chain tap"]
    #[inline(always)]
    pub fn _0000011011(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011011)
    }
    #[doc = "Select 29 delay chain tap"]
    #[inline(always)]
    pub fn _0000011100(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011100)
    }
    #[doc = "Select 30 delay chain tap"]
    #[inline(always)]
    pub fn _0000011101(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011101)
    }
    #[doc = "Select 31 delay chain tap"]
    #[inline(always)]
    pub fn _0000011110(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011110)
    }
    #[doc = "Select 32 delay chain tap"]
    #[inline(always)]
    pub fn _0000011111(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_0000011111)
    }
    #[doc = "Select 33 delay chain tap"]
    #[inline(always)]
    pub fn _00000100000(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100000)
    }
    #[doc = "Select 34 delay chain tap"]
    #[inline(always)]
    pub fn _00000100001(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100001)
    }
    #[doc = "Select 35 delay chain tap"]
    #[inline(always)]
    pub fn _00000100010(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100010)
    }
    #[doc = "Select 36 delay chain tap"]
    #[inline(always)]
    pub fn _00000100011(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100011)
    }
    #[doc = "Select 37 delay chain tap"]
    #[inline(always)]
    pub fn _00000100100(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100100)
    }
    #[doc = "Select 38 delay chain tap"]
    #[inline(always)]
    pub fn _00000100101(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100101)
    }
    #[doc = "Select 39 delay chain tap"]
    #[inline(always)]
    pub fn _00000100110(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100110)
    }
    #[doc = "Select 40 delay chain tap"]
    #[inline(always)]
    pub fn _00000100111(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000100111)
    }
    #[doc = "Select 41 delay chain tap"]
    #[inline(always)]
    pub fn _00000101000(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101000)
    }
    #[doc = "Select 42 delay chain tap"]
    #[inline(always)]
    pub fn _00000101001(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101001)
    }
    #[doc = "Select 43 delay chain tap"]
    #[inline(always)]
    pub fn _00000101010(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101010)
    }
    #[doc = "Select 44 delay chain tap"]
    #[inline(always)]
    pub fn _00000101011(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101011)
    }
    #[doc = "Select 45 delay chain tap"]
    #[inline(always)]
    pub fn _00000101100(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101100)
    }
    #[doc = "Select 46 delay chain tap"]
    #[inline(always)]
    pub fn _00000101101(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101101)
    }
    #[doc = "Select 47 delay chain tap"]
    #[inline(always)]
    pub fn _00000101110(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101110)
    }
    #[doc = "Select 48 delay chain tap"]
    #[inline(always)]
    pub fn _00000101111(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000101111)
    }
    #[doc = "Select 49 delay chain tap"]
    #[inline(always)]
    pub fn _00000110000(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110000)
    }
    #[doc = "Select 50 delay chain tap"]
    #[inline(always)]
    pub fn _00000110001(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110001)
    }
    #[doc = "Select 51 delay chain tap"]
    #[inline(always)]
    pub fn _00000110010(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110010)
    }
    #[doc = "Select 52 delay chain tap"]
    #[inline(always)]
    pub fn _00000110011(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110011)
    }
    #[doc = "Select 53 delay chain tap"]
    #[inline(always)]
    pub fn _00000110100(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110100)
    }
    #[doc = "Select 54 delay chain tap"]
    #[inline(always)]
    pub fn _00000110101(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110101)
    }
    #[doc = "Select 55 delay chain tap"]
    #[inline(always)]
    pub fn _00000110110(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110110)
    }
    #[doc = "Select 56 delay chain tap"]
    #[inline(always)]
    pub fn _00000110111(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000110111)
    }
    #[doc = "Select 57 delay chain tap"]
    #[inline(always)]
    pub fn _00000111000(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111000)
    }
    #[doc = "Select 58 delay chain tap"]
    #[inline(always)]
    pub fn _00000111001(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111001)
    }
    #[doc = "Select 59 delay chain tap"]
    #[inline(always)]
    pub fn _00000111010(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111010)
    }
    #[doc = "Select 60 delay chain tap"]
    #[inline(always)]
    pub fn _00000111011(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111011)
    }
    #[doc = "Select 61 delay chain tap"]
    #[inline(always)]
    pub fn _00000111100(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111100)
    }
    #[doc = "Select 62 delay chain tap"]
    #[inline(always)]
    pub fn _00000111101(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111101)
    }
    #[doc = "Select 63 delay chain tap"]
    #[inline(always)]
    pub fn _00000111110(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111110)
    }
    #[doc = "Select 64 delay chain tap"]
    #[inline(always)]
    pub fn _00000111111(self) -> &'a mut W {
        self.variant(DLYTAPSELB_A::_00000111111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - QSPI clock source select"]
    #[inline(always)]
    pub fn qspisrc(&self) -> QSPISRC_R {
        QSPISRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - When this bit is set the internal generated DQS is selected and looped back to QuadSPI, without going to DQS pad. DQSPADLPEN should be cleared when this bit is set."]
    #[inline(always)]
    pub fn dqslpen(&self) -> DQSLPEN_R {
        DQSLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When this bit is set the internal generated DQS will be sent to the DQS pad first and then looped back to QuadSPI. DQSLPEN should be cleared when this bit is set."]
    #[inline(always)]
    pub fn dqspadlpen(&self) -> DQSPADLPEN_R {
        DQSPADLPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Select phase shift for internal DQS generation. These bits are always zero in SDR mode."]
    #[inline(always)]
    pub fn dqsphasel(&self) -> DQSPHASEL_R {
        DQSPHASEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Select clock source for internal DQS generation"]
    #[inline(always)]
    pub fn dqsinvsel(&self) -> DQSINVSEL_R {
        DQSINVSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Flash CK2 clock pin enable"]
    #[inline(always)]
    pub fn ck2en(&self) -> CK2EN_R {
        CK2EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Differential flash clock pins enable"]
    #[inline(always)]
    pub fn diffcken(&self) -> DIFFCKEN_R {
        DIFFCKEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Octal data pins enable"]
    #[inline(always)]
    pub fn octen(&self) -> OCTEN_R {
        OCTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Delay chain tap number selection for QSPI Port A DQS"]
    #[inline(always)]
    pub fn dlytapsela(&self) -> DLYTAPSELA_R {
        DLYTAPSELA_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Delay chain tap number selection for QSPI Port B DQS"]
    #[inline(always)]
    pub fn dlytapselb(&self) -> DLYTAPSELB_R {
        DLYTAPSELB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - QSPI clock source select"]
    #[inline(always)]
    pub fn qspisrc(&mut self) -> QSPISRC_W {
        QSPISRC_W { w: self }
    }
    #[doc = "Bit 8 - When this bit is set the internal generated DQS is selected and looped back to QuadSPI, without going to DQS pad. DQSPADLPEN should be cleared when this bit is set."]
    #[inline(always)]
    pub fn dqslpen(&mut self) -> DQSLPEN_W {
        DQSLPEN_W { w: self }
    }
    #[doc = "Bit 9 - When this bit is set the internal generated DQS will be sent to the DQS pad first and then looped back to QuadSPI. DQSLPEN should be cleared when this bit is set."]
    #[inline(always)]
    pub fn dqspadlpen(&mut self) -> DQSPADLPEN_W {
        DQSPADLPEN_W { w: self }
    }
    #[doc = "Bits 10:11 - Select phase shift for internal DQS generation. These bits are always zero in SDR mode."]
    #[inline(always)]
    pub fn dqsphasel(&mut self) -> DQSPHASEL_W {
        DQSPHASEL_W { w: self }
    }
    #[doc = "Bit 12 - Select clock source for internal DQS generation"]
    #[inline(always)]
    pub fn dqsinvsel(&mut self) -> DQSINVSEL_W {
        DQSINVSEL_W { w: self }
    }
    #[doc = "Bit 13 - Flash CK2 clock pin enable"]
    #[inline(always)]
    pub fn ck2en(&mut self) -> CK2EN_W {
        CK2EN_W { w: self }
    }
    #[doc = "Bit 14 - Differential flash clock pins enable"]
    #[inline(always)]
    pub fn diffcken(&mut self) -> DIFFCKEN_W {
        DIFFCKEN_W { w: self }
    }
    #[doc = "Bit 15 - Octal data pins enable"]
    #[inline(always)]
    pub fn octen(&mut self) -> OCTEN_W {
        OCTEN_W { w: self }
    }
    #[doc = "Bits 16:21 - Delay chain tap number selection for QSPI Port A DQS"]
    #[inline(always)]
    pub fn dlytapsela(&mut self) -> DLYTAPSELA_W {
        DLYTAPSELA_W { w: self }
    }
    #[doc = "Bits 24:29 - Delay chain tap number selection for QSPI Port B DQS"]
    #[inline(always)]
    pub fn dlytapselb(&mut self) -> DLYTAPSELB_W {
        DLYTAPSELB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SOC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soccr](index.html) module"]
pub struct SOCCR_SPEC;
impl crate::RegisterSpec for SOCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soccr::R](R) reader structure"]
impl crate::Readable for SOCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soccr::W](W) writer structure"]
impl crate::Writable for SOCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOCCR to value 0"]
impl crate::Resettable for SOCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
