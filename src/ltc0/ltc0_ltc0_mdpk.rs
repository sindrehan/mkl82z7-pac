#[doc = "Register `LTC0_MDPK` reader"]
pub struct R(crate::R<LTC0_LTC0_MDPK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_LTC0_MDPK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_LTC0_MDPK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_LTC0_MDPK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_MDPK` writer"]
pub struct W(crate::W<LTC0_LTC0_MDPK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_LTC0_MDPK_SPEC>;
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
impl From<crate::W<LTC0_LTC0_MDPK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_LTC0_MDPK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKHA_MODE_LS` reader - PKHA_MODE least significant 12 bits"]
pub struct PKHA_MODE_LS_R(crate::FieldReader<u16, u16>);
impl PKHA_MODE_LS_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKHA_MODE_LS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKHA_MODE_LS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKHA_MODE_LS` writer - PKHA_MODE least significant 12 bits"]
pub struct PKHA_MODE_LS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_MODE_LS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `PKHA_MODE_MS` reader - PKHA_MODE most-significant 4 bits"]
pub struct PKHA_MODE_MS_R(crate::FieldReader<u8, u8>);
impl PKHA_MODE_MS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKHA_MODE_MS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKHA_MODE_MS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKHA_MODE_MS` writer - PKHA_MODE most-significant 4 bits"]
pub struct PKHA_MODE_MS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_MODE_MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Algorithm. This field specifies which algorithm is being selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALG_A {
    #[doc = "8: PKHA"]
    _1000 = 8,
}
impl From<ALG_A> for u8 {
    #[inline(always)]
    fn from(variant: ALG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ALG` reader - Algorithm. This field specifies which algorithm is being selected."]
pub struct ALG_R(crate::FieldReader<u8, ALG_A>);
impl ALG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALG_A> {
        match self.bits {
            8 => Some(ALG_A::_1000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == ALG_A::_1000
    }
}
impl core::ops::Deref for ALG_R {
    type Target = crate::FieldReader<u8, ALG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALG` writer - Algorithm. This field specifies which algorithm is being selected."]
pub struct ALG_W<'a> {
    w: &'a mut W,
}
impl<'a> ALG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PKHA"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ALG_A::_1000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - PKHA_MODE least significant 12 bits"]
    #[inline(always)]
    pub fn pkha_mode_ls(&self) -> PKHA_MODE_LS_R {
        PKHA_MODE_LS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - PKHA_MODE most-significant 4 bits"]
    #[inline(always)]
    pub fn pkha_mode_ms(&self) -> PKHA_MODE_MS_R {
        PKHA_MODE_MS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline(always)]
    pub fn alg(&self) -> ALG_R {
        ALG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - PKHA_MODE least significant 12 bits"]
    #[inline(always)]
    pub fn pkha_mode_ls(&mut self) -> PKHA_MODE_LS_W {
        PKHA_MODE_LS_W { w: self }
    }
    #[doc = "Bits 16:19 - PKHA_MODE most-significant 4 bits"]
    #[inline(always)]
    pub fn pkha_mode_ms(&mut self) -> PKHA_MODE_MS_W {
        PKHA_MODE_MS_W { w: self }
    }
    #[doc = "Bits 20:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline(always)]
    pub fn alg(&mut self) -> ALG_W {
        ALG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC Mode Register (PublicKey)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ltc0_mdpk](index.html) module"]
pub struct LTC0_LTC0_MDPK_SPEC;
impl crate::RegisterSpec for LTC0_LTC0_MDPK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_ltc0_mdpk::R](R) reader structure"]
impl crate::Readable for LTC0_LTC0_MDPK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_ltc0_mdpk::W](W) writer structure"]
impl crate::Writable for LTC0_LTC0_MDPK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_MDPK to value 0"]
impl crate::Resettable for LTC0_LTC0_MDPK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
