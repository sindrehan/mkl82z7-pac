#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLSHCR {
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
#[doc = r" Value of the field"]
pub struct TCSSR {
    bits: u8,
}
impl TCSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TCSHR {
    bits: u8,
}
impl TCSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TDH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDHR {
    #[doc = "Data aligned with the posedge of Internal reference clock of QuadSPI"]
    _00,
    #[doc = "Data aligned with 2x serial flash half clock"]
    _01,
    #[doc = "Data aligned with 4x serial flash half clock"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TDHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TDHR::_00 => 0,
            TDHR::_01 => 1,
            TDHR::_10 => 2,
            TDHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TDHR {
        match value {
            0 => TDHR::_00,
            1 => TDHR::_01,
            2 => TDHR::_10,
            i => TDHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TDHR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TDHR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TDHR::_10
    }
}
#[doc = r" Proxy"]
pub struct _TCSSW<'a> {
    w: &'a mut W,
}
impl<'a> _TCSSW<'a> {
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
#[doc = r" Proxy"]
pub struct _TCSHW<'a> {
    w: &'a mut W,
}
impl<'a> _TCSHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TDH`"]
pub enum TDHW {
    #[doc = "Data aligned with the posedge of Internal reference clock of QuadSPI"]
    _00,
    #[doc = "Data aligned with 2x serial flash half clock"]
    _01,
    #[doc = "Data aligned with 4x serial flash half clock"]
    _10,
}
impl TDHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TDHW::_00 => 0,
            TDHW::_01 => 1,
            TDHW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDHW<'a> {
    w: &'a mut W,
}
impl<'a> _TDHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Data aligned with the posedge of Internal reference clock of QuadSPI"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TDHW::_00)
    }
    #[doc = "Data aligned with 2x serial flash half clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TDHW::_01)
    }
    #[doc = "Data aligned with 4x serial flash half clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TDHW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - Serial flash CS setup time in terms of serial flash clock cycles"]
    #[inline]
    pub fn tcss(&self) -> TCSSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TCSSR { bits }
    }
    #[doc = "Bits 8:11 - Serial flash CS hold time in terms of serial flash clock cycles"]
    #[inline]
    pub fn tcsh(&self) -> TCSHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TCSHR { bits }
    }
    #[doc = "Bits 16:17 - Serial flash data in hold time"]
    #[inline]
    pub fn tdh(&self) -> TDHR {
        TDHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 771 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Serial flash CS setup time in terms of serial flash clock cycles"]
    #[inline]
    pub fn tcss(&mut self) -> _TCSSW {
        _TCSSW { w: self }
    }
    #[doc = "Bits 8:11 - Serial flash CS hold time in terms of serial flash clock cycles"]
    #[inline]
    pub fn tcsh(&mut self) -> _TCSHW {
        _TCSHW { w: self }
    }
    #[doc = "Bits 16:17 - Serial flash data in hold time"]
    #[inline]
    pub fn tdh(&mut self) -> _TDHW {
        _TDHW { w: self }
    }
}
