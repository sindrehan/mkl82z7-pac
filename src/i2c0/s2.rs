#[doc = "Register `S2` reader"]
pub struct R(crate::R<S2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S2` writer"]
pub struct W(crate::W<S2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S2_SPEC>;
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
impl From<crate::W<S2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPTY_A {
    #[doc = "0: Tx or Rx buffer is not empty and cannot be written to, that is new data cannot be loaded into the buffer."]
    _0 = 0,
    #[doc = "1: Tx or Rx buffer is empty and can be written to, that is new data can be loaded into the buffer."]
    _1 = 1,
}
impl From<EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMPTY` reader - Empty flag"]
pub struct EMPTY_R(crate::FieldReader<bool, EMPTY_A>);
impl EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMPTY_A {
        match self.bits {
            false => EMPTY_A::_0,
            true => EMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EMPTY_A::_1
    }
}
impl core::ops::Deref for EMPTY_R {
    type Target = crate::FieldReader<bool, EMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: The buffer is not full and all write/read operations have no errors."]
    _0 = 0,
    #[doc = "1: There are 3 or more write/read errors during the data transfer phase (when the Empty flag is not set and the buffer is busy)."]
    _1 = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Error flag"]
pub struct ERROR_R(crate::FieldReader<bool, ERROR_A>);
impl ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::_0,
            true => ERROR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERROR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERROR_A::_1
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - Error flag"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The buffer is not full and all write/read operations have no errors."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERROR_A::_0)
    }
    #[doc = "There are 3 or more write/read errors during the data transfer phase (when the Empty flag is not set and the buffer is busy)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERROR_A::_1)
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
#[doc = "Double Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFEN_A {
    #[doc = "0: Disables the double buffer mode; clock stretch is enabled."]
    _0 = 0,
    #[doc = "1: Enables the double buffer mode; clock stretch is disabled. In the slave mode, the I2C will not hold bus between data transfers."]
    _1 = 1,
}
impl From<DFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFEN` reader - Double Buffer Enable"]
pub struct DFEN_R(crate::FieldReader<bool, DFEN_A>);
impl DFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFEN_A {
        match self.bits {
            false => DFEN_A::_0,
            true => DFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFEN_A::_1
    }
}
impl core::ops::Deref for DFEN_R {
    type Target = crate::FieldReader<bool, DFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFEN` writer - Double Buffer Enable"]
pub struct DFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the double buffer mode; clock stretch is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFEN_A::_0)
    }
    #[doc = "Enables the double buffer mode; clock stretch is disabled. In the slave mode, the I2C will not hold bus between data transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Empty flag"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error flag"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Double Buffer Enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Error flag"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 2 - Double Buffer Enable"]
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W {
        DFEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Status register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2](index.html) module"]
pub struct S2_SPEC;
impl crate::RegisterSpec for S2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s2::R](R) reader structure"]
impl crate::Readable for S2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s2::W](W) writer structure"]
impl crate::Writable for S2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S2 to value 0x01"]
impl crate::Resettable for S2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
