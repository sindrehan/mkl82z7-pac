#[doc = "Register `CEEI` writer"]
pub struct W(crate::W<CEEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEEI_SPEC>;
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
impl From<crate::W<CEEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEEI` writer - Clear Enable Error Interrupt"]
pub struct CEEI_W<'a> {
    w: &'a mut W,
}
impl<'a> CEEI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "Clear All Enable Error Interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAEE_AW {
    #[doc = "0: Clear only the EEI bit specified in the CEEI field"]
    _0 = 0,
    #[doc = "1: Clear all bits in EEI"]
    _1 = 1,
}
impl From<CAEE_AW> for bool {
    #[inline(always)]
    fn from(variant: CAEE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAEE` writer - Clear All Enable Error Interrupts"]
pub struct CAEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAEE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear only the EEI bit specified in the CEEI field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAEE_AW::_0)
    }
    #[doc = "Clear all bits in EEI"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAEE_AW::_1)
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
    #[doc = "Bits 0:2 - Clear Enable Error Interrupt"]
    #[inline(always)]
    pub fn ceei(&mut self) -> CEEI_W {
        CEEI_W { w: self }
    }
    #[doc = "Bit 6 - Clear All Enable Error Interrupts"]
    #[inline(always)]
    pub fn caee(&mut self) -> CAEE_W {
        CAEE_W { w: self }
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
#[doc = "Clear Enable Error Interrupt Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceei](index.html) module"]
pub struct CEEI_SPEC;
impl crate::RegisterSpec for CEEI_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [ceei::W](W) writer structure"]
impl crate::Writable for CEEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEEI to value 0"]
impl crate::Resettable for CEEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
