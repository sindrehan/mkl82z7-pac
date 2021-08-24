#[doc = "Register `BWT_VAL` reader"]
pub struct R(crate::R<BWT_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BWT_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BWT_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BWT_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BWT_VAL` writer"]
pub struct W(crate::W<BWT_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BWT_VAL_SPEC>;
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
impl From<crate::W<BWT_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BWT_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BWT` reader - Block Wait Time Value"]
pub struct BWT_R(crate::FieldReader<u32, u32>);
impl BWT_R {
    pub(crate) fn new(bits: u32) -> Self {
        BWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BWT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWT` writer - Block Wait Time Value"]
pub struct BWT_W<'a> {
    w: &'a mut W,
}
impl<'a> BWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Block Wait Time Value"]
    #[inline(always)]
    pub fn bwt(&self) -> BWT_R {
        BWT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Block Wait Time Value"]
    #[inline(always)]
    pub fn bwt(&mut self) -> BWT_W {
        BWT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Wait Time Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bwt_val](index.html) module"]
pub struct BWT_VAL_SPEC;
impl crate::RegisterSpec for BWT_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bwt_val::R](R) reader structure"]
impl crate::Readable for BWT_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bwt_val::W](W) writer structure"]
impl crate::Writable for BWT_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BWT_VAL to value 0xffff_ffff"]
impl crate::Resettable for BWT_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
