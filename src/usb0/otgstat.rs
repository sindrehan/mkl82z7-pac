#[doc = "Register `OTGSTAT` reader"]
pub struct R(crate::R<OTGSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGSTAT` writer"]
pub struct W(crate::W<OTGSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGSTAT_SPEC>;
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
impl From<crate::W<OTGSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATESTABLE_A {
    #[doc = "0: The LINE_STAT_CHG bit is not yet stable."]
    _0 = 0,
    #[doc = "1: The LINE_STAT_CHG bit has been debounced and is stable."]
    _1 = 1,
}
impl From<LINESTATESTABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATESTABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINESTATESTABLE` reader - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
pub struct LINESTATESTABLE_R(crate::FieldReader<bool, LINESTATESTABLE_A>);
impl LINESTATESTABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINESTATESTABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATESTABLE_A {
        match self.bits {
            false => LINESTATESTABLE_A::_0,
            true => LINESTATESTABLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LINESTATESTABLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LINESTATESTABLE_A::_1
    }
}
impl core::ops::Deref for LINESTATESTABLE_R {
    type Target = crate::FieldReader<bool, LINESTATESTABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINESTATESTABLE` writer - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
pub struct LINESTATESTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINESTATESTABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINESTATESTABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATESTABLE_A::_0)
    }
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATESTABLE_A::_1)
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
#[doc = "Field `ONEMSECEN` reader - This bit is reserved for the 1ms count, but it is not useful to software."]
pub struct ONEMSECEN_R(crate::FieldReader<bool, bool>);
impl ONEMSECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONEMSECEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONEMSECEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONEMSECEN` writer - This bit is reserved for the 1ms count, but it is not useful to software."]
pub struct ONEMSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEMSECEN_W<'a> {
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
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
    #[inline(always)]
    pub fn linestatestable(&self) -> LINESTATESTABLE_R {
        LINESTATESTABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline(always)]
    pub fn onemsecen(&self) -> ONEMSECEN_R {
        ONEMSECEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 ms"]
    #[inline(always)]
    pub fn linestatestable(&mut self) -> LINESTATESTABLE_W {
        LINESTATESTABLE_W { w: self }
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
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
#[doc = "OTG Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgstat](index.html) module"]
pub struct OTGSTAT_SPEC;
impl crate::RegisterSpec for OTGSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [otgstat::R](R) reader structure"]
impl crate::Readable for OTGSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgstat::W](W) writer structure"]
impl crate::Writable for OTGSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTGSTAT to value 0"]
impl crate::Resettable for OTGSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
