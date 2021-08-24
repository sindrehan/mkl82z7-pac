#[doc = "Register `LTC0_MD` reader"]
pub struct R(crate::R<LTC0_LTC0_MD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_LTC0_MD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_LTC0_MD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_LTC0_MD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_MD` writer"]
pub struct W(crate::W<LTC0_LTC0_MD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_LTC0_MD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LTC0_LTC0_MD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_LTC0_MD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Encrypt/Decrypt. This bit selects encryption or decryption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENC_A {
    #[doc = "0: Decrypt."]
    _0 = 0,
    #[doc = "1: Encrypt."]
    _1 = 1,
}
impl From<ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - Encrypt/Decrypt. This bit selects encryption or decryption."]
pub struct ENC_R(crate::FieldReader<bool, ENC_A>);
impl ENC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENC_A {
        match self.bits {
            false => ENC_A::_0,
            true => ENC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENC_A::_1
    }
}
impl core::ops::Deref for ENC_R {
    type Target = crate::FieldReader<bool, ENC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENC` writer - Encrypt/Decrypt. This bit selects encryption or decryption."]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Decrypt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENC_A::_0)
    }
    #[doc = "Encrypt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENC_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ICV_TEST` reader - ICV Checking / Test AES fault detection"]
pub struct ICV_TEST_R(crate::FieldReader<bool, bool>);
impl ICV_TEST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICV_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICV_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICV_TEST` writer - ICV Checking / Test AES fault detection"]
pub struct ICV_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICV_TEST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Algorithm State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AS_A {
    #[doc = "0: Update"]
    _00 = 0,
    #[doc = "1: Initialize"]
    _01 = 1,
    #[doc = "2: Finalize"]
    _10 = 2,
    #[doc = "3: Initialize/Finalize"]
    _11 = 3,
}
impl From<AS_A> for u8 {
    #[inline(always)]
    fn from(variant: AS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AS` reader - Algorithm State"]
