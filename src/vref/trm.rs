#[doc = "Register `TRM` reader"]
pub struct R(crate::R<TRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRM` writer"]
pub struct W(crate::W<TRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRM_SPEC>;
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
impl From<crate::W<TRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trim bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIM_A {
    #[doc = "0: Min"]
    _000000 = 0,
    #[doc = "63: Max"]
    _111111 = 63,
}
impl From<TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIM` reader - Trim bits"]
pub struct TRIM_R(crate::FieldReader<u8, TRIM_A>);
impl TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIM_A> {
        match self.bits {
            0 => Some(TRIM_A::_000000),
            63 => Some(TRIM_A::_111111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        **self == TRIM_A::_000000
    }
    #[doc = "Checks if the value of the field is `_111111`"]
    #[inline(always)]
    pub fn is_111111(&self) -> bool {
        **self == TRIM_A::_111111
    }
}
impl core::ops::Deref for TRIM_R {
    type Target = crate::FieldReader<u8, TRIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM` writer - Trim bits"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Min"]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(TRIM_A::_000000)
    }
    #[doc = "Max"]
    #[inline(always)]
    pub fn _111111(self) -> &'a mut W {
        self.variant(TRIM_A::_111111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u8 & 0x3f);
        self.w
    }
}
#[doc = "Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHOPEN_A {
    #[doc = "0: Chop oscillator is disabled."]
    _0 = 0,
    #[doc = "1: Chop oscillator is enabled."]
    _1 = 1,
}
impl From<CHOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHOPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHOPEN` reader - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
pub struct CHOPEN_R(crate::FieldReader<bool, CHOPEN_A>);
impl CHOPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHOPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHOPEN_A {
        match self.bits {
            false => CHOPEN_A::_0,
            true => CHOPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHOPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHOPEN_A::_1
    }
}
impl core::ops::Deref for CHOPEN_R {
    type Target = crate::FieldReader<bool, CHOPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPEN` writer - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
pub struct CHOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHOPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Chop oscillator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHOPEN_A::_0)
    }
    #[doc = "Chop oscillator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHOPEN_A::_1)
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
impl R {
    #[doc = "Bits 0:5 - Trim bits"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline(always)]
    pub fn chopen(&self) -> CHOPEN_R {
        CHOPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trim bits"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Bit 6 - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline(always)]
    pub fn chopen(&mut self) -> CHOPEN_W {
        CHOPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trm](index.html) module"]
pub struct TRM_SPEC;
impl crate::RegisterSpec for TRM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [trm::R](R) reader structure"]
impl crate::Readable for TRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trm::W](W) writer structure"]
impl crate::Writable for TRM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRM to value 0"]
impl crate::Resettable for TRM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
