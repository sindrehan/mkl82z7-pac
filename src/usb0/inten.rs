#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USBRST Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRSTEN_A {
    #[doc = "0: Disables the USBRST interrupt."]
    _0 = 0,
    #[doc = "1: Enables the USBRST interrupt."]
    _1 = 1,
}
impl From<USBRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRSTEN` reader - USBRST Interrupt Enable"]
pub struct USBRSTEN_R(crate::FieldReader<bool, USBRSTEN_A>);
impl USBRSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRSTEN_A {
        match self.bits {
            false => USBRSTEN_A::_0,
            true => USBRSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBRSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBRSTEN_A::_1
    }
}
impl core::ops::Deref for USBRSTEN_R {
    type Target = crate::FieldReader<bool, USBRSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRSTEN` writer - USBRST Interrupt Enable"]
pub struct USBRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the USBRST interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRSTEN_A::_0)
    }
    #[doc = "Enables the USBRST interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRSTEN_A::_1)
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
#[doc = "ERROR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROREN_A {
    #[doc = "0: Disables the ERROR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the ERROR interrupt."]
    _1 = 1,
}
impl From<ERROREN_A> for bool {
    #[inline(always)]
    fn from(variant: ERROREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROREN` reader - ERROR Interrupt Enable"]
pub struct ERROREN_R(crate::FieldReader<bool, ERROREN_A>);
impl ERROREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROREN_A {
        match self.bits {
            false => ERROREN_A::_0,
            true => ERROREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERROREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERROREN_A::_1
    }
}
impl core::ops::Deref for ERROREN_R {
    type Target = crate::FieldReader<bool, ERROREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROREN` writer - ERROR Interrupt Enable"]
pub struct ERROREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the ERROR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERROREN_A::_0)
    }
    #[doc = "Enables the ERROR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERROREN_A::_1)
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
#[doc = "SOFTOK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTOKEN_A {
    #[doc = "0: Disbles the SOFTOK interrupt."]
    _0 = 0,
    #[doc = "1: Enables the SOFTOK interrupt."]
    _1 = 1,
}
impl From<SOFTOKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTOKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTOKEN` reader - SOFTOK Interrupt Enable"]
pub struct SOFTOKEN_R(crate::FieldReader<bool, SOFTOKEN_A>);
impl SOFTOKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTOKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTOKEN_A {
        match self.bits {
            false => SOFTOKEN_A::_0,
            true => SOFTOKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOFTOKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOFTOKEN_A::_1
    }
}
impl core::ops::Deref for SOFTOKEN_R {
    type Target = crate::FieldReader<bool, SOFTOKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTOKEN` writer - SOFTOK Interrupt Enable"]
pub struct SOFTOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTOKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFTOKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disbles the SOFTOK interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTOKEN_A::_0)
    }
    #[doc = "Enables the SOFTOK interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTOKEN_A::_1)
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
#[doc = "TOKDNE Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOKDNEEN_A {
    #[doc = "0: Disables the TOKDNE interrupt."]
    _0 = 0,
    #[doc = "1: Enables the TOKDNE interrupt."]
    _1 = 1,
}
impl From<TOKDNEEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOKDNEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOKDNEEN` reader - TOKDNE Interrupt Enable"]
pub struct TOKDNEEN_R(crate::FieldReader<bool, TOKDNEEN_A>);
impl TOKDNEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOKDNEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOKDNEEN_A {
        match self.bits {
            false => TOKDNEEN_A::_0,
            true => TOKDNEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOKDNEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOKDNEEN_A::_1
    }
}
impl core::ops::Deref for TOKDNEEN_R {
    type Target = crate::FieldReader<bool, TOKDNEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOKDNEEN` writer - TOKDNE Interrupt Enable"]
pub struct TOKDNEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOKDNEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOKDNEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOKDNEEN_A::_0)
    }
    #[doc = "Enables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOKDNEEN_A::_1)
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
#[doc = "SLEEP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPEN_A {
    #[doc = "0: Disables the SLEEP interrupt."]
    _0 = 0,
    #[doc = "1: Enables the SLEEP interrupt."]
    _1 = 1,
}
impl From<SLEEPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEN` reader - SLEEP Interrupt Enable"]
pub struct SLEEPEN_R(crate::FieldReader<bool, SLEEPEN_A>);
impl SLEEPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPEN_A {
        match self.bits {
            false => SLEEPEN_A::_0,
            true => SLEEPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLEEPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLEEPEN_A::_1
    }
}
impl core::ops::Deref for SLEEPEN_R {
    type Target = crate::FieldReader<bool, SLEEPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPEN` writer - SLEEP Interrupt Enable"]
