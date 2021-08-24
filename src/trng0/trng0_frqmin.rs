#[doc = "Register `TRNG0_FRQMIN` reader"]
pub struct R(crate::R<TRNG0_FRQMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_FRQMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_FRQMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_FRQMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_FRQMIN` writer"]
pub struct W(crate::W<TRNG0_FRQMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_FRQMIN_SPEC>;
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
impl From<crate::W<TRNG0_FRQMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_FRQMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRQ_MIN` reader - Frequency Count Minimum Limit"]
pub struct FRQ_MIN_R(crate::FieldReader<u32, u32>);
impl FRQ_MIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        FRQ_MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRQ_MIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRQ_MIN` writer - Frequency Count Minimum Limit"]
pub struct FRQ_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQ_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Frequency Count Minimum Limit"]
    #[inline(always)]
    pub fn frq_min(&self) -> FRQ_MIN_R {
        FRQ_MIN_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Frequency Count Minimum Limit"]
    #[inline(always)]
    pub fn frq_min(&mut self) -> FRQ_MIN_W {
        FRQ_MIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Frequency Count Minimum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_frqmin](index.html) module"]
pub struct TRNG0_FRQMIN_SPEC;
impl crate::RegisterSpec for TRNG0_FRQMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_frqmin::R](R) reader structure"]
impl crate::Readable for TRNG0_FRQMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_frqmin::W](W) writer structure"]
impl crate::Writable for TRNG0_FRQMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_FRQMIN to value 0x0640"]
impl crate::Resettable for TRNG0_FRQMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0640
    }
}
