#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_THD {
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
pub struct RDTR {
    bits: u8,
}
impl RDTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RNCK_THD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNCK_THDR {
    #[doc = "Zero Threshold. RTE will not be set"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RNCK_THDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RNCK_THDR::_0 => 0,
            RNCK_THDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RNCK_THDR {
        match value {
            0 => RNCK_THDR::_0,
            i => RNCK_THDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RNCK_THDR::_0
    }
}
#[doc = r" Proxy"]
pub struct _RDTW<'a> {
    w: &'a mut W,
}
impl<'a> _RDTW<'a> {
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
#[doc = "Values that can be written to the field `RNCK_THD`"]
pub enum RNCK_THDW {
    #[doc = "Zero Threshold. RTE will not be set"]
    _0,
}
impl RNCK_THDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RNCK_THDW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RNCK_THDW<'a> {
    w: &'a mut W,
}
impl<'a> _RNCK_THDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNCK_THDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Zero Threshold. RTE will not be set"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNCK_THDW::_0)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Receiver Data Threshold Value"]
    #[inline]
    pub fn rdt(&self) -> RDTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDTR { bits }
    }
    #[doc = "Bits 8:11 - Receiver NACK Threshold Value"]
    #[inline]
    pub fn rnck_thd(&self) -> RNCK_THDR {
        RNCK_THDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Receiver Data Threshold Value"]
    #[inline]
    pub fn rdt(&mut self) -> _RDTW {
        _RDTW { w: self }
    }
    #[doc = "Bits 8:11 - Receiver NACK Threshold Value"]
    #[inline]
    pub fn rnck_thd(&mut self) -> _RNCK_THDW {
        _RNCK_THDW { w: self }
    }
}
