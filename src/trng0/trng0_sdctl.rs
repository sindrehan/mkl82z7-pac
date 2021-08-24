#[doc = "Register `TRNG0_SDCTL` reader"]
pub struct R(crate::R<TRNG0_SDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_SDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_SDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_SDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_SDCTL` writer"]
pub struct W(crate::W<TRNG0_SDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_SDCTL_SPEC>;
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
impl From<crate::W<TRNG0_SDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_SDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMP_SIZE` reader - Sample Size"]
pub struct SAMP_SIZE_R(crate::FieldReader<u16, u16>);
impl SAMP_SIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        SAMP_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMP_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMP_SIZE` writer - Sample Size"]
pub struct SAMP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `ENT_DLY` reader - Entropy Delay"]
pub struct ENT_DLY_R(crate::FieldReader<u16, u16>);
impl ENT_DLY_R {
    pub(crate) fn new(bits: u16) -> Self {
        ENT_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENT_DLY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENT_DLY` writer - Entropy Delay"]
pub struct ENT_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> ENT_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Sample Size"]
    #[inline(always)]
    pub fn samp_size(&self) -> SAMP_SIZE_R {
        SAMP_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Entropy Delay"]
    #[inline(always)]
    pub fn ent_dly(&self) -> ENT_DLY_R {
        ENT_DLY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sample Size"]
    #[inline(always)]
    pub fn samp_size(&mut self) -> SAMP_SIZE_W {
        SAMP_SIZE_W { w: self }
    }
    #[doc = "Bits 16:31 - Entropy Delay"]
    #[inline(always)]
    pub fn ent_dly(&mut self) -> ENT_DLY_W {
        ENT_DLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Seed Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_sdctl](index.html) module"]
pub struct TRNG0_SDCTL_SPEC;
impl crate::RegisterSpec for TRNG0_SDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_sdctl::R](R) reader structure"]
impl crate::Readable for TRNG0_SDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_sdctl::W](W) writer structure"]
impl crate::Writable for TRNG0_SDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_SDCTL to value 0x0c80_09c4"]
impl crate::Resettable for TRNG0_SDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c80_09c4
    }
}
