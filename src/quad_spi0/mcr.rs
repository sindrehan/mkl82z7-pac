#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0x000f_400c"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000f_400c
    }
}
#[doc = "Software reset for serial flash domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTSD_A {
    #[doc = "0: No action"]
    _0 = 0,
    #[doc = "1: Serial Flash domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\]
should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTSD\\]
to 0), it is recommended to set the MCR\\[MDIS\\]
bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\]
bit to 0."]
    _1 = 1,
}
impl From<SWRSTSD_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRSTSD`"]
pub type SWRSTSD_R = crate::R<bool, SWRSTSD_A>;
impl SWRSTSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTSD_A {
        match self.bits {
            false => SWRSTSD_A::_0,
            true => SWRSTSD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRSTSD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRSTSD_A::_1
    }
}
#[doc = "Write proxy for field `SWRSTSD`"]
pub struct SWRSTSD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTSD_A::_0)
    }
    #[doc = "Serial Flash domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\]
should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTSD\\]
to 0), it is recommended to set the MCR\\[MDIS\\]
bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\]
bit to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTSD_A::_1)
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
#[doc = "Software reset for AHB domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTHD_A {
    #[doc = "0: No action"]
    _0 = 0,
    #[doc = "1: AHB domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\]
should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTHD\\]
to 0), it is recommended to set the MCR\\[MDIS\\]
bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\]
bit to 0."]
    _1 = 1,
}
impl From<SWRSTHD_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTHD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRSTHD`"]
pub type SWRSTHD_R = crate::R<bool, SWRSTHD_A>;
impl SWRSTHD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTHD_A {
        match self.bits {
            false => SWRSTHD_A::_0,
            true => SWRSTHD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRSTHD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRSTHD_A::_1
    }
}
#[doc = "Write proxy for field `SWRSTHD`"]
pub struct SWRSTHD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTHD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTHD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTHD_A::_0)
    }
    #[doc = "AHB domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\]
should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTHD\\]
to 0), it is recommended to set the MCR\\[MDIS\\]
bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\]
bit to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTHD_A::_1)
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
#[doc = "Reader of field `END_CFG`"]
pub type END_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `END_CFG`"]
pub struct END_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> END_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "DQS Latency Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQS_LAT_EN_A {
    #[doc = "0: DQS Latency disabled"]
    _0 = 0,
    #[doc = "1: DQS feature with latency included enabled"]
    _1 = 1,
}
impl From<DQS_LAT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DQS_LAT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DQS_LAT_EN`"]
pub type DQS_LAT_EN_R = crate::R<bool, DQS_LAT_EN_A>;
impl DQS_LAT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQS_LAT_EN_A {
        match self.bits {
            false => DQS_LAT_EN_A::_0,
            true => DQS_LAT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DQS_LAT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DQS_LAT_EN_A::_1
    }
}
#[doc = "Write proxy for field `DQS_LAT_EN`"]
pub struct DQS_LAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQS_LAT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQS_LAT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DQS Latency disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQS_LAT_EN_A::_0)
    }
    #[doc = "DQS feature with latency included enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQS_LAT_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "DQS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQS_EN_A {
    #[doc = "0: DQS disabled."]
    _0 = 0,
    #[doc = "1: DQS enabled. When enabled, the incoming data is sampled on both the edges of DQS input when QSPI_MCR\\[DDR_EN\\]
is set, else, on only one edge when QSPI_MCR\\[DDR_EN\\]
is 0. The QSPI_SMPR\\[DDR_SMP\\]
values are ignored."]
    _1 = 1,
}
impl From<DQS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DQS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DQS_EN`"]
pub type DQS_EN_R = crate::R<bool, DQS_EN_A>;
impl DQS_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQS_EN_A {
        match self.bits {
            false => DQS_EN_A::_0,
            true => DQS_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DQS_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DQS_EN_A::_1
    }
}
#[doc = "Write proxy for field `DQS_EN`"]
pub struct DQS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQS_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DQS disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQS_EN_A::_0)
    }
    #[doc = "DQS enabled. When enabled, the incoming data is sampled on both the edges of DQS input when QSPI_MCR\\[DDR_EN\\]
