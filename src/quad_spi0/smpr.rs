#[doc = "Reader of register SMPR"]
pub type R = crate::R<u32, super::SMPR>;
#[doc = "Writer for register SMPR"]
pub type W = crate::W<u32, super::SMPR>;
#[doc = "Register SMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Half Speed serial flash clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSENA_A {
    #[doc = "0: Disable divide by 2 of serial flash clock for half speed commands"]
    _0 = 0,
    #[doc = "1: Enable divide by 2 of serial flash clock for half speed commands"]
    _1 = 1,
}
impl From<HSENA_A> for bool {
    #[inline(always)]
    fn from(variant: HSENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSENA`"]
pub type HSENA_R = crate::R<bool, HSENA_A>;
impl HSENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSENA_A {
        match self.bits {
            false => HSENA_A::_0,
            true => HSENA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSENA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSENA_A::_1
    }
}
#[doc = "Write proxy for field `HSENA`"]
pub struct HSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable divide by 2 of serial flash clock for half speed commands"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSENA_A::_0)
    }
    #[doc = "Enable divide by 2 of serial flash clock for half speed commands"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSENA_A::_1)
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
#[doc = "Half Speed Phase selection for SDR instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPHS_A {
    #[doc = "0: Select sampling at non-inverted clock"]
    _0 = 0,
    #[doc = "1: Select sampling at inverted clock"]
    _1 = 1,
}
impl From<HSPHS_A> for bool {
    #[inline(always)]
    fn from(variant: HSPHS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSPHS`"]
pub type HSPHS_R = crate::R<bool, HSPHS_A>;
impl HSPHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSPHS_A {
        match self.bits {
            false => HSPHS_A::_0,
            true => HSPHS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSPHS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSPHS_A::_1
    }
}
#[doc = "Write proxy for field `HSPHS`"]
pub struct HSPHS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSPHS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select sampling at non-inverted clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSPHS_A::_0)
    }
    #[doc = "Select sampling at inverted clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSPHS_A::_1)
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
#[doc = "Half Speed Delay selection for SDR instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSDLY_A {
    #[doc = "0: One clock cycle delay"]
    _0 = 0,
    #[doc = "1: Two clock cycle delay"]
    _1 = 1,
}
impl From<HSDLY_A> for bool {
    #[inline(always)]
    fn from(variant: HSDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSDLY`"]
pub type HSDLY_R = crate::R<bool, HSDLY_A>;
impl HSDLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSDLY_A {
        match self.bits {
            false => HSDLY_A::_0,
            true => HSDLY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSDLY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSDLY_A::_1
    }
}
#[doc = "Write proxy for field `HSDLY`"]
pub struct HSDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSDLY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One clock cycle delay"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSDLY_A::_0)
    }
    #[doc = "Two clock cycle delay"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSDLY_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Full Speed Phase selection for SDR instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPHS_A {
    #[doc = "0: Select sampling at non-inverted clock"]
    _0 = 0,
    #[doc = "1: Select sampling at inverted clock. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    _1 = 1,
}
impl From<FSPHS_A> for bool {
    #[inline(always)]
    fn from(variant: FSPHS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSPHS`"]
pub type FSPHS_R = crate::R<bool, FSPHS_A>;
impl FSPHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSPHS_A {
        match self.bits {
            false => FSPHS_A::_0,
            true => FSPHS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSPHS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSPHS_A::_1
    }
}
#[doc = "Write proxy for field `FSPHS`"]
pub struct FSPHS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSPHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSPHS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select sampling at non-inverted clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSPHS_A::_0)
    }
    #[doc = "Select sampling at inverted clock. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSPHS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSDLY_A {
    #[doc = "0: One clock cycle delay"]
    _0 = 0,
    #[doc = "1: Two clock cycles delay. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    _1 = 1,
}
impl From<FSDLY_A> for bool {
    #[inline(always)]
    fn from(variant: FSDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSDLY`"]
pub type FSDLY_R = crate::R<bool, FSDLY_A>;
impl FSDLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSDLY_A {
        match self.bits {
            false => FSDLY_A::_0,
            true => FSDLY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSDLY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSDLY_A::_1
    }
}
#[doc = "Write proxy for field `FSDLY`"]
pub struct FSDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSDLY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One clock cycle delay"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSDLY_A::_0)
    }
    #[doc = "Two clock cycles delay. This bit is also used in DQS mode and ignored when using non-DQS DDR instructions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSDLY_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DDRSMP`"]
pub type DDRSMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDRSMP`"]
pub struct DDRSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRSMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Half Speed serial flash clock Enable"]
    #[inline(always)]
    pub fn hsena(&self) -> HSENA_R {
        HSENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Half Speed Phase selection for SDR instructions."]
    #[inline(always)]
    pub fn hsphs(&self) -> HSPHS_R {
        HSPHS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Half Speed Delay selection for SDR instructions."]
    #[inline(always)]
    pub fn hsdly(&self) -> HSDLY_R {
        HSDLY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Full Speed Phase selection for SDR instructions."]
    #[inline(always)]
    pub fn fsphs(&self) -> FSPHS_R {
        FSPHS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands."]
    #[inline(always)]
    pub fn fsdly(&self) -> FSDLY_R {
        FSDLY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - DDR Sampling point"]
    #[inline(always)]
    pub fn ddrsmp(&self) -> DDRSMP_R {
        DDRSMP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Half Speed serial flash clock Enable"]
    #[inline(always)]
    pub fn hsena(&mut self) -> HSENA_W {
        HSENA_W { w: self }
    }
    #[doc = "Bit 1 - Half Speed Phase selection for SDR instructions."]
    #[inline(always)]
    pub fn hsphs(&mut self) -> HSPHS_W {
        HSPHS_W { w: self }
    }
    #[doc = "Bit 2 - Half Speed Delay selection for SDR instructions."]
    #[inline(always)]
    pub fn hsdly(&mut self) -> HSDLY_W {
        HSDLY_W { w: self }
    }
    #[doc = "Bit 5 - Full Speed Phase selection for SDR instructions."]
    #[inline(always)]
    pub fn fsphs(&mut self) -> FSPHS_W {
        FSPHS_W { w: self }
    }
    #[doc = "Bit 6 - Full Speed Delay selection for SDR instructions. Select the delay with respect to the reference edge for the sample point valid for full speed commands."]
    #[inline(always)]
    pub fn fsdly(&mut self) -> FSDLY_W {
        FSDLY_W { w: self }
    }
    #[doc = "Bits 16:18 - DDR Sampling point"]
    #[inline(always)]
    pub fn ddrsmp(&mut self) -> DDRSMP_W {
        DDRSMP_W { w: self }
    }
}
