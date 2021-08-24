#[doc = "Register `PE4` reader"]
pub struct R(crate::R<PE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE4` writer"]
pub struct W(crate::W<PE4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE4_SPEC>;
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
impl From<crate::W<PE4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE12_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE12_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE12` reader - Wakeup Pin Enable For LLWU_P12"]
pub struct WUPE12_R(crate::FieldReader<u8, WUPE12_A>);
impl WUPE12_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE12_A {
        match self.bits {
            0 => WUPE12_A::_00,
            1 => WUPE12_A::_01,
            2 => WUPE12_A::_10,
            3 => WUPE12_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE12_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE12_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE12_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE12_A::_11
    }
}
impl core::ops::Deref for WUPE12_R {
    type Target = crate::FieldReader<u8, WUPE12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE12` writer - Wakeup Pin Enable For LLWU_P12"]
pub struct WUPE12_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE12_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE12_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE12_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE12_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE13_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE13_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE13` reader - Wakeup Pin Enable For LLWU_P13"]
pub struct WUPE13_R(crate::FieldReader<u8, WUPE13_A>);
impl WUPE13_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE13_A {
        match self.bits {
            0 => WUPE13_A::_00,
            1 => WUPE13_A::_01,
            2 => WUPE13_A::_10,
            3 => WUPE13_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE13_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE13_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE13_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE13_A::_11
    }
}
impl core::ops::Deref for WUPE13_R {
    type Target = crate::FieldReader<u8, WUPE13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE13` writer - Wakeup Pin Enable For LLWU_P13"]
pub struct WUPE13_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE13_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE13_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE13_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE13_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u8 & 0x03) << 2);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE14_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE14_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE14` reader - Wakeup Pin Enable For LLWU_P14"]
pub struct WUPE14_R(crate::FieldReader<u8, WUPE14_A>);
impl WUPE14_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE14_A {
        match self.bits {
            0 => WUPE14_A::_00,
            1 => WUPE14_A::_01,
            2 => WUPE14_A::_10,
            3 => WUPE14_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE14_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE14_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE14_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE14_A::_11
    }
}
impl core::ops::Deref for WUPE14_R {
    type Target = crate::FieldReader<u8, WUPE14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE14` writer - Wakeup Pin Enable For LLWU_P14"]
pub struct WUPE14_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE14_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE14_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE14_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE14_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE15_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE15_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE15` reader - Wakeup Pin Enable For LLWU_P15"]
pub struct WUPE15_R(crate::FieldReader<u8, WUPE15_A>);
impl WUPE15_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE15_A {
        match self.bits {
            0 => WUPE15_A::_00,
            1 => WUPE15_A::_01,
            2 => WUPE15_A::_10,
            3 => WUPE15_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE15_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE15_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE15_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE15_A::_11
    }
}
impl core::ops::Deref for WUPE15_R {
    type Target = crate::FieldReader<u8, WUPE15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE15` writer - Wakeup Pin Enable For LLWU_P15"]
pub struct WUPE15_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE15_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE15_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE15_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE15_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P12"]
    #[inline(always)]
    pub fn wupe12(&self) -> WUPE12_R {
        WUPE12_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P13"]
    #[inline(always)]
    pub fn wupe13(&self) -> WUPE13_R {
        WUPE13_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P14"]
    #[inline(always)]
    pub fn wupe14(&self) -> WUPE14_R {
        WUPE14_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P15"]
    #[inline(always)]
    pub fn wupe15(&self) -> WUPE15_R {
        WUPE15_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P12"]
    #[inline(always)]
    pub fn wupe12(&mut self) -> WUPE12_W {
        WUPE12_W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P13"]
    #[inline(always)]
    pub fn wupe13(&mut self) -> WUPE13_W {
        WUPE13_W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P14"]
    #[inline(always)]
    pub fn wupe14(&mut self) -> WUPE14_W {
        WUPE14_W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P15"]
    #[inline(always)]
    pub fn wupe15(&mut self) -> WUPE15_W {
        WUPE15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Enable 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe4](index.html) module"]
pub struct PE4_SPEC;
impl crate::RegisterSpec for PE4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pe4::R](R) reader structure"]
impl crate::Readable for PE4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe4::W](W) writer structure"]
impl crate::Writable for PE4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE4 to value 0"]
impl crate::Resettable for PE4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
