#[doc = "Register `PE5` reader"]
pub struct R(crate::R<PE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE5` writer"]
pub struct W(crate::W<PE5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE5_SPEC>;
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
impl From<crate::W<PE5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE16_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE16_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE16_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE16` reader - Wakeup Pin Enable For LLWU_P16"]
pub struct WUPE16_R(crate::FieldReader<u8, WUPE16_A>);
impl WUPE16_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE16_A {
        match self.bits {
            0 => WUPE16_A::_00,
            1 => WUPE16_A::_01,
            2 => WUPE16_A::_10,
            3 => WUPE16_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE16_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE16_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE16_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE16_A::_11
    }
}
impl core::ops::Deref for WUPE16_R {
    type Target = crate::FieldReader<u8, WUPE16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE16` writer - Wakeup Pin Enable For LLWU_P16"]
pub struct WUPE16_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE16_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE16_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE16_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE16_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE16_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE17_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE17_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE17_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE17` reader - Wakeup Pin Enable For LLWU_P17"]
pub struct WUPE17_R(crate::FieldReader<u8, WUPE17_A>);
impl WUPE17_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE17_A {
        match self.bits {
            0 => WUPE17_A::_00,
            1 => WUPE17_A::_01,
            2 => WUPE17_A::_10,
            3 => WUPE17_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE17_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE17_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE17_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE17_A::_11
    }
}
impl core::ops::Deref for WUPE17_R {
    type Target = crate::FieldReader<u8, WUPE17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE17` writer - Wakeup Pin Enable For LLWU_P17"]
pub struct WUPE17_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE17_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE17_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE17_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE17_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE17_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u8 & 0x03) << 2);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE18_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE18_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE18_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE18` reader - Wakeup Pin Enable For LLWU_P18"]
pub struct WUPE18_R(crate::FieldReader<u8, WUPE18_A>);
impl WUPE18_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE18_A {
        match self.bits {
            0 => WUPE18_A::_00,
            1 => WUPE18_A::_01,
            2 => WUPE18_A::_10,
            3 => WUPE18_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE18_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE18_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE18_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE18_A::_11
    }
}
impl core::ops::Deref for WUPE18_R {
    type Target = crate::FieldReader<u8, WUPE18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE18` writer - Wakeup Pin Enable For LLWU_P18"]
pub struct WUPE18_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE18_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE18_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE18_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE18_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE18_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUPE19_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE19_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE19_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUPE19` reader - Wakeup Pin Enable For LLWU_P19"]
pub struct WUPE19_R(crate::FieldReader<u8, WUPE19_A>);
impl WUPE19_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUPE19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE19_A {
        match self.bits {
            0 => WUPE19_A::_00,
            1 => WUPE19_A::_01,
            2 => WUPE19_A::_10,
            3 => WUPE19_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == WUPE19_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == WUPE19_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == WUPE19_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == WUPE19_A::_11
    }
}
impl core::ops::Deref for WUPE19_R {
    type Target = crate::FieldReader<u8, WUPE19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUPE19` writer - Wakeup Pin Enable For LLWU_P19"]
pub struct WUPE19_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE19_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE19_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE19_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE19_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE19_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P16"]
    #[inline(always)]
    pub fn wupe16(&self) -> WUPE16_R {
        WUPE16_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P17"]
    #[inline(always)]
    pub fn wupe17(&self) -> WUPE17_R {
        WUPE17_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P18"]
    #[inline(always)]
    pub fn wupe18(&self) -> WUPE18_R {
        WUPE18_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P19"]
    #[inline(always)]
    pub fn wupe19(&self) -> WUPE19_R {
        WUPE19_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P16"]
    #[inline(always)]
    pub fn wupe16(&mut self) -> WUPE16_W {
        WUPE16_W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P17"]
    #[inline(always)]
    pub fn wupe17(&mut self) -> WUPE17_W {
        WUPE17_W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P18"]
    #[inline(always)]
    pub fn wupe18(&mut self) -> WUPE18_W {
        WUPE18_W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P19"]
    #[inline(always)]
    pub fn wupe19(&mut self) -> WUPE19_W {
        WUPE19_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Enable 5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe5](index.html) module"]
pub struct PE5_SPEC;
impl crate::RegisterSpec for PE5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pe5::R](R) reader structure"]
impl crate::Readable for PE5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe5::W](W) writer structure"]
impl crate::Writable for PE5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE5 to value 0"]
impl crate::Resettable for PE5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
