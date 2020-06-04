#[doc = "Reader of register CHCFG%s"]
pub type R = crate::R<u8, super::CHCFG>;
#[doc = "Writer for register CHCFG%s"]
pub type W = crate::W<u8, super::CHCFG>;
#[doc = "Register CHCFG%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Channel Source (Slot)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOURCE_A {
    #[doc = "0: Disable_Signal"]
    _0 = 0,
    #[doc = "1: Group1FlexIO0_Channel0_Signal"]
    _1 = 1,
    #[doc = "2: Group1FlexIO0_Channel1_Signal"]
    _2 = 2,
    #[doc = "3: Group1FlexIO0_Channel2_Signal"]
    _3 = 3,
    #[doc = "4: Group1FlexIO0_Channel3_Signal"]
    _4 = 4,
    #[doc = "5: Group1FlexIO0_Channel4_Signal"]
    _5 = 5,
    #[doc = "6: Group1FlexIO0_Channel5_Signal"]
    _6 = 6,
    #[doc = "7: Group1FlexIO0_Channel6_Signal"]
    _7 = 7,
    #[doc = "8: Group1FlexIO0_Channel7_Signal"]
    _8 = 8,
    #[doc = "9: I2C0_Signal"]
    _9 = 9,
    #[doc = "10: I2C1_Signal"]
    _10 = 10,
    #[doc = "15: LPUART0_Rx_Signal"]
    _15 = 15,
    #[doc = "16: LPUART0_Tx_Signal"]
    _16 = 16,
    #[doc = "17: LPUART1_Rx_Signal"]
    _17 = 17,
    #[doc = "18: LPUART1_Tx_Signal"]
    _18 = 18,
    #[doc = "19: LPUART2_Rx_Signal"]
    _19 = 19,
    #[doc = "20: LPUART2_Tx_Signal"]
    _20 = 20,
    #[doc = "21: SPI0_Rx_Signal"]
    _21 = 21,
    #[doc = "22: SPI0_Tx_Signal"]
    _22 = 22,
    #[doc = "23: SPI1_Rx_Signal"]
    _23 = 23,
    #[doc = "24: SPI1_Tx_Signal"]
    _24 = 24,
    #[doc = "25: Group1QSPI0_Rx_Signal"]
    _25 = 25,
    #[doc = "26: Group1QSPI0_Tx_Signal"]
    _26 = 26,
    #[doc = "27: TPM0_Channel0_Signal"]
    _27 = 27,
    #[doc = "28: TPM0_Channel1_Signal"]
    _28 = 28,
    #[doc = "29: TPM0_Channel2_Signal"]
    _29 = 29,
    #[doc = "30: TPM0_Channel3_Signal"]
    _30 = 30,
    #[doc = "31: TPM0_Channel4_Signal"]
    _31 = 31,
    #[doc = "32: TPM0_Channel5_Signal"]
    _32 = 32,
    #[doc = "35: TPM0_Overflow_Signal"]
    _35 = 35,
    #[doc = "36: TPM1_Channel0_Signal"]
    _36 = 36,
    #[doc = "37: TPM1_Channel1_Signal"]
    _37 = 37,
    #[doc = "38: TPM1_Overflow_Signal"]
    _38 = 38,
    #[doc = "39: TPM2_Channel0_Signal"]
    _39 = 39,
    #[doc = "40: TPM2_Channel1_Signal"]
    _40 = 40,
    #[doc = "41: TPM2_Overflow_Signal"]
    _41 = 41,
    #[doc = "42: TSI0_Signal"]
    _42 = 42,
    #[doc = "43: Group1EMVSIM0_Rx_Signal"]
    _43 = 43,
    #[doc = "44: Group1EMVSIM0_Tx_Signal"]
    _44 = 44,
    #[doc = "45: Group1EMVSIM1_Rx_Signal"]
    _45 = 45,
    #[doc = "46: Group1EMVSIM1_Tx_Signal"]
    _46 = 46,
    #[doc = "47: PortA_Signal"]
    _47 = 47,
    #[doc = "48: PortB_Signal"]
    _48 = 48,
    #[doc = "49: PortC_Signal"]
    _49 = 49,
    #[doc = "50: PortD_Signal"]
    _50 = 50,
    #[doc = "51: PortE_Signal"]
    _51 = 51,
    #[doc = "52: ADC0_Signal"]
    _52 = 52,
    #[doc = "54: DAC0_Signal"]
    _54 = 54,
    #[doc = "55: LTC0_PKHA_Signal"]
    _55 = 55,
    #[doc = "56: CMP0_Signal"]
    _56 = 56,
    #[doc = "58: LTC0_Input_FIFO_Signal"]
    _58 = 58,
    #[doc = "59: LTC0_Output_FIFO_Signal"]
    _59 = 59,
    #[doc = "60: AlwaysOn60_Signal"]
    _60 = 60,
    #[doc = "61: AlwaysOn61_Signal"]
    _61 = 61,
    #[doc = "62: AlwaysOn62_Signal"]
    _62 = 62,
    #[doc = "63: AlwaysOn63_Signal"]
    _63 = 63,
}
impl From<SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u8, SOURCE_A>;
impl SOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SOURCE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SOURCE_A::_0),
            1 => Val(SOURCE_A::_1),
            2 => Val(SOURCE_A::_2),
            3 => Val(SOURCE_A::_3),
            4 => Val(SOURCE_A::_4),
            5 => Val(SOURCE_A::_5),
            6 => Val(SOURCE_A::_6),
            7 => Val(SOURCE_A::_7),
            8 => Val(SOURCE_A::_8),
            9 => Val(SOURCE_A::_9),
            10 => Val(SOURCE_A::_10),
            15 => Val(SOURCE_A::_15),
            16 => Val(SOURCE_A::_16),
            17 => Val(SOURCE_A::_17),
            18 => Val(SOURCE_A::_18),
            19 => Val(SOURCE_A::_19),
            20 => Val(SOURCE_A::_20),
            21 => Val(SOURCE_A::_21),
            22 => Val(SOURCE_A::_22),
            23 => Val(SOURCE_A::_23),
            24 => Val(SOURCE_A::_24),
            25 => Val(SOURCE_A::_25),
            26 => Val(SOURCE_A::_26),
            27 => Val(SOURCE_A::_27),
            28 => Val(SOURCE_A::_28),
            29 => Val(SOURCE_A::_29),
            30 => Val(SOURCE_A::_30),
            31 => Val(SOURCE_A::_31),
            32 => Val(SOURCE_A::_32),
            35 => Val(SOURCE_A::_35),
            36 => Val(SOURCE_A::_36),
            37 => Val(SOURCE_A::_37),
            38 => Val(SOURCE_A::_38),
            39 => Val(SOURCE_A::_39),
            40 => Val(SOURCE_A::_40),
            41 => Val(SOURCE_A::_41),
            42 => Val(SOURCE_A::_42),
            43 => Val(SOURCE_A::_43),
            44 => Val(SOURCE_A::_44),
            45 => Val(SOURCE_A::_45),
            46 => Val(SOURCE_A::_46),
            47 => Val(SOURCE_A::_47),
            48 => Val(SOURCE_A::_48),
            49 => Val(SOURCE_A::_49),
            50 => Val(SOURCE_A::_50),
            51 => Val(SOURCE_A::_51),
            52 => Val(SOURCE_A::_52),
            54 => Val(SOURCE_A::_54),
            55 => Val(SOURCE_A::_55),
            56 => Val(SOURCE_A::_56),
            58 => Val(SOURCE_A::_58),
            59 => Val(SOURCE_A::_59),
            60 => Val(SOURCE_A::_60),
            61 => Val(SOURCE_A::_61),
            62 => Val(SOURCE_A::_62),
            63 => Val(SOURCE_A::_63),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOURCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOURCE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == SOURCE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == SOURCE_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SOURCE_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == SOURCE_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == SOURCE_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == SOURCE_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SOURCE_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == SOURCE_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SOURCE_A::_10
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == SOURCE_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SOURCE_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        *self == SOURCE_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == SOURCE_A::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        *self == SOURCE_A::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == SOURCE_A::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        *self == SOURCE_A::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        *self == SOURCE_A::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        *self == SOURCE_A::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == SOURCE_A::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == SOURCE_A::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        *self == SOURCE_A::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        *self == SOURCE_A::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        *self == SOURCE_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        *self == SOURCE_A::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == SOURCE_A::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == SOURCE_A::_31
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SOURCE_A::_32
    }
    #[doc = "Checks if the value of the field is `_35`"]
    #[inline(always)]
    pub fn is_35(&self) -> bool {
        *self == SOURCE_A::_35
    }
    #[doc = "Checks if the value of the field is `_36`"]
    #[inline(always)]
    pub fn is_36(&self) -> bool {
        *self == SOURCE_A::_36
    }
    #[doc = "Checks if the value of the field is `_37`"]
    #[inline(always)]
    pub fn is_37(&self) -> bool {
        *self == SOURCE_A::_37
    }
    #[doc = "Checks if the value of the field is `_38`"]
    #[inline(always)]
    pub fn is_38(&self) -> bool {
        *self == SOURCE_A::_38
    }
    #[doc = "Checks if the value of the field is `_39`"]
    #[inline(always)]
    pub fn is_39(&self) -> bool {
        *self == SOURCE_A::_39
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == SOURCE_A::_40
    }
    #[doc = "Checks if the value of the field is `_41`"]
    #[inline(always)]
    pub fn is_41(&self) -> bool {
        *self == SOURCE_A::_41
    }
    #[doc = "Checks if the value of the field is `_42`"]
    #[inline(always)]
    pub fn is_42(&self) -> bool {
        *self == SOURCE_A::_42
    }
    #[doc = "Checks if the value of the field is `_43`"]
    #[inline(always)]
    pub fn is_43(&self) -> bool {
        *self == SOURCE_A::_43
    }
    #[doc = "Checks if the value of the field is `_44`"]
    #[inline(always)]
    pub fn is_44(&self) -> bool {
        *self == SOURCE_A::_44
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline(always)]
    pub fn is_45(&self) -> bool {
        *self == SOURCE_A::_45
    }
    #[doc = "Checks if the value of the field is `_46`"]
    #[inline(always)]
    pub fn is_46(&self) -> bool {
        *self == SOURCE_A::_46
    }
    #[doc = "Checks if the value of the field is `_47`"]
    #[inline(always)]
    pub fn is_47(&self) -> bool {
        *self == SOURCE_A::_47
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline(always)]
    pub fn is_48(&self) -> bool {
        *self == SOURCE_A::_48
    }
    #[doc = "Checks if the value of the field is `_49`"]
    #[inline(always)]
    pub fn is_49(&self) -> bool {
        *self == SOURCE_A::_49
    }
    #[doc = "Checks if the value of the field is `_50`"]
    #[inline(always)]
    pub fn is_50(&self) -> bool {
        *self == SOURCE_A::_50
    }
    #[doc = "Checks if the value of the field is `_51`"]
    #[inline(always)]
    pub fn is_51(&self) -> bool {
        *self == SOURCE_A::_51
    }
    #[doc = "Checks if the value of the field is `_52`"]
    #[inline(always)]
    pub fn is_52(&self) -> bool {
        *self == SOURCE_A::_52
    }
    #[doc = "Checks if the value of the field is `_54`"]
    #[inline(always)]
    pub fn is_54(&self) -> bool {
        *self == SOURCE_A::_54
    }
    #[doc = "Checks if the value of the field is `_55`"]
    #[inline(always)]
    pub fn is_55(&self) -> bool {
        *self == SOURCE_A::_55
    }
    #[doc = "Checks if the value of the field is `_56`"]
    #[inline(always)]
    pub fn is_56(&self) -> bool {
        *self == SOURCE_A::_56
    }
    #[doc = "Checks if the value of the field is `_58`"]
    #[inline(always)]
    pub fn is_58(&self) -> bool {
        *self == SOURCE_A::_58
    }
    #[doc = "Checks if the value of the field is `_59`"]
    #[inline(always)]
    pub fn is_59(&self) -> bool {
        *self == SOURCE_A::_59
    }
    #[doc = "Checks if the value of the field is `_60`"]
    #[inline(always)]
    pub fn is_60(&self) -> bool {
        *self == SOURCE_A::_60
    }
    #[doc = "Checks if the value of the field is `_61`"]
    #[inline(always)]
    pub fn is_61(&self) -> bool {
        *self == SOURCE_A::_61
    }
    #[doc = "Checks if the value of the field is `_62`"]
    #[inline(always)]
    pub fn is_62(&self) -> bool {
        *self == SOURCE_A::_62
    }
    #[doc = "Checks if the value of the field is `_63`"]
    #[inline(always)]
    pub fn is_63(&self) -> bool {
        *self == SOURCE_A::_63
    }
}
#[doc = "Write proxy for field `SOURCE`"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable_Signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOURCE_A::_0)
    }
    #[doc = "Group1FlexIO0_Channel0_Signal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOURCE_A::_1)
    }
    #[doc = "Group1FlexIO0_Channel1_Signal"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SOURCE_A::_2)
    }
    #[doc = "Group1FlexIO0_Channel2_Signal"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SOURCE_A::_3)
    }
    #[doc = "Group1FlexIO0_Channel3_Signal"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SOURCE_A::_4)
    }
    #[doc = "Group1FlexIO0_Channel4_Signal"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(SOURCE_A::_5)
    }
    #[doc = "Group1FlexIO0_Channel5_Signal"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(SOURCE_A::_6)
    }
    #[doc = "Group1FlexIO0_Channel6_Signal"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(SOURCE_A::_7)
    }
    #[doc = "Group1FlexIO0_Channel7_Signal"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SOURCE_A::_8)
    }
    #[doc = "I2C0_Signal"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(SOURCE_A::_9)
    }
    #[doc = "I2C1_Signal"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SOURCE_A::_10)
    }
    #[doc = "LPUART0_Rx_Signal"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(SOURCE_A::_15)
    }
    #[doc = "LPUART0_Tx_Signal"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SOURCE_A::_16)
    }
    #[doc = "LPUART1_Rx_Signal"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(SOURCE_A::_17)
    }
    #[doc = "LPUART1_Tx_Signal"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(SOURCE_A::_18)
    }
    #[doc = "LPUART2_Rx_Signal"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut W {
        self.variant(SOURCE_A::_19)
    }
    #[doc = "LPUART2_Tx_Signal"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(SOURCE_A::_20)
    }
    #[doc = "SPI0_Rx_Signal"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut W {
        self.variant(SOURCE_A::_21)
    }
    #[doc = "SPI0_Tx_Signal"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut W {
        self.variant(SOURCE_A::_22)
    }
    #[doc = "SPI1_Rx_Signal"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut W {
        self.variant(SOURCE_A::_23)
    }
    #[doc = "SPI1_Tx_Signal"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(SOURCE_A::_24)
    }
    #[doc = "Group1QSPI0_Rx_Signal"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut W {
        self.variant(SOURCE_A::_25)
    }
    #[doc = "Group1QSPI0_Tx_Signal"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut W {
        self.variant(SOURCE_A::_26)
    }
    #[doc = "TPM0_Channel0_Signal"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut W {
        self.variant(SOURCE_A::_27)
    }
    #[doc = "TPM0_Channel1_Signal"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(SOURCE_A::_28)
    }
    #[doc = "TPM0_Channel2_Signal"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(SOURCE_A::_29)
    }
    #[doc = "TPM0_Channel3_Signal"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(SOURCE_A::_30)
    }
    #[doc = "TPM0_Channel4_Signal"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(SOURCE_A::_31)
    }
    #[doc = "TPM0_Channel5_Signal"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SOURCE_A::_32)
    }
    #[doc = "TPM0_Overflow_Signal"]
    #[inline(always)]
    pub fn _35(self) -> &'a mut W {
        self.variant(SOURCE_A::_35)
    }
    #[doc = "TPM1_Channel0_Signal"]
    #[inline(always)]
    pub fn _36(self) -> &'a mut W {
        self.variant(SOURCE_A::_36)
    }
    #[doc = "TPM1_Channel1_Signal"]
    #[inline(always)]
    pub fn _37(self) -> &'a mut W {
        self.variant(SOURCE_A::_37)
    }
    #[doc = "TPM1_Overflow_Signal"]
    #[inline(always)]
    pub fn _38(self) -> &'a mut W {
        self.variant(SOURCE_A::_38)
    }
    #[doc = "TPM2_Channel0_Signal"]
    #[inline(always)]
    pub fn _39(self) -> &'a mut W {
        self.variant(SOURCE_A::_39)
    }
    #[doc = "TPM2_Channel1_Signal"]
    #[inline(always)]
    pub fn _40(self) -> &'a mut W {
        self.variant(SOURCE_A::_40)
    }
    #[doc = "TPM2_Overflow_Signal"]
    #[inline(always)]
    pub fn _41(self) -> &'a mut W {
        self.variant(SOURCE_A::_41)
    }
    #[doc = "TSI0_Signal"]
    #[inline(always)]
    pub fn _42(self) -> &'a mut W {
        self.variant(SOURCE_A::_42)
    }
    #[doc = "Group1EMVSIM0_Rx_Signal"]
    #[inline(always)]
    pub fn _43(self) -> &'a mut W {
        self.variant(SOURCE_A::_43)
    }
    #[doc = "Group1EMVSIM0_Tx_Signal"]
    #[inline(always)]
    pub fn _44(self) -> &'a mut W {
        self.variant(SOURCE_A::_44)
    }
    #[doc = "Group1EMVSIM1_Rx_Signal"]
    #[inline(always)]
    pub fn _45(self) -> &'a mut W {
        self.variant(SOURCE_A::_45)
    }
    #[doc = "Group1EMVSIM1_Tx_Signal"]
    #[inline(always)]
    pub fn _46(self) -> &'a mut W {
        self.variant(SOURCE_A::_46)
    }
    #[doc = "PortA_Signal"]
    #[inline(always)]
    pub fn _47(self) -> &'a mut W {
        self.variant(SOURCE_A::_47)
    }
    #[doc = "PortB_Signal"]
    #[inline(always)]
    pub fn _48(self) -> &'a mut W {
        self.variant(SOURCE_A::_48)
    }
    #[doc = "PortC_Signal"]
    #[inline(always)]
    pub fn _49(self) -> &'a mut W {
        self.variant(SOURCE_A::_49)
    }
    #[doc = "PortD_Signal"]
    #[inline(always)]
    pub fn _50(self) -> &'a mut W {
        self.variant(SOURCE_A::_50)
    }
    #[doc = "PortE_Signal"]
    #[inline(always)]
    pub fn _51(self) -> &'a mut W {
        self.variant(SOURCE_A::_51)
    }
    #[doc = "ADC0_Signal"]
    #[inline(always)]
    pub fn _52(self) -> &'a mut W {
        self.variant(SOURCE_A::_52)
    }
    #[doc = "DAC0_Signal"]
    #[inline(always)]
    pub fn _54(self) -> &'a mut W {
        self.variant(SOURCE_A::_54)
    }
    #[doc = "LTC0_PKHA_Signal"]
    #[inline(always)]
    pub fn _55(self) -> &'a mut W {
        self.variant(SOURCE_A::_55)
    }
    #[doc = "CMP0_Signal"]
    #[inline(always)]
    pub fn _56(self) -> &'a mut W {
        self.variant(SOURCE_A::_56)
    }
    #[doc = "LTC0_Input_FIFO_Signal"]
    #[inline(always)]
    pub fn _58(self) -> &'a mut W {
        self.variant(SOURCE_A::_58)
    }
    #[doc = "LTC0_Output_FIFO_Signal"]
    #[inline(always)]
    pub fn _59(self) -> &'a mut W {
        self.variant(SOURCE_A::_59)
    }
    #[doc = "AlwaysOn60_Signal"]
    #[inline(always)]
    pub fn _60(self) -> &'a mut W {
        self.variant(SOURCE_A::_60)
    }
    #[doc = "AlwaysOn61_Signal"]
    #[inline(always)]
    pub fn _61(self) -> &'a mut W {
        self.variant(SOURCE_A::_61)
    }
    #[doc = "AlwaysOn62_Signal"]
    #[inline(always)]
    pub fn _62(self) -> &'a mut W {
        self.variant(SOURCE_A::_62)
    }
    #[doc = "AlwaysOn63_Signal"]
    #[inline(always)]
    pub fn _63(self) -> &'a mut W {
        self.variant(SOURCE_A::_63)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
