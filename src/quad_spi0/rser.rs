#[doc = "Reader of register RSER"]
pub type R = crate::R<u32, super::RSER>;
#[doc = "Writer for register RSER"]
pub type W = crate::W<u32, super::RSER>;
#[doc = "Register RSER `reset()`'s with value 0"]
impl crate::ResetValue for super::RSER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transaction Finished Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFIE_A {
    #[doc = "0: No TFF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: TFF interrupt will be generated"]
    _1 = 1,
}
impl From<TFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TFIE`"]
pub type TFIE_R = crate::R<bool, TFIE_A>;
impl TFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFIE_A {
        match self.bits {
            false => TFIE_A::_0,
            true => TFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFIE_A::_1
    }
}
#[doc = "Write proxy for field `TFIE`"]
pub struct TFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No TFF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFIE_A::_0)
    }
    #[doc = "TFF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFIE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "IP Command Trigger during AHB Grant Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGEIE_A {
    #[doc = "0: No IPGEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: IPGEF interrupt will be generated"]
    _1 = 1,
}
impl From<IPGEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IPGEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPGEIE`"]
pub type IPGEIE_R = crate::R<bool, IPGEIE_A>;
impl IPGEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPGEIE_A {
        match self.bits {
            false => IPGEIE_A::_0,
            true => IPGEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPGEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPGEIE_A::_1
    }
}
#[doc = "Write proxy for field `IPGEIE`"]
pub struct IPGEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPGEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No IPGEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPGEIE_A::_0)
    }
    #[doc = "IPGEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPGEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "IP Command Trigger during IP Access Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPIEIE_A {
    #[doc = "0: No IPIEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: IPIEF interrupt will be generated"]
    _1 = 1,
}
impl From<IPIEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IPIEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPIEIE`"]
pub type IPIEIE_R = crate::R<bool, IPIEIE_A>;
impl IPIEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPIEIE_A {
        match self.bits {
            false => IPIEIE_A::_0,
            true => IPIEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPIEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPIEIE_A::_1
    }
}
#[doc = "Write proxy for field `IPIEIE`"]
pub struct IPIEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPIEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPIEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No IPIEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPIEIE_A::_0)
    }
    #[doc = "IPIEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPIEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "IP Command Trigger during AHB Access Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPAEIE_A {
    #[doc = "0: No IPAEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: IPAEF interrupt will be generated"]
    _1 = 1,
}
impl From<IPAEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IPAEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPAEIE`"]
pub type IPAEIE_R = crate::R<bool, IPAEIE_A>;
impl IPAEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPAEIE_A {
        match self.bits {
            false => IPAEIE_A::_0,
            true => IPAEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPAEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPAEIE_A::_1
    }
}
#[doc = "Write proxy for field `IPAEIE`"]
pub struct IPAEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPAEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPAEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No IPAEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPAEIE_A::_0)
    }
    #[doc = "IPAEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPAEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "IP Command Usage Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IUEIE_A {
    #[doc = "0: No IUEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: IUEF interrupt will be generated"]
    _1 = 1,
}
impl From<IUEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IUEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IUEIE`"]
pub type IUEIE_R = crate::R<bool, IUEIE_A>;
impl IUEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IUEIE_A {
        match self.bits {
            false => IUEIE_A::_0,
            true => IUEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IUEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IUEIE_A::_1
    }
}
#[doc = "Write proxy for field `IUEIE`"]
pub struct IUEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IUEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IUEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No IUEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IUEIE_A::_0)
    }
    #[doc = "IUEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IUEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "AHB Buffer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABOIE_A {
    #[doc = "0: No ABOF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: ABOF interrupt will be generated"]
    _1 = 1,
}
impl From<ABOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ABOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABOIE`"]
pub type ABOIE_R = crate::R<bool, ABOIE_A>;
impl ABOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABOIE_A {
        match self.bits {
            false => ABOIE_A::_0,
            true => ABOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABOIE_A::_1
    }
}
#[doc = "Write proxy for field `ABOIE`"]
pub struct ABOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ABOF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABOIE_A::_0)
    }
    #[doc = "ABOF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABOIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "AHB Illegal Burst Size Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIBSIE_A {
    #[doc = "0: No AIBSEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: AIBSEF interrupt will be generated"]
    _1 = 1,
}
impl From<AIBSIE_A> for bool {
    #[inline(always)]
    fn from(variant: AIBSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AIBSIE`"]
pub type AIBSIE_R = crate::R<bool, AIBSIE_A>;
impl AIBSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIBSIE_A {
        match self.bits {
            false => AIBSIE_A::_0,
            true => AIBSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AIBSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AIBSIE_A::_1
    }
}
#[doc = "Write proxy for field `AIBSIE`"]
pub struct AIBSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AIBSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIBSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No AIBSEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIBSIE_A::_0)
    }
    #[doc = "AIBSEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIBSIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "AHB Illegal transaction interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AITIE_A {
    #[doc = "0: No AITEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: AITEF interrupt will be generated"]
    _1 = 1,
}
impl From<AITIE_A> for bool {
    #[inline(always)]
    fn from(variant: AITIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AITIE`"]
pub type AITIE_R = crate::R<bool, AITIE_A>;
impl AITIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AITIE_A {
        match self.bits {
            false => AITIE_A::_0,
            true => AITIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AITIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AITIE_A::_1
    }
}
#[doc = "Write proxy for field `AITIE`"]
pub struct AITIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AITIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AITIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No AITEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AITIE_A::_0)
    }
    #[doc = "AITEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AITIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABSEIE_A {
    #[doc = "0: No ABSEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: ABSEF interrupt will be generated"]
    _1 = 1,
}
impl From<ABSEIE_A> for bool {
    #[inline(always)]
    fn from(variant: ABSEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABSEIE`"]
pub type ABSEIE_R = crate::R<bool, ABSEIE_A>;
impl ABSEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABSEIE_A {
        match self.bits {
            false => ABSEIE_A::_0,
            true => ABSEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABSEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABSEIE_A::_1
    }
}
#[doc = "Write proxy for field `ABSEIE`"]
pub struct ABSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABSEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABSEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ABSEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABSEIE_A::_0)
    }
    #[doc = "ABSEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABSEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBDIE_A {
    #[doc = "0: No RBDF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: RBDF Interrupt will be generated"]
    _1 = 1,
}
impl From<RBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBDIE`"]
pub type RBDIE_R = crate::R<bool, RBDIE_A>;
impl RBDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBDIE_A {
        match self.bits {
            false => RBDIE_A::_0,
            true => RBDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBDIE_A::_1
    }
}
#[doc = "Write proxy for field `RBDIE`"]
pub struct RBDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No RBDF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBDIE_A::_0)
    }
    #[doc = "RBDF Interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBDIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "RX Buffer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBOIE_A {
    #[doc = "0: No RBOF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: RBOF interrupt will be generated"]
    _1 = 1,
}
impl From<RBOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBOIE`"]
pub type RBOIE_R = crate::R<bool, RBOIE_A>;
impl RBOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBOIE_A {
        match self.bits {
            false => RBOIE_A::_0,
            true => RBOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBOIE_A::_1
    }
}
#[doc = "Write proxy for field `RBOIE`"]
pub struct RBOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No RBOF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBOIE_A::_0)
    }
    #[doc = "RBOF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBOIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBDDE_A {
    #[doc = "0: No DMA request will be generated"]
    _0 = 0,
    #[doc = "1: DMA request will be generated"]
    _1 = 1,
}
impl From<RBDDE_A> for bool {
    #[inline(always)]
    fn from(variant: RBDDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBDDE`"]
