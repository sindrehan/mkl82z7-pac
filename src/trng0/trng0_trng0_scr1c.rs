#[doc = "Register `TRNG0_SCR1C` reader"]
pub struct R(crate::R<TRNG0_TRNG0_SCR1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_TRNG0_SCR1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_TRNG0_SCR1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_TRNG0_SCR1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R1_0_CT` reader - Runs of Zero, Length 1 Count"]
pub struct R1_0_CT_R(crate::FieldReader<u16, u16>);
impl R1_0_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        R1_0_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R1_0_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R1_1_CT` reader - Runs of One, Length 1 Count"]
pub struct R1_1_CT_R(crate::FieldReader<u16, u16>);
impl R1_1_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        R1_1_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R1_1_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:14 - Runs of Zero, Length 1 Count"]
    #[inline(always)]
    pub fn r1_0_ct(&self) -> R1_0_CT_R {
        R1_0_CT_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Runs of One, Length 1 Count"]
    #[inline(always)]
    pub fn r1_1_ct(&self) -> R1_1_CT_R {
        R1_1_CT_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "TRNG0 Statistical Check Run Length 1 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_trng0_scr1c](index.html) module"]
pub struct TRNG0_TRNG0_SCR1C_SPEC;
impl crate::RegisterSpec for TRNG0_TRNG0_SCR1C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_trng0_scr1c::R](R) reader structure"]
impl crate::Readable for TRNG0_TRNG0_SCR1C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_SCR1C to value 0"]
impl crate::Resettable for TRNG0_TRNG0_SCR1C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
