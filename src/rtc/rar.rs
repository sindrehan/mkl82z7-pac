#[doc = "Register `RAR` reader"]
pub struct R(crate::R<RAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAR` writer"]
pub struct W(crate::W<RAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAR_SPEC>;
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
impl From<crate::W<RAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time Seconds Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSRR_A {
    #[doc = "0: Reads to the Time Seconds Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Time Seconds Register complete as normal."]
    _1 = 1,
}
impl From<TSRR_A> for bool {
    #[inline(always)]
    fn from(variant: TSRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSRR` reader - Time Seconds Register Read"]
pub struct TSRR_R(crate::FieldReader<bool, TSRR_A>);
impl TSRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSRR_A {
        match self.bits {
            false => TSRR_A::_0,
            true => TSRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSRR_A::_1
    }
}
impl core::ops::Deref for TSRR_R {
    type Target = crate::FieldReader<bool, TSRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSRR` writer - Time Seconds Register Read"]
pub struct TSRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSRR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reads to the Time Seconds Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSRR_A::_0)
    }
    #[doc = "Reads to the Time Seconds Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSRR_A::_1)
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
#[doc = "Time Prescaler Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRR_A {
    #[doc = "0: Reads to the Time Pprescaler Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Time Prescaler Register complete as normal."]
    _1 = 1,
}
impl From<TPRR_A> for bool {
    #[inline(always)]
    fn from(variant: TPRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPRR` reader - Time Prescaler Register Read"]
pub struct TPRR_R(crate::FieldReader<bool, TPRR_A>);
impl TPRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPRR_A {
        match self.bits {
            false => TPRR_A::_0,
            true => TPRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TPRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TPRR_A::_1
    }
}
impl core::ops::Deref for TPRR_R {
    type Target = crate::FieldReader<bool, TPRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPRR` writer - Time Prescaler Register Read"]
pub struct TPRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPRR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reads to the Time Pprescaler Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPRR_A::_0)
    }
    #[doc = "Reads to the Time Prescaler Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPRR_A::_1)
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
#[doc = "Time Alarm Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARR_A {
    #[doc = "0: Reads to the Time Alarm Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Time Alarm Register complete as normal."]
    _1 = 1,
}
impl From<TARR_A> for bool {
    #[inline(always)]
    fn from(variant: TARR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARR` reader - Time Alarm Register Read"]
pub struct TARR_R(crate::FieldReader<bool, TARR_A>);
impl TARR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARR_A {
        match self.bits {
            false => TARR_A::_0,
            true => TARR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TARR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TARR_A::_1
    }
}
impl core::ops::Deref for TARR_R {
    type Target = crate::FieldReader<bool, TARR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARR` writer - Time Alarm Register Read"]
pub struct TARR_W<'a> {
    w: &'a mut W,
}
impl<'a> TARR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TARR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reads to the Time Alarm Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TARR_A::_0)
    }
    #[doc = "Reads to the Time Alarm Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TARR_A::_1)
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
#[doc = "Time Compensation Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRR_A {
    #[doc = "0: Reads to the Time Compensation Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Time Compensation Register complete as normal."]
    _1 = 1,
}
impl From<TCRR_A> for bool {
    #[inline(always)]
    fn from(variant: TCRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCRR` reader - Time Compensation Register Read"]
pub struct TCRR_R(crate::FieldReader<bool, TCRR_A>);
impl TCRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCRR_A {
        match self.bits {
            false => TCRR_A::_0,
            true => TCRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TCRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TCRR_A::_1
    }
}
impl core::ops::Deref for TCRR_R {
    type Target = crate::FieldReader<bool, TCRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCRR` writer - Time Compensation Register Read"]
pub struct TCRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCRR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reads to the Time Compensation Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRR_A::_0)
    }
    #[doc = "Reads to the Time Compensation Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRR_A::_1)
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
#[doc = "Control Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR_A {
    #[doc = "0: Reads to the Control Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Control Register complete as normal."]
    _1 = 1,
}
impl From<CRR_A> for bool {
    #[inline(always)]
    fn from(variant: CRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRR` reader - Control Register Read"]
pub struct CRR_R(crate::FieldReader<bool, CRR_A>);
impl CRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRR_A {
        match self.bits {
            false => CRR_A::_0,
            true => CRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRR_A::_1
    }
}
impl core::ops::Deref for CRR_R {
    type Target = crate::FieldReader<bool, CRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRR` writer - Control Register Read"]
pub struct CRR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reads to the Control Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRR_A::_0)
    }
    #[doc = "Reads to the Control Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRR_A::_1)
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
#[doc = "Status Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR_A {
    #[doc = "0: Reads to the Status Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Status Register complete as normal."]
    _1 = 1,
}
impl From<SRR_A> for bool {
    #[inline(always)]
    fn from(variant: SRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRR` reader - Status Register Read"]
pub struct SRR_R(crate::FieldReader<bool, SRR_A>);
impl SRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRR_A {
        match self.bits {
            false => SRR_A::_0,
            true => SRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRR_A::_1
    }
}
impl core::ops::Deref for SRR_R {
    type Target = crate::FieldReader<bool, SRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRR` writer - Status Register Read"]
pub struct SRR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reads to the Status Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRR_A::_0)
    }
    #[doc = "Reads to the Status Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRR_A::_1)
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
#[doc = "Lock Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRR_A {
    #[doc = "0: Reads to the Lock Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Lock Register complete as normal."]
    _1 = 1,
}
impl From<LRR_A> for bool {
    #[inline(always)]
    fn from(variant: LRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRR` reader - Lock Register Read"]
pub struct LRR_R(crate::FieldReader<bool, LRR_A>);
impl LRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRR_A {
        match self.bits {
            false => LRR_A::_0,
            true => LRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LRR_A::_1
    }
}
impl core::ops::Deref for LRR_R {
    type Target = crate::FieldReader<bool, LRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRR` writer - Lock Register Read"]
pub struct LRR_W<'a> {
    w: &'a mut W,
}
impl<'a> LRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reads to the Lock Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRR_A::_0)
    }
    #[doc = "Reads to the Lock Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRR_A::_1)
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
#[doc = "Interrupt Enable Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERR_A {
    #[doc = "0: Reads to the Interrupt Enable Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Interrupt Enable Register complete as normal."]
    _1 = 1,
}
impl From<IERR_A> for bool {
    #[inline(always)]
    fn from(variant: IERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERR` reader - Interrupt Enable Register Read"]
pub struct IERR_R(crate::FieldReader<bool, IERR_A>);
impl IERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IERR_A {
        match self.bits {
            false => IERR_A::_0,
            true => IERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IERR_A::_1
    }
}
impl core::ops::Deref for IERR_R {
    type Target = crate::FieldReader<bool, IERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IERR` writer - Interrupt Enable Register Read"]
pub struct IERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reads to the Interrupt Enable Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERR_A::_0)
    }
    #[doc = "Reads to the Interrupt Enable Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERR_A::_1)
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
    #[doc = "Bit 0 - Time Seconds Register Read"]
    #[inline(always)]
    pub fn tsrr(&self) -> TSRR_R {
        TSRR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Time Prescaler Register Read"]
    #[inline(always)]
    pub fn tprr(&self) -> TPRR_R {
        TPRR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Register Read"]
    #[inline(always)]
    pub fn tarr(&self) -> TARR_R {
        TARR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time Compensation Register Read"]
    #[inline(always)]
    pub fn tcrr(&self) -> TCRR_R {
        TCRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control Register Read"]
    #[inline(always)]
    pub fn crr(&self) -> CRR_R {
        CRR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status Register Read"]
    #[inline(always)]
    pub fn srr(&self) -> SRR_R {
        SRR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Register Read"]
    #[inline(always)]
    pub fn lrr(&self) -> LRR_R {
        LRR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable Register Read"]
    #[inline(always)]
    pub fn ierr(&self) -> IERR_R {
        IERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Seconds Register Read"]
    #[inline(always)]
    pub fn tsrr(&mut self) -> TSRR_W {
        TSRR_W { w: self }
    }
    #[doc = "Bit 1 - Time Prescaler Register Read"]
    #[inline(always)]
    pub fn tprr(&mut self) -> TPRR_W {
        TPRR_W { w: self }
    }
    #[doc = "Bit 2 - Time Alarm Register Read"]
    #[inline(always)]
    pub fn tarr(&mut self) -> TARR_W {
        TARR_W { w: self }
    }
    #[doc = "Bit 3 - Time Compensation Register Read"]
    #[inline(always)]
    pub fn tcrr(&mut self) -> TCRR_W {
        TCRR_W { w: self }
    }
    #[doc = "Bit 4 - Control Register Read"]
    #[inline(always)]
    pub fn crr(&mut self) -> CRR_W {
        CRR_W { w: self }
    }
    #[doc = "Bit 5 - Status Register Read"]
    #[inline(always)]
    pub fn srr(&mut self) -> SRR_W {
        SRR_W { w: self }
    }
    #[doc = "Bit 6 - Lock Register Read"]
    #[inline(always)]
    pub fn lrr(&mut self) -> LRR_W {
        LRR_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Enable Register Read"]
    #[inline(always)]
    pub fn ierr(&mut self) -> IERR_W {
        IERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Read Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rar](index.html) module"]
pub struct RAR_SPEC;
impl crate::RegisterSpec for RAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rar::R](R) reader structure"]
impl crate::Readable for RAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rar::W](W) writer structure"]
impl crate::Writable for RAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAR to value 0xff"]
impl crate::Resettable for RAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
