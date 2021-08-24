#[doc = "Register `SOPT7` reader"]
pub struct R(crate::R<SOPT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT7` writer"]
pub struct W(crate::W<SOPT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT7_SPEC>;
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
impl From<crate::W<SOPT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC0 trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC0TRGSEL_A {
    #[doc = "0: External trigger pin input (EXTRG)"]
    _0000 = 0,
    #[doc = "1: High speed comparator 0 output"]
    _0001 = 1,
    #[doc = "4: PIT trigger 0"]
    _0100 = 4,
    #[doc = "5: PIT trigger 1"]
    _0101 = 5,
    #[doc = "6: PIT trigger 2"]
    _0110 = 6,
    #[doc = "7: PIT trigger 3"]
    _0111 = 7,
    #[doc = "8: TPM0 trigger"]
    _1000 = 8,
    #[doc = "9: TPM1 trigger"]
    _1001 = 9,
    #[doc = "10: TPM2 trigger"]
    _1010 = 10,
    #[doc = "11: Low-power timer1 (LPTMR1) trigger"]
    _1011 = 11,
    #[doc = "12: RTC alarm"]
    _1100 = 12,
    #[doc = "13: RTC seconds"]
    _1101 = 13,
    #[doc = "14: Low-power timer (LPTMR) trigger"]
    _1110 = 14,
    #[doc = "15: TPM1 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    _1111 = 15,
}
impl From<ADC0TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0TRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC0TRGSEL` reader - ADC0 trigger select"]
pub struct ADC0TRGSEL_R(crate::FieldReader<u8, ADC0TRGSEL_A>);
impl ADC0TRGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC0TRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC0TRGSEL_A> {
        match self.bits {
            0 => Some(ADC0TRGSEL_A::_0000),
            1 => Some(ADC0TRGSEL_A::_0001),
            4 => Some(ADC0TRGSEL_A::_0100),
            5 => Some(ADC0TRGSEL_A::_0101),
            6 => Some(ADC0TRGSEL_A::_0110),
            7 => Some(ADC0TRGSEL_A::_0111),
            8 => Some(ADC0TRGSEL_A::_1000),
            9 => Some(ADC0TRGSEL_A::_1001),
            10 => Some(ADC0TRGSEL_A::_1010),
            11 => Some(ADC0TRGSEL_A::_1011),
            12 => Some(ADC0TRGSEL_A::_1100),
            13 => Some(ADC0TRGSEL_A::_1101),
            14 => Some(ADC0TRGSEL_A::_1110),
            15 => Some(ADC0TRGSEL_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == ADC0TRGSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == ADC0TRGSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == ADC0TRGSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == ADC0TRGSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == ADC0TRGSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == ADC0TRGSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == ADC0TRGSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == ADC0TRGSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == ADC0TRGSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == ADC0TRGSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == ADC0TRGSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == ADC0TRGSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == ADC0TRGSEL_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == ADC0TRGSEL_A::_1111
    }
}
impl core::ops::Deref for ADC0TRGSEL_R {
    type Target = crate::FieldReader<u8, ADC0TRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0TRGSEL` writer - ADC0 trigger select"]
pub struct ADC0TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0TRGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External trigger pin input (EXTRG)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0000)
    }
    #[doc = "High speed comparator 0 output"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0001)
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0101)
    }
    #[doc = "PIT trigger 2"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0110)
    }
    #[doc = "PIT trigger 3"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0111)
    }
    #[doc = "TPM0 trigger"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1000)
    }
    #[doc = "TPM1 trigger"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1001)
    }
    #[doc = "TPM2 trigger"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1010)
    }
    #[doc = "Low-power timer1 (LPTMR1) trigger"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1011)
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1100)
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1101)
    }
    #[doc = "Low-power timer (LPTMR) trigger"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1110)
    }
    #[doc = "TPM1 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "ADC0 pretrigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0PRETRGSEL_A {
    #[doc = "0: Pre-trigger A"]
    _0 = 0,
    #[doc = "1: Pre-trigger B"]
    _1 = 1,
}
impl From<ADC0PRETRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0PRETRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0PRETRGSEL` reader - ADC0 pretrigger select"]
pub struct ADC0PRETRGSEL_R(crate::FieldReader<bool, ADC0PRETRGSEL_A>);
impl ADC0PRETRGSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC0PRETRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0PRETRGSEL_A {
        match self.bits {
            false => ADC0PRETRGSEL_A::_0,
            true => ADC0PRETRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADC0PRETRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADC0PRETRGSEL_A::_1
    }
}
impl core::ops::Deref for ADC0PRETRGSEL_R {
    type Target = crate::FieldReader<bool, ADC0PRETRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0PRETRGSEL` writer - ADC0 pretrigger select"]
pub struct ADC0PRETRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0PRETRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0PRETRGSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pre-trigger A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::_0)
    }
    #[doc = "Pre-trigger B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    pub fn adc0trgsel(&self) -> ADC0TRGSEL_R {
        ADC0TRGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&self) -> ADC0PRETRGSEL_R {
        ADC0PRETRGSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    pub fn adc0trgsel(&mut self) -> ADC0TRGSEL_W {
        ADC0TRGSEL_W { w: self }
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&mut self) -> ADC0PRETRGSEL_W {
        ADC0PRETRGSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt7](index.html) module"]
pub struct SOPT7_SPEC;
impl crate::RegisterSpec for SOPT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt7::R](R) reader structure"]
impl crate::Readable for SOPT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt7::W](W) writer structure"]
impl crate::Writable for SOPT7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOPT7 to value 0"]
impl crate::Resettable for SOPT7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
