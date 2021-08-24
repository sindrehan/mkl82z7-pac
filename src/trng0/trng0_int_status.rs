#[doc = "Register `TRNG0_INT_STATUS` reader"]
pub struct R(crate::R<TRNG0_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_INT_STATUS` writer"]
pub struct W(crate::W<TRNG0_INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_INT_STATUS_SPEC>;
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
impl From<crate::W<TRNG0_INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Read: Error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_ERR_A {
    #[doc = "0: no error"]
    _0 = 0,
    #[doc = "1: error detected."]
    _1 = 1,
}
impl From<HW_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: HW_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HW_ERR` reader - Read: Error status"]
pub struct HW_ERR_R(crate::FieldReader<bool, HW_ERR_A>);
impl HW_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HW_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HW_ERR_A {
        match self.bits {
            false => HW_ERR_A::_0,
            true => HW_ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HW_ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HW_ERR_A::_1
    }
}
impl core::ops::Deref for HW_ERR_R {
    type Target = crate::FieldReader<bool, HW_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read only: Entropy Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENT_VAL_A {
    #[doc = "0: Busy generation entropy. Any value read is invalid."]
    _0 = 0,
    #[doc = "1: TRNG can be stopped and entropy is valid if read."]
    _1 = 1,
}
impl From<ENT_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: ENT_VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENT_VAL` reader - Read only: Entropy Valid"]
pub struct ENT_VAL_R(crate::FieldReader<bool, ENT_VAL_A>);
impl ENT_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENT_VAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENT_VAL_A {
        match self.bits {
            false => ENT_VAL_A::_0,
            true => ENT_VAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENT_VAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENT_VAL_A::_1
    }
}
impl core::ops::Deref for ENT_VAL_R {
    type Target = crate::FieldReader<bool, ENT_VAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read only: Frequency Count Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRQ_CT_FAIL_A {
    #[doc = "0: No hardware nor self test frequency errors."]
    _0 = 0,
    #[doc = "1: The frequency counter has detected a failure."]
    _1 = 1,
}
impl From<FRQ_CT_FAIL_A> for bool {
    #[inline(always)]
    fn from(variant: FRQ_CT_FAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRQ_CT_FAIL` reader - Read only: Frequency Count Fail"]
pub struct FRQ_CT_FAIL_R(crate::FieldReader<bool, FRQ_CT_FAIL_A>);
impl FRQ_CT_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRQ_CT_FAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRQ_CT_FAIL_A {
        match self.bits {
            false => FRQ_CT_FAIL_A::_0,
            true => FRQ_CT_FAIL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRQ_CT_FAIL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRQ_CT_FAIL_A::_1
    }
}
impl core::ops::Deref for FRQ_CT_FAIL_R {
    type Target = crate::FieldReader<bool, FRQ_CT_FAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRQ_CT_FAIL` writer - Read only: Frequency Count Fail"]
pub struct FRQ_CT_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQ_CT_FAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRQ_CT_FAIL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No hardware nor self test frequency errors."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRQ_CT_FAIL_A::_0)
    }
    #[doc = "The frequency counter has detected a failure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRQ_CT_FAIL_A::_1)
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
impl R {
    #[doc = "Bit 0 - Read: Error status"]
    #[inline(always)]
    pub fn hw_err(&self) -> HW_ERR_R {
        HW_ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read only: Entropy Valid"]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn frq_ct_fail(&self) -> FRQ_CT_FAIL_R {
        FRQ_CT_FAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn frq_ct_fail(&mut self) -> FRQ_CT_FAIL_W {
        FRQ_CT_FAIL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_int_status](index.html) module"]
pub struct TRNG0_INT_STATUS_SPEC;
impl crate::RegisterSpec for TRNG0_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_int_status::R](R) reader structure"]
impl crate::Readable for TRNG0_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_int_status::W](W) writer structure"]
impl crate::Writable for TRNG0_INT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_INT_STATUS to value 0"]
impl crate::Resettable for TRNG0_INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
