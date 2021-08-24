#[doc = "Register `TRNG0_MCTL` reader"]
pub struct R(crate::R<TRNG0_MCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_MCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_MCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_MCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNG0_MCTL` writer"]
pub struct W(crate::W<TRNG0_MCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNG0_MCTL_SPEC>;
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
impl From<crate::W<TRNG0_MCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRNG0_MCTL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SAMP_MODE` reader - Sample Mode"]
pub struct SAMP_MODE_R(crate::FieldReader<u8, SAMP_MODE_A>);
impl SAMP_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAMP_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAMP_MODE_A> {
        match self.bits {
            0 => Some(SAMP_MODE_A::_00),
            1 => Some(SAMP_MODE_A::_01),
            2 => Some(SAMP_MODE_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == SAMP_MODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == SAMP_MODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == SAMP_MODE_A::_10
    }
}
impl core::ops::Deref for SAMP_MODE_R {
    type Target = crate::FieldReader<u8, SAMP_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMP_MODE` writer - Sample Mode"]
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
#[doc = "Field `OSC_DIV` reader - Oscillator Divide"]
pub struct OSC_DIV_R(crate::FieldReader<u8, OSC_DIV_A>);
impl OSC_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSC_DIV_R(crate::FieldReader::new(bits))
    }
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
        **self == OSC_DIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == OSC_DIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == OSC_DIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == OSC_DIV_A::_11
    }
}
impl core::ops::Deref for OSC_DIV_R {
    type Target = crate::FieldReader<u8, OSC_DIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC_DIV` writer - Oscillator Divide"]
pub struct OSC_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC_DIV_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `UNUSED` reader - This bit is unused but write-able. Must be left as zero."]
pub struct UNUSED_R(crate::FieldReader<bool, bool>);
impl UNUSED_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNUSED` writer - This bit is unused but write-able. Must be left as zero."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TRNG_ACC` reader - TRNG Access Mode"]
pub struct TRNG_ACC_R(crate::FieldReader<bool, bool>);
impl TRNG_ACC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRNG_ACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNG_ACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNG_ACC` writer - TRNG Access Mode"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RST_DEF` writer - Reset Defaults"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FOR_SCLK` reader - Force System Clock"]
pub struct FOR_SCLK_R(crate::FieldReader<bool, bool>);
impl FOR_SCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOR_SCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FOR_SCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOR_SCLK` writer - Force System Clock"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FCT_FAIL` reader - Read only: Frequency Count Fail"]
pub struct FCT_FAIL_R(crate::FieldReader<bool, bool>);
impl FCT_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCT_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCT_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCT_VAL` reader - Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
pub struct FCT_VAL_R(crate::FieldReader<bool, bool>);
impl FCT_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCT_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCT_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENT_VAL` reader - Read only: Entropy Valid"]
pub struct ENT_VAL_R(crate::FieldReader<bool, bool>);
impl ENT_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENT_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENT_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TST_OUT` reader - Read only: Test point inside ring oscillator."]
pub struct TST_OUT_R(crate::FieldReader<bool, bool>);
impl TST_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TST_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TST_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` reader - Read: Error status"]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` writer - Read: Error status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TSTOP_OK` reader - TRNG_OK_TO_STOP"]
pub struct TSTOP_OK_R(crate::FieldReader<bool, bool>);
impl TSTOP_OK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTOP_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTOP_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRGM` reader - Programming Mode Select"]
pub struct PRGM_R(crate::FieldReader<bool, bool>);
impl PRGM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRGM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRGM` writer - Programming Mode Select"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG0 Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_mctl](index.html) module"]
pub struct TRNG0_MCTL_SPEC;
impl crate::RegisterSpec for TRNG0_MCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_mctl::R](R) reader structure"]
impl crate::Readable for TRNG0_MCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trng0_mctl::W](W) writer structure"]
impl crate::Writable for TRNG0_MCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNG0_MCTL to value 0x0001_2001"]
impl crate::Resettable for TRNG0_MCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_2001
    }
}
