#[doc = "Register `FM` reader"]
pub struct R(crate::R<FM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM` writer"]
pub struct W(crate::W<FM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_SPEC>;
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
impl From<crate::W<FM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force ROM Boot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORCEROM_A {
    #[doc = "0: No effect"]
    _00 = 0,
    #[doc = "1: Force boot from ROM with RCM_MR\\[1\\]
set."]
    _01 = 1,
    #[doc = "2: Force boot from ROM with RCM_MR\\[2\\]
set."]
    _10 = 2,
    #[doc = "3: Force boot from ROM with RCM_MR\\[2:1\\]
set."]
    _11 = 3,
}
impl From<FORCEROM_A> for u8 {
    #[inline(always)]
    fn from(variant: FORCEROM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FORCEROM` reader - Force ROM Boot"]
pub struct FORCEROM_R(crate::FieldReader<u8, FORCEROM_A>);
impl FORCEROM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FORCEROM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEROM_A {
        match self.bits {
            0 => FORCEROM_A::_00,
            1 => FORCEROM_A::_01,
            2 => FORCEROM_A::_10,
            3 => FORCEROM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FORCEROM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FORCEROM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FORCEROM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FORCEROM_A::_11
    }
}
impl core::ops::Deref for FORCEROM_R {
    type Target = crate::FieldReader<u8, FORCEROM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEROM` writer - Force ROM Boot"]
pub struct FORCEROM_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEROM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FORCEROM_A::_00)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[1\\]
set."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FORCEROM_A::_01)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2\\]
set."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FORCEROM_A::_10)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2:1\\]
set."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FORCEROM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u8 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - Force ROM Boot"]
    #[inline(always)]
    pub fn forcerom(&self) -> FORCEROM_R {
        FORCEROM_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Force ROM Boot"]
    #[inline(always)]
    pub fn forcerom(&mut self) -> FORCEROM_W {
        FORCEROM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm](index.html) module"]
pub struct FM_SPEC;
impl crate::RegisterSpec for FM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fm::R](R) reader structure"]
impl crate::Readable for FM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm::W](W) writer structure"]
impl crate::Writable for FM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FM to value 0"]
impl crate::Resettable for FM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
