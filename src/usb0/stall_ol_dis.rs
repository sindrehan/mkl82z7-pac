#[doc = "Reader of register STALL_OL_DIS"]
pub type R = crate::R<u8, super::STALL_OL_DIS>;
#[doc = "Writer for register STALL_OL_DIS"]
pub type W = crate::W<u8, super::STALL_OL_DIS>;
#[doc = "Register STALL_OL_DIS `reset()`'s with value 0"]
impl crate::ResetValue for super::STALL_OL_DIS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Disable endpoint 0 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS0_A {
    #[doc = "0: Endpoint 0 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 0 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS0_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL_O_DIS0`"]
pub type STALL_O_DIS0_R = crate::R<bool, STALL_O_DIS0_A>;
impl STALL_O_DIS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS0_A {
        match self.bits {
            false => STALL_O_DIS0_A::_0,
            true => STALL_O_DIS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS0_A::_1
    }
}
#[doc = "Write proxy for field `STALL_O_DIS0`"]
pub struct STALL_O_DIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint 0 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS0_A::_0)
    }
    #[doc = "Endpoint 0 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS0_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Disable endpoint 1 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS1_A {
    #[doc = "0: Endpoint 1 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 1 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS1_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL_O_DIS1`"]
pub type STALL_O_DIS1_R = crate::R<bool, STALL_O_DIS1_A>;
impl STALL_O_DIS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS1_A {
        match self.bits {
            false => STALL_O_DIS1_A::_0,
            true => STALL_O_DIS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS1_A::_1
    }
}
#[doc = "Write proxy for field `STALL_O_DIS1`"]
pub struct STALL_O_DIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint 1 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS1_A::_0)
    }
    #[doc = "Endpoint 1 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Disable endpoint 2 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS2_A {
    #[doc = "0: Endpoint 2 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 2 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS2_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL_O_DIS2`"]
pub type STALL_O_DIS2_R = crate::R<bool, STALL_O_DIS2_A>;
impl STALL_O_DIS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS2_A {
        match self.bits {
            false => STALL_O_DIS2_A::_0,
            true => STALL_O_DIS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS2_A::_1
    }
}
#[doc = "Write proxy for field `STALL_O_DIS2`"]
pub struct STALL_O_DIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint 2 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS2_A::_0)
    }
    #[doc = "Endpoint 2 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Disable endpoint 3 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS3_A {
    #[doc = "0: Endpoint 3 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 3 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS3_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL_O_DIS3`"]
pub type STALL_O_DIS3_R = crate::R<bool, STALL_O_DIS3_A>;
impl STALL_O_DIS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS3_A {
        match self.bits {
            false => STALL_O_DIS3_A::_0,
            true => STALL_O_DIS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS3_A::_1
    }
}
#[doc = "Write proxy for field `STALL_O_DIS3`"]
pub struct STALL_O_DIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint 3 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS3_A::_0)
    }
    #[doc = "Endpoint 3 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Disable endpoint 4 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS4_A {
    #[doc = "0: Endpoint 4 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 4 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS4_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL_O_DIS4`"]
pub type STALL_O_DIS4_R = crate::R<bool, STALL_O_DIS4_A>;
impl STALL_O_DIS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS4_A {
        match self.bits {
            false => STALL_O_DIS4_A::_0,
            true => STALL_O_DIS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS4_A::_1
    }
}
#[doc = "Write proxy for field `STALL_O_DIS4`"]
pub struct STALL_O_DIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint 4 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS4_A::_0)
    }
    #[doc = "Endpoint 4 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Disable endpoint 5 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS5_A {
    #[doc = "0: Endpoint 5 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 5 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS5_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL_O_DIS5`"]
pub type STALL_O_DIS5_R = crate::R<bool, STALL_O_DIS5_A>;
impl STALL_O_DIS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS5_A {
        match self.bits {
            false => STALL_O_DIS5_A::_0,
            true => STALL_O_DIS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS5_A::_1
    }
}
#[doc = "Write proxy for field `STALL_O_DIS5`"]
pub struct STALL_O_DIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint 5 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS5_A::_0)
    }
    #[doc = "Endpoint 5 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS5_A::_1)
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
#[doc = "Disable endpoint 6 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS6_A {
    #[doc = "0: Endpoint 6 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 6 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS6_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL_O_DIS6`"]
