#[doc = "Register `CLK_RECOVER_IRC_EN` reader"]
pub struct R(crate::R<CLK_RECOVER_IRC_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RECOVER_IRC_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RECOVER_IRC_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RECOVER_IRC_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RECOVER_IRC_EN` writer"]
pub struct W(crate::W<CLK_RECOVER_IRC_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RECOVER_IRC_EN_SPEC>;
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
impl From<crate::W<CLK_RECOVER_IRC_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RECOVER_IRC_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IRC48M regulator enable This bit is used to enable the local analog regulator for IRC48M module\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_EN_A {
    #[doc = "0: IRC48M local regulator is disabled"]
    _0 = 0,
    #[doc = "1: IRC48M local regulator is enabled (default)"]
    _1 = 1,
}
impl From<REG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: REG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REG_EN` reader - IRC48M regulator enable This bit is used to enable the local analog regulator for IRC48M module"]
pub struct REG_EN_R(crate::FieldReader<bool, REG_EN_A>);
impl REG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_EN_A {
        match self.bits {
            false => REG_EN_A::_0,
            true => REG_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REG_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REG_EN_A::_1
    }
}
impl core::ops::Deref for REG_EN_R {
    type Target = crate::FieldReader<bool, REG_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_EN` writer - IRC48M regulator enable This bit is used to enable the local analog regulator for IRC48M module"]
pub struct REG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IRC48M local regulator is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REG_EN_A::_0)
    }
    #[doc = "IRC48M local regulator is enabled (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REG_EN_A::_1)
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
#[doc = "IRC48M enable This bit is used to enable the on-chip IRC48M module to generate clocks for crystal-less USB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_EN_A {
    #[doc = "0: Disable the IRC48M module (default)"]
    _0 = 0,
    #[doc = "1: Enable the IRC48M module"]
    _1 = 1,
}
impl From<IRC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IRC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC_EN` reader - IRC48M enable This bit is used to enable the on-chip IRC48M module to generate clocks for crystal-less USB"]
pub struct IRC_EN_R(crate::FieldReader<bool, IRC_EN_A>);
impl IRC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC_EN_A {
        match self.bits {
            false => IRC_EN_A::_0,
            true => IRC_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRC_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRC_EN_A::_1
    }
}
impl core::ops::Deref for IRC_EN_R {
    type Target = crate::FieldReader<bool, IRC_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRC_EN` writer - IRC48M enable This bit is used to enable the on-chip IRC48M module to generate clocks for crystal-less USB"]
pub struct IRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the IRC48M module (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRC_EN_A::_0)
    }
    #[doc = "Enable the IRC48M module"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRC_EN_A::_1)
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
impl R {
    #[doc = "Bit 0 - IRC48M regulator enable This bit is used to enable the local analog regulator for IRC48M module"]
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRC48M enable This bit is used to enable the on-chip IRC48M module to generate clocks for crystal-less USB"]
    #[inline(always)]
    pub fn irc_en(&self) -> IRC_EN_R {
        IRC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRC48M regulator enable This bit is used to enable the local analog regulator for IRC48M module"]
    #[inline(always)]
    pub fn reg_en(&mut self) -> REG_EN_W {
        REG_EN_W { w: self }
    }
    #[doc = "Bit 1 - IRC48M enable This bit is used to enable the on-chip IRC48M module to generate clocks for crystal-less USB"]
    #[inline(always)]
    pub fn irc_en(&mut self) -> IRC_EN_W {
        IRC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRC48M oscillator enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_recover_irc_en](index.html) module"]
pub struct CLK_RECOVER_IRC_EN_SPEC;
impl crate::RegisterSpec for CLK_RECOVER_IRC_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clk_recover_irc_en::R](R) reader structure"]
impl crate::Readable for CLK_RECOVER_IRC_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_recover_irc_en::W](W) writer structure"]
impl crate::Writable for CLK_RECOVER_IRC_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_RECOVER_IRC_EN to value 0x01"]
impl crate::Resettable for CLK_RECOVER_IRC_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
