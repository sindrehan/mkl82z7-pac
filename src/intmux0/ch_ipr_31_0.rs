#[doc = "Reader of register CH%s_IPR_31_0"]
pub type R = crate::R<u32, super::CH_IPR_31_0>;
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
#[doc = "Reader of field `INTP`"]
pub type INTP_R = crate::R<u32, INTP_A>;
impl INTP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, INTP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTP_A::_0),
            1 => Val(INTP_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTP_A::_1
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Pending"]
    #[inline(always)]
    pub fn intp(&self) -> INTP_R {
        INTP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
