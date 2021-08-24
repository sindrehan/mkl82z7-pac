#[doc = "Register `COMBINE` reader"]
pub struct R(crate::R<COMBINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMBINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMBINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMBINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMBINE` writer"]
pub struct W(crate::W<COMBINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMBINE_SPEC>;
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
impl From<crate::W<COMBINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMBINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Combine Channels 0 and 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBINE0_A {
    #[doc = "0: Channels 0 and 1 are independent."]
    _0 = 0,
    #[doc = "1: Channels 0 and 1 are combined."]
    _1 = 1,
}
impl From<COMBINE0_A> for bool {
    #[inline(always)]
    fn from(variant: COMBINE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMBINE0` reader - Combine Channels 0 and 1"]
pub struct COMBINE0_R(crate::FieldReader<bool, COMBINE0_A>);
impl COMBINE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMBINE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBINE0_A {
        match self.bits {
            false => COMBINE0_A::_0,
            true => COMBINE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMBINE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMBINE0_A::_1
    }
}
impl core::ops::Deref for COMBINE0_R {
    type Target = crate::FieldReader<bool, COMBINE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMBINE0` writer - Combine Channels 0 and 1"]
pub struct COMBINE0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMBINE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channels 0 and 1 are independent."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMBINE0_A::_0)
    }
    #[doc = "Channels 0 and 1 are combined."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMBINE0_A::_1)
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
#[doc = "Combine Channel 0 and 1 Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMSWAP0_A {
    #[doc = "0: Even channel is used for input capture and 1st compare."]
    _0 = 0,
    #[doc = "1: Odd channel is used for input capture and 1st compare."]
    _1 = 1,
}
impl From<COMSWAP0_A> for bool {
    #[inline(always)]
    fn from(variant: COMSWAP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMSWAP0` reader - Combine Channel 0 and 1 Swap"]
pub struct COMSWAP0_R(crate::FieldReader<bool, COMSWAP0_A>);
impl COMSWAP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMSWAP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMSWAP0_A {
        match self.bits {
            false => COMSWAP0_A::_0,
            true => COMSWAP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMSWAP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMSWAP0_A::_1
    }
}
impl core::ops::Deref for COMSWAP0_R {
    type Target = crate::FieldReader<bool, COMSWAP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMSWAP0` writer - Combine Channel 0 and 1 Swap"]
pub struct COMSWAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMSWAP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMSWAP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Even channel is used for input capture and 1st compare."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMSWAP0_A::_0)
    }
    #[doc = "Odd channel is used for input capture and 1st compare."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMSWAP0_A::_1)
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
#[doc = "Combine Channels 2 and 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBINE1_A {
    #[doc = "0: Channels 2 and 3 are independent."]
    _0 = 0,
    #[doc = "1: Channels 2 and 3 are combined."]
    _1 = 1,
}
impl From<COMBINE1_A> for bool {
    #[inline(always)]
    fn from(variant: COMBINE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMBINE1` reader - Combine Channels 2 and 3"]
pub struct COMBINE1_R(crate::FieldReader<bool, COMBINE1_A>);
impl COMBINE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMBINE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBINE1_A {
        match self.bits {
            false => COMBINE1_A::_0,
            true => COMBINE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMBINE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMBINE1_A::_1
    }
}
impl core::ops::Deref for COMBINE1_R {
    type Target = crate::FieldReader<bool, COMBINE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMBINE1` writer - Combine Channels 2 and 3"]
pub struct COMBINE1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMBINE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channels 2 and 3 are independent."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMBINE1_A::_0)
    }
    #[doc = "Channels 2 and 3 are combined."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMBINE1_A::_1)
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
#[doc = "Combine Channels 2 and 3 Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMSWAP1_A {
    #[doc = "0: Even channel is used for input capture and 1st compare."]
    _0 = 0,
    #[doc = "1: Odd channel is used for input capture and 1st compare."]
    _1 = 1,
}
impl From<COMSWAP1_A> for bool {
    #[inline(always)]
    fn from(variant: COMSWAP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMSWAP1` reader - Combine Channels 2 and 3 Swap"]
pub struct COMSWAP1_R(crate::FieldReader<bool, COMSWAP1_A>);
impl COMSWAP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMSWAP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMSWAP1_A {
        match self.bits {
            false => COMSWAP1_A::_0,
            true => COMSWAP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMSWAP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMSWAP1_A::_1
    }
}
impl core::ops::Deref for COMSWAP1_R {
    type Target = crate::FieldReader<bool, COMSWAP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMSWAP1` writer - Combine Channels 2 and 3 Swap"]
pub struct COMSWAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMSWAP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMSWAP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Even channel is used for input capture and 1st compare."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMSWAP1_A::_0)
    }
    #[doc = "Odd channel is used for input capture and 1st compare."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMSWAP1_A::_1)
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
#[doc = "Combine Channels 4 and 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBINE2_A {
    #[doc = "0: Channels 4 and 5 are independent."]
    _0 = 0,
    #[doc = "1: Channels 4 and 5 are combined."]
    _1 = 1,
}
impl From<COMBINE2_A> for bool {
    #[inline(always)]
    fn from(variant: COMBINE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMBINE2` reader - Combine Channels 4 and 5"]
pub struct COMBINE2_R(crate::FieldReader<bool, COMBINE2_A>);
impl COMBINE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMBINE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBINE2_A {
        match self.bits {
            false => COMBINE2_A::_0,
            true => COMBINE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMBINE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMBINE2_A::_1
    }
}
impl core::ops::Deref for COMBINE2_R {
    type Target = crate::FieldReader<bool, COMBINE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMBINE2` writer - Combine Channels 4 and 5"]
pub struct COMBINE2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMBINE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channels 4 and 5 are independent."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMBINE2_A::_0)
    }
    #[doc = "Channels 4 and 5 are combined."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMBINE2_A::_1)
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
#[doc = "Combine Channels 4 and 5 Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMSWAP2_A {
    #[doc = "0: Even channel is used for input capture and 1st compare."]
    _0 = 0,
    #[doc = "1: Odd channel is used for input capture and 1st compare."]
    _1 = 1,
}
impl From<COMSWAP2_A> for bool {
    #[inline(always)]
    fn from(variant: COMSWAP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMSWAP2` reader - Combine Channels 4 and 5 Swap"]
pub struct COMSWAP2_R(crate::FieldReader<bool, COMSWAP2_A>);
impl COMSWAP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMSWAP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMSWAP2_A {
        match self.bits {
            false => COMSWAP2_A::_0,
            true => COMSWAP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMSWAP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMSWAP2_A::_1
    }
}
impl core::ops::Deref for COMSWAP2_R {
    type Target = crate::FieldReader<bool, COMSWAP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMSWAP2` writer - Combine Channels 4 and 5 Swap"]
pub struct COMSWAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMSWAP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMSWAP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Even channel is used for input capture and 1st compare."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMSWAP2_A::_0)
    }
    #[doc = "Odd channel is used for input capture and 1st compare."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMSWAP2_A::_1)
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
impl R {
    #[doc = "Bit 0 - Combine Channels 0 and 1"]
    #[inline(always)]
    pub fn combine0(&self) -> COMBINE0_R {
        COMBINE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Combine Channel 0 and 1 Swap"]
    #[inline(always)]
    pub fn comswap0(&self) -> COMSWAP0_R {
        COMSWAP0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Combine Channels 2 and 3"]
    #[inline(always)]
    pub fn combine1(&self) -> COMBINE1_R {
        COMBINE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Combine Channels 2 and 3 Swap"]
    #[inline(always)]
    pub fn comswap1(&self) -> COMSWAP1_R {
        COMSWAP1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Combine Channels 4 and 5"]
    #[inline(always)]
    pub fn combine2(&self) -> COMBINE2_R {
        COMBINE2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Combine Channels 4 and 5 Swap"]
    #[inline(always)]
    pub fn comswap2(&self) -> COMSWAP2_R {
        COMSWAP2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Combine Channels 0 and 1"]
    #[inline(always)]
    pub fn combine0(&mut self) -> COMBINE0_W {
        COMBINE0_W { w: self }
    }
    #[doc = "Bit 1 - Combine Channel 0 and 1 Swap"]
    #[inline(always)]
    pub fn comswap0(&mut self) -> COMSWAP0_W {
        COMSWAP0_W { w: self }
    }
    #[doc = "Bit 8 - Combine Channels 2 and 3"]
    #[inline(always)]
    pub fn combine1(&mut self) -> COMBINE1_W {
        COMBINE1_W { w: self }
    }
    #[doc = "Bit 9 - Combine Channels 2 and 3 Swap"]
    #[inline(always)]
    pub fn comswap1(&mut self) -> COMSWAP1_W {
        COMSWAP1_W { w: self }
    }
    #[doc = "Bit 16 - Combine Channels 4 and 5"]
    #[inline(always)]
    pub fn combine2(&mut self) -> COMBINE2_W {
        COMBINE2_W { w: self }
    }
    #[doc = "Bit 17 - Combine Channels 4 and 5 Swap"]
    #[inline(always)]
    pub fn comswap2(&mut self) -> COMSWAP2_W {
        COMSWAP2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combine Channel Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combine](index.html) module"]
pub struct COMBINE_SPEC;
impl crate::RegisterSpec for COMBINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [combine::R](R) reader structure"]
impl crate::Readable for COMBINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [combine::W](W) writer structure"]
impl crate::Writable for COMBINE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMBINE to value 0"]
impl crate::Resettable for COMBINE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
