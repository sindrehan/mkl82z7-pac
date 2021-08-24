#[doc = "Register `ERRSTAT` reader"]
pub struct R(crate::R<ERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRSTAT` writer"]
pub struct W(crate::W<ERRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRSTAT_SPEC>;
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
impl From<crate::W<ERRSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIDERR` reader - This bit is set when the PID check field fails."]
pub struct PIDERR_R(crate::FieldReader<bool, bool>);
impl PIDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIDERR` writer - This bit is set when the PID check field fails."]
pub struct PIDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDERR_W<'a> {
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
#[doc = "Field `CRC5EOF` reader - This error interrupt has two functions"]
pub struct CRC5EOF_R(crate::FieldReader<bool, bool>);
impl CRC5EOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC5EOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC5EOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC5EOF` writer - This error interrupt has two functions"]
pub struct CRC5EOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC5EOF_W<'a> {
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
#[doc = "Field `CRC16` reader - This bit is set when a data packet is rejected due to a CRC16 error."]
pub struct CRC16_R(crate::FieldReader<bool, bool>);
impl CRC16_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC16` writer - This bit is set when a data packet is rejected due to a CRC16 error."]
pub struct CRC16_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16_W<'a> {
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
#[doc = "Field `DFN8` reader - This bit is set if the data field received was not 8 bits in length"]
pub struct DFN8_R(crate::FieldReader<bool, bool>);
impl DFN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFN8` writer - This bit is set if the data field received was not 8 bits in length"]
pub struct DFN8_W<'a> {
    w: &'a mut W,
}
impl<'a> DFN8_W<'a> {
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
#[doc = "Field `BTOERR` reader - This bit is set when a bus turnaround timeout error occurs"]
pub struct BTOERR_R(crate::FieldReader<bool, bool>);
impl BTOERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTOERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTOERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTOERR` writer - This bit is set when a bus turnaround timeout error occurs"]
pub struct BTOERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BTOERR_W<'a> {
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
#[doc = "Field `DMAERR` reader - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
pub struct DMAERR_R(crate::FieldReader<bool, bool>);
impl DMAERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAERR` writer - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
pub struct DMAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAERR_W<'a> {
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
#[doc = "Field `OWNERR` reader - This field is valid when the USB Module is operating in peripheral mode (CTL\\[HOSTMODEEN\\]=0)"]
pub struct OWNERR_R(crate::FieldReader<bool, bool>);
impl OWNERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OWNERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OWNERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OWNERR` writer - This field is valid when the USB Module is operating in peripheral mode (CTL\\[HOSTMODEEN\\]=0)"]
pub struct OWNERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OWNERR_W<'a> {
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
#[doc = "Field `BTSERR` reader - This bit is set when a bit stuff error is detected"]
pub struct BTSERR_R(crate::FieldReader<bool, bool>);
impl BTSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTSERR` writer - This bit is set when a bit stuff error is detected"]
pub struct BTSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BTSERR_W<'a> {
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
    #[doc = "Bit 0 - This bit is set when the PID check field fails."]
    #[inline(always)]
    pub fn piderr(&self) -> PIDERR_R {
        PIDERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This error interrupt has two functions"]
    #[inline(always)]
    pub fn crc5eof(&self) -> CRC5EOF_R {
        CRC5EOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is set when a data packet is rejected due to a CRC16 error."]
    #[inline(always)]
    pub fn crc16(&self) -> CRC16_R {
        CRC16_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is set if the data field received was not 8 bits in length"]
    #[inline(always)]
    pub fn dfn8(&self) -> DFN8_R {
        DFN8_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is set when a bus turnaround timeout error occurs"]
    #[inline(always)]
    pub fn btoerr(&self) -> BTOERR_R {
        BTOERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
    #[inline(always)]
    pub fn dmaerr(&self) -> DMAERR_R {
        DMAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This field is valid when the USB Module is operating in peripheral mode (CTL\\[HOSTMODEEN\\]=0)"]
    #[inline(always)]
    pub fn ownerr(&self) -> OWNERR_R {
        OWNERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is set when a bit stuff error is detected"]
    #[inline(always)]
    pub fn btserr(&self) -> BTSERR_R {
        BTSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when the PID check field fails."]
    #[inline(always)]
    pub fn piderr(&mut self) -> PIDERR_W {
        PIDERR_W { w: self }
    }
    #[doc = "Bit 1 - This error interrupt has two functions"]
    #[inline(always)]
    pub fn crc5eof(&mut self) -> CRC5EOF_W {
        CRC5EOF_W { w: self }
    }
    #[doc = "Bit 2 - This bit is set when a data packet is rejected due to a CRC16 error."]
    #[inline(always)]
    pub fn crc16(&mut self) -> CRC16_W {
        CRC16_W { w: self }
    }
    #[doc = "Bit 3 - This bit is set if the data field received was not 8 bits in length"]
    #[inline(always)]
    pub fn dfn8(&mut self) -> DFN8_W {
        DFN8_W { w: self }
    }
    #[doc = "Bit 4 - This bit is set when a bus turnaround timeout error occurs"]
    #[inline(always)]
    pub fn btoerr(&mut self) -> BTOERR_W {
        BTOERR_W { w: self }
    }
    #[doc = "Bit 5 - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
    #[inline(always)]
    pub fn dmaerr(&mut self) -> DMAERR_W {
        DMAERR_W { w: self }
    }
    #[doc = "Bit 6 - This field is valid when the USB Module is operating in peripheral mode (CTL\\[HOSTMODEEN\\]=0)"]
    #[inline(always)]
    pub fn ownerr(&mut self) -> OWNERR_W {
        OWNERR_W { w: self }
    }
    #[doc = "Bit 7 - This bit is set when a bit stuff error is detected"]
    #[inline(always)]
    pub fn btserr(&mut self) -> BTSERR_W {
        BTSERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errstat](index.html) module"]
pub struct ERRSTAT_SPEC;
impl crate::RegisterSpec for ERRSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [errstat::R](R) reader structure"]
impl crate::Readable for ERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errstat::W](W) writer structure"]
impl crate::Writable for ERRSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRSTAT to value 0"]
impl crate::Resettable for ERRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
