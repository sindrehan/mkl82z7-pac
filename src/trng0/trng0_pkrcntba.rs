#[doc = "Register `TRNG0_PKRCNTBA` reader"]
pub struct R(crate::R<TRNG0_PKRCNTBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_PKRCNTBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_PKRCNTBA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_PKRCNTBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_A_CT` reader - Poker Ah Count"]
pub struct PKR_A_CT_R(crate::FieldReader<u16, u16>);
impl PKR_A_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKR_A_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_A_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKR_B_CT` reader - Poker Bh Count"]
pub struct PKR_B_CT_R(crate::FieldReader<u16, u16>);
impl PKR_B_CT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PKR_B_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKR_B_CT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Poker Ah Count"]
    #[inline(always)]
    pub fn pkr_a_ct(&self) -> PKR_A_CT_R {
        PKR_A_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker Bh Count"]
    #[inline(always)]
    pub fn pkr_b_ct(&self) -> PKR_B_CT_R {
        PKR_B_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TRNG0 Statistical Check Poker Count B and A Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_pkrcntba](index.html) module"]
pub struct TRNG0_PKRCNTBA_SPEC;
impl crate::RegisterSpec for TRNG0_PKRCNTBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_pkrcntba::R](R) reader structure"]
impl crate::Readable for TRNG0_PKRCNTBA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_PKRCNTBA to value 0"]
impl crate::Resettable for TRNG0_PKRCNTBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
