#[doc = "Register `LTC0_COM` reader"]
pub struct R(crate::R<LTC0_COM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_COM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_COM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_COM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_COM` writer"]
pub struct W(crate::W<LTC0_COM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_COM_SPEC>;
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
impl From<crate::W<LTC0_COM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_COM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset All Internal Logic\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALL_AW {
    #[doc = "0: Do Not Reset"]
    _0 = 0,
    #[doc = "1: Reset all CHAs in use by this CCB."]
    _1 = 1,
}
impl From<ALL_AW> for bool {
    #[inline(always)]
    fn from(variant: ALL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALL` writer - Reset All Internal Logic"]
pub struct ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do Not Reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALL_AW::_0)
    }
    #[doc = "Reset all CHAs in use by this CCB."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALL_AW::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Reset AESA. Writing a 1 to this bit resets the AES Accelerator core engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_AW {
    #[doc = "0: Do Not Reset"]
    _0 = 0,
    #[doc = "1: Reset AES Accelerator"]
    _1 = 1,
}
impl From<AES_AW> for bool {
    #[inline(always)]
    fn from(variant: AES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AES` writer - Reset AESA. Writing a 1 to this bit resets the AES Accelerator core engine."]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do Not Reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AES_AW::_0)
    }
    #[doc = "Reset AES Accelerator"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AES_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Reset DESA. Writing a 1 to this bit resets the DES Accelerator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DES_AW {
    #[doc = "0: Do Not Reset"]
    _0 = 0,
    #[doc = "1: Reset DES Accelerator"]
    _1 = 1,
}
impl From<DES_AW> for bool {
    #[inline(always)]
    fn from(variant: DES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DES` writer - Reset DESA. Writing a 1 to this bit resets the DES Accelerator."]
pub struct DES_W<'a> {
    w: &'a mut W,
}
impl<'a> DES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DES_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do Not Reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DES_AW::_0)
    }
    #[doc = "Reset DES Accelerator"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DES_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Reset PKHA. Writing a 1 to this bit resets the Public Key Hardware Accelerator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PK_AW {
    #[doc = "0: Do Not Reset"]
    _0 = 0,
    #[doc = "1: Reset Public Key Hardware Accelerator"]
    _1 = 1,
}
impl From<PK_AW> for bool {
    #[inline(always)]
    fn from(variant: PK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PK` writer - Reset PKHA. Writing a 1 to this bit resets the Public Key Hardware Accelerator."]
pub struct PK_W<'a> {
    w: &'a mut W,
}
impl<'a> PK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do Not Reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PK_AW::_0)
    }
    #[doc = "Reset Public Key Hardware Accelerator"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PK_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Reset MDHA. Writing a 1 to this bit resets the Message Digest Hardware Accelerator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MD_AW {
    #[doc = "0: Do Not Reset"]
    _0 = 0,
    #[doc = "1: Reset Message Digest Hardware Accelerator"]
    _1 = 1,
}
impl From<MD_AW> for bool {
    #[inline(always)]
    fn from(variant: MD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MD` writer - Reset MDHA. Writing a 1 to this bit resets the Message Digest Hardware Accelerator."]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MD_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do Not Reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MD_AW::_0)
    }
    #[doc = "Reset Message Digest Hardware Accelerator"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MD_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Reset All Internal Logic"]
    #[inline(always)]
    pub fn all(&mut self) -> ALL_W {
        ALL_W { w: self }
    }
    #[doc = "Bit 1 - Reset AESA. Writing a 1 to this bit resets the AES Accelerator core engine."]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 2 - Reset DESA. Writing a 1 to this bit resets the DES Accelerator."]
    #[inline(always)]
    pub fn des(&mut self) -> DES_W {
        DES_W { w: self }
    }
    #[doc = "Bit 6 - Reset PKHA. Writing a 1 to this bit resets the Public Key Hardware Accelerator."]
    #[inline(always)]
    pub fn pk(&mut self) -> PK_W {
        PK_W { w: self }
    }
    #[doc = "Bit 7 - Reset MDHA. Writing a 1 to this bit resets the Message Digest Hardware Accelerator."]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_com](index.html) module"]
pub struct LTC0_COM_SPEC;
impl crate::RegisterSpec for LTC0_COM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_com::R](R) reader structure"]
impl crate::Readable for LTC0_COM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_com::W](W) writer structure"]
impl crate::Writable for LTC0_COM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_COM to value 0"]
impl crate::Resettable for LTC0_COM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
