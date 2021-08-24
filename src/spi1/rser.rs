#[doc = "Register `RSER` reader"]
pub struct R(crate::R<RSER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSER` writer"]
pub struct W(crate::W<RSER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSER_SPEC>;
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
impl From<crate::W<RSER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO Drain DMA or Interrupt Request Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDF_DIRS_A {
    #[doc = "0: Interrupt request."]
    _0 = 0,
    #[doc = "1: DMA request."]
    _1 = 1,
}
impl From<RFDF_DIRS_A> for bool {
    #[inline(always)]
    fn from(variant: RFDF_DIRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFDF_DIRS` reader - Receive FIFO Drain DMA or Interrupt Request Select"]
pub struct RFDF_DIRS_R(crate::FieldReader<bool, RFDF_DIRS_A>);
impl RFDF_DIRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFDF_DIRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDF_DIRS_A {
        match self.bits {
            false => RFDF_DIRS_A::_0,
            true => RFDF_DIRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFDF_DIRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFDF_DIRS_A::_1
    }
}
impl core::ops::Deref for RFDF_DIRS_R {
    type Target = crate::FieldReader<bool, RFDF_DIRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFDF_DIRS` writer - Receive FIFO Drain DMA or Interrupt Request Select"]
pub struct RFDF_DIRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RFDF_DIRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFDF_DIRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_DIRS_A::_0)
    }
    #[doc = "DMA request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_DIRS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Receive FIFO Drain Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDF_RE_A {
    #[doc = "0: RFDF interrupt or DMA requests are disabled."]
    _0 = 0,
    #[doc = "1: RFDF interrupt or DMA requests are enabled."]
    _1 = 1,
}
impl From<RFDF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: RFDF_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFDF_RE` reader - Receive FIFO Drain Request Enable"]
pub struct RFDF_RE_R(crate::FieldReader<bool, RFDF_RE_A>);
impl RFDF_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFDF_RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDF_RE_A {
        match self.bits {
            false => RFDF_RE_A::_0,
            true => RFDF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFDF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFDF_RE_A::_1
    }
}
impl core::ops::Deref for RFDF_RE_R {
    type Target = crate::FieldReader<bool, RFDF_RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFDF_RE` writer - Receive FIFO Drain Request Enable"]
pub struct RFDF_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFDF_RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFDF_RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RFDF interrupt or DMA requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_RE_A::_0)
    }
    #[doc = "RFDF interrupt or DMA requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_RE_A::_1)
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
#[doc = "Receive FIFO Overflow Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOF_RE_A {
    #[doc = "0: RFOF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: RFOF interrupt requests are enabled."]
    _1 = 1,
}
impl From<RFOF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: RFOF_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOF_RE` reader - Receive FIFO Overflow Request Enable"]
pub struct RFOF_RE_R(crate::FieldReader<bool, RFOF_RE_A>);
impl RFOF_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFOF_RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOF_RE_A {
        match self.bits {
            false => RFOF_RE_A::_0,
            true => RFOF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RFOF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RFOF_RE_A::_1
    }
}
impl core::ops::Deref for RFOF_RE_R {
    type Target = crate::FieldReader<bool, RFOF_RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFOF_RE` writer - Receive FIFO Overflow Request Enable"]
pub struct RFOF_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOF_RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFOF_RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RFOF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOF_RE_A::_0)
    }
    #[doc = "RFOF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOF_RE_A::_1)
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
#[doc = "Transmit FIFO Fill DMA or Interrupt Request Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFF_DIRS_A {
    #[doc = "0: TFFF flag generates interrupt requests."]
    _0 = 0,
    #[doc = "1: TFFF flag generates DMA requests."]
    _1 = 1,
}
impl From<TFFF_DIRS_A> for bool {
    #[inline(always)]
    fn from(variant: TFFF_DIRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFF_DIRS` reader - Transmit FIFO Fill DMA or Interrupt Request Select"]
pub struct TFFF_DIRS_R(crate::FieldReader<bool, TFFF_DIRS_A>);
impl TFFF_DIRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFFF_DIRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFF_DIRS_A {
        match self.bits {
            false => TFFF_DIRS_A::_0,
            true => TFFF_DIRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFFF_DIRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFFF_DIRS_A::_1
    }
}
impl core::ops::Deref for TFFF_DIRS_R {
    type Target = crate::FieldReader<bool, TFFF_DIRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFFF_DIRS` writer - Transmit FIFO Fill DMA or Interrupt Request Select"]
pub struct TFFF_DIRS_W<'a> {
    w: &'a mut W,
}
impl<'a> TFFF_DIRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFFF_DIRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TFFF flag generates interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_DIRS_A::_0)
    }
    #[doc = "TFFF flag generates DMA requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_DIRS_A::_1)
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
#[doc = "Transmit FIFO Fill Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFF_RE_A {
    #[doc = "0: TFFF interrupts or DMA requests are disabled."]
    _0 = 0,
    #[doc = "1: TFFF interrupts or DMA requests are enabled."]
    _1 = 1,
}
impl From<TFFF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TFFF_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFF_RE` reader - Transmit FIFO Fill Request Enable"]
pub struct TFFF_RE_R(crate::FieldReader<bool, TFFF_RE_A>);
impl TFFF_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFFF_RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFF_RE_A {
        match self.bits {
            false => TFFF_RE_A::_0,
            true => TFFF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFFF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFFF_RE_A::_1
    }
}
impl core::ops::Deref for TFFF_RE_R {
    type Target = crate::FieldReader<bool, TFFF_RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFFF_RE` writer - Transmit FIFO Fill Request Enable"]
pub struct TFFF_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFFF_RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFFF_RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TFFF interrupts or DMA requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_RE_A::_0)
    }
    #[doc = "TFFF interrupts or DMA requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_RE_A::_1)
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
#[doc = "Transmit FIFO Underflow Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFUF_RE_A {
    #[doc = "0: TFUF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: TFUF interrupt requests are enabled."]
    _1 = 1,
}
impl From<TFUF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TFUF_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFUF_RE` reader - Transmit FIFO Underflow Request Enable"]
pub struct TFUF_RE_R(crate::FieldReader<bool, TFUF_RE_A>);
impl TFUF_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFUF_RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUF_RE_A {
        match self.bits {
            false => TFUF_RE_A::_0,
            true => TFUF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFUF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFUF_RE_A::_1
    }
}
impl core::ops::Deref for TFUF_RE_R {
    type Target = crate::FieldReader<bool, TFUF_RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFUF_RE` writer - Transmit FIFO Underflow Request Enable"]
pub struct TFUF_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFUF_RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFUF_RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TFUF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUF_RE_A::_0)
    }
    #[doc = "TFUF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUF_RE_A::_1)
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
#[doc = "Finished Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOQF_RE_A {
    #[doc = "0: EOQF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: EOQF interrupt requests are enabled."]
    _1 = 1,
}
impl From<EOQF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: EOQF_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOQF_RE` reader - Finished Request Enable"]
pub struct EOQF_RE_R(crate::FieldReader<bool, EOQF_RE_A>);
impl EOQF_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOQF_RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOQF_RE_A {
        match self.bits {
            false => EOQF_RE_A::_0,
            true => EOQF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EOQF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EOQF_RE_A::_1
    }
}
impl core::ops::Deref for EOQF_RE_R {
    type Target = crate::FieldReader<bool, EOQF_RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOQF_RE` writer - Finished Request Enable"]
pub struct EOQF_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOQF_RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOQF_RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EOQF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQF_RE_A::_0)
    }
    #[doc = "EOQF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQF_RE_A::_1)
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
#[doc = "Transmission Complete Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_RE_A {
    #[doc = "0: TCF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: TCF interrupt requests are enabled."]
    _1 = 1,
}
impl From<TCF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF_RE` reader - Transmission Complete Request Enable"]
pub struct TCF_RE_R(crate::FieldReader<bool, TCF_RE_A>);
impl TCF_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_RE_A {
        match self.bits {
            false => TCF_RE_A::_0,
            true => TCF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TCF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TCF_RE_A::_1
    }
}
impl core::ops::Deref for TCF_RE_R {
    type Target = crate::FieldReader<bool, TCF_RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCF_RE` writer - Transmission Complete Request Enable"]
pub struct TCF_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCF_RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TCF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_RE_A::_0)
    }
    #[doc = "TCF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCF_RE_A::_1)
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
    #[doc = "Bit 16 - Receive FIFO Drain DMA or Interrupt Request Select"]
    #[inline(always)]
    pub fn rfdf_dirs(&self) -> RFDF_DIRS_R {
        RFDF_DIRS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Request Enable"]
    #[inline(always)]
    pub fn rfdf_re(&self) -> RFDF_RE_R {
        RFDF_RE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Request Enable"]
    #[inline(always)]
    pub fn rfof_re(&self) -> RFOF_RE_R {
        RFOF_RE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline(always)]
    pub fn tfff_dirs(&self) -> TFFF_DIRS_R {
        TFFF_DIRS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Request Enable"]
    #[inline(always)]
    pub fn tfff_re(&self) -> TFFF_RE_R {
        TFFF_RE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Request Enable"]
    #[inline(always)]
    pub fn tfuf_re(&self) -> TFUF_RE_R {
        TFUF_RE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Finished Request Enable"]
    #[inline(always)]
    pub fn eoqf_re(&self) -> EOQF_RE_R {
        EOQF_RE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmission Complete Request Enable"]
    #[inline(always)]
    pub fn tcf_re(&self) -> TCF_RE_R {
        TCF_RE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Receive FIFO Drain DMA or Interrupt Request Select"]
    #[inline(always)]
    pub fn rfdf_dirs(&mut self) -> RFDF_DIRS_W {
        RFDF_DIRS_W { w: self }
    }
    #[doc = "Bit 17 - Receive FIFO Drain Request Enable"]
    #[inline(always)]
    pub fn rfdf_re(&mut self) -> RFDF_RE_W {
        RFDF_RE_W { w: self }
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Request Enable"]
    #[inline(always)]
    pub fn rfof_re(&mut self) -> RFOF_RE_W {
        RFOF_RE_W { w: self }
    }
    #[doc = "Bit 24 - Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline(always)]
    pub fn tfff_dirs(&mut self) -> TFFF_DIRS_W {
        TFFF_DIRS_W { w: self }
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Request Enable"]
    #[inline(always)]
    pub fn tfff_re(&mut self) -> TFFF_RE_W {
        TFFF_RE_W { w: self }
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Request Enable"]
    #[inline(always)]
    pub fn tfuf_re(&mut self) -> TFUF_RE_W {
        TFUF_RE_W { w: self }
    }
    #[doc = "Bit 28 - Finished Request Enable"]
    #[inline(always)]
    pub fn eoqf_re(&mut self) -> EOQF_RE_W {
        EOQF_RE_W { w: self }
    }
    #[doc = "Bit 31 - Transmission Complete Request Enable"]
    #[inline(always)]
    pub fn tcf_re(&mut self) -> TCF_RE_W {
        TCF_RE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA/Interrupt Request Select and Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rser](index.html) module"]
pub struct RSER_SPEC;
impl crate::RegisterSpec for RSER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rser::R](R) reader structure"]
impl crate::Readable for RSER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rser::W](W) writer structure"]
impl crate::Writable for RSER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSER to value 0"]
impl crate::Resettable for RSER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
