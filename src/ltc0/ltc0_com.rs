#[doc = "Reader of register LTC0_COM"]
pub type R = crate::R<u32, super::LTC0_COM>;
#[doc = "Writer for register LTC0_COM"]
pub type W = crate::W<u32, super::LTC0_COM>;
#[doc = "Register LTC0_COM `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_COM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Write proxy for field `ALL`"]
pub struct ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AES_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
#[doc = "Write proxy for field `DES`"]
pub struct DES_W<'a> {
    w: &'a mut W,
}
impl<'a> DES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DES_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
#[doc = "Write proxy for field `PK`"]
pub struct PK_W<'a> {
    w: &'a mut W,
}
impl<'a> PK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
#[doc = "Write proxy for field `MD`"]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {}
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
}
