#[doc = "Register `STALL_OH_DIS` reader"]
pub struct R(crate::R<STALL_OH_DIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALL_OH_DIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALL_OH_DIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALL_OH_DIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STALL_OH_DIS` writer"]
pub struct W(crate::W<STALL_OH_DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STALL_OH_DIS_SPEC>;
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
impl From<crate::W<STALL_OH_DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STALL_OH_DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Disable endpoint 8 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS8_A {
    #[doc = "0: Endpoint 8 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 8 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS8_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS8` reader - Disable endpoint 8 OUT direction."]
pub struct STALL_O_DIS8_R(crate::FieldReader<bool, STALL_O_DIS8_A>);
impl STALL_O_DIS8_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS8_A {
        match self.bits {
            false => STALL_O_DIS8_A::_0,
            true => STALL_O_DIS8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS8_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS8_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS8` writer - Disable endpoint 8 OUT direction."]
pub struct STALL_O_DIS8_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 8 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS8_A::_0)
    }
    #[doc = "Endpoint 8 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS8_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Disable endpoint 9 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS9_A {
    #[doc = "0: Endpoint 9 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 9 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS9_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS9` reader - Disable endpoint 9 OUT direction."]
pub struct STALL_O_DIS9_R(crate::FieldReader<bool, STALL_O_DIS9_A>);
impl STALL_O_DIS9_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS9_A {
        match self.bits {
            false => STALL_O_DIS9_A::_0,
            true => STALL_O_DIS9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS9_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS9_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS9` writer - Disable endpoint 9 OUT direction."]
pub struct STALL_O_DIS9_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 9 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS9_A::_0)
    }
    #[doc = "Endpoint 9 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS9_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Disable endpoint 10 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS10_A {
    #[doc = "0: Endpoint 10 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 10 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS10_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS10` reader - Disable endpoint 10 OUT direction."]
pub struct STALL_O_DIS10_R(crate::FieldReader<bool, STALL_O_DIS10_A>);
impl STALL_O_DIS10_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS10_A {
        match self.bits {
            false => STALL_O_DIS10_A::_0,
            true => STALL_O_DIS10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS10_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS10_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS10` writer - Disable endpoint 10 OUT direction."]
pub struct STALL_O_DIS10_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 10 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS10_A::_0)
    }
    #[doc = "Endpoint 10 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS10_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Disable endpoint 11 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS11_A {
    #[doc = "0: Endpoint 11 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 11 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS11_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS11` reader - Disable endpoint 11 OUT direction."]
pub struct STALL_O_DIS11_R(crate::FieldReader<bool, STALL_O_DIS11_A>);
impl STALL_O_DIS11_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS11_A {
        match self.bits {
            false => STALL_O_DIS11_A::_0,
            true => STALL_O_DIS11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS11_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS11_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS11` writer - Disable endpoint 11 OUT direction."]
pub struct STALL_O_DIS11_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 11 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS11_A::_0)
    }
    #[doc = "Endpoint 11 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS11_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Disable endpoint 12 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS12_A {
    #[doc = "0: Endpoint 12 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 12 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS12_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS12` reader - Disable endpoint 12 OUT direction."]
pub struct STALL_O_DIS12_R(crate::FieldReader<bool, STALL_O_DIS12_A>);
impl STALL_O_DIS12_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS12_A {
        match self.bits {
            false => STALL_O_DIS12_A::_0,
            true => STALL_O_DIS12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS12_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS12_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS12` writer - Disable endpoint 12 OUT direction."]
pub struct STALL_O_DIS12_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 12 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS12_A::_0)
    }
    #[doc = "Endpoint 12 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS12_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Disable endpoint 13 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS13_A {
    #[doc = "0: Endpoint 13 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 13 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS13_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS13` reader - Disable endpoint 13 OUT direction."]
pub struct STALL_O_DIS13_R(crate::FieldReader<bool, STALL_O_DIS13_A>);
impl STALL_O_DIS13_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS13_A {
        match self.bits {
            false => STALL_O_DIS13_A::_0,
            true => STALL_O_DIS13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS13_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS13_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS13` writer - Disable endpoint 13 OUT direction."]
pub struct STALL_O_DIS13_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 13 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS13_A::_0)
    }
    #[doc = "Endpoint 13 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS13_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Disable endpoint 14 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS14_A {
    #[doc = "0: Endpoint 14 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 14 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS14_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS14` reader - Disable endpoint 14 OUT direction."]
pub struct STALL_O_DIS14_R(crate::FieldReader<bool, STALL_O_DIS14_A>);
impl STALL_O_DIS14_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS14_A {
        match self.bits {
            false => STALL_O_DIS14_A::_0,
            true => STALL_O_DIS14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS14_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS14_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS14` writer - Disable endpoint 14 OUT direction."]
pub struct STALL_O_DIS14_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 14 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS14_A::_0)
    }
    #[doc = "Endpoint 14 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS14_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Disable endpoint 15 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS15_A {
    #[doc = "0: Endpoint 15 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 15 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS15_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS15` reader - Disable endpoint 15 OUT direction."]
pub struct STALL_O_DIS15_R(crate::FieldReader<bool, STALL_O_DIS15_A>);
impl STALL_O_DIS15_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS15_A {
        match self.bits {
            false => STALL_O_DIS15_A::_0,
            true => STALL_O_DIS15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS15_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS15_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS15` writer - Disable endpoint 15 OUT direction."]
pub struct STALL_O_DIS15_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 15 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS15_A::_0)
    }
    #[doc = "Endpoint 15 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS15_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable endpoint 8 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis8(&self) -> STALL_O_DIS8_R {
        STALL_O_DIS8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable endpoint 9 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis9(&self) -> STALL_O_DIS9_R {
        STALL_O_DIS9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable endpoint 10 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis10(&self) -> STALL_O_DIS10_R {
        STALL_O_DIS10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable endpoint 11 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis11(&self) -> STALL_O_DIS11_R {
        STALL_O_DIS11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Disable endpoint 12 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis12(&self) -> STALL_O_DIS12_R {
        STALL_O_DIS12_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable endpoint 13 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis13(&self) -> STALL_O_DIS13_R {
        STALL_O_DIS13_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable endpoint 14 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis14(&self) -> STALL_O_DIS14_R {
        STALL_O_DIS14_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Disable endpoint 15 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis15(&self) -> STALL_O_DIS15_R {
        STALL_O_DIS15_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable endpoint 8 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis8(&mut self) -> STALL_O_DIS8_W {
        STALL_O_DIS8_W { w: self }
    }
    #[doc = "Bit 1 - Disable endpoint 9 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis9(&mut self) -> STALL_O_DIS9_W {
        STALL_O_DIS9_W { w: self }
    }
    #[doc = "Bit 2 - Disable endpoint 10 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis10(&mut self) -> STALL_O_DIS10_W {
        STALL_O_DIS10_W { w: self }
    }
    #[doc = "Bit 3 - Disable endpoint 11 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis11(&mut self) -> STALL_O_DIS11_W {
        STALL_O_DIS11_W { w: self }
    }
    #[doc = "Bit 4 - Disable endpoint 12 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis12(&mut self) -> STALL_O_DIS12_W {
        STALL_O_DIS12_W { w: self }
    }
    #[doc = "Bit 5 - Disable endpoint 13 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis13(&mut self) -> STALL_O_DIS13_W {
        STALL_O_DIS13_W { w: self }
    }
    #[doc = "Bit 6 - Disable endpoint 14 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis14(&mut self) -> STALL_O_DIS14_W {
        STALL_O_DIS14_W { w: self }
    }
    #[doc = "Bit 7 - Disable endpoint 15 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis15(&mut self) -> STALL_O_DIS15_W {
        STALL_O_DIS15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral mode stall disable for endpoints 15 to 8 in OUT direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stall_oh_dis](index.html) module"]
pub struct STALL_OH_DIS_SPEC;
impl crate::RegisterSpec for STALL_OH_DIS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [stall_oh_dis::R](R) reader structure"]
impl crate::Readable for STALL_OH_DIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stall_oh_dis::W](W) writer structure"]
impl crate::Writable for STALL_OH_DIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STALL_OH_DIS to value 0"]
impl crate::Resettable for STALL_OH_DIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
