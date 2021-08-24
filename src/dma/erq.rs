#[doc = "Register `ERQ` reader"]
pub struct R(crate::R<ERQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERQ` writer"]
pub struct W(crate::W<ERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERQ_SPEC>;
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
impl From<crate::W<ERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable DMA Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ0_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ0_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ0` reader - Enable DMA Request 0"]
pub struct ERQ0_R(crate::FieldReader<bool, ERQ0_A>);
impl ERQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERQ0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ0_A {
        match self.bits {
            false => ERQ0_A::_0,
            true => ERQ0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERQ0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERQ0_A::_1
    }
}
impl core::ops::Deref for ERQ0_R {
    type Target = crate::FieldReader<bool, ERQ0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ0` writer - Enable DMA Request 0"]
pub struct ERQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ0_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ0_A::_1)
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
#[doc = "Enable DMA Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ1_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ1_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ1` reader - Enable DMA Request 1"]
pub struct ERQ1_R(crate::FieldReader<bool, ERQ1_A>);
impl ERQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERQ1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ1_A {
        match self.bits {
            false => ERQ1_A::_0,
            true => ERQ1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERQ1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERQ1_A::_1
    }
}
impl core::ops::Deref for ERQ1_R {
    type Target = crate::FieldReader<bool, ERQ1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ1` writer - Enable DMA Request 1"]
pub struct ERQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ1_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ1_A::_1)
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
#[doc = "Enable DMA Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ2_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ2_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ2` reader - Enable DMA Request 2"]
pub struct ERQ2_R(crate::FieldReader<bool, ERQ2_A>);
impl ERQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERQ2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ2_A {
        match self.bits {
            false => ERQ2_A::_0,
            true => ERQ2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERQ2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERQ2_A::_1
    }
}
impl core::ops::Deref for ERQ2_R {
    type Target = crate::FieldReader<bool, ERQ2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ2` writer - Enable DMA Request 2"]
pub struct ERQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ2_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ2_A::_1)
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
#[doc = "Enable DMA Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ3_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ3_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ3` reader - Enable DMA Request 3"]
pub struct ERQ3_R(crate::FieldReader<bool, ERQ3_A>);
impl ERQ3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERQ3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ3_A {
        match self.bits {
            false => ERQ3_A::_0,
            true => ERQ3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERQ3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERQ3_A::_1
    }
}
impl core::ops::Deref for ERQ3_R {
    type Target = crate::FieldReader<bool, ERQ3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ3` writer - Enable DMA Request 3"]
pub struct ERQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ3_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ3_A::_1)
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
#[doc = "Enable DMA Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ4_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ4_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ4` reader - Enable DMA Request 4"]
pub struct ERQ4_R(crate::FieldReader<bool, ERQ4_A>);
impl ERQ4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERQ4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ4_A {
        match self.bits {
            false => ERQ4_A::_0,
            true => ERQ4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERQ4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERQ4_A::_1
    }
}
impl core::ops::Deref for ERQ4_R {
    type Target = crate::FieldReader<bool, ERQ4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ4` writer - Enable DMA Request 4"]
pub struct ERQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ4_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ4_A::_1)
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
#[doc = "Enable DMA Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ5_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ5_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ5` reader - Enable DMA Request 5"]
pub struct ERQ5_R(crate::FieldReader<bool, ERQ5_A>);
impl ERQ5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERQ5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ5_A {
        match self.bits {
            false => ERQ5_A::_0,
            true => ERQ5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERQ5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERQ5_A::_1
    }
}
impl core::ops::Deref for ERQ5_R {
    type Target = crate::FieldReader<bool, ERQ5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ5` writer - Enable DMA Request 5"]
pub struct ERQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ5_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ5_A::_1)
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
#[doc = "Enable DMA Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ6_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ6_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ6` reader - Enable DMA Request 6"]
pub struct ERQ6_R(crate::FieldReader<bool, ERQ6_A>);
impl ERQ6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERQ6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ6_A {
        match self.bits {
            false => ERQ6_A::_0,
            true => ERQ6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERQ6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERQ6_A::_1
    }
}
impl core::ops::Deref for ERQ6_R {
    type Target = crate::FieldReader<bool, ERQ6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ6` writer - Enable DMA Request 6"]
pub struct ERQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ6_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ6_A::_1)
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
#[doc = "Enable DMA Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ7_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ7_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ7` reader - Enable DMA Request 7"]
pub struct ERQ7_R(crate::FieldReader<bool, ERQ7_A>);
impl ERQ7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERQ7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ7_A {
        match self.bits {
            false => ERQ7_A::_0,
            true => ERQ7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERQ7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERQ7_A::_1
    }
}
impl core::ops::Deref for ERQ7_R {
    type Target = crate::FieldReader<bool, ERQ7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ7` writer - Enable DMA Request 7"]
pub struct ERQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ7_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ7_A::_1)
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&self) -> ERQ0_R {
        ERQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&self) -> ERQ1_R {
        ERQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&self) -> ERQ2_R {
        ERQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&self) -> ERQ3_R {
        ERQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    pub fn erq4(&self) -> ERQ4_R {
        ERQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    pub fn erq5(&self) -> ERQ5_R {
        ERQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    pub fn erq6(&self) -> ERQ6_R {
        ERQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    pub fn erq7(&self) -> ERQ7_R {
        ERQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&mut self) -> ERQ0_W {
        ERQ0_W { w: self }
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&mut self) -> ERQ1_W {
        ERQ1_W { w: self }
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&mut self) -> ERQ2_W {
        ERQ2_W { w: self }
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&mut self) -> ERQ3_W {
        ERQ3_W { w: self }
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    pub fn erq4(&mut self) -> ERQ4_W {
        ERQ4_W { w: self }
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    pub fn erq5(&mut self) -> ERQ5_W {
        ERQ5_W { w: self }
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    pub fn erq6(&mut self) -> ERQ6_W {
        ERQ6_W { w: self }
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    pub fn erq7(&mut self) -> ERQ7_W {
        ERQ7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erq](index.html) module"]
pub struct ERQ_SPEC;
impl crate::RegisterSpec for ERQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erq::R](R) reader structure"]
impl crate::Readable for ERQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erq::W](W) writer structure"]
impl crate::Writable for ERQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERQ to value 0"]
impl crate::Resettable for ERQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
