#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::KEEP_ALIVE_WKCTRL {
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
#[doc = "Possible values of the field `WAKE_ON_THIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_ON_THISR {
    #[doc = "Wake up on receiving OUT/SETUP token packet."]
    _0001,
    #[doc = "Wake up on receiving SETUP token packet.All other values are reserved."]
    _1101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WAKE_ON_THISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAKE_ON_THISR::_0001 => 1,
            WAKE_ON_THISR::_1101 => 13,
            WAKE_ON_THISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAKE_ON_THISR {
        match value {
            1 => WAKE_ON_THISR::_0001,
            13 => WAKE_ON_THISR::_1101,
            i => WAKE_ON_THISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == WAKE_ON_THISR::_0001
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == WAKE_ON_THISR::_1101
    }
}
#[doc = r" Value of the field"]
pub struct WAKE_ENDPTR {
    bits: u8,
}
impl WAKE_ENDPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `WAKE_ON_THIS`"]
pub enum WAKE_ON_THISW {
    #[doc = "Wake up on receiving OUT/SETUP token packet."]
    _0001,
    #[doc = "Wake up on receiving SETUP token packet.All other values are reserved."]
    _1101,
}
impl WAKE_ON_THISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAKE_ON_THISW::_0001 => 1,
            WAKE_ON_THISW::_1101 => 13,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKE_ON_THISW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKE_ON_THISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKE_ON_THISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Wake up on receiving OUT/SETUP token packet."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(WAKE_ON_THISW::_0001)
    }
    #[doc = "Wake up on receiving SETUP token packet.All other values are reserved."]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(WAKE_ON_THISW::_1101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Software configure it to which token can wakeup usb during KEEP_ALIVE mode"]
    #[inline]
    pub fn wake_on_this(&self) -> WAKE_ON_THISR {
        WAKE_ON_THISR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:7 - Indicates which endpoint causes the wakeup interrupt. Reset to 0, software read only."]
    #[inline]
    pub fn wake_endpt(&self) -> WAKE_ENDPTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        WAKE_ENDPTR { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Software configure it to which token can wakeup usb during KEEP_ALIVE mode"]
    #[inline]
    pub fn wake_on_this(&mut self) -> _WAKE_ON_THISW {
        _WAKE_ON_THISW { w: self }
    }
}
