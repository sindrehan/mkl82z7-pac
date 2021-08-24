#[doc = "Register `DLPR` reader"]
pub struct R(crate::R<DLPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLPR` writer"]
pub struct W(crate::W<DLPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLPR_SPEC>;
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
impl From<crate::W<DLPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLPV` reader - Data Learning Pattern Value: This value is used for data learning in DDR and DQS mode"]
pub struct DLPV_R(crate::FieldReader<u32, u32>);
impl DLPV_R {
    pub(crate) fn new(bits: u32) -> Self {
        DLPV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLPV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLPV` writer - Data Learning Pattern Value: This value is used for data learning in DDR and DQS mode"]
pub struct DLPV_W<'a> {
    w: &'a mut W,
}
impl<'a> DLPV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Learning Pattern Value: This value is used for data learning in DDR and DQS mode"]
    #[inline(always)]
    pub fn dlpv(&self) -> DLPV_R {
        DLPV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Learning Pattern Value: This value is used for data learning in DDR and DQS mode"]
    #[inline(always)]
    pub fn dlpv(&mut self) -> DLPV_W {
        DLPV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Learn Pattern Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlpr](index.html) module"]
pub struct DLPR_SPEC;
impl crate::RegisterSpec for DLPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlpr::R](R) reader structure"]
impl crate::Readable for DLPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlpr::W](W) writer structure"]
impl crate::Writable for DLPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLPR to value 0xaa55_3443"]
impl crate::Resettable for DLPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaa55_3443
    }
}
