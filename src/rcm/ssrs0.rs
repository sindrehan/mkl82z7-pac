#[doc = "Register `SSRS0` reader"]
pub struct R(crate::R<SSRS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSRS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSRS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSRS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSRS0` writer"]
pub struct W(crate::W<SSRS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSRS0_SPEC>;
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
impl From<crate::W<SSRS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSRS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sticky Low Leakage Wakeup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAKEUP_A {
    #[doc = "0: Reset not caused by LLWU module wakeup source"]
    _0 = 0,
    #[doc = "1: Reset caused by LLWU module wakeup source"]
    _1 = 1,
}
impl From<SWAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAKEUP` reader - Sticky Low Leakage Wakeup Reset"]
pub struct SWAKEUP_R(crate::FieldReader<bool, SWAKEUP_A>);
impl SWAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWAKEUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAKEUP_A {
        match self.bits {
            false => SWAKEUP_A::_0,
            true => SWAKEUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWAKEUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWAKEUP_A::_1
    }
}
impl core::ops::Deref for SWAKEUP_R {
    type Target = crate::FieldReader<bool, SWAKEUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWAKEUP` writer - Sticky Low Leakage Wakeup Reset"]
pub struct SWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAKEUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWAKEUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWAKEUP_A::_0)
    }
    #[doc = "Reset caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWAKEUP_A::_1)
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
#[doc = "Sticky Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVD_A {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    _0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    _1 = 1,
}
impl From<SLVD_A> for bool {
    #[inline(always)]
    fn from(variant: SLVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVD` reader - Sticky Low-Voltage Detect Reset"]
pub struct SLVD_R(crate::FieldReader<bool, SLVD_A>);
impl SLVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVD_A {
        match self.bits {
            false => SLVD_A::_0,
            true => SLVD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLVD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLVD_A::_1
    }
}
impl core::ops::Deref for SLVD_R {
    type Target = crate::FieldReader<bool, SLVD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVD` writer - Sticky Low-Voltage Detect Reset"]
pub struct SLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVD_A::_0)
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVD_A::_1)
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
#[doc = "Sticky Loss-of-Clock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOC_A {
    #[doc = "0: Reset not caused by a loss of external clock."]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of external clock."]
    _1 = 1,
}
impl From<SLOC_A> for bool {
    #[inline(always)]
    fn from(variant: SLOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLOC` reader - Sticky Loss-of-Clock Reset"]
pub struct SLOC_R(crate::FieldReader<bool, SLOC_A>);
impl SLOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOC_A {
        match self.bits {
            false => SLOC_A::_0,
            true => SLOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLOC_A::_1
    }
}
impl core::ops::Deref for SLOC_R {
    type Target = crate::FieldReader<bool, SLOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOC` writer - Sticky Loss-of-Clock Reset"]
pub struct SLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by a loss of external clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOC_A::_0)
    }
    #[doc = "Reset caused by a loss of external clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOC_A::_1)
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
#[doc = "Sticky Loss-of-Lock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOL_A {
    #[doc = "0: Reset not caused by a loss of lock in the PLL"]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of lock in the PLL"]
    _1 = 1,
}
impl From<SLOL_A> for bool {
    #[inline(always)]
    fn from(variant: SLOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLOL` reader - Sticky Loss-of-Lock Reset"]
pub struct SLOL_R(crate::FieldReader<bool, SLOL_A>);
impl SLOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOL_A {
        match self.bits {
            false => SLOL_A::_0,
            true => SLOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLOL_A::_1
    }
}
impl core::ops::Deref for SLOL_R {
    type Target = crate::FieldReader<bool, SLOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOL` writer - Sticky Loss-of-Lock Reset"]
pub struct SLOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by a loss of lock in the PLL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOL_A::_0)
    }
    #[doc = "Reset caused by a loss of lock in the PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOL_A::_1)
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
#[doc = "Sticky Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDOG_A {
    #[doc = "0: Reset not caused by watchdog timeout"]
    _0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    _1 = 1,
}
impl From<SWDOG_A> for bool {
    #[inline(always)]
    fn from(variant: SWDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDOG` reader - Sticky Watchdog"]
pub struct SWDOG_R(crate::FieldReader<bool, SWDOG_A>);
impl SWDOG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWDOG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDOG_A {
        match self.bits {
            false => SWDOG_A::_0,
            true => SWDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWDOG_A::_1
    }
}
impl core::ops::Deref for SWDOG_R {
    type Target = crate::FieldReader<bool, SWDOG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWDOG` writer - Sticky Watchdog"]
pub struct SWDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWDOG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWDOG_A::_0)
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWDOG_A::_1)
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
#[doc = "Sticky External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    _0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    _1 = 1,
}
impl From<SPIN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIN` reader - Sticky External Reset Pin"]
pub struct SPIN_R(crate::FieldReader<bool, SPIN_A>);
impl SPIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIN_A {
        match self.bits {
            false => SPIN_A::_0,
            true => SPIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPIN_A::_1
    }
}
impl core::ops::Deref for SPIN_R {
    type Target = crate::FieldReader<bool, SPIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIN` writer - Sticky External Reset Pin"]
pub struct SPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIN_A::_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIN_A::_1)
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
#[doc = "Sticky Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOR_A {
    #[doc = "0: Reset not caused by POR"]
    _0 = 0,
    #[doc = "1: Reset caused by POR"]
    _1 = 1,
}
impl From<SPOR_A> for bool {
    #[inline(always)]
    fn from(variant: SPOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOR` reader - Sticky Power-On Reset"]
pub struct SPOR_R(crate::FieldReader<bool, SPOR_A>);
impl SPOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOR_A {
        match self.bits {
            false => SPOR_A::_0,
            true => SPOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPOR_A::_1
    }
}
impl core::ops::Deref for SPOR_R {
    type Target = crate::FieldReader<bool, SPOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPOR` writer - Sticky Power-On Reset"]
pub struct SPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPOR_A::_0)
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPOR_A::_1)
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
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn swakeup(&self) -> SWAKEUP_R {
        SWAKEUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&self) -> SLVD_R {
        SLVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn sloc(&self) -> SLOC_R {
        SLOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn slol(&self) -> SLOL_R {
        SLOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&self) -> SWDOG_R {
        SWDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&self) -> SPIN_R {
        SPIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&self) -> SPOR_R {
        SPOR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn swakeup(&mut self) -> SWAKEUP_W {
        SWAKEUP_W { w: self }
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&mut self) -> SLVD_W {
        SLVD_W { w: self }
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn sloc(&mut self) -> SLOC_W {
        SLOC_W { w: self }
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn slol(&mut self) -> SLOL_W {
        SLOL_W { w: self }
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&mut self) -> SWDOG_W {
        SWDOG_W { w: self }
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&mut self) -> SPIN_W {
        SPIN_W { w: self }
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&mut self) -> SPOR_W {
        SPOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sticky System Reset Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrs0](index.html) module"]
pub struct SSRS0_SPEC;
impl crate::RegisterSpec for SSRS0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ssrs0::R](R) reader structure"]
impl crate::Readable for SSRS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssrs0::W](W) writer structure"]
impl crate::Writable for SSRS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSRS0 to value 0x82"]
impl crate::Resettable for SSRS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x82
    }
}
