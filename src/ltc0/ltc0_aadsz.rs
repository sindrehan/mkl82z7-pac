#[doc = "Register `LTC0_AADSZ` reader"]
pub struct R(crate::R<LTC0_AADSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_AADSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_AADSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_AADSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_AADSZ` writer"]
pub struct W(crate::W<LTC0_AADSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_AADSZ_SPEC>;
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
impl From<crate::W<LTC0_AADSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_AADSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AADSZ` reader - AAD size in Bytes, mod 16."]
pub struct AADSZ_R(crate::FieldReader<u8, u8>);
impl AADSZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        AADSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AADSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AADSZ` writer - AAD size in Bytes, mod 16."]
pub struct AADSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> AADSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `AL` reader - AAD Last. Only AAD data will be written into the Input FIFO."]
pub struct AL_R(crate::FieldReader<bool, bool>);
impl AL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AL` writer - AAD Last. Only AAD data will be written into the Input FIFO."]
pub struct AL_W<'a> {
    w: &'a mut W,
}
impl<'a> AL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - AAD size in Bytes, mod 16."]
    #[inline(always)]
    pub fn aadsz(&self) -> AADSZ_R {
        AADSZ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - AAD Last. Only AAD data will be written into the Input FIFO."]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - AAD size in Bytes, mod 16."]
    #[inline(always)]
    pub fn aadsz(&mut self) -> AADSZ_W {
        AADSZ_W { w: self }
    }
    #[doc = "Bit 31 - AAD Last. Only AAD data will be written into the Input FIFO."]
    #[inline(always)]
    pub fn al(&mut self) -> AL_W {
        AL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC AAD Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_aadsz](index.html) module"]
pub struct LTC0_AADSZ_SPEC;
impl crate::RegisterSpec for LTC0_AADSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_aadsz::R](R) reader structure"]
impl crate::Readable for LTC0_AADSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_aadsz::W](W) writer structure"]
impl crate::Writable for LTC0_AADSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_AADSZ to value 0"]
impl crate::Resettable for LTC0_AADSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
