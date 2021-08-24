#[doc = "Register `REG%s` reader"]
pub struct R(crate::R<REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG%s` writer"]
pub struct W(crate::W<REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SPEC>;
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
impl From<crate::W<REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LL` reader - Low lower byte"]
pub struct LL_R(crate::FieldReader<u8, u8>);
impl LL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LL` writer - Low lower byte"]
pub struct LL_W<'a> {
    w: &'a mut W,
}
impl<'a> LL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `LH` reader - Low higher byte"]
pub struct LH_R(crate::FieldReader<u8, u8>);
impl LH_R {
    pub(crate) fn new(bits: u8) -> Self {
        LH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LH` writer - Low higher byte"]
pub struct LH_W<'a> {
    w: &'a mut W,
}
impl<'a> LH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `HL` reader - High lower byte"]
pub struct HL_R(crate::FieldReader<u8, u8>);
impl HL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HL` writer - High lower byte"]
pub struct HL_W<'a> {
    w: &'a mut W,
}
impl<'a> HL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `HH` reader - High higher byte"]
pub struct HH_R(crate::FieldReader<u8, u8>);
impl HH_R {
    pub(crate) fn new(bits: u8) -> Self {
        HH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HH` writer - High higher byte"]
pub struct HH_W<'a> {
    w: &'a mut W,
}
impl<'a> HH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low lower byte"]
    #[inline(always)]
    pub fn ll(&self) -> LL_R {
        LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Low higher byte"]
    #[inline(always)]
    pub fn lh(&self) -> LH_R {
        LH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - High lower byte"]
    #[inline(always)]
    pub fn hl(&self) -> HL_R {
        HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - High higher byte"]
    #[inline(always)]
    pub fn hh(&self) -> HH_R {
        HH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low lower byte"]
    #[inline(always)]
    pub fn ll(&mut self) -> LL_W {
        LL_W { w: self }
    }
    #[doc = "Bits 8:15 - Low higher byte"]
    #[inline(always)]
    pub fn lh(&mut self) -> LH_W {
        LH_W { w: self }
    }
    #[doc = "Bits 16:23 - High lower byte"]
    #[inline(always)]
    pub fn hl(&mut self) -> HL_W {
        HL_W { w: self }
    }
    #[doc = "Bits 24:31 - High higher byte"]
    #[inline(always)]
    pub fn hh(&mut self) -> HH_W {
        HH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register file register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg](index.html) module"]
pub struct REG_SPEC;
impl crate::RegisterSpec for REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg::R](R) reader structure"]
impl crate::Readable for REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg::W](W) writer structure"]
impl crate::Writable for REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG%s to value 0"]
impl crate::Resettable for REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
