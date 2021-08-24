#[doc = "Register `SHIFTEIEN` reader"]
pub struct R(crate::R<SHIFTEIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTEIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTEIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTEIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTEIEN` writer"]
pub struct W(crate::W<SHIFTEIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTEIEN_SPEC>;
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
impl From<crate::W<SHIFTEIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTEIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shifter Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEIE_A {
    #[doc = "0: Shifter Error Flag interrupt disabled"]
    _0 = 0,
    #[doc = "1: Shifter Error Flag interrupt enabled"]
    _1 = 1,
}
impl From<SEIE_A> for u8 {
    #[inline(always)]
    fn from(variant: SEIE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEIE` reader - Shifter Error Interrupt Enable"]
pub struct SEIE_R(crate::FieldReader<u8, SEIE_A>);
impl SEIE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEIE_A> {
        match self.bits {
            0 => Some(SEIE_A::_0),
            1 => Some(SEIE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SEIE_A::_1
    }
}
impl core::ops::Deref for SEIE_R {
    type Target = crate::FieldReader<u8, SEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEIE` writer - Shifter Error Interrupt Enable"]
pub struct SEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shifter Error Flag interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEIE_A::_0)
    }
    #[doc = "Shifter Error Flag interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEIE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W {
        SEIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Error Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shifteien](index.html) module"]
pub struct SHIFTEIEN_SPEC;
impl crate::RegisterSpec for SHIFTEIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shifteien::R](R) reader structure"]
impl crate::Readable for SHIFTEIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shifteien::W](W) writer structure"]
impl crate::Writable for SHIFTEIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTEIEN to value 0"]
impl crate::Resettable for SHIFTEIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
