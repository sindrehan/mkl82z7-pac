#[doc = "Register `EARS` reader"]
pub struct R(crate::R<EARS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EARS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EARS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EARS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EARS` writer"]
pub struct W(crate::W<EARS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EARS_SPEC>;
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
impl From<crate::W<EARS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EARS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable asynchronous DMA request in stop mode for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_0_A {
    #[doc = "0: Disable asynchronous DMA request for channel 0."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 0."]
    _1 = 1,
}
impl From<EDREQ_0_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDREQ_0` reader - Enable asynchronous DMA request in stop mode for channel 0."]
pub struct EDREQ_0_R(crate::FieldReader<bool, EDREQ_0_A>);
impl EDREQ_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDREQ_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_0_A {
        match self.bits {
            false => EDREQ_0_A::_0,
            true => EDREQ_0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDREQ_0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDREQ_0_A::_1
    }
}
impl core::ops::Deref for EDREQ_0_R {
    type Target = crate::FieldReader<bool, EDREQ_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDREQ_0` writer - Enable asynchronous DMA request in stop mode for channel 0."]
pub struct EDREQ_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable asynchronous DMA request for channel 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_0_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_0_A::_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_1_A {
    #[doc = "0: Disable asynchronous DMA request for channel 1"]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 1."]
    _1 = 1,
}
impl From<EDREQ_1_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDREQ_1` reader - Enable asynchronous DMA request in stop mode for channel 1."]
pub struct EDREQ_1_R(crate::FieldReader<bool, EDREQ_1_A>);
impl EDREQ_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDREQ_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_1_A {
        match self.bits {
            false => EDREQ_1_A::_0,
            true => EDREQ_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDREQ_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDREQ_1_A::_1
    }
}
impl core::ops::Deref for EDREQ_1_R {
    type Target = crate::FieldReader<bool, EDREQ_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDREQ_1` writer - Enable asynchronous DMA request in stop mode for channel 1."]
pub struct EDREQ_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable asynchronous DMA request for channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_1_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_1_A::_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_2_A {
    #[doc = "0: Disable asynchronous DMA request for channel 2."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 2."]
    _1 = 1,
}
impl From<EDREQ_2_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDREQ_2` reader - Enable asynchronous DMA request in stop mode for channel 2."]
pub struct EDREQ_2_R(crate::FieldReader<bool, EDREQ_2_A>);
impl EDREQ_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDREQ_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_2_A {
        match self.bits {
            false => EDREQ_2_A::_0,
            true => EDREQ_2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDREQ_2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDREQ_2_A::_1
    }
}
impl core::ops::Deref for EDREQ_2_R {
    type Target = crate::FieldReader<bool, EDREQ_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDREQ_2` writer - Enable asynchronous DMA request in stop mode for channel 2."]
pub struct EDREQ_2_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable asynchronous DMA request for channel 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_2_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_2_A::_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_3_A {
    #[doc = "0: Disable asynchronous DMA request for channel 3."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 3."]
    _1 = 1,
}
impl From<EDREQ_3_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDREQ_3` reader - Enable asynchronous DMA request in stop mode for channel 3."]
pub struct EDREQ_3_R(crate::FieldReader<bool, EDREQ_3_A>);
impl EDREQ_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDREQ_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_3_A {
        match self.bits {
            false => EDREQ_3_A::_0,
            true => EDREQ_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDREQ_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDREQ_3_A::_1
    }
}
impl core::ops::Deref for EDREQ_3_R {
    type Target = crate::FieldReader<bool, EDREQ_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDREQ_3` writer - Enable asynchronous DMA request in stop mode for channel 3."]
pub struct EDREQ_3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable asynchronous DMA request for channel 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_3_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_3_A::_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_4_A {
    #[doc = "0: Disable asynchronous DMA request for channel 4."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 4."]
    _1 = 1,
}
impl From<EDREQ_4_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDREQ_4` reader - Enable asynchronous DMA request in stop mode for channel 4"]
pub struct EDREQ_4_R(crate::FieldReader<bool, EDREQ_4_A>);
impl EDREQ_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDREQ_4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_4_A {
        match self.bits {
            false => EDREQ_4_A::_0,
            true => EDREQ_4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDREQ_4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDREQ_4_A::_1
    }
}
impl core::ops::Deref for EDREQ_4_R {
    type Target = crate::FieldReader<bool, EDREQ_4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDREQ_4` writer - Enable asynchronous DMA request in stop mode for channel 4"]
pub struct EDREQ_4_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable asynchronous DMA request for channel 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_4_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_4_A::_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_5_A {
    #[doc = "0: Disable asynchronous DMA request for channel 5."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 5."]
    _1 = 1,
}
impl From<EDREQ_5_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDREQ_5` reader - Enable asynchronous DMA request in stop mode for channel 5"]
pub struct EDREQ_5_R(crate::FieldReader<bool, EDREQ_5_A>);
impl EDREQ_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDREQ_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_5_A {
        match self.bits {
            false => EDREQ_5_A::_0,
            true => EDREQ_5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDREQ_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDREQ_5_A::_1
    }
}
impl core::ops::Deref for EDREQ_5_R {
    type Target = crate::FieldReader<bool, EDREQ_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDREQ_5` writer - Enable asynchronous DMA request in stop mode for channel 5"]
pub struct EDREQ_5_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable asynchronous DMA request for channel 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_5_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_5_A::_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_6_A {
    #[doc = "0: Disable asynchronous DMA request for channel 6."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 6."]
    _1 = 1,
}
impl From<EDREQ_6_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDREQ_6` reader - Enable asynchronous DMA request in stop mode for channel 6"]
pub struct EDREQ_6_R(crate::FieldReader<bool, EDREQ_6_A>);
impl EDREQ_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDREQ_6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_6_A {
        match self.bits {
            false => EDREQ_6_A::_0,
            true => EDREQ_6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDREQ_6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDREQ_6_A::_1
    }
}
impl core::ops::Deref for EDREQ_6_R {
    type Target = crate::FieldReader<bool, EDREQ_6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDREQ_6` writer - Enable asynchronous DMA request in stop mode for channel 6"]
pub struct EDREQ_6_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable asynchronous DMA request for channel 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_6_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_6_A::_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_7_A {
    #[doc = "0: Disable asynchronous DMA request for channel 7."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 7."]
    _1 = 1,
}
impl From<EDREQ_7_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDREQ_7` reader - Enable asynchronous DMA request in stop mode for channel 7"]
pub struct EDREQ_7_R(crate::FieldReader<bool, EDREQ_7_A>);
impl EDREQ_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDREQ_7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_7_A {
        match self.bits {
            false => EDREQ_7_A::_0,
            true => EDREQ_7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EDREQ_7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EDREQ_7_A::_1
    }
}
impl core::ops::Deref for EDREQ_7_R {
    type Target = crate::FieldReader<bool, EDREQ_7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDREQ_7` writer - Enable asynchronous DMA request in stop mode for channel 7"]
pub struct EDREQ_7_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable asynchronous DMA request for channel 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_7_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_7_A::_1)
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
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub fn edreq_0(&self) -> EDREQ_0_R {
        EDREQ_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    pub fn edreq_1(&self) -> EDREQ_1_R {
        EDREQ_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    pub fn edreq_2(&self) -> EDREQ_2_R {
        EDREQ_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    pub fn edreq_3(&self) -> EDREQ_3_R {
        EDREQ_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
    #[inline(always)]
    pub fn edreq_4(&self) -> EDREQ_4_R {
        EDREQ_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
    #[inline(always)]
    pub fn edreq_5(&self) -> EDREQ_5_R {
        EDREQ_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
    #[inline(always)]
    pub fn edreq_6(&self) -> EDREQ_6_R {
        EDREQ_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
    #[inline(always)]
    pub fn edreq_7(&self) -> EDREQ_7_R {
        EDREQ_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub fn edreq_0(&mut self) -> EDREQ_0_W {
        EDREQ_0_W { w: self }
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    pub fn edreq_1(&mut self) -> EDREQ_1_W {
        EDREQ_1_W { w: self }
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    pub fn edreq_2(&mut self) -> EDREQ_2_W {
        EDREQ_2_W { w: self }
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    pub fn edreq_3(&mut self) -> EDREQ_3_W {
        EDREQ_3_W { w: self }
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
    #[inline(always)]
    pub fn edreq_4(&mut self) -> EDREQ_4_W {
        EDREQ_4_W { w: self }
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
    #[inline(always)]
    pub fn edreq_5(&mut self) -> EDREQ_5_W {
        EDREQ_5_W { w: self }
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
    #[inline(always)]
    pub fn edreq_6(&mut self) -> EDREQ_6_W {
        EDREQ_6_W { w: self }
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
    #[inline(always)]
    pub fn edreq_7(&mut self) -> EDREQ_7_W {
        EDREQ_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Asynchronous Request in Stop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ears](index.html) module"]
pub struct EARS_SPEC;
impl crate::RegisterSpec for EARS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ears::R](R) reader structure"]
impl crate::Readable for EARS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ears::W](W) writer structure"]
impl crate::Writable for EARS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EARS to value 0"]
impl crate::Resettable for EARS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
