#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::KEEP_ALIVE_CTRL {
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
pub struct KEEP_ALIVE_ENR {
    bits: bool,
}
impl KEEP_ALIVE_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OWN_OVERRD_ENR {
    bits: bool,
}
impl OWN_OVERRD_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `WAKE_REQ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_REQ_ENR {
    #[doc = "USB bus wakeup request is disabled."]
    _0,
    #[doc = "USB bus wakeup request is enabled."]
    _1,
}
impl WAKE_REQ_ENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WAKE_REQ_ENR::_0 => false,
            WAKE_REQ_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKE_REQ_ENR {
        match value {
            false => WAKE_REQ_ENR::_0,
            true => WAKE_REQ_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKE_REQ_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKE_REQ_ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct WAKE_INT_ENR {
    bits: bool,
}
impl WAKE_INT_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct WAKE_INT_STSR {
    bits: bool,
}
impl WAKE_INT_STSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _KEEP_ALIVE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _KEEP_ALIVE_ENW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OWN_OVERRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OWN_OVERRD_ENW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKE_REQ_EN`"]
pub enum WAKE_REQ_ENW {
    #[doc = "USB bus wakeup request is disabled."]
    _0,
    #[doc = "USB bus wakeup request is enabled."]
    _1,
}
impl WAKE_REQ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKE_REQ_ENW::_0 => false,
            WAKE_REQ_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKE_REQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKE_REQ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKE_REQ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB bus wakeup request is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_REQ_ENW::_0)
    }
    #[doc = "USB bus wakeup request is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKE_REQ_ENW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAKE_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKE_INT_ENW<'a> {
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Global enable for USB_KEEP_ALIVE mode"]
    #[inline]
    pub fn keep_alive_en(&self) -> KEEP_ALIVE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        KEEP_ALIVE_ENR { bits }
    }
    #[doc = "Bit 1 - When set to 1, during KEEP_ALIVE mode, if received token is not SETUP, the OWN bit of current BD will be forced to 0, so usb core will respond with NAK"]
    #[inline]
    pub fn own_overrd_en(&self) -> OWN_OVERRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        OWN_OVERRD_ENR { bits }
    }
    #[doc = "Bit 3 - During KEEP_ALIVE mode, a bus access by the USB controller to a memory location outside the USB SRAM will cause the bus access to stall until KEEP_ALIVE mode is exited"]
    #[inline]
    pub fn wake_req_en(&self) -> WAKE_REQ_ENR {
        WAKE_REQ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Wakeup Interrupt Enable."]
    #[inline]
    pub fn wake_int_en(&self) -> WAKE_INT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        WAKE_INT_ENR { bits }
    }
    #[doc = "Bit 7 - Wakeup Interrupt Status."]
    #[inline]
    pub fn wake_int_sts(&self) -> WAKE_INT_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        WAKE_INT_STSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Global enable for USB_KEEP_ALIVE mode"]
    #[inline]
    pub fn keep_alive_en(&mut self) -> _KEEP_ALIVE_ENW {
        _KEEP_ALIVE_ENW { w: self }
    }
    #[doc = "Bit 1 - When set to 1, during KEEP_ALIVE mode, if received token is not SETUP, the OWN bit of current BD will be forced to 0, so usb core will respond with NAK"]
    #[inline]
    pub fn own_overrd_en(&mut self) -> _OWN_OVERRD_ENW {
        _OWN_OVERRD_ENW { w: self }
    }
    #[doc = "Bit 3 - During KEEP_ALIVE mode, a bus access by the USB controller to a memory location outside the USB SRAM will cause the bus access to stall until KEEP_ALIVE mode is exited"]
    #[inline]
    pub fn wake_req_en(&mut self) -> _WAKE_REQ_ENW {
        _WAKE_REQ_ENW { w: self }
    }
    #[doc = "Bit 4 - Wakeup Interrupt Enable."]
    #[inline]
    pub fn wake_int_en(&mut self) -> _WAKE_INT_ENW {
        _WAKE_INT_ENW { w: self }
    }
}
