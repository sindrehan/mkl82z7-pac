#[doc = "Register `PUSHR_SLAVE` reader"]
pub struct R(crate::R<SPI0_PUSHR_SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI0_PUSHR_SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI0_PUSHR_SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI0_PUSHR_SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUSHR_SLAVE` writer"]
pub struct W(crate::W<SPI0_PUSHR_SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI0_PUSHR_SLAVE_SPEC>;
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
impl From<crate::W<SPI0_PUSHR_SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI0_PUSHR_SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` reader - Transmit Data"]
pub struct TXDATA_R(crate::FieldReader<u16, u16>);
impl TXDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDATA` writer - Transmit Data"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUSH TX FIFO Register In Slave Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_pushr_slave](index.html) module"]
pub struct SPI0_PUSHR_SLAVE_SPEC;
impl crate::RegisterSpec for SPI0_PUSHR_SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi0_pushr_slave::R](R) reader structure"]
impl crate::Readable for SPI0_PUSHR_SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi0_pushr_slave::W](W) writer structure"]
impl crate::Writable for SPI0_PUSHR_SLAVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUSHR_SLAVE to value 0"]
impl crate::Resettable for SPI0_PUSHR_SLAVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
