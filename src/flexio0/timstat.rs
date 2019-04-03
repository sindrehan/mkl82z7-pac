#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMSTAT {
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
#[doc = "Possible values of the field `TSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSFR {
    #[doc = "Timer Status Flag is clear"]
    _0,
    #[doc = "Timer Status Flag is set"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSFR::_0 => 0,
            TSFR::_1 => 1,
            TSFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSFR {
        match value {
            0 => TSFR::_0,
            1 => TSFR::_1,
            i => TSFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSFR::_1
    }
}
#[doc = "Values that can be written to the field `TSF`"]
pub enum TSFW {
    #[doc = "Timer Status Flag is clear"]
    _0,
    #[doc = "Timer Status Flag is set"]
    _1,
}
impl TSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSFW::_0 => 0,
            TSFW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSFW<'a> {
    w: &'a mut W,
}
impl<'a> _TSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer Status Flag is clear"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSFW::_0)
    }
    #[doc = "Timer Status Flag is set"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSFW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:7 - Timer Status Flags"]
    #[inline]
    pub fn tsf(&self) -> TSFR {
        TSFR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:7 - Timer Status Flags"]
    #[inline]
    pub fn tsf(&mut self) -> _TSFW {
        _TSFW { w: self }
    }
}
