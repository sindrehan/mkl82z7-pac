#[doc = "Register `SHIFTERR` reader"]
pub struct R(crate::R<SHIFTERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTERR` writer"]
pub struct W(crate::W<SHIFTERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTERR_SPEC>;
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
impl From<crate::W<SHIFTERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shifter Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEF_A {
    #[doc = "0: Shifter Error Flag is clear"]
    _0 = 0,
    #[doc = "1: Shifter Error Flag is set"]
    _1 = 1,
}
impl From<SEF_A> for u8 {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEF` reader - Shifter Error Flags"]
pub struct SEF_R(crate::FieldReader<u8, SEF_A>);
impl SEF_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEF_A> {
        match self.bits {
            0 => Some(SEF_A::_0),
            1 => Some(SEF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SEF_A::_1
    }
}
impl core::ops::Deref for SEF_R {
    type Target = crate::FieldReader<u8, SEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEF` writer - Shifter Error Flags"]
pub struct SEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shifter Error Flag is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEF_A::_0)
    }
    #[doc = "Shifter Error Flag is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W {
        SEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shifterr](index.html) module"]
pub struct SHIFTERR_SPEC;
impl crate::RegisterSpec for SHIFTERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shifterr::R](R) reader structure"]
impl crate::Readable for SHIFTERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shifterr::W](W) writer structure"]
impl crate::Writable for SHIFTERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTERR to value 0"]
impl crate::Resettable for SHIFTERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