#[doc = "DMA Channel Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG_A {
    #[doc = "0: Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    _0 = 0,
    #[doc = "1: Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    _1 = 1,
}
impl From<TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIG`"]
pub type TRIG_R = crate::R<bool, TRIG_A>;
impl TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_A {
        match self.bits {
            false => TRIG_A::_0,
            true => TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG_A::_1
    }
}
#[doc = "Write proxy for field `TRIG`"]
pub struct TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG_A::_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "DMA Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBL_A {
    #[doc = "0: DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    _0 = 0,
    #[doc = "1: DMA channel is enabled"]
    _1 = 1,
}
impl From<ENBL_A> for bool {
    #[inline(always)]
    fn from(variant: ENBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENBL`"]
pub type ENBL_R = crate::R<bool, ENBL_A>;
impl ENBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBL_A {
        match self.bits {
            false => ENBL_A::_0,
            true => ENBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENBL_A::_1
    }
}
#[doc = "Write proxy for field `ENBL`"]
pub struct ENBL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENBL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENBL_A::_0)
    }
    #[doc = "DMA channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENBL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> ENBL_R {
        ENBL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W {
        SOURCE_W { w: self }
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W {
        TRIG_W { w: self }
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    pub fn enbl(&mut self) -> ENBL_W {
        ENBL_W { w: self }
    }
}
