#[doc = "Register `TRNG0_PKRCNT98` reader"]
pub struct R(crate::R<TRNG0_PKRCNT98_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_PKRCNT98_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_PKRCNT98_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_PKRCNT98_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_8_CT` reader - Poker 8h Count"]
pub struct PKR_8_CT_R(crate::FieldReader<u16, u16>);
impl PKR_8_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKR_8_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_8_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKR_9_CT` reader - Poker 9h Count"]
pub struct PKR_9_CT_R(crate::FieldReader<u16, u16>);
impl PKR_9_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKR_9_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_9_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Poker 8h Count"]
    #[inline(always)]
    pub fn pkr_8_ct(&self) -> PKR_8_CT_R {
        PKR_8_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 9h Count"]
    #[inline(always)]
    pub fn pkr_9_ct(&self) -> PKR_9_CT_R {
        PKR_9_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TRNG0 Statistical Check Poker Count 9 and 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcnt98](index.html) module"]
pub struct TRNG0_PKRCNT98_SPEC;
impl crate::RegisterSpec for TRNG0_PKRCNT98_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_pkrcnt98::R](R) reader structure"]
impl crate::Readable for TRNG0_PKRCNT98_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_PKRCNT98 to value 0"]
impl crate::Resettable for TRNG0_PKRCNT98_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
