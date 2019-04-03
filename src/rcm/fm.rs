#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FM {
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
#[doc = "Possible values of the field `FORCEROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEROMR {
    #[doc = "No effect"]
    _00,
    #[doc = "Force boot from ROM with RCM_MR\\[1\\] set."]
    _01,
    #[doc = "Force boot from ROM with RCM_MR\\[2\\] set."]
    _10,
    #[doc = "Force boot from ROM with RCM_MR\\[2:1\\] set."]
    _11,
}
impl FORCEROMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORCEROMR::_00 => 0,
            FORCEROMR::_01 => 1,
            FORCEROMR::_10 => 2,
            FORCEROMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORCEROMR {
        match value {
            0 => FORCEROMR::_00,
            1 => FORCEROMR::_01,
            2 => FORCEROMR::_10,
            3 => FORCEROMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FORCEROMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FORCEROMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FORCEROMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FORCEROMR::_11
    }
}
#[doc = "Values that can be written to the field `FORCEROM`"]
pub enum FORCEROMW {
    #[doc = "No effect"]
    _00,
    #[doc = "Force boot from ROM with RCM_MR\\[1\\] set."]
    _01,
    #[doc = "Force boot from ROM with RCM_MR\\[2\\] set."]
    _10,
    #[doc = "Force boot from ROM with RCM_MR\\[2:1\\] set."]
    _11,
}
impl FORCEROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORCEROMW::_00 => 0,
            FORCEROMW::_01 => 1,
            FORCEROMW::_10 => 2,
            FORCEROMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEROMW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEROMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEROMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FORCEROMW::_00)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[1\\] set."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FORCEROMW::_01)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2\\] set."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FORCEROMW::_10)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2:1\\] set."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FORCEROMW::_11)
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
    #[doc = "Bits 1:2 - Force ROM Boot"]
    #[inline]
    pub fn forcerom(&self) -> FORCEROMR {
        FORCEROMR::_from({
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
    #[doc = "Bits 1:2 - Force ROM Boot"]
    #[inline]
    pub fn forcerom(&mut self) -> _FORCEROMW {
        _FORCEROMW { w: self }
    }
}
