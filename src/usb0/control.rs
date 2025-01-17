#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPULLUPNONOTG_A {
    #[doc = "0: DP Pullup in non-OTG device mode is not enabled."]
    _0 = 0,
    #[doc = "1: DP Pullup in non-OTG device mode is enabled."]
    _1 = 1,
}
impl From<DPPULLUPNONOTG_A> for bool {
    #[inline(always)]
    fn from(variant: DPPULLUPNONOTG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPULLUPNONOTG` reader - Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode."]
pub struct DPPULLUPNONOTG_R(crate::FieldReader<bool, DPPULLUPNONOTG_A>);
impl DPPULLUPNONOTG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPPULLUPNONOTG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPULLUPNONOTG_A {
        match self.bits {
            false => DPPULLUPNONOTG_A::_0,
            true => DPPULLUPNONOTG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DPPULLUPNONOTG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DPPULLUPNONOTG_A::_1
    }
}
impl core::ops::Deref for DPPULLUPNONOTG_R {
    type Target = crate::FieldReader<bool, DPPULLUPNONOTG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPPULLUPNONOTG` writer - Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode."]
pub struct DPPULLUPNONOTG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPULLUPNONOTG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPPULLUPNONOTG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DP Pullup in non-OTG device mode is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPPULLUPNONOTG_A::_0)
    }
    #[doc = "DP Pullup in non-OTG device mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPPULLUPNONOTG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode."]
    #[inline(always)]
    pub fn dppullupnonotg(&self) -> DPPULLUPNONOTG_R {
        DPPULLUPNONOTG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Provides control of the DP Pullup in USBOTG, if USB is configured in non-OTG device mode."]
    #[inline(always)]
    pub fn dppullupnonotg(&mut self) -> DPPULLUPNONOTG_W {
        DPPULLUPNONOTG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB OTG Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
