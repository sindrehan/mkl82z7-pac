#[doc = "Register `TRNG0_STATUS` reader"]
pub struct R(crate::R<TRNG0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNG0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRNG0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRNG0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TF1BR0` reader - Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
pub struct TF1BR0_R(crate::FieldReader<bool, bool>);
impl TF1BR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF1BR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF1BR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF1BR1` reader - Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
pub struct TF1BR1_R(crate::FieldReader<bool, bool>);
impl TF1BR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF1BR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF1BR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF2BR0` reader - Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
pub struct TF2BR0_R(crate::FieldReader<bool, bool>);
impl TF2BR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF2BR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF2BR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF2BR1` reader - Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
pub struct TF2BR1_R(crate::FieldReader<bool, bool>);
impl TF2BR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF2BR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF2BR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF3BR0` reader - Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
pub struct TF3BR0_R(crate::FieldReader<bool, bool>);
impl TF3BR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF3BR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF3BR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF3BR1` reader - Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
pub struct TF3BR1_R(crate::FieldReader<bool, bool>);
impl TF3BR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF3BR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF3BR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF4BR0` reader - Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
pub struct TF4BR0_R(crate::FieldReader<bool, bool>);
impl TF4BR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF4BR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF4BR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF4BR1` reader - Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
pub struct TF4BR1_R(crate::FieldReader<bool, bool>);
impl TF4BR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF4BR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF4BR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF5BR0` reader - Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
pub struct TF5BR0_R(crate::FieldReader<bool, bool>);
impl TF5BR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF5BR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF5BR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF5BR1` reader - Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
pub struct TF5BR1_R(crate::FieldReader<bool, bool>);
impl TF5BR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF5BR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF5BR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF6PBR0` reader - Test Fail, 6 Plus Bit Run, Sampling 0s"]
pub struct TF6PBR0_R(crate::FieldReader<bool, bool>);
impl TF6PBR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF6PBR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF6PBR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF6PBR1` reader - Test Fail, 6 Plus Bit Run, Sampling 1s"]
pub struct TF6PBR1_R(crate::FieldReader<bool, bool>);
impl TF6PBR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF6PBR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF6PBR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFSB` reader - Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
pub struct TFSB_R(crate::FieldReader<bool, bool>);
impl TFSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFLR` reader - Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
pub struct TFLR_R(crate::FieldReader<bool, bool>);
impl TFLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFP` reader - Test Fail, Poker. If TFP=1, the Poker Test has failed."]
pub struct TFP_R(crate::FieldReader<bool, bool>);
impl TFP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFMB` reader - Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
pub struct TFMB_R(crate::FieldReader<bool, bool>);
impl TFMB_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFMB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRY_CT` reader - RETRY COUNT"]
pub struct RETRY_CT_R(crate::FieldReader<u8, u8>);
impl RETRY_CT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RETRY_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRY_CT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf1br0(&self) -> TF1BR0_R {
        TF1BR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf1br1(&self) -> TF1BR1_R {
        TF1BR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf2br0(&self) -> TF2BR0_R {
        TF2BR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf2br1(&self) -> TF2BR1_R {
        TF2BR1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf3br0(&self) -> TF3BR0_R {
        TF3BR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf3br1(&self) -> TF3BR1_R {
        TF3BR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf4br0(&self) -> TF4BR0_R {
        TF4BR0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf4br1(&self) -> TF4BR1_R {
        TF4BR1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf5br0(&self) -> TF5BR0_R {
        TF5BR0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf5br1(&self) -> TF5BR1_R {
        TF5BR1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Test Fail, 6 Plus Bit Run, Sampling 0s"]
    #[inline(always)]
    pub fn tf6pbr0(&self) -> TF6PBR0_R {
        TF6PBR0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Test Fail, 6 Plus Bit Run, Sampling 1s"]
    #[inline(always)]
    pub fn tf6pbr1(&self) -> TF6PBR1_R {
        TF6PBR1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
    #[inline(always)]
    pub fn tfsb(&self) -> TFSB_R {
        TFSB_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
    #[inline(always)]
    pub fn tflr(&self) -> TFLR_R {
        TFLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Test Fail, Poker. If TFP=1, the Poker Test has failed."]
    #[inline(always)]
    pub fn tfp(&self) -> TFP_R {
        TFP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
    #[inline(always)]
    pub fn tfmb(&self) -> TFMB_R {
        TFMB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn retry_ct(&self) -> RETRY_CT_R {
        RETRY_CT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "TRNG0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng0_status](index.html) module"]
pub struct TRNG0_STATUS_SPEC;
impl crate::RegisterSpec for TRNG0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trng0_status::R](R) reader structure"]
impl crate::Readable for TRNG0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRNG0_STATUS to value 0"]
impl crate::Resettable for TRNG0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
