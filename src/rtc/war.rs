#[doc = "Register `WAR` reader"]
pub struct R(crate::R<WAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAR` writer"]
pub struct W(crate::W<WAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAR_SPEC>;
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
impl From<crate::W<WAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time Seconds Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSRW_A {
    #[doc = "0: Writes to the Time Seconds Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Time Seconds Register complete as normal."]
    _1 = 1,
}
impl From<TSRW_A> for bool {
    #[inline(always)]
    fn from(variant: TSRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSRW` reader - Time Seconds Register Write"]
pub struct TSRW_R(crate::FieldReader<bool, TSRW_A>);
impl TSRW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSRW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSRW_A {
        match self.bits {
            false => TSRW_A::_0,
            true => TSRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSRW_A::_1
    }
}
impl core::ops::Deref for TSRW_R {
    type Target = crate::FieldReader<bool, TSRW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSRW` writer - Time Seconds Register Write"]
pub struct TSRW_W<'a> {
    w: &'a mut W,
}
impl<'a> TSRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSRW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the Time Seconds Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSRW_A::_0)
    }
    #[doc = "Writes to the Time Seconds Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSRW_A::_1)
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
#[doc = "Time Prescaler Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRW_A {
    #[doc = "0: Writes to the Time Prescaler Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Time Prescaler Register complete as normal."]
    _1 = 1,
}
impl From<TPRW_A> for bool {
    #[inline(always)]
    fn from(variant: TPRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPRW` reader - Time Prescaler Register Write"]
pub struct TPRW_R(crate::FieldReader<bool, TPRW_A>);
impl TPRW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPRW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPRW_A {
        match self.bits {
            false => TPRW_A::_0,
            true => TPRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TPRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TPRW_A::_1
    }
}
impl core::ops::Deref for TPRW_R {
    type Target = crate::FieldReader<bool, TPRW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPRW` writer - Time Prescaler Register Write"]
pub struct TPRW_W<'a> {
    w: &'a mut W,
}
impl<'a> TPRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPRW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the Time Prescaler Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPRW_A::_0)
    }
    #[doc = "Writes to the Time Prescaler Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPRW_A::_1)
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
#[doc = "Time Alarm Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARW_A {
    #[doc = "0: Writes to the Time Alarm Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Time Alarm Register complete as normal."]
    _1 = 1,
}
impl From<TARW_A> for bool {
    #[inline(always)]
    fn from(variant: TARW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARW` reader - Time Alarm Register Write"]
pub struct TARW_R(crate::FieldReader<bool, TARW_A>);
impl TARW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARW_A {
        match self.bits {
            false => TARW_A::_0,
            true => TARW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TARW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TARW_A::_1
    }
}
impl core::ops::Deref for TARW_R {
    type Target = crate::FieldReader<bool, TARW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARW` writer - Time Alarm Register Write"]
pub struct TARW_W<'a> {
    w: &'a mut W,
}
impl<'a> TARW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TARW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the Time Alarm Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TARW_A::_0)
    }
    #[doc = "Writes to the Time Alarm Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TARW_A::_1)
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
#[doc = "Time Compensation Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRW_A {
    #[doc = "0: Writes to the Time Compensation Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Time Compensation Register complete as normal."]
    _1 = 1,
}
impl From<TCRW_A> for bool {
    #[inline(always)]
    fn from(variant: TCRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCRW` reader - Time Compensation Register Write"]
pub struct TCRW_R(crate::FieldReader<bool, TCRW_A>);
impl TCRW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCRW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCRW_A {
        match self.bits {
            false => TCRW_A::_0,
            true => TCRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TCRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TCRW_A::_1
    }
}
impl core::ops::Deref for TCRW_R {
    type Target = crate::FieldReader<bool, TCRW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCRW` writer - Time Compensation Register Write"]
pub struct TCRW_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCRW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the Time Compensation Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRW_A::_0)
    }
    #[doc = "Writes to the Time Compensation Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRW_A::_1)
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
#[doc = "Control Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRW_A {
    #[doc = "0: Writes to the Control Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Control Register complete as normal."]
    _1 = 1,
}
impl From<CRW_A> for bool {
    #[inline(always)]
    fn from(variant: CRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRW` reader - Control Register Write"]
pub struct CRW_R(crate::FieldReader<bool, CRW_A>);
impl CRW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRW_A {
        match self.bits {
            false => CRW_A::_0,
            true => CRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRW_A::_1
    }
}
impl core::ops::Deref for CRW_R {
    type Target = crate::FieldReader<bool, CRW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRW` writer - Control Register Write"]
pub struct CRW_W<'a> {
    w: &'a mut W,
}
impl<'a> CRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the Control Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRW_A::_0)
    }
    #[doc = "Writes to the Control Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRW_A::_1)
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
#[doc = "Status Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRW_A {
    #[doc = "0: Writes to the Status Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Status Register complete as normal."]
    _1 = 1,
}
impl From<SRW_A> for bool {
    #[inline(always)]
    fn from(variant: SRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRW` reader - Status Register Write"]
pub struct SRW_R(crate::FieldReader<bool, SRW_A>);
impl SRW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRW_A {
        match self.bits {
            false => SRW_A::_0,
            true => SRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRW_A::_1
    }
}
impl core::ops::Deref for SRW_R {
    type Target = crate::FieldReader<bool, SRW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRW` writer - Status Register Write"]
pub struct SRW_W<'a> {
    w: &'a mut W,
}
impl<'a> SRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the Status Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRW_A::_0)
    }
    #[doc = "Writes to the Status Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRW_A::_1)
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
#[doc = "Lock Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRW_A {
    #[doc = "0: Writes to the Lock Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Lock Register complete as normal."]
    _1 = 1,
}
impl From<LRW_A> for bool {
    #[inline(always)]
    fn from(variant: LRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRW` reader - Lock Register Write"]
pub struct LRW_R(crate::FieldReader<bool, LRW_A>);
impl LRW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRW_A {
        match self.bits {
            false => LRW_A::_0,
            true => LRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LRW_A::_1
    }
}
impl core::ops::Deref for LRW_R {
    type Target = crate::FieldReader<bool, LRW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRW` writer - Lock Register Write"]
pub struct LRW_W<'a> {
    w: &'a mut W,
}
impl<'a> LRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the Lock Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRW_A::_0)
    }
    #[doc = "Writes to the Lock Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRW_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Interrupt Enable Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERW_A {
    #[doc = "0: Writes to the Interupt Enable Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Interrupt Enable Register complete as normal."]
    _1 = 1,
}
impl From<IERW_A> for bool {
    #[inline(always)]
    fn from(variant: IERW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERW` reader - Interrupt Enable Register Write"]
pub struct IERW_R(crate::FieldReader<bool, IERW_A>);
impl IERW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IERW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IERW_A {
        match self.bits {
            false => IERW_A::_0,
            true => IERW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IERW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IERW_A::_1
    }
}
impl core::ops::Deref for IERW_R {
    type Target = crate::FieldReader<bool, IERW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IERW` writer - Interrupt Enable Register Write"]
pub struct IERW_W<'a> {
    w: &'a mut W,
}
impl<'a> IERW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IERW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writes to the Interupt Enable Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERW_A::_0)
    }
    #[doc = "Writes to the Interrupt Enable Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERW_A::_1)
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
impl R {
    #[doc = "Bit 0 - Time Seconds Register Write"]
    #[inline(always)]
    pub fn tsrw(&self) -> TSRW_R {
        TSRW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Time Prescaler Register Write"]
    #[inline(always)]
    pub fn tprw(&self) -> TPRW_R {
        TPRW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Register Write"]
    #[inline(always)]
    pub fn tarw(&self) -> TARW_R {
        TARW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time Compensation Register Write"]
    #[inline(always)]
    pub fn tcrw(&self) -> TCRW_R {
        TCRW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control Register Write"]
    #[inline(always)]
    pub fn crw(&self) -> CRW_R {
        CRW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status Register Write"]
    #[inline(always)]
    pub fn srw(&self) -> SRW_R {
        SRW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Register Write"]
    #[inline(always)]
    pub fn lrw(&self) -> LRW_R {
        LRW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable Register Write"]
    #[inline(always)]
    pub fn ierw(&self) -> IERW_R {
        IERW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Seconds Register Write"]
    #[inline(always)]
    pub fn tsrw(&mut self) -> TSRW_W {
        TSRW_W { w: self }
    }
    #[doc = "Bit 1 - Time Prescaler Register Write"]
    #[inline(always)]
    pub fn tprw(&mut self) -> TPRW_W {
        TPRW_W { w: self }
    }
    #[doc = "Bit 2 - Time Alarm Register Write"]
    #[inline(always)]
    pub fn tarw(&mut self) -> TARW_W {
        TARW_W { w: self }
    }
    #[doc = "Bit 3 - Time Compensation Register Write"]
    #[inline(always)]
    pub fn tcrw(&mut self) -> TCRW_W {
        TCRW_W { w: self }
    }
    #[doc = "Bit 4 - Control Register Write"]
    #[inline(always)]
    pub fn crw(&mut self) -> CRW_W {
        CRW_W { w: self }
    }
    #[doc = "Bit 5 - Status Register Write"]
    #[inline(always)]
    pub fn srw(&mut self) -> SRW_W {
        SRW_W { w: self }
    }
    #[doc = "Bit 6 - Lock Register Write"]
    #[inline(always)]
    pub fn lrw(&mut self) -> LRW_W {
        LRW_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Enable Register Write"]
    #[inline(always)]
    pub fn ierw(&mut self) -> IERW_W {
        IERW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Write Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [war](index.html) module"]
pub struct WAR_SPEC;
impl crate::RegisterSpec for WAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [war::R](R) reader structure"]
impl crate::Readable for WAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [war::W](W) writer structure"]
impl crate::Writable for WAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAR to value 0xff"]
impl crate::Resettable for WAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
