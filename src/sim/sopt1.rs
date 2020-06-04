#[doc = "Reader of register SOPT1"]
pub type R = crate::R<u32, super::SOPT1>;
#[doc = "Writer for register SOPT1"]
pub type W = crate::W<u32, super::SOPT1>;
#[doc = "Register SOPT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System RAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMSIZE_A {
    #[doc = "1: 8 KB"]
    _0001 = 1,
    #[doc = "3: 16 KB"]
    _0011 = 3,
    #[doc = "4: 24 KB"]
    _0100 = 4,
    #[doc = "5: 32 KB"]
    _0101 = 5,
    #[doc = "6: 48 KB"]
    _0110 = 6,
    #[doc = "7: 64 KB"]
    _0111 = 7,
    #[doc = "8: 96 KB"]
    _1000 = 8,
    #[doc = "9: 128 KB"]
    _1001 = 9,
    #[doc = "11: 256 KB"]
    _1011 = 11,
}
impl From<RAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAMSIZE`"]
pub type RAMSIZE_R = crate::R<u8, RAMSIZE_A>;
impl RAMSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RAMSIZE_A::_0001),
            3 => Val(RAMSIZE_A::_0011),
            4 => Val(RAMSIZE_A::_0100),
            5 => Val(RAMSIZE_A::_0101),
            6 => Val(RAMSIZE_A::_0110),
            7 => Val(RAMSIZE_A::_0111),
            8 => Val(RAMSIZE_A::_1000),
            9 => Val(RAMSIZE_A::_1001),
            11 => Val(RAMSIZE_A::_1011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == RAMSIZE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == RAMSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == RAMSIZE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == RAMSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == RAMSIZE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == RAMSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == RAMSIZE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == RAMSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == RAMSIZE_A::_1011
    }
}
#[doc = "32K Oscillator Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSC32KSEL_A {
    #[doc = "0: System oscillator (OSC32KCLK)"]
    _00 = 0,
    #[doc = "2: RTC oscillator"]
    _10 = 2,
    #[doc = "3: LPO 1 kHz"]
    _11 = 3,
}
impl From<OSC32KSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC32KSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSC32KSEL`"]
pub type OSC32KSEL_R = crate::R<u8, OSC32KSEL_A>;
impl OSC32KSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSC32KSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSC32KSEL_A::_00),
            2 => Val(OSC32KSEL_A::_10),
            3 => Val(OSC32KSEL_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSC32KSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OSC32KSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OSC32KSEL_A::_11
    }
}
#[doc = "Write proxy for field `OSC32KSEL`"]
pub struct OSC32KSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC32KSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "System oscillator (OSC32KCLK)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_00)
    }
    #[doc = "RTC oscillator"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_10)
    }
    #[doc = "LPO 1 kHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - System RAM Size"]
    #[inline(always)]
    pub fn ramsize(&self) -> RAMSIZE_R {
        RAMSIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - 32K Oscillator Clock Select"]
    #[inline(always)]
    pub fn osc32ksel(&self) -> OSC32KSEL_R {
        OSC32KSEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - 32K Oscillator Clock Select"]
    #[inline(always)]
    pub fn osc32ksel(&mut self) -> OSC32KSEL_W {
        OSC32KSEL_W { w: self }
    }
}