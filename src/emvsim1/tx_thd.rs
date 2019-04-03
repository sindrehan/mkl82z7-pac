#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TX_THD {
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
pub struct TDTR {
    bits: u8,
}
impl TDTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TNCK_THD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNCK_THDR {
    #[doc = "TNTE will never be set; retransmission after NACK reception is disabled."]
    _0,
    #[doc = "TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TNCK_THDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TNCK_THDR::_0 => 0,
            TNCK_THDR::_1 => 1,
            TNCK_THDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TNCK_THDR {
        match value {
            0 => TNCK_THDR::_0,
            1 => TNCK_THDR::_1,
            i => TNCK_THDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TNCK_THDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TNCK_THDR::_1
    }
}
#[doc = r" Proxy"]
pub struct _TDTW<'a> {
    w: &'a mut W,
}
impl<'a> _TDTW<'a> {
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
#[doc = "Values that can be written to the field `TNCK_THD`"]
pub enum TNCK_THDW {
    #[doc = "TNTE will never be set; retransmission after NACK reception is disabled."]
    _0,
    #[doc = "TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
    _1,
}
impl TNCK_THDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TNCK_THDW::_0 => 0,
            TNCK_THDW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNCK_THDW<'a> {
    w: &'a mut W,
}
impl<'a> _TNCK_THDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNCK_THDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TNTE will never be set; retransmission after NACK reception is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TNCK_THDW::_0)
    }
    #[doc = "TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TNCK_THDW::_1)
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
    #[doc = "Bits 0:3 - Transmitter Data Threshold Value"]
    #[inline]
    pub fn tdt(&self) -> TDTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDTR { bits }
    }
    #[doc = "Bits 8:11 - Transmitter NACK Threshold Value"]
    #[inline]
    pub fn tnck_thd(&self) -> TNCK_THDR {
        TNCK_THDR::_from({
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
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Transmitter Data Threshold Value"]
    #[inline]
    pub fn tdt(&mut self) -> _TDTW {
        _TDTW { w: self }
    }
    #[doc = "Bits 8:11 - Transmitter NACK Threshold Value"]
    #[inline]
    pub fn tnck_thd(&mut self) -> _TNCK_THDW {
        _TNCK_THDW { w: self }
    }
}