pub struct AS_R(crate::FieldReader<u8, AS_A>);
impl AS_R {
    pub(crate) fn new(bits: u8) -> Self {
        AS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AS_A {
        match self.bits {
            0 => AS_A::_00,
            1 => AS_A::_01,
            2 => AS_A::_10,
            3 => AS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == AS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == AS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == AS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == AS_A::_11
    }
}
impl core::ops::Deref for AS_R {
    type Target = crate::FieldReader<u8, AS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AS` writer - Algorithm State"]
pub struct AS_W<'a> {
    w: &'a mut W,
}
impl<'a> AS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Update"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AS_A::_00)
    }
    #[doc = "Initialize"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AS_A::_01)
    }
    #[doc = "Finalize"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AS_A::_10)
    }
    #[doc = "Initialize/Finalize"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `AAI` reader - Additional Algorithm information"]
pub struct AAI_R(crate::FieldReader<u16, u16>);
impl AAI_R {
    pub(crate) fn new(bits: u16) -> Self {
        AAI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAI` writer - Additional Algorithm information"]
pub struct AAI_W<'a> {
    w: &'a mut W,
}
impl<'a> AAI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | ((value as u32 & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Algorithm. This field specifies which algorithm is being selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALG_A {
    #[doc = "16: AES"]
    _00010000 = 16,
    #[doc = "32: DES"]
    _00100000 = 32,
    #[doc = "33: 3DES"]
    _00100001 = 33,
    #[doc = "65: MDHA - SHA-1"]
    _01000001 = 65,
    #[doc = "66: MDHA - SHA-224"]
    _01000010 = 66,
    #[doc = "67: MDHA - SHA-256"]
    _01000011 = 67,
}
impl From<ALG_A> for u8 {
    #[inline(always)]
    fn from(variant: ALG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ALG` reader - Algorithm. This field specifies which algorithm is being selected."]
pub struct ALG_R(crate::FieldReader<u8, ALG_A>);
impl ALG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALG_A> {
        match self.bits {
            16 => Some(ALG_A::_00010000),
            32 => Some(ALG_A::_00100000),
            33 => Some(ALG_A::_00100001),
            65 => Some(ALG_A::_01000001),
            66 => Some(ALG_A::_01000010),
            67 => Some(ALG_A::_01000011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00010000`"]
    #[inline(always)]
    pub fn is_00010000(&self) -> bool {
        **self == ALG_A::_00010000
    }
    #[doc = "Checks if the value of the field is `_00100000`"]
    #[inline(always)]
    pub fn is_00100000(&self) -> bool {
        **self == ALG_A::_00100000
    }
    #[doc = "Checks if the value of the field is `_00100001`"]
    #[inline(always)]
    pub fn is_00100001(&self) -> bool {
        **self == ALG_A::_00100001
    }
    #[doc = "Checks if the value of the field is `_01000001`"]
    #[inline(always)]
    pub fn is_01000001(&self) -> bool {
        **self == ALG_A::_01000001
    }
    #[doc = "Checks if the value of the field is `_01000010`"]
    #[inline(always)]
    pub fn is_01000010(&self) -> bool {
        **self == ALG_A::_01000010
    }
    #[doc = "Checks if the value of the field is `_01000011`"]
    #[inline(always)]
    pub fn is_01000011(&self) -> bool {
        **self == ALG_A::_01000011
    }
}
impl core::ops::Deref for ALG_R {
    type Target = crate::FieldReader<u8, ALG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALG` writer - Algorithm. This field specifies which algorithm is being selected."]
pub struct ALG_W<'a> {
    w: &'a mut W,
}
impl<'a> ALG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AES"]
    #[inline(always)]
    pub fn _00010000(self) -> &'a mut W {
        self.variant(ALG_A::_00010000)
    }
    #[doc = "DES"]
    #[inline(always)]
    pub fn _00100000(self) -> &'a mut W {
        self.variant(ALG_A::_00100000)
    }
    #[doc = "3DES"]
    #[inline(always)]
    pub fn _00100001(self) -> &'a mut W {
        self.variant(ALG_A::_00100001)
    }
    #[doc = "MDHA - SHA-1"]
    #[inline(always)]
    pub fn _01000001(self) -> &'a mut W {
        self.variant(ALG_A::_01000001)
    }
    #[doc = "MDHA - SHA-224"]
    #[inline(always)]
    pub fn _01000010(self) -> &'a mut W {
        self.variant(ALG_A::_01000010)
    }
    #[doc = "MDHA - SHA-256"]
    #[inline(always)]
    pub fn _01000011(self) -> &'a mut W {
        self.variant(ALG_A::_01000011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Encrypt/Decrypt. This bit selects encryption or decryption."]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ICV Checking / Test AES fault detection"]
    #[inline(always)]
    pub fn icv_test(&self) -> ICV_TEST_R {
        ICV_TEST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Algorithm State"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:12 - Additional Algorithm information"]
    #[inline(always)]
    pub fn aai(&self) -> AAI_R {
        AAI_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline(always)]
    pub fn alg(&self) -> ALG_R {
        ALG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Encrypt/Decrypt. This bit selects encryption or decryption."]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bit 1 - ICV Checking / Test AES fault detection"]
    #[inline(always)]
    pub fn icv_test(&mut self) -> ICV_TEST_W {
        ICV_TEST_W { w: self }
    }
    #[doc = "Bits 2:3 - Algorithm State"]
    #[inline(always)]
    pub fn as_(&mut self) -> AS_W {
        AS_W { w: self }
    }
    #[doc = "Bits 4:12 - Additional Algorithm information"]
    #[inline(always)]
    pub fn aai(&mut self) -> AAI_W {
        AAI_W { w: self }
    }
    #[doc = "Bits 16:23 - Algorithm. This field specifies which algorithm is being selected."]
    #[inline(always)]
    pub fn alg(&mut self) -> ALG_W {
        ALG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC Mode Register (non-PKHA/non-RNG use)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ltc0_md](index.html) module"]
pub struct LTC0_LTC0_MD_SPEC;
impl crate::RegisterSpec for LTC0_LTC0_MD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_ltc0_md::R](R) reader structure"]
impl crate::Readable for LTC0_LTC0_MD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_ltc0_md::W](W) writer structure"]
impl crate::Writable for LTC0_LTC0_MD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_MD to value 0"]
impl crate::Resettable for LTC0_LTC0_MD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
