#[doc = "Register `MISCCTRL` reader"]
pub struct R(crate::R<MISCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISCCTRL` writer"]
pub struct W(crate::W<MISCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISCCTRL_SPEC>;
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
impl From<crate::W<MISCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Dynamic SOF Threshold Compare mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFDYNTHLD_A {
    #[doc = "0: SOF_TOK interrupt is set when byte times SOF threshold is reached."]
    _0 = 0,
    #[doc = "1: SOF_TOK interrupt is set when 8 byte times SOF threshold is reached or overstepped."]
    _1 = 1,
}
impl From<SOFDYNTHLD_A> for bool {
    #[inline(always)]
    fn from(variant: SOFDYNTHLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFDYNTHLD` reader - Dynamic SOF Threshold Compare mode"]
pub struct SOFDYNTHLD_R(crate::FieldReader<bool, SOFDYNTHLD_A>);
impl SOFDYNTHLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFDYNTHLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFDYNTHLD_A {
        match self.bits {
            false => SOFDYNTHLD_A::_0,
            true => SOFDYNTHLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOFDYNTHLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOFDYNTHLD_A::_1
    }
}
impl core::ops::Deref for SOFDYNTHLD_R {
    type Target = crate::FieldReader<bool, SOFDYNTHLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFDYNTHLD` writer - Dynamic SOF Threshold Compare mode"]
pub struct SOFDYNTHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFDYNTHLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFDYNTHLD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SOF_TOK interrupt is set when byte times SOF threshold is reached."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFDYNTHLD_A::_0)
    }
    #[doc = "SOF_TOK interrupt is set when 8 byte times SOF threshold is reached or overstepped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFDYNTHLD_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "SOF_TOK Interrupt Generation Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFBUSSET_A {
    #[doc = "0: SOF_TOK interrupt is set according to SOF threshold value."]
    _0 = 0,
    #[doc = "1: SOF_TOK interrupt is set when SOF counter reaches 0."]
    _1 = 1,
}
impl From<SOFBUSSET_A> for bool {
    #[inline(always)]
    fn from(variant: SOFBUSSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFBUSSET` reader - SOF_TOK Interrupt Generation Mode Select"]
pub struct SOFBUSSET_R(crate::FieldReader<bool, SOFBUSSET_A>);
impl SOFBUSSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFBUSSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFBUSSET_A {
        match self.bits {
            false => SOFBUSSET_A::_0,
            true => SOFBUSSET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SOFBUSSET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SOFBUSSET_A::_1
    }
}
impl core::ops::Deref for SOFBUSSET_R {
    type Target = crate::FieldReader<bool, SOFBUSSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFBUSSET` writer - SOF_TOK Interrupt Generation Mode Select"]
pub struct SOFBUSSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFBUSSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFBUSSET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SOF_TOK interrupt is set according to SOF threshold value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFBUSSET_A::_0)
    }
    #[doc = "SOF_TOK interrupt is set when SOF counter reaches 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFBUSSET_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "OWN Error Detect for ISO IN / ISO OUT Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWNERRISODIS_A {
    #[doc = "0: OWN error detect for ISO IN / ISO OUT is not disabled."]
    _0 = 0,
    #[doc = "1: OWN error detect for ISO IN / ISO OUT is disabled."]
    _1 = 1,
}
impl From<OWNERRISODIS_A> for bool {
    #[inline(always)]
    fn from(variant: OWNERRISODIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWNERRISODIS` reader - OWN Error Detect for ISO IN / ISO OUT Disable"]
pub struct OWNERRISODIS_R(crate::FieldReader<bool, OWNERRISODIS_A>);
impl OWNERRISODIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OWNERRISODIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWNERRISODIS_A {
        match self.bits {
            false => OWNERRISODIS_A::_0,
            true => OWNERRISODIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OWNERRISODIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OWNERRISODIS_A::_1
    }
}
impl core::ops::Deref for OWNERRISODIS_R {
    type Target = crate::FieldReader<bool, OWNERRISODIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OWNERRISODIS` writer - OWN Error Detect for ISO IN / ISO OUT Disable"]
pub struct OWNERRISODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OWNERRISODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWNERRISODIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OWN error detect for ISO IN / ISO OUT is not disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OWNERRISODIS_A::_0)
    }
    #[doc = "OWN error detect for ISO IN / ISO OUT is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OWNERRISODIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "VREGIN Rising Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREDG_EN_A {
    #[doc = "0: VREGIN rising edge interrupt disabled."]
    _0 = 0,
    #[doc = "1: VREGIN rising edge interrupt enabled."]
    _1 = 1,
}
impl From<VREDG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VREDG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREDG_EN` reader - VREGIN Rising Edge Interrupt Enable"]
pub struct VREDG_EN_R(crate::FieldReader<bool, VREDG_EN_A>);
impl VREDG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREDG_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREDG_EN_A {
        match self.bits {
            false => VREDG_EN_A::_0,
            true => VREDG_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VREDG_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VREDG_EN_A::_1
    }
}
impl core::ops::Deref for VREDG_EN_R {
    type Target = crate::FieldReader<bool, VREDG_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREDG_EN` writer - VREGIN Rising Edge Interrupt Enable"]
pub struct VREDG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREDG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREDG_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VREGIN rising edge interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREDG_EN_A::_0)
    }
    #[doc = "VREGIN rising edge interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREDG_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "VREGIN Falling Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VFEDG_EN_A {
    #[doc = "0: VREGIN falling edge interrupt disabled."]
    _0 = 0,
    #[doc = "1: VREGIN falling edge interrupt enabled."]
    _1 = 1,
}
impl From<VFEDG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VFEDG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VFEDG_EN` reader - VREGIN Falling Edge Interrupt Enable"]
pub struct VFEDG_EN_R(crate::FieldReader<bool, VFEDG_EN_A>);
impl VFEDG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VFEDG_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VFEDG_EN_A {
        match self.bits {
            false => VFEDG_EN_A::_0,
            true => VFEDG_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VFEDG_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VFEDG_EN_A::_1
    }
}
impl core::ops::Deref for VFEDG_EN_R {
    type Target = crate::FieldReader<bool, VFEDG_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VFEDG_EN` writer - VREGIN Falling Edge Interrupt Enable"]
pub struct VFEDG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VFEDG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VFEDG_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VREGIN falling edge interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VFEDG_EN_A::_0)
    }
    #[doc = "VREGIN falling edge interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VFEDG_EN_A::_1)
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
#[doc = "USB Peripheral mode Stall Adjust Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STL_ADJ_EN_A {
    #[doc = "0: If USB_ENDPTn\\[END_STALL\\]
= 1, both IN and OUT directions for the associated endpoint will be stalled"]
    _0 = 0,
    #[doc = "1: If USB_ENDPTn\\[END_STALL\\]
= 1, the USB_STALL_xx_DIS registers control which directions for the associated endpoint will be stalled."]
    _1 = 1,
}
impl From<STL_ADJ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: STL_ADJ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STL_ADJ_EN` reader - USB Peripheral mode Stall Adjust Enable"]
pub struct STL_ADJ_EN_R(crate::FieldReader<bool, STL_ADJ_EN_A>);
impl STL_ADJ_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STL_ADJ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STL_ADJ_EN_A {
        match self.bits {
            false => STL_ADJ_EN_A::_0,
            true => STL_ADJ_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STL_ADJ_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STL_ADJ_EN_A::_1
    }
}
impl core::ops::Deref for STL_ADJ_EN_R {
    type Target = crate::FieldReader<bool, STL_ADJ_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STL_ADJ_EN` writer - USB Peripheral mode Stall Adjust Enable"]
pub struct STL_ADJ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STL_ADJ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STL_ADJ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If USB_ENDPTn\\[END_STALL\\]
= 1, both IN and OUT directions for the associated endpoint will be stalled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STL_ADJ_EN_A::_0)
    }
    #[doc = "If USB_ENDPTn\\[END_STALL\\]
= 1, the USB_STALL_xx_DIS registers control which directions for the associated endpoint will be stalled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STL_ADJ_EN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Dynamic SOF Threshold Compare mode"]
    #[inline(always)]
    pub fn sofdynthld(&self) -> SOFDYNTHLD_R {
        SOFDYNTHLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SOF_TOK Interrupt Generation Mode Select"]
    #[inline(always)]
    pub fn sofbusset(&self) -> SOFBUSSET_R {
        SOFBUSSET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OWN Error Detect for ISO IN / ISO OUT Disable"]
    #[inline(always)]
    pub fn ownerrisodis(&self) -> OWNERRISODIS_R {
        OWNERRISODIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn vredg_en(&self) -> VREDG_EN_R {
        VREDG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn vfedg_en(&self) -> VFEDG_EN_R {
        VFEDG_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB Peripheral mode Stall Adjust Enable"]
    #[inline(always)]
    pub fn stl_adj_en(&self) -> STL_ADJ_EN_R {
        STL_ADJ_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dynamic SOF Threshold Compare mode"]
    #[inline(always)]
    pub fn sofdynthld(&mut self) -> SOFDYNTHLD_W {
        SOFDYNTHLD_W { w: self }
    }
    #[doc = "Bit 1 - SOF_TOK Interrupt Generation Mode Select"]
    #[inline(always)]
    pub fn sofbusset(&mut self) -> SOFBUSSET_W {
        SOFBUSSET_W { w: self }
    }
    #[doc = "Bit 2 - OWN Error Detect for ISO IN / ISO OUT Disable"]
    #[inline(always)]
    pub fn ownerrisodis(&mut self) -> OWNERRISODIS_W {
        OWNERRISODIS_W { w: self }
    }
    #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn vredg_en(&mut self) -> VREDG_EN_W {
        VREDG_EN_W { w: self }
    }
    #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn vfedg_en(&mut self) -> VFEDG_EN_W {
        VFEDG_EN_W { w: self }
    }
    #[doc = "Bit 7 - USB Peripheral mode Stall Adjust Enable"]
    #[inline(always)]
    pub fn stl_adj_en(&mut self) -> STL_ADJ_EN_W {
        STL_ADJ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miscctrl](index.html) module"]
pub struct MISCCTRL_SPEC;
impl crate::RegisterSpec for MISCCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [miscctrl::R](R) reader structure"]
impl crate::Readable for MISCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miscctrl::W](W) writer structure"]
impl crate::Writable for MISCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISCCTRL to value 0"]
impl crate::Resettable for MISCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