pub type STALL_O_DIS6_R = crate::R<bool, STALL_O_DIS6_A>;
impl STALL_O_DIS6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS6_A {
        match self.bits {
            false => STALL_O_DIS6_A::_0,
            true => STALL_O_DIS6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS6_A::_1
    }
}
#[doc = "Write proxy for field `STALL_O_DIS6`"]
pub struct STALL_O_DIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint 6 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS6_A::_0)
    }
    #[doc = "Endpoint 6 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS6_A::_1)
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
#[doc = "Disable endpoint 7 OUT direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_O_DIS7_A {
    #[doc = "0: Endpoint 7 OUT direction stall is enabled."]
    _0 = 0,
    #[doc = "1: Endpoint 7 OUT direction stall is disabled."]
    _1 = 1,
}
impl From<STALL_O_DIS7_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_O_DIS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL_O_DIS7`"]
pub type STALL_O_DIS7_R = crate::R<bool, STALL_O_DIS7_A>;
impl STALL_O_DIS7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_O_DIS7_A {
        match self.bits {
            false => STALL_O_DIS7_A::_0,
            true => STALL_O_DIS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALL_O_DIS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALL_O_DIS7_A::_1
    }
}
#[doc = "Write proxy for field `STALL_O_DIS7`"]
pub struct STALL_O_DIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_O_DIS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_O_DIS7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint 7 OUT direction stall is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALL_O_DIS7_A::_0)
    }
    #[doc = "Endpoint 7 OUT direction stall is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALL_O_DIS7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable endpoint 0 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis0(&self) -> STALL_O_DIS0_R {
        STALL_O_DIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable endpoint 1 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis1(&self) -> STALL_O_DIS1_R {
        STALL_O_DIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable endpoint 2 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis2(&self) -> STALL_O_DIS2_R {
        STALL_O_DIS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable endpoint 3 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis3(&self) -> STALL_O_DIS3_R {
        STALL_O_DIS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Disable endpoint 4 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis4(&self) -> STALL_O_DIS4_R {
        STALL_O_DIS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable endpoint 5 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis5(&self) -> STALL_O_DIS5_R {
        STALL_O_DIS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable endpoint 6 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis6(&self) -> STALL_O_DIS6_R {
        STALL_O_DIS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Disable endpoint 7 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis7(&self) -> STALL_O_DIS7_R {
        STALL_O_DIS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable endpoint 0 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis0(&mut self) -> STALL_O_DIS0_W {
        STALL_O_DIS0_W { w: self }
    }
    #[doc = "Bit 1 - Disable endpoint 1 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis1(&mut self) -> STALL_O_DIS1_W {
        STALL_O_DIS1_W { w: self }
    }
    #[doc = "Bit 2 - Disable endpoint 2 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis2(&mut self) -> STALL_O_DIS2_W {
        STALL_O_DIS2_W { w: self }
    }
    #[doc = "Bit 3 - Disable endpoint 3 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis3(&mut self) -> STALL_O_DIS3_W {
        STALL_O_DIS3_W { w: self }
    }
    #[doc = "Bit 4 - Disable endpoint 4 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis4(&mut self) -> STALL_O_DIS4_W {
        STALL_O_DIS4_W { w: self }
    }
    #[doc = "Bit 5 - Disable endpoint 5 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis5(&mut self) -> STALL_O_DIS5_W {
        STALL_O_DIS5_W { w: self }
    }
    #[doc = "Bit 6 - Disable endpoint 6 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis6(&mut self) -> STALL_O_DIS6_W {
        STALL_O_DIS6_W { w: self }
    }
    #[doc = "Bit 7 - Disable endpoint 7 OUT direction."]
    #[inline(always)]
    pub fn stall_o_dis7(&mut self) -> STALL_O_DIS7_W {
        STALL_O_DIS7_W { w: self }
    }
}
