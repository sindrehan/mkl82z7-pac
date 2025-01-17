#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH0F_A> for bool {
    #[inline(always)]
    fn from(variant: CH0F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0F` reader - Channel 0 Flag"]
pub struct CH0F_R(crate::FieldReader<bool, CH0F_A>);
impl CH0F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0F_A {
        match self.bits {
            false => CH0F_A::_0,
            true => CH0F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH0F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH0F_A::_1
    }
}
impl core::ops::Deref for CH0F_R {
    type Target = crate::FieldReader<bool, CH0F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0F` writer - Channel 0 Flag"]
pub struct CH0F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0F_A::_1)
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
#[doc = "Channel 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH1F_A> for bool {
    #[inline(always)]
    fn from(variant: CH1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1F` reader - Channel 1 Flag"]
pub struct CH1F_R(crate::FieldReader<bool, CH1F_A>);
impl CH1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1F_A {
        match self.bits {
            false => CH1F_A::_0,
            true => CH1F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH1F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH1F_A::_1
    }
}
impl core::ops::Deref for CH1F_R {
    type Target = crate::FieldReader<bool, CH1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1F` writer - Channel 1 Flag"]
pub struct CH1F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1F_A::_1)
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
#[doc = "Channel 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH2F_A> for bool {
    #[inline(always)]
    fn from(variant: CH2F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2F` reader - Channel 2 Flag"]
pub struct CH2F_R(crate::FieldReader<bool, CH2F_A>);
impl CH2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2F_A {
        match self.bits {
            false => CH2F_A::_0,
            true => CH2F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH2F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH2F_A::_1
    }
}
impl core::ops::Deref for CH2F_R {
    type Target = crate::FieldReader<bool, CH2F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2F` writer - Channel 2 Flag"]
pub struct CH2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2F_A::_1)
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
#[doc = "Channel 3 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH3F_A> for bool {
    #[inline(always)]
    fn from(variant: CH3F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3F` reader - Channel 3 Flag"]
pub struct CH3F_R(crate::FieldReader<bool, CH3F_A>);
impl CH3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3F_A {
        match self.bits {
            false => CH3F_A::_0,
            true => CH3F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH3F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH3F_A::_1
    }
}
impl core::ops::Deref for CH3F_R {
    type Target = crate::FieldReader<bool, CH3F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3F` writer - Channel 3 Flag"]
pub struct CH3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3F_A::_1)
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
#[doc = "Channel 4 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH4F_A> for bool {
    #[inline(always)]
    fn from(variant: CH4F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4F` reader - Channel 4 Flag"]
pub struct CH4F_R(crate::FieldReader<bool, CH4F_A>);
impl CH4F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4F_A {
        match self.bits {
            false => CH4F_A::_0,
            true => CH4F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH4F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH4F_A::_1
    }
}
impl core::ops::Deref for CH4F_R {
    type Target = crate::FieldReader<bool, CH4F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4F` writer - Channel 4 Flag"]
pub struct CH4F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4F_A::_1)
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
#[doc = "Channel 5 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH5F_A> for bool {
    #[inline(always)]
    fn from(variant: CH5F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5F` reader - Channel 5 Flag"]
pub struct CH5F_R(crate::FieldReader<bool, CH5F_A>);
impl CH5F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5F_A {
        match self.bits {
            false => CH5F_A::_0,
            true => CH5F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH5F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH5F_A::_1
    }
}
impl core::ops::Deref for CH5F_R {
    type Target = crate::FieldReader<bool, CH5F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5F` writer - Channel 5 Flag"]
pub struct CH5F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5F_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5F_A::_1)
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
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
    #[doc = "0: TPM counter has not overflowed."]
    _0 = 0,
    #[doc = "1: TPM counter has overflowed."]
    _1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Timer Overflow Flag"]
pub struct TOF_R(crate::FieldReader<bool, TOF_A>);
impl TOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::_0,
            true => TOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOF_A::_1
    }
}
impl core::ops::Deref for TOF_R {
    type Target = crate::FieldReader<bool, TOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOF` writer - Timer Overflow Flag"]
pub struct TOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TPM counter has not overflowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOF_A::_0)
    }
    #[doc = "TPM counter has overflowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOF_A::_1)
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
impl R {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    pub fn ch0f(&self) -> CH0F_R {
        CH0F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    pub fn ch0f(&mut self) -> CH0F_W {
        CH0F_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    pub fn ch1f(&mut self) -> CH1F_W {
        CH1F_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    pub fn ch2f(&mut self) -> CH2F_W {
        CH2F_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    pub fn ch3f(&mut self) -> CH3F_W {
        CH3F_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    pub fn ch4f(&mut self) -> CH4F_W {
        CH4F_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    pub fn ch5f(&mut self) -> CH5F_W {
        CH5F_W { w: self }
    }
    #[doc = "Bit 8 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W {
        TOF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture and Compare Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
