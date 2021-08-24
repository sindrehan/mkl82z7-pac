#[doc = "Register `SOPT5` reader"]
pub struct R(crate::R<SOPT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT5` writer"]
pub struct W(crate::W<SOPT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT5_SPEC>;
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
impl From<crate::W<SOPT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LPUART0 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART0TXSRC_A {
    #[doc = "0: LPUART0_TX pin"]
    _00 = 0,
    #[doc = "1: LPUART0_TX pin modulated with TPM1 channel 0 output"]
    _01 = 1,
    #[doc = "2: LPUART0_TX pin modulated with TPM2 channel 0 output"]
    _10 = 2,
}
impl From<LPUART0TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART0TXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPUART0TXSRC` reader - LPUART0 transmit data source select"]
pub struct LPUART0TXSRC_R(crate::FieldReader<u8, LPUART0TXSRC_A>);
impl LPUART0TXSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART0TXSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPUART0TXSRC_A> {
        match self.bits {
            0 => Some(LPUART0TXSRC_A::_00),
            1 => Some(LPUART0TXSRC_A::_01),
            2 => Some(LPUART0TXSRC_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LPUART0TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LPUART0TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == LPUART0TXSRC_A::_10
    }
}
impl core::ops::Deref for LPUART0TXSRC_R {
    type Target = crate::FieldReader<u8, LPUART0TXSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART0TXSRC` writer - LPUART0 transmit data source select"]
pub struct LPUART0TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART0TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART0TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART0_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_00)
    }
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_01)
    }
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "LPUART 0 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART0RXSRC_A {
    #[doc = "0: LPUART0_RX pin"]
    _00 = 0,
    #[doc = "1: CMP0"]
    _01 = 1,
}
impl From<LPUART0RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART0RXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPUART0RXSRC` reader - LPUART 0 receive data source select"]
pub struct LPUART0RXSRC_R(crate::FieldReader<u8, LPUART0RXSRC_A>);
impl LPUART0RXSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART0RXSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPUART0RXSRC_A> {
        match self.bits {
            0 => Some(LPUART0RXSRC_A::_00),
            1 => Some(LPUART0RXSRC_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LPUART0RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LPUART0RXSRC_A::_01
    }
}
impl core::ops::Deref for LPUART0RXSRC_R {
    type Target = crate::FieldReader<u8, LPUART0RXSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART0RXSRC` writer - LPUART 0 receive data source select"]
pub struct LPUART0RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART0RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART0RXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART0_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0RXSRC_A::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0RXSRC_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "LPUART1 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART1TXSRC_A {
    #[doc = "0: LPUART1_TX pin"]
    _00 = 0,
    #[doc = "1: LPUART1_TX pin modulated with TPM1 channel 0 output"]
    _01 = 1,
    #[doc = "2: LPUART1_TX pin modulated with TPM2 channel 0 output"]
    _10 = 2,
}
impl From<LPUART1TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1TXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPUART1TXSRC` reader - LPUART1 transmit data source select"]
pub struct LPUART1TXSRC_R(crate::FieldReader<u8, LPUART1TXSRC_A>);
impl LPUART1TXSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART1TXSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPUART1TXSRC_A> {
        match self.bits {
            0 => Some(LPUART1TXSRC_A::_00),
            1 => Some(LPUART1TXSRC_A::_01),
            2 => Some(LPUART1TXSRC_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LPUART1TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LPUART1TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == LPUART1TXSRC_A::_10
    }
}
impl core::ops::Deref for LPUART1TXSRC_R {
    type Target = crate::FieldReader<u8, LPUART1TXSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1TXSRC` writer - LPUART1 transmit data source select"]
pub struct LPUART1TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART1_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART1TXSRC_A::_00)
    }
    #[doc = "LPUART1_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART1TXSRC_A::_01)
    }
    #[doc = "LPUART1_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART1TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "LPUART1 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART1RXSRC_A {
    #[doc = "0: LPUART1_RX pin"]
    _00 = 0,
    #[doc = "1: CMP0"]
    _01 = 1,
}
impl From<LPUART1RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1RXSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPUART1RXSRC` reader - LPUART1 receive data source select"]
pub struct LPUART1RXSRC_R(crate::FieldReader<u8, LPUART1RXSRC_A>);
impl LPUART1RXSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART1RXSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPUART1RXSRC_A> {
        match self.bits {
            0 => Some(LPUART1RXSRC_A::_00),
            1 => Some(LPUART1RXSRC_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LPUART1RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LPUART1RXSRC_A::_01
    }
}
impl core::ops::Deref for LPUART1RXSRC_R {
    type Target = crate::FieldReader<u8, LPUART1RXSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1RXSRC` writer - LPUART1 receive data source select"]
pub struct LPUART1RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1RXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART1_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART1RXSRC_A::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART1RXSRC_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - LPUART0 transmit data source select"]
    #[inline(always)]
    pub fn lpuart0txsrc(&self) -> LPUART0TXSRC_R {
        LPUART0TXSRC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - LPUART 0 receive data source select"]
    #[inline(always)]
    pub fn lpuart0rxsrc(&self) -> LPUART0RXSRC_R {
        LPUART0RXSRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - LPUART1 transmit data source select"]
    #[inline(always)]
    pub fn lpuart1txsrc(&self) -> LPUART1TXSRC_R {
        LPUART1TXSRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - LPUART1 receive data source select"]
    #[inline(always)]
    pub fn lpuart1rxsrc(&self) -> LPUART1RXSRC_R {
        LPUART1RXSRC_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - LPUART0 transmit data source select"]
    #[inline(always)]
    pub fn lpuart0txsrc(&mut self) -> LPUART0TXSRC_W {
        LPUART0TXSRC_W { w: self }
    }
    #[doc = "Bits 18:19 - LPUART 0 receive data source select"]
    #[inline(always)]
    pub fn lpuart0rxsrc(&mut self) -> LPUART0RXSRC_W {
        LPUART0RXSRC_W { w: self }
    }
    #[doc = "Bits 20:21 - LPUART1 transmit data source select"]
    #[inline(always)]
    pub fn lpuart1txsrc(&mut self) -> LPUART1TXSRC_W {
        LPUART1TXSRC_W { w: self }
    }
    #[doc = "Bits 22:23 - LPUART1 receive data source select"]
    #[inline(always)]
    pub fn lpuart1rxsrc(&mut self) -> LPUART1RXSRC_W {
        LPUART1RXSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt5](index.html) module"]
pub struct SOPT5_SPEC;
impl crate::RegisterSpec for SOPT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt5::R](R) reader structure"]
impl crate::Readable for SOPT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt5::W](W) writer structure"]
impl crate::Writable for SOPT5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOPT5 to value 0"]
impl crate::Resettable for SOPT5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
