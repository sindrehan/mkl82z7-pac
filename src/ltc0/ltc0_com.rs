#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LTC0_COM {
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
#[doc = "Values that can be written to the field `ALL`"]
pub enum ALLW {
    #[doc = "Do Not Reset"]
    _0,
    #[doc = "Reset all CHAs in use by this CCB."]
    _1,
}
impl ALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALLW::_0 => false,
            ALLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALLW<'a> {
    w: &'a mut W,
}
impl<'a> _ALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALLW::_0)
    }
    #[doc = "Reset all CHAs in use by this CCB."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALLW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AES`"]
pub enum AESW {
    #[doc = "Do Not Reset"]
    _0,
    #[doc = "Reset AES Accelerator"]
    _1,
}
impl AESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AESW::_0 => false,
            AESW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AESW<'a> {
    w: &'a mut W,
}
impl<'a> _AESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AESW::_0)
    }
    #[doc = "Reset AES Accelerator"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AESW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DES`"]
pub enum DESW {
    #[doc = "Do Not Reset"]
    _0,
    #[doc = "Reset DES Accelerator"]
    _1,
}
impl DESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DESW::_0 => false,
            DESW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DESW<'a> {
    w: &'a mut W,
}
impl<'a> _DESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DESW::_0)
    }
    #[doc = "Reset DES Accelerator"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DESW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PK`"]
pub enum PKW {
    #[doc = "Do Not Reset"]
    _0,
    #[doc = "Reset Public Key Hardware Accelerator"]
    _1,
}
impl PKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PKW::_0 => false,
            PKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PKW<'a> {
    w: &'a mut W,
}
impl<'a> _PKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PKW::_0)
    }
    #[doc = "Reset Public Key Hardware Accelerator"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PKW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MD`"]
pub enum MDW {
    #[doc = "Do Not Reset"]
    _0,
    #[doc = "Reset Message Digest Hardware Accelerator"]
    _1,
}
impl MDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDW::_0 => false,
            MDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDW<'a> {
    w: &'a mut W,
}
impl<'a> _MDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do Not Reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDW::_0)
    }
    #[doc = "Reset Message Digest Hardware Accelerator"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Reset All Internal Logic"]
    #[inline]
    pub fn all(&mut self) -> _ALLW {
        _ALLW { w: self }
    }
    #[doc = "Bit 1 - Reset AESA. Writing a 1 to this bit resets the AES Accelerator core engine."]
    #[inline]
    pub fn aes(&mut self) -> _AESW {
        _AESW { w: self }
    }
    #[doc = "Bit 2 - Reset DESA. Writing a 1 to this bit resets the DES Accelerator."]
    #[inline]
    pub fn des(&mut self) -> _DESW {
        _DESW { w: self }
    }
    #[doc = "Bit 6 - Reset PKHA. Writing a 1 to this bit resets the Public Key Hardware Accelerator."]
    #[inline]
    pub fn pk(&mut self) -> _PKW {
        _PKW { w: self }
    }
    #[doc = "Bit 7 - Reset MDHA. Writing a 1 to this bit resets the Message Digest Hardware Accelerator."]
    #[inline]
    pub fn md(&mut self) -> _MDW {
        _MDW { w: self }
    }
}
