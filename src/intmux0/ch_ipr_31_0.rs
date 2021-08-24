#[doc = "Register `CH%s_IPR_31_0` reader"]
pub struct R(crate::R<CH_IPR_31_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_IPR_31_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_IPR_31_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_IPR_31_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum INTP_A {
    #[doc = "0: No interrupt."]
    _0 = 0,
    #[doc = "1: Interrupt is pending."]
    _1 = 1,
}
impl From<INTP_A> for u32 {
    #[inline(always)]
    fn from(variant: INTP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTP` reader - Interrupt Pending"]
pub struct INTP_R(crate::FieldReader<u32, INTP_A>);
impl INTP_R {
    pub(crate) fn new(bits: u32) -> Self {
        INTP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTP_A> {
        match self.bits {
            0 => Some(INTP_A::_0),
            1 => Some(INTP_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTP_A::_1
    }
}
impl core::ops::Deref for INTP_R {
    type Target = crate::FieldReader<u32, INTP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Pending"]
    #[inline(always)]
    pub fn intp(&self) -> INTP_R {
        INTP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel n Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ipr_31_0](index.html) module"]
pub struct CH_IPR_31_0_SPEC;
impl crate::RegisterSpec for CH_IPR_31_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_ipr_31_0::R](R) reader structure"]
impl crate::Readable for CH_IPR_31_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%s_IPR_31_0 to value 0"]
impl crate::Resettable for CH_IPR_31_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
