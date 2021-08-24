#[doc = "Register `TRNG0_PKRCNT10` reader"]
pub struct R(crate::R<TRNG0_PKRCNT10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_PKRCNT10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_PKRCNT10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_PKRCNT10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_0_CT` reader - Poker 0h Count"]
pub struct PKR_0_CT_R(crate::FieldReader<u16, u16>);
impl PKR_0_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKR_0_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_0_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKR_1_CT` reader - Poker 1h Count"]
pub struct PKR_1_CT_R(crate::FieldReader<u16, u16>);
impl PKR_1_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKR_1_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_1_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Poker 0h Count"]
    #[inline(always)]
    pub fn pkr_0_ct(&self) -> PKR_0_CT_R {
        PKR_0_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 1h Count"]
    #[inline(always)]
    pub fn pkr_1_ct(&self) -> PKR_1_CT_R {
        PKR_1_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TRNG0 Statistical Check Poker Count 1 and 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcnt10](index.html) module"]
pub struct TRNG0_PKRCNT10_SPEC;
impl crate::RegisterSpec for TRNG0_PKRCNT10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_pkrcnt10::R](R) reader structure"]
impl crate::Readable for TRNG0_PKRCNT10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_PKRCNT10 to value 0"]
impl crate::Resettable for TRNG0_PKRCNT10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
