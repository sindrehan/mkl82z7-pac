#[doc = "Reader of register FM"]
pub type R = crate::R<u8, super::FM>;
#[doc = "Writer for register FM"]
pub type W = crate::W<u8, super::FM>;
#[doc = "Register FM `reset()`'s with value 0"]
impl crate::ResetValue for super::FM {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force ROM Boot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORCEROM_A {
    #[doc = "0: No effect"]
    _00 = 0,
    #[doc = "1: Force boot from ROM with RCM_MR\\[1\\]
set."]
    _01 = 1,
    #[doc = "2: Force boot from ROM with RCM_MR\\[2\\]
set."]
    _10 = 2,
    #[doc = "3: Force boot from ROM with RCM_MR\\[2:1\\]
set."]
    _11 = 3,
}
impl From<FORCEROM_A> for u8 {
    #[inline(always)]
    fn from(variant: FORCEROM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORCEROM`"]
pub type FORCEROM_R = crate::R<u8, FORCEROM_A>;
impl FORCEROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEROM_A {
        match self.bits {
            0 => FORCEROM_A::_00,
            1 => FORCEROM_A::_01,
            2 => FORCEROM_A::_10,
            3 => FORCEROM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FORCEROM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FORCEROM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FORCEROM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FORCEROM_A::_11
    }
}
#[doc = "Write proxy for field `FORCEROM`"]
pub struct FORCEROM_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEROM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FORCEROM_A::_00)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[1\\]
set."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FORCEROM_A::_01)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2\\]
set."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FORCEROM_A::_10)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2:1\\]
set."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FORCEROM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u8) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - Force ROM Boot"]
    #[inline(always)]
    pub fn forcerom(&self) -> FORCEROM_R {
        FORCEROM_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Force ROM Boot"]
    #[inline(always)]
    pub fn forcerom(&mut self) -> FORCEROM_W {
        FORCEROM_W { w: self }
    }
}
