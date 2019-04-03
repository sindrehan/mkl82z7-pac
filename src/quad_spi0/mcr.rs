#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SWRSTSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTSDR {
    #[doc = "No action"]
    _0,
    #[doc = "Serial Flash domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\] should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTSD\\] to 0), it is recommended to set the MCR\\[MDIS\\] bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\] bit to 0."]
    _1,
}
impl SWRSTSDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SWRSTSDR::_0 => false,
            SWRSTSDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTSDR {
        match value {
            false => SWRSTSDR::_0,
            true => SWRSTSDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWRSTSDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWRSTSDR::_1
    }
}
#[doc = "Possible values of the field `SWRSTHD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTHDR {
    #[doc = "No action"]
    _0,
    #[doc = "AHB domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\] should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTHD\\] to 0), it is recommended to set the MCR\\[MDIS\\] bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\] bit to 0."]
    _1,
}
impl SWRSTHDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SWRSTHDR::_0 => false,
            SWRSTHDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTHDR {
        match value {
            false => SWRSTHDR::_0,
            true => SWRSTHDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWRSTHDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWRSTHDR::_1
    }
}
#[doc = r" Value of the field"]
pub struct END_CFGR {
    bits: u8,
}
impl END_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DQS_LAT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQS_LAT_ENR {
    #[doc = "DQS Latency disabled"]
    _0,
    #[doc = "DQS feature with latency included enabled"]
    _1,
}
impl DQS_LAT_ENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DQS_LAT_ENR::_0 => false,
            DQS_LAT_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DQS_LAT_ENR {
        match value {
            false => DQS_LAT_ENR::_0,
            true => DQS_LAT_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DQS_LAT_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DQS_LAT_ENR::_1
    }
}
#[doc = "Possible values of the field `DQS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQS_ENR {
    #[doc = "DQS disabled."]
    _0,
    #[doc = "DQS enabled. When enabled, the incoming data is sampled on both the edges of DQS input when QSPI_MCR\\[DDR_EN\\] is set, else, on only one edge when QSPI_MCR\\[DDR_EN\\] is 0. The QSPI_SMPR\\[DDR_SMP\\] values are ignored."]
    _1,
}
impl DQS_ENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DQS_ENR::_0 => false,
            DQS_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DQS_ENR {
        match value {
            false => DQS_ENR::_0,
            true => DQS_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DQS_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DQS_ENR::_1
    }
}
#[doc = "Possible values of the field `DDR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR_ENR {
    #[doc = "2x and 4x clocks are disabled for SDR instructions only"]
    _0,
    #[doc = "2x and 4x clocks are enabled supports both SDR and DDR instruction."]
    _1,
}
impl DDR_ENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DDR_ENR::_0 => false,
            DDR_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DDR_ENR {
        match value {
            false => DDR_ENR::_0,
            true => DDR_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DDR_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DDR_ENR::_1
    }
}
#[doc = "Possible values of the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISR {
    #[doc = "Enable QuadSPI clocks."]
    _0,
    #[doc = "Allow external logic to disable QuadSPI clocks."]
    _1,
}
impl MDISR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MDISR::_0 => false,
            MDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDISR {
        match value {
            false => MDISR::_0,
            true => MDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MDISR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SCLKCFGR {
    bits: u8,
}
impl SCLKCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SWRSTSD`"]
pub enum SWRSTSDW {
    #[doc = "No action"]
    _0,
    #[doc = "Serial Flash domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\] should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTSD\\] to 0), it is recommended to set the MCR\\[MDIS\\] bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\] bit to 0."]
    _1,
}
impl SWRSTSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTSDW::_0 => false,
            SWRSTSDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTSDW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTSDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTSDW::_0)
    }
    #[doc = "Serial Flash domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\] should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTSD\\] to 0), it is recommended to set the MCR\\[MDIS\\] bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\] bit to 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTSDW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWRSTHD`"]
pub enum SWRSTHDW {
    #[doc = "No action"]
    _0,
    #[doc = "AHB domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\] should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTHD\\] to 0), it is recommended to set the MCR\\[MDIS\\] bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\] bit to 0."]
    _1,
}
impl SWRSTHDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTHDW::_0 => false,
            SWRSTHDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTHDW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTHDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTHDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTHDW::_0)
    }
    #[doc = "AHB domain flops are reset. Does not reset configuration registers. It is advisable to reset both the serial flash domain and AHB domain at the same time. Resetting only one domain might lead to side effects. The software resets need the clock to be running to propagate to the design. The MCR\\[MDIS\\] should therefore be set to 0 when the software reset bits are asserted. Also, before they can be deasserted again (by setting MCR\\[SWRSTHD\\] to 0), it is recommended to set the MCR\\[MDIS\\] bit to 1. Once the software resets have been deasserted, the normal operation can be started by setting the MCR\\[MDIS\\] bit to 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTHDW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _END_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _END_CFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DQS_LAT_EN`"]
