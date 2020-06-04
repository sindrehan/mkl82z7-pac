#[doc = "Reader of register OTGICR"]
pub type R = crate::R<u8, super::OTGICR>;
#[doc = "Writer for register OTGICR"]
pub type W = crate::W<u8, super::OTGICR>;
#[doc = "Register OTGICR `reset()`'s with value 0"]
impl crate::ResetValue for super::OTGICR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Line State Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATEEN_A {
    #[doc = "0: Disables the LINE_STAT_CHG interrupt."]
    _0 = 0,
    #[doc = "1: Enables the LINE_STAT_CHG interrupt."]
    _1 = 1,
}
impl From<LINESTATEEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINESTATEEN`"]
pub type LINESTATEEN_R = crate::R<bool, LINESTATEEN_A>;
impl LINESTATEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATEEN_A {
        match self.bits {
            false => LINESTATEEN_A::_0,
            true => LINESTATEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINESTATEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINESTATEEN_A::_1
    }
}
#[doc = "Write proxy for field `LINESTATEEN`"]
pub struct LINESTATEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LINESTATEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINESTATEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATEEN_A::_0)
    }
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATEEN_A::_1)
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
#[doc = "One Millisecond Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEMSECEN_A {
    #[doc = "0: Diables the 1ms timer interrupt."]
    _0 = 0,
    #[doc = "1: Enables the 1ms timer interrupt."]
    _1 = 1,
}
impl From<ONEMSECEN_A> for bool {
    #[inline(always)]
    fn from(variant: ONEMSECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONEMSECEN`"]
pub type ONEMSECEN_R = crate::R<bool, ONEMSECEN_A>;
impl ONEMSECEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONEMSECEN_A {
        match self.bits {
            false => ONEMSECEN_A::_0,
            true => ONEMSECEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONEMSECEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONEMSECEN_A::_1
    }
}
#[doc = "Write proxy for field `ONEMSECEN`"]
pub struct ONEMSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEMSECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONEMSECEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Diables the 1ms timer interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONEMSECEN_A::_0)
    }
    #[doc = "Enables the 1ms timer interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONEMSECEN_A::_1)
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
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline(always)]
    pub fn linestateen(&self) -> LINESTATEEN_R {
        LINESTATEEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline(always)]
    pub fn onemsecen(&self) -> ONEMSECEN_R {
        ONEMSECEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline(always)]
    pub fn linestateen(&mut self) -> LINESTATEEN_W {
        LINESTATEEN_W { w: self }
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline(always)]
    pub fn onemsecen(&mut self) -> ONEMSECEN_W {
        ONEMSECEN_W { w: self }
    }
}
