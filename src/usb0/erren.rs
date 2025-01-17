#[doc = "Register `ERREN` reader"]
pub struct R(crate::R<ERREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERREN` writer"]
pub struct W(crate::W<ERREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERREN_SPEC>;
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
impl From<crate::W<ERREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PIDERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDERREN_A {
    #[doc = "0: Disables the PIDERR interrupt."]
    _0 = 0,
    #[doc = "1: Enters the PIDERR interrupt."]
    _1 = 1,
}
impl From<PIDERREN_A> for bool {
    #[inline(always)]
    fn from(variant: PIDERREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDERREN` reader - PIDERR Interrupt Enable"]
pub struct PIDERREN_R(crate::FieldReader<bool, PIDERREN_A>);
impl PIDERREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIDERREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDERREN_A {
        match self.bits {
            false => PIDERREN_A::_0,
            true => PIDERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIDERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIDERREN_A::_1
    }
}
impl core::ops::Deref for PIDERREN_R {
    type Target = crate::FieldReader<bool, PIDERREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIDERREN` writer - PIDERR Interrupt Enable"]
pub struct PIDERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDERREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIDERREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the PIDERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIDERREN_A::_0)
    }
    #[doc = "Enters the PIDERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIDERREN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "CRC5/EOF Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC5EOFEN_A {
    #[doc = "0: Disables the CRC5/EOF interrupt."]
    _0 = 0,
    #[doc = "1: Enables the CRC5/EOF interrupt."]
    _1 = 1,
}
impl From<CRC5EOFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC5EOFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC5EOFEN` reader - CRC5/EOF Interrupt Enable"]
pub struct CRC5EOFEN_R(crate::FieldReader<bool, CRC5EOFEN_A>);
impl CRC5EOFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC5EOFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC5EOFEN_A {
        match self.bits {
            false => CRC5EOFEN_A::_0,
            true => CRC5EOFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRC5EOFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRC5EOFEN_A::_1
    }
}
impl core::ops::Deref for CRC5EOFEN_R {
    type Target = crate::FieldReader<bool, CRC5EOFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC5EOFEN` writer - CRC5/EOF Interrupt Enable"]
pub struct CRC5EOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC5EOFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC5EOFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the CRC5/EOF interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC5EOFEN_A::_0)
    }
    #[doc = "Enables the CRC5/EOF interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC5EOFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "CRC16 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC16EN_A {
    #[doc = "0: Disables the CRC16 interrupt."]
    _0 = 0,
    #[doc = "1: Enables the CRC16 interrupt."]
    _1 = 1,
}
impl From<CRC16EN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC16EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC16EN` reader - CRC16 Interrupt Enable"]
pub struct CRC16EN_R(crate::FieldReader<bool, CRC16EN_A>);
impl CRC16EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC16EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC16EN_A {
        match self.bits {
            false => CRC16EN_A::_0,
            true => CRC16EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRC16EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRC16EN_A::_1
    }
}
impl core::ops::Deref for CRC16EN_R {
    type Target = crate::FieldReader<bool, CRC16EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC16EN` writer - CRC16 Interrupt Enable"]
pub struct CRC16EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC16EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the CRC16 interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC16EN_A::_0)
    }
    #[doc = "Enables the CRC16 interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC16EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "DFN8 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFN8EN_A {
    #[doc = "0: Disables the DFN8 interrupt."]
    _0 = 0,
    #[doc = "1: Enables the DFN8 interrupt."]
    _1 = 1,
}
impl From<DFN8EN_A> for bool {
    #[inline(always)]
    fn from(variant: DFN8EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFN8EN` reader - DFN8 Interrupt Enable"]
pub struct DFN8EN_R(crate::FieldReader<bool, DFN8EN_A>);
impl DFN8EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFN8EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFN8EN_A {
        match self.bits {
            false => DFN8EN_A::_0,
            true => DFN8EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFN8EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFN8EN_A::_1
    }
}
impl core::ops::Deref for DFN8EN_R {
    type Target = crate::FieldReader<bool, DFN8EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFN8EN` writer - DFN8 Interrupt Enable"]
pub struct DFN8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFN8EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFN8EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the DFN8 interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFN8EN_A::_0)
    }
    #[doc = "Enables the DFN8 interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFN8EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "BTOERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTOERREN_A {
    #[doc = "0: Disables the BTOERR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the BTOERR interrupt."]
    _1 = 1,
}
impl From<BTOERREN_A> for bool {
    #[inline(always)]
    fn from(variant: BTOERREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTOERREN` reader - BTOERR Interrupt Enable"]
pub struct BTOERREN_R(crate::FieldReader<bool, BTOERREN_A>);
impl BTOERREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTOERREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTOERREN_A {
        match self.bits {
            false => BTOERREN_A::_0,
            true => BTOERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BTOERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BTOERREN_A::_1
    }
}
impl core::ops::Deref for BTOERREN_R {
    type Target = crate::FieldReader<bool, BTOERREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTOERREN` writer - BTOERR Interrupt Enable"]
pub struct BTOERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BTOERREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTOERREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the BTOERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BTOERREN_A::_0)
    }
    #[doc = "Enables the BTOERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BTOERREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "DMAERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAERREN_A {
    #[doc = "0: Disables the DMAERR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the DMAERR interrupt."]
    _1 = 1,
}
impl From<DMAERREN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAERREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAERREN` reader - DMAERR Interrupt Enable"]
pub struct DMAERREN_R(crate::FieldReader<bool, DMAERREN_A>);
impl DMAERREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAERREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAERREN_A {
        match self.bits {
            false => DMAERREN_A::_0,
            true => DMAERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMAERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMAERREN_A::_1
    }
}
impl core::ops::Deref for DMAERREN_R {
    type Target = crate::FieldReader<bool, DMAERREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAERREN` writer - DMAERR Interrupt Enable"]
pub struct DMAERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAERREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAERREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the DMAERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAERREN_A::_0)
    }
    #[doc = "Enables the DMAERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAERREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "OWNERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWNERREN_A {
    #[doc = "0: Disables the OWNERR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the OWNERR interrupt."]
    _1 = 1,
}
impl From<OWNERREN_A> for bool {
    #[inline(always)]
    fn from(variant: OWNERREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWNERREN` reader - OWNERR Interrupt Enable"]
pub struct OWNERREN_R(crate::FieldReader<bool, OWNERREN_A>);
impl OWNERREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OWNERREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWNERREN_A {
        match self.bits {
            false => OWNERREN_A::_0,
            true => OWNERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OWNERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OWNERREN_A::_1
    }
}
impl core::ops::Deref for OWNERREN_R {
    type Target = crate::FieldReader<bool, OWNERREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OWNERREN` writer - OWNERR Interrupt Enable"]
pub struct OWNERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> OWNERREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWNERREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the OWNERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OWNERREN_A::_0)
    }
    #[doc = "Enables the OWNERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OWNERREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "BTSERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTSERREN_A {
    #[doc = "0: Disables the BTSERR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the BTSERR interrupt."]
    _1 = 1,
}
impl From<BTSERREN_A> for bool {
    #[inline(always)]
    fn from(variant: BTSERREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTSERREN` reader - BTSERR Interrupt Enable"]
pub struct BTSERREN_R(crate::FieldReader<bool, BTSERREN_A>);
impl BTSERREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTSERREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTSERREN_A {
        match self.bits {
            false => BTSERREN_A::_0,
            true => BTSERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BTSERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BTSERREN_A::_1
    }
}
impl core::ops::Deref for BTSERREN_R {
    type Target = crate::FieldReader<bool, BTSERREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTSERREN` writer - BTSERR Interrupt Enable"]
pub struct BTSERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BTSERREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTSERREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the BTSERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BTSERREN_A::_0)
    }
    #[doc = "Enables the BTSERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BTSERREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PIDERR Interrupt Enable"]
    #[inline(always)]
    pub fn piderren(&self) -> PIDERREN_R {
        PIDERREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
    #[inline(always)]
    pub fn crc5eofen(&self) -> CRC5EOFEN_R {
        CRC5EOFEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CRC16 Interrupt Enable"]
    #[inline(always)]
    pub fn crc16en(&self) -> CRC16EN_R {
        CRC16EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DFN8 Interrupt Enable"]
    #[inline(always)]
    pub fn dfn8en(&self) -> DFN8EN_R {
        DFN8EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BTOERR Interrupt Enable"]
    #[inline(always)]
    pub fn btoerren(&self) -> BTOERREN_R {
        BTOERREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMAERR Interrupt Enable"]
    #[inline(always)]
    pub fn dmaerren(&self) -> DMAERREN_R {
        DMAERREN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OWNERR Interrupt Enable"]
    #[inline(always)]
    pub fn ownerren(&self) -> OWNERREN_R {
        OWNERREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BTSERR Interrupt Enable"]
    #[inline(always)]
    pub fn btserren(&self) -> BTSERREN_R {
        BTSERREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIDERR Interrupt Enable"]
    #[inline(always)]
    pub fn piderren(&mut self) -> PIDERREN_W {
        PIDERREN_W { w: self }
    }
    #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
    #[inline(always)]
    pub fn crc5eofen(&mut self) -> CRC5EOFEN_W {
        CRC5EOFEN_W { w: self }
    }
    #[doc = "Bit 2 - CRC16 Interrupt Enable"]
    #[inline(always)]
    pub fn crc16en(&mut self) -> CRC16EN_W {
        CRC16EN_W { w: self }
    }
    #[doc = "Bit 3 - DFN8 Interrupt Enable"]
    #[inline(always)]
    pub fn dfn8en(&mut self) -> DFN8EN_W {
        DFN8EN_W { w: self }
    }
    #[doc = "Bit 4 - BTOERR Interrupt Enable"]
    #[inline(always)]
    pub fn btoerren(&mut self) -> BTOERREN_W {
        BTOERREN_W { w: self }
    }
    #[doc = "Bit 5 - DMAERR Interrupt Enable"]
    #[inline(always)]
    pub fn dmaerren(&mut self) -> DMAERREN_W {
        DMAERREN_W { w: self }
    }
    #[doc = "Bit 6 - OWNERR Interrupt Enable"]
    #[inline(always)]
    pub fn ownerren(&mut self) -> OWNERREN_W {
        OWNERREN_W { w: self }
    }
    #[doc = "Bit 7 - BTSERR Interrupt Enable"]
    #[inline(always)]
    pub fn btserren(&mut self) -> BTSERREN_W {
        BTSERREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erren](index.html) module"]
pub struct ERREN_SPEC;
impl crate::RegisterSpec for ERREN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [erren::R](R) reader structure"]
impl crate::Readable for ERREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erren::W](W) writer structure"]
impl crate::Writable for ERREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERREN to value 0"]
impl crate::Resettable for ERREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