is set, else, on only one edge when QSPI_MCR\\[DDR_EN\\]
is 0. The QSPI_SMPR\\[DDR_SMP\\]
values are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQS_EN_A::_1)
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
#[doc = "DDR mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR_EN_A {
    #[doc = "0: 2x and 4x clocks are disabled for SDR instructions only"]
    _0 = 0,
    #[doc = "1: 2x and 4x clocks are enabled supports both SDR and DDR instruction."]
    _1 = 1,
}
impl From<DDR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DDR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDR_EN`"]
pub type DDR_EN_R = crate::R<bool, DDR_EN_A>;
impl DDR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDR_EN_A {
        match self.bits {
            false => DDR_EN_A::_0,
            true => DDR_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DDR_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DDR_EN_A::_1
    }
}
#[doc = "Write proxy for field `DDR_EN`"]
pub struct DDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "2x and 4x clocks are disabled for SDR instructions only"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DDR_EN_A::_0)
    }
    #[doc = "2x and 4x clocks are enabled supports both SDR and DDR instruction."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DDR_EN_A::_1)
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
#[doc = "Clear RX FIFO. Invalidates the RX Buffer. This is a self-clearing field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_RXF_AW {
    #[doc = "0: No action."]
    _0 = 0,
    #[doc = "1: Read and write pointers of the RX Buffer are reset to 0. QSPI_RBSR\\[RDBFL\\]
is reset to 0."]
    _1 = 1,
}
impl From<CLR_RXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_RXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLR_RXF`"]
pub struct CLR_RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_RXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_RXF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_0)
    }
    #[doc = "Read and write pointers of the RX Buffer are reset to 0. QSPI_RBSR\\[RDBFL\\]
is reset to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Clear TX FIFO/Buffer. Invalidates the TX Buffer content. This is a self-clearing field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_TXF_AW {
    #[doc = "0: No action."]
    _0 = 0,
    #[doc = "1: Read and write pointers of the TX Buffer are reset to 0. QSPI_TBSR\\[TRCTR\\]
is reset to 0."]
    _1 = 1,
}
impl From<CLR_TXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_TXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLR_TXF`"]
pub struct CLR_TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_TXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_TXF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_0)
    }
    #[doc = "Read and write pointers of the TX Buffer are reset to 0. QSPI_TBSR\\[TRCTR\\]
is reset to 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Enable QuadSPI clocks."]
    _0 = 0,
    #[doc = "1: Allow external logic to disable QuadSPI clocks."]
    _1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, MDIS_A>;
impl MDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::_0,
            true => MDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDIS_A::_1
    }
}
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable QuadSPI clocks."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Allow external logic to disable QuadSPI clocks."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SCLKCFG`"]
pub type SCLKCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLKCFG`"]
pub struct SCLKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLKCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software reset for serial flash domain"]
    #[inline(always)]
    pub fn swrstsd(&self) -> SWRSTSD_R {
        SWRSTSD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software reset for AHB domain"]
    #[inline(always)]
    pub fn swrsthd(&self) -> SWRSTHD_R {
        SWRSTHD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Defines the endianness of the QuadSPI module. For more details refer to Byte Ordering Endianess"]
    #[inline(always)]
    pub fn end_cfg(&self) -> END_CFG_R {
        END_CFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - DQS Latency Enable"]
    #[inline(always)]
    pub fn dqs_lat_en(&self) -> DQS_LAT_EN_R {
        DQS_LAT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DQS enable"]
    #[inline(always)]
    pub fn dqs_en(&self) -> DQS_EN_R {
        DQS_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DDR mode enable"]
    #[inline(always)]
    pub fn ddr_en(&self) -> DDR_EN_R {
        DDR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Serial Clock Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&self) -> SCLKCFG_R {
        SCLKCFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset for serial flash domain"]
    #[inline(always)]
    pub fn swrstsd(&mut self) -> SWRSTSD_W {
        SWRSTSD_W { w: self }
    }
    #[doc = "Bit 1 - Software reset for AHB domain"]
    #[inline(always)]
    pub fn swrsthd(&mut self) -> SWRSTHD_W {
        SWRSTHD_W { w: self }
    }
    #[doc = "Bits 2:3 - Defines the endianness of the QuadSPI module. For more details refer to Byte Ordering Endianess"]
    #[inline(always)]
    pub fn end_cfg(&mut self) -> END_CFG_W {
        END_CFG_W { w: self }
    }
    #[doc = "Bit 5 - DQS Latency Enable"]
    #[inline(always)]
    pub fn dqs_lat_en(&mut self) -> DQS_LAT_EN_W {
        DQS_LAT_EN_W { w: self }
    }
    #[doc = "Bit 6 - DQS enable"]
    #[inline(always)]
    pub fn dqs_en(&mut self) -> DQS_EN_W {
        DQS_EN_W { w: self }
    }
    #[doc = "Bit 7 - DDR mode enable"]
    #[inline(always)]
    pub fn ddr_en(&mut self) -> DDR_EN_W {
        DDR_EN_W { w: self }
    }
    #[doc = "Bit 10 - Clear RX FIFO. Invalidates the RX Buffer. This is a self-clearing field."]
    #[inline(always)]
    pub fn clr_rxf(&mut self) -> CLR_RXF_W {
        CLR_RXF_W { w: self }
    }
    #[doc = "Bit 11 - Clear TX FIFO/Buffer. Invalidates the TX Buffer content. This is a self-clearing field."]
    #[inline(always)]
    pub fn clr_txf(&mut self) -> CLR_TXF_W {
        CLR_TXF_W { w: self }
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
    #[doc = "Bits 24:31 - Serial Clock Configuration"]
    #[inline(always)]
    pub fn sclkcfg(&mut self) -> SCLKCFG_W {
        SCLKCFG_W { w: self }
    }
}
