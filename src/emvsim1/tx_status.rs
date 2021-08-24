#[doc = "Register `TX_STATUS` reader"]
pub struct R(crate::R<TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_STATUS` writer"]
pub struct W(crate::W<TX_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_STATUS_SPEC>;
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
impl From<crate::W<TX_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit NACK Threshold Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNTE_A {
    #[doc = "0: Transmit NACK threshold has not been reached (default)"]
    _0 = 0,
    #[doc = "1: Transmit NACK threshold reached; transmitter frozen"]
    _1 = 1,
}
impl From<TNTE_A> for bool {
    #[inline(always)]
    fn from(variant: TNTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNTE` reader - Transmit NACK Threshold Error Flag"]
pub struct TNTE_R(crate::FieldReader<bool, TNTE_A>);
impl TNTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TNTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNTE_A {
        match self.bits {
            false => TNTE_A::_0,
            true => TNTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TNTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TNTE_A::_1
    }
}
impl core::ops::Deref for TNTE_R {
    type Target = crate::FieldReader<bool, TNTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TNTE` writer - Transmit NACK Threshold Error Flag"]
pub struct TNTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TNTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TNTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit NACK threshold has not been reached (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TNTE_A::_0)
    }
    #[doc = "Transmit NACK threshold reached; transmitter frozen"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TNTE_A::_1)
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
#[doc = "Transmit FIFO Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_A {
    #[doc = "0: Transmit FIFO is not empty"]
    _0 = 0,
    #[doc = "1: Transmit FIFO is empty (default)"]
    _1 = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty Flag"]
pub struct TFE_R(crate::FieldReader<bool, TFE_A>);
impl TFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::_0,
            true => TFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFE_A::_1
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, TFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFE` writer - Transmit FIFO Empty Flag"]
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit FIFO is not empty"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFE_A::_0)
    }
    #[doc = "Transmit FIFO is empty (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Early Transmit Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETCF_A {
    #[doc = "0: Transmit pending or in progress"]
    _0 = 0,
    #[doc = "1: Transmit complete (default)"]
    _1 = 1,
}
impl From<ETCF_A> for bool {
    #[inline(always)]
    fn from(variant: ETCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETCF` reader - Early Transmit Complete Flag"]
pub struct ETCF_R(crate::FieldReader<bool, ETCF_A>);
impl ETCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETCF_A {
        match self.bits {
            false => ETCF_A::_0,
            true => ETCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ETCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ETCF_A::_1
    }
}
impl core::ops::Deref for ETCF_R {
    type Target = crate::FieldReader<bool, ETCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETCF` writer - Early Transmit Complete Flag"]
pub struct ETCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ETCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit pending or in progress"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETCF_A::_0)
    }
    #[doc = "Transmit complete (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETCF_A::_1)
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
#[doc = "Transmit Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    #[doc = "0: Transmit pending or in progress"]
    _0 = 0,
    #[doc = "1: Transmit complete (default)"]
    _1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Transmit Complete Flag"]
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
#[doc = "Field `TCF` writer - Transmit Complete Flag"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit pending or in progress"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_A::_0)
    }
    #[doc = "Transmit complete (default)"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Transmit FIFO Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFF_A {
    #[doc = "0: Transmit FIFO Full condition has not occurred (default)"]
    _0 = 0,
    #[doc = "1: A Transmit FIFO Full condition has occurred"]
    _1 = 1,
}
impl From<TFF_A> for bool {
    #[inline(always)]
    fn from(variant: TFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFF` reader - Transmit FIFO Full Flag"]
pub struct TFF_R(crate::FieldReader<bool, TFF_A>);
impl TFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFF_A {
        match self.bits {
            false => TFF_A::_0,
            true => TFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFF_A::_1
    }
}
impl core::ops::Deref for TFF_R {
    type Target = crate::FieldReader<bool, TFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFF` writer - Transmit FIFO Full Flag"]
pub struct TFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit FIFO Full condition has not occurred (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFF_A::_0)
    }
    #[doc = "A Transmit FIFO Full condition has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Transmit Data Threshold Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDTF_A {
    #[doc = "0: Number of bytes in FIFO is greater than TDT\\[3:0\\], or bit has been cleared"]
    _0 = 0,
    #[doc = "1: Number of bytes in FIFO is less than or equal to TDT\\[3:0\\]
(default)"]
    _1 = 1,
}
impl From<TDTF_A> for bool {
    #[inline(always)]
    fn from(variant: TDTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDTF` reader - Transmit Data Threshold Flag"]
pub struct TDTF_R(crate::FieldReader<bool, TDTF_A>);
impl TDTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDTF_A {
        match self.bits {
            false => TDTF_A::_0,
            true => TDTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDTF_A::_1
    }
}
impl core::ops::Deref for TDTF_R {
    type Target = crate::FieldReader<bool, TDTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "General Purpose Counter 0 Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT0_TO_A {
    #[doc = "0: GPCNT0_VAL time not reached, or bit has been cleared. (default)"]
    _0 = 0,
    #[doc = "1: General Purpose counter has reached the GPCNT0_VAL value"]
    _1 = 1,
}
impl From<GPCNT0_TO_A> for bool {
    #[inline(always)]
    fn from(variant: GPCNT0_TO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPCNT0_TO` reader - General Purpose Counter 0 Timeout Flag"]
pub struct GPCNT0_TO_R(crate::FieldReader<bool, GPCNT0_TO_A>);
impl GPCNT0_TO_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPCNT0_TO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCNT0_TO_A {
        match self.bits {
            false => GPCNT0_TO_A::_0,
            true => GPCNT0_TO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GPCNT0_TO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GPCNT0_TO_A::_1
    }
}
impl core::ops::Deref for GPCNT0_TO_R {
    type Target = crate::FieldReader<bool, GPCNT0_TO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPCNT0_TO` writer - General Purpose Counter 0 Timeout Flag"]
pub struct GPCNT0_TO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCNT0_TO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCNT0_TO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "GPCNT0_VAL time not reached, or bit has been cleared. (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPCNT0_TO_A::_0)
    }
    #[doc = "General Purpose counter has reached the GPCNT0_VAL value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPCNT0_TO_A::_1)
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
#[doc = "General Purpose Counter 1 Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT1_TO_A {
    #[doc = "0: GPCNT1_VAL time not reached, or bit has been cleared. (default)"]
    _0 = 0,
    #[doc = "1: General Purpose counter has reached the GPCNT1_VAL value"]
    _1 = 1,
}
impl From<GPCNT1_TO_A> for bool {
    #[inline(always)]
    fn from(variant: GPCNT1_TO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPCNT1_TO` reader - General Purpose Counter 1 Timeout Flag"]
pub struct GPCNT1_TO_R(crate::FieldReader<bool, GPCNT1_TO_A>);
impl GPCNT1_TO_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPCNT1_TO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCNT1_TO_A {
        match self.bits {
            false => GPCNT1_TO_A::_0,
            true => GPCNT1_TO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GPCNT1_TO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GPCNT1_TO_A::_1
    }
}
impl core::ops::Deref for GPCNT1_TO_R {
    type Target = crate::FieldReader<bool, GPCNT1_TO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPCNT1_TO` writer - General Purpose Counter 1 Timeout Flag"]
pub struct GPCNT1_TO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCNT1_TO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCNT1_TO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "GPCNT1_VAL time not reached, or bit has been cleared. (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPCNT1_TO_A::_0)
    }
    #[doc = "General Purpose counter has reached the GPCNT1_VAL value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPCNT1_TO_A::_1)
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
#[doc = "Field `TX_RPTR` reader - Transmit FIFO Read Pointer"]
pub struct TX_RPTR_R(crate::FieldReader<u8, u8>);
impl TX_RPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_RPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Byte Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_CNT_A {
    #[doc = "0: FIFO is emtpy"]
    _0 = 0,
}
impl From<TX_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TX_CNT` reader - Transmit FIFO Byte Count"]
pub struct TX_CNT_R(crate::FieldReader<u8, TX_CNT_A>);
impl TX_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_CNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_CNT_A> {
        match self.bits {
            0 => Some(TX_CNT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TX_CNT_A::_0
    }
}
impl core::ops::Deref for TX_CNT_R {
    type Target = crate::FieldReader<u8, TX_CNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit NACK Threshold Error Flag"]
    #[inline(always)]
    pub fn tnte(&self) -> TNTE_R {
        TNTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO Empty Flag"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Early Transmit Complete Flag"]
    #[inline(always)]
    pub fn etcf(&self) -> ETCF_R {
        ETCF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Full Flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Threshold Flag"]
    #[inline(always)]
    pub fn tdtf(&self) -> TDTF_R {
        TDTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - General Purpose Counter 0 Timeout Flag"]
    #[inline(always)]
    pub fn gpcnt0_to(&self) -> GPCNT0_TO_R {
        GPCNT0_TO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - General Purpose Counter 1 Timeout Flag"]
    #[inline(always)]
    pub fn gpcnt1_to(&self) -> GPCNT1_TO_R {
        GPCNT1_TO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Transmit FIFO Read Pointer"]
    #[inline(always)]
    pub fn tx_rptr(&self) -> TX_RPTR_R {
        TX_RPTR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Transmit FIFO Byte Count"]
    #[inline(always)]
    pub fn tx_cnt(&self) -> TX_CNT_R {
        TX_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit NACK Threshold Error Flag"]
    #[inline(always)]
    pub fn tnte(&mut self) -> TNTE_W {
        TNTE_W { w: self }
    }
    #[doc = "Bit 3 - Transmit FIFO Empty Flag"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
    #[doc = "Bit 4 - Early Transmit Complete Flag"]
    #[inline(always)]
    pub fn etcf(&mut self) -> ETCF_W {
        ETCF_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Complete Flag"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    #[doc = "Bit 6 - Transmit FIFO Full Flag"]
    #[inline(always)]
    pub fn tff(&mut self) -> TFF_W {
        TFF_W { w: self }
    }
    #[doc = "Bit 8 - General Purpose Counter 0 Timeout Flag"]
    #[inline(always)]
    pub fn gpcnt0_to(&mut self) -> GPCNT0_TO_W {
        GPCNT0_TO_W { w: self }
    }
    #[doc = "Bit 9 - General Purpose Counter 1 Timeout Flag"]
    #[inline(always)]
    pub fn gpcnt1_to(&mut self) -> GPCNT1_TO_W {
        GPCNT1_TO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_status](index.html) module"]
pub struct TX_STATUS_SPEC;
impl crate::RegisterSpec for TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_status::R](R) reader structure"]
impl crate::Readable for TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_status::W](W) writer structure"]
impl crate::Writable for TX_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_STATUS to value 0xb8"]
impl crate::Resettable for TX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb8
    }
}
