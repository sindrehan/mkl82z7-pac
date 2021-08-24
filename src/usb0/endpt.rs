#[doc = "Register `ENDPT%s` reader"]
pub struct R(crate::R<ENDPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPT%s` writer"]
pub struct W(crate::W<ENDPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPT_SPEC>;
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
impl From<crate::W<ENDPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPHSHK` reader - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
pub struct EPHSHK_R(crate::FieldReader<bool, bool>);
impl EPHSHK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPHSHK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPHSHK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPHSHK` writer - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
pub struct EPHSHK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPHSHK_W<'a> {
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
#[doc = "Field `EPSTALL` reader - When set, this bit indicates that the endpoint is stalled"]
pub struct EPSTALL_R(crate::FieldReader<bool, bool>);
impl EPSTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPSTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPSTALL` writer - When set, this bit indicates that the endpoint is stalled"]
pub struct EPSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSTALL_W<'a> {
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
#[doc = "Field `EPTXEN` reader - This bit, when set, enables the endpoint for TX transfers. See"]
pub struct EPTXEN_R(crate::FieldReader<bool, bool>);
impl EPTXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXEN` writer - This bit, when set, enables the endpoint for TX transfers. See"]
pub struct EPTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXEN_W<'a> {
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
#[doc = "Field `EPRXEN` reader - This bit, when set, enables the endpoint for RX transfers. See"]
pub struct EPRXEN_R(crate::FieldReader<bool, bool>);
impl EPRXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRXEN` writer - This bit, when set, enables the endpoint for RX transfers. See"]
pub struct EPRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRXEN_W<'a> {
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
#[doc = "Field `EPCTLDIS` reader - This bit, when set, disables control (SETUP) transfers"]
pub struct EPCTLDIS_R(crate::FieldReader<bool, bool>);
impl EPCTLDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPCTLDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPCTLDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPCTLDIS` writer - This bit, when set, disables control (SETUP) transfers"]
pub struct EPCTLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCTLDIS_W<'a> {
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
#[doc = "Field `RETRYDIS` reader - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
pub struct RETRYDIS_R(crate::FieldReader<bool, bool>);
impl RETRYDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETRYDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRYDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRYDIS` writer - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
pub struct RETRYDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRYDIS_W<'a> {
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
#[doc = "Host without a hub This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTWOHUB_A {
    #[doc = "0: Low-speed device connected to Host through a hub. PRE_PID will be generated as required."]
    _0 = 0,
    #[doc = "1: Low-speed device directly connected. No hub, or no low-speed device attached."]
    _1 = 1,
}
impl From<HOSTWOHUB_A> for bool {
    #[inline(always)]
    fn from(variant: HOSTWOHUB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOSTWOHUB` reader - Host without a hub This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
pub struct HOSTWOHUB_R(crate::FieldReader<bool, HOSTWOHUB_A>);
impl HOSTWOHUB_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOSTWOHUB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOSTWOHUB_A {
        match self.bits {
            false => HOSTWOHUB_A::_0,
            true => HOSTWOHUB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HOSTWOHUB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HOSTWOHUB_A::_1
    }
}
impl core::ops::Deref for HOSTWOHUB_R {
    type Target = crate::FieldReader<bool, HOSTWOHUB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOSTWOHUB` writer - Host without a hub This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
pub struct HOSTWOHUB_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTWOHUB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOSTWOHUB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-speed device connected to Host through a hub. PRE_PID will be generated as required."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOSTWOHUB_A::_0)
    }
    #[doc = "Low-speed device directly connected. No hub, or no low-speed device attached."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOSTWOHUB_A::_1)
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
    #[doc = "Bit 0 - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline(always)]
    pub fn ephshk(&self) -> EPHSHK_R {
        EPHSHK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When set, this bit indicates that the endpoint is stalled"]
    #[inline(always)]
    pub fn epstall(&self) -> EPSTALL_R {
        EPSTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit, when set, enables the endpoint for TX transfers. See"]
    #[inline(always)]
    pub fn eptxen(&self) -> EPTXEN_R {
        EPTXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit, when set, enables the endpoint for RX transfers. See"]
    #[inline(always)]
    pub fn eprxen(&self) -> EPRXEN_R {
        EPRXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit, when set, disables control (SETUP) transfers"]
    #[inline(always)]
    pub fn epctldis(&self) -> EPCTLDIS_R {
        EPCTLDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline(always)]
    pub fn retrydis(&self) -> RETRYDIS_R {
        RETRYDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Host without a hub This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline(always)]
    pub fn hostwohub(&self) -> HOSTWOHUB_R {
        HOSTWOHUB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline(always)]
    pub fn ephshk(&mut self) -> EPHSHK_W {
        EPHSHK_W { w: self }
    }
    #[doc = "Bit 1 - When set, this bit indicates that the endpoint is stalled"]
    #[inline(always)]
    pub fn epstall(&mut self) -> EPSTALL_W {
        EPSTALL_W { w: self }
    }
    #[doc = "Bit 2 - This bit, when set, enables the endpoint for TX transfers. See"]
    #[inline(always)]
    pub fn eptxen(&mut self) -> EPTXEN_W {
        EPTXEN_W { w: self }
    }
    #[doc = "Bit 3 - This bit, when set, enables the endpoint for RX transfers. See"]
    #[inline(always)]
    pub fn eprxen(&mut self) -> EPRXEN_W {
        EPRXEN_W { w: self }
    }
    #[doc = "Bit 4 - This bit, when set, disables control (SETUP) transfers"]
    #[inline(always)]
    pub fn epctldis(&mut self) -> EPCTLDIS_W {
        EPCTLDIS_W { w: self }
    }
    #[doc = "Bit 6 - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline(always)]
    pub fn retrydis(&mut self) -> RETRYDIS_W {
        RETRYDIS_W { w: self }
    }
    #[doc = "Bit 7 - Host without a hub This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline(always)]
    pub fn hostwohub(&mut self) -> HOSTWOHUB_W {
        HOSTWOHUB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endpt](index.html) module"]
pub struct ENDPT_SPEC;
impl crate::RegisterSpec for ENDPT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [endpt::R](R) reader structure"]
impl crate::Readable for ENDPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endpt::W](W) writer structure"]
impl crate::Writable for ENDPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDPT%s to value 0"]
impl crate::Resettable for ENDPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
