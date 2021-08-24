#[doc = "Register `GPCNT0_VAL` reader"]
pub struct R(crate::R<GPCNT0_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPCNT0_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPCNT0_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPCNT0_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPCNT0_VAL` writer"]
pub struct W(crate::W<GPCNT0_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCNT0_VAL_SPEC>;
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
impl From<crate::W<GPCNT0_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCNT0_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPCNT0` reader - General Purpose Counter 0 Timeout Value"]
pub struct GPCNT0_R(crate::FieldReader<u16, u16>);
impl GPCNT0_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPCNT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPCNT0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPCNT0` writer - General Purpose Counter 0 Timeout Value"]
pub struct GPCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCNT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - General Purpose Counter 0 Timeout Value"]
    #[inline(always)]
    pub fn gpcnt0(&self) -> GPCNT0_R {
        GPCNT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - General Purpose Counter 0 Timeout Value"]
    #[inline(always)]
    pub fn gpcnt0(&mut self) -> GPCNT0_W {
        GPCNT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Counter 0 Timeout Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpcnt0_val](index.html) module"]
pub struct GPCNT0_VAL_SPEC;
impl crate::RegisterSpec for GPCNT0_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpcnt0_val::R](R) reader structure"]
impl crate::Readable for GPCNT0_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpcnt0_val::W](W) writer structure"]
impl crate::Writable for GPCNT0_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPCNT0_VAL to value 0xffff"]
impl crate::Resettable for GPCNT0_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
