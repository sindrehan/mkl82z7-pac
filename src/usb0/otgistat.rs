#[doc = "Register `OTGISTAT` reader"]
pub struct R(crate::R<OTGISTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGISTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGISTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGISTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGISTAT` writer"]
pub struct W(crate::W<OTGISTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGISTAT_SPEC>;
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
impl From<crate::W<OTGISTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGISTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE_STATE_CHG` reader - This interrupt is set when the USB line state (CTL\\[SE0\\]
and CTL\\[JSTATE\\]
bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
pub struct LINE_STATE_CHG_R(crate::FieldReader<bool, bool>);
impl LINE_STATE_CHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINE_STATE_CHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_STATE_CHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_STATE_CHG` writer - This interrupt is set when the USB line state (CTL\\[SE0\\]
and CTL\\[JSTATE\\]
bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ONEMSEC` reader - This bit is set when the 1 millisecond timer expires"]
pub struct ONEMSEC_R(crate::FieldReader<bool, bool>);
impl ONEMSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONEMSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONEMSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONEMSEC` writer - This bit is set when the 1 millisecond timer expires"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgistat](index.html) module"]
pub struct OTGISTAT_SPEC;
impl crate::RegisterSpec for OTGISTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [otgistat::R](R) reader structure"]
impl crate::Readable for OTGISTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgistat::W](W) writer structure"]
impl crate::Writable for OTGISTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTGISTAT to value 0"]
impl crate::Resettable for OTGISTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
