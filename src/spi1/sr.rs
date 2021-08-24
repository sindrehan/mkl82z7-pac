#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POPNXTPTR` reader - Pop Next Pointer"]
pub struct POPNXTPTR_R(crate::FieldReader<u8, u8>);
impl POPNXTPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        POPNXTPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POPNXTPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCTR` reader - RX FIFO Counter"]
pub struct RXCTR_R(crate::FieldReader<u8, u8>);
impl RXCTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXCTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXNXTPTR` reader - Transmit Next Pointer"]
pub struct TXNXTPTR_R(crate::FieldReader<u8, u8>);
impl TXNXTPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXNXTPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXNXTPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCTR` reader - TX FIFO Counter"]
pub struct TXCTR_R(crate::FieldReader<u8, u8>);
impl TXCTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXCTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Drain Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDF_A {
    #[doc = "0: RX FIFO is empty."]
    _0 = 0,
    #[doc = "1: RX FIFO is not empty."]
    _1 = 1,
}
impl From<RFDF_A> for bool {
    #[inline(always)]
    fn from(variant: RFDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFDF` reader - Receive FIFO Drain Flag"]
pub struct RFDF_R(crate::FieldReader<bool, RFDF_A>);
impl RFDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDF_A {
        match self.bits {
            false => RFDF_A::_0,
            true => RFDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFDF_A::_1
    }
}
impl core::ops::Deref for RFDF_R {
    type Target = crate::FieldReader<bool, RFDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFDF` writer - Receive FIFO Drain Flag"]
pub struct RFDF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFDF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX FIFO is empty."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_A::_0)
    }
    #[doc = "RX FIFO is not empty."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Receive FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOF_A {
    #[doc = "0: No Rx FIFO overflow."]
    _0 = 0,
    #[doc = "1: Rx FIFO overflow has occurred."]
    _1 = 1,
}
impl From<RFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOF` reader - Receive FIFO Overflow Flag"]
pub struct RFOF_R(crate::FieldReader<bool, RFOF_A>);
impl RFOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOF_A {
        match self.bits {
            false => RFOF_A::_0,
            true => RFOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFOF_A::_1
    }
}
impl core::ops::Deref for RFOF_R {
    type Target = crate::FieldReader<bool, RFOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFOF` writer - Receive FIFO Overflow Flag"]
pub struct RFOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Rx FIFO overflow."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOF_A::_0)
    }
    #[doc = "Rx FIFO overflow has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Transmit FIFO Fill Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFF_A {
    #[doc = "0: TX FIFO is full."]
    _0 = 0,
    #[doc = "1: TX FIFO is not full."]
    _1 = 1,
}
impl From<TFFF_A> for bool {
    #[inline(always)]
    fn from(variant: TFFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFF` reader - Transmit FIFO Fill Flag"]
pub struct TFFF_R(crate::FieldReader<bool, TFFF_A>);
impl TFFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFF_A {
        match self.bits {
            false => TFFF_A::_0,
            true => TFFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFFF_A::_1
    }
}
impl core::ops::Deref for TFFF_R {
    type Target = crate::FieldReader<bool, TFFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFFF` writer - Transmit FIFO Fill Flag"]
pub struct TFFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TX FIFO is full."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_A::_0)
    }
    #[doc = "TX FIFO is not full."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_A::_1)
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
#[doc = "Transmit FIFO Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFUF_A {
    #[doc = "0: No TX FIFO underflow."]
    _0 = 0,
    #[doc = "1: TX FIFO underflow has occurred."]
    _1 = 1,
}
impl From<TFUF_A> for bool {
    #[inline(always)]
    fn from(variant: TFUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFUF` reader - Transmit FIFO Underflow Flag"]
pub struct TFUF_R(crate::FieldReader<bool, TFUF_A>);
impl TFUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFUF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUF_A {
        match self.bits {
            false => TFUF_A::_0,
            true => TFUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFUF_A::_1
    }
}
impl core::ops::Deref for TFUF_R {
    type Target = crate::FieldReader<bool, TFUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFUF` writer - Transmit FIFO Underflow Flag"]
pub struct TFUF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFUF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No TX FIFO underflow."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUF_A::_0)
    }
    #[doc = "TX FIFO underflow has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUF_A::_1)
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
#[doc = "End of Queue Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOQF_A {
    #[doc = "0: EOQ is not set in the executing command."]
    _0 = 0,
    #[doc = "1: EOQ is set in the executing SPI command."]
    _1 = 1,
}
impl From<EOQF_A> for bool {
    #[inline(always)]
    fn from(variant: EOQF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOQF` reader - End of Queue Flag"]
pub struct EOQF_R(crate::FieldReader<bool, EOQF_A>);
impl EOQF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOQF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOQF_A {
        match self.bits {
            false => EOQF_A::_0,
            true => EOQF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EOQF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EOQF_A::_1
    }
}
impl core::ops::Deref for EOQF_R {
    type Target = crate::FieldReader<bool, EOQF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOQF` writer - End of Queue Flag"]
pub struct EOQF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOQF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOQF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EOQ is not set in the executing command."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQF_A::_0)
    }
    #[doc = "EOQ is set in the executing SPI command."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "TX and RX Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRXS_A {
    #[doc = "0: Transmit and receive operations are disabled (The module is in Stopped state)."]
    _0 = 0,
    #[doc = "1: Transmit and receive operations are enabled (The module is in Running state)."]
    _1 = 1,
}
impl From<TXRXS_A> for bool {
    #[inline(always)]
    fn from(variant: TXRXS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRXS` reader - TX and RX Status"]
pub struct TXRXS_R(crate::FieldReader<bool, TXRXS_A>);
impl TXRXS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRXS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRXS_A {
        match self.bits {
            false => TXRXS_A::_0,
            true => TXRXS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRXS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRXS_A::_1
    }
}
impl core::ops::Deref for TXRXS_R {
    type Target = crate::FieldReader<bool, TXRXS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRXS` writer - TX and RX Status"]
pub struct TXRXS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRXS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRXS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit and receive operations are disabled (The module is in Stopped state)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRXS_A::_0)
    }
    #[doc = "Transmit and receive operations are enabled (The module is in Running state)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRXS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Transfer Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    #[doc = "0: Transfer not complete."]
    _0 = 0,
    #[doc = "1: Transfer complete."]
    _1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Transfer Complete Flag"]
pub struct TCF_R(crate::FieldReader<bool, TCF_A>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::_0,
            true => TCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TCF_A::_1
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, TCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCF` writer - Transfer Complete Flag"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transfer not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_A::_0)
    }
    #[doc = "Transfer complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCF_A::_1)
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
    #[doc = "Bits 0:3 - Pop Next Pointer"]
    #[inline(always)]
    pub fn popnxtptr(&self) -> POPNXTPTR_R {
        POPNXTPTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RX FIFO Counter"]
    #[inline(always)]
    pub fn rxctr(&self) -> RXCTR_R {
        RXCTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Next Pointer"]
    #[inline(always)]
    pub fn txnxtptr(&self) -> TXNXTPTR_R {
        TXNXTPTR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TX FIFO Counter"]
    #[inline(always)]
    pub fn txctr(&self) -> TXCTR_R {
        TXCTR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&self) -> RFOF_R {
        RFOF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline(always)]
    pub fn tfff(&self) -> TFFF_R {
        TFFF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&self) -> TFUF_R {
        TFUF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline(always)]
    pub fn eoqf(&self) -> EOQF_R {
        EOQF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline(always)]
    pub fn txrxs(&self) -> TXRXS_R {
        TXRXS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline(always)]
    pub fn rfdf(&mut self) -> RFDF_W {
        RFDF_W { w: self }
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&mut self) -> RFOF_W {
        RFOF_W { w: self }
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline(always)]
    pub fn tfff(&mut self) -> TFFF_W {
        TFFF_W { w: self }
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&mut self) -> TFUF_W {
        TFUF_W { w: self }
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline(always)]
    pub fn eoqf(&mut self) -> EOQF_W {
        EOQF_W { w: self }
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline(always)]
    pub fn txrxs(&mut self) -> TXRXS_W {
        TXRXS_W { w: self }
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x0200_0000"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
