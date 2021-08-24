#[doc = "Register `SDID` reader"]
pub struct R(crate::R<SDID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Pincount identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINID_A {
    #[doc = "5: 64-pin"]
    _0101 = 5,
    #[doc = "6: 80-pin"]
    _0110 = 6,
    #[doc = "8: 100-pin"]
    _1000 = 8,
    #[doc = "9: 121-pin"]
    _1001 = 9,
    #[doc = "11: Custom pinout (WLCSP)"]
    _1011 = 11,
}
impl From<PINID_A> for u8 {
    #[inline(always)]
    fn from(variant: PINID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PINID` reader - Pincount identification"]
pub struct PINID_R(crate::FieldReader<u8, PINID_A>);
impl PINID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PINID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PINID_A> {
        match self.bits {
            5 => Some(PINID_A::_0101),
            6 => Some(PINID_A::_0110),
            8 => Some(PINID_A::_1000),
            9 => Some(PINID_A::_1001),
            11 => Some(PINID_A::_1011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == PINID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == PINID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == PINID_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == PINID_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == PINID_A::_1011
    }
}
impl core::ops::Deref for PINID_R {
    type Target = crate::FieldReader<u8, PINID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAMID` reader - Kinetis family ID"]
pub struct FAMID_R(crate::FieldReader<u8, u8>);
impl FAMID_R {
    pub(crate) fn new(bits: u8) -> Self {
        FAMID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAMID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIEID` reader - Device die number"]
pub struct DIEID_R(crate::FieldReader<u8, u8>);
impl DIEID_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIEID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIEID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVID` reader - Device Revision Number"]
pub struct REVID_R(crate::FieldReader<u8, u8>);
impl REVID_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Kinetis Sub-Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBFAMID_A {
    #[doc = "0: KLx0 Subfamily"]
    _0000 = 0,
    #[doc = "1: KLx1 Subfamily"]
    _0001 = 1,
    #[doc = "2: KLx2 Subfamily"]
    _0010 = 2,
    #[doc = "3: KLx3 Subfamily"]
    _0011 = 3,
    #[doc = "4: KLx4 Subfamily"]
    _0100 = 4,
    #[doc = "5: KLx5 Subfamily"]
    _0101 = 5,
    #[doc = "6: KLx6 Subfamily"]
    _0110 = 6,
    #[doc = "7: KLx7 Subfamily"]
    _0111 = 7,
    #[doc = "8: KLx8 Subfamily"]
    _1000 = 8,
    #[doc = "9: KLx9 Subfamily"]
    _1001 = 9,
}
impl From<SUBFAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBFAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUBFAMID` reader - Kinetis Sub-Family ID"]
pub struct SUBFAMID_R(crate::FieldReader<u8, SUBFAMID_A>);
impl SUBFAMID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUBFAMID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUBFAMID_A> {
        match self.bits {
            0 => Some(SUBFAMID_A::_0000),
            1 => Some(SUBFAMID_A::_0001),
            2 => Some(SUBFAMID_A::_0010),
            3 => Some(SUBFAMID_A::_0011),
            4 => Some(SUBFAMID_A::_0100),
            5 => Some(SUBFAMID_A::_0101),
            6 => Some(SUBFAMID_A::_0110),
            7 => Some(SUBFAMID_A::_0111),
            8 => Some(SUBFAMID_A::_1000),
            9 => Some(SUBFAMID_A::_1001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == SUBFAMID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == SUBFAMID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == SUBFAMID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == SUBFAMID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == SUBFAMID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == SUBFAMID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == SUBFAMID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == SUBFAMID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == SUBFAMID_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == SUBFAMID_A::_1001
    }
}
impl core::ops::Deref for SUBFAMID_R {
    type Target = crate::FieldReader<u8, SUBFAMID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Kinetis L family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAMILYID_A {
    #[doc = "0: KL0x Family"]
    _0000 = 0,
    #[doc = "1: KL1x Family"]
    _0001 = 1,
    #[doc = "2: KL2x Family"]
    _0010 = 2,
    #[doc = "3: KL3x Family)"]
    _0011 = 3,
    #[doc = "4: KL4x Family)"]
    _0100 = 4,
    #[doc = "6: KL6x Family"]
    _0110 = 6,
    #[doc = "7: KL7x Family"]
    _0111 = 7,
    #[doc = "9: KL8x Family"]
    _1001 = 9,
}
impl From<FAMILYID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMILYID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FAMILYID` reader - Kinetis L family ID"]
pub struct FAMILYID_R(crate::FieldReader<u8, FAMILYID_A>);
impl FAMILYID_R {
    pub(crate) fn new(bits: u8) -> Self {
        FAMILYID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAMILYID_A> {
        match self.bits {
            0 => Some(FAMILYID_A::_0000),
            1 => Some(FAMILYID_A::_0001),
            2 => Some(FAMILYID_A::_0010),
            3 => Some(FAMILYID_A::_0011),
            4 => Some(FAMILYID_A::_0100),
            6 => Some(FAMILYID_A::_0110),
            7 => Some(FAMILYID_A::_0111),
            9 => Some(FAMILYID_A::_1001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == FAMILYID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == FAMILYID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == FAMILYID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == FAMILYID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == FAMILYID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == FAMILYID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == FAMILYID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == FAMILYID_A::_1001
    }
}
impl core::ops::Deref for FAMILYID_R {
    type Target = crate::FieldReader<u8, FAMILYID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PINID_R {
        PINID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Kinetis family ID"]
    #[inline(always)]
    pub fn famid(&self) -> FAMID_R {
        FAMID_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:11 - Device die number"]
    #[inline(always)]
    pub fn dieid(&self) -> DIEID_R {
        DIEID_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Device Revision Number"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Kinetis Sub-Family ID"]
    #[inline(always)]
    pub fn subfamid(&self) -> SUBFAMID_R {
        SUBFAMID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Kinetis L family ID"]
    #[inline(always)]
    pub fn familyid(&self) -> FAMILYID_R {
        FAMILYID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "System Device Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdid](index.html) module"]
pub struct SDID_SPEC;
impl crate::RegisterSpec for SDID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdid::R](R) reader structure"]
impl crate::Readable for SDID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDID to value 0"]
impl crate::Resettable for SDID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
