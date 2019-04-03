#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TX_GETU {
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
#[doc = "Possible values of the field `GETU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GETUR {
    #[doc = "no additional ETUs inserted (default)"]
    _0,
    #[doc = "1 additional ETU inserted"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GETUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GETUR::_0 => 0,
            GETUR::_1 => 1,
            GETUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GETUR {
        match value {
            0 => GETUR::_0,
            1 => GETUR::_1,
            i => GETUR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GETUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GETUR::_1
    }
}
#[doc = "Values that can be written to the field `GETU`"]
pub enum GETUW {
    #[doc = "no additional ETUs inserted (default)"]
    _0,
    #[doc = "1 additional ETU inserted"]
    _1,
}
impl GETUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GETUW::_0 => 0,
            GETUW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GETUW<'a> {
    w: &'a mut W,
}
impl<'a> _GETUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GETUW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "no additional ETUs inserted (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GETUW::_0)
    }
    #[doc = "1 additional ETU inserted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GETUW::_1)
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
    #[doc = "Bits 0:7 - Transmitter Guard Time Value in ETU"]
    #[inline]
    pub fn getu(&self) -> GETUR {
        GETUR::_from({
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
    #[doc = "Bits 0:7 - Transmitter Guard Time Value in ETU"]
    #[inline]
    pub fn getu(&mut self) -> _GETUW {
        _GETUW { w: self }
    }
}
