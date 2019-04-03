#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT7 {
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
#[doc = "Possible values of the field `ADC0TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0TRGSELR {
    #[doc = "External trigger pin input (EXTRG)"]
    _0000,
    #[doc = "High speed comparator 0 output"]
    _0001,
    #[doc = "PIT trigger 0"]
    _0100,
    #[doc = "PIT trigger 1"]
    _0101,
    #[doc = "PIT trigger 2"]
    _0110,
    #[doc = "PIT trigger 3"]
    _0111,
    #[doc = "TPM0 trigger"]
    _1000,
    #[doc = "TPM1 trigger"]
    _1001,
    #[doc = "TPM2 trigger"]
    _1010,
    #[doc = "Low-power timer1 (LPTMR1) trigger"]
    _1011,
    #[doc = "RTC alarm"]
    _1100,
    #[doc = "RTC seconds"]
    _1101,
    #[doc = "Low-power timer (LPTMR) trigger"]
    _1110,
    #[doc = "TPM1 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADC0TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC0TRGSELR::_0000 => 0,
            ADC0TRGSELR::_0001 => 1,
            ADC0TRGSELR::_0100 => 4,
            ADC0TRGSELR::_0101 => 5,
            ADC0TRGSELR::_0110 => 6,
            ADC0TRGSELR::_0111 => 7,
            ADC0TRGSELR::_1000 => 8,
            ADC0TRGSELR::_1001 => 9,
            ADC0TRGSELR::_1010 => 10,
            ADC0TRGSELR::_1011 => 11,
            ADC0TRGSELR::_1100 => 12,
            ADC0TRGSELR::_1101 => 13,
            ADC0TRGSELR::_1110 => 14,
            ADC0TRGSELR::_1111 => 15,
            ADC0TRGSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC0TRGSELR {
        match value {
            0 => ADC0TRGSELR::_0000,
            1 => ADC0TRGSELR::_0001,
            4 => ADC0TRGSELR::_0100,
            5 => ADC0TRGSELR::_0101,
            6 => ADC0TRGSELR::_0110,
            7 => ADC0TRGSELR::_0111,
            8 => ADC0TRGSELR::_1000,
            9 => ADC0TRGSELR::_1001,
            10 => ADC0TRGSELR::_1010,
            11 => ADC0TRGSELR::_1011,
            12 => ADC0TRGSELR::_1100,
            13 => ADC0TRGSELR::_1101,
            14 => ADC0TRGSELR::_1110,
            15 => ADC0TRGSELR::_1111,
            i => ADC0TRGSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == ADC0TRGSELR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == ADC0TRGSELR::_0001
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == ADC0TRGSELR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == ADC0TRGSELR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == ADC0TRGSELR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == ADC0TRGSELR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == ADC0TRGSELR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == ADC0TRGSELR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == ADC0TRGSELR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == ADC0TRGSELR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == ADC0TRGSELR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == ADC0TRGSELR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == ADC0TRGSELR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == ADC0TRGSELR::_1111
    }
}
#[doc = "Possible values of the field `ADC0PRETRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0PRETRGSELR {
    #[doc = "Pre-trigger A"]
    _0,
    #[doc = "Pre-trigger B"]
    _1,
}
impl ADC0PRETRGSELR {
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
            ADC0PRETRGSELR::_0 => false,
            ADC0PRETRGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0PRETRGSELR {
        match value {
            false => ADC0PRETRGSELR::_0,
            true => ADC0PRETRGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADC0PRETRGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADC0PRETRGSELR::_1
    }
}
#[doc = "Values that can be written to the field `ADC0TRGSEL`"]
pub enum ADC0TRGSELW {
    #[doc = "External trigger pin input (EXTRG)"]
    _0000,
    #[doc = "High speed comparator 0 output"]
    _0001,
    #[doc = "PIT trigger 0"]
    _0100,
    #[doc = "PIT trigger 1"]
    _0101,
    #[doc = "PIT trigger 2"]
    _0110,
    #[doc = "PIT trigger 3"]
    _0111,
    #[doc = "TPM0 trigger"]
    _1000,
    #[doc = "TPM1 trigger"]
    _1001,
    #[doc = "TPM2 trigger"]
    _1010,
    #[doc = "Low-power timer1 (LPTMR1) trigger"]
    _1011,
    #[doc = "RTC alarm"]
    _1100,
    #[doc = "RTC seconds"]
    _1101,
    #[doc = "Low-power timer (LPTMR) trigger"]
    _1110,
    #[doc = "TPM1 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    _1111,
}
impl ADC0TRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC0TRGSELW::_0000 => 0,
            ADC0TRGSELW::_0001 => 1,
            ADC0TRGSELW::_0100 => 4,
            ADC0TRGSELW::_0101 => 5,
            ADC0TRGSELW::_0110 => 6,
            ADC0TRGSELW::_0111 => 7,
            ADC0TRGSELW::_1000 => 8,
            ADC0TRGSELW::_1001 => 9,
            ADC0TRGSELW::_1010 => 10,
            ADC0TRGSELW::_1011 => 11,
            ADC0TRGSELW::_1100 => 12,
            ADC0TRGSELW::_1101 => 13,
            ADC0TRGSELW::_1110 => 14,
            ADC0TRGSELW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0TRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0TRGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "External trigger pin input (EXTRG)"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_0000)
    }
    #[doc = "High speed comparator 0 output"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_0001)
    }
    #[doc = "PIT trigger 0"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_0101)
    }
    #[doc = "PIT trigger 2"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_0110)
    }
    #[doc = "PIT trigger 3"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_0111)
    }
    #[doc = "TPM0 trigger"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1000)
    }
    #[doc = "TPM1 trigger"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1001)
    }
    #[doc = "TPM2 trigger"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1010)
    }
    #[doc = "Low-power timer1 (LPTMR1) trigger"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1011)
    }
    #[doc = "RTC alarm"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1100)
    }
    #[doc = "RTC seconds"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1101)
    }
    #[doc = "Low-power timer (LPTMR) trigger"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1110)
    }
    #[doc = "TPM1 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC0PRETRGSEL`"]
pub enum ADC0PRETRGSELW {
    #[doc = "Pre-trigger A"]
    _0,
    #[doc = "Pre-trigger B"]
    _1,
}
impl ADC0PRETRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0PRETRGSELW::_0 => false,
            ADC0PRETRGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0PRETRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0PRETRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0PRETRGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pre-trigger A"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0PRETRGSELW::_0)
    }
    #[doc = "Pre-trigger B"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0PRETRGSELW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline]
    pub fn adc0trgsel(&self) -> ADC0TRGSELR {
        ADC0TRGSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline]
    pub fn adc0pretrgsel(&self) -> ADC0PRETRGSELR {
        ADC0PRETRGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline]
    pub fn adc0trgsel(&mut self) -> _ADC0TRGSELW {
        _ADC0TRGSELW { w: self }
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline]
    pub fn adc0pretrgsel(&mut self) -> _ADC0PRETRGSELW {
        _ADC0PRETRGSELW { w: self }
    }
}
