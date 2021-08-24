#[doc = "Register `TX_GETU` reader"]
pub struct R(crate::R<TX_GETU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_GETU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_GETU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_GETU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_GETU` writer"]
pub struct W(crate::W<TX_GETU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_GETU_SPEC>;
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
impl From<crate::W<TX_GETU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_GETU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmitter Guard Time Value in ETU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GETU_A {
    #[doc = "0: no additional ETUs inserted (default)"]
    _0 = 0,
    #[doc = "1: 1 additional ETU inserted"]
    _1 = 1,
}
impl From<GETU_A> for u8 {
    #[inline(always)]
    fn from(variant: GETU_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GETU` reader - Transmitter Guard Time Value in ETU"]
pub struct GETU_R(crate::FieldReader<u8, GETU_A>);
impl GETU_R {
    pub(crate) fn new(bits: u8) -> Self {
        GETU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GETU_A> {
        match self.bits {
            0 => Some(GETU_A::_0),
            1 => Some(GETU_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GETU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GETU_A::_1
    }
}
impl core::ops::Deref for GETU_R {
    type Target = crate::FieldReader<u8, GETU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GETU` writer - Transmitter Guard Time Value in ETU"]
pub struct GETU_W<'a> {
    w: &'a mut W,
}
impl<'a> GETU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GETU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no additional ETUs inserted (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GETU_A::_0)
    }
    #[doc = "1 additional ETU inserted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GETU_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmitter Guard Time Value in ETU"]
    #[inline(always)]
    pub fn getu(&self) -> GETU_R {
        GETU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmitter Guard Time Value in ETU"]
    #[inline(always)]
    pub fn getu(&mut self) -> GETU_W {
        GETU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Guard ETU Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_getu](index.html) module"]
pub struct TX_GETU_SPEC;
impl crate::RegisterSpec for TX_GETU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_getu::R](R) reader structure"]
impl crate::Readable for TX_GETU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_getu::W](W) writer structure"]
impl crate::Writable for TX_GETU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_GETU to value 0"]
impl crate::Resettable for TX_GETU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
