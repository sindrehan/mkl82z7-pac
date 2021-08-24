#[doc = "Register `LTC0_DS` reader"]
pub struct R(crate::R<LTC0_DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTC0_DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTC0_DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTC0_DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTC0_DS` writer"]
pub struct W(crate::W<LTC0_DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTC0_DS_SPEC>;
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
impl From<crate::W<LTC0_DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTC0_DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DS` reader - Data Size"]
pub struct DS_R(crate::FieldReader<u16, u16>);
impl DS_R {
    pub(crate) fn new(bits: u16) -> Self {
        DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS` writer - Data Size"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Data Size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data Size"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTC Data Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ds](index.html) module"]
pub struct LTC0_DS_SPEC;
impl crate::RegisterSpec for LTC0_DS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltc0_ds::R](R) reader structure"]
impl crate::Readable for LTC0_DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltc0_ds::W](W) writer structure"]
impl crate::Writable for LTC0_DS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTC0_DS to value 0"]
impl crate::Resettable for LTC0_DS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
