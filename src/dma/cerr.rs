#[doc = "Register `CERR` writer"]
pub struct W(crate::W<CERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CERR_SPEC>;
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
impl From<crate::W<CERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERR` writer - Clear Error Indicator"]
pub struct CERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "Clear All Error Indicators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAEI_AW {
    #[doc = "0: Clear only the ERR bit specified in the CERR field"]
    _0 = 0,
    #[doc = "1: Clear all bits in ERR"]
    _1 = 1,
}
impl From<CAEI_AW> for bool {
    #[inline(always)]
    fn from(variant: CAEI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAEI` writer - Clear All Error Indicators"]
pub struct CAEI_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAEI_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear only the ERR bit specified in the CERR field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAEI_AW::_0)
    }
    #[doc = "Clear all bits in ERR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAEI_AW::_1)
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
#[doc = "No Op enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOP_AW {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    _1 = 1,
}
impl From<NOP_AW> for bool {
    #[inline(always)]
    fn from(variant: NOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOP` writer - No Op enable"]
pub struct NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOP_AW::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOP_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:2 - Clear Error Indicator"]
    #[inline(always)]
    pub fn cerr(&mut self) -> CERR_W {
        CERR_W { w: self }
    }
    #[doc = "Bit 6 - Clear All Error Indicators"]
    #[inline(always)]
    pub fn caei(&mut self) -> CAEI_W {
        CAEI_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Error Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerr](index.html) module"]
pub struct CERR_SPEC;
impl crate::RegisterSpec for CERR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [cerr::W](W) writer structure"]
impl crate::Writable for CERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CERR to value 0"]
impl crate::Resettable for CERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
