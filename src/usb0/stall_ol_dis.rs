#[doc = "Register `STALL_OL_DIS` reader"]
pub struct R(crate::R<STALL_OL_DIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALL_OL_DIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALL_OL_DIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALL_OL_DIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STALL_OL_DIS` writer"]
pub struct W(crate::W<STALL_OL_DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STALL_OL_DIS_SPEC>;
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
impl From<crate::W<STALL_OL_DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STALL_OL_DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Disable endpoint 0 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS0_A {
    #[doc = "0: Endpoint 0 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 0 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS0_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS0` reader - Disable endpoint 0 OUT direction."]
pub struct STALL_O_DIS0_R(crate::FieldReader<bool, STALL_O_DIS0_A>);
impl STALL_O_DIS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS0_A {
        match self.bits {
            false => STALL_O_DIS0_A::_0,
            true => STALL_O_DIS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS0_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS0_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS0` writer - Disable endpoint 0 OUT direction."]
pub struct STALL_O_DIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 0 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS0_A::_0)
    }
    #[doc = "Endpoint 0 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS0_A::_1)
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
#[doc = "Disable endpoint 1 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS1_A {
    #[doc = "0: Endpoint 1 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 1 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS1_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS1` reader - Disable endpoint 1 OUT direction."]
pub struct STALL_O_DIS1_R(crate::FieldReader<bool, STALL_O_DIS1_A>);
impl STALL_O_DIS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS1_A {
        match self.bits {
            false => STALL_O_DIS1_A::_0,
            true => STALL_O_DIS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS1_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS1_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS1` writer - Disable endpoint 1 OUT direction."]
pub struct STALL_O_DIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 1 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS1_A::_0)
    }
    #[doc = "Endpoint 1 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS1_A::_1)
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
#[doc = "Disable endpoint 2 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS2_A {
    #[doc = "0: Endpoint 2 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 2 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS2_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS2` reader - Disable endpoint 2 OUT direction."]
pub struct STALL_O_DIS2_R(crate::FieldReader<bool, STALL_O_DIS2_A>);
impl STALL_O_DIS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS2_A {
        match self.bits {
            false => STALL_O_DIS2_A::_0,
            true => STALL_O_DIS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS2_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS2_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS2` writer - Disable endpoint 2 OUT direction."]
pub struct STALL_O_DIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 2 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS2_A::_0)
    }
    #[doc = "Endpoint 2 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS2_A::_1)
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
#[doc = "Disable endpoint 3 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS3_A {
    #[doc = "0: Endpoint 3 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 3 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS3_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS3` reader - Disable endpoint 3 OUT direction."]
pub struct STALL_O_DIS3_R(crate::FieldReader<bool, STALL_O_DIS3_A>);
impl STALL_O_DIS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS3_A {
        match self.bits {
            false => STALL_O_DIS3_A::_0,
            true => STALL_O_DIS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS3_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS3_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS3` writer - Disable endpoint 3 OUT direction."]
pub struct STALL_O_DIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 3 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS3_A::_0)
    }
    #[doc = "Endpoint 3 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS3_A::_1)
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
#[doc = "Disable endpoint 4 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS4_A {
    #[doc = "0: Endpoint 4 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 4 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS4_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS4` reader - Disable endpoint 4 OUT direction."]
pub struct STALL_O_DIS4_R(crate::FieldReader<bool, STALL_O_DIS4_A>);
impl STALL_O_DIS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS4_A {
        match self.bits {
            false => STALL_O_DIS4_A::_0,
            true => STALL_O_DIS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS4_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS4_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS4` writer - Disable endpoint 4 OUT direction."]
pub struct STALL_O_DIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 4 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS4_A::_0)
    }
    #[doc = "Endpoint 4 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS4_A::_1)
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
#[doc = "Disable endpoint 5 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS5_A {
    #[doc = "0: Endpoint 5 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 5 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS5_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS5` reader - Disable endpoint 5 OUT direction."]
pub struct STALL_O_DIS5_R(crate::FieldReader<bool, STALL_O_DIS5_A>);
impl STALL_O_DIS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS5_A {
        match self.bits {
            false => STALL_O_DIS5_A::_0,
            true => STALL_O_DIS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS5_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS5_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS5` writer - Disable endpoint 5 OUT direction."]
pub struct STALL_O_DIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 5 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS5_A::_0)
    }
    #[doc = "Endpoint 5 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS5_A::_1)
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
#[doc = "Disable endpoint 6 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS6_A {
    #[doc = "0: Endpoint 6 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 6 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS6_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS6` reader - Disable endpoint 6 OUT direction."]
pub struct STALL_O_DIS6_R(crate::FieldReader<bool, STALL_O_DIS6_A>);
impl STALL_O_DIS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS6_A {
        match self.bits {
            false => STALL_O_DIS6_A::_0,
            true => STALL_O_DIS6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS6_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS6_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS6` writer - Disable endpoint 6 OUT direction."]
pub struct STALL_O_DIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 6 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS6_A::_0)
    }
    #[doc = "Endpoint 6 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS6_A::_1)
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
#[doc = "Disable endpoint 7 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS7_A {
    #[doc = "0: Endpoint 7 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 7 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS7_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL_O_DIS7` reader - Disable endpoint 7 OUT direction."]
pub struct STALL_O_DIS7_R(crate::FieldReader<bool, STALL_O_DIS7_A>);
impl STALL_O_DIS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_O_DIS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS7_A {
        match self.bits {
            false => STALL_O_DIS7_A::_0,
            true => STALL_O_DIS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALL_O_DIS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALL_O_DIS7_A::_1
    }
}
impl core::ops::Deref for STALL_O_DIS7_R {
    type Target = crate::FieldReader<bool, STALL_O_DIS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL_O_DIS7` writer - Disable endpoint 7 OUT direction."]
pub struct STALL_O_DIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Endpoint 7 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS7_A::_0)
    }
    #[doc = "Endpoint 7 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS7_A::_1)
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
    #[doc = "Bit 0 - Disable endpoint 0 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis0(&self) -> STALL_O_DIS0_R {
        STALL_O_DIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable endpoint 1 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis1(&self) -> STALL_O_DIS1_R {
        STALL_O_DIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable endpoint 2 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis2(&self) -> STALL_O_DIS2_R {
        STALL_O_DIS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable endpoint 3 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis3(&self) -> STALL_O_DIS3_R {
        STALL_O_DIS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Disable endpoint 4 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis4(&self) -> STALL_O_DIS4_R {
        STALL_O_DIS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable endpoint 5 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis5(&self) -> STALL_O_DIS5_R {
        STALL_O_DIS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable endpoint 6 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis6(&self) -> STALL_O_DIS6_R {
        STALL_O_DIS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Disable endpoint 7 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis7(&self) -> STALL_O_DIS7_R {
        STALL_O_DIS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable endpoint 0 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis0(&mut self) -> STALL_O_DIS0_W {
        STALL_O_DIS0_W { w: self }
    }
    #[doc = "Bit 1 - Disable endpoint 1 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis1(&mut self) -> STALL_O_DIS1_W {
        STALL_O_DIS1_W { w: self }
    }
    #[doc = "Bit 2 - Disable endpoint 2 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis2(&mut self) -> STALL_O_DIS2_W {
        STALL_O_DIS2_W { w: self }
    }
    #[doc = "Bit 3 - Disable endpoint 3 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis3(&mut self) -> STALL_O_DIS3_W {
        STALL_O_DIS3_W { w: self }
    }
    #[doc = "Bit 4 - Disable endpoint 4 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis4(&mut self) -> STALL_O_DIS4_W {
        STALL_O_DIS4_W { w: self }
    }
    #[doc = "Bit 5 - Disable endpoint 5 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis5(&mut self) -> STALL_O_DIS5_W {
        STALL_O_DIS5_W { w: self }
    }
    #[doc = "Bit 6 - Disable endpoint 6 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis6(&mut self) -> STALL_O_DIS6_W {
        STALL_O_DIS6_W { w: self }
    }
    #[doc = "Bit 7 - Disable endpoint 7 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis7(&mut self) -> STALL_O_DIS7_W {
        STALL_O_DIS7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral mode stall disable for endpoints 7 to 0 in OUT direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stall_ol_dis](index.html) module"]
pub struct STALL_OL_DIS_SPEC;
impl crate::RegisterSpec for STALL_OL_DIS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [stall_ol_dis::R](R) reader structure"]
impl crate::Readable for STALL_OL_DIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stall_ol_dis::W](W) writer structure"]
impl crate::Writable for STALL_OL_DIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STALL_OL_DIS to value 0"]
impl crate::Resettable for STALL_OL_DIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
