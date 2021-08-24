#[doc = "Register `TRNG0_INT_CTRL` reader"]
pub struct R(crate::R<TRNG0_INT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_INT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_INT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_INT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_INT_CTRL` writer"]
pub struct W(crate::W<TRNG0_INT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_INT_CTRL_SPEC>;
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
impl From<crate::W<TRNG0_INT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_INT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_ERR_A {
    #[doc = "0: Corresponding bit of INT_STATUS cleared."]
    _0 = 0,
    #[doc = "1: Corresponding bit of INT_STATUS active."]
    _1 = 1,
}
impl From<HW_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: HW_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HW_ERR` reader - Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
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
#[doc = "Field `HW_ERR` writer - Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
pub struct HW_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW_ERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding bit of INT_STATUS cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HW_ERR_A::_0)
    }
    #[doc = "Corresponding bit of INT_STATUS active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HW_ERR_A::_1)
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
#[doc = "Same behavior as bit 0 above.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENT_VAL_A {
    #[doc = "0: Same behavior as bit 0 above."]
    _0 = 0,
    #[doc = "1: Same behavior as bit 0 above."]
    _1 = 1,
}
impl From<ENT_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: ENT_VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENT_VAL` reader - Same behavior as bit 0 above."]
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
#[doc = "Field `ENT_VAL` writer - Same behavior as bit 0 above."]
pub struct ENT_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENT_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENT_VAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENT_VAL_A::_0)
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENT_VAL_A::_1)
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
#[doc = "Same behavior as bit 0 above.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRQ_CT_FAIL_A {
    #[doc = "0: Same behavior as bit 0 above."]
    _0 = 0,
    #[doc = "1: Same behavior as bit 0 above."]
    _1 = 1,
}
impl From<FRQ_CT_FAIL_A> for bool {
    #[inline(always)]
    fn from(variant: FRQ_CT_FAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRQ_CT_FAIL` reader - Same behavior as bit 0 above."]
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
#[doc = "Field `FRQ_CT_FAIL` writer - Same behavior as bit 0 above."]
pub struct FRQ_CT_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQ_CT_FAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRQ_CT_FAIL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRQ_CT_FAIL_A::_0)
    }
    #[doc = "Same behavior as bit 0 above."]
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
#[doc = "Field `UNUSED` reader - Reserved but writeable."]
pub struct UNUSED_R(crate::FieldReader<u32, u32>);
impl UNUSED_R {
    pub(crate) fn new(bits: u32) -> Self {
        UNUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNUSED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNUSED` writer - Reserved but writeable."]
pub struct UNUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    #[inline(always)]
    pub fn hw_err(&self) -> HW_ERR_R {
        HW_ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn frq_ct_fail(&self) -> FRQ_CT_FAIL_R {
        FRQ_CT_FAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:31 - Reserved but writeable."]
    #[inline(always)]
    pub fn unused(&self) -> UNUSED_R {
        UNUSED_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    #[inline(always)]
    pub fn hw_err(&mut self) -> HW_ERR_W {
        HW_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn ent_val(&mut self) -> ENT_VAL_W {
        ENT_VAL_W { w: self }
    }
    #[doc = "Bit 2 - Same behavior as bit 0 above."]
    #[inline(always)]
    pub fn frq_ct_fail(&mut self) -> FRQ_CT_FAIL_W {
        FRQ_CT_FAIL_W { w: self }
    }
    #[doc = "Bits 3:31 - Reserved but writeable."]
    #[inline(always)]
    pub fn unused(&mut self) -> UNUSED_W {
        UNUSED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_int_ctrl](index.html) module"]
pub struct TRNG0_INT_CTRL_SPEC;
impl crate::RegisterSpec for TRNG0_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_int_ctrl::R](R) reader structure"]
impl crate::Readable for TRNG0_INT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_int_ctrl::W](W) writer structure"]
impl crate::Writable for TRNG0_INT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_INT_CTRL to value 0xffff_ffff"]
impl crate::Resettable for TRNG0_INT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