pub enum DQS_LAT_ENW {
    #[doc = "DQS Latency disabled"]
    _0,
    #[doc = "DQS feature with latency included enabled"]
    _1,
}
impl DQS_LAT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DQS_LAT_ENW::_0 => false,
            DQS_LAT_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DQS_LAT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DQS_LAT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DQS_LAT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DQS Latency disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQS_LAT_ENW::_0)
    }
    #[doc = "DQS feature with latency included enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQS_LAT_ENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DQS_EN`"]
pub enum DQS_ENW {
    #[doc = "DQS disabled."]
    _0,
    #[doc = "DQS enabled. When enabled, the incoming data is sampled on both the edges of DQS input when QSPI_MCR\\[DDR_EN\\] is set, else, on only one edge when QSPI_MCR\\[DDR_EN\\] is 0. The QSPI_SMPR\\[DDR_SMP\\] values are ignored."]
    _1,
}
impl DQS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DQS_ENW::_0 => false,
            DQS_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DQS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DQS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DQS_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DQS disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DQS_ENW::_0)
    }
    #[doc = "DQS enabled. When enabled, the incoming data is sampled on both the edges of DQS input when QSPI_MCR\\[DDR_EN\\] is set, else, on only one edge when QSPI_MCR\\[DDR_EN\\] is 0. The QSPI_SMPR\\[DDR_SMP\\] values are ignored."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DQS_ENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DDR_EN`"]
pub enum DDR_ENW {
    #[doc = "2x and 4x clocks are disabled for SDR instructions only"]
    _0,
    #[doc = "2x and 4x clocks are enabled supports both SDR and DDR instruction."]
    _1,
}
impl DDR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DDR_ENW::_0 => false,
            DDR_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DDR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DDR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "2x and 4x clocks are disabled for SDR instructions only"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DDR_ENW::_0)
    }
    #[doc = "2x and 4x clocks are enabled supports both SDR and DDR instruction."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DDR_ENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLR_RXF`"]
pub enum CLR_RXFW {
    #[doc = "No action."]
    _0,
    #[doc = "Read and write pointers of the RX Buffer are reset to 0. QSPI_RBSR\\[RDBFL\\] is reset to 0."]
    _1,
}
impl CLR_RXFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR_RXFW::_0 => false,
            CLR_RXFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_RXFW<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_RXFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_RXFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_RXFW::_0)
    }
    #[doc = "Read and write pointers of the RX Buffer are reset to 0. QSPI_RBSR\\[RDBFL\\] is reset to 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_RXFW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLR_TXF`"]
pub enum CLR_TXFW {
    #[doc = "No action."]
    _0,
    #[doc = "Read and write pointers of the TX Buffer are reset to 0. QSPI_TBSR\\[TRCTR\\] is reset to 0."]
    _1,
}
impl CLR_TXFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR_TXFW::_0 => false,
            CLR_TXFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_TXFW<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_TXFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_TXFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_TXFW::_0)
    }
    #[doc = "Read and write pointers of the TX Buffer are reset to 0. QSPI_TBSR\\[TRCTR\\] is reset to 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_TXFW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDIS`"]
