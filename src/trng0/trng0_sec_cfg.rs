#[doc = "Register `TRNG0_SEC_CFG` reader"]
pub struct R(crate::R<TRNG0_SEC_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_SEC_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_SEC_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_SEC_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_SEC_CFG` writer"]
pub struct W(crate::W<TRNG0_SEC_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_SEC_CFG_SPEC>;
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
impl From<crate::W<TRNG0_SEC_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_SEC_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reserved. DRNG specific, not applicable to this version.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SH0_A {
    #[doc = "0: See DRNG version."]
    _0 = 0,
    #[doc = "1: See DRNG version."]
    _1 = 1,
}
impl From<SH0_A> for bool {
    #[inline(always)]
    fn from(variant: SH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SH0` reader - Reserved. DRNG specific, not applicable to this version."]
pub struct SH0_R(crate::FieldReader<bool, SH0_A>);
impl SH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SH0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SH0_A {
        match self.bits {
            false => SH0_A::_0,
            true => SH0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SH0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SH0_A::_1
    }
}
impl core::ops::Deref for SH0_R {
    type Target = crate::FieldReader<bool, SH0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SH0` writer - Reserved. DRNG specific, not applicable to this version."]
pub struct SH0_W<'a> {
    w: &'a mut W,
}
impl<'a> SH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SH0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "See DRNG version."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SH0_A::_0)
    }
    #[doc = "See DRNG version."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SH0_A::_1)
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
#[doc = "If set the TRNG registers cannot be programmed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NO_PRGM_A {
    #[doc = "0: Programability of registers controlled only by the TRNG0 Miscellaneous Control Register's access mode bit."]
    _0 = 0,
    #[doc = "1: Overides TRNG0 Miscellaneous Control Register access mode and prevents TRNG register programming."]
    _1 = 1,
}
impl From<NO_PRGM_A> for bool {
    #[inline(always)]
    fn from(variant: NO_PRGM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NO_PRGM` reader - If set the TRNG registers cannot be programmed"]
pub struct NO_PRGM_R(crate::FieldReader<bool, NO_PRGM_A>);
impl NO_PRGM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NO_PRGM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NO_PRGM_A {
        match self.bits {
            false => NO_PRGM_A::_0,
            true => NO_PRGM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NO_PRGM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NO_PRGM_A::_1
    }
}
impl core::ops::Deref for NO_PRGM_R {
    type Target = crate::FieldReader<bool, NO_PRGM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NO_PRGM` writer - If set the TRNG registers cannot be programmed"]
pub struct NO_PRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_PRGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NO_PRGM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Programability of registers controlled only by the TRNG0 Miscellaneous Control Register's access mode bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NO_PRGM_A::_0)
    }
    #[doc = "Overides TRNG0 Miscellaneous Control Register access mode and prevents TRNG register programming."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NO_PRGM_A::_1)
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
#[doc = "Reserved. DRNG-specific, not applicable to this version.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SK_VAL_A {
    #[doc = "0: See DRNG version."]
    _0 = 0,
    #[doc = "1: See DRNG version."]
    _1 = 1,
}
impl From<SK_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: SK_VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SK_VAL` reader - Reserved. DRNG-specific, not applicable to this version."]
pub struct SK_VAL_R(crate::FieldReader<bool, SK_VAL_A>);
impl SK_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SK_VAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SK_VAL_A {
        match self.bits {
            false => SK_VAL_A::_0,
            true => SK_VAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SK_VAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SK_VAL_A::_1
    }
}
impl core::ops::Deref for SK_VAL_R {
    type Target = crate::FieldReader<bool, SK_VAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SK_VAL` writer - Reserved. DRNG-specific, not applicable to this version."]
pub struct SK_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SK_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SK_VAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "See DRNG version."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SK_VAL_A::_0)
    }
    #[doc = "See DRNG version."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SK_VAL_A::_1)
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
    #[doc = "Bit 0 - Reserved. DRNG specific, not applicable to this version."]
    #[inline(always)]
    pub fn sh0(&self) -> SH0_R {
        SH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If set the TRNG registers cannot be programmed"]
    #[inline(always)]
    pub fn no_prgm(&self) -> NO_PRGM_R {
        NO_PRGM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reserved. DRNG-specific, not applicable to this version."]
    #[inline(always)]
    pub fn sk_val(&self) -> SK_VAL_R {
        SK_VAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved. DRNG specific, not applicable to this version."]
    #[inline(always)]
    pub fn sh0(&mut self) -> SH0_W {
        SH0_W { w: self }
    }
    #[doc = "Bit 1 - If set the TRNG registers cannot be programmed"]
    #[inline(always)]
    pub fn no_prgm(&mut self) -> NO_PRGM_W {
        NO_PRGM_W { w: self }
    }
    #[doc = "Bit 2 - Reserved. DRNG-specific, not applicable to this version."]
    #[inline(always)]
    pub fn sk_val(&mut self) -> SK_VAL_W {
        SK_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Security Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_sec_cfg](index.html) module"]
pub struct TRNG0_SEC_CFG_SPEC;
impl crate::RegisterSpec for TRNG0_SEC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_sec_cfg::R](R) reader structure"]
impl crate::Readable for TRNG0_SEC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_sec_cfg::W](W) writer structure"]
impl crate::Writable for TRNG0_SEC_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_SEC_CFG to value 0"]
impl crate::Resettable for TRNG0_SEC_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
