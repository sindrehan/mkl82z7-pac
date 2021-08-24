#[doc = "Register `PF3` reader"]
pub struct R(crate::R<PF3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF3` writer"]
pub struct W(crate::W<PF3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF3_SPEC>;
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
impl From<crate::W<PF3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Flag For LLWU_P16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF16_A {
    #[doc = "0: LLWU_P16 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P16 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF16_A> for bool {
    #[inline(always)]
    fn from(variant: WUF16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF16` reader - Wakeup Flag For LLWU_P16"]
pub struct WUF16_R(crate::FieldReader<bool, WUF16_A>);
impl WUF16_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF16_A {
        match self.bits {
            false => WUF16_A::_0,
            true => WUF16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF16_A::_1
    }
}
impl core::ops::Deref for WUF16_R {
    type Target = crate::FieldReader<bool, WUF16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF16` writer - Wakeup Flag For LLWU_P16"]
pub struct WUF16_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P16 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF16_A::_0)
    }
    #[doc = "LLWU_P16 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF16_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF17_A {
    #[doc = "0: LLWU_P17 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P17 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF17_A> for bool {
    #[inline(always)]
    fn from(variant: WUF17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF17` reader - Wakeup Flag For LLWU_P17"]
pub struct WUF17_R(crate::FieldReader<bool, WUF17_A>);
impl WUF17_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF17_A {
        match self.bits {
            false => WUF17_A::_0,
            true => WUF17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF17_A::_1
    }
}
impl core::ops::Deref for WUF17_R {
    type Target = crate::FieldReader<bool, WUF17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF17` writer - Wakeup Flag For LLWU_P17"]
pub struct WUF17_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P17 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF17_A::_0)
    }
    #[doc = "LLWU_P17 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF17_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF18_A {
    #[doc = "0: LLWU_P18 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P18 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF18_A> for bool {
    #[inline(always)]
    fn from(variant: WUF18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF18` reader - Wakeup Flag For LLWU_P18"]
pub struct WUF18_R(crate::FieldReader<bool, WUF18_A>);
impl WUF18_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF18_A {
        match self.bits {
            false => WUF18_A::_0,
            true => WUF18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF18_A::_1
    }
}
impl core::ops::Deref for WUF18_R {
    type Target = crate::FieldReader<bool, WUF18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF18` writer - Wakeup Flag For LLWU_P18"]
pub struct WUF18_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P18 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF18_A::_0)
    }
    #[doc = "LLWU_P18 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF18_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF19_A {
    #[doc = "0: LLWU_P19 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P19 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF19_A> for bool {
    #[inline(always)]
    fn from(variant: WUF19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF19` reader - Wakeup Flag For LLWU_P19"]
pub struct WUF19_R(crate::FieldReader<bool, WUF19_A>);
impl WUF19_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF19_A {
        match self.bits {
            false => WUF19_A::_0,
            true => WUF19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF19_A::_1
    }
}
impl core::ops::Deref for WUF19_R {
    type Target = crate::FieldReader<bool, WUF19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF19` writer - Wakeup Flag For LLWU_P19"]
pub struct WUF19_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P19 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF19_A::_0)
    }
    #[doc = "LLWU_P19 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF19_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF20_A {
    #[doc = "0: LLWU_P20 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P20 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF20_A> for bool {
    #[inline(always)]
    fn from(variant: WUF20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF20` reader - Wakeup Flag For LLWU_P20"]
pub struct WUF20_R(crate::FieldReader<bool, WUF20_A>);
impl WUF20_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF20_A {
        match self.bits {
            false => WUF20_A::_0,
            true => WUF20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF20_A::_1
    }
}
impl core::ops::Deref for WUF20_R {
    type Target = crate::FieldReader<bool, WUF20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF20` writer - Wakeup Flag For LLWU_P20"]
pub struct WUF20_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P20 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF20_A::_0)
    }
    #[doc = "LLWU_P20 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF20_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF21_A {
    #[doc = "0: LLWU_P21 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P21 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF21_A> for bool {
    #[inline(always)]
    fn from(variant: WUF21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF21` reader - Wakeup Flag For LLWU_P21"]
pub struct WUF21_R(crate::FieldReader<bool, WUF21_A>);
impl WUF21_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF21_A {
        match self.bits {
            false => WUF21_A::_0,
            true => WUF21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF21_A::_1
    }
}
impl core::ops::Deref for WUF21_R {
    type Target = crate::FieldReader<bool, WUF21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF21` writer - Wakeup Flag For LLWU_P21"]
pub struct WUF21_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P21 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF21_A::_0)
    }
    #[doc = "LLWU_P21 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF21_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF22_A {
    #[doc = "0: LLWU_P22 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P22 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF22_A> for bool {
    #[inline(always)]
    fn from(variant: WUF22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF22` reader - Wakeup Flag For LLWU_P22"]
pub struct WUF22_R(crate::FieldReader<bool, WUF22_A>);
impl WUF22_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF22_A {
        match self.bits {
            false => WUF22_A::_0,
            true => WUF22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF22_A::_1
    }
}
impl core::ops::Deref for WUF22_R {
    type Target = crate::FieldReader<bool, WUF22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF22` writer - Wakeup Flag For LLWU_P22"]
pub struct WUF22_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P22 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF22_A::_0)
    }
    #[doc = "LLWU_P22 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF22_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF23_A {
    #[doc = "0: LLWU_P23 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P23 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF23_A> for bool {
    #[inline(always)]
    fn from(variant: WUF23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF23` reader - Wakeup Flag For LLWU_P23"]
pub struct WUF23_R(crate::FieldReader<bool, WUF23_A>);
impl WUF23_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF23_A {
        match self.bits {
            false => WUF23_A::_0,
            true => WUF23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUF23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUF23_A::_1
    }
}
impl core::ops::Deref for WUF23_R {
    type Target = crate::FieldReader<bool, WUF23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUF23` writer - Wakeup Flag For LLWU_P23"]
pub struct WUF23_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LLWU_P23 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF23_A::_0)
    }
    #[doc = "LLWU_P23 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF23_A::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P16"]
    #[inline(always)]
    pub fn wuf16(&self) -> WUF16_R {
        WUF16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P17"]
    #[inline(always)]
    pub fn wuf17(&self) -> WUF17_R {
        WUF17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P18"]
    #[inline(always)]
    pub fn wuf18(&self) -> WUF18_R {
        WUF18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P19"]
    #[inline(always)]
    pub fn wuf19(&self) -> WUF19_R {
        WUF19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P20"]
    #[inline(always)]
    pub fn wuf20(&self) -> WUF20_R {
        WUF20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P21"]
    #[inline(always)]
    pub fn wuf21(&self) -> WUF21_R {
        WUF21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P22"]
    #[inline(always)]
    pub fn wuf22(&self) -> WUF22_R {
        WUF22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P23"]
    #[inline(always)]
    pub fn wuf23(&self) -> WUF23_R {
        WUF23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P16"]
    #[inline(always)]
    pub fn wuf16(&mut self) -> WUF16_W {
        WUF16_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P17"]
    #[inline(always)]
    pub fn wuf17(&mut self) -> WUF17_W {
        WUF17_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P18"]
    #[inline(always)]
    pub fn wuf18(&mut self) -> WUF18_W {
        WUF18_W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P19"]
    #[inline(always)]
    pub fn wuf19(&mut self) -> WUF19_W {
        WUF19_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P20"]
    #[inline(always)]
    pub fn wuf20(&mut self) -> WUF20_W {
        WUF20_W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P21"]
    #[inline(always)]
    pub fn wuf21(&mut self) -> WUF21_W {
        WUF21_W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P22"]
    #[inline(always)]
    pub fn wuf22(&mut self) -> WUF22_W {
        WUF22_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P23"]
    #[inline(always)]
    pub fn wuf23(&mut self) -> WUF23_W {
        WUF23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Flag 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf3](index.html) module"]
pub struct PF3_SPEC;
impl crate::RegisterSpec for PF3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pf3::R](R) reader structure"]
impl crate::Readable for PF3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf3::W](W) writer structure"]
impl crate::Writable for PF3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF3 to value 0"]
impl crate::Resettable for PF3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
