#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LTC0_MDPK {
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
pub struct PKHA_MODE_LSR {
    bits: u16,
}
impl PKHA_MODE_LSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKHA_MODE_MSR {
    bits: u8,
}
impl PKHA_MODE_MSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ALG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALGR {
    #[doc = "PKHA"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALGR::_1000 => 8,
            ALGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALGR {
        match value {
            8 => ALGR::_1000,
            i => ALGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == ALGR::_1000
    }
}
#[doc = r" Proxy"]
pub struct _PKHA_MODE_LSW<'a> {
    w: &'a mut W,
}
impl<'a> _PKHA_MODE_LSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PKHA_MODE_MSW<'a> {
    w: &'a mut W,
}
impl<'a> _PKHA_MODE_MSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALG`"]
pub enum ALGW {
    #[doc = "PKHA"]
    _1000,
}
impl ALGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALGW::_1000 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALGW<'a> {
    w: &'a mut W,
}
impl<'a> _ALGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PKHA"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ALGW::_1000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:11 - PKHA_MODE least significant 12 bits"]
    #[inline]
    pub fn pkha_mode_ls(&self) -> PKHA_MODE_LSR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKHA_MODE_LSR { bits }
    }
    #[doc = "Bits 16:19 - PKHA_MODE most-significant 4 bits"]
    #[inline]
    pub fn pkha_mode_ms(&self) -> PKHA_MODE_MSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PKHA_MODE_MSR { bits }
    }
    #[doc = "Bits 20:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline]
    pub fn alg(&self) -> ALGR {
        ALGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:11 - PKHA_MODE least significant 12 bits"]
    #[inline]
    pub fn pkha_mode_ls(&mut self) -> _PKHA_MODE_LSW {
        _PKHA_MODE_LSW { w: self }
    }
    #[doc = "Bits 16:19 - PKHA_MODE most-significant 4 bits"]
    #[inline]
    pub fn pkha_mode_ms(&mut self) -> _PKHA_MODE_MSW {
        _PKHA_MODE_MSW { w: self }
    }
    #[doc = "Bits 20:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline]
    pub fn alg(&mut self) -> _ALGW {
        _ALGW { w: self }
    }
}
