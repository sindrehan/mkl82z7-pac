#[doc = "Reader of register MISCCTRL"]
pub type R = crate::R<u32, super::MISCCTRL>;
#[doc = "Writer for register MISCCTRL"]
pub type W = crate::W<u32, super::MISCCTRL>;
#[doc = "Register MISCCTRL `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::MISCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "DMA Channel Interrupts Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTSEL0_A {
    #[doc = "0: DMA0 channel 4 is not available in vector 16."]
    _0 = 0,
    #[doc = "1: DMA0 channel 4 is available in vector 16."]
    _1 = 1,
}
impl From<DMAINTSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINTSEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAINTSEL0`"]
pub type DMAINTSEL0_R = crate::R<bool, DMAINTSEL0_A>;
impl DMAINTSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAINTSEL0_A {
        match self.bits {
            false => DMAINTSEL0_A::_0,
            true => DMAINTSEL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAINTSEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAINTSEL0_A::_1
    }
}
#[doc = "Write proxy for field `DMAINTSEL0`"]
pub struct DMAINTSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINTSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAINTSEL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA0 channel 4 is not available in vector 16."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAINTSEL0_A::_0)
    }
    #[doc = "DMA0 channel 4 is available in vector 16."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAINTSEL0_A::_1)
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
#[doc = "DMA Channel Interrupts Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTSEL1_A {
    #[doc = "0: DMA0 channel 5 is not available in vector 17."]
    _0 = 0,
    #[doc = "1: DMA0 channel 5 is available in vector 17."]
    _1 = 1,
}
impl From<DMAINTSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINTSEL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAINTSEL1`"]
pub type DMAINTSEL1_R = crate::R<bool, DMAINTSEL1_A>;
impl DMAINTSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAINTSEL1_A {
        match self.bits {
            false => DMAINTSEL1_A::_0,
            true => DMAINTSEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAINTSEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAINTSEL1_A::_1
    }
}
#[doc = "Write proxy for field `DMAINTSEL1`"]
pub struct DMAINTSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINTSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAINTSEL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA0 channel 5 is not available in vector 17."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAINTSEL1_A::_0)
    }
    #[doc = "DMA0 channel 5 is available in vector 17."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAINTSEL1_A::_1)
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
#[doc = "DMA Channel Interrupts Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTSEL2_A {
    #[doc = "0: DMA0 channel 6 is not available in vector 18."]
    _0 = 0,
    #[doc = "1: DMA0 channel 6 is available in vector 18."]
    _1 = 1,
}
impl From<DMAINTSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINTSEL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAINTSEL2`"]
pub type DMAINTSEL2_R = crate::R<bool, DMAINTSEL2_A>;
impl DMAINTSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAINTSEL2_A {
        match self.bits {
            false => DMAINTSEL2_A::_0,
            true => DMAINTSEL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAINTSEL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAINTSEL2_A::_1
    }
}
#[doc = "Write proxy for field `DMAINTSEL2`"]
pub struct DMAINTSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINTSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAINTSEL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA0 channel 6 is not available in vector 18."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAINTSEL2_A::_0)
    }
    #[doc = "DMA0 channel 6 is available in vector 18."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAINTSEL2_A::_1)
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
#[doc = "DMA Channel Interrupts Select 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTSEL3_A {
    #[doc = "0: DMA0 channel 7 is not available in vector 19."]
    _0 = 0,
    #[doc = "1: DMA0 channel 7 is available in vector 19."]
    _1 = 1,
}
impl From<DMAINTSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINTSEL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAINTSEL3`"]
pub type DMAINTSEL3_R = crate::R<bool, DMAINTSEL3_A>;
impl DMAINTSEL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAINTSEL3_A {
        match self.bits {
            false => DMAINTSEL3_A::_0,
            true => DMAINTSEL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAINTSEL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAINTSEL3_A::_1
    }
}
#[doc = "Write proxy for field `DMAINTSEL3`"]
pub struct DMAINTSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINTSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAINTSEL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA0 channel 7 is not available in vector 19."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAINTSEL3_A::_0)
    }
    #[doc = "DMA0 channel 7 is available in vector 19."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAINTSEL3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "LTC Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTCEN_A {
    #[doc = "0: LTC is not available."]
    _0 = 0,
    #[doc = "1: LTC is available."]
    _1 = 1,
}
impl From<LTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LTCEN`"]
pub type LTCEN_R = crate::R<bool, LTCEN_A>;
impl LTCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTCEN_A {
        match self.bits {
            false => LTCEN_A::_0,
            true => LTCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LTCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LTCEN_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel Interrupts Select 0"]
    #[inline(always)]
    pub fn dmaintsel0(&self) -> DMAINTSEL0_R {
        DMAINTSEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Channel Interrupts Select 1"]
    #[inline(always)]
    pub fn dmaintsel1(&self) -> DMAINTSEL1_R {
        DMAINTSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Channel Interrupts Select 2"]
    #[inline(always)]
    pub fn dmaintsel2(&self) -> DMAINTSEL2_R {
        DMAINTSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Channel Interrupts Select 3"]
    #[inline(always)]
    pub fn dmaintsel3(&self) -> DMAINTSEL3_R {
        DMAINTSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LTC Status"]
    #[inline(always)]
    pub fn ltcen(&self) -> LTCEN_R {
        LTCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel Interrupts Select 0"]
    #[inline(always)]
    pub fn dmaintsel0(&mut self) -> DMAINTSEL0_W {
        DMAINTSEL0_W { w: self }
    }
    #[doc = "Bit 1 - DMA Channel Interrupts Select 1"]
    #[inline(always)]
    pub fn dmaintsel1(&mut self) -> DMAINTSEL1_W {
        DMAINTSEL1_W { w: self }
    }
    #[doc = "Bit 2 - DMA Channel Interrupts Select 2"]
    #[inline(always)]
    pub fn dmaintsel2(&mut self) -> DMAINTSEL2_W {
        DMAINTSEL2_W { w: self }
    }
    #[doc = "Bit 3 - DMA Channel Interrupts Select 3"]
    #[inline(always)]
    pub fn dmaintsel3(&mut self) -> DMAINTSEL3_W {
        DMAINTSEL3_W { w: self }
    }
}
