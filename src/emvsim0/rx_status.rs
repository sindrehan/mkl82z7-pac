#[doc = "Register `RX_STATUS` reader"]
pub struct R(crate::R<RX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_STATUS` writer"]
pub struct W(crate::W<RX_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_STATUS_SPEC>;
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
impl From<crate::W<RX_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFO_A {
    #[doc = "0: No overrun error has occurred (default)"]
    _0 = 0,
    #[doc = "1: A byte was received when the received FIFO was already full"]
    _1 = 1,
}
impl From<RFO_A> for bool {
    #[inline(always)]
    fn from(variant: RFO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO` reader - Receive FIFO Overflow Flag"]
pub struct RFO_R(crate::FieldReader<bool, RFO_A>);
impl RFO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFO_A {
        match self.bits {
            false => RFO_A::_0,
            true => RFO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFO_A::_1
    }
}
impl core::ops::Deref for RFO_R {
    type Target = crate::FieldReader<bool, RFO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFO` writer - Receive FIFO Overflow Flag"]
pub struct RFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RFO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No overrun error has occurred (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFO_A::_0)
    }
    #[doc = "A byte was received when the received FIFO was already full"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFO_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Receive Data Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATA_A {
    #[doc = "0: No new byte is received"]
    _0 = 0,
    #[doc = "1: New byte is received ans stored in Receive FIFO"]
    _1 = 1,
}
impl From<RX_DATA_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DATA` reader - Receive Data Interrupt Flag"]
pub struct RX_DATA_R(crate::FieldReader<bool, RX_DATA_A>);
impl RX_DATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DATA_A {
        match self.bits {
            false => RX_DATA_A::_0,
            true => RX_DATA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RX_DATA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RX_DATA_A::_1
    }
}
impl core::ops::Deref for RX_DATA_R {
    type Target = crate::FieldReader<bool, RX_DATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA` writer - Receive Data Interrupt Flag"]
pub struct RX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No new byte is received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DATA_A::_0)
    }
    #[doc = "New byte is received ans stored in Receive FIFO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DATA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Receive Data Threshold Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDTF_A {
    #[doc = "0: Number of unread bytes in receive FIFO less than the value set by RDT\\[3:0\\]
(default)."]
    _0 = 0,
    #[doc = "1: Number of unread bytes in receive FIFO greater or than equal to value set by RDT\\[3:0\\]."]
    _1 = 1,
}
impl From<RDTF_A> for bool {
    #[inline(always)]
    fn from(variant: RDTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDTF` reader - Receive Data Threshold Interrupt Flag"]
pub struct RDTF_R(crate::FieldReader<bool, RDTF_A>);
impl RDTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDTF_A {
        match self.bits {
            false => RDTF_A::_0,
            true => RDTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDTF_A::_1
    }
}
impl core::ops::Deref for RDTF_R {
    type Target = crate::FieldReader<bool, RDTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LRC Check OK Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC_OK_A {
    #[doc = "0: Current LRC value does not match remainder."]
    _0 = 0,
    #[doc = "1: Current calculated LRC value matches the expected result (i.e. zero)."]
    _1 = 1,
}
impl From<LRC_OK_A> for bool {
    #[inline(always)]
    fn from(variant: LRC_OK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRC_OK` reader - LRC Check OK Flag"]
pub struct LRC_OK_R(crate::FieldReader<bool, LRC_OK_A>);
impl LRC_OK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRC_OK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRC_OK_A {
        match self.bits {
            false => LRC_OK_A::_0,
            true => LRC_OK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LRC_OK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LRC_OK_A::_1
    }
}
impl core::ops::Deref for LRC_OK_R {
    type Target = crate::FieldReader<bool, LRC_OK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CRC Check OK Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_OK_A {
    #[doc = "0: Current CRC value does not match remainder."]
    _0 = 0,
    #[doc = "1: Current calculated CRC value matches the expected result."]
    _1 = 1,
}
impl From<CRC_OK_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_OK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_OK` reader - CRC Check OK Flag"]
pub struct CRC_OK_R(crate::FieldReader<bool, CRC_OK_A>);
impl CRC_OK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_OK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_OK_A {
        match self.bits {
            false => CRC_OK_A::_0,
            true => CRC_OK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRC_OK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRC_OK_A::_1
    }
}
impl core::ops::Deref for CRC_OK_R {
    type Target = crate::FieldReader<bool, CRC_OK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Character Wait Time Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWT_ERR_A {
    #[doc = "0: No CWT violation has occurred (default)."]
    _0 = 0,
    #[doc = "1: Time between two consecutive characters has exceeded the value in CHAR_WAIT."]
    _1 = 1,
}
impl From<CWT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: CWT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWT_ERR` reader - Character Wait Time Error Flag"]
pub struct CWT_ERR_R(crate::FieldReader<bool, CWT_ERR_A>);
impl CWT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWT_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CWT_ERR_A {
        match self.bits {
            false => CWT_ERR_A::_0,
            true => CWT_ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CWT_ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CWT_ERR_A::_1
    }
}
impl core::ops::Deref for CWT_ERR_R {
    type Target = crate::FieldReader<bool, CWT_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWT_ERR` writer - Character Wait Time Error Flag"]
pub struct CWT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CWT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWT_ERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No CWT violation has occurred (default)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWT_ERR_A::_0)
    }
    #[doc = "Time between two consecutive characters has exceeded the value in CHAR_WAIT."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWT_ERR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Received NACK Threshold Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTE_A {
    #[doc = "0: Number of NACKs generated by the receiver is less than the value programmed in RTH\\[3:0\\]"]
    _0 = 0,
    #[doc = "1: Number of NACKs generated by the receiver is equal to the value programmed in RTH\\[3:0\\]"]
    _1 = 1,
}
impl From<RTE_A> for bool {
    #[inline(always)]
    fn from(variant: RTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTE` reader - Received NACK Threshold Error Flag"]
pub struct RTE_R(crate::FieldReader<bool, RTE_A>);
impl RTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTE_A {
        match self.bits {
            false => RTE_A::_0,
            true => RTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTE_A::_1
    }
}
impl core::ops::Deref for RTE_R {
    type Target = crate::FieldReader<bool, RTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTE` writer - Received NACK Threshold Error Flag"]
pub struct RTE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Number of NACKs generated by the receiver is less than the value programmed in RTH\\[3:0\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTE_A::_0)
    }
    #[doc = "Number of NACKs generated by the receiver is equal to the value programmed in RTH\\[3:0\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Block Wait Time Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWT_ERR_A {
    #[doc = "0: Block wait time not exceeded"]
    _0 = 0,
    #[doc = "1: Block wait time was exceeded"]
    _1 = 1,
}
impl From<BWT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BWT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWT_ERR` reader - Block Wait Time Error Flag"]
pub struct BWT_ERR_R(crate::FieldReader<bool, BWT_ERR_A>);
impl BWT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWT_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWT_ERR_A {
        match self.bits {
            false => BWT_ERR_A::_0,
            true => BWT_ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BWT_ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BWT_ERR_A::_1
    }
}
impl core::ops::Deref for BWT_ERR_R {
    type Target = crate::FieldReader<bool, BWT_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWT_ERR` writer - Block Wait Time Error Flag"]
pub struct BWT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BWT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWT_ERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Block wait time not exceeded"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWT_ERR_A::_0)
    }
    #[doc = "Block wait time was exceeded"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWT_ERR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Block Guard Time Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGT_ERR_A {
    #[doc = "0: Block guard time was sufficient"]
    _0 = 0,
    #[doc = "1: Block guard time was too small"]
    _1 = 1,
}
impl From<BGT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BGT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGT_ERR` reader - Block Guard Time Error Flag"]
pub struct BGT_ERR_R(crate::FieldReader<bool, BGT_ERR_A>);
impl BGT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGT_ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGT_ERR_A {
        match self.bits {
            false => BGT_ERR_A::_0,
            true => BGT_ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BGT_ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BGT_ERR_A::_1
    }
}
impl core::ops::Deref for BGT_ERR_R {
    type Target = crate::FieldReader<bool, BGT_ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGT_ERR` writer - Block Guard Time Error Flag"]
pub struct BGT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BGT_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGT_ERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Block guard time was sufficient"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGT_ERR_A::_0)
    }
    #[doc = "Block guard time was too small"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGT_ERR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEF_A {
    #[doc = "0: No parity error detected"]
    _0 = 0,
    #[doc = "1: Parity error detected"]
    _1 = 1,
}
impl From<PEF_A> for bool {
    #[inline(always)]
    fn from(variant: PEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEF` reader - Parity Error Flag"]
pub struct PEF_R(crate::FieldReader<bool, PEF_A>);
impl PEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEF_A {
        match self.bits {
            false => PEF_A::_0,
            true => PEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PEF_A::_1
    }
}
impl core::ops::Deref for PEF_R {
    type Target = crate::FieldReader<bool, PEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEF` writer - Parity Error Flag"]
pub struct PEF_W<'a> {
    w: &'a mut W,
}
impl<'a> PEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEF_A::_0)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Frame Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: No frame error detected"]
    _0 = 0,
    #[doc = "1: Frame error detected"]
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - Frame Error Flag"]
pub struct FEF_R(crate::FieldReader<bool, FEF_A>);
impl FEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FEF_A::_1
    }
}
impl core::ops::Deref for FEF_R {
    type Target = crate::FieldReader<bool, FEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEF` writer - Frame Error Flag"]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No frame error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEF_A::_0)
    }
    #[doc = "Frame error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RX_WPTR` reader - Receive FIFO Write Pointer Value"]
pub struct RX_WPTR_R(crate::FieldReader<u8, u8>);
impl RX_WPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_WPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Byte Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_CNT_A {
    #[doc = "0: FIFO is emtpy"]
    _0 = 0,
}
impl From<RX_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_CNT` reader - Receive FIFO Byte Count"]
pub struct RX_CNT_R(crate::FieldReader<u8, RX_CNT_A>);
impl RX_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_CNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_CNT_A> {
        match self.bits {
            0 => Some(RX_CNT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RX_CNT_A::_0
    }
}
impl core::ops::Deref for RX_CNT_R {
    type Target = crate::FieldReader<u8, RX_CNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfo(&self) -> RFO_R {
        RFO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Data Interrupt Flag"]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Data Threshold Interrupt Flag"]
    #[inline(always)]
    pub fn rdtf(&self) -> RDTF_R {
        RDTF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LRC Check OK Flag"]
    #[inline(always)]
    pub fn lrc_ok(&self) -> LRC_OK_R {
        LRC_OK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC Check OK Flag"]
    #[inline(always)]
    pub fn crc_ok(&self) -> CRC_OK_R {
        CRC_OK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Character Wait Time Error Flag"]
    #[inline(always)]
    pub fn cwt_err(&self) -> CWT_ERR_R {
        CWT_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Received NACK Threshold Error Flag"]
    #[inline(always)]
    pub fn rte(&self) -> RTE_R {
        RTE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Block Wait Time Error Flag"]
    #[inline(always)]
    pub fn bwt_err(&self) -> BWT_ERR_R {
        BWT_ERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Block Guard Time Error Flag"]
    #[inline(always)]
    pub fn bgt_err(&self) -> BGT_ERR_R {
        BGT_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Parity Error Flag"]
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Frame Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Receive FIFO Write Pointer Value"]
    #[inline(always)]
    pub fn rx_wptr(&self) -> RX_WPTR_R {
        RX_WPTR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Receive FIFO Byte Count"]
    #[inline(always)]
    pub fn rx_cnt(&self) -> RX_CNT_R {
        RX_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfo(&mut self) -> RFO_W {
        RFO_W { w: self }
    }
    #[doc = "Bit 4 - Receive Data Interrupt Flag"]
    #[inline(always)]
    pub fn rx_data(&mut self) -> RX_DATA_W {
        RX_DATA_W { w: self }
    }
    #[doc = "Bit 8 - Character Wait Time Error Flag"]
    #[inline(always)]
    pub fn cwt_err(&mut self) -> CWT_ERR_W {
        CWT_ERR_W { w: self }
    }
    #[doc = "Bit 9 - Received NACK Threshold Error Flag"]
    #[inline(always)]
    pub fn rte(&mut self) -> RTE_W {
        RTE_W { w: self }
    }
    #[doc = "Bit 10 - Block Wait Time Error Flag"]
    #[inline(always)]
    pub fn bwt_err(&mut self) -> BWT_ERR_W {
        BWT_ERR_W { w: self }
    }
    #[doc = "Bit 11 - Block Guard Time Error Flag"]
    #[inline(always)]
    pub fn bgt_err(&mut self) -> BGT_ERR_W {
        BGT_ERR_W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Flag"]
    #[inline(always)]
    pub fn pef(&mut self) -> PEF_W {
        PEF_W { w: self }
    }
    #[doc = "Bit 13 - Frame Error Flag"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_status](index.html) module"]
pub struct RX_STATUS_SPEC;
impl crate::RegisterSpec for RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_status::R](R) reader structure"]
impl crate::Readable for RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_status::W](W) writer structure"]
impl crate::Writable for RX_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_STATUS to value 0"]
impl crate::Resettable for RX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
