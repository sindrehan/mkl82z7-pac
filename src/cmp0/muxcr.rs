#[doc = "Register `MUXCR` reader"]
pub struct R(crate::R<MUXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUXCR` writer"]
pub struct W(crate::W<MUXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUXCR_SPEC>;
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
impl From<crate::W<MUXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Minus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: IN0"]
    _000 = 0,
    #[doc = "1: IN1"]
    _001 = 1,
    #[doc = "2: IN2"]
    _010 = 2,
    #[doc = "3: IN3"]
    _011 = 3,
    #[doc = "4: IN4"]
    _100 = 4,
    #[doc = "5: IN5"]
    _101 = 5,
    #[doc = "6: IN6"]
    _110 = 6,
    #[doc = "7: IN7"]
    _111 = 7,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSEL` reader - Minus Input Mux Control"]
pub struct MSEL_R(crate::FieldReader<u8, MSEL_A>);
impl MSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::_000,
            1 => MSEL_A::_001,
            2 => MSEL_A::_010,
            3 => MSEL_A::_011,
            4 => MSEL_A::_100,
            5 => MSEL_A::_101,
            6 => MSEL_A::_110,
            7 => MSEL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == MSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == MSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == MSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == MSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == MSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == MSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == MSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == MSEL_A::_111
    }
}
impl core::ops::Deref for MSEL_R {
    type Target = crate::FieldReader<u8, MSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSEL` writer - Minus Input Mux Control"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MSEL_A::_000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MSEL_A::_001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MSEL_A::_010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MSEL_A::_011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MSEL_A::_100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MSEL_A::_101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MSEL_A::_110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MSEL_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "Plus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: IN0"]
    _000 = 0,
    #[doc = "1: IN1"]
    _001 = 1,
    #[doc = "2: IN2"]
    _010 = 2,
    #[doc = "3: IN3"]
    _011 = 3,
    #[doc = "4: IN4"]
    _100 = 4,
    #[doc = "5: IN5"]
    _101 = 5,
    #[doc = "6: IN6"]
    _110 = 6,
    #[doc = "7: IN7"]
    _111 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSEL` reader - Plus Input Mux Control"]
pub struct PSEL_R(crate::FieldReader<u8, PSEL_A>);
impl PSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::_000,
            1 => PSEL_A::_001,
            2 => PSEL_A::_010,
            3 => PSEL_A::_011,
            4 => PSEL_A::_100,
            5 => PSEL_A::_101,
            6 => PSEL_A::_110,
            7 => PSEL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == PSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == PSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == PSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == PSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == PSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == PSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == PSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == PSEL_A::_111
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, PSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` writer - Plus Input Mux Control"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSEL_A::_000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSEL_A::_001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSEL_A::_010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSEL_A::_011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSEL_A::_100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSEL_A::_101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSEL_A::_110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSEL_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u8 & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxcr](index.html) module"]
pub struct MUXCR_SPEC;
impl crate::RegisterSpec for MUXCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [muxcr::R](R) reader structure"]
impl crate::Readable for MUXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [muxcr::W](W) writer structure"]
impl crate::Writable for MUXCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUXCR to value 0"]
impl crate::Resettable for MUXCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
