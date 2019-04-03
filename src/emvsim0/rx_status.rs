#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_STATUS {
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
#[doc = "Possible values of the field `RFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOR {
    #[doc = "No overrun error has occurred (default)"]
    _0,
    #[doc = "A byte was received when the received FIFO was already full"]
    _1,
}
impl RFOR {
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
            RFOR::_0 => false,
            RFOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFOR {
        match value {
            false => RFOR::_0,
            true => RFOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFOR::_1
    }
}
#[doc = "Possible values of the field `RX_DATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATAR {
    #[doc = "No new byte is received"]
    _0,
    #[doc = "New byte is received ans stored in Receive FIFO"]
    _1,
}
impl RX_DATAR {
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
            RX_DATAR::_0 => false,
            RX_DATAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DATAR {
        match value {
            false => RX_DATAR::_0,
            true => RX_DATAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_DATAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX_DATAR::_1
    }
}
#[doc = "Possible values of the field `RDTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDTFR {
    #[doc = "Number of unread bytes in receive FIFO less than the value set by RDT\\[3:0\\] (default)."]
    _0,
    #[doc = "Number of unread bytes in receive FIFO greater or than equal to value set by RDT\\[3:0\\]."]
    _1,
}
impl RDTFR {
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
            RDTFR::_0 => false,
            RDTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDTFR {
        match value {
            false => RDTFR::_0,
            true => RDTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDTFR::_1
    }
}
#[doc = "Possible values of the field `LRC_OK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRC_OKR {
    #[doc = "Current LRC value does not match remainder."]
    _0,
    #[doc = "Current calculated LRC value matches the expected result (i.e. zero)."]
    _1,
}
impl LRC_OKR {
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
            LRC_OKR::_0 => false,
            LRC_OKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRC_OKR {
        match value {
            false => LRC_OKR::_0,
            true => LRC_OKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LRC_OKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LRC_OKR::_1
    }
}
#[doc = "Possible values of the field `CRC_OK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_OKR {
    #[doc = "Current CRC value does not match remainder."]
    _0,
    #[doc = "Current calculated CRC value matches the expected result."]
    _1,
}
impl CRC_OKR {
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
            CRC_OKR::_0 => false,
            CRC_OKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_OKR {
        match value {
            false => CRC_OKR::_0,
            true => CRC_OKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC_OKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC_OKR::_1
    }
}
#[doc = "Possible values of the field `CWT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWT_ERRR {
    #[doc = "No CWT violation has occurred (default)."]
    _0,
    #[doc = "Time between two consecutive characters has exceeded the value in CHAR_WAIT."]
    _1,
}
impl CWT_ERRR {
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
            CWT_ERRR::_0 => false,
            CWT_ERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CWT_ERRR {
        match value {
            false => CWT_ERRR::_0,
            true => CWT_ERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CWT_ERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CWT_ERRR::_1
    }
}
#[doc = "Possible values of the field `RTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTER {
    #[doc = "Number of NACKs generated by the receiver is less than the value programmed in RTH\\[3:0\\]"]
    _0,
    #[doc = "Number of NACKs generated by the receiver is equal to the value programmed in RTH\\[3:0\\]"]
    _1,
}
impl RTER {
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
            RTER::_0 => false,
            RTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTER {
        match value {
            false => RTER::_0,
            true => RTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTER::_1
    }
}
#[doc = "Possible values of the field `BWT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWT_ERRR {
    #[doc = "Block wait time not exceeded"]
    _0,
    #[doc = "Block wait time was exceeded"]
    _1,
}
impl BWT_ERRR {
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
            BWT_ERRR::_0 => false,
            BWT_ERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWT_ERRR {
        match value {
            false => BWT_ERRR::_0,
            true => BWT_ERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWT_ERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWT_ERRR::_1
    }
}
#[doc = "Possible values of the field `BGT_ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGT_ERRR {
    #[doc = "Block guard time was sufficient"]
    _0,
    #[doc = "Block guard time was too small"]
    _1,
}
impl BGT_ERRR {
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
            BGT_ERRR::_0 => false,
            BGT_ERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGT_ERRR {
        match value {
            false => BGT_ERRR::_0,
            true => BGT_ERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BGT_ERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BGT_ERRR::_1
    }
}
#[doc = "Possible values of the field `PEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFR {
    #[doc = "No parity error detected"]
    _0,
    #[doc = "Parity error detected"]
    _1,
}
impl PEFR {
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
            PEFR::_0 => false,
            PEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEFR {
        match value {
            false => PEFR::_0,
            true => PEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEFR::_1
    }
}
#[doc = "Possible values of the field `FEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEFR {
    #[doc = "No frame error detected"]
    _0,
    #[doc = "Frame error detected"]
    _1,
}
impl FEFR {
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
            FEFR::_0 => false,
            FEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEFR {
        match value {
            false => FEFR::_0,
            true => FEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FEFR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_WPTRR {
    bits: u8,
}
impl RX_WPTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RX_CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CNTR {
    #[doc = "FIFO is emtpy"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_CNTR::_0 => 0,
            RX_CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_CNTR {
        match value {
            0 => RX_CNTR::_0,
            i => RX_CNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX_CNTR::_0
    }
}
#[doc = "Values that can be written to the field `RFO`"]
pub enum RFOW {
    #[doc = "No overrun error has occurred (default)"]
    _0,
    #[doc = "A byte was received when the received FIFO was already full"]
    _1,
}
impl RFOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFOW::_0 => false,
            RFOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFOW<'a> {
    w: &'a mut W,
}
impl<'a> _RFOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun error has occurred (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOW::_0)
    }
    #[doc = "A byte was received when the received FIFO was already full"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOW::_1)
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
#[doc = "Values that can be written to the field `RX_DATA`"]
pub enum RX_DATAW {
    #[doc = "No new byte is received"]
    _0,
    #[doc = "New byte is received ans stored in Receive FIFO"]
    _1,
}
impl RX_DATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_DATAW::_0 => false,
            RX_DATAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No new byte is received"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_DATAW::_0)
    }
    #[doc = "New byte is received ans stored in Receive FIFO"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_DATAW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CWT_ERR`"]
pub enum CWT_ERRW {
    #[doc = "No CWT violation has occurred (default)."]
    _0,
    #[doc = "Time between two consecutive characters has exceeded the value in CHAR_WAIT."]
    _1,
}
impl CWT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CWT_ERRW::_0 => false,
            CWT_ERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CWT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _CWT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CWT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No CWT violation has occurred (default)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWT_ERRW::_0)
    }
    #[doc = "Time between two consecutive characters has exceeded the value in CHAR_WAIT."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWT_ERRW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTE`"]
