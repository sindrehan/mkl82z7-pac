#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMCFG {
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
#[doc = "Possible values of the field `TSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTARTR {
    #[doc = "Start bit disabled"]
    _0,
    #[doc = "Start bit enabled"]
    _1,
}
impl TSTARTR {
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
            TSTARTR::_0 => false,
            TSTARTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSTARTR {
        match value {
            false => TSTARTR::_0,
            true => TSTARTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSTARTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSTARTR::_1
    }
}
#[doc = "Possible values of the field `TSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTOPR {
    #[doc = "Stop bit disabled"]
    _00,
    #[doc = "Stop bit is enabled on timer compare"]
    _01,
    #[doc = "Stop bit is enabled on timer disable"]
    _10,
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    _11,
}
impl TSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTOPR::_00 => 0,
            TSTOPR::_01 => 1,
            TSTOPR::_10 => 2,
            TSTOPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTOPR {
        match value {
            0 => TSTOPR::_00,
            1 => TSTOPR::_01,
            2 => TSTOPR::_10,
            3 => TSTOPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TSTOPR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TSTOPR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TSTOPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TSTOPR::_11
    }
}
#[doc = "Possible values of the field `TIMENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMENAR {
    #[doc = "Timer always enabled"]
    _000,
    #[doc = "Timer enabled on Timer N-1 enable"]
    _001,
    #[doc = "Timer enabled on Trigger high"]
    _010,
    #[doc = "Timer enabled on Trigger high and Pin high"]
    _011,
    #[doc = "Timer enabled on Pin rising edge"]
    _100,
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    _101,
    #[doc = "Timer enabled on Trigger rising edge"]
    _110,
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    _111,
}
impl TIMENAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMENAR::_000 => 0,
            TIMENAR::_001 => 1,
            TIMENAR::_010 => 2,
            TIMENAR::_011 => 3,
            TIMENAR::_100 => 4,
            TIMENAR::_101 => 5,
            TIMENAR::_110 => 6,
            TIMENAR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMENAR {
        match value {
            0 => TIMENAR::_000,
            1 => TIMENAR::_001,
            2 => TIMENAR::_010,
            3 => TIMENAR::_011,
            4 => TIMENAR::_100,
            5 => TIMENAR::_101,
            6 => TIMENAR::_110,
            7 => TIMENAR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TIMENAR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == TIMENAR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TIMENAR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TIMENAR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TIMENAR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TIMENAR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TIMENAR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TIMENAR::_111
    }
}
#[doc = "Possible values of the field `TIMDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMDISR {
    #[doc = "Timer never disabled"]
    _000,
    #[doc = "Timer disabled on Timer N-1 disable"]
    _001,
    #[doc = "Timer disabled on Timer compare"]
    _010,
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    _011,
    #[doc = "Timer disabled on Pin rising or falling edge"]
    _100,
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    _101,
    #[doc = "Timer disabled on Trigger falling edge"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMDISR::_000 => 0,
            TIMDISR::_001 => 1,
            TIMDISR::_010 => 2,
            TIMDISR::_011 => 3,
            TIMDISR::_100 => 4,
            TIMDISR::_101 => 5,
            TIMDISR::_110 => 6,
            TIMDISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMDISR {
        match value {
            0 => TIMDISR::_000,
            1 => TIMDISR::_001,
            2 => TIMDISR::_010,
            3 => TIMDISR::_011,
            4 => TIMDISR::_100,
            5 => TIMDISR::_101,
            6 => TIMDISR::_110,
            i => TIMDISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TIMDISR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == TIMDISR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TIMDISR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TIMDISR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TIMDISR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TIMDISR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TIMDISR::_110
    }
}
#[doc = "Possible values of the field `TIMRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMRSTR {
    #[doc = "Timer never reset"]
    _000,
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    _010,
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    _011,
    #[doc = "Timer reset on Timer Pin rising edge"]
    _100,
    #[doc = "Timer reset on Trigger rising edge"]
    _110,
    #[doc = "Timer reset on Trigger rising or falling edge"]
    _111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMRSTR::_000 => 0,
            TIMRSTR::_010 => 2,
            TIMRSTR::_011 => 3,
            TIMRSTR::_100 => 4,
            TIMRSTR::_110 => 6,
            TIMRSTR::_111 => 7,
            TIMRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMRSTR {
        match value {
            0 => TIMRSTR::_000,
            2 => TIMRSTR::_010,
            3 => TIMRSTR::_011,
            4 => TIMRSTR::_100,
            6 => TIMRSTR::_110,
            7 => TIMRSTR::_111,
            i => TIMRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TIMRSTR::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TIMRSTR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TIMRSTR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TIMRSTR::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TIMRSTR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TIMRSTR::_111
    }
}
#[doc = "Possible values of the field `TIMDEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMDECR {
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    _00,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    _01,
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    _10,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    _11,
}
impl TIMDECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMDECR::_00 => 0,
            TIMDECR::_01 => 1,
            TIMDECR::_10 => 2,
            TIMDECR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMDECR {
        match value {
            0 => TIMDECR::_00,
            1 => TIMDECR::_01,
            2 => TIMDECR::_10,
            3 => TIMDECR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TIMDECR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TIMDECR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMDECR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMDECR::_11
    }
}
#[doc = "Possible values of the field `TIMOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMOUTR {
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    _00,
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    _01,
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    _10,
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    _11,
}
impl TIMOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMOUTR::_00 => 0,
            TIMOUTR::_01 => 1,
            TIMOUTR::_10 => 2,
            TIMOUTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMOUTR {
        match value {
            0 => TIMOUTR::_00,
            1 => TIMOUTR::_01,
            2 => TIMOUTR::_10,
            3 => TIMOUTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TIMOUTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TIMOUTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMOUTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMOUTR::_11
    }
}
#[doc = "Values that can be written to the field `TSTART`"]
pub enum TSTARTW {
    #[doc = "Start bit disabled"]
    _0,
    #[doc = "Start bit enabled"]
    _1,
}
impl TSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSTARTW::_0 => false,
            TSTARTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start bit disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTARTW::_0)
    }
    #[doc = "Start bit enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTARTW::_1)
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
#[doc = "Values that can be written to the field `TSTOP`"]
pub enum TSTOPW {
    #[doc = "Stop bit disabled"]
    _00,
    #[doc = "Stop bit is enabled on timer compare"]
    _01,
    #[doc = "Stop bit is enabled on timer disable"]
    _10,
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    _11,
}
impl TSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTOPW::_00 => 0,
            TSTOPW::_01 => 1,
            TSTOPW::_10 => 2,
            TSTOPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTOPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Stop bit disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TSTOPW::_00)
    }
    #[doc = "Stop bit is enabled on timer compare"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TSTOPW::_01)
    }
    #[doc = "Stop bit is enabled on timer disable"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TSTOPW::_10)
    }
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TSTOPW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMENA`"]
pub enum TIMENAW {
    #[doc = "Timer always enabled"]
    _000,
    #[doc = "Timer enabled on Timer N-1 enable"]
    _001,
    #[doc = "Timer enabled on Trigger high"]
    _010,
    #[doc = "Timer enabled on Trigger high and Pin high"]
    _011,
    #[doc = "Timer enabled on Pin rising edge"]
    _100,
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    _101,
    #[doc = "Timer enabled on Trigger rising edge"]
    _110,
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    _111,
}
impl TIMENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMENAW::_000 => 0,
            TIMENAW::_001 => 1,
            TIMENAW::_010 => 2,
            TIMENAW::_011 => 3,
            TIMENAW::_100 => 4,
            TIMENAW::_101 => 5,
            TIMENAW::_110 => 6,
            TIMENAW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMENAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer always enabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(TIMENAW::_000)
    }
    #[doc = "Timer enabled on Timer N-1 enable"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(TIMENAW::_001)
    }
    #[doc = "Timer enabled on Trigger high"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(TIMENAW::_010)
    }
    #[doc = "Timer enabled on Trigger high and Pin high"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(TIMENAW::_011)
    }
    #[doc = "Timer enabled on Pin rising edge"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMENAW::_100)
    }
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TIMENAW::_101)
    }
    #[doc = "Timer enabled on Trigger rising edge"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMENAW::_110)
    }
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TIMENAW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMDIS`"]
pub enum TIMDISW {
    #[doc = "Timer never disabled"]
    _000,
    #[doc = "Timer disabled on Timer N-1 disable"]
    _001,
    #[doc = "Timer disabled on Timer compare"]
    _010,
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    _011,
    #[doc = "Timer disabled on Pin rising or falling edge"]
    _100,
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    _101,
    #[doc = "Timer disabled on Trigger falling edge"]
    _110,
}
impl TIMDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMDISW::_000 => 0,
            TIMDISW::_001 => 1,
            TIMDISW::_010 => 2,
            TIMDISW::_011 => 3,
            TIMDISW::_100 => 4,
            TIMDISW::_101 => 5,
            TIMDISW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMDISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer never disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(TIMDISW::_000)
    }
    #[doc = "Timer disabled on Timer N-1 disable"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(TIMDISW::_001)
    }
    #[doc = "Timer disabled on Timer compare"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(TIMDISW::_010)
    }
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(TIMDISW::_011)
    }
    #[doc = "Timer disabled on Pin rising or falling edge"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMDISW::_100)
    }
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TIMDISW::_101)
    }
    #[doc = "Timer disabled on Trigger falling edge"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMDISW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMRST`"]
