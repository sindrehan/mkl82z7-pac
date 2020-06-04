#[doc = "Reader of register CLKCFG"]
pub type R = crate::R<u32, super::CLKCFG>;
#[doc = "Writer for register CLKCFG"]
pub type W = crate::W<u32, super::CLKCFG>;
#[doc = "Register CLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_PRSC`"]
pub type CLK_PRSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_PRSC`"]
pub struct CLK_PRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "General Purpose Counter 1 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPCNT1_CLK_SEL_A {
    #[doc = "0: Disabled / Reset (default)"]
    _00 = 0,
    #[doc = "1: Card Clock"]
    _01 = 1,
    #[doc = "2: Receive Clock"]
    _10 = 2,
    #[doc = "3: ETU Clock (transmit clock)"]
    _11 = 3,
}
impl From<GPCNT1_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPCNT1_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPCNT1_CLK_SEL`"]
pub type GPCNT1_CLK_SEL_R = crate::R<u8, GPCNT1_CLK_SEL_A>;
impl GPCNT1_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCNT1_CLK_SEL_A {
        match self.bits {
            0 => GPCNT1_CLK_SEL_A::_00,
            1 => GPCNT1_CLK_SEL_A::_01,
            2 => GPCNT1_CLK_SEL_A::_10,
            3 => GPCNT1_CLK_SEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GPCNT1_CLK_SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GPCNT1_CLK_SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == GPCNT1_CLK_SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == GPCNT1_CLK_SEL_A::_11
    }
}
#[doc = "Write proxy for field `GPCNT1_CLK_SEL`"]
pub struct GPCNT1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCNT1_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCNT1_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled / Reset (default)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SEL_A::_00)
    }
    #[doc = "Card Clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SEL_A::_01)
    }
    #[doc = "Receive Clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SEL_A::_10)
    }
    #[doc = "ETU Clock (transmit clock)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "General Purpose Counter 0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPCNT0_CLK_SEL_A {
    #[doc = "0: Disabled / Reset (default)"]
    _00 = 0,
    #[doc = "1: Card Clock"]
    _01 = 1,
    #[doc = "2: Receive Clock"]
    _10 = 2,
    #[doc = "3: ETU Clock (transmit clock)"]
    _11 = 3,
}
impl From<GPCNT0_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPCNT0_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPCNT0_CLK_SEL`"]
pub type GPCNT0_CLK_SEL_R = crate::R<u8, GPCNT0_CLK_SEL_A>;
impl GPCNT0_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCNT0_CLK_SEL_A {
        match self.bits {
            0 => GPCNT0_CLK_SEL_A::_00,
            1 => GPCNT0_CLK_SEL_A::_01,
            2 => GPCNT0_CLK_SEL_A::_10,
            3 => GPCNT0_CLK_SEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GPCNT0_CLK_SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GPCNT0_CLK_SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == GPCNT0_CLK_SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == GPCNT0_CLK_SEL_A::_11
    }
}
#[doc = "Write proxy for field `GPCNT0_CLK_SEL`"]
pub struct GPCNT0_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCNT0_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCNT0_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled / Reset (default)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SEL_A::_00)
    }
    #[doc = "Card Clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SEL_A::_01)
    }
    #[doc = "Receive Clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SEL_A::_10)
    }
    #[doc = "ETU Clock (transmit clock)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Prescaler Value"]
    #[inline(always)]
    pub fn clk_prsc(&self) -> CLK_PRSC_R {
        CLK_PRSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - General Purpose Counter 1 Clock Select"]
    #[inline(always)]
    pub fn gpcnt1_clk_sel(&self) -> GPCNT1_CLK_SEL_R {
        GPCNT1_CLK_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - General Purpose Counter 0 Clock Select"]
    #[inline(always)]
    pub fn gpcnt0_clk_sel(&self) -> GPCNT0_CLK_SEL_R {
        GPCNT0_CLK_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Prescaler Value"]
    #[inline(always)]
    pub fn clk_prsc(&mut self) -> CLK_PRSC_W {
        CLK_PRSC_W { w: self }
    }
    #[doc = "Bits 8:9 - General Purpose Counter 1 Clock Select"]
    #[inline(always)]
    pub fn gpcnt1_clk_sel(&mut self) -> GPCNT1_CLK_SEL_W {
        GPCNT1_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - General Purpose Counter 0 Clock Select"]
    #[inline(always)]
    pub fn gpcnt0_clk_sel(&mut self) -> GPCNT0_CLK_SEL_W {
        GPCNT0_CLK_SEL_W { w: self }
    }
}
