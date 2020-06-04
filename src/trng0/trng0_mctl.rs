#[doc = "Reader of register TRNG0_MCTL"]
pub type R = crate::R<u32, super::TRNG0_MCTL>;
#[doc = "Writer for register TRNG0_MCTL"]
pub type W = crate::W<u32, super::TRNG0_MCTL>;
#[doc = "Register TRNG0_MCTL `reset()`'s with value 0x0001_2001"]
impl crate::ResetValue for super::TRNG0_MCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_2001
    }
}
#[doc = "Sample Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAMP_MODE_A {
    #[doc = "0: use Von Neumann data into both Entropy shifter and Statistical Checker"]
    _00 = 0,
    #[doc = "1: use raw data into both Entropy shifter and Statistical Checker"]
    _01 = 1,
    #[doc = "2: use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    _10 = 2,
}
impl From<SAMP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMP_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAMP_MODE`"]
pub type SAMP_MODE_R = crate::R<u8, SAMP_MODE_A>;
impl SAMP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAMP_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAMP_MODE_A::_00),
            1 => Val(SAMP_MODE_A::_01),
            2 => Val(SAMP_MODE_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SAMP_MODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SAMP_MODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SAMP_MODE_A::_10
    }
}
#[doc = "Write proxy for field `SAMP_MODE`"]
pub struct SAMP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMP_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::_00)
    }
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::_01)
    }
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Oscillator Divide\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSC_DIV_A {
    #[doc = "0: use ring oscillator with no divide"]
    _00 = 0,
    #[doc = "1: use ring oscillator divided-by-2"]
    _01 = 1,
    #[doc = "2: use ring oscillator divided-by-4"]
    _10 = 2,
    #[doc = "3: use ring oscillator divided-by-8"]
    _11 = 3,
}
impl From<OSC_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSC_DIV`"]
pub type OSC_DIV_R = crate::R<u8, OSC_DIV_A>;
impl OSC_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_DIV_A {
        match self.bits {
            0 => OSC_DIV_A::_00,
            1 => OSC_DIV_A::_01,
            2 => OSC_DIV_A::_10,
            3 => OSC_DIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSC_DIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OSC_DIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OSC_DIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OSC_DIV_A::_11
    }
}
#[doc = "Write proxy for field `OSC_DIV`"]
pub struct OSC_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "use ring oscillator with no divide"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC_DIV_A::_00)
    }
    #[doc = "use ring oscillator divided-by-2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OSC_DIV_A::_01)
    }
    #[doc = "use ring oscillator divided-by-4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC_DIV_A::_10)
    }
    #[doc = "use ring oscillator divided-by-8"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC_DIV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `UNUSED`"]
pub type UNUSED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNUSED`"]
pub struct UNUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED_W<'a> {
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
#[doc = "Reader of field `TRNG_ACC`"]
pub type TRNG_ACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG_ACC`"]
pub struct TRNG_ACC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_ACC_W<'a> {
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
#[doc = "Write proxy for field `RST_DEF`"]
pub struct RST_DEF_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_DEF_W<'a> {
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
#[doc = "Reader of field `FOR_SCLK`"]
pub type FOR_SCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOR_SCLK`"]
pub struct FOR_SCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FOR_SCLK_W<'a> {
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
#[doc = "Reader of field `FCT_FAIL`"]
pub type FCT_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCT_VAL`"]
pub type FCT_VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENT_VAL`"]
pub type ENT_VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TST_OUT`"]
pub type TST_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
#[doc = "Reader of field `TSTOP_OK`"]
pub type TSTOP_OK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRGM`"]
pub type PRGM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRGM`"]
pub struct PRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGM_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline(always)]
    pub fn samp_mode(&self) -> SAMP_MODE_R {
        SAMP_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline(always)]
    pub fn osc_div(&self) -> OSC_DIV_R {
        OSC_DIV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - This bit is unused but write-able. Must be left as zero."]
    #[inline(always)]
    pub fn unused(&self) -> UNUSED_R {
        UNUSED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TRNG Access Mode"]
    #[inline(always)]
    pub fn trng_acc(&self) -> TRNG_ACC_R {
        TRNG_ACC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline(always)]
    pub fn for_sclk(&self) -> FOR_SCLK_R {
        FOR_SCLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn fct_fail(&self) -> FCT_FAIL_R {
        FCT_FAIL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
    #[inline(always)]
    pub fn fct_val(&self) -> FCT_VAL_R {
        FCT_VAL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Read only: Entropy Valid"]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Read only: Test point inside ring oscillator."]
    #[inline(always)]
    pub fn tst_out(&self) -> TST_OUT_R {
        TST_OUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TRNG_OK_TO_STOP"]
    #[inline(always)]
    pub fn tstop_ok(&self) -> TSTOP_OK_R {
        TSTOP_OK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline(always)]
    pub fn prgm(&self) -> PRGM_R {
        PRGM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline(always)]
    pub fn samp_mode(&mut self) -> SAMP_MODE_W {
        SAMP_MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline(always)]
    pub fn osc_div(&mut self) -> OSC_DIV_W {
        OSC_DIV_W { w: self }
    }
    #[doc = "Bit 4 - This bit is unused but write-able. Must be left as zero."]
    #[inline(always)]
    pub fn unused(&mut self) -> UNUSED_W {
        UNUSED_W { w: self }
    }
    #[doc = "Bit 5 - TRNG Access Mode"]
    #[inline(always)]
    pub fn trng_acc(&mut self) -> TRNG_ACC_W {
        TRNG_ACC_W { w: self }
    }
    #[doc = "Bit 6 - Reset Defaults"]
    #[inline(always)]
    pub fn rst_def(&mut self) -> RST_DEF_W {
        RST_DEF_W { w: self }
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline(always)]
    pub fn for_sclk(&mut self) -> FOR_SCLK_W {
        FOR_SCLK_W { w: self }
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline(always)]
    pub fn prgm(&mut self) -> PRGM_W {
        PRGM_W { w: self }
    }
}
