#[doc = "Register `KEEP_ALIVE_WKCTRL` reader"]
pub struct R(crate::R<KEEP_ALIVE_WKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEEP_ALIVE_WKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEEP_ALIVE_WKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEEP_ALIVE_WKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEEP_ALIVE_WKCTRL` writer"]
pub struct W(crate::W<KEEP_ALIVE_WKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEEP_ALIVE_WKCTRL_SPEC>;
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
impl From<crate::W<KEEP_ALIVE_WKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEEP_ALIVE_WKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software configure it to which token can wakeup usb during KEEP_ALIVE mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAKE_ON_THIS_A {
    #[doc = "1: Wake up on receiving OUT/SETUP token packet."]
    _0001 = 1,
    #[doc = "13: Wake up on receiving SETUP token packet.All other values are reserved."]
    _1101 = 13,
}
impl From<WAKE_ON_THIS_A> for u8 {
    #[inline(always)]
    fn from(variant: WAKE_ON_THIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAKE_ON_THIS` reader - Software configure it to which token can wakeup usb during KEEP_ALIVE mode"]
pub struct WAKE_ON_THIS_R(crate::FieldReader<u8, WAKE_ON_THIS_A>);
impl WAKE_ON_THIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAKE_ON_THIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAKE_ON_THIS_A> {
        match self.bits {
            1 => Some(WAKE_ON_THIS_A::_0001),
            13 => Some(WAKE_ON_THIS_A::_1101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == WAKE_ON_THIS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == WAKE_ON_THIS_A::_1101
    }
}
impl core::ops::Deref for WAKE_ON_THIS_R {
    type Target = crate::FieldReader<u8, WAKE_ON_THIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKE_ON_THIS` writer - Software configure it to which token can wakeup usb during KEEP_ALIVE mode"]
pub struct WAKE_ON_THIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_ON_THIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKE_ON_THIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Wake up on receiving OUT/SETUP token packet."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(WAKE_ON_THIS_A::_0001)
    }
    #[doc = "Wake up on receiving SETUP token packet.All other values are reserved."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(WAKE_ON_THIS_A::_1101)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Field `WAKE_ENDPT` reader - Indicates which endpoint causes the wakeup interrupt. Reset to 0, software read only."]
pub struct WAKE_ENDPT_R(crate::FieldReader<u8, u8>);
impl WAKE_ENDPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAKE_ENDPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKE_ENDPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Software configure it to which token can wakeup usb during KEEP_ALIVE mode"]
    #[inline(always)]
    pub fn wake_on_this(&self) -> WAKE_ON_THIS_R {
        WAKE_ON_THIS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates which endpoint causes the wakeup interrupt. Reset to 0, software read only."]
    #[inline(always)]
    pub fn wake_endpt(&self) -> WAKE_ENDPT_R {
        WAKE_ENDPT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Software configure it to which token can wakeup usb during KEEP_ALIVE mode"]
    #[inline(always)]
    pub fn wake_on_this(&mut self) -> WAKE_ON_THIS_W {
        WAKE_ON_THIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keep Alive mode wakeup control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keep_alive_wkctrl](index.html) module"]
pub struct KEEP_ALIVE_WKCTRL_SPEC;
impl crate::RegisterSpec for KEEP_ALIVE_WKCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [keep_alive_wkctrl::R](R) reader structure"]
impl crate::Readable for KEEP_ALIVE_WKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keep_alive_wkctrl::W](W) writer structure"]
impl crate::Writable for KEEP_ALIVE_WKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEEP_ALIVE_WKCTRL to value 0x01"]
impl crate::Resettable for KEEP_ALIVE_WKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
