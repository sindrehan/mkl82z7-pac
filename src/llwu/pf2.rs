#[doc = "Register `PF2` reader"]
pub struct R(crate::R<PF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF2` writer"]
pub struct W(crate::W<PF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF2_SPEC>;
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
impl From<crate::W<PF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Flag For LLWU_P8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF8_A {
    #[doc = "0: LLWU_P8 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P8 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF8_A> for bool {
    #[inline(always)]
    fn from(variant: WUF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF8` reader - Wakeup Flag For LLWU_P8"]
pub struct WUF8_R(crate::FieldReader<bool, WUF8_A>);
impl WUF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF8_A {
        match self.bits {
            false => WUF8_A::_0,
            true => WUF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF8_A::_1
    }
}
impl core::ops::Deref for WUF8_R {
    type Target = crate::FieldReader<bool, WUF8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF8` writer - Wakeup Flag For LLWU_P8"]
pub struct WUF8_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P8 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF8_A::_0)
    }
    #[doc = "LLWU_P8 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF8_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF9_A {
    #[doc = "0: LLWU_P9 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P9 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF9_A> for bool {
    #[inline(always)]
    fn from(variant: WUF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF9` reader - Wakeup Flag For LLWU_P9"]
pub struct WUF9_R(crate::FieldReader<bool, WUF9_A>);
impl WUF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF9_A {
        match self.bits {
            false => WUF9_A::_0,
            true => WUF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF9_A::_1
    }
}
impl core::ops::Deref for WUF9_R {
    type Target = crate::FieldReader<bool, WUF9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF9` writer - Wakeup Flag For LLWU_P9"]
pub struct WUF9_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P9 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF9_A::_0)
    }
    #[doc = "LLWU_P9 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF9_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF10_A {
    #[doc = "0: LLWU_P10 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P10 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF10_A> for bool {
    #[inline(always)]
    fn from(variant: WUF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF10` reader - Wakeup Flag For LLWU_P10"]
pub struct WUF10_R(crate::FieldReader<bool, WUF10_A>);
impl WUF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF10_A {
        match self.bits {
            false => WUF10_A::_0,
            true => WUF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF10_A::_1
    }
}
impl core::ops::Deref for WUF10_R {
    type Target = crate::FieldReader<bool, WUF10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF10` writer - Wakeup Flag For LLWU_P10"]
pub struct WUF10_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P10 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF10_A::_0)
    }
    #[doc = "LLWU_P10 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF10_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF11_A {
    #[doc = "0: LLWU_P11 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P11 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF11_A> for bool {
    #[inline(always)]
    fn from(variant: WUF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF11` reader - Wakeup Flag For LLWU_P11"]
pub struct WUF11_R(crate::FieldReader<bool, WUF11_A>);
impl WUF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF11_A {
        match self.bits {
            false => WUF11_A::_0,
            true => WUF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF11_A::_1
    }
}
impl core::ops::Deref for WUF11_R {
    type Target = crate::FieldReader<bool, WUF11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF11` writer - Wakeup Flag For LLWU_P11"]
pub struct WUF11_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P11 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF11_A::_0)
    }
    #[doc = "LLWU_P11 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF11_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF12_A {
    #[doc = "0: LLWU_P12 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P12 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF12_A> for bool {
    #[inline(always)]
    fn from(variant: WUF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF12` reader - Wakeup Flag For LLWU_P12"]
pub struct WUF12_R(crate::FieldReader<bool, WUF12_A>);
impl WUF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF12_A {
        match self.bits {
            false => WUF12_A::_0,
            true => WUF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF12_A::_1
    }
}
impl core::ops::Deref for WUF12_R {
    type Target = crate::FieldReader<bool, WUF12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF12` writer - Wakeup Flag For LLWU_P12"]
pub struct WUF12_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P12 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF12_A::_0)
    }
    #[doc = "LLWU_P12 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF12_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF13_A {
    #[doc = "0: LLWU_P13 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P13 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF13_A> for bool {
    #[inline(always)]
    fn from(variant: WUF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF13` reader - Wakeup Flag For LLWU_P13"]
pub struct WUF13_R(crate::FieldReader<bool, WUF13_A>);
impl WUF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF13_A {
        match self.bits {
            false => WUF13_A::_0,
            true => WUF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF13_A::_1
    }
}
impl core::ops::Deref for WUF13_R {
    type Target = crate::FieldReader<bool, WUF13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF13` writer - Wakeup Flag For LLWU_P13"]
pub struct WUF13_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P13 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF13_A::_0)
    }
    #[doc = "LLWU_P13 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF13_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF14_A {
    #[doc = "0: LLWU_P14 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P14 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF14_A> for bool {
    #[inline(always)]
    fn from(variant: WUF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF14` reader - Wakeup Flag For LLWU_P14"]
pub struct WUF14_R(crate::FieldReader<bool, WUF14_A>);
impl WUF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF14_A {
        match self.bits {
            false => WUF14_A::_0,
            true => WUF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF14_A::_1
    }
}
impl core::ops::Deref for WUF14_R {
    type Target = crate::FieldReader<bool, WUF14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF14` writer - Wakeup Flag For LLWU_P14"]
pub struct WUF14_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P14 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF14_A::_0)
    }
    #[doc = "LLWU_P14 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF14_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF15_A {
    #[doc = "0: LLWU_P15 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P15 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF15_A> for bool {
    #[inline(always)]
    fn from(variant: WUF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF15` reader - Wakeup Flag For LLWU_P15"]
pub struct WUF15_R(crate::FieldReader<bool, WUF15_A>);
impl WUF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF15_A {
        match self.bits {
            false => WUF15_A::_0,
            true => WUF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF15_A::_1
    }
}
impl core::ops::Deref for WUF15_R {
    type Target = crate::FieldReader<bool, WUF15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF15` writer - Wakeup Flag For LLWU_P15"]
pub struct WUF15_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P15 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF15_A::_0)
    }
    #[doc = "LLWU_P15 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF15_A::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P8"]
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P9"]
    #[inline(always)]
    pub fn wuf9(&self) -> WUF9_R {
        WUF9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P10"]
    #[inline(always)]
    pub fn wuf10(&self) -> WUF10_R {
        WUF10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P11"]
    #[inline(always)]
    pub fn wuf11(&self) -> WUF11_R {
        WUF11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P12"]
    #[inline(always)]
    pub fn wuf12(&self) -> WUF12_R {
        WUF12_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P13"]
    #[inline(always)]
    pub fn wuf13(&self) -> WUF13_R {
        WUF13_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P14"]
    #[inline(always)]
    pub fn wuf14(&self) -> WUF14_R {
        WUF14_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P15"]
    #[inline(always)]
    pub fn wuf15(&self) -> WUF15_R {
        WUF15_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P8"]
    #[inline(always)]
    pub fn wuf8(&mut self) -> WUF8_W {
        WUF8_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P9"]
    #[inline(always)]
    pub fn wuf9(&mut self) -> WUF9_W {
        WUF9_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P10"]
    #[inline(always)]
    pub fn wuf10(&mut self) -> WUF10_W {
        WUF10_W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P11"]
    #[inline(always)]
    pub fn wuf11(&mut self) -> WUF11_W {
        WUF11_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P12"]
    #[inline(always)]
    pub fn wuf12(&mut self) -> WUF12_W {
        WUF12_W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P13"]
    #[inline(always)]
    pub fn wuf13(&mut self) -> WUF13_W {
        WUF13_W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P14"]
    #[inline(always)]
    pub fn wuf14(&mut self) -> WUF14_W {
        WUF14_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P15"]
    #[inline(always)]
    pub fn wuf15(&mut self) -> WUF15_W {
        WUF15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Flag 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf2](index.html) module"]
pub struct PF2_SPEC;
impl crate::RegisterSpec for PF2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pf2::R](R) reader structure"]
impl crate::Readable for PF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf2::W](W) writer structure"]
impl crate::Writable for PF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF2 to value 0"]
impl crate::Resettable for PF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
