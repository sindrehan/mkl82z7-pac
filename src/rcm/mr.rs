#[doc = "Reader of register MR"]
pub type R = crate::R<u8, super::MR>;
#[doc = "Writer for register MR"]
pub type W = crate::W<u8, super::MR>;
#[doc = "Register MR `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Boot ROM Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOTROM_A {
    #[doc = "0: Boot from Flash"]
    _00 = 0,
    #[doc = "1: Boot from ROM due to BOOTCFG0 pin assertion"]
    _01 = 1,
    #[doc = "2: Boot form ROM due to FOPT\\[7\\]
configuration"]
    _10 = 2,
    #[doc = "3: Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\]
configuration"]
    _11 = 3,
}
impl From<BOOTROM_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTROM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BOOTROM`"]
pub type BOOTROM_R = crate::R<u8, BOOTROM_A>;
impl BOOTROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTROM_A {
        match self.bits {
            0 => BOOTROM_A::_00,
            1 => BOOTROM_A::_01,
            2 => BOOTROM_A::_10,
            3 => BOOTROM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BOOTROM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BOOTROM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BOOTROM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BOOTROM_A::_11
    }
}
#[doc = "Write proxy for field `BOOTROM`"]
pub struct BOOTROM_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOTROM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Boot from Flash"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BOOTROM_A::_00)
    }
    #[doc = "Boot from ROM due to BOOTCFG0 pin assertion"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BOOTROM_A::_01)
    }
    #[doc = "Boot form ROM due to FOPT\\[7\\]
configuration"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BOOTROM_A::_10)
    }
    #[doc = "Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\]
configuration"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BOOTROM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u8) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - Boot ROM Configuration"]
    #[inline(always)]
    pub fn bootrom(&self) -> BOOTROM_R {
        BOOTROM_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Boot ROM Configuration"]
    #[inline(always)]
    pub fn bootrom(&mut self) -> BOOTROM_W {
        BOOTROM_W { w: self }
    }
}
