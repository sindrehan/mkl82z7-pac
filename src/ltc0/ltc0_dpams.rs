#[doc = "Register `LTC0_DPAMS` writer"]
pub struct W(crate::W<LTC0_DPAMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_DPAMS_SPEC>;
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
impl From<crate::W<LTC0_DPAMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_DPAMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPAMS` writer - Differential Power Analysis Mask Seed"]
pub struct DPAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPAMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Differential Power Analysis Mask Seed"]
    #[inline(always)]
    pub fn dpams(&mut self) -> DPAMS_W {
        DPAMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC DPA Mask Seed Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_dpams](index.html) module"]
pub struct LTC0_DPAMS_SPEC;
impl crate::RegisterSpec for LTC0_DPAMS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ltc0_dpams::W](W) writer structure"]
impl crate::Writable for LTC0_DPAMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_DPAMS to value 0"]
impl crate::Resettable for LTC0_DPAMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
