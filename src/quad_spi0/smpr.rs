#[doc = "Register `SMPR` reader"]
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR` writer"]
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
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
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Half Speed serial flash clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSENA_A {
    #[doc = "0: Disable divide by 2 of serial flash clock for half speed commands"]
    _0 = 0,
    #[doc = "1: Enable divide by 2 of serial flash clock for half speed commands"]
    _1 = 1,
}
impl From<HSENA_A> for bool {
    #[inline(always)]
    fn from(variant: HSENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSENA` reader - Half Speed serial flash clock Enable"]
pub struct HSENA_R(crate::FieldReader<bool, HSENA_A>);
impl HSENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSENA_A {
        match self.bits {
            false => HSENA_A::_0,
            true => HSENA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSENA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSENA_A::_1
    }
}
impl core::ops::Deref for HSENA_R {
    type Target = crate::FieldReader<bool, HSENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSENA` writer - Half Speed serial flash clock Enable"]
pub struct HSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable divide by 2 of serial flash clock for half speed commands"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSENA_A::_0)
    }
    #[doc = "Enable divide by 2 of serial flash clock for half speed commands"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSENA_A::_1)
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
#[doc = "Half Speed Phase selection for SDR instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPHS_A {
    #[doc = "0: Select sampling at non-inverted clock"]
    _0 = 0,
    #[doc = "1: Select sampling at inverted clock"]
    _1 = 1,
}
impl From<HSPHS_A> for bool {
    #[inline(always)]
    fn from(variant: HSPHS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPHS` reader - Half Speed Phase selection for SDR instructions."]
pub struct HSPHS_R(crate::FieldReader<bool, HSPHS_A>);
impl HSPHS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSPHS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSPHS_A {
        match self.bits {
            false => HSPHS_A::_0,
            true => HSPHS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSPHS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSPHS_A::_1
    }
}
impl core::ops::Deref for HSPHS_R {
    type Target = crate::FieldReader<bool, HSPHS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSPHS` writer - Half Speed Phase selection for SDR instructions."]
pub struct HSPHS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSPHS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select sampling at non-inverted clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSPHS_A::_0)
    }
    #[doc = "Select sampling at inverted clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSPHS_A::_1)
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
#[doc = "Half Speed Delay selection for SDR instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSDLY_A {
    #[doc = "0: One clock cycle delay"]
    _0 = 0,
    #[doc = "1: Two clock cycle delay"]
    _1 = 1,
}
impl From<HSDLY_A> for bool {
    #[inline(always)]
    fn from(variant: HSDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSDLY` reader - Half Speed Delay selection for SDR instructions."]
pub struct HSDLY_R(crate::FieldReader<bool, HSDLY_A>);
impl HSDLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSDLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSDLY_A {
        match self.bits {
            false => HSDLY_A::_0,
            true => HSDLY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HSDLY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HSDLY_A::_1
    }
}
impl core::ops::Deref for HSDLY_R {
    type Target = crate::FieldReader<bool, HSDLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSDLY` writer - Half Speed Delay selection for SDR instructions."]
pub struct HSDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSDLY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One clock cycle delay"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSDLY_A::_0)
    }
    #[doc = "Two clock cycle delay"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSDLY_A::_1)
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
#[doc = "Full Speed Phase selection for SDR instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPHS_A {
    #[doc = "0: Select sampling at non-inverted clock"]
    _0 = 0,
    #[doc = "1: Select sampling at inverted clock. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    _1 = 1,
}
impl From<FSPHS_A> for bool {
    #[inline(always)]
    fn from(variant: FSPHS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSPHS` reader - Full Speed Phase selection for SDR instructions."]
pub struct FSPHS_R(crate::FieldReader<bool, FSPHS_A>);
impl FSPHS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSPHS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSPHS_A {
        match self.bits {
            false => FSPHS_A::_0,
            true => FSPHS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FSPHS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FSPHS_A::_1
    }
}
impl core::ops::Deref for FSPHS_R {
    type Target = crate::FieldReader<bool, FSPHS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSPHS` writer - Full Speed Phase selection for SDR instructions."]
pub struct FSPHS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSPHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSPHS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select sampling at non-inverted clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSPHS_A::_0)
    }
    #[doc = "Select sampling at inverted clock. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSPHS_A::_1)
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
#[doc = "Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSDLY_A {
    #[doc = "0: One clock cycle delay"]
    _0 = 0,
    #[doc = "1: Two clock cycles delay. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    _1 = 1,
}
impl From<FSDLY_A> for bool {
    #[inline(always)]
    fn from(variant: FSDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSDLY` reader - Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands."]
pub struct FSDLY_R(crate::FieldReader<bool, FSDLY_A>);
impl FSDLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSDLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSDLY_A {
        match self.bits {
            false => FSDLY_A::_0,
            true => FSDLY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FSDLY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FSDLY_A::_1
    }
}
impl core::ops::Deref for FSDLY_R {
    type Target = crate::FieldReader<bool, FSDLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSDLY` writer - Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands."]
pub struct FSDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSDLY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One clock cycle delay"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSDLY_A::_0)
    }
    #[doc = "Two clock cycles delay. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSDLY_A::_1)
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
#[doc = "Field `DDRSMP` reader - DDR Sampling point"]
pub struct DDRSMP_R(crate::FieldReader<u8, u8>);
impl DDRSMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DDRSMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRSMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRSMP` writer - DDR Sampling point"]
pub struct DDRSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRSMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Half Speed serial flash clock Enable"]
    #[inline(always)]
    pub fn hsena(&self) -> HSENA_R {
        HSENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Half Speed Phase selection for SDR instructions."]
    #[inline(always)]
    pub fn hsphs(&self) -> HSPHS_R {
        HSPHS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Half Speed Delay selection for SDR instructions."]
    #[inline(always)]
    pub fn hsdly(&self) -> HSDLY_R {
        HSDLY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Full Speed Phase selection for SDR instructions."]
    #[inline(always)]
    pub fn fsphs(&self) -> FSPHS_R {
        FSPHS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands."]
    #[inline(always)]
    pub fn fsdly(&self) -> FSDLY_R {
        FSDLY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - DDR Sampling point"]
    #[inline(always)]
    pub fn ddrsmp(&self) -> DDRSMP_R {
        DDRSMP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Half Speed serial flash clock Enable"]
    #[inline(always)]
    pub fn hsena(&mut self) -> HSENA_W {
        HSENA_W { w: self }
    }
    #[doc = "Bit 1 - Half Speed Phase selection for SDR instructions."]
    #[inline(always)]
    pub fn hsphs(&mut self) -> HSPHS_W {
        HSPHS_W { w: self }
    }
    #[doc = "Bit 2 - Half Speed Delay selection for SDR instructions."]
    #[inline(always)]
    pub fn hsdly(&mut self) -> HSDLY_W {
        HSDLY_W { w: self }
    }
    #[doc = "Bit 5 - Full Speed Phase selection for SDR instructions."]
    #[inline(always)]
    pub fn fsphs(&mut self) -> FSPHS_W {
        FSPHS_W { w: self }
    }
    #[doc = "Bit 6 - Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands."]
    #[inline(always)]
    pub fn fsdly(&mut self) -> FSDLY_W {
        FSDLY_W { w: self }
    }
    #[doc = "Bits 16:18 - DDR Sampling point"]
    #[inline(always)]
    pub fn ddrsmp(&mut self) -> DDRSMP_W {
        DDRSMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sampling Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](index.html) module"]
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr::R](R) reader structure"]
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr::W](W) writer structure"]
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
