#[doc = "Register `PF4` reader"]
pub struct R(crate::R<PF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF4` writer"]
pub struct W(crate::W<PF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF4_SPEC>;
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
impl From<crate::W<PF4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Flag For LLWU_P24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF24_A {
    #[doc = "0: LLWU_P24 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P24 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF24_A> for bool {
    #[inline(always)]
    fn from(variant: WUF24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF24` reader - Wakeup Flag For LLWU_P24"]
pub struct WUF24_R(crate::FieldReader<bool, WUF24_A>);
impl WUF24_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF24_A {
        match self.bits {
            false => WUF24_A::_0,
            true => WUF24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF24_A::_1
    }
}
impl core::ops::Deref for WUF24_R {
    type Target = crate::FieldReader<bool, WUF24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF24` writer - Wakeup Flag For LLWU_P24"]
pub struct WUF24_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P24 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF24_A::_0)
    }
    #[doc = "LLWU_P24 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF24_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF25_A {
    #[doc = "0: LLWU_P25 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P25 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF25_A> for bool {
    #[inline(always)]
    fn from(variant: WUF25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF25` reader - Wakeup Flag For LLWU_P25"]
pub struct WUF25_R(crate::FieldReader<bool, WUF25_A>);
impl WUF25_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF25_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF25_A {
        match self.bits {
            false => WUF25_A::_0,
            true => WUF25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF25_A::_1
    }
}
impl core::ops::Deref for WUF25_R {
    type Target = crate::FieldReader<bool, WUF25_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF25` writer - Wakeup Flag For LLWU_P25"]
pub struct WUF25_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P25 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF25_A::_0)
    }
    #[doc = "LLWU_P25 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF25_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF26_A {
    #[doc = "0: LLWU_P26 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P26 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF26_A> for bool {
    #[inline(always)]
    fn from(variant: WUF26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF26` reader - Wakeup Flag For LLWU_P26"]
pub struct WUF26_R(crate::FieldReader<bool, WUF26_A>);
impl WUF26_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF26_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF26_A {
        match self.bits {
            false => WUF26_A::_0,
            true => WUF26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF26_A::_1
    }
}
impl core::ops::Deref for WUF26_R {
    type Target = crate::FieldReader<bool, WUF26_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF26` writer - Wakeup Flag For LLWU_P26"]
pub struct WUF26_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P26 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF26_A::_0)
    }
    #[doc = "LLWU_P26 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF26_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF27_A {
    #[doc = "0: LLWU_P27 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P27 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF27_A> for bool {
    #[inline(always)]
    fn from(variant: WUF27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF27` reader - Wakeup Flag For LLWU_P27"]
pub struct WUF27_R(crate::FieldReader<bool, WUF27_A>);
impl WUF27_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF27_A {
        match self.bits {
            false => WUF27_A::_0,
            true => WUF27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF27_A::_1
    }
}
impl core::ops::Deref for WUF27_R {
    type Target = crate::FieldReader<bool, WUF27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF27` writer - Wakeup Flag For LLWU_P27"]
pub struct WUF27_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P27 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF27_A::_0)
    }
    #[doc = "LLWU_P27 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF27_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF28_A {
    #[doc = "0: LLWU_P28 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P28 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF28_A> for bool {
    #[inline(always)]
    fn from(variant: WUF28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF28` reader - Wakeup Flag For LLWU_P28"]
pub struct WUF28_R(crate::FieldReader<bool, WUF28_A>);
impl WUF28_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF28_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF28_A {
        match self.bits {
            false => WUF28_A::_0,
            true => WUF28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF28_A::_1
    }
}
impl core::ops::Deref for WUF28_R {
    type Target = crate::FieldReader<bool, WUF28_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF28` writer - Wakeup Flag For LLWU_P28"]
pub struct WUF28_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P28 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF28_A::_0)
    }
    #[doc = "LLWU_P28 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF28_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF29_A {
    #[doc = "0: LLWU_P29 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P29 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF29_A> for bool {
    #[inline(always)]
    fn from(variant: WUF29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF29` reader - Wakeup Flag For LLWU_P29"]
pub struct WUF29_R(crate::FieldReader<bool, WUF29_A>);
impl WUF29_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF29_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF29_A {
        match self.bits {
            false => WUF29_A::_0,
            true => WUF29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF29_A::_1
    }
}
impl core::ops::Deref for WUF29_R {
    type Target = crate::FieldReader<bool, WUF29_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF29` writer - Wakeup Flag For LLWU_P29"]
pub struct WUF29_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P29 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF29_A::_0)
    }
    #[doc = "LLWU_P29 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF29_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF30_A {
    #[doc = "0: LLWU_P30 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P30 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF30_A> for bool {
    #[inline(always)]
    fn from(variant: WUF30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF30` reader - Wakeup Flag For LLWU_P30"]
pub struct WUF30_R(crate::FieldReader<bool, WUF30_A>);
impl WUF30_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF30_A {
        match self.bits {
            false => WUF30_A::_0,
            true => WUF30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF30_A::_1
    }
}
impl core::ops::Deref for WUF30_R {
    type Target = crate::FieldReader<bool, WUF30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF30` writer - Wakeup Flag For LLWU_P30"]
pub struct WUF30_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P30 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF30_A::_0)
    }
    #[doc = "LLWU_P30 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF30_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF31_A {
    #[doc = "0: LLWU_P31 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P31 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF31_A> for bool {
    #[inline(always)]
    fn from(variant: WUF31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF31` reader - Wakeup Flag For LLWU_P31"]
pub struct WUF31_R(crate::FieldReader<bool, WUF31_A>);
impl WUF31_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF31_A {
        match self.bits {
            false => WUF31_A::_0,
            true => WUF31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF31_A::_1
    }
}
impl core::ops::Deref for WUF31_R {
    type Target = crate::FieldReader<bool, WUF31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF31` writer - Wakeup Flag For LLWU_P31"]
pub struct WUF31_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P31 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF31_A::_0)
    }
    #[doc = "LLWU_P31 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF31_A::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P24"]
    #[inline(always)]
    pub fn wuf24(&self) -> WUF24_R {
        WUF24_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P25"]
    #[inline(always)]
    pub fn wuf25(&self) -> WUF25_R {
        WUF25_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P26"]
    #[inline(always)]
    pub fn wuf26(&self) -> WUF26_R {
        WUF26_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P27"]
    #[inline(always)]
    pub fn wuf27(&self) -> WUF27_R {
        WUF27_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P28"]
    #[inline(always)]
    pub fn wuf28(&self) -> WUF28_R {
        WUF28_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P29"]
    #[inline(always)]
    pub fn wuf29(&self) -> WUF29_R {
        WUF29_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P30"]
    #[inline(always)]
    pub fn wuf30(&self) -> WUF30_R {
        WUF30_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P31"]
    #[inline(always)]
    pub fn wuf31(&self) -> WUF31_R {
        WUF31_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P24"]
    #[inline(always)]
    pub fn wuf24(&mut self) -> WUF24_W {
        WUF24_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P25"]
    #[inline(always)]
    pub fn wuf25(&mut self) -> WUF25_W {
        WUF25_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P26"]
    #[inline(always)]
    pub fn wuf26(&mut self) -> WUF26_W {
        WUF26_W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P27"]
    #[inline(always)]
    pub fn wuf27(&mut self) -> WUF27_W {
        WUF27_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P28"]
    #[inline(always)]
    pub fn wuf28(&mut self) -> WUF28_W {
        WUF28_W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P29"]
    #[inline(always)]
    pub fn wuf29(&mut self) -> WUF29_W {
        WUF29_W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P30"]
    #[inline(always)]
    pub fn wuf30(&mut self) -> WUF30_W {
        WUF30_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P31"]
    #[inline(always)]
    pub fn wuf31(&mut self) -> WUF31_W {
        WUF31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Flag 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf4](index.html) module"]
pub struct PF4_SPEC;
impl crate::RegisterSpec for PF4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pf4::R](R) reader structure"]
impl crate::Readable for PF4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf4::W](W) writer structure"]
impl crate::Writable for PF4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF4 to value 0"]
impl crate::Resettable for PF4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
