#[doc = "Reader of register HVDSC1"]
pub type R = crate::R<u8, super::HVDSC1>;
#[doc = "Writer for register HVDSC1"]
pub type W = crate::W<u8, super::HVDSC1>;
#[doc = "Register HVDSC1 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::HVDSC1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "High-Voltage Detect Voltage Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVDV_A {
    #[doc = "0: Low trip point selected (V HVD = V HVDL )"]
    _0 = 0,
    #[doc = "1: High trip point selected (V HVD = V HVDH )"]
    _1 = 1,
}
impl From<HVDV_A> for bool {
    #[inline(always)]
    fn from(variant: HVDV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HVDV`"]
pub type HVDV_R = crate::R<bool, HVDV_A>;
impl HVDV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVDV_A {
        match self.bits {
            false => HVDV_A::_0,
            true => HVDV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HVDV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HVDV_A::_1
    }
}
#[doc = "Write proxy for field `HVDV`"]
pub struct HVDV_W<'a> {
    w: &'a mut W,
}
impl<'a> HVDV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HVDV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low trip point selected (V HVD = V HVDL )"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HVDV_A::_0)
    }
    #[doc = "High trip point selected (V HVD = V HVDH )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HVDV_A::_1)
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
#[doc = "High-Voltage Detect Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVDRE_A {
    #[doc = "0: HVDF does not generate hardware resets"]
    _0 = 0,
    #[doc = "1: Force an MCU reset when HVDF = 1"]
    _1 = 1,
}
impl From<HVDRE_A> for bool {
    #[inline(always)]
    fn from(variant: HVDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HVDRE`"]
pub type HVDRE_R = crate::R<bool, HVDRE_A>;
impl HVDRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVDRE_A {
        match self.bits {
            false => HVDRE_A::_0,
            true => HVDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HVDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HVDRE_A::_1
    }
}
#[doc = "Write proxy for field `HVDRE`"]
pub struct HVDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HVDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HVDRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HVDF does not generate hardware resets"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HVDRE_A::_0)
    }
    #[doc = "Force an MCU reset when HVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HVDRE_A::_1)
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
#[doc = "High-Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVDIE_A {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0 = 0,
    #[doc = "1: Request a hardware interrupt when HVDF = 1"]
    _1 = 1,
}
impl From<HVDIE_A> for bool {
    #[inline(always)]
    fn from(variant: HVDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HVDIE`"]
pub type HVDIE_R = crate::R<bool, HVDIE_A>;
impl HVDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVDIE_A {
        match self.bits {
            false => HVDIE_A::_0,
            true => HVDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HVDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HVDIE_A::_1
    }
}
#[doc = "Write proxy for field `HVDIE`"]
pub struct HVDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HVDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HVDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HVDIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when HVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HVDIE_A::_1)
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
#[doc = "Write proxy for field `HVDACK`"]
pub struct HVDACK_W<'a> {
    w: &'a mut W,
}
impl<'a> HVDACK_W<'a> {
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
#[doc = "High-Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVDF_A {
    #[doc = "0: High-voltage event not detected"]
    _0 = 0,
    #[doc = "1: High-voltage event detected"]
    _1 = 1,
}
impl From<HVDF_A> for bool {
    #[inline(always)]
    fn from(variant: HVDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HVDF`"]
pub type HVDF_R = crate::R<bool, HVDF_A>;
impl HVDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVDF_A {
        match self.bits {
            false => HVDF_A::_0,
            true => HVDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HVDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HVDF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - High-Voltage Detect Voltage Select"]
    #[inline(always)]
    pub fn hvdv(&self) -> HVDV_R {
        HVDV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - High-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn hvdre(&self) -> HVDRE_R {
        HVDRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - High-Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn hvdie(&self) -> HVDIE_R {
        HVDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - High-Voltage Detect Flag"]
    #[inline(always)]
    pub fn hvdf(&self) -> HVDF_R {
        HVDF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High-Voltage Detect Voltage Select"]
    #[inline(always)]
    pub fn hvdv(&mut self) -> HVDV_W {
        HVDV_W { w: self }
    }
    #[doc = "Bit 4 - High-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn hvdre(&mut self) -> HVDRE_W {
        HVDRE_W { w: self }
    }
    #[doc = "Bit 5 - High-Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn hvdie(&mut self) -> HVDIE_W {
        HVDIE_W { w: self }
    }
    #[doc = "Bit 6 - High-Voltage Detect Acknowledge"]
    #[inline(always)]
    pub fn hvdack(&mut self) -> HVDACK_W {
        HVDACK_W { w: self }
    }
}
