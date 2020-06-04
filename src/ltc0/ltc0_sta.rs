#[doc = "Reader of register LTC0_STA"]
pub type R = crate::R<u32, super::LTC0_STA>;
#[doc = "Writer for register LTC0_STA"]
pub type W = crate::W<u32, super::LTC0_STA>;
#[doc = "Register LTC0_STA `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_STA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AESA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AB_A {
    #[doc = "0: AESA Idle"]
    _0 = 0,
    #[doc = "1: AESA Busy."]
    _1 = 1,
}
impl From<AB_A> for bool {
    #[inline(always)]
    fn from(variant: AB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AB`"]
pub type AB_R = crate::R<bool, AB_A>;
impl AB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AB_A {
        match self.bits {
            false => AB_A::_0,
            true => AB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AB_A::_1
    }
}
#[doc = "DESA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DB_A {
    #[doc = "0: DESA Idle"]
    _0 = 0,
    #[doc = "1: DESA Busy."]
    _1 = 1,
}
impl From<DB_A> for bool {
    #[inline(always)]
    fn from(variant: DB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DB`"]
pub type DB_R = crate::R<bool, DB_A>;
impl DB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DB_A {
        match self.bits {
            false => DB_A::_0,
            true => DB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DB_A::_1
    }
}
#[doc = "PKHA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PB_A {
    #[doc = "0: PKHA Idle"]
    _0 = 0,
    #[doc = "1: PKHA Busy."]
    _1 = 1,
}
impl From<PB_A> for bool {
    #[inline(always)]
    fn from(variant: PB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PB`"]
pub type PB_R = crate::R<bool, PB_A>;
impl PB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB_A {
        match self.bits {
            false => PB_A::_0,
            true => PB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PB_A::_1
    }
}
#[doc = "MDHA Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MB_A {
    #[doc = "0: MDHA Idle"]
    _0 = 0,
    #[doc = "1: MDHA Busy"]
    _1 = 1,
}
impl From<MB_A> for bool {
    #[inline(always)]
    fn from(variant: MB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MB`"]
pub type MB_R = crate::R<bool, MB_A>;
impl MB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MB_A {
        match self.bits {
            false => MB_A::_0,
            true => MB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB_A::_1
    }
}
#[doc = "Reader of field `DI`"]
pub type DI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DI`"]
pub struct DI_W<'a> {
    w: &'a mut W,
}
impl<'a> DI_W<'a> {
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
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EI_A {
    #[doc = "0: Not Error."]
    _0 = 0,
    #[doc = "1: Error Interrupt."]
    _1 = 1,
}
impl From<EI_A> for bool {
    #[inline(always)]
    fn from(variant: EI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EI`"]
pub type EI_R = crate::R<bool, EI_A>;
impl EI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EI_A {
        match self.bits {
            false => EI_A::_0,
            true => EI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EI_A::_1
    }
}
#[doc = "Reader of field `DPARRN`"]
pub type DPARRN_R = crate::R<bool, bool>;
#[doc = "Reader of field `PKP`"]
pub type PKP_R = crate::R<bool, bool>;
#[doc = "Reader of field `PKO`"]
pub type PKO_R = crate::R<bool, bool>;
#[doc = "Reader of field `PKZ`"]
pub type PKZ_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - AESA Busy"]
    #[inline(always)]
    pub fn ab(&self) -> AB_R {
        AB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DESA Busy"]
    #[inline(always)]
    pub fn db(&self) -> DB_R {
        DB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PKHA Busy"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MDHA Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Done Interrupt"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Error Interrupt"]
    #[inline(always)]
    pub fn ei(&self) -> EI_R {
        EI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is asserted after POR and after every 50K blocks processed by AESA to indicate it is advisable for added security to write a new seed to"]
    #[inline(always)]
    pub fn dparrn(&self) -> DPARRN_R {
        DPARRN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Public Key is Prime"]
    #[inline(always)]
    pub fn pkp(&self) -> PKP_R {
        PKP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Public Key Operation is One"]
    #[inline(always)]
    pub fn pko(&self) -> PKO_R {
        PKO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Public Key Operation is Zero"]
    #[inline(always)]
    pub fn pkz(&self) -> PKZ_R {
        PKZ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Done Interrupt"]
    #[inline(always)]
    pub fn di(&mut self) -> DI_W {
        DI_W { w: self }
    }
}
