#[doc = "Reader of register FACSN"]
pub type R = crate::R<u8, super::FACSN>;
#[doc = "Number of Segments Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUMSG_A {
    #[doc = "32: Program flash memory is divided into 32 segments (64 Kbytes, 128 Kbytes)"]
    _100000 = 32,
    #[doc = "40: Program flash memory is divided into 40 segments (160 Kbytes)"]
    _101000 = 40,
    #[doc = "64: Program flash memory is divided into 64 segments (256 Kbytes, 512 Kbytes)"]
    _1000000 = 64,
}
impl From<NUMSG_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMSG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NUMSG`"]
pub type NUMSG_R = crate::R<u8, NUMSG_A>;
impl NUMSG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NUMSG_A> {
        use crate::Variant::*;
        match self.bits {
            32 => Val(NUMSG_A::_100000),
            40 => Val(NUMSG_A::_101000),
            64 => Val(NUMSG_A::_1000000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline(always)]
    pub fn is_100000(&self) -> bool {
        *self == NUMSG_A::_100000
    }
    #[doc = "Checks if the value of the field is `_101000`"]
    #[inline(always)]
    pub fn is_101000(&self) -> bool {
        *self == NUMSG_A::_101000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline(always)]
    pub fn is_1000000(&self) -> bool {
        *self == NUMSG_A::_1000000
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Segments Indicator"]
    #[inline(always)]
    pub fn numsg(&self) -> NUMSG_R {
        NUMSG_R::new((self.bits & 0xff) as u8)
    }
}
