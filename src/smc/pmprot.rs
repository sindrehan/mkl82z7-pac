#[doc = "Register `PMPROT` reader"]
pub struct R(crate::R<PMPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMPROT` writer"]
pub struct W(crate::W<PMPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMPROT_SPEC>;
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
impl From<crate::W<PMPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Allow Very-Low-Leakage Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLLS_A {
    #[doc = "0: Any VLLSx mode is not allowed"]
    _0 = 0,
    #[doc = "1: Any VLLSx mode is allowed"]
    _1 = 1,
}
impl From<AVLLS_A> for bool {
    #[inline(always)]
    fn from(variant: AVLLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLLS` reader - Allow Very-Low-Leakage Stop Mode"]
pub struct AVLLS_R(crate::FieldReader<bool, AVLLS_A>);
impl AVLLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVLLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLLS_A {
        match self.bits {
            false => AVLLS_A::_0,
            true => AVLLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AVLLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AVLLS_A::_1
    }
}
impl core::ops::Deref for AVLLS_R {
    type Target = crate::FieldReader<bool, AVLLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVLLS` writer - Allow Very-Low-Leakage Stop Mode"]
pub struct AVLLS_W<'a> {
    w: &'a mut W,
}
impl<'a> AVLLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVLLS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Any VLLSx mode is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLLS_A::_0)
    }
    #[doc = "Any VLLSx mode is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLLS_A::_1)
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
#[doc = "Allow Low-Leakage Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALLS_A {
    #[doc = "0: Any LLSx mode is not allowed"]
    _0 = 0,
    #[doc = "1: Any LLSx mode is allowed"]
    _1 = 1,
}
impl From<ALLS_A> for bool {
    #[inline(always)]
    fn from(variant: ALLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLS` reader - Allow Low-Leakage Stop Mode"]
pub struct ALLS_R(crate::FieldReader<bool, ALLS_A>);
impl ALLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALLS_A {
        match self.bits {
            false => ALLS_A::_0,
            true => ALLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALLS_A::_1
    }
}
impl core::ops::Deref for ALLS_R {
    type Target = crate::FieldReader<bool, ALLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLS` writer - Allow Low-Leakage Stop Mode"]
pub struct ALLS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALLS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Any LLSx mode is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALLS_A::_0)
    }
    #[doc = "Any LLSx mode is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALLS_A::_1)
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
#[doc = "Allow Very-Low-Power Modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLP_A {
    #[doc = "0: VLPR, VLPW, and VLPS are not allowed."]
    _0 = 0,
    #[doc = "1: VLPR, VLPW, and VLPS are allowed."]
    _1 = 1,
}
impl From<AVLP_A> for bool {
    #[inline(always)]
    fn from(variant: AVLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLP` reader - Allow Very-Low-Power Modes"]
pub struct AVLP_R(crate::FieldReader<bool, AVLP_A>);
impl AVLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLP_A {
        match self.bits {
            false => AVLP_A::_0,
            true => AVLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AVLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AVLP_A::_1
    }
}
impl core::ops::Deref for AVLP_R {
    type Target = crate::FieldReader<bool, AVLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVLP` writer - Allow Very-Low-Power Modes"]
pub struct AVLP_W<'a> {
    w: &'a mut W,
}
impl<'a> AVLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VLPR, VLPW, and VLPS are not allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLP_A::_0)
    }
    #[doc = "VLPR, VLPW, and VLPS are allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLP_A::_1)
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
#[doc = "Allow High Speed Run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHSRUN_A {
    #[doc = "0: HSRUN is not allowed"]
    _0 = 0,
    #[doc = "1: HSRUN is allowed"]
    _1 = 1,
}
impl From<AHSRUN_A> for bool {
    #[inline(always)]
    fn from(variant: AHSRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHSRUN` reader - Allow High Speed Run mode"]
pub struct AHSRUN_R(crate::FieldReader<bool, AHSRUN_A>);
impl AHSRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHSRUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHSRUN_A {
        match self.bits {
            false => AHSRUN_A::_0,
            true => AHSRUN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AHSRUN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AHSRUN_A::_1
    }
}
impl core::ops::Deref for AHSRUN_R {
    type Target = crate::FieldReader<bool, AHSRUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHSRUN` writer - Allow High Speed Run mode"]
pub struct AHSRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHSRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHSRUN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSRUN is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AHSRUN_A::_0)
    }
    #[doc = "HSRUN is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AHSRUN_A::_1)
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
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn avlls(&self) -> AVLLS_R {
        AVLLS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn alls(&self) -> ALLS_R {
        ALLS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&self) -> AVLP_R {
        AVLP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Allow High Speed Run mode"]
    #[inline(always)]
    pub fn ahsrun(&self) -> AHSRUN_R {
        AHSRUN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn avlls(&mut self) -> AVLLS_W {
        AVLLS_W { w: self }
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn alls(&mut self) -> ALLS_W {
        ALLS_W { w: self }
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&mut self) -> AVLP_W {
        AVLP_W { w: self }
    }
    #[doc = "Bit 7 - Allow High Speed Run mode"]
    #[inline(always)]
    pub fn ahsrun(&mut self) -> AHSRUN_W {
        AHSRUN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmprot](index.html) module"]
pub struct PMPROT_SPEC;
impl crate::RegisterSpec for PMPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pmprot::R](R) reader structure"]
impl crate::Readable for PMPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmprot::W](W) writer structure"]
impl crate::Writable for PMPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMPROT to value 0x20"]
impl crate::Resettable for PMPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
