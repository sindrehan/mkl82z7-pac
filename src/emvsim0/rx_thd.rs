#[doc = "Register `RX_THD` reader"]
pub struct R(crate::R<RX_THD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_THD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_THD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_THD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_THD` writer"]
pub struct W(crate::W<RX_THD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_THD_SPEC>;
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
impl From<crate::W<RX_THD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_THD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDT` reader - Receiver Data Threshold Value"]
pub struct RDT_R(crate::FieldReader<u8, u8>);
impl RDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDT` writer - Receiver Data Threshold Value"]
pub struct RDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Receiver NACK Threshold Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RNCK_THD_A {
    #[doc = "0: Zero Threshold. RTE will not be set"]
    _0 = 0,
}
impl From<RNCK_THD_A> for u8 {
    #[inline(always)]
    fn from(variant: RNCK_THD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RNCK_THD` reader - Receiver NACK Threshold Value"]
pub struct RNCK_THD_R(crate::FieldReader<u8, RNCK_THD_A>);
impl RNCK_THD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNCK_THD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RNCK_THD_A> {
        match self.bits {
            0 => Some(RNCK_THD_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RNCK_THD_A::_0
    }
}
impl core::ops::Deref for RNCK_THD_R {
    type Target = crate::FieldReader<u8, RNCK_THD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNCK_THD` writer - Receiver NACK Threshold Value"]
pub struct RNCK_THD_W<'a> {
    w: &'a mut W,
}
impl<'a> RNCK_THD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNCK_THD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Zero Threshold. RTE will not be set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNCK_THD_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Receiver Data Threshold Value"]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Receiver NACK Threshold Value"]
    #[inline(always)]
    pub fn rnck_thd(&self) -> RNCK_THD_R {
        RNCK_THD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receiver Data Threshold Value"]
    #[inline(always)]
    pub fn rdt(&mut self) -> RDT_W {
        RDT_W { w: self }
    }
    #[doc = "Bits 8:11 - Receiver NACK Threshold Value"]
    #[inline(always)]
    pub fn rnck_thd(&mut self) -> RNCK_THD_W {
        RNCK_THD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_thd](index.html) module"]
pub struct RX_THD_SPEC;
impl crate::RegisterSpec for RX_THD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_thd::R](R) reader structure"]
impl crate::Readable for RX_THD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_thd::W](W) writer structure"]
impl crate::Writable for RX_THD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_THD to value 0x01"]
impl crate::Resettable for RX_THD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
