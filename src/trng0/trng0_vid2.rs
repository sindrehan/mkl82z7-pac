#[doc = "Reader of register TRNG0_VID2"]
pub type R = crate::R<u32, super::TRNG0_VID2>;
#[doc = "Shows the Freescale IP's Configuaration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_CONFIG_OPT_A {
    #[doc = "0: TRNG_CONFIG_OPT for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_CONFIG_OPT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_CONFIG_OPT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRNG0_CONFIG_OPT`"]
pub type TRNG0_CONFIG_OPT_R = crate::R<u8, TRNG0_CONFIG_OPT_A>;
impl TRNG0_CONFIG_OPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRNG0_CONFIG_OPT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRNG0_CONFIG_OPT_A::_0X00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_CONFIG_OPT_A::_0X00
    }
}
#[doc = "Shows the Freescale IP's ECO revision of the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_ECO_REV_A {
    #[doc = "0: TRNG_ECO_REV for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_ECO_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_ECO_REV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRNG0_ECO_REV`"]
pub type TRNG0_ECO_REV_R = crate::R<u8, TRNG0_ECO_REV_A>;
impl TRNG0_ECO_REV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRNG0_ECO_REV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRNG0_ECO_REV_A::_0X00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_ECO_REV_A::_0X00
    }
}
#[doc = "Shows the Freescale integration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_INTG_OPT_A {
    #[doc = "0: INTG_OPT for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_INTG_OPT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_INTG_OPT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRNG0_INTG_OPT`"]
pub type TRNG0_INTG_OPT_R = crate::R<u8, TRNG0_INTG_OPT_A>;
impl TRNG0_INTG_OPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRNG0_INTG_OPT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRNG0_INTG_OPT_A::_0X00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_INTG_OPT_A::_0X00
    }
}
#[doc = "Shows the Freescale compile options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRNG0_ERA_A {
    #[doc = "0: COMPILE_OPT for TRNG."]
    _0X00 = 0,
}
impl From<TRNG0_ERA_A> for u8 {
    #[inline(always)]
    fn from(variant: TRNG0_ERA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRNG0_ERA`"]
pub type TRNG0_ERA_R = crate::R<u8, TRNG0_ERA_A>;
impl TRNG0_ERA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRNG0_ERA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRNG0_ERA_A::_0X00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == TRNG0_ERA_A::_0X00
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the Freescale IP's Configuaration options for the TRNG."]
    #[inline(always)]
    pub fn trng0_config_opt(&self) -> TRNG0_CONFIG_OPT_R {
        TRNG0_CONFIG_OPT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the Freescale IP's ECO revision of the TRNG."]
    #[inline(always)]
    pub fn trng0_eco_rev(&self) -> TRNG0_ECO_REV_R {
        TRNG0_ECO_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Shows the Freescale integration options for the TRNG."]
    #[inline(always)]
    pub fn trng0_intg_opt(&self) -> TRNG0_INTG_OPT_R {
        TRNG0_INTG_OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Shows the Freescale compile options for the TRNG."]
    #[inline(always)]
    pub fn trng0_era(&self) -> TRNG0_ERA_R {
        TRNG0_ERA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
