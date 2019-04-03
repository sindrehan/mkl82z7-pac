#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CH_IPR_31_0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `INTP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTPR {
    #[doc = "No interrupt."]
    _0,
    #[doc = "Interrupt is pending."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl INTPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            INTPR::_0 => 0,
            INTPR::_1 => 1,
            INTPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> INTPR {
        match value {
            0 => INTPR::_0,
            1 => INTPR::_1,
            i => INTPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INTPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INTPR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Interrupt Pending"]
    #[inline]
    pub fn intp(&self) -> INTPR {
        INTPR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
