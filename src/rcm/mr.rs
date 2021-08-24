#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Boot ROM Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOTROM_A {
    #[doc = "0: Boot from Flash"]
    _00 = 0,
    #[doc = "1: Boot from ROM due to BOOTCFG0 pin assertion"]
    _01 = 1,
    #[doc = "2: Boot form ROM due to FOPT\\[7\\]
configuration"]
    _10 = 2,
    #[doc = "3: Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\]
configuration"]
    _11 = 3,
}
impl From<BOOTROM_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTROM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BOOTROM` reader - Boot ROM Configuration"]
pub struct BOOTROM_R(crate::FieldReader<u8, BOOTROM_A>);
impl BOOTROM_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOOTROM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTROM_A {
        match self.bits {
            0 => BOOTROM_A::_00,
            1 => BOOTROM_A::_01,
            2 => BOOTROM_A::_10,
            3 => BOOTROM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == BOOTROM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == BOOTROM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == BOOTROM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == BOOTROM_A::_11
    }
}
impl core::ops::Deref for BOOTROM_R {
    type Target = crate::FieldReader<u8, BOOTROM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOTROM` writer - Boot ROM Configuration"]
pub struct BOOTROM_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOTROM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Boot from Flash"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BOOTROM_A::_00)
    }
    #[doc = "Boot from ROM due to BOOTCFG0 pin assertion"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BOOTROM_A::_01)
    }
    #[doc = "Boot form ROM due to FOPT\\[7\\]
configuration"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BOOTROM_A::_10)
    }
    #[doc = "Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\]
configuration"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BOOTROM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u8 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - Boot ROM Configuration"]
    #[inline(always)]
    pub fn bootrom(&self) -> BOOTROM_R {
        BOOTROM_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Boot ROM Configuration"]
    #[inline(always)]
    pub fn bootrom(&mut self) -> BOOTROM_W {
        BOOTROM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
