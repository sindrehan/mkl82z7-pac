#[doc = "Register `CTAR%s` reader"]
pub struct R(crate::R<SPI0_CTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI0_CTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI0_CTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI0_CTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTAR%s` writer"]
pub struct W(crate::W<SPI0_CTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI0_CTAR_SPEC>;
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
impl From<crate::W<SPI0_CTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI0_CTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BR` reader - Baud Rate Scaler"]
pub struct BR_R(crate::FieldReader<u8, u8>);
impl BR_R {
    pub(crate) fn new(bits: u8) -> Self {
        BR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BR` writer - Baud Rate Scaler"]
pub struct BR_W<'a> {
    w: &'a mut W,
}
impl<'a> BR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DT` reader - Delay After Transfer Scaler"]
pub struct DT_R(crate::FieldReader<u8, u8>);
impl DT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT` writer - Delay After Transfer Scaler"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ASC` reader - After SCK Delay Scaler"]
pub struct ASC_R(crate::FieldReader<u8, u8>);
impl ASC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ASC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC` writer - After SCK Delay Scaler"]
pub struct ASC_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CSSCK` reader - PCS to SCK Delay Scaler"]
pub struct CSSCK_R(crate::FieldReader<u8, u8>);
impl CSSCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSSCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSCK` writer - PCS to SCK Delay Scaler"]
pub struct CSSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Baud Rate Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PBR_A {
    #[doc = "0: Baud Rate Prescaler value is 2."]
    _00 = 0,
    #[doc = "1: Baud Rate Prescaler value is 3."]
    _01 = 1,
    #[doc = "2: Baud Rate Prescaler value is 5."]
    _10 = 2,
    #[doc = "3: Baud Rate Prescaler value is 7."]
    _11 = 3,
}
impl From<PBR_A> for u8 {
    #[inline(always)]
    fn from(variant: PBR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PBR` reader - Baud Rate Prescaler"]
pub struct PBR_R(crate::FieldReader<u8, PBR_A>);
impl PBR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBR_A {
        match self.bits {
            0 => PBR_A::_00,
            1 => PBR_A::_01,
            2 => PBR_A::_10,
            3 => PBR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == PBR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == PBR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PBR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PBR_A::_11
    }
}
impl core::ops::Deref for PBR_R {
    type Target = crate::FieldReader<u8, PBR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBR` writer - Baud Rate Prescaler"]
pub struct PBR_W<'a> {
    w: &'a mut W,
}
impl<'a> PBR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Baud Rate Prescaler value is 2."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PBR_A::_00)
    }
    #[doc = "Baud Rate Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PBR_A::_01)
    }
    #[doc = "Baud Rate Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PBR_A::_10)
    }
    #[doc = "Baud Rate Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PBR_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Delay after Transfer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PDT_A {
    #[doc = "0: Delay after Transfer Prescaler value is 1."]
    _00 = 0,
    #[doc = "1: Delay after Transfer Prescaler value is 3."]
    _01 = 1,
    #[doc = "2: Delay after Transfer Prescaler value is 5."]
    _10 = 2,
    #[doc = "3: Delay after Transfer Prescaler value is 7."]
    _11 = 3,
}
impl From<PDT_A> for u8 {
    #[inline(always)]
    fn from(variant: PDT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PDT` reader - Delay after Transfer Prescaler"]
pub struct PDT_R(crate::FieldReader<u8, PDT_A>);
impl PDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDT_A {
        match self.bits {
            0 => PDT_A::_00,
            1 => PDT_A::_01,
            2 => PDT_A::_10,
            3 => PDT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == PDT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == PDT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PDT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PDT_A::_11
    }
}
impl core::ops::Deref for PDT_R {
    type Target = crate::FieldReader<u8, PDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDT` writer - Delay after Transfer Prescaler"]
pub struct PDT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Delay after Transfer Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PDT_A::_00)
    }
    #[doc = "Delay after Transfer Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PDT_A::_01)
    }
    #[doc = "Delay after Transfer Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PDT_A::_10)
    }
    #[doc = "Delay after Transfer Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PDT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "After SCK Delay Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PASC_A {
    #[doc = "0: Delay after Transfer Prescaler value is 1."]
    _00 = 0,
    #[doc = "1: Delay after Transfer Prescaler value is 3."]
    _01 = 1,
    #[doc = "2: Delay after Transfer Prescaler value is 5."]
    _10 = 2,
    #[doc = "3: Delay after Transfer Prescaler value is 7."]
    _11 = 3,
}
impl From<PASC_A> for u8 {
    #[inline(always)]
    fn from(variant: PASC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PASC` reader - After SCK Delay Prescaler"]
pub struct PASC_R(crate::FieldReader<u8, PASC_A>);
impl PASC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PASC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PASC_A {
        match self.bits {
            0 => PASC_A::_00,
            1 => PASC_A::_01,
            2 => PASC_A::_10,
            3 => PASC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == PASC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == PASC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PASC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PASC_A::_11
    }
}
impl core::ops::Deref for PASC_R {
    type Target = crate::FieldReader<u8, PASC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASC` writer - After SCK Delay Prescaler"]
pub struct PASC_W<'a> {
    w: &'a mut W,
}
impl<'a> PASC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PASC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Delay after Transfer Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PASC_A::_00)
    }
    #[doc = "Delay after Transfer Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PASC_A::_01)
    }
    #[doc = "Delay after Transfer Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PASC_A::_10)
    }
    #[doc = "Delay after Transfer Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PASC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "PCS to SCK Delay Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCSSCK_A {
    #[doc = "0: PCS to SCK Prescaler value is 1."]
    _00 = 0,
    #[doc = "1: PCS to SCK Prescaler value is 3."]
    _01 = 1,
    #[doc = "2: PCS to SCK Prescaler value is 5."]
    _10 = 2,
    #[doc = "3: PCS to SCK Prescaler value is 7."]
    _11 = 3,
}
impl From<PCSSCK_A> for u8 {
    #[inline(always)]
    fn from(variant: PCSSCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCSSCK` reader - PCS to SCK Delay Prescaler"]
pub struct PCSSCK_R(crate::FieldReader<u8, PCSSCK_A>);
impl PCSSCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCSSCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSSCK_A {
        match self.bits {
            0 => PCSSCK_A::_00,
            1 => PCSSCK_A::_01,
            2 => PCSSCK_A::_10,
            3 => PCSSCK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == PCSSCK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == PCSSCK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PCSSCK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PCSSCK_A::_11
    }
}
impl core::ops::Deref for PCSSCK_R {
    type Target = crate::FieldReader<u8, PCSSCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSSCK` writer - PCS to SCK Delay Prescaler"]
pub struct PCSSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSSCK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCS to SCK Prescaler value is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCSSCK_A::_00)
    }
    #[doc = "PCS to SCK Prescaler value is 3."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCSSCK_A::_01)
    }
    #[doc = "PCS to SCK Prescaler value is 5."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCSSCK_A::_10)
    }
    #[doc = "PCS to SCK Prescaler value is 7."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PCSSCK_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFE_A {
    #[doc = "0: Data is transferred MSB first."]
    _0 = 0,
    #[doc = "1: Data is transferred LSB first."]
    _1 = 1,
}
impl From<LSBFE_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBFE` reader - LSB First"]
pub struct LSBFE_R(crate::FieldReader<bool, LSBFE_A>);
impl LSBFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSBFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBFE_A {
        match self.bits {
            false => LSBFE_A::_0,
            true => LSBFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LSBFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LSBFE_A::_1
    }
}
impl core::ops::Deref for LSBFE_R {
    type Target = crate::FieldReader<bool, LSBFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSBFE` writer - LSB First"]
pub struct LSBFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is transferred MSB first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBFE_A::_0)
    }
    #[doc = "Data is transferred LSB first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data is captured on the leading edge of SCK and changed on the following edge."]
    _0 = 0,
    #[doc = "1: Data is changed on the leading edge of SCK and captured on the following edge."]
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub struct CPHA_R(crate::FieldReader<bool, CPHA_A>);
impl CPHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPHA_A::_1
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, CPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: The inactive state value of SCK is low."]
    _0 = 0,
    #[doc = "1: The inactive state value of SCK is high."]
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPOL_A::_1
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The inactive state value of SCK is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "The inactive state value of SCK is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
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
#[doc = "Field `FMSZ` reader - Frame Size"]
pub struct FMSZ_R(crate::FieldReader<u8, u8>);
impl FMSZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMSZ` writer - Frame Size"]
pub struct FMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FMSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 27)) | ((value as u32 & 0x0f) << 27);
        self.w
    }
}
#[doc = "Double Baud Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBR_A {
    #[doc = "0: The baud rate is computed normally with a 50/50 duty cycle."]
    _0 = 0,
    #[doc = "1: The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    _1 = 1,
}
impl From<DBR_A> for bool {
    #[inline(always)]
    fn from(variant: DBR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBR` reader - Double Baud Rate"]
pub struct DBR_R(crate::FieldReader<bool, DBR_A>);
impl DBR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBR_A {
        match self.bits {
            false => DBR_A::_0,
            true => DBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBR_A::_1
    }
}
impl core::ops::Deref for DBR_R {
    type Target = crate::FieldReader<bool, DBR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBR` writer - Double Baud Rate"]
pub struct DBR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The baud rate is computed normally with a 50/50 duty cycle."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBR_A::_0)
    }
    #[doc = "The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBR_A::_1)
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
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline(always)]
    pub fn cssck(&self) -> CSSCK_R {
        CSSCK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn pbr(&self) -> PBR_R {
        PBR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline(always)]
    pub fn pdt(&self) -> PDT_R {
        PDT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pasc(&self) -> PASC_R {
        PASC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pcssck(&self) -> PCSSCK_R {
        PCSSCK_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - LSB First"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LSBFE_R {
        LSBFE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline(always)]
    pub fn fmsz(&self) -> FMSZ_R {
        FMSZ_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline(always)]
    pub fn dbr(&self) -> DBR_R {
        DBR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline(always)]
    pub fn br(&mut self) -> BR_W {
        BR_W { w: self }
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W {
        ASC_W { w: self }
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline(always)]
    pub fn cssck(&mut self) -> CSSCK_W {
        CSSCK_W { w: self }
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn pbr(&mut self) -> PBR_W {
        PBR_W { w: self }
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline(always)]
    pub fn pdt(&mut self) -> PDT_W {
        PDT_W { w: self }
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pasc(&mut self) -> PASC_W {
        PASC_W { w: self }
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pcssck(&mut self) -> PCSSCK_W {
        PCSSCK_W { w: self }
    }
    #[doc = "Bit 24 - LSB First"]
    #[inline(always)]
    pub fn lsbfe(&mut self) -> LSBFE_W {
        LSBFE_W { w: self }
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline(always)]
    pub fn fmsz(&mut self) -> FMSZ_W {
        FMSZ_W { w: self }
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline(always)]
    pub fn dbr(&mut self) -> DBR_W {
        DBR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock and Transfer Attributes Register (In Master Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_ctar](index.html) module"]
pub struct SPI0_CTAR_SPEC;
impl crate::RegisterSpec for SPI0_CTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi0_ctar::R](R) reader structure"]
impl crate::Readable for SPI0_CTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi0_ctar::W](W) writer structure"]
impl crate::Writable for SPI0_CTAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTAR%s to value 0x7800_0000"]
impl crate::Resettable for SPI0_CTAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7800_0000
    }
}