pub type RBDDE_R = crate::R<bool, RBDDE_A>;
impl RBDDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBDDE_A {
        match self.bits {
            false => RBDDE_A::_0,
            true => RBDDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBDDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBDDE_A::_1
    }
}
#[doc = "Write proxy for field `RBDDE`"]
pub struct RBDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBDDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA request will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBDDE_A::_0)
    }
    #[doc = "DMA request will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBDDE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILLINIE_A {
    #[doc = "0: No ILLINE interrupt will be generated"]
    _0 = 0,
    #[doc = "1: ILLINE interrupt will be generated"]
    _1 = 1,
}
impl From<ILLINIE_A> for bool {
    #[inline(always)]
    fn from(variant: ILLINIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILLINIE`"]
pub type ILLINIE_R = crate::R<bool, ILLINIE_A>;
impl ILLINIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILLINIE_A {
        match self.bits {
            false => ILLINIE_A::_0,
            true => ILLINIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILLINIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILLINIE_A::_1
    }
}
#[doc = "Write proxy for field `ILLINIE`"]
pub struct ILLINIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ILLINIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILLINIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ILLINE interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILLINIE_A::_0)
    }
    #[doc = "ILLINE interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILLINIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "TX Buffer Fill DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBFDE_A {
    #[doc = "0: No DMA request will be generated"]
    _0 = 0,
    #[doc = "1: DMA request will be generated"]
    _1 = 1,
}
impl From<TBFDE_A> for bool {
    #[inline(always)]
    fn from(variant: TBFDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBFDE`"]
pub type TBFDE_R = crate::R<bool, TBFDE_A>;
impl TBFDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBFDE_A {
        match self.bits {
            false => TBFDE_A::_0,
            true => TBFDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TBFDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TBFDE_A::_1
    }
}
#[doc = "Write proxy for field `TBFDE`"]
pub struct TBFDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBFDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBFDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA request will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBFDE_A::_0)
    }
    #[doc = "DMA request will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBFDE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "TX Buffer Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBUIE_A {
    #[doc = "0: No TBUF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: TBUF interrupt will be generated"]
    _1 = 1,
}
impl From<TBUIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBUIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBUIE`"]
pub type TBUIE_R = crate::R<bool, TBUIE_A>;
impl TBUIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBUIE_A {
        match self.bits {
            false => TBUIE_A::_0,
            true => TBUIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TBUIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TBUIE_A::_1
    }
}
#[doc = "Write proxy for field `TBUIE`"]
pub struct TBUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBUIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBUIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No TBUF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBUIE_A::_0)
    }
    #[doc = "TBUF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBUIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "TX Buffer Fill Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBFIE_A {
    #[doc = "0: No TBFF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: TBFF interrupt will be generated"]
    _1 = 1,
}
impl From<TBFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBFIE`"]
pub type TBFIE_R = crate::R<bool, TBFIE_A>;
impl TBFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBFIE_A {
        match self.bits {
            false => TBFIE_A::_0,
            true => TBFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TBFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TBFIE_A::_1
    }
}
#[doc = "Write proxy for field `TBFIE`"]
pub struct TBFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No TBFF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBFIE_A::_0)
    }
    #[doc = "TBFF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBFIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Data Learning Pattern Failure Interrupt enable . Triggered by DLPFF flag in QSPI_FR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLPFIE_A {
    #[doc = "0: No DLPFF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: DLPFF interrupt will be generated"]
    _1 = 1,
}
impl From<DLPFIE_A> for bool {
    #[inline(always)]
    fn from(variant: DLPFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLPFIE`"]
pub type DLPFIE_R = crate::R<bool, DLPFIE_A>;
impl DLPFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLPFIE_A {
        match self.bits {
            false => DLPFIE_A::_0,
            true => DLPFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLPFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLPFIE_A::_1
    }
}
#[doc = "Write proxy for field `DLPFIE`"]
pub struct DLPFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLPFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLPFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DLPFF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLPFIE_A::_0)
    }
    #[doc = "DLPFF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLPFIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transaction Finished Interrupt Enable"]
    #[inline(always)]
    pub fn tfie(&self) -> TFIE_R {
        TFIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - IP Command Trigger during AHB Grant Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipgeie(&self) -> IPGEIE_R {
        IPGEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IP Command Trigger during IP Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipieie(&self) -> IPIEIE_R {
        IPIEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipaeie(&self) -> IPAEIE_R {
        IPAEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IP Command Usage Error Interrupt Enable"]
    #[inline(always)]
    pub fn iueie(&self) -> IUEIE_R {
        IUEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn aboie(&self) -> ABOIE_R {
        ABOIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Interrupt Enable"]
    #[inline(always)]
    pub fn aibsie(&self) -> AIBSIE_R {
        AIBSIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AHB Illegal transaction interrupt enable."]
    #[inline(always)]
    pub fn aitie(&self) -> AITIE_R {
        AITIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR"]
    #[inline(always)]
    pub fn abseie(&self) -> ABSEIE_R {
        ABSEIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain"]
    #[inline(always)]
    pub fn rbdie(&self) -> RBDIE_R {
        RBDIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rboie(&self) -> RBOIE_R {
        RBOIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain"]
    #[inline(always)]
    pub fn rbdde(&self) -> RBDDE_R {
        RBDDE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR"]
    #[inline(always)]
    pub fn illinie(&self) -> ILLINIE_R {
        ILLINIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TX Buffer Fill DMA Enable"]
    #[inline(always)]
    pub fn tbfde(&self) -> TBFDE_R {
        TBFDE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TX Buffer Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TX Buffer Fill Interrupt Enable"]
    #[inline(always)]
    pub fn tbfie(&self) -> TBFIE_R {
        TBFIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Data Learning Pattern Failure Interrupt enable . Triggered by DLPFF flag in QSPI_FR register"]
    #[inline(always)]
    pub fn dlpfie(&self) -> DLPFIE_R {
        DLPFIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transaction Finished Interrupt Enable"]
    #[inline(always)]
    pub fn tfie(&mut self) -> TFIE_W {
        TFIE_W { w: self }
    }
    #[doc = "Bit 4 - IP Command Trigger during AHB Grant Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipgeie(&mut self) -> IPGEIE_W {
        IPGEIE_W { w: self }
    }
    #[doc = "Bit 6 - IP Command Trigger during IP Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipieie(&mut self) -> IPIEIE_W {
        IPIEIE_W { w: self }
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipaeie(&mut self) -> IPAEIE_W {
        IPAEIE_W { w: self }
    }
    #[doc = "Bit 11 - IP Command Usage Error Interrupt Enable"]
    #[inline(always)]
    pub fn iueie(&mut self) -> IUEIE_W {
        IUEIE_W { w: self }
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn aboie(&mut self) -> ABOIE_W {
        ABOIE_W { w: self }
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Interrupt Enable"]
    #[inline(always)]
    pub fn aibsie(&mut self) -> AIBSIE_W {
        AIBSIE_W { w: self }
    }
    #[doc = "Bit 14 - AHB Illegal transaction interrupt enable."]
    #[inline(always)]
    pub fn aitie(&mut self) -> AITIE_W {
        AITIE_W { w: self }
    }
    #[doc = "Bit 15 - AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR"]
    #[inline(always)]
    pub fn abseie(&mut self) -> ABSEIE_W {
        ABSEIE_W { w: self }
    }
    #[doc = "Bit 16 - RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain"]
    #[inline(always)]
    pub fn rbdie(&mut self) -> RBDIE_W {
        RBDIE_W { w: self }
    }
    #[doc = "Bit 17 - RX Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rboie(&mut self) -> RBOIE_W {
        RBOIE_W { w: self }
    }
    #[doc = "Bit 21 - RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain"]
    #[inline(always)]
    pub fn rbdde(&mut self) -> RBDDE_W {
        RBDDE_W { w: self }
    }
    #[doc = "Bit 23 - Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR"]
    #[inline(always)]
    pub fn illinie(&mut self) -> ILLINIE_W {
        ILLINIE_W { w: self }
    }
    #[doc = "Bit 25 - TX Buffer Fill DMA Enable"]
    #[inline(always)]
    pub fn tbfde(&mut self) -> TBFDE_W {
        TBFDE_W { w: self }
    }
    #[doc = "Bit 26 - TX Buffer Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn tbuie(&mut self) -> TBUIE_W {
        TBUIE_W { w: self }
    }
    #[doc = "Bit 27 - TX Buffer Fill Interrupt Enable"]
    #[inline(always)]
    pub fn tbfie(&mut self) -> TBFIE_W {
        TBFIE_W { w: self }
    }
    #[doc = "Bit 31 - Data Learning Pattern Failure Interrupt enable . Triggered by DLPFF flag in QSPI_FR register"]
    #[inline(always)]
    pub fn dlpfie(&mut self) -> DLPFIE_W {
        DLPFIE_W { w: self }
    }
}
