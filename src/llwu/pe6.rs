#[doc = "Register `PE6` reader"]
pub struct R(crate::R<PE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE6` writer"]
pub struct W(crate::W<PE6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE6_SPEC>;
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
impl From<crate::W<PE6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE20_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE20_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE20_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE20` reader - Wakeup Pin Enable For LLWU_P20"]
pub struct WUPE20_R(crate::FieldReader<u8, WUPE20_A>);
impl WUPE20_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE20_A {
        match self.bits {
            0 => WUPE20_A::_00,
            1 => WUPE20_A::_01,
            2 => WUPE20_A::_10,
            3 => WUPE20_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE20_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE20_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE20_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE20_A::_11
    }
}
impl core::ops::Deref for WUPE20_R {
    type Target = crate::FieldReader<u8, WUPE20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE20` writer - Wakeup Pin Enable For LLWU_P20"]
pub struct WUPE20_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE20_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE20_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE20_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE20_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE20_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE21_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE21_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE21_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE21` reader - Wakeup Pin Enable For LLWU_P21"]
pub struct WUPE21_R(crate::FieldReader<u8, WUPE21_A>);
impl WUPE21_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE21_A {
        match self.bits {
            0 => WUPE21_A::_00,
            1 => WUPE21_A::_01,
            2 => WUPE21_A::_10,
            3 => WUPE21_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE21_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE21_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE21_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE21_A::_11
    }
}
impl core::ops::Deref for WUPE21_R {
    type Target = crate::FieldReader<u8, WUPE21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE21` writer - Wakeup Pin Enable For LLWU_P21"]
pub struct WUPE21_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE21_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE21_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE21_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE21_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE21_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u8 & 0x03) << 2);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE22_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE22_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE22_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE22` reader - Wakeup Pin Enable For LLWU_P22"]
pub struct WUPE22_R(crate::FieldReader<u8, WUPE22_A>);
impl WUPE22_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE22_A {
        match self.bits {
            0 => WUPE22_A::_00,
            1 => WUPE22_A::_01,
            2 => WUPE22_A::_10,
            3 => WUPE22_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE22_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE22_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE22_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE22_A::_11
    }
}
impl core::ops::Deref for WUPE22_R {
    type Target = crate::FieldReader<u8, WUPE22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE22` writer - Wakeup Pin Enable For LLWU_P22"]
pub struct WUPE22_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE22_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE22_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE22_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE22_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE22_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE23_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE23_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE23_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE23` reader - Wakeup Pin Enable For LLWU_P23"]
pub struct WUPE23_R(crate::FieldReader<u8, WUPE23_A>);
impl WUPE23_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE23_A {
        match self.bits {
            0 => WUPE23_A::_00,
            1 => WUPE23_A::_01,
            2 => WUPE23_A::_10,
            3 => WUPE23_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE23_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE23_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE23_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE23_A::_11
    }
}
impl core::ops::Deref for WUPE23_R {
    type Target = crate::FieldReader<u8, WUPE23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE23` writer - Wakeup Pin Enable For LLWU_P23"]
pub struct WUPE23_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE23_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE23_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE23_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE23_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE23_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P20"]
    #[inline(always)]
    pub fn wupe20(&self) -> WUPE20_R {
        WUPE20_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P21"]
    #[inline(always)]
    pub fn wupe21(&self) -> WUPE21_R {
        WUPE21_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P22"]
    #[inline(always)]
    pub fn wupe22(&self) -> WUPE22_R {
        WUPE22_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P23"]
    #[inline(always)]
    pub fn wupe23(&self) -> WUPE23_R {
        WUPE23_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P20"]
    #[inline(always)]
    pub fn wupe20(&mut self) -> WUPE20_W {
        WUPE20_W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P21"]
    #[inline(always)]
    pub fn wupe21(&mut self) -> WUPE21_W {
        WUPE21_W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P22"]
    #[inline(always)]
    pub fn wupe22(&mut self) -> WUPE22_W {
        WUPE22_W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P23"]
    #[inline(always)]
    pub fn wupe23(&mut self) -> WUPE23_W {
        WUPE23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Enable 6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe6](index.html) module"]
pub struct PE6_SPEC;
impl crate::RegisterSpec for PE6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pe6::R](R) reader structure"]
impl crate::Readable for PE6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe6::W](W) writer structure"]
impl crate::Writable for PE6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE6 to value 0"]
impl crate::Resettable for PE6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
