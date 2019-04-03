#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LTC0_MD {
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
#[doc = "Possible values of the field `ENC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCR {
    #[doc = "Decrypt."]
    _0,
    #[doc = "Encrypt."]
    _1,
}
impl ENCR {
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
            ENCR::_0 => false,
            ENCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENCR {
        match value {
            false => ENCR::_0,
            true => ENCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENCR::_1
    }
}
#[doc = r" Value of the field"]
pub struct ICV_TESTR {
    bits: bool,
}
impl ICV_TESTR {
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
#[doc = "Possible values of the field `AS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASR {
    #[doc = "Update"]
    _00,
    #[doc = "Initialize"]
    _01,
    #[doc = "Finalize"]
    _10,
    #[doc = "Initialize/Finalize"]
    _11,
}
impl ASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ASR::_00 => 0,
            ASR::_01 => 1,
            ASR::_10 => 2,
            ASR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ASR {
        match value {
            0 => ASR::_00,
            1 => ASR::_01,
            2 => ASR::_10,
            3 => ASR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ASR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ASR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ASR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ASR::_11
    }
}
#[doc = r" Value of the field"]
pub struct AAIR {
    bits: u16,
}
impl AAIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ALG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALGR {
    #[doc = "AES"]
    _00010000,
    #[doc = "DES"]
    _00100000,
    #[doc = "3DES"]
    _00100001,
    #[doc = "MDHA - SHA-1"]
    _01000001,
    #[doc = "MDHA - SHA-224"]
    _01000010,
    #[doc = "MDHA - SHA-256"]
    _01000011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALGR::_00010000 => 16,
            ALGR::_00100000 => 32,
            ALGR::_00100001 => 33,
            ALGR::_01000001 => 65,
            ALGR::_01000010 => 66,
            ALGR::_01000011 => 67,
            ALGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALGR {
        match value {
            16 => ALGR::_00010000,
            32 => ALGR::_00100000,
            33 => ALGR::_00100001,
            65 => ALGR::_01000001,
            66 => ALGR::_01000010,
            67 => ALGR::_01000011,
            i => ALGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00010000`"]
    #[inline]
    pub fn is_00010000(&self) -> bool {
        *self == ALGR::_00010000
    }
    #[doc = "Checks if the value of the field is `_00100000`"]
    #[inline]
    pub fn is_00100000(&self) -> bool {
        *self == ALGR::_00100000
    }
    #[doc = "Checks if the value of the field is `_00100001`"]
    #[inline]
    pub fn is_00100001(&self) -> bool {
        *self == ALGR::_00100001
    }
    #[doc = "Checks if the value of the field is `_01000001`"]
    #[inline]
    pub fn is_01000001(&self) -> bool {
        *self == ALGR::_01000001
    }
    #[doc = "Checks if the value of the field is `_01000010`"]
    #[inline]
    pub fn is_01000010(&self) -> bool {
        *self == ALGR::_01000010
    }
    #[doc = "Checks if the value of the field is `_01000011`"]
    #[inline]
    pub fn is_01000011(&self) -> bool {
        *self == ALGR::_01000011
    }
}
#[doc = "Values that can be written to the field `ENC`"]
pub enum ENCW {
    #[doc = "Decrypt."]
    _0,
    #[doc = "Encrypt."]
    _1,
}
impl ENCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENCW::_0 => false,
            ENCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENCW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Decrypt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENCW::_0)
    }
    #[doc = "Encrypt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENCW::_1)
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
#[doc = r" Proxy"]
pub struct _ICV_TESTW<'a> {
    w: &'a mut W,
}
impl<'a> _ICV_TESTW<'a> {
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
#[doc = "Values that can be written to the field `AS`"]
pub enum ASW {
    #[doc = "Update"]
    _00,
    #[doc = "Initialize"]
    _01,
    #[doc = "Finalize"]
    _10,
    #[doc = "Initialize/Finalize"]
    _11,
}
impl ASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ASW::_00 => 0,
            ASW::_01 => 1,
            ASW::_10 => 2,
            ASW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASW<'a> {
    w: &'a mut W,
}
impl<'a> _ASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Update"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ASW::_00)
    }
    #[doc = "Initialize"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ASW::_01)
    }
    #[doc = "Finalize"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ASW::_10)
    }
    #[doc = "Initialize/Finalize"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ASW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AAIW<'a> {
    w: &'a mut W,
}
impl<'a> _AAIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALG`"]
pub enum ALGW {
    #[doc = "AES"]
    _00010000,
    #[doc = "DES"]
    _00100000,
    #[doc = "3DES"]
    _00100001,
    #[doc = "MDHA - SHA-1"]
    _01000001,
    #[doc = "MDHA - SHA-224"]
    _01000010,
    #[doc = "MDHA - SHA-256"]
    _01000011,
}
impl ALGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALGW::_00010000 => 16,
            ALGW::_00100000 => 32,
            ALGW::_00100001 => 33,
            ALGW::_01000001 => 65,
            ALGW::_01000010 => 66,
            ALGW::_01000011 => 67,
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
    #[doc = "AES"]
    #[inline]
    pub fn _00010000(self) -> &'a mut W {
        self.variant(ALGW::_00010000)
    }
    #[doc = "DES"]
    #[inline]
    pub fn _00100000(self) -> &'a mut W {
        self.variant(ALGW::_00100000)
    }
    #[doc = "3DES"]
    #[inline]
    pub fn _00100001(self) -> &'a mut W {
        self.variant(ALGW::_00100001)
    }
    #[doc = "MDHA - SHA-1"]
    #[inline]
    pub fn _01000001(self) -> &'a mut W {
        self.variant(ALGW::_01000001)
    }
    #[doc = "MDHA - SHA-224"]
    #[inline]
    pub fn _01000010(self) -> &'a mut W {
        self.variant(ALGW::_01000010)
    }
    #[doc = "MDHA - SHA-256"]
    #[inline]
    pub fn _01000011(self) -> &'a mut W {
        self.variant(ALGW::_01000011)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Encrypt/Decrypt. This bit selects encryption or decryption."]
    #[inline]
    pub fn enc(&self) -> ENCR {
        ENCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ICV Checking / Test AES fault detection"]
    #[inline]
    pub fn icv_test(&self) -> ICV_TESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ICV_TESTR { bits }
    }
    #[doc = "Bits 2:3 - Algorithm State"]
    #[inline]
    pub fn as_(&self) -> ASR {
        ASR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:12 - Additional Algorithm information"]
    #[inline]
    pub fn aai(&self) -> AAIR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        AAIR { bits }
    }
    #[doc = "Bits 16:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline]
    pub fn alg(&self) -> ALGR {
        ALGR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Encrypt/Decrypt. This bit selects encryption or decryption."]
    #[inline]
    pub fn enc(&mut self) -> _ENCW {
        _ENCW { w: self }
    }
    #[doc = "Bit 1 - ICV Checking / Test AES fault detection"]
    #[inline]
    pub fn icv_test(&mut self) -> _ICV_TESTW {
        _ICV_TESTW { w: self }
    }
    #[doc = "Bits 2:3 - Algorithm State"]
    #[inline]
    pub fn as_(&mut self) -> _ASW {
        _ASW { w: self }
    }
    #[doc = "Bits 4:12 - Additional Algorithm information"]
    #[inline]
    pub fn aai(&mut self) -> _AAIW {
        _AAIW { w: self }
    }
    #[doc = "Bits 16:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline]
    pub fn alg(&mut self) -> _ALGW {
        _ALGW { w: self }
    }
}