pub struct SLEEPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the SLEEP interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEPEN_A::_0)
    }
    #[doc = "Enables the SLEEP interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEPEN_A::_1)
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
#[doc = "RESUME Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUMEEN_A {
    #[doc = "0: Disables the RESUME interrupt."]
    _0 = 0,
    #[doc = "1: Enables the RESUME interrupt."]
    _1 = 1,
}
impl From<RESUMEEN_A> for bool {
    #[inline(always)]
    fn from(variant: RESUMEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUMEEN` reader - RESUME Interrupt Enable"]
pub struct RESUMEEN_R(crate::FieldReader<bool, RESUMEEN_A>);
impl RESUMEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUMEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESUMEEN_A {
        match self.bits {
            false => RESUMEEN_A::_0,
            true => RESUMEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RESUMEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RESUMEEN_A::_1
    }
}
impl core::ops::Deref for RESUMEEN_R {
    type Target = crate::FieldReader<bool, RESUMEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUMEEN` writer - RESUME Interrupt Enable"]
pub struct RESUMEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESUMEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the RESUME interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESUMEEN_A::_0)
    }
    #[doc = "Enables the RESUME interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESUMEEN_A::_1)
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
#[doc = "ATTACH Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATTACHEN_A {
    #[doc = "0: Disables the ATTACH interrupt."]
    _0 = 0,
    #[doc = "1: Enables the ATTACH interrupt."]
    _1 = 1,
}
impl From<ATTACHEN_A> for bool {
    #[inline(always)]
    fn from(variant: ATTACHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATTACHEN` reader - ATTACH Interrupt Enable"]
pub struct ATTACHEN_R(crate::FieldReader<bool, ATTACHEN_A>);
impl ATTACHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATTACHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATTACHEN_A {
        match self.bits {
            false => ATTACHEN_A::_0,
            true => ATTACHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ATTACHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ATTACHEN_A::_1
    }
}
impl core::ops::Deref for ATTACHEN_R {
    type Target = crate::FieldReader<bool, ATTACHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTACHEN` writer - ATTACH Interrupt Enable"]
pub struct ATTACHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTACHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATTACHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the ATTACH interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATTACHEN_A::_0)
    }
    #[doc = "Enables the ATTACH interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATTACHEN_A::_1)
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
#[doc = "STALL Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALLEN_A {
    #[doc = "0: Diasbles the STALL interrupt."]
    _0 = 0,
    #[doc = "1: Enables the STALL interrupt."]
    _1 = 1,
}
impl From<STALLEN_A> for bool {
    #[inline(always)]
    fn from(variant: STALLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALLEN` reader - STALL Interrupt Enable"]
pub struct STALLEN_R(crate::FieldReader<bool, STALLEN_A>);
impl STALLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALLEN_A {
        match self.bits {
            false => STALLEN_A::_0,
            true => STALLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STALLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STALLEN_A::_1
    }
}
impl core::ops::Deref for STALLEN_R {
    type Target = crate::FieldReader<bool, STALLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLEN` writer - STALL Interrupt Enable"]
pub struct STALLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Diasbles the STALL interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALLEN_A::_0)
    }
    #[doc = "Enables the STALL interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALLEN_A::_1)
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
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    pub fn usbrsten(&self) -> USBRSTEN_R {
        USBRSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn erroren(&self) -> ERROREN_R {
        ERROREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    pub fn softoken(&self) -> SOFTOKEN_R {
        SOFTOKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    pub fn tokdneen(&self) -> TOKDNEEN_R {
        TOKDNEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    pub fn sleepen(&self) -> SLEEPEN_R {
        SLEEPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&self) -> RESUMEEN_R {
        RESUMEEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline(always)]
    pub fn attachen(&self) -> ATTACHEN_R {
        ATTACHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    pub fn stallen(&self) -> STALLEN_R {
        STALLEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    pub fn usbrsten(&mut self) -> USBRSTEN_W {
        USBRSTEN_W { w: self }
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn erroren(&mut self) -> ERROREN_W {
        ERROREN_W { w: self }
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    pub fn softoken(&mut self) -> SOFTOKEN_W {
        SOFTOKEN_W { w: self }
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    pub fn tokdneen(&mut self) -> TOKDNEEN_W {
        TOKDNEEN_W { w: self }
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    pub fn sleepen(&mut self) -> SLEEPEN_W {
        SLEEPEN_W { w: self }
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&mut self) -> RESUMEEN_W {
        RESUMEEN_W { w: self }
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline(always)]
    pub fn attachen(&mut self) -> ATTACHEN_W {
        ATTACHEN_W { w: self }
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    pub fn stallen(&mut self) -> STALLEN_W {
        STALLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
