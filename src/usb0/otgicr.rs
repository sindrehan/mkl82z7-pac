#[doc = "Register `OTGICR` reader"]
pub struct R(crate::R<OTGICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGICR` writer"]
pub struct W(crate::W<OTGICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGICR_SPEC>;
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
impl From<crate::W<OTGICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Line State Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATEEN_A {
    #[doc = "0: Disables the LINE_STAT_CHG interrupt."]
    _0 = 0,
    #[doc = "1: Enables the LINE_STAT_CHG interrupt."]
    _1 = 1,
}
impl From<LINESTATEEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINESTATEEN` reader - Line State Change Interrupt Enable"]
pub struct LINESTATEEN_R(crate::FieldReader<bool, LINESTATEEN_A>);
impl LINESTATEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINESTATEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATEEN_A {
        match self.bits {
            false => LINESTATEEN_A::_0,
            true => LINESTATEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LINESTATEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LINESTATEEN_A::_1
    }
}
impl core::ops::Deref for LINESTATEEN_R {
    type Target = crate::FieldReader<bool, LINESTATEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINESTATEEN` writer - Line State Change Interrupt Enable"]
pub struct LINESTATEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LINESTATEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINESTATEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATEEN_A::_0)
    }
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "One Millisecond Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEMSECEN_A {
    #[doc = "0: Diables the 1ms timer interrupt."]
    _0 = 0,
    #[doc = "1: Enables the 1ms timer interrupt."]
    _1 = 1,
}
impl From<ONEMSECEN_A> for bool {
    #[inline(always)]
    fn from(variant: ONEMSECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONEMSECEN` reader - One Millisecond Interrupt Enable"]
pub struct ONEMSECEN_R(crate::FieldReader<bool, ONEMSECEN_A>);
impl ONEMSECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONEMSECEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONEMSECEN_A {
        match self.bits {
            false => ONEMSECEN_A::_0,
            true => ONEMSECEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ONEMSECEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ONEMSECEN_A::_1
    }
}
impl core::ops::Deref for ONEMSECEN_R {
    type Target = crate::FieldReader<bool, ONEMSECEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONEMSECEN` writer - One Millisecond Interrupt Enable"]
pub struct ONEMSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEMSECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONEMSECEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Diables the 1ms timer interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONEMSECEN_A::_0)
    }
    #[doc = "Enables the 1ms timer interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONEMSECEN_A::_1)
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
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline(always)]
    pub fn linestateen(&self) -> LINESTATEEN_R {
        LINESTATEEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline(always)]
    pub fn onemsecen(&self) -> ONEMSECEN_R {
        ONEMSECEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline(always)]
    pub fn linestateen(&mut self) -> LINESTATEEN_W {
        LINESTATEEN_W { w: self }
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline(always)]
    pub fn onemsecen(&mut self) -> ONEMSECEN_W {
        ONEMSECEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgicr](index.html) module"]
pub struct OTGICR_SPEC;
impl crate::RegisterSpec for OTGICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [otgicr::R](R) reader structure"]
impl crate::Readable for OTGICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgicr::W](W) writer structure"]
impl crate::Writable for OTGICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTGICR to value 0"]
impl crate::Resettable for OTGICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
