#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Buffer Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_LV_A {
    #[doc = "0: Bandgap on only, for stabilization and startup"]
    _00 = 0,
    #[doc = "1: High power buffer mode enabled"]
    _01 = 1,
    #[doc = "2: Low-power buffer mode enabled"]
    _10 = 2,
}
impl From<MODE_LV_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_LV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE_LV` reader - Buffer Mode selection"]
pub struct MODE_LV_R(crate::FieldReader<u8, MODE_LV_A>);
impl MODE_LV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_LV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_LV_A> {
        match self.bits {
            0 => Some(MODE_LV_A::_00),
            1 => Some(MODE_LV_A::_01),
            2 => Some(MODE_LV_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == MODE_LV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == MODE_LV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == MODE_LV_A::_10
    }
}
impl core::ops::Deref for MODE_LV_R {
    type Target = crate::FieldReader<u8, MODE_LV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_LV` writer - Buffer Mode selection"]
pub struct MODE_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_LV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_LV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bandgap on only, for stabilization and startup"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODE_LV_A::_00)
    }
    #[doc = "High power buffer mode enabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODE_LV_A::_01)
    }
    #[doc = "Low-power buffer mode enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODE_LV_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Internal Voltage Reference stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFST_A {
    #[doc = "0: The module is disabled or not stable."]
    _0 = 0,
    #[doc = "1: The module is stable."]
    _1 = 1,
}
impl From<VREFST_A> for bool {
    #[inline(always)]
    fn from(variant: VREFST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFST` reader - Internal Voltage Reference stable"]
pub struct VREFST_R(crate::FieldReader<bool, VREFST_A>);
impl VREFST_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFST_A {
        match self.bits {
            false => VREFST_A::_0,
            true => VREFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VREFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VREFST_A::_1
    }
}
impl core::ops::Deref for VREFST_R {
    type Target = crate::FieldReader<bool, VREFST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Second order curvature compensation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICOMPEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<ICOMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICOMPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICOMPEN` reader - Second order curvature compensation enable"]
pub struct ICOMPEN_R(crate::FieldReader<bool, ICOMPEN_A>);
impl ICOMPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICOMPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICOMPEN_A {
        match self.bits {
            false => ICOMPEN_A::_0,
            true => ICOMPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ICOMPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ICOMPEN_A::_1
    }
}
impl core::ops::Deref for ICOMPEN_R {
    type Target = crate::FieldReader<bool, ICOMPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICOMPEN` writer - Second order curvature compensation enable"]
pub struct ICOMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICOMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICOMPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICOMPEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICOMPEN_A::_1)
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
#[doc = "Regulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGEN_A {
    #[doc = "0: Internal 1.75 V regulator is disabled."]
    _0 = 0,
    #[doc = "1: Internal 1.75 V regulator is enabled."]
    _1 = 1,
}
impl From<REGEN_A> for bool {
    #[inline(always)]
    fn from(variant: REGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGEN` reader - Regulator enable"]
pub struct REGEN_R(crate::FieldReader<bool, REGEN_A>);
impl REGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGEN_A {
        match self.bits {
            false => REGEN_A::_0,
            true => REGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REGEN_A::_1
    }
}
impl core::ops::Deref for REGEN_R {
    type Target = crate::FieldReader<bool, REGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGEN` writer - Regulator enable"]
pub struct REGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal 1.75 V regulator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REGEN_A::_0)
    }
    #[doc = "Internal 1.75 V regulator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REGEN_A::_1)
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
#[doc = "Internal Voltage Reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
    #[doc = "0: The module is disabled."]
    _0 = 0,
    #[doc = "1: The module is enabled."]
    _1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - Internal Voltage Reference enable"]
pub struct VREFEN_R(crate::FieldReader<bool, VREFEN_A>);
impl VREFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::_0,
            true => VREFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VREFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VREFEN_A::_1
    }
}
impl core::ops::Deref for VREFEN_R {
    type Target = crate::FieldReader<bool, VREFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFEN` writer - Internal Voltage Reference enable"]
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFEN_A::_0)
    }
    #[doc = "The module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFEN_A::_1)
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
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline(always)]
    pub fn mode_lv(&self) -> MODE_LV_R {
        MODE_LV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Internal Voltage Reference stable"]
    #[inline(always)]
    pub fn vrefst(&self) -> VREFST_R {
        VREFST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Second order curvature compensation enable"]
    #[inline(always)]
    pub fn icompen(&self) -> ICOMPEN_R {
        ICOMPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline(always)]
    pub fn mode_lv(&mut self) -> MODE_LV_W {
        MODE_LV_W { w: self }
    }
    #[doc = "Bit 5 - Second order curvature compensation enable"]
    #[inline(always)]
    pub fn icompen(&mut self) -> ICOMPEN_W {
        ICOMPEN_W { w: self }
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline(always)]
    pub fn regen(&mut self) -> REGEN_W {
        REGEN_W { w: self }
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
