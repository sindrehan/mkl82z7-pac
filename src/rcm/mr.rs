#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MR {
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
#[doc = "Possible values of the field `BOOTROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTROMR {
    #[doc = "Boot from Flash"]
    _00,
    #[doc = "Boot from ROM due to BOOTCFG0 pin assertion"]
    _01,
    #[doc = "Boot form ROM due to FOPT\\[7\\] configuration"]
    _10,
    #[doc = "Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\] configuration"]
    _11,
}
impl BOOTROMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BOOTROMR::_00 => 0,
            BOOTROMR::_01 => 1,
            BOOTROMR::_10 => 2,
            BOOTROMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BOOTROMR {
        match value {
            0 => BOOTROMR::_00,
            1 => BOOTROMR::_01,
            2 => BOOTROMR::_10,
            3 => BOOTROMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == BOOTROMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == BOOTROMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == BOOTROMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == BOOTROMR::_11
    }
}
#[doc = "Values that can be written to the field `BOOTROM`"]
pub enum BOOTROMW {
    #[doc = "Boot from Flash"]
    _00,
    #[doc = "Boot from ROM due to BOOTCFG0 pin assertion"]
    _01,
    #[doc = "Boot form ROM due to FOPT\\[7\\] configuration"]
    _10,
    #[doc = "Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\] configuration"]
    _11,
}
impl BOOTROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BOOTROMW::_00 => 0,
            BOOTROMW::_01 => 1,
            BOOTROMW::_10 => 2,
            BOOTROMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOTROMW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTROMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOTROMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Boot from Flash"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(BOOTROMW::_00)
    }
    #[doc = "Boot from ROM due to BOOTCFG0 pin assertion"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(BOOTROMW::_01)
    }
    #[doc = "Boot form ROM due to FOPT\\[7\\] configuration"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(BOOTROMW::_10)
    }
    #[doc = "Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\] configuration"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(BOOTROMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 1:2 - Boot ROM Configuration"]
    #[inline]
    pub fn bootrom(&self) -> BOOTROMR {
        BOOTROMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 1:2 - Boot ROM Configuration"]
    #[inline]
    pub fn bootrom(&mut self) -> _BOOTROMW {
        _BOOTROMW { w: self }
    }
}
