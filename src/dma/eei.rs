#[doc = "Register `EEI` reader"]
pub struct R(crate::R<EEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEI` writer"]
pub struct W(crate::W<EEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEI_SPEC>;
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
impl From<crate::W<EEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable Error Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI0_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI0_A> for bool {
    #[inline(always)]
    fn from(variant: EEI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI0` reader - Enable Error Interrupt 0"]
pub struct EEI0_R(crate::FieldReader<bool, EEI0_A>);
impl EEI0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI0_A {
        match self.bits {
            false => EEI0_A::_0,
            true => EEI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EEI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EEI0_A::_1
    }
}
impl core::ops::Deref for EEI0_R {
    type Target = crate::FieldReader<bool, EEI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI0` writer - Enable Error Interrupt 0"]
pub struct EEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI0_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI0_A::_1)
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
#[doc = "Enable Error Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI1_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI1_A> for bool {
    #[inline(always)]
    fn from(variant: EEI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI1` reader - Enable Error Interrupt 1"]
pub struct EEI1_R(crate::FieldReader<bool, EEI1_A>);
impl EEI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI1_A {
        match self.bits {
            false => EEI1_A::_0,
            true => EEI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EEI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EEI1_A::_1
    }
}
impl core::ops::Deref for EEI1_R {
    type Target = crate::FieldReader<bool, EEI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI1` writer - Enable Error Interrupt 1"]
pub struct EEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI1_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI1_A::_1)
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
#[doc = "Enable Error Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI2_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI2_A> for bool {
    #[inline(always)]
    fn from(variant: EEI2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI2` reader - Enable Error Interrupt 2"]
pub struct EEI2_R(crate::FieldReader<bool, EEI2_A>);
impl EEI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEI2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI2_A {
        match self.bits {
            false => EEI2_A::_0,
            true => EEI2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EEI2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EEI2_A::_1
    }
}
impl core::ops::Deref for EEI2_R {
    type Target = crate::FieldReader<bool, EEI2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI2` writer - Enable Error Interrupt 2"]
pub struct EEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI2_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI2_A::_1)
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
#[doc = "Enable Error Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI3_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI3_A> for bool {
    #[inline(always)]
    fn from(variant: EEI3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI3` reader - Enable Error Interrupt 3"]
pub struct EEI3_R(crate::FieldReader<bool, EEI3_A>);
impl EEI3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEI3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI3_A {
        match self.bits {
            false => EEI3_A::_0,
            true => EEI3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EEI3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EEI3_A::_1
    }
}
impl core::ops::Deref for EEI3_R {
    type Target = crate::FieldReader<bool, EEI3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI3` writer - Enable Error Interrupt 3"]
pub struct EEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI3_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI3_A::_1)
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
#[doc = "Enable Error Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI4_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI4_A> for bool {
    #[inline(always)]
    fn from(variant: EEI4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI4` reader - Enable Error Interrupt 4"]
pub struct EEI4_R(crate::FieldReader<bool, EEI4_A>);
impl EEI4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEI4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI4_A {
        match self.bits {
            false => EEI4_A::_0,
            true => EEI4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EEI4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EEI4_A::_1
    }
}
impl core::ops::Deref for EEI4_R {
    type Target = crate::FieldReader<bool, EEI4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI4` writer - Enable Error Interrupt 4"]
pub struct EEI4_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI4_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI4_A::_1)
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
#[doc = "Enable Error Interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI5_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI5_A> for bool {
    #[inline(always)]
    fn from(variant: EEI5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI5` reader - Enable Error Interrupt 5"]
pub struct EEI5_R(crate::FieldReader<bool, EEI5_A>);
impl EEI5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEI5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI5_A {
        match self.bits {
            false => EEI5_A::_0,
            true => EEI5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EEI5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EEI5_A::_1
    }
}
impl core::ops::Deref for EEI5_R {
    type Target = crate::FieldReader<bool, EEI5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI5` writer - Enable Error Interrupt 5"]
pub struct EEI5_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI5_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI5_A::_1)
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
#[doc = "Enable Error Interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI6_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI6_A> for bool {
    #[inline(always)]
    fn from(variant: EEI6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI6` reader - Enable Error Interrupt 6"]
pub struct EEI6_R(crate::FieldReader<bool, EEI6_A>);
impl EEI6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEI6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI6_A {
        match self.bits {
            false => EEI6_A::_0,
            true => EEI6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EEI6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EEI6_A::_1
    }
}
impl core::ops::Deref for EEI6_R {
    type Target = crate::FieldReader<bool, EEI6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI6` writer - Enable Error Interrupt 6"]
pub struct EEI6_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI6_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI6_A::_1)
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
#[doc = "Enable Error Interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI7_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI7_A> for bool {
    #[inline(always)]
    fn from(variant: EEI7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI7` reader - Enable Error Interrupt 7"]
pub struct EEI7_R(crate::FieldReader<bool, EEI7_A>);
impl EEI7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEI7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI7_A {
        match self.bits {
            false => EEI7_A::_0,
            true => EEI7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EEI7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EEI7_A::_1
    }
}
impl core::ops::Deref for EEI7_R {
    type Target = crate::FieldReader<bool, EEI7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI7` writer - Enable Error Interrupt 7"]
pub struct EEI7_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI7_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI7_A::_1)
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
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&self) -> EEI0_R {
        EEI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&self) -> EEI1_R {
        EEI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&self) -> EEI2_R {
        EEI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&self) -> EEI3_R {
        EEI3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    pub fn eei4(&self) -> EEI4_R {
        EEI4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    pub fn eei5(&self) -> EEI5_R {
        EEI5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    pub fn eei6(&self) -> EEI6_R {
        EEI6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    pub fn eei7(&self) -> EEI7_R {
        EEI7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&mut self) -> EEI0_W {
        EEI0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&mut self) -> EEI1_W {
        EEI1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&mut self) -> EEI2_W {
        EEI2_W { w: self }
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&mut self) -> EEI3_W {
        EEI3_W { w: self }
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    pub fn eei4(&mut self) -> EEI4_W {
        EEI4_W { w: self }
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    pub fn eei5(&mut self) -> EEI5_W {
        EEI5_W { w: self }
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    pub fn eei6(&mut self) -> EEI6_W {
        EEI6_W { w: self }
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    pub fn eei7(&mut self) -> EEI7_W {
        EEI7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eei](index.html) module"]
pub struct EEI_SPEC;
impl crate::RegisterSpec for EEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eei::R](R) reader structure"]
impl crate::Readable for EEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eei::W](W) writer structure"]
impl crate::Writable for EEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEI to value 0"]
impl crate::Resettable for EEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
