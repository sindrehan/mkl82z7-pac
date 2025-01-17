#[doc = "Register `TIMSTAT` reader"]
pub struct R(crate::R<TIMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSTAT` writer"]
pub struct W(crate::W<TIMSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSTAT_SPEC>;
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
impl From<crate::W<TIMSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Status Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSF_A {
    #[doc = "0: Timer Status Flag is clear"]
    _0 = 0,
    #[doc = "1: Timer Status Flag is set"]
    _1 = 1,
}
impl From<TSF_A> for u8 {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSF` reader - Timer Status Flags"]
pub struct TSF_R(crate::FieldReader<u8, TSF_A>);
impl TSF_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSF_A> {
        match self.bits {
            0 => Some(TSF_A::_0),
            1 => Some(TSF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSF_A::_1
    }
}
impl core::ops::Deref for TSF_R {
    type Target = crate::FieldReader<u8, TSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSF` writer - Timer Status Flags"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer Status Flag is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSF_A::_0)
    }
    #[doc = "Timer Status Flag is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer Status Flags"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Status Flags"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timstat](index.html) module"]
pub struct TIMSTAT_SPEC;
impl crate::RegisterSpec for TIMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timstat::R](R) reader structure"]
impl crate::Readable for TIMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timstat::W](W) writer structure"]
impl crate::Writable for TIMSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMSTAT to value 0"]
impl crate::Resettable for TIMSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