pub enum RTEW {
    #[doc = "Number of NACKs generated by the receiver is less than the value programmed in RTH\\[3:0\\]"]
    _0,
    #[doc = "Number of NACKs generated by the receiver is equal to the value programmed in RTH\\[3:0\\]"]
    _1,
}
impl RTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTEW::_0 => false,
            RTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Number of NACKs generated by the receiver is less than the value programmed in RTH\\[3:0\\]"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTEW::_0)
    }
    #[doc = "Number of NACKs generated by the receiver is equal to the value programmed in RTH\\[3:0\\]"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTEW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWT_ERR`"]
pub enum BWT_ERRW {
    #[doc = "Block wait time not exceeded"]
    _0,
    #[doc = "Block wait time was exceeded"]
    _1,
}
impl BWT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWT_ERRW::_0 => false,
            BWT_ERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _BWT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Block wait time not exceeded"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWT_ERRW::_0)
    }
    #[doc = "Block wait time was exceeded"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWT_ERRW::_1)
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
#[doc = "Values that can be written to the field `BGT_ERR`"]
pub enum BGT_ERRW {
    #[doc = "Block guard time was sufficient"]
    _0,
    #[doc = "Block guard time was too small"]
    _1,
}
impl BGT_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGT_ERRW::_0 => false,
            BGT_ERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGT_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _BGT_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGT_ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Block guard time was sufficient"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGT_ERRW::_0)
    }
    #[doc = "Block guard time was too small"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGT_ERRW::_1)
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
#[doc = "Values that can be written to the field `PEF`"]
pub enum PEFW {
    #[doc = "No parity error detected"]
    _0,
    #[doc = "Parity error detected"]
    _1,
}
impl PEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEFW::_0 => false,
            PEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEFW<'a> {
    w: &'a mut W,
}
impl<'a> _PEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEFW::_0)
    }
    #[doc = "Parity error detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEFW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEF`"]
