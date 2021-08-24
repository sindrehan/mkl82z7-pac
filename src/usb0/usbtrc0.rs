#[doc = "Register `USBTRC0` reader"]
pub struct R(crate::R<USBTRC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBTRC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBTRC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBTRC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBTRC0` writer"]
pub struct W(crate::W<USBTRC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBTRC0_SPEC>;
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
impl From<crate::W<USBTRC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBTRC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Asynchronous Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RESUME_INT_A {
    #[doc = "0: No interrupt was generated."]
    _0 = 0,
    #[doc = "1: Interrupt was generated because of the USB asynchronous interrupt."]
    _1 = 1,
}
impl From<USB_RESUME_INT_A> for bool {
    #[inline(always)]
    fn from(variant: USB_RESUME_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_RESUME_INT` reader - USB Asynchronous Interrupt"]
pub struct USB_RESUME_INT_R(crate::FieldReader<bool, USB_RESUME_INT_A>);
impl USB_RESUME_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_RESUME_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_RESUME_INT_A {
        match self.bits {
            false => USB_RESUME_INT_A::_0,
            true => USB_RESUME_INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USB_RESUME_INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USB_RESUME_INT_A::_1
    }
}
impl core::ops::Deref for USB_RESUME_INT_R {
    type Target = crate::FieldReader<bool, USB_RESUME_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Synchronous USB Interrupt Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_DET_A {
    #[doc = "0: Synchronous interrupt has not been detected."]
    _0 = 0,
    #[doc = "1: Synchronous interrupt has been detected."]
    _1 = 1,
}
impl From<SYNC_DET_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_DET` reader - Synchronous USB Interrupt Detect"]
pub struct SYNC_DET_R(crate::FieldReader<bool, SYNC_DET_A>);
impl SYNC_DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_DET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_DET_A {
        match self.bits {
            false => SYNC_DET_A::_0,
            true => SYNC_DET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYNC_DET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYNC_DET_A::_1
    }
}
impl core::ops::Deref for SYNC_DET_R {
    type Target = crate::FieldReader<bool, SYNC_DET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_CLK_RECOVERY_INT` reader - Combined USB Clock Recovery interrupt status"]
pub struct USB_CLK_RECOVERY_INT_R(crate::FieldReader<bool, bool>);
impl USB_CLK_RECOVERY_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_CLK_RECOVERY_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_CLK_RECOVERY_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "VREGIN Rising Edge Interrupt Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREDG_DET_A {
    #[doc = "0: VREGIN rising edge interrupt has not been detected."]
    _0 = 0,
    #[doc = "1: VREGIN rising edge interrupt has been detected."]
    _1 = 1,
}
impl From<VREDG_DET_A> for bool {
    #[inline(always)]
    fn from(variant: VREDG_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREDG_DET` reader - VREGIN Rising Edge Interrupt Detect"]
pub struct VREDG_DET_R(crate::FieldReader<bool, VREDG_DET_A>);
impl VREDG_DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREDG_DET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREDG_DET_A {
        match self.bits {
            false => VREDG_DET_A::_0,
            true => VREDG_DET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VREDG_DET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VREDG_DET_A::_1
    }
}
impl core::ops::Deref for VREDG_DET_R {
    type Target = crate::FieldReader<bool, VREDG_DET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "VREGIN Falling Edge Interrupt Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VFEDG_DET_A {
    #[doc = "0: VREGIN falling edge interrupt has not been detected."]
    _0 = 0,
    #[doc = "1: VREGIN falling edge interrupt has been detected."]
    _1 = 1,
}
impl From<VFEDG_DET_A> for bool {
    #[inline(always)]
    fn from(variant: VFEDG_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VFEDG_DET` reader - VREGIN Falling Edge Interrupt Detect"]
pub struct VFEDG_DET_R(crate::FieldReader<bool, VFEDG_DET_A>);
impl VFEDG_DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        VFEDG_DET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VFEDG_DET_A {
        match self.bits {
            false => VFEDG_DET_A::_0,
            true => VFEDG_DET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VFEDG_DET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VFEDG_DET_A::_1
    }
}
impl core::ops::Deref for VFEDG_DET_R {
    type Target = crate::FieldReader<bool, VFEDG_DET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Asynchronous Resume Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESMEN_A {
    #[doc = "0: USB asynchronous wakeup from suspend mode disabled."]
    _0 = 0,
    #[doc = "1: USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interrupt should only be enabled when the Transceiver is suspended."]
    _1 = 1,
}
impl From<USBRESMEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBRESMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRESMEN` reader - Asynchronous Resume Interrupt Enable"]
pub struct USBRESMEN_R(crate::FieldReader<bool, USBRESMEN_A>);
impl USBRESMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRESMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRESMEN_A {
        match self.bits {
            false => USBRESMEN_A::_0,
            true => USBRESMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBRESMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBRESMEN_A::_1
    }
}
impl core::ops::Deref for USBRESMEN_R {
    type Target = crate::FieldReader<bool, USBRESMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRESMEN` writer - Asynchronous Resume Interrupt Enable"]
pub struct USBRESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRESMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRESMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB asynchronous wakeup from suspend mode disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRESMEN_A::_0)
    }
    #[doc = "USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interrupt should only be enabled when the Transceiver is suspended."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRESMEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "USB Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESET_AW {
    #[doc = "0: Normal USB module operation."]
    _0 = 0,
    #[doc = "1: Returns the USB module to its reset state."]
    _1 = 1,
}
impl From<USBRESET_AW> for bool {
    #[inline(always)]
    fn from(variant: USBRESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRESET` writer - USB Reset"]
pub struct USBRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRESET_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal USB module operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRESET_AW::_0)
    }
    #[doc = "Returns the USB module to its reset state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRESET_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB Asynchronous Interrupt"]
    #[inline(always)]
    pub fn usb_resume_int(&self) -> USB_RESUME_INT_R {
        USB_RESUME_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronous USB Interrupt Detect"]
    #[inline(always)]
    pub fn sync_det(&self) -> SYNC_DET_R {
        SYNC_DET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Combined USB Clock Recovery interrupt status"]
    #[inline(always)]
    pub fn usb_clk_recovery_int(&self) -> USB_CLK_RECOVERY_INT_R {
        USB_CLK_RECOVERY_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Detect"]
    #[inline(always)]
    pub fn vredg_det(&self) -> VREDG_DET_R {
        VREDG_DET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Detect"]
    #[inline(always)]
    pub fn vfedg_det(&self) -> VFEDG_DET_R {
        VFEDG_DET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn usbresmen(&self) -> USBRESMEN_R {
        USBRESMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn usbresmen(&mut self) -> USBRESMEN_W {
        USBRESMEN_W { w: self }
    }
    #[doc = "Bit 7 - USB Reset"]
    #[inline(always)]
    pub fn usbreset(&mut self) -> USBRESET_W {
        USBRESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transceiver Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbtrc0](index.html) module"]
pub struct USBTRC0_SPEC;
impl crate::RegisterSpec for USBTRC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbtrc0::R](R) reader structure"]
impl crate::Readable for USBTRC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbtrc0::W](W) writer structure"]
impl crate::Writable for USBTRC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBTRC0 to value 0"]
impl crate::Resettable for USBTRC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
