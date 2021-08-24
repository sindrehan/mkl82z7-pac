#[doc = "Register `LTC0_STA` reader"]
pub struct R(crate::R<LTC0_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_STA` writer"]
pub struct W(crate::W<LTC0_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_STA_SPEC>;
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
impl From<crate::W<LTC0_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AESA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AB_A {
    #[doc = "0: AESA Idle"]
    _0 = 0,
    #[doc = "1: AESA Busy."]
    _1 = 1,
}
impl From<AB_A> for bool {
    #[inline(always)]
    fn from(variant: AB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AB` reader - AESA Busy"]
pub struct AB_R(crate::FieldReader<bool, AB_A>);
impl AB_R {
    pub(crate) fn new(bits: bool) -> Self {
        AB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AB_A {
        match self.bits {
            false => AB_A::_0,
            true => AB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AB_A::_1
    }
}
impl core::ops::Deref for AB_R {
    type Target = crate::FieldReader<bool, AB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DESA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DB_A {
    #[doc = "0: DESA Idle"]
    _0 = 0,
    #[doc = "1: DESA Busy."]
    _1 = 1,
}
impl From<DB_A> for bool {
    #[inline(always)]
    fn from(variant: DB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DB` reader - DESA Busy"]
pub struct DB_R(crate::FieldReader<bool, DB_A>);
impl DB_R {
    pub(crate) fn new(bits: bool) -> Self {
        DB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DB_A {
        match self.bits {
            false => DB_A::_0,
            true => DB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DB_A::_1
    }
}
impl core::ops::Deref for DB_R {
    type Target = crate::FieldReader<bool, DB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PKHA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PB_A {
    #[doc = "0: PKHA Idle"]
    _0 = 0,
    #[doc = "1: PKHA Busy."]
    _1 = 1,
}
impl From<PB_A> for bool {
    #[inline(always)]
    fn from(variant: PB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB` reader - PKHA Busy"]
pub struct PB_R(crate::FieldReader<bool, PB_A>);
impl PB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB_A {
        match self.bits {
            false => PB_A::_0,
            true => PB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PB_A::_1
    }
}
impl core::ops::Deref for PB_R {
    type Target = crate::FieldReader<bool, PB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MDHA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MB_A {
    #[doc = "0: MDHA Idle"]
    _0 = 0,
    #[doc = "1: MDHA Busy"]
    _1 = 1,
}
impl From<MB_A> for bool {
    #[inline(always)]
    fn from(variant: MB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB` reader - MDHA Busy"]
pub struct MB_R(crate::FieldReader<bool, MB_A>);
impl MB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB_A {
        match self.bits {
            false => MB_A::_0,
            true => MB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MB_A::_1
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, MB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DI` reader - Done Interrupt"]
pub struct DI_R(crate::FieldReader<bool, bool>);
impl DI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DI` writer - Done Interrupt"]
pub struct DI_W<'a> {
    w: &'a mut W,
}
impl<'a> DI_W<'a> {
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
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EI_A {
    #[doc = "0: Not Error."]
    _0 = 0,
    #[doc = "1: Error Interrupt."]
    _1 = 1,
}
impl From<EI_A> for bool {
    #[inline(always)]
    fn from(variant: EI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EI` reader - Error Interrupt"]
pub struct EI_R(crate::FieldReader<bool, EI_A>);
impl EI_R {
    pub(crate) fn new(bits: bool) -> Self {
        EI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EI_A {
        match self.bits {
            false => EI_A::_0,
            true => EI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EI_A::_1
    }
}
impl core::ops::Deref for EI_R {
    type Target = crate::FieldReader<bool, EI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPARRN` reader - This bit is asserted after POR and after every 50K blocks processed by AESA to indicate it is advisable for added security to write a new seed to"]
pub struct DPARRN_R(crate::FieldReader<bool, bool>);
impl DPARRN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPARRN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPARRN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKP` reader - Public Key is Prime"]
pub struct PKP_R(crate::FieldReader<bool, bool>);
impl PKP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKO` reader - Public Key Operation is One"]
pub struct PKO_R(crate::FieldReader<bool, bool>);
impl PKO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKZ` reader - Public Key Operation is Zero"]
pub struct PKZ_R(crate::FieldReader<bool, bool>);
impl PKZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - AESA Busy"]
    #[inline(always)]
    pub fn ab(&self) -> AB_R {
        AB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DESA Busy"]
    #[inline(always)]
    pub fn db(&self) -> DB_R {
        DB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PKHA Busy"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MDHA Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Done Interrupt"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Error Interrupt"]
    #[inline(always)]
    pub fn ei(&self) -> EI_R {
        EI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is asserted after POR and after every 50K blocks processed by AESA to indicate it is advisable for added security to write a new seed to"]
    #[inline(always)]
    pub fn dparrn(&self) -> DPARRN_R {
        DPARRN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Public Key is Prime"]
    #[inline(always)]
    pub fn pkp(&self) -> PKP_R {
        PKP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Public Key Operation is One"]
    #[inline(always)]
    pub fn pko(&self) -> PKO_R {
        PKO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Public Key Operation is Zero"]
    #[inline(always)]
    pub fn pkz(&self) -> PKZ_R {
        PKZ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Done Interrupt"]
    #[inline(always)]
    pub fn di(&mut self) -> DI_W {
        DI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_sta](index.html) module"]
pub struct LTC0_STA_SPEC;
impl crate::RegisterSpec for LTC0_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_sta::R](R) reader structure"]
impl crate::Readable for LTC0_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_sta::W](W) writer structure"]
impl crate::Writable for LTC0_STA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_STA to value 0"]
impl crate::Resettable for LTC0_STA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
