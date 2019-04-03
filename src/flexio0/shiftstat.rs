#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHIFTSTAT {
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
#[doc = "Possible values of the field `SSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSFR {
    #[doc = "Status flag is clear"]
    _0,
    #[doc = "Status flag is set"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSFR::_0 => 0,
            SSFR::_1 => 1,
            SSFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSFR {
        match value {
            0 => SSFR::_0,
            1 => SSFR::_1,
            i => SSFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SSFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SSFR::_1
    }
}
#[doc = "Values that can be written to the field `SSF`"]
pub enum SSFW {
    #[doc = "Status flag is clear"]
    _0,
    #[doc = "Status flag is set"]
    _1,
}
impl SSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSFW::_0 => 0,
            SSFW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSFW<'a> {
    w: &'a mut W,
}
impl<'a> _SSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Status flag is clear"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSFW::_0)
    }
    #[doc = "Status flag is set"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSFW::_1)
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
    #[doc = "Bits 0:7 - Shifter Status Flag"]
    #[inline]
    pub fn ssf(&self) -> SSFR {
        SSFR::_from({
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
    #[doc = "Bits 0:7 - Shifter Status Flag"]
    #[inline]
    pub fn ssf(&mut self) -> _SSFW {
        _SSFW { w: self }
    }
}
