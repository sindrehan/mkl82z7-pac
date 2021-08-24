#[doc = "Register `C7` reader"]
pub struct R(crate::R<C7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C7` writer"]
pub struct W(crate::W<C7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C7_SPEC>;
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
impl From<crate::W<C7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MCG OSC Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSCSEL_A {
    #[doc = "0: Selects Oscillator (OSCCLK0)."]
    _00 = 0,
    #[doc = "1: Selects 32 kHz RTC Oscillator."]
    _01 = 1,
    #[doc = "2: Selects Oscillator (OSCCLK1)."]
    _10 = 2,
}
impl From<OSCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSCSEL` reader - MCG OSC Clock Select"]
pub struct OSCSEL_R(crate::FieldReader<u8, OSCSEL_A>);
impl OSCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSCSEL_A> {
        match self.bits {
            0 => Some(OSCSEL_A::_00),
            1 => Some(OSCSEL_A::_01),
            2 => Some(OSCSEL_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == OSCSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == OSCSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == OSCSEL_A::_10
    }
}
impl core::ops::Deref for OSCSEL_R {
    type Target = crate::FieldReader<u8, OSCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCSEL` writer - MCG OSC Clock Select"]
pub struct OSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects Oscillator (OSCCLK0)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSCSEL_A::_00)
    }
    #[doc = "Selects 32 kHz RTC Oscillator."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OSCSEL_A::_01)
    }
    #[doc = "Selects Oscillator (OSCCLK1)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSCSEL_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - MCG OSC Clock Select"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MCG OSC Clock Select"]
    #[inline(always)]
    pub fn oscsel(&mut self) -> OSCSEL_W {
        OSCSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7](index.html) module"]
pub struct C7_SPEC;
impl crate::RegisterSpec for C7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c7::R](R) reader structure"]
impl crate::Readable for C7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c7::W](W) writer structure"]
impl crate::Writable for C7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C7 to value 0"]
impl crate::Resettable for C7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
