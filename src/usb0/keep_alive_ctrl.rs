#[doc = "Register `KEEP_ALIVE_CTRL` reader"]
pub struct R(crate::R<KEEP_ALIVE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEEP_ALIVE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEEP_ALIVE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEEP_ALIVE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEEP_ALIVE_CTRL` writer"]
pub struct W(crate::W<KEEP_ALIVE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEEP_ALIVE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<KEEP_ALIVE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEEP_ALIVE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEEP_ALIVE_EN` reader - Global enable for USB_KEEP_ALIVE mode"]
pub struct KEEP_ALIVE_EN_R(crate::FieldReader<bool, bool>);
impl KEEP_ALIVE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEEP_ALIVE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEEP_ALIVE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEEP_ALIVE_EN` writer - Global enable for USB_KEEP_ALIVE mode"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `OWN_OVERRD_EN` reader - When set to 1, during KEEP_ALIVE mode, if received token is not SETUP, the OWN bit of current BD will be forced to 0, so usb core will respond with NAK"]
pub struct OWN_OVERRD_EN_R(crate::FieldReader<bool, bool>);
impl OWN_OVERRD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OWN_OVERRD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OWN_OVERRD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OWN_OVERRD_EN` writer - When set to 1, during KEEP_ALIVE mode, if received token is not SETUP, the OWN bit of current BD will be forced to 0, so usb core will respond with NAK"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
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
#[doc = "Field `WAKE_REQ_EN` reader - During KEEP_ALIVE mode, a bus access by the USB controller to a memory location outside the USB SRAM will cause the bus access to stall until KEEP_ALIVE mode is exited"]
pub struct WAKE_REQ_EN_R(crate::FieldReader<bool, WAKE_REQ_EN_A>);
impl WAKE_REQ_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_REQ_EN_R(crate::FieldReader::new(bits))
    }
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
        **self == WAKE_REQ_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WAKE_REQ_EN_A::_1
    }
}
impl core::ops::Deref for WAKE_REQ_EN_R {
    type Target = crate::FieldReader<bool, WAKE_REQ_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKE_REQ_EN` writer - During KEEP_ALIVE mode, a bus access by the USB controller to a memory location outside the USB SRAM will cause the bus access to stall until KEEP_ALIVE mode is exited"]
pub struct WAKE_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_REQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKE_REQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `WAKE_INT_EN` reader - Wakeup Interrupt Enable."]
pub struct WAKE_INT_EN_R(crate::FieldReader<bool, bool>);
impl WAKE_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKE_INT_EN` writer - Wakeup Interrupt Enable."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WAKE_INT_STS` reader - Wakeup Interrupt Status."]
pub struct WAKE_INT_STS_R(crate::FieldReader<bool, bool>);
impl WAKE_INT_STS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_INT_STS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKE_INT_STS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keep Alive mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keep_alive_ctrl](index.html) module"]
pub struct KEEP_ALIVE_CTRL_SPEC;
impl crate::RegisterSpec for KEEP_ALIVE_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [keep_alive_ctrl::R](R) reader structure"]
impl crate::Readable for KEEP_ALIVE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keep_alive_ctrl::W](W) writer structure"]
impl crate::Writable for KEEP_ALIVE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEEP_ALIVE_CTRL to value 0x08"]
impl crate::Resettable for KEEP_ALIVE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
