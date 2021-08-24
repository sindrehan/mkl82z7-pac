#[doc = "Register `MF5` reader"]
pub struct R(crate::R<MF5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MF5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MF5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MF5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Wakeup flag For module 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF0_A {
    #[doc = "0: Module 0 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 0 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF0_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF0` reader - Wakeup flag For module 0"]
pub struct MWUF0_R(crate::FieldReader<bool, MWUF0_A>);
impl MWUF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MWUF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF0_A {
        match self.bits {
            false => MWUF0_A::_0,
            true => MWUF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MWUF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MWUF0_A::_1
    }
}
impl core::ops::Deref for MWUF0_R {
    type Target = crate::FieldReader<bool, MWUF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup flag For module 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF1_A {
    #[doc = "0: Module 1 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 1 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF1_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF1` reader - Wakeup flag For module 1"]
pub struct MWUF1_R(crate::FieldReader<bool, MWUF1_A>);
impl MWUF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MWUF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF1_A {
        match self.bits {
            false => MWUF1_A::_0,
            true => MWUF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MWUF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MWUF1_A::_1
    }
}
impl core::ops::Deref for MWUF1_R {
    type Target = crate::FieldReader<bool, MWUF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup flag For module 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF2_A {
    #[doc = "0: Module 2 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 2 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF2_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF2` reader - Wakeup flag For module 2"]
pub struct MWUF2_R(crate::FieldReader<bool, MWUF2_A>);
impl MWUF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MWUF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF2_A {
        match self.bits {
            false => MWUF2_A::_0,
            true => MWUF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MWUF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MWUF2_A::_1
    }
}
impl core::ops::Deref for MWUF2_R {
    type Target = crate::FieldReader<bool, MWUF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup flag For module 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF3_A {
    #[doc = "0: Module 3 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 3 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF3_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF3` reader - Wakeup flag For module 3"]
pub struct MWUF3_R(crate::FieldReader<bool, MWUF3_A>);
impl MWUF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MWUF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF3_A {
        match self.bits {
            false => MWUF3_A::_0,
            true => MWUF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MWUF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MWUF3_A::_1
    }
}
impl core::ops::Deref for MWUF3_R {
    type Target = crate::FieldReader<bool, MWUF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup flag For module 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF4_A {
    #[doc = "0: Module 4 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 4 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF4_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF4` reader - Wakeup flag For module 4"]
pub struct MWUF4_R(crate::FieldReader<bool, MWUF4_A>);
impl MWUF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MWUF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF4_A {
        match self.bits {
            false => MWUF4_A::_0,
            true => MWUF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MWUF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MWUF4_A::_1
    }
}
impl core::ops::Deref for MWUF4_R {
    type Target = crate::FieldReader<bool, MWUF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup flag For module 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF5_A {
    #[doc = "0: Module 5 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 5 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF5_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF5` reader - Wakeup flag For module 5"]
pub struct MWUF5_R(crate::FieldReader<bool, MWUF5_A>);
impl MWUF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        MWUF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF5_A {
        match self.bits {
            false => MWUF5_A::_0,
            true => MWUF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MWUF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MWUF5_A::_1
    }
}
impl core::ops::Deref for MWUF5_R {
    type Target = crate::FieldReader<bool, MWUF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup flag For module 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF6_A {
    #[doc = "0: Module 6 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 6 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF6_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF6` reader - Wakeup flag For module 6"]
pub struct MWUF6_R(crate::FieldReader<bool, MWUF6_A>);
impl MWUF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        MWUF6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF6_A {
        match self.bits {
            false => MWUF6_A::_0,
            true => MWUF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MWUF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MWUF6_A::_1
    }
}
impl core::ops::Deref for MWUF6_R {
    type Target = crate::FieldReader<bool, MWUF6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup flag For module 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF7_A {
    #[doc = "0: Module 7 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 7 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF7_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWUF7` reader - Wakeup flag For module 7"]
pub struct MWUF7_R(crate::FieldReader<bool, MWUF7_A>);
impl MWUF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        MWUF7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF7_A {
        match self.bits {
            false => MWUF7_A::_0,
            true => MWUF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MWUF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MWUF7_A::_1
    }
}
impl core::ops::Deref for MWUF7_R {
    type Target = crate::FieldReader<bool, MWUF7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag For module 0"]
    #[inline(always)]
    pub fn mwuf0(&self) -> MWUF0_R {
        MWUF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag For module 1"]
    #[inline(always)]
    pub fn mwuf1(&self) -> MWUF1_R {
        MWUF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag For module 2"]
    #[inline(always)]
    pub fn mwuf2(&self) -> MWUF2_R {
        MWUF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag For module 3"]
    #[inline(always)]
    pub fn mwuf3(&self) -> MWUF3_R {
        MWUF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag For module 4"]
    #[inline(always)]
    pub fn mwuf4(&self) -> MWUF4_R {
        MWUF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup flag For module 5"]
    #[inline(always)]
    pub fn mwuf5(&self) -> MWUF5_R {
        MWUF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup flag For module 6"]
    #[inline(always)]
    pub fn mwuf6(&self) -> MWUF6_R {
        MWUF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup flag For module 7"]
    #[inline(always)]
    pub fn mwuf7(&self) -> MWUF7_R {
        MWUF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "LLWU Module Flag 5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mf5](index.html) module"]
pub struct MF5_SPEC;
impl crate::RegisterSpec for MF5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mf5::R](R) reader structure"]
impl crate::Readable for MF5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MF5 to value 0"]
impl crate::Resettable for MF5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
