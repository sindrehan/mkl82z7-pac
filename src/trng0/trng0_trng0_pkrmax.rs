#[doc = "Register `TRNG0_PKRMAX` reader"]
pub struct R(crate::R<TRNG0_TRNG0_PKRMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_PKRMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_PKRMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_PKRMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_PKRMAX` writer"]
pub struct W(crate::W<TRNG0_TRNG0_PKRMAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_TRNG0_PKRMAX_SPEC>;
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
impl From<crate::W<TRNG0_TRNG0_PKRMAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_TRNG0_PKRMAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKR_MAX` reader - Poker Maximum Limit"]
pub struct PKR_MAX_R(crate::FieldReader<u32, u32>);
impl PKR_MAX_R {
    pub(crate) fn new(bits: u32) -> Self {
        PKR_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_MAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKR_MAX` writer - Poker Maximum Limit"]
pub struct PKR_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> PKR_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Poker Maximum Limit"]
    #[inline(always)]
    pub fn pkr_max(&self) -> PKR_MAX_R {
        PKR_MAX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Poker Maximum Limit"]
    #[inline(always)]
    pub fn pkr_max(&mut self) -> PKR_MAX_W {
        PKR_MAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Poker Maximum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_pkrmax](index.html) module"]
pub struct TRNG0_TRNG0_PKRMAX_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_PKRMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_pkrmax::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_PKRMAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_trng0_pkrmax::W](W) writer structure"]
impl crate::Writable for TRNG0_TRNG0_PKRMAX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_PKRMAX to value 0x6920"]
impl crate::Resettable for TRNG0_TRNG0_PKRMAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6920
    }
}
