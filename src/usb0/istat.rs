#[doc = "Register `ISTAT` reader"]
pub struct R(crate::R<ISTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISTAT` writer"]
pub struct W(crate::W<ISTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTAT_SPEC>;
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
impl From<crate::W<ISTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBRST` reader - This bit is set when the USB Module has decoded a valid USB reset"]
pub struct USBRST_R(crate::FieldReader<bool, bool>);
impl USBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRST` writer - This bit is set when the USB Module has decoded a valid USB reset"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Field `ERROR` reader - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
pub struct ERROR_R(crate::FieldReader<bool, bool>);
impl ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
#[doc = "Field `SOFTOK` reader - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
pub struct SOFTOK_R(crate::FieldReader<bool, bool>);
impl SOFTOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTOK` writer - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
pub struct SOFTOK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTOK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TOKDNE` reader - This bit is set when the current token being processed has completed"]
pub struct TOKDNE_R(crate::FieldReader<bool, bool>);
impl TOKDNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOKDNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOKDNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOKDNE` writer - This bit is set when the current token being processed has completed"]
pub struct TOKDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOKDNE_W<'a> {
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
#[doc = "Field `SLEEP` reader - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
pub struct SLEEP_R(crate::FieldReader<bool, bool>);
impl SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP` writer - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
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
#[doc = "Field `RESUME` reader - This bit is set when a K-state is observed on the DP/DM signals for 2"]
pub struct RESUME_R(crate::FieldReader<bool, bool>);
impl RESUME_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - This bit is set when a K-state is observed on the DP/DM signals for 2"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Attach Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATTACH_A {
    #[doc = "0: No Attach is detected since the last time the ATTACH bit was cleared."]
    _0 = 0,
    #[doc = "1: A peripheral is now present and must be configured (a stable non-SE0 state is detected for more than 2.5 us)."]
    _1 = 1,
}
impl From<ATTACH_A> for bool {
    #[inline(always)]
    fn from(variant: ATTACH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATTACH` reader - Attach Interrupt"]
pub struct ATTACH_R(crate::FieldReader<bool, ATTACH_A>);
impl ATTACH_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATTACH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATTACH_A {
        match self.bits {
            false => ATTACH_A::_0,
            true => ATTACH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ATTACH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ATTACH_A::_1
    }
}
impl core::ops::Deref for ATTACH_R {
    type Target = crate::FieldReader<bool, ATTACH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTACH` writer - Attach Interrupt"]
pub struct ATTACH_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTACH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATTACH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Attach is detected since the last time the ATTACH bit was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATTACH_A::_0)
    }
    #[doc = "A peripheral is now present and must be configured (a stable non-SE0 state is detected for more than 2.5 us)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATTACH_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `STALL` reader - Stall Interrupt"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` writer - Stall Interrupt"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
    #[doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
    #[inline(always)]
    pub fn softok(&self) -> SOFTOK_R {
        SOFTOK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the current token being processed has completed"]
    #[inline(always)]
    pub fn tokdne(&self) -> TOKDNE_R {
        TOKDNE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Attach Interrupt"]
    #[inline(always)]
    pub fn attach(&self) -> ATTACH_R {
        ATTACH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Stall Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 1 - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
    #[inline(always)]
    pub fn softok(&mut self) -> SOFTOK_W {
        SOFTOK_W { w: self }
    }
    #[doc = "Bit 3 - This bit is set when the current token being processed has completed"]
    #[inline(always)]
    pub fn tokdne(&mut self) -> TOKDNE_W {
        TOKDNE_W { w: self }
    }
    #[doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 5 - This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 6 - Attach Interrupt"]
    #[inline(always)]
    pub fn attach(&mut self) -> ATTACH_W {
        ATTACH_W { w: self }
    }
    #[doc = "Bit 7 - Stall Interrupt"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istat](index.html) module"]
pub struct ISTAT_SPEC;
impl crate::RegisterSpec for ISTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [istat::R](R) reader structure"]
impl crate::Readable for ISTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [istat::W](W) writer structure"]
impl crate::Writable for ISTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for ISTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