pub enum FEFW {
    #[doc = "No frame error detected"]
    _0,
    #[doc = "Frame error detected"]
    _1,
}
impl FEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEFW::_0 => false,
            FEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEFW<'a> {
    w: &'a mut W,
}
impl<'a> _FEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No frame error detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEFW::_0)
    }
    #[doc = "Frame error detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEFW::_1)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - Receive FIFO Overflow Flag"]
    #[inline]
    pub fn rfo(&self) -> RFOR {
        RFOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receive Data Interrupt Flag"]
    #[inline]
    pub fn rx_data(&self) -> RX_DATAR {
        RX_DATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Receive Data Threshold Interrupt Flag"]
    #[inline]
    pub fn rdtf(&self) -> RDTFR {
        RDTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - LRC Check OK Flag"]
    #[inline]
    pub fn lrc_ok(&self) -> LRC_OKR {
        LRC_OKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CRC Check OK Flag"]
    #[inline]
    pub fn crc_ok(&self) -> CRC_OKR {
        CRC_OKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Character Wait Time Error Flag"]
    #[inline]
    pub fn cwt_err(&self) -> CWT_ERRR {
        CWT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Received NACK Threshold Error Flag"]
    #[inline]
    pub fn rte(&self) -> RTER {
        RTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Block Wait Time Error Flag"]
    #[inline]
    pub fn bwt_err(&self) -> BWT_ERRR {
        BWT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Block Guard Time Error Flag"]
    #[inline]
    pub fn bgt_err(&self) -> BGT_ERRR {
        BGT_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Parity Error Flag"]
    #[inline]
    pub fn pef(&self) -> PEFR {
        PEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Frame Error Flag"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        FEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Receive FIFO Write Pointer Value"]
    #[inline]
    pub fn rx_wptr(&self) -> RX_WPTRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_WPTRR { bits }
    }
    #[doc = "Bits 24:31 - Receive FIFO Byte Count"]
    #[inline]
    pub fn rx_cnt(&self) -> RX_CNTR {
        RX_CNTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Receive FIFO Overflow Flag"]
    #[inline]
    pub fn rfo(&mut self) -> _RFOW {
        _RFOW { w: self }
    }
    #[doc = "Bit 4 - Receive Data Interrupt Flag"]
    #[inline]
    pub fn rx_data(&mut self) -> _RX_DATAW {
        _RX_DATAW { w: self }
    }
    #[doc = "Bit 8 - Character Wait Time Error Flag"]
    #[inline]
    pub fn cwt_err(&mut self) -> _CWT_ERRW {
        _CWT_ERRW { w: self }
    }
    #[doc = "Bit 9 - Received NACK Threshold Error Flag"]
    #[inline]
    pub fn rte(&mut self) -> _RTEW {
        _RTEW { w: self }
    }
    #[doc = "Bit 10 - Block Wait Time Error Flag"]
    #[inline]
    pub fn bwt_err(&mut self) -> _BWT_ERRW {
        _BWT_ERRW { w: self }
    }
    #[doc = "Bit 11 - Block Guard Time Error Flag"]
    #[inline]
    pub fn bgt_err(&mut self) -> _BGT_ERRW {
        _BGT_ERRW { w: self }
    }
    #[doc = "Bit 12 - Parity Error Flag"]
    #[inline]
    pub fn pef(&mut self) -> _PEFW {
        _PEFW { w: self }
    }
    #[doc = "Bit 13 - Frame Error Flag"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
}