pub enum TIMRSTW {
    #[doc = "Timer never reset"]
    _000,
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    _010,
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    _011,
    #[doc = "Timer reset on Timer Pin rising edge"]
    _100,
    #[doc = "Timer reset on Trigger rising edge"]
    _110,
    #[doc = "Timer reset on Trigger rising or falling edge"]
    _111,
}
impl TIMRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMRSTW::_000 => 0,
            TIMRSTW::_010 => 2,
            TIMRSTW::_011 => 3,
            TIMRSTW::_100 => 4,
            TIMRSTW::_110 => 6,
            TIMRSTW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMRSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer never reset"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(TIMRSTW::_000)
    }
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(TIMRSTW::_010)
    }
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(TIMRSTW::_011)
    }
    #[doc = "Timer reset on Timer Pin rising edge"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMRSTW::_100)
    }
    #[doc = "Timer reset on Trigger rising edge"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMRSTW::_110)
    }
    #[doc = "Timer reset on Trigger rising or falling edge"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TIMRSTW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMDEC`"]
pub enum TIMDECW {
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    _00,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    _01,
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    _10,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    _11,
}
impl TIMDECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMDECW::_00 => 0,
            TIMDECW::_01 => 1,
            TIMDECW::_10 => 2,
            TIMDECW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMDECW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMDECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMDECW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TIMDECW::_00)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TIMDECW::_01)
    }
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMDECW::_10)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMDECW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMOUT`"]
pub enum TIMOUTW {
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    _00,
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    _01,
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    _10,
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    _11,
}
impl TIMOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMOUTW::_00 => 0,
            TIMOUTW::_01 => 1,
            TIMOUTW::_10 => 2,
            TIMOUTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TIMOUTW::_00)
    }
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TIMOUTW::_01)
    }
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMOUTW::_10)
    }
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMOUTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline]
    pub fn tstart(&self) -> TSTARTR {
        TSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline]
    pub fn tstop(&self) -> TSTOPR {
        TSTOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline]
    pub fn timena(&self) -> TIMENAR {
        TIMENAR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline]
    pub fn timdis(&self) -> TIMDISR {
        TIMDISR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline]
    pub fn timrst(&self) -> TIMRSTR {
        TIMRSTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline]
    pub fn timdec(&self) -> TIMDECR {
        TIMDECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline]
    pub fn timout(&self) -> TIMOUTR {
        TIMOUTR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline]
    pub fn tstart(&mut self) -> _TSTARTW {
        _TSTARTW { w: self }
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline]
    pub fn tstop(&mut self) -> _TSTOPW {
        _TSTOPW { w: self }
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline]
    pub fn timena(&mut self) -> _TIMENAW {
        _TIMENAW { w: self }
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline]
    pub fn timdis(&mut self) -> _TIMDISW {
        _TIMDISW { w: self }
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline]
    pub fn timrst(&mut self) -> _TIMRSTW {
        _TIMRSTW { w: self }
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline]
    pub fn timdec(&mut self) -> _TIMDECW {
        _TIMDECW { w: self }
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline]
    pub fn timout(&mut self) -> _TIMOUTW {
        _TIMOUTW { w: self }
    }
}
