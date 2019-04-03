#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LUT {
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
pub struct OPRND0R {
    bits: u8,
}
impl OPRND0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PAD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0R {
    #[doc = "1 Pad"]
    _00,
    #[doc = "2 Pads"]
    _01,
    #[doc = "4 Pads"]
    _10,
    #[doc = "8 Pads"]
    _11,
}
impl PAD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD0R::_00 => 0,
            PAD0R::_01 => 1,
            PAD0R::_10 => 2,
            PAD0R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD0R {
        match value {
            0 => PAD0R::_00,
            1 => PAD0R::_01,
            2 => PAD0R::_10,
            3 => PAD0R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PAD0R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PAD0R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PAD0R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PAD0R::_11
    }
}
#[doc = r" Value of the field"]
pub struct INSTR0R {
    bits: u8,
}
impl INSTR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OPRND1R {
    bits: u8,
}
impl OPRND1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PAD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1R {
    #[doc = "1 Pad"]
    _00,
    #[doc = "2 Pads"]
    _01,
    #[doc = "4 Pads"]
    _10,
    #[doc = "8 Pads"]
    _11,
}
impl PAD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD1R::_00 => 0,
            PAD1R::_01 => 1,
            PAD1R::_10 => 2,
            PAD1R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD1R {
        match value {
            0 => PAD1R::_00,
            1 => PAD1R::_01,
            2 => PAD1R::_10,
            3 => PAD1R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PAD1R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PAD1R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PAD1R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PAD1R::_11
    }
}
#[doc = r" Value of the field"]
pub struct INSTR1R {
    bits: u8,
}
impl INSTR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _OPRND0W<'a> {
    w: &'a mut W,
}
impl<'a> _OPRND0W<'a> {
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
#[doc = "Values that can be written to the field `PAD0`"]
pub enum PAD0W {
    #[doc = "1 Pad"]
    _00,
    #[doc = "2 Pads"]
    _01,
    #[doc = "4 Pads"]
    _10,
    #[doc = "8 Pads"]
    _11,
}
impl PAD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD0W::_00 => 0,
            PAD0W::_01 => 1,
            PAD0W::_10 => 2,
            PAD0W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD0W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 Pad"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PAD0W::_00)
    }
    #[doc = "2 Pads"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PAD0W::_01)
    }
    #[doc = "4 Pads"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PAD0W::_10)
    }
    #[doc = "8 Pads"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PAD0W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INSTR0W<'a> {
    w: &'a mut W,
}
impl<'a> _INSTR0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPRND1W<'a> {
    w: &'a mut W,
}
impl<'a> _OPRND1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAD1`"]
pub enum PAD1W {
    #[doc = "1 Pad"]
    _00,
    #[doc = "2 Pads"]
    _01,
    #[doc = "4 Pads"]
    _10,
    #[doc = "8 Pads"]
    _11,
}
impl PAD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD1W::_00 => 0,
            PAD1W::_01 => 1,
            PAD1W::_10 => 2,
            PAD1W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD1W<'a> {
    w: &'a mut W,
}
impl<'a> _PAD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 Pad"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PAD1W::_00)
    }
    #[doc = "2 Pads"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PAD1W::_01)
    }
    #[doc = "4 Pads"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PAD1W::_10)
    }
    #[doc = "8 Pads"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PAD1W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INSTR1W<'a> {
    w: &'a mut W,
}
impl<'a> _INSTR1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:7 - Operand for INSTR0."]
    #[inline]
    pub fn oprnd0(&self) -> OPRND0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPRND0R { bits }
    }
    #[doc = "Bits 8:9 - Pad information for INSTR0."]
    #[inline]
    pub fn pad0(&self) -> PAD0R {
        PAD0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:15 - Instruction 0"]
    #[inline]
    pub fn instr0(&self) -> INSTR0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INSTR0R { bits }
    }
    #[doc = "Bits 16:23 - Operand for INSTR1."]
    #[inline]
    pub fn oprnd1(&self) -> OPRND1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPRND1R { bits }
    }
    #[doc = "Bits 24:25 - Pad information for INSTR1."]
    #[inline]
    pub fn pad1(&self) -> PAD1R {
        PAD1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:31 - Instruction 1"]
    #[inline]
    pub fn instr1(&self) -> INSTR1R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INSTR1R { bits }
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
    #[doc = "Bits 0:7 - Operand for INSTR0."]
    #[inline]
    pub fn oprnd0(&mut self) -> _OPRND0W {
        _OPRND0W { w: self }
    }
    #[doc = "Bits 8:9 - Pad information for INSTR0."]
    #[inline]
    pub fn pad0(&mut self) -> _PAD0W {
        _PAD0W { w: self }
    }
    #[doc = "Bits 10:15 - Instruction 0"]
    #[inline]
    pub fn instr0(&mut self) -> _INSTR0W {
        _INSTR0W { w: self }
    }
    #[doc = "Bits 16:23 - Operand for INSTR1."]
    #[inline]
    pub fn oprnd1(&mut self) -> _OPRND1W {
        _OPRND1W { w: self }
    }
    #[doc = "Bits 24:25 - Pad information for INSTR1."]
    #[inline]
    pub fn pad1(&mut self) -> _PAD1W {
        _PAD1W { w: self }
    }
    #[doc = "Bits 26:31 - Instruction 1"]
    #[inline]
    pub fn instr1(&mut self) -> _INSTR1W {
        _INSTR1W { w: self }
    }
}
