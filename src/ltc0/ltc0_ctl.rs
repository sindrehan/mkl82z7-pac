#[doc = "Reader of register LTC0_CTL"]
pub type R = crate::R<u32, super::LTC0_CTL>;
#[doc = "Writer for register LTC0_CTL"]
pub type W = crate::W<u32, super::LTC0_CTL>;
#[doc = "Register LTC0_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Mask. Once this bit is set, it can only be cleared by hard reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM_A {
    #[doc = "0: Interrupt not masked."]
    _0 = 0,
    #[doc = "1: Interrupt masked"]
    _1 = 1,
}
impl From<IM_A> for bool {
    #[inline(always)]
    fn from(variant: IM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IM`"]
pub type IM_R = crate::R<bool, IM_A>;
impl IM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM_A {
        match self.bits {
            false => IM_A::_0,
            true => IM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IM_A::_1
    }
}
#[doc = "Write proxy for field `IM`"]
pub struct IM_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IM_A::_0)
    }
    #[doc = "Interrupt masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IM_A::_1)
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
#[doc = "PKHA Register DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDE_A {
    #[doc = "0: DMA Request and Done signals disabled for the PKHA Registers."]
    _0 = 0,
    #[doc = "1: DMA Request and Done signals enabled for the PKHA Registers."]
    _1 = 1,
}
impl From<PDE_A> for bool {
    #[inline(always)]
    fn from(variant: PDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDE`"]
pub type PDE_R = crate::R<bool, PDE_A>;
impl PDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDE_A {
        match self.bits {
            false => PDE_A::_0,
            true => PDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDE_A::_1
    }
}
#[doc = "Write proxy for field `PDE`"]
pub struct PDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA Request and Done signals disabled for the PKHA Registers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDE_A::_0)
    }
    #[doc = "DMA Request and Done signals enabled for the PKHA Registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Input FIFO DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFE_A {
    #[doc = "0: DMA Request and Done signals disabled for the Input FIFO."]
    _0 = 0,
    #[doc = "1: DMA Request and Done signals enabled for the Input FIFO."]
    _1 = 1,
}
impl From<IFE_A> for bool {
    #[inline(always)]
    fn from(variant: IFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFE`"]
pub type IFE_R = crate::R<bool, IFE_A>;
impl IFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFE_A {
        match self.bits {
            false => IFE_A::_0,
            true => IFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IFE_A::_1
    }
}
#[doc = "Write proxy for field `IFE`"]
pub struct IFE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA Request and Done signals disabled for the Input FIFO."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFE_A::_0)
    }
    #[doc = "DMA Request and Done signals enabled for the Input FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Input FIFO DMA Request Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFR_A {
    #[doc = "0: DMA request size is 1 entry."]
    _0 = 0,
    #[doc = "1: DMA request size is 4 entries."]
    _1 = 1,
}
impl From<IFR_A> for bool {
    #[inline(always)]
    fn from(variant: IFR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFR`"]
pub type IFR_R = crate::R<bool, IFR_A>;
impl IFR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFR_A {
        match self.bits {
            false => IFR_A::_0,
            true => IFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IFR_A::_1
    }
}
#[doc = "Write proxy for field `IFR`"]
pub struct IFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request size is 1 entry."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFR_A::_0)
    }
    #[doc = "DMA request size is 4 entries."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Output FIFO DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFE_A {
    #[doc = "0: DMA Request and Done signals disabled for the Output FIFO."]
    _0 = 0,
    #[doc = "1: DMA Request and Done signals enabled for the Output FIFO."]
    _1 = 1,
}
impl From<OFE_A> for bool {
    #[inline(always)]
    fn from(variant: OFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFE`"]
pub type OFE_R = crate::R<bool, OFE_A>;
impl OFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFE_A {
        match self.bits {
            false => OFE_A::_0,
            true => OFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFE_A::_1
    }
}
#[doc = "Write proxy for field `OFE`"]
pub struct OFE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA Request and Done signals disabled for the Output FIFO."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFE_A::_0)
    }
    #[doc = "DMA Request and Done signals enabled for the Output FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Output FIFO DMA Request Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFR_A {
    #[doc = "0: DMA request size is 1 entry."]
    _0 = 0,
    #[doc = "1: DMA request size is 4 entries."]
    _1 = 1,
}
impl From<OFR_A> for bool {
    #[inline(always)]
    fn from(variant: OFR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFR`"]
pub type OFR_R = crate::R<bool, OFR_A>;
impl OFR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFR_A {
        match self.bits {
            false => OFR_A::_0,
            true => OFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFR_A::_1
    }
}
#[doc = "Write proxy for field `OFR`"]
pub struct OFR_W<'a> {
    w: &'a mut W,
}
impl<'a> OFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request size is 1 entry."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFR_A::_0)
    }
    #[doc = "DMA request size is 4 entries."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Input FIFO Byte Swap. Byte swap all data that is written to the Input FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFS_A {
    #[doc = "0: Do Not Byte Swap Data."]
    _0 = 0,
    #[doc = "1: Byte Swap Data."]
    _1 = 1,
}
impl From<IFS_A> for bool {
    #[inline(always)]
    fn from(variant: IFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFS`"]
pub type IFS_R = crate::R<bool, IFS_A>;
impl IFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFS_A {
        match self.bits {
            false => IFS_A::_0,
            true => IFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IFS_A::_1
    }
}
#[doc = "Write proxy for field `IFS`"]
pub struct IFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFS_A::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Output FIFO Byte Swap. Byte swap all data that is read from the Onput FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFS_A {
    #[doc = "0: Do Not Byte Swap Data."]
    _0 = 0,
    #[doc = "1: Byte Swap Data."]
    _1 = 1,
}
impl From<OFS_A> for bool {
    #[inline(always)]
    fn from(variant: OFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFS`"]
pub type OFS_R = crate::R<bool, OFS_A>;
impl OFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFS_A {
        match self.bits {
            false => OFS_A::_0,
            true => OFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OFS_A::_1
    }
}
#[doc = "Write proxy for field `OFS`"]
pub struct OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> OFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFS_A::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Key Register Input Byte Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KIS_A {
    #[doc = "0: Do Not Byte Swap Data."]
    _0 = 0,
    #[doc = "1: Byte Swap Data."]
    _1 = 1,
}
impl From<KIS_A> for bool {
    #[inline(always)]
    fn from(variant: KIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KIS`"]
pub type KIS_R = crate::R<bool, KIS_A>;
impl KIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIS_A {
        match self.bits {
            false => KIS_A::_0,
            true => KIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIS_A::_1
    }
}
#[doc = "Write proxy for field `KIS`"]
pub struct KIS_W<'a> {
    w: &'a mut W,
}
impl<'a> KIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIS_A::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Key Register Output Byte Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KOS_A {
    #[doc = "0: Do Not Byte Swap Data."]
    _0 = 0,
    #[doc = "1: Byte Swap Data."]
    _1 = 1,
}
impl From<KOS_A> for bool {
    #[inline(always)]
    fn from(variant: KOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KOS`"]
pub type KOS_R = crate::R<bool, KOS_A>;
impl KOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KOS_A {
        match self.bits {
            false => KOS_A::_0,
            true => KOS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KOS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KOS_A::_1
    }
}
#[doc = "Write proxy for field `KOS`"]
pub struct KOS_W<'a> {
    w: &'a mut W,
}
impl<'a> KOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KOS_A::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KOS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Context Register Input Byte Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIS_A {
    #[doc = "0: Do Not Byte Swap Data."]
    _0 = 0,
    #[doc = "1: Byte Swap Data."]
    _1 = 1,
}
impl From<CIS_A> for bool {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CIS`"]
pub type CIS_R = crate::R<bool, CIS_A>;
impl CIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIS_A {
        match self.bits {
            false => CIS_A::_0,
            true => CIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIS_A::_1
    }
}
#[doc = "Write proxy for field `CIS`"]
pub struct CIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIS_A::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Context Register Output Byte Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COS_A {
    #[doc = "0: Do Not Byte Swap Data."]
    _0 = 0,
    #[doc = "1: Byte Swap Data."]
    _1 = 1,
}
impl From<COS_A> for bool {
    #[inline(always)]
    fn from(variant: COS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COS`"]
pub type COS_R = crate::R<bool, COS_A>;
impl COS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COS_A {
        match self.bits {
            false => COS_A::_0,
            true => COS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COS_A::_1
    }
}
#[doc = "Write proxy for field `COS`"]
pub struct COS_W<'a> {
    w: &'a mut W,
}
impl<'a> COS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do Not Byte Swap Data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COS_A::_0)
    }
    #[doc = "Byte Swap Data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Key Register Access Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KAL_A {
    #[doc = "0: Key Register is readable."]
    _0 = 0,
    #[doc = "1: Key Register is not readable."]
    _1 = 1,
}
impl From<KAL_A> for bool {
    #[inline(always)]
    fn from(variant: KAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KAL`"]
pub type KAL_R = crate::R<bool, KAL_A>;
impl KAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KAL_A {
        match self.bits {
            false => KAL_A::_0,
            true => KAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KAL_A::_1
    }
}
#[doc = "Write proxy for field `KAL`"]
pub struct KAL_W<'a> {
    w: &'a mut W,
}
impl<'a> KAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Key Register is readable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KAL_A::_0)
    }
    #[doc = "Key Register is not readable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KAL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Mask. Once this bit is set, it can only be cleared by hard reset."]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - PKHA Register DMA Enable."]
    #[inline(always)]
    pub fn pde(&self) -> PDE_R {
        PDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Input FIFO DMA Enable."]
    #[inline(always)]
    pub fn ife(&self) -> IFE_R {
        IFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Input FIFO DMA Request Size"]
    #[inline(always)]
    pub fn ifr(&self) -> IFR_R {
        IFR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output FIFO DMA Enable."]
    #[inline(always)]
    pub fn ofe(&self) -> OFE_R {
        OFE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Output FIFO DMA Request Size"]
    #[inline(always)]
    pub fn ofr(&self) -> OFR_R {
        OFR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input FIFO Byte Swap. Byte swap all data that is written to the Input FIFO."]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output FIFO Byte Swap. Byte swap all data that is read from the Onput FIFO."]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Key Register Input Byte Swap"]
    #[inline(always)]
    pub fn kis(&self) -> KIS_R {
        KIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Key Register Output Byte Swap"]
    #[inline(always)]
    pub fn kos(&self) -> KOS_R {
        KOS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Context Register Input Byte Swap"]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Context Register Output Byte Swap"]
    #[inline(always)]
    pub fn cos(&self) -> COS_R {
        COS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Key Register Access Lock"]
    #[inline(always)]
    pub fn kal(&self) -> KAL_R {
        KAL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask. Once this bit is set, it can only be cleared by hard reset."]
    #[inline(always)]
    pub fn im(&mut self) -> IM_W {
        IM_W { w: self }
    }
    #[doc = "Bit 4 - PKHA Register DMA Enable."]
    #[inline(always)]
    pub fn pde(&mut self) -> PDE_W {
        PDE_W { w: self }
    }
    #[doc = "Bit 8 - Input FIFO DMA Enable."]
    #[inline(always)]
    pub fn ife(&mut self) -> IFE_W {
        IFE_W { w: self }
    }
    #[doc = "Bit 9 - Input FIFO DMA Request Size"]
    #[inline(always)]
    pub fn ifr(&mut self) -> IFR_W {
        IFR_W { w: self }
    }
    #[doc = "Bit 12 - Output FIFO DMA Enable."]
    #[inline(always)]
    pub fn ofe(&mut self) -> OFE_W {
        OFE_W { w: self }
    }
    #[doc = "Bit 13 - Output FIFO DMA Request Size"]
    #[inline(always)]
    pub fn ofr(&mut self) -> OFR_W {
        OFR_W { w: self }
    }
    #[doc = "Bit 16 - Input FIFO Byte Swap. Byte swap all data that is written to the Input FIFO."]
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W {
        IFS_W { w: self }
    }
    #[doc = "Bit 17 - Output FIFO Byte Swap. Byte swap all data that is read from the Onput FIFO."]
    #[inline(always)]
    pub fn ofs(&mut self) -> OFS_W {
        OFS_W { w: self }
    }
    #[doc = "Bit 20 - Key Register Input Byte Swap"]
    #[inline(always)]
    pub fn kis(&mut self) -> KIS_W {
        KIS_W { w: self }
    }
    #[doc = "Bit 21 - Key Register Output Byte Swap"]
    #[inline(always)]
    pub fn kos(&mut self) -> KOS_W {
        KOS_W { w: self }
    }
    #[doc = "Bit 22 - Context Register Input Byte Swap"]
    #[inline(always)]
    pub fn cis(&mut self) -> CIS_W {
        CIS_W { w: self }
    }
    #[doc = "Bit 23 - Context Register Output Byte Swap"]
    #[inline(always)]
    pub fn cos(&mut self) -> COS_W {
        COS_W { w: self }
    }
    #[doc = "Bit 31 - Key Register Access Lock"]
    #[inline(always)]
    pub fn kal(&mut self) -> KAL_W {
        KAL_W { w: self }
    }
}