pub enum MDISW {
    #[doc = "Enable QuadSPI clocks."]
    _0,
    #[doc = "Allow external logic to disable QuadSPI clocks."]
    _1,
}
impl MDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDISW::_0 => false,
            MDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable QuadSPI clocks."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDISW::_0)
    }
    #[doc = "Allow external logic to disable QuadSPI clocks."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDISW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCLKCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLKCFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software reset for serial flash domain"]
    #[inline]
    pub fn swrstsd(&self) -> SWRSTSDR {
        SWRSTSDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software reset for AHB domain"]
    #[inline]
    pub fn swrsthd(&self) -> SWRSTHDR {
        SWRSTHDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Defines the endianness of the QuadSPI module. For more details refer to Byte Ordering Endianess"]
    #[inline]
    pub fn end_cfg(&self) -> END_CFGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        END_CFGR { bits }
    }
    #[doc = "Bit 5 - DQS Latency Enable"]
    #[inline]
    pub fn dqs_lat_en(&self) -> DQS_LAT_ENR {
        DQS_LAT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - DQS enable"]
    #[inline]
    pub fn dqs_en(&self) -> DQS_ENR {
        DQS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - DDR mode enable"]
    #[inline]
    pub fn ddr_en(&self) -> DDR_ENR {
        DDR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline]
    pub fn mdis(&self) -> MDISR {
        MDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - Serial Clock Configuration"]
    #[inline]
    pub fn sclkcfg(&self) -> SCLKCFGR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCLKCFGR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 999436 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software reset for serial flash domain"]
    #[inline]
    pub fn swrstsd(&mut self) -> _SWRSTSDW {
        _SWRSTSDW { w: self }
    }
    #[doc = "Bit 1 - Software reset for AHB domain"]
    #[inline]
    pub fn swrsthd(&mut self) -> _SWRSTHDW {
        _SWRSTHDW { w: self }
    }
    #[doc = "Bits 2:3 - Defines the endianness of the QuadSPI module. For more details refer to Byte Ordering Endianess"]
    #[inline]
    pub fn end_cfg(&mut self) -> _END_CFGW {
        _END_CFGW { w: self }
    }
    #[doc = "Bit 5 - DQS Latency Enable"]
    #[inline]
    pub fn dqs_lat_en(&mut self) -> _DQS_LAT_ENW {
        _DQS_LAT_ENW { w: self }
    }
    #[doc = "Bit 6 - DQS enable"]
    #[inline]
    pub fn dqs_en(&mut self) -> _DQS_ENW {
        _DQS_ENW { w: self }
    }
    #[doc = "Bit 7 - DDR mode enable"]
    #[inline]
    pub fn ddr_en(&mut self) -> _DDR_ENW {
        _DDR_ENW { w: self }
    }
    #[doc = "Bit 10 - Clear RX FIFO. Invalidates the RX Buffer. This is a self-clearing field."]
    #[inline]
    pub fn clr_rxf(&mut self) -> _CLR_RXFW {
        _CLR_RXFW { w: self }
    }
    #[doc = "Bit 11 - Clear TX FIFO/Buffer. Invalidates the TX Buffer content. This is a self-clearing field."]
    #[inline]
    pub fn clr_txf(&mut self) -> _CLR_TXFW {
        _CLR_TXFW { w: self }
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline]
    pub fn mdis(&mut self) -> _MDISW {
        _MDISW { w: self }
    }
    #[doc = "Bits 24:31 - Serial Clock Configuration"]
    #[inline]
    pub fn sclkcfg(&mut self) -> _SCLKCFGW {
        _SCLKCFGW { w: self }
    }
}
