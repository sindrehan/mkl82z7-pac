#[doc = "Register `TX_THD` reader"]
pub struct R(crate::R<TX_THD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_THD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_THD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_THD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_THD` writer"]
pub struct W(crate::W<TX_THD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_THD_SPEC>;
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
impl From<crate::W<TX_THD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_THD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDT` reader - Transmitter Data Threshold Value"]
pub struct TDT_R(crate::FieldReader<u8, u8>);
impl TDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDT` writer - Transmitter Data Threshold Value"]
pub struct TDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Transmitter NACK Threshold Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TNCK_THD_A {
    #[doc = "0: TNTE will never be set; retransmission after NACK reception is disabled."]
    _0 = 0,
    #[doc = "1: TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
    _1 = 1,
}
impl From<TNCK_THD_A> for u8 {
    #[inline(always)]
    fn from(variant: TNCK_THD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TNCK_THD` reader - Transmitter NACK Threshold Value"]
pub struct TNCK_THD_R(crate::FieldReader<u8, TNCK_THD_A>);
impl TNCK_THD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TNCK_THD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TNCK_THD_A> {
        match self.bits {
            0 => Some(TNCK_THD_A::_0),
            1 => Some(TNCK_THD_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TNCK_THD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TNCK_THD_A::_1
    }
}
impl core::ops::Deref for TNCK_THD_R {
    type Target = crate::FieldReader<u8, TNCK_THD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TNCK_THD` writer - Transmitter NACK Threshold Value"]
pub struct TNCK_THD_W<'a> {
    w: &'a mut W,
}
impl<'a> TNCK_THD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TNCK_THD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TNTE will never be set; retransmission after NACK reception is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TNCK_THD_A::_0)
    }
    #[doc = "TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TNCK_THD_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmitter Data Threshold Value"]
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transmitter NACK Threshold Value"]
    #[inline(always)]
    pub fn tnck_thd(&self) -> TNCK_THD_R {
        TNCK_THD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Data Threshold Value"]
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W {
        TDT_W { w: self }
    }
    #[doc = "Bits 8:11 - Transmitter NACK Threshold Value"]
    #[inline(always)]
    pub fn tnck_thd(&mut self) -> TNCK_THD_W {
        TNCK_THD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_thd](index.html) module"]
pub struct TX_THD_SPEC;
impl crate::RegisterSpec for TX_THD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_thd::R](R) reader structure"]
impl crate::Readable for TX_THD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_thd::W](W) writer structure"]
impl crate::Writable for TX_THD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_THD to value 0x0f"]
impl crate::Resettable for TX_THD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
