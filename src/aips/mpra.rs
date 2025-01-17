#[doc = "Register `MPRA` reader"]
pub struct R(crate::R<MPRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPRA` writer"]
pub struct W(crate::W<MPRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPRA_SPEC>;
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
impl From<crate::W<MPRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master 4 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL4_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL4_A> for bool {
    #[inline(always)]
    fn from(variant: MPL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPL4` reader - Master 4 Privilege Level"]
pub struct MPL4_R(crate::FieldReader<bool, MPL4_A>);
impl MPL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL4_A {
        match self.bits {
            false => MPL4_A::_0,
            true => MPL4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MPL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MPL4_A::_1
    }
}
impl core::ops::Deref for MPL4_R {
    type Target = crate::FieldReader<bool, MPL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPL4` writer - Master 4 Privilege Level"]
pub struct MPL4_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL4_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL4_A::_1)
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
#[doc = "Master 4 Trusted For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW4_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW4_A> for bool {
    #[inline(always)]
    fn from(variant: MTW4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTW4` reader - Master 4 Trusted For Writes"]
pub struct MTW4_R(crate::FieldReader<bool, MTW4_A>);
impl MTW4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTW4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW4_A {
        match self.bits {
            false => MTW4_A::_0,
            true => MTW4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTW4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTW4_A::_1
    }
}
impl core::ops::Deref for MTW4_R {
    type Target = crate::FieldReader<bool, MTW4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTW4` writer - Master 4 Trusted For Writes"]
pub struct MTW4_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW4_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW4_A::_1)
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
#[doc = "Master 4 Trusted For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR4_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR4_A> for bool {
    #[inline(always)]
    fn from(variant: MTR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTR4` reader - Master 4 Trusted For Read"]
pub struct MTR4_R(crate::FieldReader<bool, MTR4_A>);
impl MTR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTR4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR4_A {
        match self.bits {
            false => MTR4_A::_0,
            true => MTR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTR4_A::_1
    }
}
impl core::ops::Deref for MTR4_R {
    type Target = crate::FieldReader<bool, MTR4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTR4` writer - Master 4 Trusted For Read"]
pub struct MTR4_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR4_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR4_A::_1)
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
#[doc = "Master 3 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL3_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL3_A> for bool {
    #[inline(always)]
    fn from(variant: MPL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPL3` reader - Master 3 Privilege Level"]
pub struct MPL3_R(crate::FieldReader<bool, MPL3_A>);
impl MPL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL3_A {
        match self.bits {
            false => MPL3_A::_0,
            true => MPL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MPL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MPL3_A::_1
    }
}
impl core::ops::Deref for MPL3_R {
    type Target = crate::FieldReader<bool, MPL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPL3` writer - Master 3 Privilege Level"]
pub struct MPL3_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL3_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL3_A::_1)
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
#[doc = "Master 3 Trusted For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW3_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW3_A> for bool {
    #[inline(always)]
    fn from(variant: MTW3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTW3` reader - Master 3 Trusted For Writes"]
pub struct MTW3_R(crate::FieldReader<bool, MTW3_A>);
impl MTW3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTW3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW3_A {
        match self.bits {
            false => MTW3_A::_0,
            true => MTW3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTW3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTW3_A::_1
    }
}
impl core::ops::Deref for MTW3_R {
    type Target = crate::FieldReader<bool, MTW3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTW3` writer - Master 3 Trusted For Writes"]
pub struct MTW3_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW3_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW3_A::_1)
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
#[doc = "Master 3 Trusted For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR3_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR3_A> for bool {
    #[inline(always)]
    fn from(variant: MTR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTR3` reader - Master 3 Trusted For Read"]
pub struct MTR3_R(crate::FieldReader<bool, MTR3_A>);
impl MTR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR3_A {
        match self.bits {
            false => MTR3_A::_0,
            true => MTR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTR3_A::_1
    }
}
impl core::ops::Deref for MTR3_R {
    type Target = crate::FieldReader<bool, MTR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTR3` writer - Master 3 Trusted For Read"]
pub struct MTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR3_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR3_A::_1)
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
#[doc = "Master 2 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL2_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL2_A> for bool {
    #[inline(always)]
    fn from(variant: MPL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPL2` reader - Master 2 Privilege Level"]
pub struct MPL2_R(crate::FieldReader<bool, MPL2_A>);
impl MPL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL2_A {
        match self.bits {
            false => MPL2_A::_0,
            true => MPL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MPL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MPL2_A::_1
    }
}
impl core::ops::Deref for MPL2_R {
    type Target = crate::FieldReader<bool, MPL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPL2` writer - Master 2 Privilege Level"]
pub struct MPL2_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL2_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL2_A::_1)
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
#[doc = "Master 2 Trusted For Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW2_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW2_A> for bool {
    #[inline(always)]
    fn from(variant: MTW2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTW2` reader - Master 2 Trusted For Writes"]
pub struct MTW2_R(crate::FieldReader<bool, MTW2_A>);
impl MTW2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTW2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW2_A {
        match self.bits {
            false => MTW2_A::_0,
            true => MTW2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTW2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTW2_A::_1
    }
}
impl core::ops::Deref for MTW2_R {
    type Target = crate::FieldReader<bool, MTW2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTW2` writer - Master 2 Trusted For Writes"]
pub struct MTW2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW2_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Master 2 Trusted For Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR2_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR2_A> for bool {
    #[inline(always)]
    fn from(variant: MTR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTR2` reader - Master 2 Trusted For Read"]
pub struct MTR2_R(crate::FieldReader<bool, MTR2_A>);
impl MTR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR2_A {
        match self.bits {
            false => MTR2_A::_0,
            true => MTR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTR2_A::_1
    }
}
impl core::ops::Deref for MTR2_R {
    type Target = crate::FieldReader<bool, MTR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTR2` writer - Master 2 Trusted For Read"]
pub struct MTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR2_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Master 1 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL1_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL1_A> for bool {
    #[inline(always)]
    fn from(variant: MPL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPL1` reader - Master 1 Privilege Level"]
pub struct MPL1_R(crate::FieldReader<bool, MPL1_A>);
impl MPL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL1_A {
        match self.bits {
            false => MPL1_A::_0,
            true => MPL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MPL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MPL1_A::_1
    }
}
impl core::ops::Deref for MPL1_R {
    type Target = crate::FieldReader<bool, MPL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPL1` writer - Master 1 Privilege Level"]
pub struct MPL1_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL1_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL1_A::_1)
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
#[doc = "Master 1 Trusted for Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW1_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW1_A> for bool {
    #[inline(always)]
    fn from(variant: MTW1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTW1` reader - Master 1 Trusted for Writes"]
pub struct MTW1_R(crate::FieldReader<bool, MTW1_A>);
impl MTW1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTW1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW1_A {
        match self.bits {
            false => MTW1_A::_0,
            true => MTW1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTW1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTW1_A::_1
    }
}
impl core::ops::Deref for MTW1_R {
    type Target = crate::FieldReader<bool, MTW1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTW1` writer - Master 1 Trusted for Writes"]
pub struct MTW1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW1_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW1_A::_1)
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
#[doc = "Master 1 Trusted for Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR1_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR1_A> for bool {
    #[inline(always)]
    fn from(variant: MTR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTR1` reader - Master 1 Trusted for Read"]
pub struct MTR1_R(crate::FieldReader<bool, MTR1_A>);
impl MTR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR1_A {
        match self.bits {
            false => MTR1_A::_0,
            true => MTR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTR1_A::_1
    }
}
impl core::ops::Deref for MTR1_R {
    type Target = crate::FieldReader<bool, MTR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTR1` writer - Master 1 Trusted for Read"]
pub struct MTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR1_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR1_A::_1)
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
#[doc = "Master 0 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL0_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL0_A> for bool {
    #[inline(always)]
    fn from(variant: MPL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPL0` reader - Master 0 Privilege Level"]
pub struct MPL0_R(crate::FieldReader<bool, MPL0_A>);
impl MPL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL0_A {
        match self.bits {
            false => MPL0_A::_0,
            true => MPL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MPL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MPL0_A::_1
    }
}
impl core::ops::Deref for MPL0_R {
    type Target = crate::FieldReader<bool, MPL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPL0` writer - Master 0 Privilege Level"]
pub struct MPL0_W<'a> {
    w: &'a mut W,
}
impl<'a> MPL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL0_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL0_A::_1)
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
#[doc = "Master 0 Trusted For Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW0_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW0_A> for bool {
    #[inline(always)]
    fn from(variant: MTW0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTW0` reader - Master 0 Trusted For Writes"]
pub struct MTW0_R(crate::FieldReader<bool, MTW0_A>);
impl MTW0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTW0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW0_A {
        match self.bits {
            false => MTW0_A::_0,
            true => MTW0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTW0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTW0_A::_1
    }
}
impl core::ops::Deref for MTW0_R {
    type Target = crate::FieldReader<bool, MTW0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTW0` writer - Master 0 Trusted For Writes"]
pub struct MTW0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTW0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW0_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW0_A::_1)
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
#[doc = "Master 0 Trusted For Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR0_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR0_A> for bool {
    #[inline(always)]
    fn from(variant: MTR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTR0` reader - Master 0 Trusted For Read"]
pub struct MTR0_R(crate::FieldReader<bool, MTR0_A>);
impl MTR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR0_A {
        match self.bits {
            false => MTR0_A::_0,
            true => MTR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTR0_A::_1
    }
}
impl core::ops::Deref for MTR0_R {
    type Target = crate::FieldReader<bool, MTR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTR0` writer - Master 0 Trusted For Read"]
pub struct MTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR0_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR0_A::_1)
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
impl R {
    #[doc = "Bit 12 - Master 4 Privilege Level"]
    #[inline(always)]
    pub fn mpl4(&self) -> MPL4_R {
        MPL4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Master 4 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw4(&self) -> MTW4_R {
        MTW4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Master 4 Trusted For Read"]
    #[inline(always)]
    pub fn mtr4(&self) -> MTR4_R {
        MTR4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Master 3 Privilege Level"]
    #[inline(always)]
    pub fn mpl3(&self) -> MPL3_R {
        MPL3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Master 3 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw3(&self) -> MTW3_R {
        MTW3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Master 3 Trusted For Read"]
    #[inline(always)]
    pub fn mtr3(&self) -> MTR3_R {
        MTR3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline(always)]
    pub fn mpl2(&self) -> MPL2_R {
        MPL2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw2(&self) -> MTW2_R {
        MTW2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline(always)]
    pub fn mtr2(&self) -> MTR2_R {
        MTR2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline(always)]
    pub fn mpl1(&self) -> MPL1_R {
        MPL1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw1(&self) -> MTW1_R {
        MTW1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline(always)]
    pub fn mtr1(&self) -> MTR1_R {
        MTR1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline(always)]
    pub fn mpl0(&self) -> MPL0_R {
        MPL0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw0(&self) -> MTW0_R {
        MTW0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline(always)]
    pub fn mtr0(&self) -> MTR0_R {
        MTR0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Master 4 Privilege Level"]
    #[inline(always)]
    pub fn mpl4(&mut self) -> MPL4_W {
        MPL4_W { w: self }
    }
    #[doc = "Bit 13 - Master 4 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw4(&mut self) -> MTW4_W {
        MTW4_W { w: self }
    }
    #[doc = "Bit 14 - Master 4 Trusted For Read"]
    #[inline(always)]
    pub fn mtr4(&mut self) -> MTR4_W {
        MTR4_W { w: self }
    }
    #[doc = "Bit 16 - Master 3 Privilege Level"]
    #[inline(always)]
    pub fn mpl3(&mut self) -> MPL3_W {
        MPL3_W { w: self }
    }
    #[doc = "Bit 17 - Master 3 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw3(&mut self) -> MTW3_W {
        MTW3_W { w: self }
    }
    #[doc = "Bit 18 - Master 3 Trusted For Read"]
    #[inline(always)]
    pub fn mtr3(&mut self) -> MTR3_W {
        MTR3_W { w: self }
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline(always)]
    pub fn mpl2(&mut self) -> MPL2_W {
        MPL2_W { w: self }
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw2(&mut self) -> MTW2_W {
        MTW2_W { w: self }
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline(always)]
    pub fn mtr2(&mut self) -> MTR2_W {
        MTR2_W { w: self }
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline(always)]
    pub fn mpl1(&mut self) -> MPL1_W {
        MPL1_W { w: self }
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw1(&mut self) -> MTW1_W {
        MTW1_W { w: self }
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline(always)]
    pub fn mtr1(&mut self) -> MTR1_W {
        MTR1_W { w: self }
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline(always)]
    pub fn mpl0(&mut self) -> MPL0_W {
        MPL0_W { w: self }
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw0(&mut self) -> MTW0_W {
        MTW0_W { w: self }
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline(always)]
    pub fn mtr0(&mut self) -> MTR0_W {
        MTR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Privilege Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpra](index.html) module"]
pub struct MPRA_SPEC;
impl crate::RegisterSpec for MPRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpra::R](R) reader structure"]
impl crate::Readable for MPRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpra::W](W) writer structure"]
impl crate::Writable for MPRA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPRA to value 0x7770_0000"]
impl crate::Resettable for MPRA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7770_0000
    }
}
