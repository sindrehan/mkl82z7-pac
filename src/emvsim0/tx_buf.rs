#[doc = "Register `TX_BUF` reader"]
pub struct R(crate::R<TX_BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_BUF` writer"]
pub struct W(crate::W<TX_BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_BUF_SPEC>;
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
impl From<crate::W<TX_BUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE` writer - Transmit Data Byte"]
pub struct TX_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data Byte"]
    #[inline(always)]
    pub fn tx_byte(&mut self) -> TX_BYTE_W {
        TX_BYTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_buf](index.html) module"]
pub struct TX_BUF_SPEC;
impl crate::RegisterSpec for TX_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_buf::R](R) reader structure"]
impl crate::Readable for TX_BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_buf::W](W) writer structure"]
impl crate::Writable for TX_BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_BUF to value 0"]
impl crate::Resettable for TX_BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
