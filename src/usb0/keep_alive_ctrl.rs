#[doc = "Reader of register KEEP_ALIVE_CTRL"]
pub type R = crate::R<u8, super::KEEP_ALIVE_CTRL>;
#[doc = "Writer for register KEEP_ALIVE_CTRL"]
pub type W = crate::W<u8, super::KEEP_ALIVE_CTRL>;
#[doc = "Register KEEP_ALIVE_CTRL `reset()`'s with value 0x08"]
impl crate::ResetValue for super::KEEP_ALIVE_CTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `KEEP_ALIVE_EN`"]
pub type KEEP_ALIVE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEEP_ALIVE_EN`"]
pub struct KEEP_ALIVE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEP_ALIVE_EN_W<'a> {
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
#[doc = "Reader of field `OWN_OVERRD_EN`"]
pub type OWN_OVERRD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OWN_OVERRD_EN`"]
pub struct OWN_OVERRD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OWN_OVERRD_EN_W<'a> {
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
#[doc = "During KEEP_ALIVE mode, a bus access by the USB controller to a memory location outside the USB SRAM will cause the bus access to stall until KEEP_ALIVE mode is exited\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_REQ_EN_A {
    #[doc = "0: USB bus wakeup request is disabled."]
    _0 = 0,
    #[doc = "1: USB bus wakeup request is enabled."]
    _1 = 1,
}
impl From<WAKE_REQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_REQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKE_REQ_EN`"]
pub type WAKE_REQ_EN_R = crate::R<bool, WAKE_REQ_EN_A>;
impl WAKE_REQ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_REQ_EN_A {
        match self.bits {
            false => WAKE_REQ_EN_A::_0,
            true => WAKE_REQ_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKE_REQ_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKE_REQ_EN_A::_1
    }
}
#[doc = "Write proxy for field `WAKE_REQ_EN`"]
pub struct WAKE_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_REQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKE_REQ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB bus wakeup request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_REQ_EN_A::_0)
    }
    #[doc = "USB bus wakeup request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKE_REQ_EN_A::_1)
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
#[doc = "Reader of field `WAKE_INT_EN`"]
pub type WAKE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKE_INT_EN`"]
pub struct WAKE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_INT_EN_W<'a> {
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
#[doc = "Reader of field `WAKE_INT_STS`"]
pub type WAKE_INT_STS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Global enable for USB_KEEP_ALIVE mode"]
    #[inline(always)]
    pub fn keep_alive_en(&self) -> KEEP_ALIVE_EN_R {
        KEEP_ALIVE_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When set to 1, during KEEP_ALIVE mode, if received token is not SETUP, the OWN bit of current BD will be forced to 0, so usb core will respond with NAK"]
    #[inline(always)]
    pub fn own_overrd_en(&self) -> OWN_OVERRD_EN_R {
        OWN_OVERRD_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - During KEEP_ALIVE mode, a bus access by the USB controller to a memory location outside the USB SRAM will cause the bus access to stall until KEEP_ALIVE mode is exited"]
    #[inline(always)]
    pub fn wake_req_en(&self) -> WAKE_REQ_EN_R {
        WAKE_REQ_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Interrupt Enable."]
    #[inline(always)]
    pub fn wake_int_en(&self) -> WAKE_INT_EN_R {
        WAKE_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Interrupt Status."]
    #[inline(always)]
    pub fn wake_int_sts(&self) -> WAKE_INT_STS_R {
        WAKE_INT_STS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global enable for USB_KEEP_ALIVE mode"]
    #[inline(always)]
    pub fn keep_alive_en(&mut self) -> KEEP_ALIVE_EN_W {
        KEEP_ALIVE_EN_W { w: self }
    }
    #[doc = "Bit 1 - When set to 1, during KEEP_ALIVE mode, if received token is not SETUP, the OWN bit of current BD will be forced to 0, so usb core will respond with NAK"]
    #[inline(always)]
    pub fn own_overrd_en(&mut self) -> OWN_OVERRD_EN_W {
        OWN_OVERRD_EN_W { w: self }
    }
    #[doc = "Bit 3 - During KEEP_ALIVE mode, a bus access by the USB controller to a memory location outside the USB SRAM will cause the bus access to stall until KEEP_ALIVE mode is exited"]
    #[inline(always)]
    pub fn wake_req_en(&mut self) -> WAKE_REQ_EN_W {
        WAKE_REQ_EN_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Interrupt Enable."]
    #[inline(always)]
    pub fn wake_int_en(&mut self) -> WAKE_INT_EN_W {
        WAKE_INT_EN_W { w: self }
    }
}
