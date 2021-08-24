#[doc = "Register `SHIFTSTAT` reader"]
pub struct R(crate::R<SHIFTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTSTAT` writer"]
pub struct W(crate::W<SHIFTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTSTAT_SPEC>;
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
impl From<crate::W<SHIFTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shifter Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSF_A {
    #[doc = "0: Status flag is clear"]
    _0 = 0,
    #[doc = "1: Status flag is set"]
    _1 = 1,
}
impl From<SSF_A> for u8 {
    #[inline(always)]
    fn from(variant: SSF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSF` reader - Shifter Status Flag"]
pub struct SSF_R(crate::FieldReader<u8, SSF_A>);
impl SSF_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSF_A> {
        match self.bits {
            0 => Some(SSF_A::_0),
            1 => Some(SSF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSF_A::_1
    }
}
impl core::ops::Deref for SSF_R {
    type Target = crate::FieldReader<u8, SSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSF` writer - Shifter Status Flag"]
pub struct SSF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Status flag is clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSF_A::_0)
    }
    #[doc = "Status flag is set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&mut self) -> SSF_W {
        SSF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftstat](index.html) module"]
pub struct SHIFTSTAT_SPEC;
impl crate::RegisterSpec for SHIFTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftstat::R](R) reader structure"]
impl crate::Readable for SHIFTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftstat::W](W) writer structure"]
impl crate::Writable for SHIFTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTSTAT to value 0"]
impl crate::Resettable for SHIFTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
