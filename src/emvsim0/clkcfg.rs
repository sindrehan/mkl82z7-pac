#[doc = "Register `CLKCFG` reader"]
pub struct R(crate::R<CLKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCFG` writer"]
pub struct W(crate::W<CLKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCFG_SPEC>;
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
impl From<crate::W<CLKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_PRSC` reader - Clock Prescaler Value"]
pub struct CLK_PRSC_R(crate::FieldReader<u8, u8>);
impl CLK_PRSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_PRSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_PRSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_PRSC` writer - Clock Prescaler Value"]
pub struct CLK_PRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "General Purpose Counter 1 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPCNT1_CLK_SEL_A {
    #[doc = "0: Disabled / Reset (default)"]
    _00 = 0,
    #[doc = "1: Card Clock"]
    _01 = 1,
    #[doc = "2: Receive Clock"]
    _10 = 2,
    #[doc = "3: ETU Clock (transmit clock)"]
    _11 = 3,
}
impl From<GPCNT1_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPCNT1_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPCNT1_CLK_SEL` reader - General Purpose Counter 1 Clock Select"]
pub struct GPCNT1_CLK_SEL_R(crate::FieldReader<u8, GPCNT1_CLK_SEL_A>);
impl GPCNT1_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPCNT1_CLK_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCNT1_CLK_SEL_A {
        match self.bits {
            0 => GPCNT1_CLK_SEL_A::_00,
            1 => GPCNT1_CLK_SEL_A::_01,
            2 => GPCNT1_CLK_SEL_A::_10,
            3 => GPCNT1_CLK_SEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == GPCNT1_CLK_SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == GPCNT1_CLK_SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == GPCNT1_CLK_SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == GPCNT1_CLK_SEL_A::_11
    }
}
impl core::ops::Deref for GPCNT1_CLK_SEL_R {
    type Target = crate::FieldReader<u8, GPCNT1_CLK_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPCNT1_CLK_SEL` writer - General Purpose Counter 1 Clock Select"]
pub struct GPCNT1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCNT1_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCNT1_CLK_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Disabled / Reset (default)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SEL_A::_00)
    }
    #[doc = "Card Clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SEL_A::_01)
    }
    #[doc = "Receive Clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SEL_A::_10)
    }
    #[doc = "ETU Clock (transmit clock)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(GPCNT1_CLK_SEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "General Purpose Counter 0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPCNT0_CLK_SEL_A {
    #[doc = "0: Disabled / Reset (default)"]
    _00 = 0,
    #[doc = "1: Card Clock"]
    _01 = 1,
    #[doc = "2: Receive Clock"]
    _10 = 2,
    #[doc = "3: ETU Clock (transmit clock)"]
    _11 = 3,
}
impl From<GPCNT0_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPCNT0_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPCNT0_CLK_SEL` reader - General Purpose Counter 0 Clock Select"]
pub struct GPCNT0_CLK_SEL_R(crate::FieldReader<u8, GPCNT0_CLK_SEL_A>);
impl GPCNT0_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPCNT0_CLK_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCNT0_CLK_SEL_A {
        match self.bits {
            0 => GPCNT0_CLK_SEL_A::_00,
            1 => GPCNT0_CLK_SEL_A::_01,
            2 => GPCNT0_CLK_SEL_A::_10,
            3 => GPCNT0_CLK_SEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == GPCNT0_CLK_SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == GPCNT0_CLK_SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == GPCNT0_CLK_SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == GPCNT0_CLK_SEL_A::_11
    }
}
impl core::ops::Deref for GPCNT0_CLK_SEL_R {
    type Target = crate::FieldReader<u8, GPCNT0_CLK_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPCNT0_CLK_SEL` writer - General Purpose Counter 0 Clock Select"]
pub struct GPCNT0_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCNT0_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCNT0_CLK_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Disabled / Reset (default)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SEL_A::_00)
    }
    #[doc = "Card Clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SEL_A::_01)
    }
    #[doc = "Receive Clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SEL_A::_10)
    }
    #[doc = "ETU Clock (transmit clock)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(GPCNT0_CLK_SEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Prescaler Value"]
    #[inline(always)]
    pub fn clk_prsc(&self) -> CLK_PRSC_R {
        CLK_PRSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - General Purpose Counter 1 Clock Select"]
    #[inline(always)]
    pub fn gpcnt1_clk_sel(&self) -> GPCNT1_CLK_SEL_R {
        GPCNT1_CLK_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - General Purpose Counter 0 Clock Select"]
    #[inline(always)]
    pub fn gpcnt0_clk_sel(&self) -> GPCNT0_CLK_SEL_R {
        GPCNT0_CLK_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Prescaler Value"]
    #[inline(always)]
    pub fn clk_prsc(&mut self) -> CLK_PRSC_W {
        CLK_PRSC_W { w: self }
    }
    #[doc = "Bits 8:9 - General Purpose Counter 1 Clock Select"]
    #[inline(always)]
    pub fn gpcnt1_clk_sel(&mut self) -> GPCNT1_CLK_SEL_W {
        GPCNT1_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - General Purpose Counter 0 Clock Select"]
    #[inline(always)]
    pub fn gpcnt0_clk_sel(&mut self) -> GPCNT0_CLK_SEL_W {
        GPCNT0_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcfg](index.html) module"]
pub struct CLKCFG_SPEC;
impl crate::RegisterSpec for CLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcfg::R](R) reader structure"]
impl crate::Readable for CLKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcfg::W](W) writer structure"]
impl crate::Writable for CLKCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCFG to value 0"]
impl crate::Resettable for CLKCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
