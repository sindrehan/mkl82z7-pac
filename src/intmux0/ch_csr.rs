#[doc = "Reader of register CH%s_CSR"]
pub type R = crate::R<u32, super::CH_CSR>;
#[doc = "Writer for register CH%s_CSR"]
pub type W = crate::W<u32, super::CH_CSR>;
#[doc = "Register CH%s_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: No operation."]
    _0 = 0,
    #[doc = "1: Perform a software reset on this channel."]
    _1 = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, RST_A>;
impl RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::_0,
            true => RST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RST_A::_1
    }
}
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RST_A::_0)
    }
    #[doc = "Perform a software reset on this channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RST_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Logic AND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AND_A {
    #[doc = "0: Logic OR all enabled interrupt inputs."]
    _0 = 0,
    #[doc = "1: Logic AND all enabled interrupt inputs."]
    _1 = 1,
}
impl From<AND_A> for bool {
    #[inline(always)]
    fn from(variant: AND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AND`"]
pub type AND_R = crate::R<bool, AND_A>;
impl AND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AND_A {
        match self.bits {
            false => AND_A::_0,
            true => AND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AND_A::_1
    }
}
#[doc = "Write proxy for field `AND`"]
pub struct AND_W<'a> {
    w: &'a mut W,
}
impl<'a> AND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Logic OR all enabled interrupt inputs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AND_A::_0)
    }
    #[doc = "Logic AND all enabled interrupt inputs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AND_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Channel Input Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IRQN_A {
    #[doc = "0: 32 interrupt inputs"]
    _00 = 0,
}
impl From<IRQN_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IRQN`"]
pub type IRQN_R = crate::R<u8, IRQN_A>;
impl IRQN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IRQN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IRQN_A::_00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IRQN_A::_00
    }
}
#[doc = "Reader of field `CHIN`"]
pub type CHIN_R = crate::R<u8, u8>;
#[doc = "Channel Interrupt Request Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQP_A {
    #[doc = "0: No interrupt is pending."]
    _0 = 0,
    #[doc = "1: The interrupt output of this channel is pending."]
    _1 = 1,
}
impl From<IRQP_A> for bool {
    #[inline(always)]
    fn from(variant: IRQP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRQP`"]
pub type IRQP_R = crate::R<bool, IRQP_A>;
impl IRQP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQP_A {
        match self.bits {
            false => IRQP_A::_0,
            true => IRQP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQP_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logic AND"]
    #[inline(always)]
    pub fn and(&self) -> AND_R {
        AND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Channel Input Number"]
    #[inline(always)]
    pub fn irqn(&self) -> IRQN_R {
        IRQN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Channel Instance Number"]
    #[inline(always)]
    pub fn chin(&self) -> CHIN_R {
        CHIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Channel Interrupt Request Pending"]
    #[inline(always)]
    pub fn irqp(&self) -> IRQP_R {
        IRQP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 1 - Logic AND"]
    #[inline(always)]
    pub fn and(&mut self) -> AND_W {
        AND_W { w: self }
    }
}
