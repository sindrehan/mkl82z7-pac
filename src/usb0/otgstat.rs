#[doc = "Reader of register OTGSTAT"]
pub type R = crate::R<u8, super::OTGSTAT>;
#[doc = "Writer for register OTGSTAT"]
pub type W = crate::W<u8, super::OTGSTAT>;
#[doc = "Register OTGSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OTGSTAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATESTABLE_A {
    #[doc = "0: The LINE_STAT_CHG bit is not yet stable."]
    _0 = 0,
    #[doc = "1: The LINE_STAT_CHG bit has been debounced and is stable."]
    _1 = 1,
}
impl From<LINESTATESTABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATESTABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINESTATESTABLE`"]
pub type LINESTATESTABLE_R = crate::R<bool, LINESTATESTABLE_A>;
impl LINESTATESTABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATESTABLE_A {
        match self.bits {
            false => LINESTATESTABLE_A::_0,
            true => LINESTATESTABLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINESTATESTABLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINESTATESTABLE_A::_1
    }
}
#[doc = "Write proxy for field `LINESTATESTABLE`"]
pub struct LINESTATESTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINESTATESTABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINESTATESTABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATESTABLE_A::_0)
    }
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATESTABLE_A::_1)
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
#[doc = "Reader of field `ONEMSECEN`"]
pub type ONEMSECEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONEMSECEN`"]
pub struct ONEMSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEMSECEN_W<'a> {
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
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
    #[inline(always)]
    pub fn linestatestable(&self) -> LINESTATESTABLE_R {
        LINESTATESTABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline(always)]
    pub fn onemsecen(&self) -> ONEMSECEN_R {
        ONEMSECEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
    #[inline(always)]
    pub fn linestatestable(&mut self) -> LINESTATESTABLE_W {
        LINESTATESTABLE_W { w: self }
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline(always)]
    pub fn onemsecen(&mut self) -> ONEMSECEN_W {
        ONEMSECEN_W { w: self }
    }
}
