#[doc = "Reader of register C5"]
pub type R = crate::R<u8, super::C5>;
#[doc = "Writer for register C5"]
pub type W = crate::W<u8, super::C5>;
#[doc = "Register C5 `reset()`'s with value 0"]
impl crate::ResetValue for super::C5 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRDIV`"]
pub type PRDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRDIV`"]
pub struct PRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "PLL Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTEN_A {
    #[doc = "0: MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    _1 = 1,
}
impl From<PLLSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLSTEN`"]
pub type PLLSTEN_R = crate::R<bool, PLLSTEN_A>;
impl PLLSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTEN_A {
        match self.bits {
            false => PLLSTEN_A::_0,
            true => PLLSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSTEN_A::_1
    }
}
#[doc = "Write proxy for field `PLLSTEN`"]
pub struct PLLSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are disabled in any of the Stop modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTEN_A::_0)
    }
    #[doc = "MCGPLLCLK and MCGPLLCLK2X are enabled if system is in Normal Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "PLL Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCLKEN_A {
    #[doc = "0: MCGPLLCLK is inactive."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK is active."]
    _1 = 1,
}
impl From<PLLCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLCLKEN`"]
pub type PLLCLKEN_R = crate::R<bool, PLLCLKEN_A>;
impl PLLCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLCLKEN_A {
        match self.bits {
            false => PLLCLKEN_A::_0,
            true => PLLCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLCLKEN_A::_1
    }
}
#[doc = "Write proxy for field `PLLCLKEN`"]
pub struct PLLCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGPLLCLK is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCLKEN_A::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCLKEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv(&self) -> PRDIV_R {
        PRDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten(&self) -> PLLSTEN_R {
        PLLSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken(&self) -> PLLCLKEN_R {
        PLLCLKEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv(&mut self) -> PRDIV_W {
        PRDIV_W { w: self }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten(&mut self) -> PLLSTEN_W {
        PLLSTEN_W { w: self }
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken(&mut self) -> PLLCLKEN_W {
        PLLCLKEN_W { w: self }
    }
}
