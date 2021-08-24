#[doc = "Register `GPCNT1_VAL` reader"]
pub struct R(crate::R<GPCNT1_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPCNT1_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPCNT1_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPCNT1_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPCNT1_VAL` writer"]
pub struct W(crate::W<GPCNT1_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCNT1_VAL_SPEC>;
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
impl From<crate::W<GPCNT1_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCNT1_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPCNT1` reader - General Purpose Counter 1 Timeout Value"]
pub struct GPCNT1_R(crate::FieldReader<u16, u16>);
impl GPCNT1_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPCNT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPCNT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPCNT1` writer - General Purpose Counter 1 Timeout Value"]
pub struct GPCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCNT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - General Purpose Counter 1 Timeout Value"]
    #[inline(always)]
    pub fn gpcnt1(&self) -> GPCNT1_R {
        GPCNT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - General Purpose Counter 1 Timeout Value"]
    #[inline(always)]
    pub fn gpcnt1(&mut self) -> GPCNT1_W {
        GPCNT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Counter 1 Timeout Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpcnt1_val](index.html) module"]
pub struct GPCNT1_VAL_SPEC;
impl crate::RegisterSpec for GPCNT1_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpcnt1_val::R](R) reader structure"]
impl crate::Readable for GPCNT1_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpcnt1_val::W](W) writer structure"]
impl crate::Writable for GPCNT1_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPCNT1_VAL to value 0xffff"]
impl crate::Resettable for GPCNT1_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
