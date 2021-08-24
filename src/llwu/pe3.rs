#[doc = "Register `PE3` reader"]
pub struct R(crate::R<PE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE3` writer"]
pub struct W(crate::W<PE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE3_SPEC>;
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
impl From<crate::W<PE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE8_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE8_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE8` reader - Wakeup Pin Enable For LLWU_P8"]
pub struct WUPE8_R(crate::FieldReader<u8, WUPE8_A>);
impl WUPE8_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE8_A {
        match self.bits {
            0 => WUPE8_A::_00,
            1 => WUPE8_A::_01,
            2 => WUPE8_A::_10,
            3 => WUPE8_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE8_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE8_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE8_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE8_A::_11
    }
}
impl core::ops::Deref for WUPE8_R {
    type Target = crate::FieldReader<u8, WUPE8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE8` writer - Wakeup Pin Enable For LLWU_P8"]
pub struct WUPE8_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE8_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE8_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE8_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE8_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE9_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE9_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE9` reader - Wakeup Pin Enable For LLWU_P9"]
pub struct WUPE9_R(crate::FieldReader<u8, WUPE9_A>);
impl WUPE9_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE9_A {
        match self.bits {
            0 => WUPE9_A::_00,
            1 => WUPE9_A::_01,
            2 => WUPE9_A::_10,
            3 => WUPE9_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE9_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE9_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE9_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE9_A::_11
    }
}
impl core::ops::Deref for WUPE9_R {
    type Target = crate::FieldReader<u8, WUPE9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE9` writer - Wakeup Pin Enable For LLWU_P9"]
pub struct WUPE9_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE9_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE9_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE9_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE9_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u8 & 0x03) << 2);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE10_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE10_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE10` reader - Wakeup Pin Enable For LLWU_P10"]
pub struct WUPE10_R(crate::FieldReader<u8, WUPE10_A>);
impl WUPE10_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE10_A {
        match self.bits {
            0 => WUPE10_A::_00,
            1 => WUPE10_A::_01,
            2 => WUPE10_A::_10,
            3 => WUPE10_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE10_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE10_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE10_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE10_A::_11
    }
}
impl core::ops::Deref for WUPE10_R {
    type Target = crate::FieldReader<u8, WUPE10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE10` writer - Wakeup Pin Enable For LLWU_P10"]
pub struct WUPE10_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE10_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE10_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE10_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE10_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE11_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE11_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE11` reader - Wakeup Pin Enable For LLWU_P11"]
pub struct WUPE11_R(crate::FieldReader<u8, WUPE11_A>);
impl WUPE11_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE11_A {
        match self.bits {
            0 => WUPE11_A::_00,
            1 => WUPE11_A::_01,
            2 => WUPE11_A::_10,
            3 => WUPE11_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE11_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE11_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE11_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE11_A::_11
    }
}
impl core::ops::Deref for WUPE11_R {
    type Target = crate::FieldReader<u8, WUPE11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE11` writer - Wakeup Pin Enable For LLWU_P11"]
pub struct WUPE11_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE11_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE11_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE11_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE11_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P8"]
    #[inline(always)]
    pub fn wupe8(&self) -> WUPE8_R {
        WUPE8_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P9"]
    #[inline(always)]
    pub fn wupe9(&self) -> WUPE9_R {
        WUPE9_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P10"]
    #[inline(always)]
    pub fn wupe10(&self) -> WUPE10_R {
        WUPE10_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P11"]
    #[inline(always)]
    pub fn wupe11(&self) -> WUPE11_R {
        WUPE11_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P8"]
    #[inline(always)]
    pub fn wupe8(&mut self) -> WUPE8_W {
        WUPE8_W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P9"]
    #[inline(always)]
    pub fn wupe9(&mut self) -> WUPE9_W {
        WUPE9_W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P10"]
    #[inline(always)]
    pub fn wupe10(&mut self) -> WUPE10_W {
        WUPE10_W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P11"]
    #[inline(always)]
    pub fn wupe11(&mut self) -> WUPE11_W {
        WUPE11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Enable 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe3](index.html) module"]
pub struct PE3_SPEC;
impl crate::RegisterSpec for PE3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pe3::R](R) reader structure"]
impl crate::Readable for PE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe3::W](W) writer structure"]
impl crate::Writable for PE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE3 to value 0"]
impl crate::Resettable for PE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
