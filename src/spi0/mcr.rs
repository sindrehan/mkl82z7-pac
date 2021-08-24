#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Halt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: Start transfers."]
    _0 = 0,
    #[doc = "1: Stop transfers."]
    _1 = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halt"]
pub struct HALT_R(crate::FieldReader<bool, HALT_A>);
impl HALT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::_0,
            true => HALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HALT_A::_1
    }
}
impl core::ops::Deref for HALT_R {
    type Target = crate::FieldReader<bool, HALT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT` writer - Halt"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Stop transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALT_A::_1)
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
#[doc = "Sample Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMPL_PT_A {
    #[doc = "0: 0 protocol clock cycles between SCK edge and SIN sample"]
    _00 = 0,
    #[doc = "1: 1 protocol clock cycle between SCK edge and SIN sample"]
    _01 = 1,
    #[doc = "2: 2 protocol clock cycles between SCK edge and SIN sample"]
    _10 = 2,
}
impl From<SMPL_PT_A> for u8 {
    #[inline(always)]
    fn from(variant: SMPL_PT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMPL_PT` reader - Sample Point"]
pub struct SMPL_PT_R(crate::FieldReader<u8, SMPL_PT_A>);
impl SMPL_PT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPL_PT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMPL_PT_A> {
        match self.bits {
            0 => Some(SMPL_PT_A::_00),
            1 => Some(SMPL_PT_A::_01),
            2 => Some(SMPL_PT_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == SMPL_PT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == SMPL_PT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == SMPL_PT_A::_10
    }
}
impl core::ops::Deref for SMPL_PT_R {
    type Target = crate::FieldReader<u8, SMPL_PT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPL_PT` writer - Sample Point"]
pub struct SMPL_PT_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_PT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPL_PT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 protocol clock cycles between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_00)
    }
    #[doc = "1 protocol clock cycle between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_01)
    }
    #[doc = "2 protocol clock cycles between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "CLR_RXF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_RXF_AW {
    #[doc = "0: Do not clear the RX FIFO counter."]
    _0 = 0,
    #[doc = "1: Clear the RX FIFO counter."]
    _1 = 1,
}
impl From<CLR_RXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_RXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_RXF` writer - CLR_RXF"]
pub struct CLR_RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_RXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_RXF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not clear the RX FIFO counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_0)
    }
    #[doc = "Clear the RX FIFO counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_1)
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
#[doc = "Clear TX FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_TXF_AW {
    #[doc = "0: Do not clear the TX FIFO counter."]
    _0 = 0,
    #[doc = "1: Clear the TX FIFO counter."]
    _1 = 1,
}
impl From<CLR_TXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_TXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_TXF` writer - Clear TX FIFO"]
pub struct CLR_TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_TXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_TXF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not clear the TX FIFO counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_0)
    }
    #[doc = "Clear the TX FIFO counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_1)
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
#[doc = "Disable Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_RXF_A {
    #[doc = "0: RX FIFO is enabled."]
    _0 = 0,
    #[doc = "1: RX FIFO is disabled."]
    _1 = 1,
}
impl From<DIS_RXF_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_RXF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_RXF` reader - Disable Receive FIFO"]
pub struct DIS_RXF_R(crate::FieldReader<bool, DIS_RXF_A>);
impl DIS_RXF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_RXF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_RXF_A {
        match self.bits {
            false => DIS_RXF_A::_0,
            true => DIS_RXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIS_RXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIS_RXF_A::_1
    }
}
impl core::ops::Deref for DIS_RXF_R {
    type Target = crate::FieldReader<bool, DIS_RXF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_RXF` writer - Disable Receive FIFO"]
pub struct DIS_RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_RXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_RXF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX FIFO is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_RXF_A::_0)
    }
    #[doc = "RX FIFO is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_RXF_A::_1)
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
#[doc = "Disable Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_TXF_A {
    #[doc = "0: TX FIFO is enabled."]
    _0 = 0,
    #[doc = "1: TX FIFO is disabled."]
    _1 = 1,
}
impl From<DIS_TXF_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_TXF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_TXF` reader - Disable Transmit FIFO"]
pub struct DIS_TXF_R(crate::FieldReader<bool, DIS_TXF_A>);
impl DIS_TXF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_TXF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_TXF_A {
        match self.bits {
            false => DIS_TXF_A::_0,
            true => DIS_TXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIS_TXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIS_TXF_A::_1
    }
}
impl core::ops::Deref for DIS_TXF_R {
    type Target = crate::FieldReader<bool, DIS_TXF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_TXF` writer - Disable Transmit FIFO"]
pub struct DIS_TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_TXF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TX FIFO is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_TXF_A::_0)
    }
    #[doc = "TX FIFO is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_TXF_A::_1)
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
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Enables the module clocks."]
    _0 = 0,
    #[doc = "1: Allows external logic to disable the module clocks."]
    _1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub struct MDIS_R(crate::FieldReader<bool, MDIS_A>);
impl MDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::_0,
            true => MDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MDIS_A::_1
    }
}
impl core::ops::Deref for MDIS_R {
    type Target = crate::FieldReader<bool, MDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables the module clocks."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Allows external logic to disable the module clocks."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZE_A {
    #[doc = "0: Doze mode has no effect on the module."]
    _0 = 0,
    #[doc = "1: Doze mode disables the module."]
    _1 = 1,
}
impl From<DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: DOZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZE` reader - Doze Enable"]
pub struct DOZE_R(crate::FieldReader<bool, DOZE_A>);
impl DOZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZE_A {
        match self.bits {
            false => DOZE_A::_0,
            true => DOZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DOZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DOZE_A::_1
    }
}
impl core::ops::Deref for DOZE_R {
    type Target = crate::FieldReader<bool, DOZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZE` writer - Doze Enable"]
pub struct DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Doze mode has no effect on the module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZE_A::_0)
    }
    #[doc = "Doze mode disables the module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCSIS_A {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<PCSIS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCSIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCSIS` reader - Peripheral Chip Select x Inactive State"]
pub struct PCSIS_R(crate::FieldReader<u8, PCSIS_A>);
impl PCSIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCSIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PCSIS_A> {
        match self.bits {
            0 => Some(PCSIS_A::_0),
            1 => Some(PCSIS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PCSIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PCSIS_A::_1
    }
}
impl core::ops::Deref for PCSIS_R {
    type Target = crate::FieldReader<u8, PCSIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSIS` writer - Peripheral Chip Select x Inactive State"]
pub struct PCSIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSIS_A::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSIS_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Receive FIFO Overflow Overwrite Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROOE_A {
    #[doc = "0: Incoming data is ignored."]
    _0 = 0,
    #[doc = "1: Incoming data is shifted into the shift register."]
    _1 = 1,
}
impl From<ROOE_A> for bool {
    #[inline(always)]
    fn from(variant: ROOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROOE` reader - Receive FIFO Overflow Overwrite Enable"]
pub struct ROOE_R(crate::FieldReader<bool, ROOE_A>);
impl ROOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROOE_A {
        match self.bits {
            false => ROOE_A::_0,
            true => ROOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ROOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ROOE_A::_1
    }
}
impl core::ops::Deref for ROOE_R {
    type Target = crate::FieldReader<bool, ROOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROOE` writer - Receive FIFO Overflow Overwrite Enable"]
pub struct ROOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Incoming data is ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROOE_A::_0)
    }
    #[doc = "Incoming data is shifted into the shift register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROOE_A::_1)
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
#[doc = "Peripheral Chip Select Strobe Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSSE_A {
    #[doc = "0: PCS5/ PCSS is used as the Peripheral Chip Select\\[5\\]
signal."]
    _0 = 0,
    #[doc = "1: PCS5/ PCSS is used as an active-low PCS Strobe signal."]
    _1 = 1,
}
impl From<PCSSE_A> for bool {
    #[inline(always)]
    fn from(variant: PCSSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCSSE` reader - Peripheral Chip Select Strobe Enable"]
pub struct PCSSE_R(crate::FieldReader<bool, PCSSE_A>);
impl PCSSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSSE_A {
        match self.bits {
            false => PCSSE_A::_0,
            true => PCSSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PCSSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PCSSE_A::_1
    }
}
impl core::ops::Deref for PCSSE_R {
    type Target = crate::FieldReader<bool, PCSSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSSE` writer - Peripheral Chip Select Strobe Enable"]
pub struct PCSSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PCS5/ PCSS is used as the Peripheral Chip Select\\[5\\]
signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSSE_A::_0)
    }
    #[doc = "PCS5/ PCSS is used as an active-low PCS Strobe signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSSE_A::_1)
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
#[doc = "Modified Transfer Format Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTFE_A {
    #[doc = "0: Modified SPI transfer format disabled."]
    _0 = 0,
    #[doc = "1: Modified SPI transfer format enabled."]
    _1 = 1,
}
impl From<MTFE_A> for bool {
    #[inline(always)]
    fn from(variant: MTFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTFE` reader - Modified Transfer Format Enable"]
pub struct MTFE_R(crate::FieldReader<bool, MTFE_A>);
impl MTFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTFE_A {
        match self.bits {
            false => MTFE_A::_0,
            true => MTFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTFE_A::_1
    }
}
impl core::ops::Deref for MTFE_R {
    type Target = crate::FieldReader<bool, MTFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTFE` writer - Modified Transfer Format Enable"]
pub struct MTFE_W<'a> {
    w: &'a mut W,
}
impl<'a> MTFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Modified SPI transfer format disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTFE_A::_0)
    }
    #[doc = "Modified SPI transfer format enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTFE_A::_1)
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
#[doc = "Freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZ_A {
    #[doc = "0: Do not halt serial transfers in Debug mode."]
    _0 = 0,
    #[doc = "1: Halt serial transfers in Debug mode."]
    _1 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZ` reader - Freeze"]
pub struct FRZ_R(crate::FieldReader<bool, FRZ_A>);
impl FRZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::_0,
            true => FRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRZ_A::_1
    }
}
impl core::ops::Deref for FRZ_R {
    type Target = crate::FieldReader<bool, FRZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ` writer - Freeze"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRZ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not halt serial transfers in Debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZ_A::_0)
    }
    #[doc = "Halt serial transfers in Debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZ_A::_1)
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
#[doc = "SPI Configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCONF_A {
    #[doc = "0: SPI"]
    _00 = 0,
}
impl From<DCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: DCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DCONF` reader - SPI Configuration."]
pub struct DCONF_R(crate::FieldReader<u8, DCONF_A>);
impl DCONF_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCONF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCONF_A> {
        match self.bits {
            0 => Some(DCONF_A::_00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == DCONF_A::_00
    }
}
impl core::ops::Deref for DCONF_R {
    type Target = crate::FieldReader<u8, DCONF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Continuous SCK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_SCKE_A {
    #[doc = "0: Continuous SCK disabled."]
    _0 = 0,
    #[doc = "1: Continuous SCK enabled."]
    _1 = 1,
}
impl From<CONT_SCKE_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_SCKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT_SCKE` reader - Continuous SCK Enable"]
pub struct CONT_SCKE_R(crate::FieldReader<bool, CONT_SCKE_A>);
impl CONT_SCKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_SCKE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_SCKE_A {
        match self.bits {
            false => CONT_SCKE_A::_0,
            true => CONT_SCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CONT_SCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CONT_SCKE_A::_1
    }
}
impl core::ops::Deref for CONT_SCKE_R {
    type Target = crate::FieldReader<bool, CONT_SCKE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT_SCKE` writer - Continuous SCK Enable"]
pub struct CONT_SCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_SCKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_SCKE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Continuous SCK disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_SCKE_A::_0)
    }
    #[doc = "Continuous SCK enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_SCKE_A::_1)
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
#[doc = "Master/Slave Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    #[doc = "0: Enables Slave mode"]
    _0 = 0,
    #[doc = "1: Enables Master mode"]
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode Select"]
pub struct MSTR_R(crate::FieldReader<bool, MSTR_A>);
impl MSTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSTR_A::_1
    }
}
impl core::ops::Deref for MSTR_R {
    type Target = crate::FieldReader<bool, MSTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode Select"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTR_A::_0)
    }
    #[doc = "Enables Master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTR_A::_1)
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
    #[doc = "Bit 0 - Halt"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline(always)]
    pub fn smpl_pt(&self) -> SMPL_PT_R {
        SMPL_PT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline(always)]
    pub fn dis_rxf(&self) -> DIS_RXF_R {
        DIS_RXF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline(always)]
    pub fn dis_txf(&self) -> DIS_TXF_R {
        DIS_TXF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&self) -> DOZE_R {
        DOZE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis(&self) -> PCSIS_R {
        PCSIS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline(always)]
    pub fn rooe(&self) -> ROOE_R {
        ROOE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline(always)]
    pub fn pcsse(&self) -> PCSSE_R {
        PCSSE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Modified Transfer Format Enable"]
    #[inline(always)]
    pub fn mtfe(&self) -> MTFE_R {
        MTFE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - SPI Configuration."]
    #[inline(always)]
    pub fn dconf(&self) -> DCONF_R {
        DCONF_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline(always)]
    pub fn cont_scke(&self) -> CONT_SCKE_R {
        CONT_SCKE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline(always)]
    pub fn smpl_pt(&mut self) -> SMPL_PT_W {
        SMPL_PT_W { w: self }
    }
    #[doc = "Bit 10 - CLR_RXF"]
    #[inline(always)]
    pub fn clr_rxf(&mut self) -> CLR_RXF_W {
        CLR_RXF_W { w: self }
    }
    #[doc = "Bit 11 - Clear TX FIFO"]
    #[inline(always)]
    pub fn clr_txf(&mut self) -> CLR_TXF_W {
        CLR_TXF_W { w: self }
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline(always)]
    pub fn dis_rxf(&mut self) -> DIS_RXF_W {
        DIS_RXF_W { w: self }
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline(always)]
    pub fn dis_txf(&mut self) -> DIS_TXF_W {
        DIS_TXF_W { w: self }
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&mut self) -> DOZE_W {
        DOZE_W { w: self }
    }
    #[doc = "Bits 16:21 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis(&mut self) -> PCSIS_W {
        PCSIS_W { w: self }
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline(always)]
    pub fn rooe(&mut self) -> ROOE_W {
        ROOE_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline(always)]
    pub fn pcsse(&mut self) -> PCSSE_W {
        PCSSE_W { w: self }
    }
    #[doc = "Bit 26 - Modified Transfer Format Enable"]
    #[inline(always)]
    pub fn mtfe(&mut self) -> MTFE_W {
        MTFE_W { w: self }
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline(always)]
    pub fn cont_scke(&mut self) -> CONT_SCKE_W {
        CONT_SCKE_W { w: self }
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0x4001"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4001
    }
}
