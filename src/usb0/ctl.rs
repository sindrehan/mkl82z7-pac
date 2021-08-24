#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBENSOFEN_A {
    #[doc = "0: Disables the USB Module."]
    _0 = 0,
    #[doc = "1: Enables the USB Module."]
    _1 = 1,
}
impl From<USBENSOFEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBENSOFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBENSOFEN` reader - USB Enable"]
pub struct USBENSOFEN_R(crate::FieldReader<bool, USBENSOFEN_A>);
impl USBENSOFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBENSOFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBENSOFEN_A {
        match self.bits {
            false => USBENSOFEN_A::_0,
            true => USBENSOFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBENSOFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBENSOFEN_A::_1
    }
}
impl core::ops::Deref for USBENSOFEN_R {
    type Target = crate::FieldReader<bool, USBENSOFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBENSOFEN` writer - USB Enable"]
pub struct USBENSOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBENSOFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBENSOFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the USB Module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBENSOFEN_A::_0)
    }
    #[doc = "Enables the USB Module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBENSOFEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `ODDRST` reader - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
pub struct ODDRST_R(crate::FieldReader<bool, bool>);
impl ODDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODDRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODDRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODDRST` writer - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
pub struct ODDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ODDRST_W<'a> {
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
#[doc = "Field `RESUME` reader - When set to 1 this bit enables the USB Module to execute resume signaling"]
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
#[doc = "Field `RESUME` writer - When set to 1 this bit enables the USB Module to execute resume signaling"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HOSTMODEEN` reader - When set to 1, this bit enables the USB Module to operate in Host mode"]
pub struct HOSTMODEEN_R(crate::FieldReader<bool, bool>);
impl HOSTMODEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOSTMODEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOSTMODEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOSTMODEEN` writer - When set to 1, this bit enables the USB Module to operate in Host mode"]
pub struct HOSTMODEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTMODEEN_W<'a> {
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
#[doc = "Field `RESET` reader - Setting this bit enables the USB Module to generate USB reset signaling"]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - Setting this bit enables the USB Module to generate USB reset signaling"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Field `TXSUSPENDTOKENBUSY` reader - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
pub struct TXSUSPENDTOKENBUSY_R(crate::FieldReader<bool, bool>);
impl TXSUSPENDTOKENBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSUSPENDTOKENBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSUSPENDTOKENBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSUSPENDTOKENBUSY` writer - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
pub struct TXSUSPENDTOKENBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSUSPENDTOKENBUSY_W<'a> {
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
#[doc = "Field `SE0` reader - Live USB Single Ended Zero signal"]
pub struct SE0_R(crate::FieldReader<bool, bool>);
impl SE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE0` writer - Live USB Single Ended Zero signal"]
pub struct SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0_W<'a> {
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
#[doc = "Field `JSTATE` reader - Live USB differential receiver JSTATE signal"]
pub struct JSTATE_R(crate::FieldReader<bool, bool>);
impl JSTATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSTATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JSTATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSTATE` writer - Live USB differential receiver JSTATE signal"]
pub struct JSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> JSTATE_W<'a> {
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
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    pub fn usbensofen(&self) -> USBENSOFEN_R {
        USBENSOFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
    #[inline(always)]
    pub fn oddrst(&self) -> ODDRST_R {
        ODDRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline(always)]
    pub fn hostmodeen(&self) -> HOSTMODEEN_R {
        HOSTMODEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
    #[inline(always)]
    pub fn txsuspendtokenbusy(&self) -> TXSUSPENDTOKENBUSY_R {
        TXSUSPENDTOKENBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Live USB Single Ended Zero signal"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
    #[inline(always)]
    pub fn jstate(&self) -> JSTATE_R {
        JSTATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    pub fn usbensofen(&mut self) -> USBENSOFEN_W {
        USBENSOFEN_W { w: self }
    }
    #[doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
    #[inline(always)]
    pub fn oddrst(&mut self) -> ODDRST_W {
        ODDRST_W { w: self }
    }
    #[doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline(always)]
    pub fn hostmodeen(&mut self) -> HOSTMODEEN_W {
        HOSTMODEEN_W { w: self }
    }
    #[doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 5 - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
    #[inline(always)]
    pub fn txsuspendtokenbusy(&mut self) -> TXSUSPENDTOKENBUSY_W {
        TXSUSPENDTOKENBUSY_W { w: self }
    }
    #[doc = "Bit 6 - Live USB Single Ended Zero signal"]
    #[inline(always)]
    pub fn se0(&mut self) -> SE0_W {
        SE0_W { w: self }
    }
    #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
    #[inline(always)]
    pub fn jstate(&mut self) -> JSTATE_W {
        JSTATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
