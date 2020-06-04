#[doc = "Reader of register SCGC5"]
pub type R = crate::R<u32, super::SCGC5>;
#[doc = "Writer for register SCGC5"]
pub type W = crate::W<u32, super::SCGC5>;
#[doc = "Register SCGC5 `reset()`'s with value 0x0004_0182"]
impl crate::ResetValue for super::SCGC5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0182
    }
}
#[doc = "LPTMR0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTMR0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<LPTMR0_A> for bool {
    #[inline(always)]
    fn from(variant: LPTMR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTMR0`"]
pub type LPTMR0_R = crate::R<bool, LPTMR0_A>;
impl LPTMR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTMR0_A {
        match self.bits {
            false => LPTMR0_A::_0,
            true => LPTMR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPTMR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPTMR0_A::_1
    }
}
#[doc = "Write proxy for field `LPTMR0`"]
pub struct LPTMR0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPTMR0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPTMR0_A::_1)
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
#[doc = "SECREG Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECREG_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<SECREG_A> for bool {
    #[inline(always)]
    fn from(variant: SECREG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SECREG`"]
pub type SECREG_R = crate::R<bool, SECREG_A>;
impl SECREG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECREG_A {
        match self.bits {
            false => SECREG_A::_0,
            true => SECREG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECREG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECREG_A::_1
    }
}
#[doc = "Write proxy for field `SECREG`"]
pub struct SECREG_W<'a> {
    w: &'a mut W,
}
impl<'a> SECREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECREG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECREG_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECREG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "LPTMR1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTMR1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<LPTMR1_A> for bool {
    #[inline(always)]
    fn from(variant: LPTMR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTMR1`"]
pub type LPTMR1_R = crate::R<bool, LPTMR1_A>;
impl LPTMR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTMR1_A {
        match self.bits {
            false => LPTMR1_A::_0,
            true => LPTMR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPTMR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPTMR1_A::_1
    }
}
#[doc = "Write proxy for field `LPTMR1`"]
pub struct LPTMR1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPTMR1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPTMR1_A::_1)
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
#[doc = "TSI Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSI_A {
    #[doc = "0: Access disabled"]
    _0 = 0,
    #[doc = "1: Access enabled"]
    _1 = 1,
}
impl From<TSI_A> for bool {
    #[inline(always)]
    fn from(variant: TSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSI`"]
pub type TSI_R = crate::R<bool, TSI_A>;
impl TSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSI_A {
        match self.bits {
            false => TSI_A::_0,
            true => TSI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSI_A::_1
    }
}
#[doc = "Write proxy for field `TSI`"]
pub struct TSI_W<'a> {
    w: &'a mut W,
}
impl<'a> TSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSI_A::_0)
    }
    #[doc = "Access enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSI_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "PTA Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PTA_A> for bool {
    #[inline(always)]
    fn from(variant: PTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTA`"]
pub type PTA_R = crate::R<bool, PTA_A>;
impl PTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTA_A {
        match self.bits {
            false => PTA_A::_0,
            true => PTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTA_A::_1
    }
}
#[doc = "Write proxy for field `PTA`"]
pub struct PTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "PTB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTB_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PTB_A> for bool {
    #[inline(always)]
    fn from(variant: PTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTB`"]
pub type PTB_R = crate::R<bool, PTB_A>;
impl PTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTB_A {
        match self.bits {
            false => PTB_A::_0,
            true => PTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTB_A::_1
    }
}
#[doc = "Write proxy for field `PTB`"]
pub struct PTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTB_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "PTC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PTC_A> for bool {
    #[inline(always)]
    fn from(variant: PTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTC`"]
pub type PTC_R = crate::R<bool, PTC_A>;
impl PTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTC_A {
        match self.bits {
            false => PTC_A::_0,
            true => PTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTC_A::_1
    }
}
#[doc = "Write proxy for field `PTC`"]
pub struct PTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTC_A::_1)
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
#[doc = "PTD Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTD_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PTD_A> for bool {
    #[inline(always)]
    fn from(variant: PTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTD`"]
pub type PTD_R = crate::R<bool, PTD_A>;
impl PTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTD_A {
        match self.bits {
            false => PTD_A::_0,
            true => PTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTD_A::_1
    }
}
#[doc = "Write proxy for field `PTD`"]
pub struct PTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTD_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTD_A::_1)
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
#[doc = "PTE Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTE_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PTE_A> for bool {
    #[inline(always)]
    fn from(variant: PTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTE`"]
pub type PTE_R = crate::R<bool, PTE_A>;
impl PTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTE_A {
        match self.bits {
            false => PTE_A::_0,
            true => PTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTE_A::_1
    }
}
#[doc = "Write proxy for field `PTE`"]
pub struct PTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTE_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTE_A::_1)
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
#[doc = "EMVSIM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMVSIM0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<EMVSIM0_A> for bool {
    #[inline(always)]
    fn from(variant: EMVSIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMVSIM0`"]
pub type EMVSIM0_R = crate::R<bool, EMVSIM0_A>;
impl EMVSIM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMVSIM0_A {
        match self.bits {
            false => EMVSIM0_A::_0,
            true => EMVSIM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMVSIM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMVSIM0_A::_1
    }
}
#[doc = "Write proxy for field `EMVSIM0`"]
pub struct EMVSIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMVSIM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMVSIM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMVSIM0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMVSIM0_A::_1)
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
#[doc = "EMVSIM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMVSIM1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<EMVSIM1_A> for bool {
    #[inline(always)]
    fn from(variant: EMVSIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMVSIM1`"]
pub type EMVSIM1_R = crate::R<bool, EMVSIM1_A>;
impl EMVSIM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMVSIM1_A {
        match self.bits {
            false => EMVSIM1_A::_0,
            true => EMVSIM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMVSIM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMVSIM1_A::_1
    }
}
#[doc = "Write proxy for field `EMVSIM1`"]
pub struct EMVSIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMVSIM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMVSIM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMVSIM1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMVSIM1_A::_1)
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
#[doc = "LTC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<LTC_A> for bool {
    #[inline(always)]
    fn from(variant: LTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LTC`"]
pub type LTC_R = crate::R<bool, LTC_A>;
impl LTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTC_A {
        match self.bits {
            false => LTC_A::_0,
            true => LTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LTC_A::_1
    }
}
#[doc = "Write proxy for field `LTC`"]
pub struct LTC_W<'a> {
    w: &'a mut W,
}
impl<'a> LTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LTC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LTC_A::_1)
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
#[doc = "LPUART0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<LPUART0_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART0`"]
pub type LPUART0_R = crate::R<bool, LPUART0_A>;
impl LPUART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART0_A {
        match self.bits {
            false => LPUART0_A::_0,
            true => LPUART0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPUART0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPUART0_A::_1
    }
}
#[doc = "Write proxy for field `LPUART0`"]
pub struct LPUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "LPUART1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<LPUART1_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART1`"]
pub type LPUART1_R = crate::R<bool, LPUART1_A>;
impl LPUART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_A {
        match self.bits {
            false => LPUART1_A::_0,
            true => LPUART1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPUART1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPUART1_A::_1
    }
}
#[doc = "Write proxy for field `LPUART1`"]
pub struct LPUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART1_A::_1)
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
#[doc = "LPUART2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<LPUART2_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART2`"]
pub type LPUART2_R = crate::R<bool, LPUART2_A>;
impl LPUART2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_A {
        match self.bits {
            false => LPUART2_A::_0,
            true => LPUART2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPUART2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPUART2_A::_1
    }
}
#[doc = "Write proxy for field `LPUART2`"]
pub struct LPUART2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "QSPI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPI0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<QSPI0_A> for bool {
    #[inline(always)]
    fn from(variant: QSPI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QSPI0`"]
pub type QSPI0_R = crate::R<bool, QSPI0_A>;
impl QSPI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPI0_A {
        match self.bits {
            false => QSPI0_A::_0,
            true => QSPI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QSPI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QSPI0_A::_1
    }
}
#[doc = "Write proxy for field `QSPI0`"]
pub struct QSPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPI0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QSPI0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QSPI0_A::_1)
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
#[doc = "FLEXIO0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FLEXIO0_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLEXIO0`"]
pub type FLEXIO0_R = crate::R<bool, FLEXIO0_A>;
impl FLEXIO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO0_A {
        match self.bits {
            false => FLEXIO0_A::_0,
            true => FLEXIO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLEXIO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLEXIO0_A::_1
    }
}
#[doc = "Write proxy for field `FLEXIO0`"]
pub struct FLEXIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLEXIO0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLEXIO0_A::_1)
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
    #[doc = "Bit 0 - LPTMR0 Clock Gate Control"]
    #[inline(always)]
    pub fn lptmr0(&self) -> LPTMR0_R {
        LPTMR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - SECREG Clock Gate Control"]
    #[inline(always)]
    pub fn secreg(&self) -> SECREG_R {
        SECREG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPTMR1 Clock Gate Control"]
    #[inline(always)]
    pub fn lptmr1(&self) -> LPTMR1_R {
        LPTMR1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TSI Access Control"]
    #[inline(always)]
    pub fn tsi(&self) -> TSI_R {
        TSI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PTA Clock Gate Control"]
    #[inline(always)]
    pub fn pta(&self) -> PTA_R {
        PTA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PTB Clock Gate Control"]
    #[inline(always)]
    pub fn ptb(&self) -> PTB_R {
        PTB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PTC Clock Gate Control"]
    #[inline(always)]
    pub fn ptc(&self) -> PTC_R {
        PTC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PTD Clock Gate Control"]
    #[inline(always)]
    pub fn ptd(&self) -> PTD_R {
        PTD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PTE Clock Gate Control"]
    #[inline(always)]
    pub fn pte(&self) -> PTE_R {
        PTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EMVSIM0 Clock Gate Control"]
    #[inline(always)]
    pub fn emvsim0(&self) -> EMVSIM0_R {
        EMVSIM0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EMVSIM1 Clock Gate Control"]
    #[inline(always)]
    pub fn emvsim1(&self) -> EMVSIM1_R {
        EMVSIM1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LTC Clock Gate Control"]
    #[inline(always)]
    pub fn ltc(&self) -> LTC_R {
        LTC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPUART0 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart0(&self) -> LPUART0_R {
        LPUART0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPUART1 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart1(&self) -> LPUART1_R {
        LPUART1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LPUART2 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart2(&self) -> LPUART2_R {
        LPUART2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - QSPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn qspi0(&self) -> QSPI0_R {
        QSPI0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 31 - FLEXIO0 Clock Gate Control"]
    #[inline(always)]
    pub fn flexio0(&self) -> FLEXIO0_R {
        FLEXIO0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTMR0 Clock Gate Control"]
    #[inline(always)]
    pub fn lptmr0(&mut self) -> LPTMR0_W {
        LPTMR0_W { w: self }
    }
    #[doc = "Bit 3 - SECREG Clock Gate Control"]
    #[inline(always)]
    pub fn secreg(&mut self) -> SECREG_W {
        SECREG_W { w: self }
    }
    #[doc = "Bit 4 - LPTMR1 Clock Gate Control"]
    #[inline(always)]
    pub fn lptmr1(&mut self) -> LPTMR1_W {
        LPTMR1_W { w: self }
    }
    #[doc = "Bit 5 - TSI Access Control"]
    #[inline(always)]
    pub fn tsi(&mut self) -> TSI_W {
        TSI_W { w: self }
    }
    #[doc = "Bit 9 - PTA Clock Gate Control"]
    #[inline(always)]
    pub fn pta(&mut self) -> PTA_W {
        PTA_W { w: self }
    }
    #[doc = "Bit 10 - PTB Clock Gate Control"]
    #[inline(always)]
    pub fn ptb(&mut self) -> PTB_W {
        PTB_W { w: self }
    }
    #[doc = "Bit 11 - PTC Clock Gate Control"]
    #[inline(always)]
    pub fn ptc(&mut self) -> PTC_W {
        PTC_W { w: self }
    }
    #[doc = "Bit 12 - PTD Clock Gate Control"]
    #[inline(always)]
    pub fn ptd(&mut self) -> PTD_W {
        PTD_W { w: self }
    }
    #[doc = "Bit 13 - PTE Clock Gate Control"]
    #[inline(always)]
    pub fn pte(&mut self) -> PTE_W {
        PTE_W { w: self }
    }
    #[doc = "Bit 14 - EMVSIM0 Clock Gate Control"]
    #[inline(always)]
    pub fn emvsim0(&mut self) -> EMVSIM0_W {
        EMVSIM0_W { w: self }
    }
    #[doc = "Bit 15 - EMVSIM1 Clock Gate Control"]
    #[inline(always)]
    pub fn emvsim1(&mut self) -> EMVSIM1_W {
        EMVSIM1_W { w: self }
    }
    #[doc = "Bit 17 - LTC Clock Gate Control"]
    #[inline(always)]
    pub fn ltc(&mut self) -> LTC_W {
        LTC_W { w: self }
    }
    #[doc = "Bit 20 - LPUART0 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart0(&mut self) -> LPUART0_W {
        LPUART0_W { w: self }
    }
    #[doc = "Bit 21 - LPUART1 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart1(&mut self) -> LPUART1_W {
        LPUART1_W { w: self }
    }
    #[doc = "Bit 22 - LPUART2 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart2(&mut self) -> LPUART2_W {
        LPUART2_W { w: self }
    }
    #[doc = "Bit 26 - QSPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn qspi0(&mut self) -> QSPI0_W {
        QSPI0_W { w: self }
    }
    #[doc = "Bit 31 - FLEXIO0 Clock Gate Control"]
    #[inline(always)]
    pub fn flexio0(&mut self) -> FLEXIO0_W {
        FLEXIO0_W { w: self }
    }
}
