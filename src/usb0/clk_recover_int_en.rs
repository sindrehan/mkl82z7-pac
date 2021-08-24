#[doc = "Register `CLK_RECOVER_INT_EN` reader"]
pub struct R(crate::R<CLK_RECOVER_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RECOVER_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RECOVER_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RECOVER_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RECOVER_INT_EN` writer"]
pub struct W(crate::W<CLK_RECOVER_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RECOVER_INT_EN_SPEC>;
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
impl From<crate::W<CLK_RECOVER_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RECOVER_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVF_ERROR_EN_A {
    #[doc = "0: The interrupt will be masked"]
    _0 = 0,
    #[doc = "1: The interrupt will be enabled (default)"]
    _1 = 1,
}
impl From<OVF_ERROR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OVF_ERROR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF_ERROR_EN` reader - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
pub struct OVF_ERROR_EN_R(crate::FieldReader<bool, OVF_ERROR_EN_A>);
impl OVF_ERROR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_ERROR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVF_ERROR_EN_A {
        match self.bits {
            false => OVF_ERROR_EN_A::_0,
            true => OVF_ERROR_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OVF_ERROR_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OVF_ERROR_EN_A::_1
    }
}
impl core::ops::Deref for OVF_ERROR_EN_R {
    type Target = crate::FieldReader<bool, OVF_ERROR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_ERROR_EN` writer - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
pub struct OVF_ERROR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_ERROR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVF_ERROR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt will be masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVF_ERROR_EN_A::_0)
    }
    #[doc = "The interrupt will be enabled (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVF_ERROR_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
    #[inline(always)]
    pub fn ovf_error_en(&self) -> OVF_ERROR_EN_R {
        OVF_ERROR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
    #[inline(always)]
    pub fn ovf_error_en(&mut self) -> OVF_ERROR_EN_W {
        OVF_ERROR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock recovery combined interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_recover_int_en](index.html) module"]
pub struct CLK_RECOVER_INT_EN_SPEC;
impl crate::RegisterSpec for CLK_RECOVER_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clk_recover_int_en::R](R) reader structure"]
impl crate::Readable for CLK_RECOVER_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_recover_int_en::W](W) writer structure"]
impl crate::Writable for CLK_RECOVER_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_RECOVER_INT_EN to value 0x10"]
impl crate::Resettable for CLK_RECOVER_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
