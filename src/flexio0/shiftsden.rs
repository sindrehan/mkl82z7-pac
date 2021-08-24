#[doc = "Register `SHIFTSDEN` reader"]
pub struct R(crate::R<SHIFTSDEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTSDEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTSDEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTSDEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTSDEN` writer"]
pub struct W(crate::W<SHIFTSDEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTSDEN_SPEC>;
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
impl From<crate::W<SHIFTSDEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTSDEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shifter Status DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSDE_A {
    #[doc = "0: Shifter Status Flag DMA request is disabled"]
    _0 = 0,
    #[doc = "1: Shifter Status Flag DMA request is enabled"]
    _1 = 1,
}
impl From<SSDE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSDE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSDE` reader - Shifter Status DMA Enable"]
pub struct SSDE_R(crate::FieldReader<u8, SSDE_A>);
impl SSDE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSDE_A> {
        match self.bits {
            0 => Some(SSDE_A::_0),
            1 => Some(SSDE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSDE_A::_1
    }
}
impl core::ops::Deref for SSDE_R {
    type Target = crate::FieldReader<u8, SSDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSDE` writer - Shifter Status DMA Enable"]
pub struct SSDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Shifter Status Flag DMA request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSDE_A::_0)
    }
    #[doc = "Shifter Status Flag DMA request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSDE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SSDE_R {
        SSDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub fn ssde(&mut self) -> SSDE_W {
        SSDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Status DMA Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftsden](index.html) module"]
pub struct SHIFTSDEN_SPEC;
impl crate::RegisterSpec for SHIFTSDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftsden::R](R) reader structure"]
impl crate::Readable for SHIFTSDEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftsden::W](W) writer structure"]
impl crate::Writable for SHIFTSDEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTSDEN to value 0"]
impl crate::Resettable for SHIFTSDEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
