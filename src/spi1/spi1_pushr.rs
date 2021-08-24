#[doc = "Register `PUSHR` reader"]
pub struct R(crate::R<SPI1_PUSHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI1_PUSHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI1_PUSHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI1_PUSHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUSHR` writer"]
pub struct W(crate::W<SPI1_PUSHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI1_PUSHR_SPEC>;
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
impl From<crate::W<SPI1_PUSHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI1_PUSHR_SPEC>) -> Self {
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
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Negate the PCS\\[x\\]
signal."]
    _0 = 0,
    #[doc = "1: Assert the PCS\\[x\\]
signal."]
    _1 = 1,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS` reader - Select which PCS signals are to be asserted for the transfer"]
pub struct PCS_R(crate::FieldReader<u8, PCS_A>);
impl PCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PCS_A> {
        match self.bits {
            0 => Some(PCS_A::_0),
            1 => Some(PCS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PCS_A::_1
    }
}
impl core::ops::Deref for PCS_R {
    type Target = crate::FieldReader<u8, PCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS` writer - Select which PCS signals are to be asserted for the transfer"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Negate the PCS\\[x\\]
signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS_A::_0)
    }
    #[doc = "Assert the PCS\\[x\\]
signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Clear Transfer Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCNT_A {
    #[doc = "0: Do not clear the TCR\\[TCNT\\]
field."]
    _0 = 0,
    #[doc = "1: Clear the TCR\\[TCNT\\]
field."]
    _1 = 1,
}
impl From<CTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: CTCNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCNT` reader - Clear Transfer Counter"]
pub struct CTCNT_R(crate::FieldReader<bool, CTCNT_A>);
impl CTCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCNT_A {
        match self.bits {
            false => CTCNT_A::_0,
            true => CTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTCNT_A::_1
    }
}
impl core::ops::Deref for CTCNT_R {
    type Target = crate::FieldReader<bool, CTCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCNT` writer - Clear Transfer Counter"]
pub struct CTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCNT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not clear the TCR\\[TCNT\\]
field."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTCNT_A::_0)
    }
    #[doc = "Clear the TCR\\[TCNT\\]
field."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTCNT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "End Of Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOQ_A {
    #[doc = "0: The SPI data is not the last data to transfer."]
    _0 = 0,
    #[doc = "1: The SPI data is the last data to transfer."]
    _1 = 1,
}
impl From<EOQ_A> for bool {
    #[inline(always)]
    fn from(variant: EOQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOQ` reader - End Of Queue"]
pub struct EOQ_R(crate::FieldReader<bool, EOQ_A>);
impl EOQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOQ_A {
        match self.bits {
            false => EOQ_A::_0,
            true => EOQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EOQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EOQ_A::_1
    }
}
impl core::ops::Deref for EOQ_R {
    type Target = crate::FieldReader<bool, EOQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOQ` writer - End Of Queue"]
pub struct EOQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EOQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SPI data is not the last data to transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQ_A::_0)
    }
    #[doc = "The SPI data is the last data to transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Clock and Transfer Attributes Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTAS_A {
    #[doc = "0: CTAR0"]
    _000 = 0,
    #[doc = "1: CTAR1"]
    _001 = 1,
}
impl From<CTAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CTAS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTAS` reader - Clock and Transfer Attributes Select"]
pub struct CTAS_R(crate::FieldReader<u8, CTAS_A>);
impl CTAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTAS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTAS_A> {
        match self.bits {
            0 => Some(CTAS_A::_000),
            1 => Some(CTAS_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == CTAS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == CTAS_A::_001
    }
}
impl core::ops::Deref for CTAS_R {
    type Target = crate::FieldReader<u8, CTAS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTAS` writer - Clock and Transfer Attributes Select"]
pub struct CTAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTAS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CTAR0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CTAS_A::_000)
    }
    #[doc = "CTAR1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CTAS_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Continuous Peripheral Chip Select Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: Return PCSn signals to their inactive state between transfers."]
    _0 = 0,
    #[doc = "1: Keep PCSn signals asserted between transfers."]
    _1 = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Continuous Peripheral Chip Select Enable"]
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::_0,
            true => CONT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CONT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CONT_A::_1
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Continuous Peripheral Chip Select Enable"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Return PCSn signals to their inactive state between transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_A::_0)
    }
    #[doc = "Keep PCSn signals asserted between transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - Clear Transfer Counter"]
    #[inline(always)]
    pub fn ctcnt(&self) -> CTCNT_R {
        CTCNT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline(always)]
    pub fn eoq(&self) -> EOQ_R {
        EOQ_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select"]
    #[inline(always)]
    pub fn ctas(&self) -> CTAS_R {
        CTAS_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Bits 16:21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bit 26 - Clear Transfer Counter"]
    #[inline(always)]
    pub fn ctcnt(&mut self) -> CTCNT_W {
        CTCNT_W { w: self }
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline(always)]
    pub fn eoq(&mut self) -> EOQ_W {
        EOQ_W { w: self }
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select"]
    #[inline(always)]
    pub fn ctas(&mut self) -> CTAS_W {
        CTAS_W { w: self }
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUSH TX FIFO Register In Master Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_pushr](index.html) module"]
pub struct SPI1_PUSHR_SPEC;
impl crate::RegisterSpec for SPI1_PUSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi1_pushr::R](R) reader structure"]
impl crate::Readable for SPI1_PUSHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi1_pushr::W](W) writer structure"]
impl crate::Writable for SPI1_PUSHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUSHR to value 0"]
impl crate::Resettable for SPI1_PUSHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
