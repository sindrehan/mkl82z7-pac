#[doc = "Register `PACRH` reader"]
pub struct R(crate::R<PACRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PACRH` writer"]
pub struct W(crate::W<PACRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PACRH_SPEC>;
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
impl From<crate::W<PACRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PACRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP7_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP7_A> for bool {
    #[inline(always)]
    fn from(variant: TP7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP7` reader - Trusted Protect"]
pub struct TP7_R(crate::FieldReader<bool, TP7_A>);
impl TP7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP7_A {
        match self.bits {
            false => TP7_A::_0,
            true => TP7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP7_A::_1
    }
}
impl core::ops::Deref for TP7_R {
    type Target = crate::FieldReader<bool, TP7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP7` writer - Trusted Protect"]
pub struct TP7_W<'a> {
    w: &'a mut W,
}
impl<'a> TP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP7_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP7_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP7_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP7_A> for bool {
    #[inline(always)]
    fn from(variant: WP7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP7` reader - Write Protect"]
pub struct WP7_R(crate::FieldReader<bool, WP7_A>);
impl WP7_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP7_A {
        match self.bits {
            false => WP7_A::_0,
            true => WP7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP7_A::_1
    }
}
impl core::ops::Deref for WP7_R {
    type Target = crate::FieldReader<bool, WP7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP7` writer - Write Protect"]
pub struct WP7_W<'a> {
    w: &'a mut W,
}
impl<'a> WP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP7_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP7_A::_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP7_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP7_A> for bool {
    #[inline(always)]
    fn from(variant: SP7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP7` reader - Supervisor Protect"]
pub struct SP7_R(crate::FieldReader<bool, SP7_A>);
impl SP7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP7_A {
        match self.bits {
            false => SP7_A::_0,
            true => SP7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP7_A::_1
    }
}
impl core::ops::Deref for SP7_R {
    type Target = crate::FieldReader<bool, SP7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP7` writer - Supervisor Protect"]
pub struct SP7_W<'a> {
    w: &'a mut W,
}
impl<'a> SP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP7_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP7_A::_1)
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
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP6_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP6_A> for bool {
    #[inline(always)]
    fn from(variant: TP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP6` reader - Trusted Protect"]
pub struct TP6_R(crate::FieldReader<bool, TP6_A>);
impl TP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP6_A {
        match self.bits {
            false => TP6_A::_0,
            true => TP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP6_A::_1
    }
}
impl core::ops::Deref for TP6_R {
    type Target = crate::FieldReader<bool, TP6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP6` writer - Trusted Protect"]
pub struct TP6_W<'a> {
    w: &'a mut W,
}
impl<'a> TP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP6_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP6_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP6_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP6_A> for bool {
    #[inline(always)]
    fn from(variant: WP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP6` reader - Write Protect"]
pub struct WP6_R(crate::FieldReader<bool, WP6_A>);
impl WP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP6_A {
        match self.bits {
            false => WP6_A::_0,
            true => WP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP6_A::_1
    }
}
impl core::ops::Deref for WP6_R {
    type Target = crate::FieldReader<bool, WP6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP6` writer - Write Protect"]
pub struct WP6_W<'a> {
    w: &'a mut W,
}
impl<'a> WP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP6_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP6_A::_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP6_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP6_A> for bool {
    #[inline(always)]
    fn from(variant: SP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP6` reader - Supervisor Protect"]
pub struct SP6_R(crate::FieldReader<bool, SP6_A>);
impl SP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP6_A {
        match self.bits {
            false => SP6_A::_0,
            true => SP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP6_A::_1
    }
}
impl core::ops::Deref for SP6_R {
    type Target = crate::FieldReader<bool, SP6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP6` writer - Supervisor Protect"]
pub struct SP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP6_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP6_A::_1)
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
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP5_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP5_A> for bool {
    #[inline(always)]
    fn from(variant: TP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP5` reader - Trusted Protect"]
pub struct TP5_R(crate::FieldReader<bool, TP5_A>);
impl TP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP5_A {
        match self.bits {
            false => TP5_A::_0,
            true => TP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP5_A::_1
    }
}
impl core::ops::Deref for TP5_R {
    type Target = crate::FieldReader<bool, TP5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP5` writer - Trusted Protect"]
pub struct TP5_W<'a> {
    w: &'a mut W,
}
impl<'a> TP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP5_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP5_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP5_A> for bool {
    #[inline(always)]
    fn from(variant: WP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP5` reader - Write Protect"]
pub struct WP5_R(crate::FieldReader<bool, WP5_A>);
impl WP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP5_A {
        match self.bits {
            false => WP5_A::_0,
            true => WP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP5_A::_1
    }
}
impl core::ops::Deref for WP5_R {
    type Target = crate::FieldReader<bool, WP5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP5` writer - Write Protect"]
pub struct WP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP5_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Supervisor Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP5_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP5_A> for bool {
    #[inline(always)]
    fn from(variant: SP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP5` reader - Supervisor Protect"]
pub struct SP5_R(crate::FieldReader<bool, SP5_A>);
impl SP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP5_A {
        match self.bits {
            false => SP5_A::_0,
            true => SP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP5_A::_1
    }
}
impl core::ops::Deref for SP5_R {
    type Target = crate::FieldReader<bool, SP5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP5` writer - Supervisor Protect"]
pub struct SP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP5_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP4_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP4_A> for bool {
    #[inline(always)]
    fn from(variant: TP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP4` reader - Trusted Protect"]
pub struct TP4_R(crate::FieldReader<bool, TP4_A>);
impl TP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP4_A {
        match self.bits {
            false => TP4_A::_0,
            true => TP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP4_A::_1
    }
}
impl core::ops::Deref for TP4_R {
    type Target = crate::FieldReader<bool, TP4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP4` writer - Trusted Protect"]
pub struct TP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP4_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP4_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP4_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP4_A> for bool {
    #[inline(always)]
    fn from(variant: WP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP4` reader - Write Protect"]
pub struct WP4_R(crate::FieldReader<bool, WP4_A>);
impl WP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP4_A {
        match self.bits {
            false => WP4_A::_0,
            true => WP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP4_A::_1
    }
}
impl core::ops::Deref for WP4_R {
    type Target = crate::FieldReader<bool, WP4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP4` writer - Write Protect"]
pub struct WP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP4_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP4_A::_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP4_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP4_A> for bool {
    #[inline(always)]
    fn from(variant: SP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP4` reader - Supervisor Protect"]
pub struct SP4_R(crate::FieldReader<bool, SP4_A>);
impl SP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP4_A {
        match self.bits {
            false => SP4_A::_0,
            true => SP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP4_A::_1
    }
}
impl core::ops::Deref for SP4_R {
    type Target = crate::FieldReader<bool, SP4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP4` writer - Supervisor Protect"]
pub struct SP4_W<'a> {
    w: &'a mut W,
}
impl<'a> SP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP4_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP4_A::_1)
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
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP3_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP3_A> for bool {
    #[inline(always)]
    fn from(variant: TP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP3` reader - Trusted Protect"]
pub struct TP3_R(crate::FieldReader<bool, TP3_A>);
impl TP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP3_A {
        match self.bits {
            false => TP3_A::_0,
            true => TP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP3_A::_1
    }
}
impl core::ops::Deref for TP3_R {
    type Target = crate::FieldReader<bool, TP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP3` writer - Trusted Protect"]
pub struct TP3_W<'a> {
    w: &'a mut W,
}
impl<'a> TP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP3_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP3_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP3_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP3_A> for bool {
    #[inline(always)]
    fn from(variant: WP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP3` reader - Write Protect"]
pub struct WP3_R(crate::FieldReader<bool, WP3_A>);
impl WP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP3_A {
        match self.bits {
            false => WP3_A::_0,
            true => WP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP3_A::_1
    }
}
impl core::ops::Deref for WP3_R {
    type Target = crate::FieldReader<bool, WP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP3` writer - Write Protect"]
pub struct WP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP3_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP3_A::_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP3_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP3_A> for bool {
    #[inline(always)]
    fn from(variant: SP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP3` reader - Supervisor Protect"]
pub struct SP3_R(crate::FieldReader<bool, SP3_A>);
impl SP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP3_A {
        match self.bits {
            false => SP3_A::_0,
            true => SP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP3_A::_1
    }
}
impl core::ops::Deref for SP3_R {
    type Target = crate::FieldReader<bool, SP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP3` writer - Supervisor Protect"]
pub struct SP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP3_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP3_A::_1)
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
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP2_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP2_A> for bool {
    #[inline(always)]
    fn from(variant: TP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP2` reader - Trusted Protect"]
pub struct TP2_R(crate::FieldReader<bool, TP2_A>);
impl TP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP2_A {
        match self.bits {
            false => TP2_A::_0,
            true => TP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP2_A::_1
    }
}
impl core::ops::Deref for TP2_R {
    type Target = crate::FieldReader<bool, TP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP2` writer - Trusted Protect"]
pub struct TP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP2_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP2_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP2_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP2_A> for bool {
    #[inline(always)]
    fn from(variant: WP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP2` reader - Write Protect"]
pub struct WP2_R(crate::FieldReader<bool, WP2_A>);
impl WP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP2_A {
        match self.bits {
            false => WP2_A::_0,
            true => WP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP2_A::_1
    }
}
impl core::ops::Deref for WP2_R {
    type Target = crate::FieldReader<bool, WP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP2` writer - Write Protect"]
pub struct WP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP2_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP2_A::_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP2_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP2_A> for bool {
    #[inline(always)]
    fn from(variant: SP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP2` reader - Supervisor Protect"]
pub struct SP2_R(crate::FieldReader<bool, SP2_A>);
impl SP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP2_A {
        match self.bits {
            false => SP2_A::_0,
            true => SP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP2_A::_1
    }
}
impl core::ops::Deref for SP2_R {
    type Target = crate::FieldReader<bool, SP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP2` writer - Supervisor Protect"]
pub struct SP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP2_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP2_A::_1)
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
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP1_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP1_A> for bool {
    #[inline(always)]
    fn from(variant: TP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP1` reader - Trusted Protect"]
pub struct TP1_R(crate::FieldReader<bool, TP1_A>);
impl TP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP1_A {
        match self.bits {
            false => TP1_A::_0,
            true => TP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP1_A::_1
    }
}
impl core::ops::Deref for TP1_R {
    type Target = crate::FieldReader<bool, TP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP1` writer - Trusted Protect"]
pub struct TP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP1_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP1_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP1_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP1_A> for bool {
    #[inline(always)]
    fn from(variant: WP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP1` reader - Write Protect"]
pub struct WP1_R(crate::FieldReader<bool, WP1_A>);
impl WP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP1_A {
        match self.bits {
            false => WP1_A::_0,
            true => WP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP1_A::_1
    }
}
impl core::ops::Deref for WP1_R {
    type Target = crate::FieldReader<bool, WP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP1` writer - Write Protect"]
pub struct WP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP1_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP1_A::_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP1_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP1_A> for bool {
    #[inline(always)]
    fn from(variant: SP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP1` reader - Supervisor Protect"]
pub struct SP1_R(crate::FieldReader<bool, SP1_A>);
impl SP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP1_A {
        match self.bits {
            false => SP1_A::_0,
            true => SP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP1_A::_1
    }
}
impl core::ops::Deref for SP1_R {
    type Target = crate::FieldReader<bool, SP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP1` writer - Supervisor Protect"]
pub struct SP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP1_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP1_A::_1)
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
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP0_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP0_A> for bool {
    #[inline(always)]
    fn from(variant: TP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP0` reader - Trusted Protect"]
pub struct TP0_R(crate::FieldReader<bool, TP0_A>);
impl TP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP0_A {
        match self.bits {
            false => TP0_A::_0,
            true => TP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TP0_A::_1
    }
}
impl core::ops::Deref for TP0_R {
    type Target = crate::FieldReader<bool, TP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP0` writer - Trusted Protect"]
pub struct TP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP0_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP0_A::_1)
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
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP0_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP0_A> for bool {
    #[inline(always)]
    fn from(variant: WP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP0` reader - Write Protect"]
pub struct WP0_R(crate::FieldReader<bool, WP0_A>);
impl WP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP0_A {
        match self.bits {
            false => WP0_A::_0,
            true => WP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WP0_A::_1
    }
}
impl core::ops::Deref for WP0_R {
    type Target = crate::FieldReader<bool, WP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP0` writer - Write Protect"]
pub struct WP0_W<'a> {
    w: &'a mut W,
}
impl<'a> WP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP0_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP0_A::_1)
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
#[doc = "Supervisor Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP0_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP0_A> for bool {
    #[inline(always)]
    fn from(variant: SP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP0` reader - Supervisor Protect"]
pub struct SP0_R(crate::FieldReader<bool, SP0_A>);
impl SP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP0_A {
        match self.bits {
            false => SP0_A::_0,
            true => SP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SP0_A::_1
    }
}
impl core::ops::Deref for SP0_R {
    type Target = crate::FieldReader<bool, SP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP0` writer - Supervisor Protect"]
pub struct SP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP0_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP0_A::_1)
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
    #[doc = "Bit 0 - Trusted Protect"]
    #[inline(always)]
    pub fn tp7(&self) -> TP7_R {
        TP7_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Protect"]
    #[inline(always)]
    pub fn wp7(&self) -> WP7_R {
        WP7_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp7(&self) -> SP7_R {
        SP7_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline(always)]
    pub fn tp6(&self) -> TP6_R {
        TP6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline(always)]
    pub fn wp6(&self) -> WP6_R {
        WP6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp6(&self) -> SP6_R {
        SP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline(always)]
    pub fn tp5(&self) -> TP5_R {
        TP5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp5(&self) -> SP5_R {
        SP5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Trusted Protect"]
    #[inline(always)]
    pub fn tp4(&self) -> TP4_R {
        TP4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write Protect"]
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp4(&self) -> SP4_R {
        SP4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline(always)]
    pub fn tp3(&self) -> TP3_R {
        TP3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp3(&self) -> SP3_R {
        SP3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Trusted Protect"]
    #[inline(always)]
    pub fn tp2(&self) -> TP2_R {
        TP2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write Protect"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp2(&self) -> SP2_R {
        SP2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline(always)]
    pub fn tp1(&self) -> TP1_R {
        TP1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp1(&self) -> SP1_R {
        SP1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline(always)]
    pub fn tp0(&self) -> TP0_R {
        TP0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline(always)]
    pub fn wp0(&self) -> WP0_R {
        WP0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp0(&self) -> SP0_R {
        SP0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trusted Protect"]
    #[inline(always)]
    pub fn tp7(&mut self) -> TP7_W {
        TP7_W { w: self }
    }
    #[doc = "Bit 1 - Write Protect"]
    #[inline(always)]
    pub fn wp7(&mut self) -> WP7_W {
        WP7_W { w: self }
    }
    #[doc = "Bit 2 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp7(&mut self) -> SP7_W {
        SP7_W { w: self }
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline(always)]
    pub fn tp6(&mut self) -> TP6_W {
        TP6_W { w: self }
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline(always)]
    pub fn wp6(&mut self) -> WP6_W {
        WP6_W { w: self }
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp6(&mut self) -> SP6_W {
        SP6_W { w: self }
    }
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline(always)]
    pub fn tp5(&mut self) -> TP5_W {
        TP5_W { w: self }
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline(always)]
    pub fn wp5(&mut self) -> WP5_W {
        WP5_W { w: self }
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp5(&mut self) -> SP5_W {
        SP5_W { w: self }
    }
    #[doc = "Bit 12 - Trusted Protect"]
    #[inline(always)]
    pub fn tp4(&mut self) -> TP4_W {
        TP4_W { w: self }
    }
    #[doc = "Bit 13 - Write Protect"]
    #[inline(always)]
    pub fn wp4(&mut self) -> WP4_W {
        WP4_W { w: self }
    }
    #[doc = "Bit 14 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp4(&mut self) -> SP4_W {
        SP4_W { w: self }
    }
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline(always)]
    pub fn tp3(&mut self) -> TP3_W {
        TP3_W { w: self }
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline(always)]
    pub fn wp3(&mut self) -> WP3_W {
        WP3_W { w: self }
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp3(&mut self) -> SP3_W {
        SP3_W { w: self }
    }
    #[doc = "Bit 20 - Trusted Protect"]
    #[inline(always)]
    pub fn tp2(&mut self) -> TP2_W {
        TP2_W { w: self }
    }
    #[doc = "Bit 21 - Write Protect"]
    #[inline(always)]
    pub fn wp2(&mut self) -> WP2_W {
        WP2_W { w: self }
    }
    #[doc = "Bit 22 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp2(&mut self) -> SP2_W {
        SP2_W { w: self }
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline(always)]
    pub fn tp1(&mut self) -> TP1_W {
        TP1_W { w: self }
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline(always)]
    pub fn wp1(&mut self) -> WP1_W {
        WP1_W { w: self }
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp1(&mut self) -> SP1_W {
        SP1_W { w: self }
    }
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline(always)]
    pub fn tp0(&mut self) -> TP0_W {
        TP0_W { w: self }
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline(always)]
    pub fn wp0(&mut self) -> WP0_W {
        WP0_W { w: self }
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp0(&mut self) -> SP0_W {
        SP0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacrh](index.html) module"]
pub struct PACRH_SPEC;
impl crate::RegisterSpec for PACRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pacrh::R](R) reader structure"]
impl crate::Readable for PACRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pacrh::W](W) writer structure"]
impl crate::Writable for PACRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PACRH to value 0"]
impl crate::Resettable for PACRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
