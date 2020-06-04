#[doc = "Reader of register OTGISTAT"]
pub type R = crate::R<u8, super::OTGISTAT>;
#[doc = "Writer for register OTGISTAT"]
pub type W = crate::W<u8, super::OTGISTAT>;
#[doc = "Register OTGISTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::OTGISTAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LINE_STATE_CHG`"]
pub type LINE_STATE_CHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LINE_STATE_CHG`"]
pub struct LINE_STATE_CHG_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_STATE_CHG_W<'a> {
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
#[doc = "Reader of field `ONEMSEC`"]
pub type ONEMSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONEMSEC`"]
pub struct ONEMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEMSEC_W<'a> {
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
    #[doc = "Bit 5 - This interrupt is set when the USB line state (CTL\\[SE0\\]
and CTL\\[JSTATE\\]
bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
    #[inline(always)]
    pub fn line_state_chg(&self) -> LINE_STATE_CHG_R {
        LINE_STATE_CHG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is set when the 1 millisecond timer expires"]
    #[inline(always)]
    pub fn onemsec(&self) -> ONEMSEC_R {
        ONEMSEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - This interrupt is set when the USB line state (CTL\\[SE0\\]
and CTL\\[JSTATE\\]
bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
    #[inline(always)]
    pub fn line_state_chg(&mut self) -> LINE_STATE_CHG_W {
        LINE_STATE_CHG_W { w: self }
    }
    #[doc = "Bit 6 - This bit is set when the 1 millisecond timer expires"]
    #[inline(always)]
    pub fn onemsec(&mut self) -> ONEMSEC_W {
        ONEMSEC_W { w: self }
    }
}
