#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "LTC Mode Register (PublicKey) LTC Mode Register (non-PKHA/non-RNG use)"]
    pub ltc0: LTC0_UNION,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - LTC Key Size Register"]
    pub ltc0_ks: LTC0_KS,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - LTC Data Size Register"]
    pub ltc0_ds: LTC0_DS,
    _reserved3: [u8; 4usize],
    #[doc = "0x18 - LTC ICV Size Register"]
    pub ltc0_icvs: LTC0_ICVS,
    _reserved4: [u8; 20usize],
    #[doc = "0x30 - LTC Command Register"]
    pub ltc0_com: LTC0_COM,
    #[doc = "0x34 - LTC Control Register"]
    pub ltc0_ctl: LTC0_CTL,
    _reserved6: [u8; 8usize],
    #[doc = "0x40 - LTC Clear Written Register"]
    pub ltc0_cw: LTC0_CW,
    _reserved7: [u8; 4usize],
    #[doc = "0x48 - LTC Status Register"]
    pub ltc0_sta: LTC0_STA,
    #[doc = "0x4c - LTC Error Status Register"]
    pub ltc0_esta: LTC0_ESTA,
    _reserved9: [u8; 8usize],
    #[doc = "0x58 - LTC AAD Size Register"]
    pub ltc0_aadsz: LTC0_AADSZ,
    _reserved10: [u8; 4usize],
    #[doc = "0x60 - LTC IV Size Register"]
    pub ltc0_ivsz: LTC0_IVSZ,
    _reserved11: [u8; 4usize],
    #[doc = "0x68 - LTC DPA Mask Seed Register"]
    pub ltc0_dpams: LTC0_DPAMS,
    _reserved12: [u8; 20usize],
    #[doc = "0x80 - LTC PKHA A Size Register"]
    pub ltc0_pkasz: LTC0_PKASZ,
    _reserved13: [u8; 4usize],
    #[doc = "0x88 - LTC PKHA B Size Register"]
    pub ltc0_pkbsz: LTC0_PKBSZ,
    _reserved14: [u8; 4usize],
    #[doc = "0x90 - LTC PKHA N Size Register"]
    pub ltc0_pknsz: LTC0_PKNSZ,
    _reserved15: [u8; 4usize],
    #[doc = "0x98 - LTC PKHA E Size Register"]
    pub ltc0_pkesz: LTC0_PKESZ,
    _reserved16: [u8; 100usize],
    #[doc = "0x100 - LTC Context Register"]
    pub ltc0_ctx_0: LTC0_CTX_0,
    #[doc = "0x104 - LTC Context Register"]
    pub ltc0_ctx_1: LTC0_CTX_1,
    #[doc = "0x108 - LTC Context Register"]
    pub ltc0_ctx_2: LTC0_CTX_2,
    #[doc = "0x10c - LTC Context Register"]
    pub ltc0_ctx_3: LTC0_CTX_3,
    #[doc = "0x110 - LTC Context Register"]
    pub ltc0_ctx_4: LTC0_CTX_4,
    #[doc = "0x114 - LTC Context Register"]
    pub ltc0_ctx_5: LTC0_CTX_5,
    #[doc = "0x118 - LTC Context Register"]
    pub ltc0_ctx_6: LTC0_CTX_6,
    #[doc = "0x11c - LTC Context Register"]
    pub ltc0_ctx_7: LTC0_CTX_7,
    #[doc = "0x120 - LTC Context Register"]
    pub ltc0_ctx_8: LTC0_CTX_8,
    #[doc = "0x124 - LTC Context Register"]
    pub ltc0_ctx_9: LTC0_CTX_9,
    #[doc = "0x128 - LTC Context Register"]
    pub ltc0_ctx_10: LTC0_CTX_10,
    #[doc = "0x12c - LTC Context Register"]
    pub ltc0_ctx_11: LTC0_CTX_11,
    #[doc = "0x130 - LTC Context Register"]
    pub ltc0_ctx_12: LTC0_CTX_12,
    #[doc = "0x134 - LTC Context Register"]
    pub ltc0_ctx_13: LTC0_CTX_13,
    #[doc = "0x138 - LTC Context Register"]
    pub ltc0_ctx_14: LTC0_CTX_14,
    #[doc = "0x13c - LTC Context Register"]
    pub ltc0_ctx_15: LTC0_CTX_15,
    _reserved32: [u8; 192usize],
    #[doc = "0x200 - LTC Key Registers"]
    pub ltc0_key_0: LTC0_KEY_0,
    #[doc = "0x204 - LTC Key Registers"]
    pub ltc0_key_1: LTC0_KEY_1,
    #[doc = "0x208 - LTC Key Registers"]
    pub ltc0_key_2: LTC0_KEY_2,
    #[doc = "0x20c - LTC Key Registers"]
    pub ltc0_key_3: LTC0_KEY_3,
    #[doc = "0x210 - LTC Key Registers"]
    pub ltc0_key_4: LTC0_KEY_4,
    #[doc = "0x214 - LTC Key Registers"]
    pub ltc0_key_5: LTC0_KEY_5,
    #[doc = "0x218 - LTC Key Registers"]
    pub ltc0_key_6: LTC0_KEY_6,
    #[doc = "0x21c - LTC Key Registers"]
    pub ltc0_key_7: LTC0_KEY_7,
    _reserved40: [u8; 720usize],
    #[doc = "0x4f0 - LTC Version ID Register"]
    pub ltc0_vid1: LTC0_VID1,
    #[doc = "0x4f4 - LTC Version ID 2 Register"]
    pub ltc0_vid2: LTC0_VID2,
    #[doc = "0x4f8 - LTC CHA Version ID Register"]
    pub ltc0_chavid: LTC0_CHAVID,
    _reserved43: [u8; 708usize],
    #[doc = "0x7c0 - LTC FIFO Status Register"]
    pub ltc0_fifosta: LTC0_FIFOSTA,
    _reserved44: [u8; 28usize],
    #[doc = "0x7e0 - LTC Input Data FIFO"]
    pub ltc0_ififo: LTC0_IFIFO,
    _reserved45: [u8; 12usize],
    #[doc = "0x7f0 - LTC Output Data FIFO"]
    pub ltc0_ofifo: LTC0_OFIFO,
    _reserved46: [u8; 12usize],
    #[doc = "LTC PKHA A 0 Register LTC PKHA A0 0 Register"]
    pub ltc0_pka_0: LTC0_PKA_0_UNION,
    #[doc = "LTC PKHA A 1 Register LTC PKHA A0 1 Register"]
    pub ltc0_pka_1: LTC0_PKA_1_UNION,
    #[doc = "LTC PKHA A 2 Register LTC PKHA A0 2 Register"]
    pub ltc0_pka_2: LTC0_PKA_2_UNION,
    #[doc = "LTC PKHA A 3 Register LTC PKHA A0 3 Register"]
    pub ltc0_pka_3: LTC0_PKA_3_UNION,
    #[doc = "LTC PKHA A 4 Register LTC PKHA A0 4 Register"]
    pub ltc0_pka_4: LTC0_PKA_4_UNION,
    #[doc = "LTC PKHA A 5 Register LTC PKHA A0 5 Register"]
    pub ltc0_pka_5: LTC0_PKA_5_UNION,
    #[doc = "LTC PKHA A 6 Register LTC PKHA A0 6 Register"]
    pub ltc0_pka_6: LTC0_PKA_6_UNION,
    #[doc = "LTC PKHA A 7 Register LTC PKHA A0 7 Register"]
    pub ltc0_pka_7: LTC0_PKA_7_UNION,
    #[doc = "LTC PKHA A 8 Register LTC PKHA A0 8 Register"]
    pub ltc0_pka_8: LTC0_PKA_8_UNION,
    #[doc = "LTC PKHA A 9 Register LTC PKHA A0 9 Register"]
    pub ltc0_pka_9: LTC0_PKA_9_UNION,
    #[doc = "LTC PKHA A 10 Register LTC PKHA A0 10 Register"]
    pub ltc0_pka_10: LTC0_PKA_10_UNION,
    #[doc = "LTC PKHA A 11 Register LTC PKHA A0 11 Register"]
    pub ltc0_pka_11: LTC0_PKA_11_UNION,
    #[doc = "LTC PKHA A 12 Register LTC PKHA A0 12 Register"]
    pub ltc0_pka_12: LTC0_PKA_12_UNION,
    #[doc = "LTC PKHA A 13 Register LTC PKHA A0 13 Register"]
    pub ltc0_pka_13: LTC0_PKA_13_UNION,
    #[doc = "LTC PKHA A 14 Register LTC PKHA A0 14 Register"]
    pub ltc0_pka_14: LTC0_PKA_14_UNION,
    #[doc = "LTC PKHA A 15 Register LTC PKHA A0 15 Register"]
    pub ltc0_pka_15: LTC0_PKA_15_UNION,
    #[doc = "LTC PKHA A 16 Register LTC PKHA A1 0 Register"]
    pub ltc0_pka1_0: LTC0_PKA1_0_UNION,
    #[doc = "LTC PKHA A 17 Register LTC PKHA A1 1 Register"]
    pub ltc0_pka1_1: LTC0_PKA1_1_UNION,
    #[doc = "LTC PKHA A 18 Register LTC PKHA A1 2 Register"]
    pub ltc0_pka1_2: LTC0_PKA1_2_UNION,
    #[doc = "LTC PKHA A 19 Register LTC PKHA A1 3 Register"]
    pub ltc0_pka1_3: LTC0_PKA1_3_UNION,
    #[doc = "LTC PKHA A 20 Register LTC PKHA A1 4 Register"]
    pub ltc0_pka1_4: LTC0_PKA1_4_UNION,
    #[doc = "LTC PKHA A 21 Register LTC PKHA A1 5 Register"]
    pub ltc0_pka1_5: LTC0_PKA1_5_UNION,
    #[doc = "LTC PKHA A 22 Register LTC PKHA A1 6 Register"]
    pub ltc0_pka1_6: LTC0_PKA1_6_UNION,
    #[doc = "LTC PKHA A 23 Register LTC PKHA A1 7 Register"]
    pub ltc0_pka1_7: LTC0_PKA1_7_UNION,
    #[doc = "LTC PKHA A 24 Register LTC PKHA A1 8 Register"]
    pub ltc0_pka1_8: LTC0_PKA1_8_UNION,
    #[doc = "LTC PKHA A 25 Register LTC PKHA A1 9 Register"]
    pub ltc0_pka1_9: LTC0_PKA1_9_UNION,
    #[doc = "LTC PKHA A 26 Register LTC PKHA A1 10 Register"]
    pub ltc0_pka_26: LTC0_PKA_26_UNION,
    #[doc = "LTC PKHA A 27 Register LTC PKHA A1 11 Register"]
    pub ltc0_pka_27: LTC0_PKA_27_UNION,
    #[doc = "LTC PKHA A 28 Register LTC PKHA A1 12 Register"]
    pub ltc0_pka_28: LTC0_PKA_28_UNION,
    #[doc = "LTC PKHA A 29 Register LTC PKHA A1 13 Register"]
    pub ltc0_pka_29: LTC0_PKA_29_UNION,
    #[doc = "LTC PKHA A 30 Register LTC PKHA A1 14 Register"]
    pub ltc0_pka_30: LTC0_PKA_30_UNION,
    #[doc = "LTC PKHA A 31 Register LTC PKHA A1 15 Register"]
    pub ltc0_pka_31: LTC0_PKA_31_UNION,
    #[doc = "LTC PKHA A 32 Register LTC PKHA A2 0 Register"]
    pub ltc0_pka2_0: LTC0_PKA2_0_UNION,
    #[doc = "LTC PKHA A 33 Register LTC PKHA A2 1 Register"]
    pub ltc0_pka2_1: LTC0_PKA2_1_UNION,
    #[doc = "LTC PKHA A 34 Register LTC PKHA A2 2 Register"]
    pub ltc0_pka2_2: LTC0_PKA2_2_UNION,
    #[doc = "LTC PKHA A 35 Register LTC PKHA A2 3 Register"]
    pub ltc0_pka2_3: LTC0_PKA2_3_UNION,
    #[doc = "LTC PKHA A 36 Register LTC PKHA A2 4 Register"]
    pub ltc0_pka2_4: LTC0_PKA2_4_UNION,
    #[doc = "LTC PKHA A 37 Register LTC PKHA A2 5 Register"]
    pub ltc0_pka2_5: LTC0_PKA2_5_UNION,
    #[doc = "LTC PKHA A 38 Register LTC PKHA A2 6 Register"]
    pub ltc0_pka2_6: LTC0_PKA2_6_UNION,
    #[doc = "LTC PKHA A 39 Register LTC PKHA A2 7 Register"]
    pub ltc0_pka2_7: LTC0_PKA2_7_UNION,
    #[doc = "LTC PKHA A 40 Register LTC PKHA A2 8 Register"]
    pub ltc0_pka2_8: LTC0_PKA2_8_UNION,
    #[doc = "LTC PKHA A 41 Register LTC PKHA A2 9 Register"]
    pub ltc0_pka2_9: LTC0_PKA2_9_UNION,
    #[doc = "LTC PKHA A 42 Register LTC PKHA A2 10 Register"]
    pub ltc0_pka_42: LTC0_PKA_42_UNION,
    #[doc = "LTC PKHA A 43 Register LTC PKHA A2 11 Register"]
    pub ltc0_pka_43: LTC0_PKA_43_UNION,
    #[doc = "LTC PKHA A 44 Register LTC PKHA A2 12 Register"]
    pub ltc0_pka_44: LTC0_PKA_44_UNION,
    #[doc = "LTC PKHA A 45 Register LTC PKHA A2 13 Register"]
    pub ltc0_pka_45: LTC0_PKA_45_UNION,
    #[doc = "LTC PKHA A 46 Register LTC PKHA A2 14 Register"]
    pub ltc0_pka_46: LTC0_PKA_46_UNION,
    #[doc = "LTC PKHA A 47 Register LTC PKHA A2 15 Register"]
    pub ltc0_pka_47: LTC0_PKA_47_UNION,
    #[doc = "LTC PKHA A 48 Register LTC PKHA A3 0 Register"]
    pub ltc0_pka3_0: LTC0_PKA3_0_UNION,
    #[doc = "LTC PKHA A 49 Register LTC PKHA A3 1 Register"]
    pub ltc0_pka3_1: LTC0_PKA3_1_UNION,
    #[doc = "LTC PKHA A 50 Register LTC PKHA A3 2 Register"]
    pub ltc0_pka3_2: LTC0_PKA3_2_UNION,
    #[doc = "LTC PKHA A 51 Register LTC PKHA A3 3 Register"]
    pub ltc0_pka3_3: LTC0_PKA3_3_UNION,
    #[doc = "LTC PKHA A 52 Register LTC PKHA A3 4 Register"]
    pub ltc0_pka3_4: LTC0_PKA3_4_UNION,
    #[doc = "LTC PKHA A 53 Register LTC PKHA A3 5 Register"]
    pub ltc0_pka3_5: LTC0_PKA3_5_UNION,
    #[doc = "LTC PKHA A 54 Register LTC PKHA A3 6 Register"]
    pub ltc0_pka3_6: LTC0_PKA3_6_UNION,
    #[doc = "LTC PKHA A 55 Register LTC PKHA A3 7 Register"]
    pub ltc0_pka3_7: LTC0_PKA3_7_UNION,
    #[doc = "LTC PKHA A 56 Register LTC PKHA A3 8 Register"]
    pub ltc0_pka3_8: LTC0_PKA3_8_UNION,
    #[doc = "LTC PKHA A 57 Register LTC PKHA A3 9 Register"]
    pub ltc0_pka3_9: LTC0_PKA3_9_UNION,
    #[doc = "LTC PKHA A 58 Register LTC PKHA A3 10 Register"]
    pub ltc0_pka_58: LTC0_PKA_58_UNION,
    #[doc = "LTC PKHA A 59 Register LTC PKHA A3 11 Register"]
    pub ltc0_pka_59: LTC0_PKA_59_UNION,
    #[doc = "LTC PKHA A 60 Register LTC PKHA A3 12 Register"]
    pub ltc0_pka_60: LTC0_PKA_60_UNION,
    #[doc = "LTC PKHA A 61 Register LTC PKHA A3 13 Register"]
    pub ltc0_pka_61: LTC0_PKA_61_UNION,
    #[doc = "LTC PKHA A 62 Register LTC PKHA A3 14 Register"]
    pub ltc0_pka_62: LTC0_PKA_62_UNION,
    #[doc = "LTC PKHA A 63 Register LTC PKHA A3 15 Register"]
    pub ltc0_pka_63: LTC0_PKA_63_UNION,
    _reserved110: [u8; 256usize],
    #[doc = "LTC PKHA B 0 Register LTC PKHA B0 0 Register"]
    pub ltc0_pkb_0: LTC0_PKB_0_UNION,
    #[doc = "LTC PKHA B 1 Register LTC PKHA B0 1 Register"]
    pub ltc0_pkb_1: LTC0_PKB_1_UNION,
    #[doc = "LTC PKHA B 2 Register LTC PKHA B0 2 Register"]
    pub ltc0_pkb_2: LTC0_PKB_2_UNION,
    #[doc = "LTC PKHA B 3 Register LTC PKHA B0 3 Register"]
    pub ltc0_pkb_3: LTC0_PKB_3_UNION,
    #[doc = "LTC PKHA B 4 Register LTC PKHA B0 4 Register"]
    pub ltc0_pkb_4: LTC0_PKB_4_UNION,
    #[doc = "LTC PKHA B 5 Register LTC PKHA B0 5 Register"]
    pub ltc0_pkb_5: LTC0_PKB_5_UNION,
    #[doc = "LTC PKHA B 6 Register LTC PKHA B0 6 Register"]
    pub ltc0_pkb_6: LTC0_PKB_6_UNION,
    #[doc = "LTC PKHA B 7 Register LTC PKHA B0 7 Register"]
    pub ltc0_pkb_7: LTC0_PKB_7_UNION,
    #[doc = "LTC PKHA B 8 Register LTC PKHA B0 8 Register"]
    pub ltc0_pkb_8: LTC0_PKB_8_UNION,
    #[doc = "LTC PKHA B 9 Register LTC PKHA B0 9 Register"]
    pub ltc0_pkb_9: LTC0_PKB_9_UNION,
    #[doc = "LTC PKHA B 10 Register LTC PKHA B0 10 Register"]
    pub ltc0_pkb_10: LTC0_PKB_10_UNION,
    #[doc = "LTC PKHA B 11 Register LTC PKHA B0 11 Register"]
    pub ltc0_pkb_11: LTC0_PKB_11_UNION,
    #[doc = "LTC PKHA B 12 Register LTC PKHA B0 12 Register"]
    pub ltc0_pkb_12: LTC0_PKB_12_UNION,
    #[doc = "LTC PKHA B 13 Register LTC PKHA B0 13 Register"]
    pub ltc0_pkb_13: LTC0_PKB_13_UNION,
    #[doc = "LTC PKHA B 14 Register LTC PKHA B0 14 Register"]
    pub ltc0_pkb_14: LTC0_PKB_14_UNION,
    #[doc = "LTC PKHA B 15 Register LTC PKHA B0 15 Register"]
    pub ltc0_pkb_15: LTC0_PKB_15_UNION,
    #[doc = "LTC PKHA B 16 Register LTC PKHA B1 0 Register"]
    pub ltc0_pkb1_0: LTC0_PKB1_0_UNION,
    #[doc = "LTC PKHA B 17 Register LTC PKHA B1 1 Register"]
    pub ltc0_pkb1_1: LTC0_PKB1_1_UNION,
    #[doc = "LTC PKHA B 18 Register LTC PKHA B1 2 Register"]
    pub ltc0_pkb1_2: LTC0_PKB1_2_UNION,
    #[doc = "LTC PKHA B 19 Register LTC PKHA B1 3 Register"]
    pub ltc0_pkb1_3: LTC0_PKB1_3_UNION,
    #[doc = "LTC PKHA B 20 Register LTC PKHA B1 4 Register"]
    pub ltc0_pkb1_4: LTC0_PKB1_4_UNION,
    #[doc = "LTC PKHA B 21 Register LTC PKHA B1 5 Register"]
    pub ltc0_pkb1_5: LTC0_PKB1_5_UNION,
    #[doc = "LTC PKHA B 22 Register LTC PKHA B1 6 Register"]
    pub ltc0_pkb1_6: LTC0_PKB1_6_UNION,
    #[doc = "LTC PKHA B 23 Register LTC PKHA B1 7 Register"]
    pub ltc0_pkb1_7: LTC0_PKB1_7_UNION,
    #[doc = "LTC PKHA B 24 Register LTC PKHA B1 8 Register"]
    pub ltc0_pkb1_8: LTC0_PKB1_8_UNION,
    #[doc = "LTC PKHA B 25 Register LTC PKHA B1 9 Register"]
    pub ltc0_pkb1_9: LTC0_PKB1_9_UNION,
    #[doc = "LTC PKHA B 26 Register LTC PKHA B1 10 Register"]
    pub ltc0_pkb_26: LTC0_PKB_26_UNION,
    #[doc = "LTC PKHA B 27 Register LTC PKHA B1 11 Register"]
    pub ltc0_pkb_27: LTC0_PKB_27_UNION,
    #[doc = "LTC PKHA B 28 Register LTC PKHA B1 12 Register"]
    pub ltc0_pkb_28: LTC0_PKB_28_UNION,
    #[doc = "LTC PKHA B 29 Register LTC PKHA B1 13 Register"]
    pub ltc0_pkb_29: LTC0_PKB_29_UNION,
    #[doc = "LTC PKHA B 30 Register LTC PKHA B1 14 Register"]
    pub ltc0_pkb_30: LTC0_PKB_30_UNION,
    #[doc = "LTC PKHA B 31 Register LTC PKHA B1 15 Register"]
    pub ltc0_pkb_31: LTC0_PKB_31_UNION,
    #[doc = "LTC PKHA B 32 Register LTC PKHA B2 0 Register"]
    pub ltc0_pkb2_0: LTC0_PKB2_0_UNION,
    #[doc = "LTC PKHA B 33 Register LTC PKHA B2 1 Register"]
    pub ltc0_pkb2_1: LTC0_PKB2_1_UNION,
    #[doc = "LTC PKHA B 34 Register LTC PKHA B2 2 Register"]
    pub ltc0_pkb2_2: LTC0_PKB2_2_UNION,
    #[doc = "LTC PKHA B 35 Register LTC PKHA B2 3 Register"]
    pub ltc0_pkb2_3: LTC0_PKB2_3_UNION,
    #[doc = "LTC PKHA B 36 Register LTC PKHA B2 4 Register"]
    pub ltc0_pkb2_4: LTC0_PKB2_4_UNION,
    #[doc = "LTC PKHA B 37 Register LTC PKHA B2 5 Register"]
    pub ltc0_pkb2_5: LTC0_PKB2_5_UNION,
    #[doc = "LTC PKHA B 38 Register LTC PKHA B2 6 Register"]
    pub ltc0_pkb2_6: LTC0_PKB2_6_UNION,
    #[doc = "LTC PKHA B 39 Register LTC PKHA B2 7 Register"]
    pub ltc0_pkb2_7: LTC0_PKB2_7_UNION,
    #[doc = "LTC PKHA B 40 Register LTC PKHA B2 8 Register"]
    pub ltc0_pkb2_8: LTC0_PKB2_8_UNION,
    #[doc = "LTC PKHA B 41 Register LTC PKHA B2 9 Register"]
    pub ltc0_pkb2_9: LTC0_PKB2_9_UNION,
    #[doc = "LTC PKHA B 42 Register LTC PKHA B2 10 Register"]
    pub ltc0_pkb_42: LTC0_PKB_42_UNION,
    #[doc = "LTC PKHA B 43 Register LTC PKHA B2 11 Register"]
    pub ltc0_pkb_43: LTC0_PKB_43_UNION,
    #[doc = "LTC PKHA B 44 Register LTC PKHA B2 12 Register"]
    pub ltc0_pkb_44: LTC0_PKB_44_UNION,
    #[doc = "LTC PKHA B 45 Register LTC PKHA B2 13 Register"]
    pub ltc0_pkb_45: LTC0_PKB_45_UNION,
    #[doc = "LTC PKHA B 46 Register LTC PKHA B2 14 Register"]
    pub ltc0_pkb_46: LTC0_PKB_46_UNION,
    #[doc = "LTC PKHA B 47 Register LTC PKHA B2 15 Register"]
    pub ltc0_pkb_47: LTC0_PKB_47_UNION,
    #[doc = "LTC PKHA B 48 Register LTC PKHA B3 0 Register"]
    pub ltc0_pkb3_0: LTC0_PKB3_0_UNION,
    #[doc = "LTC PKHA B 49 Register LTC PKHA B3 1 Register"]
    pub ltc0_pkb3_1: LTC0_PKB3_1_UNION,
    #[doc = "LTC PKHA B 50 Register LTC PKHA B3 2 Register"]
    pub ltc0_pkb3_2: LTC0_PKB3_2_UNION,
    #[doc = "LTC PKHA B 51 Register LTC PKHA B3 3 Register"]
    pub ltc0_pkb3_3: LTC0_PKB3_3_UNION,
    #[doc = "LTC PKHA B 52 Register LTC PKHA B3 4 Register"]
    pub ltc0_pkb3_4: LTC0_PKB3_4_UNION,
    #[doc = "LTC PKHA B 53 Register LTC PKHA B3 5 Register"]
    pub ltc0_pkb3_5: LTC0_PKB3_5_UNION,
    #[doc = "LTC PKHA B 54 Register LTC PKHA B3 6 Register"]
    pub ltc0_pkb3_6: LTC0_PKB3_6_UNION,
    #[doc = "LTC PKHA B 55 Register LTC PKHA B3 7 Register"]
    pub ltc0_pkb3_7: LTC0_PKB3_7_UNION,
    #[doc = "LTC PKHA B 56 Register LTC PKHA B3 8 Register"]
    pub ltc0_pkb3_8: LTC0_PKB3_8_UNION,
    #[doc = "LTC PKHA B 57 Register LTC PKHA B3 9 Register"]
    pub ltc0_pkb3_9: LTC0_PKB3_9_UNION,
    #[doc = "LTC PKHA B 58 Register LTC PKHA B3 10 Register"]
    pub ltc0_pkb_58: LTC0_PKB_58_UNION,
    #[doc = "LTC PKHA B 59 Register LTC PKHA B3 11 Register"]
    pub ltc0_pkb_59: LTC0_PKB_59_UNION,
    #[doc = "LTC PKHA B 60 Register LTC PKHA B3 12 Register"]
    pub ltc0_pkb_60: LTC0_PKB_60_UNION,
    #[doc = "LTC PKHA B 61 Register LTC PKHA B3 13 Register"]
    pub ltc0_pkb_61: LTC0_PKB_61_UNION,
    #[doc = "LTC PKHA B 62 Register LTC PKHA B3 14 Register"]
    pub ltc0_pkb_62: LTC0_PKB_62_UNION,
    #[doc = "LTC PKHA B 63 Register LTC PKHA B3 15 Register"]
    pub ltc0_pkb_63: LTC0_PKB_63_UNION,
    _reserved174: [u8; 256usize],
    #[doc = "LTC PKHA N 0 Register LTC PKHA N0 0 Register"]
    pub ltc0_pkn_0: LTC0_PKN_0_UNION,
    #[doc = "LTC PKHA N 1 Register LTC PKHA N0 1 Register"]
    pub ltc0_pkn_1: LTC0_PKN_1_UNION,
    #[doc = "LTC PKHA N 2 Register LTC PKHA N0 2 Register"]
    pub ltc0_pkn_2: LTC0_PKN_2_UNION,
    #[doc = "LTC PKHA N 3 Register LTC PKHA N0 3 Register"]
    pub ltc0_pkn_3: LTC0_PKN_3_UNION,
    #[doc = "LTC PKHA N 4 Register LTC PKHA N0 4 Register"]
    pub ltc0_pkn_4: LTC0_PKN_4_UNION,
    #[doc = "LTC PKHA N 5 Register LTC PKHA N0 5 Register"]
    pub ltc0_pkn_5: LTC0_PKN_5_UNION,
    #[doc = "LTC PKHA N 6 Register LTC PKHA N0 6 Register"]
    pub ltc0_pkn_6: LTC0_PKN_6_UNION,
    #[doc = "LTC PKHA N 7 Register LTC PKHA N0 7 Register"]
    pub ltc0_pkn_7: LTC0_PKN_7_UNION,
    #[doc = "LTC PKHA N 8 Register LTC PKHA N0 8 Register"]
    pub ltc0_pkn_8: LTC0_PKN_8_UNION,
    #[doc = "LTC PKHA N 9 Register LTC PKHA N0 9 Register"]
    pub ltc0_pkn_9: LTC0_PKN_9_UNION,
    #[doc = "LTC PKHA N 10 Register LTC PKHA N0 10 Register"]
    pub ltc0_pkn_10: LTC0_PKN_10_UNION,
    #[doc = "LTC PKHA N 11 Register LTC PKHA N0 11 Register"]
    pub ltc0_pkn_11: LTC0_PKN_11_UNION,
    #[doc = "LTC PKHA N 12 Register LTC PKHA N0 12 Register"]
    pub ltc0_pkn_12: LTC0_PKN_12_UNION,
    #[doc = "LTC PKHA N 13 Register LTC PKHA N0 13 Register"]
    pub ltc0_pkn_13: LTC0_PKN_13_UNION,
    #[doc = "LTC PKHA N 14 Register LTC PKHA N0 14 Register"]
    pub ltc0_pkn_14: LTC0_PKN_14_UNION,
    #[doc = "LTC PKHA N 15 Register LTC PKHA N0 15 Register"]
    pub ltc0_pkn_15: LTC0_PKN_15_UNION,
    #[doc = "LTC PKHA N 16 Register LTC PKHA N1 0 Register"]
    pub ltc0_pkn1_0: LTC0_PKN1_0_UNION,
    #[doc = "LTC PKHA N 17 Register LTC PKHA N1 1 Register"]
    pub ltc0_pkn1_1: LTC0_PKN1_1_UNION,
    #[doc = "LTC PKHA N 18 Register LTC PKHA N1 2 Register"]
    pub ltc0_pkn1_2: LTC0_PKN1_2_UNION,
    #[doc = "LTC PKHA N 19 Register LTC PKHA N1 3 Register"]
    pub ltc0_pkn1_3: LTC0_PKN1_3_UNION,
    #[doc = "LTC PKHA N 20 Register LTC PKHA N1 4 Register"]
    pub ltc0_pkn1_4: LTC0_PKN1_4_UNION,
    #[doc = "LTC PKHA N 21 Register LTC PKHA N1 5 Register"]
    pub ltc0_pkn1_5: LTC0_PKN1_5_UNION,
    #[doc = "LTC PKHA N 22 Register LTC PKHA N1 6 Register"]
    pub ltc0_pkn1_6: LTC0_PKN1_6_UNION,
    #[doc = "LTC PKHA N 23 Register LTC PKHA N1 7 Register"]
    pub ltc0_pkn1_7: LTC0_PKN1_7_UNION,
    #[doc = "LTC PKHA N 24 Register LTC PKHA N1 8 Register"]
    pub ltc0_pkn1_8: LTC0_PKN1_8_UNION,
    #[doc = "LTC PKHA N 25 Register LTC PKHA N1 9 Register"]
    pub ltc0_pkn1_9: LTC0_PKN1_9_UNION,
    #[doc = "LTC PKHA N 26 Register LTC PKHA N1 10 Register"]
    pub ltc0_pkn_26: LTC0_PKN_26_UNION,
    #[doc = "LTC PKHA N 27 Register LTC PKHA N1 11 Register"]
    pub ltc0_pkn_27: LTC0_PKN_27_UNION,
    #[doc = "LTC PKHA N 28 Register LTC PKHA N1 12 Register"]
    pub ltc0_pkn_28: LTC0_PKN_28_UNION,
    #[doc = "LTC PKHA N 29 Register LTC PKHA N1 13 Register"]
    pub ltc0_pkn_29: LTC0_PKN_29_UNION,
    #[doc = "LTC PKHA N 30 Register LTC PKHA N1 14 Register"]
    pub ltc0_pkn_30: LTC0_PKN_30_UNION,
    #[doc = "LTC PKHA N 31 Register LTC PKHA N1 15 Register"]
    pub ltc0_pkn_31: LTC0_PKN_31_UNION,
    #[doc = "LTC PKHA N 32 Register LTC PKHA N2 0 Register"]
    pub ltc0_pkn2_0: LTC0_PKN2_0_UNION,
    #[doc = "LTC PKHA N 33 Register LTC PKHA N2 1 Register"]
    pub ltc0_pkn2_1: LTC0_PKN2_1_UNION,
    #[doc = "LTC PKHA N 34 Register LTC PKHA N2 2 Register"]
    pub ltc0_pkn2_2: LTC0_PKN2_2_UNION,
    #[doc = "LTC PKHA N 35 Register LTC PKHA N2 3 Register"]
    pub ltc0_pkn2_3: LTC0_PKN2_3_UNION,
    #[doc = "LTC PKHA N 36 Register LTC PKHA N2 4 Register"]
    pub ltc0_pkn2_4: LTC0_PKN2_4_UNION,
    #[doc = "LTC PKHA N 37 Register LTC PKHA N2 5 Register"]
    pub ltc0_pkn2_5: LTC0_PKN2_5_UNION,
    #[doc = "LTC PKHA N 38 Register LTC PKHA N2 6 Register"]
    pub ltc0_pkn2_6: LTC0_PKN2_6_UNION,
    #[doc = "LTC PKHA N 39 Register LTC PKHA N2 7 Register"]
    pub ltc0_pkn2_7: LTC0_PKN2_7_UNION,
    #[doc = "LTC PKHA N 40 Register LTC PKHA N2 8 Register"]
    pub ltc0_pkn2_8: LTC0_PKN2_8_UNION,
    #[doc = "LTC PKHA N 41 Register LTC PKHA N2 9 Register"]
    pub ltc0_pkn2_9: LTC0_PKN2_9_UNION,
    #[doc = "LTC PKHA N 42 Register LTC PKHA N2 10 Register"]
    pub ltc0_pkn_42: LTC0_PKN_42_UNION,
    #[doc = "LTC PKHA N 43 Register LTC PKHA N2 11 Register"]
    pub ltc0_pkn_43: LTC0_PKN_43_UNION,
    #[doc = "LTC PKHA N 44 Register LTC PKHA N2 12 Register"]
    pub ltc0_pkn_44: LTC0_PKN_44_UNION,
    #[doc = "LTC PKHA N 45 Register LTC PKHA N2 13 Register"]
    pub ltc0_pkn_45: LTC0_PKN_45_UNION,
    #[doc = "LTC PKHA N 46 Register LTC PKHA N2 14 Register"]
    pub ltc0_pkn_46: LTC0_PKN_46_UNION,
    #[doc = "LTC PKHA N 47 Register LTC PKHA N2 15 Register"]
    pub ltc0_pkn_47: LTC0_PKN_47_UNION,
    #[doc = "LTC PKHA N 48 Register LTC PKHA N3 0 Register"]
    pub ltc0_pkn3_0: LTC0_PKN3_0_UNION,
    #[doc = "LTC PKHA N 49 Register LTC PKHA N3 1 Register"]
    pub ltc0_pkn3_1: LTC0_PKN3_1_UNION,
    #[doc = "LTC PKHA N 50 Register LTC PKHA N3 2 Register"]
    pub ltc0_pkn3_2: LTC0_PKN3_2_UNION,
    #[doc = "LTC PKHA N 51 Register LTC PKHA N3 3 Register"]
    pub ltc0_pkn3_3: LTC0_PKN3_3_UNION,
    #[doc = "LTC PKHA N 52 Register LTC PKHA N3 4 Register"]
    pub ltc0_pkn3_4: LTC0_PKN3_4_UNION,
    #[doc = "LTC PKHA N 53 Register LTC PKHA N3 5 Register"]
    pub ltc0_pkn3_5: LTC0_PKN3_5_UNION,
    #[doc = "LTC PKHA N 54 Register LTC PKHA N3 6 Register"]
    pub ltc0_pkn3_6: LTC0_PKN3_6_UNION,
    #[doc = "LTC PKHA N 55 Register LTC PKHA N3 7 Register"]
    pub ltc0_pkn3_7: LTC0_PKN3_7_UNION,
    #[doc = "LTC PKHA N 56 Register LTC PKHA N3 8 Register"]
    pub ltc0_pkn3_8: LTC0_PKN3_8_UNION,
    #[doc = "LTC PKHA N 57 Register LTC PKHA N3 9 Register"]
    pub ltc0_pkn3_9: LTC0_PKN3_9_UNION,
    #[doc = "LTC PKHA N 58 Register LTC PKHA N3 10 Register"]
    pub ltc0_pkn_58: LTC0_PKN_58_UNION,
    #[doc = "LTC PKHA N 59 Register LTC PKHA N3 11 Register"]
    pub ltc0_pkn_59: LTC0_PKN_59_UNION,
    #[doc = "LTC PKHA N 60 Register LTC PKHA N3 12 Register"]
    pub ltc0_pkn_60: LTC0_PKN_60_UNION,
    #[doc = "LTC PKHA N 61 Register LTC PKHA N3 13 Register"]
    pub ltc0_pkn_61: LTC0_PKN_61_UNION,
    #[doc = "LTC PKHA N 62 Register LTC PKHA N3 14 Register"]
    pub ltc0_pkn_62: LTC0_PKN_62_UNION,
    #[doc = "LTC PKHA N 63 Register LTC PKHA N3 15 Register"]
    pub ltc0_pkn_63: LTC0_PKN_63_UNION,
    _reserved238: [u8; 256usize],
    #[doc = "LTC PKHA E 0 Register LTC PKHA E0 0 Register"]
    pub ltc0_pke_0: LTC0_PKE_0_UNION,
    #[doc = "LTC PKHA E 1 Register LTC PKHA E0 1 Register"]
    pub ltc0_pke_1: LTC0_PKE_1_UNION,
    #[doc = "LTC PKHA E 2 Register LTC PKHA E0 2 Register"]
    pub ltc0_pke_2: LTC0_PKE_2_UNION,
    #[doc = "LTC PKHA E 3 Register LTC PKHA E0 3 Register"]
    pub ltc0_pke_3: LTC0_PKE_3_UNION,
    #[doc = "LTC PKHA E 4 Register LTC PKHA E0 4 Register"]
    pub ltc0_pke_4: LTC0_PKE_4_UNION,
    #[doc = "LTC PKHA E 5 Register LTC PKHA E0 5 Register"]
    pub ltc0_pke_5: LTC0_PKE_5_UNION,
    #[doc = "LTC PKHA E 6 Register LTC PKHA E0 6 Register"]
    pub ltc0_pke_6: LTC0_PKE_6_UNION,
    #[doc = "LTC PKHA E 7 Register LTC PKHA E0 7 Register"]
    pub ltc0_pke_7: LTC0_PKE_7_UNION,
    #[doc = "LTC PKHA E 8 Register LTC PKHA E0 8 Register"]
    pub ltc0_pke_8: LTC0_PKE_8_UNION,
    #[doc = "LTC PKHA E 9 Register LTC PKHA E0 9 Register"]
    pub ltc0_pke_9: LTC0_PKE_9_UNION,
    #[doc = "LTC PKHA E 10 Register LTC PKHA E0 10 Register"]
    pub ltc0_pke_10: LTC0_PKE_10_UNION,
    #[doc = "LTC PKHA E 11 Register LTC PKHA E0 11 Register"]
    pub ltc0_pke_11: LTC0_PKE_11_UNION,
    #[doc = "LTC PKHA E 12 Register LTC PKHA E0 12 Register"]
    pub ltc0_pke_12: LTC0_PKE_12_UNION,
    #[doc = "LTC PKHA E 13 Register LTC PKHA E0 13 Register"]
    pub ltc0_pke_13: LTC0_PKE_13_UNION,
    #[doc = "LTC PKHA E 14 Register LTC PKHA E0 14 Register"]
    pub ltc0_pke_14: LTC0_PKE_14_UNION,
    #[doc = "LTC PKHA E 15 Register LTC PKHA E0 15 Register"]
    pub ltc0_pke_15: LTC0_PKE_15_UNION,
    #[doc = "LTC PKHA E 16 Register LTC PKHA E1 0 Register"]
    pub ltc0_pke1_0: LTC0_PKE1_0_UNION,
    #[doc = "LTC PKHA E 17 Register LTC PKHA E1 1 Register"]
    pub ltc0_pke1_1: LTC0_PKE1_1_UNION,
    #[doc = "LTC PKHA E 18 Register LTC PKHA E1 2 Register"]
    pub ltc0_pke1_2: LTC0_PKE1_2_UNION,
    #[doc = "LTC PKHA E 19 Register LTC PKHA E1 3 Register"]
    pub ltc0_pke1_3: LTC0_PKE1_3_UNION,
    #[doc = "LTC PKHA E 20 Register LTC PKHA E1 4 Register"]
    pub ltc0_pke1_4: LTC0_PKE1_4_UNION,
    #[doc = "LTC PKHA E 21 Register LTC PKHA E1 5 Register"]
    pub ltc0_pke1_5: LTC0_PKE1_5_UNION,
    #[doc = "LTC PKHA E 22 Register LTC PKHA E1 6 Register"]
    pub ltc0_pke1_6: LTC0_PKE1_6_UNION,
    #[doc = "LTC PKHA E 23 Register LTC PKHA E1 7 Register"]
    pub ltc0_pke1_7: LTC0_PKE1_7_UNION,
    #[doc = "LTC PKHA E 24 Register LTC PKHA E1 8 Register"]
    pub ltc0_pke1_8: LTC0_PKE1_8_UNION,
    #[doc = "LTC PKHA E 25 Register LTC PKHA E1 9 Register"]
    pub ltc0_pke1_9: LTC0_PKE1_9_UNION,
    #[doc = "LTC PKHA E 26 Register LTC PKHA E1 10 Register"]
    pub ltc0_pke_26: LTC0_PKE_26_UNION,
    #[doc = "LTC PKHA E 27 Register LTC PKHA E1 11 Register"]
    pub ltc0_pke_27: LTC0_PKE_27_UNION,
    #[doc = "LTC PKHA E 28 Register LTC PKHA E1 12 Register"]
    pub ltc0_pke_28: LTC0_PKE_28_UNION,
    #[doc = "LTC PKHA E 29 Register LTC PKHA E1 13 Register"]
    pub ltc0_pke_29: LTC0_PKE_29_UNION,
    #[doc = "LTC PKHA E 30 Register LTC PKHA E1 14 Register"]
    pub ltc0_pke_30: LTC0_PKE_30_UNION,
    #[doc = "LTC PKHA E 31 Register LTC PKHA E1 15 Register"]
    pub ltc0_pke_31: LTC0_PKE_31_UNION,
    #[doc = "LTC PKHA E 32 Register LTC PKHA E2 0 Register"]
    pub ltc0_pke2_0: LTC0_PKE2_0_UNION,
    #[doc = "LTC PKHA E 33 Register LTC PKHA E2 1 Register"]
    pub ltc0_pke2_1: LTC0_PKE2_1_UNION,
    #[doc = "LTC PKHA E 34 Register LTC PKHA E2 2 Register"]
    pub ltc0_pke2_2: LTC0_PKE2_2_UNION,
    #[doc = "LTC PKHA E 35 Register LTC PKHA E2 3 Register"]
    pub ltc0_pke2_3: LTC0_PKE2_3_UNION,
    #[doc = "LTC PKHA E 36 Register LTC PKHA E2 4 Register"]
    pub ltc0_pke2_4: LTC0_PKE2_4_UNION,
    #[doc = "LTC PKHA E 37 Register LTC PKHA E2 5 Register"]
    pub ltc0_pke2_5: LTC0_PKE2_5_UNION,
    #[doc = "LTC PKHA E 38 Register LTC PKHA E2 6 Register"]
    pub ltc0_pke2_6: LTC0_PKE2_6_UNION,
    #[doc = "LTC PKHA E 39 Register LTC PKHA E2 7 Register"]
    pub ltc0_pke2_7: LTC0_PKE2_7_UNION,
    #[doc = "LTC PKHA E 40 Register LTC PKHA E2 8 Register"]
    pub ltc0_pke2_8: LTC0_PKE2_8_UNION,
    #[doc = "LTC PKHA E 41 Register LTC PKHA E2 9 Register"]
    pub ltc0_pke2_9: LTC0_PKE2_9_UNION,
    #[doc = "LTC PKHA E 42 Register LTC PKHA E2 10 Register"]
    pub ltc0_pke_42: LTC0_PKE_42_UNION,
    #[doc = "LTC PKHA E 43 Register LTC PKHA E2 11 Register"]
    pub ltc0_pke_43: LTC0_PKE_43_UNION,
    #[doc = "LTC PKHA E 44 Register LTC PKHA E2 12 Register"]
    pub ltc0_pke_44: LTC0_PKE_44_UNION,
    #[doc = "LTC PKHA E 45 Register LTC PKHA E2 13 Register"]
    pub ltc0_pke_45: LTC0_PKE_45_UNION,
    #[doc = "LTC PKHA E 46 Register LTC PKHA E2 14 Register"]
    pub ltc0_pke_46: LTC0_PKE_46_UNION,
    #[doc = "LTC PKHA E 47 Register LTC PKHA E2 15 Register"]
    pub ltc0_pke_47: LTC0_PKE_47_UNION,
    #[doc = "LTC PKHA E 48 Register LTC PKHA E3 0 Register"]
    pub ltc0_pke3_0: LTC0_PKE3_0_UNION,
    #[doc = "LTC PKHA E 49 Register LTC PKHA E3 1 Register"]
    pub ltc0_pke3_1: LTC0_PKE3_1_UNION,
    #[doc = "LTC PKHA E 50 Register LTC PKHA E3 2 Register"]
    pub ltc0_pke3_2: LTC0_PKE3_2_UNION,
    #[doc = "LTC PKHA E 51 Register LTC PKHA E3 3 Register"]
    pub ltc0_pke3_3: LTC0_PKE3_3_UNION,
    #[doc = "LTC PKHA E 52 Register LTC PKHA E3 4 Register"]
    pub ltc0_pke3_4: LTC0_PKE3_4_UNION,
    #[doc = "LTC PKHA E 53 Register LTC PKHA E3 5 Register"]
    pub ltc0_pke3_5: LTC0_PKE3_5_UNION,
    #[doc = "LTC PKHA E 54 Register LTC PKHA E3 6 Register"]
    pub ltc0_pke3_6: LTC0_PKE3_6_UNION,
    #[doc = "LTC PKHA E 55 Register LTC PKHA E3 7 Register"]
    pub ltc0_pke3_7: LTC0_PKE3_7_UNION,
    #[doc = "LTC PKHA E 56 Register LTC PKHA E3 8 Register"]
    pub ltc0_pke3_8: LTC0_PKE3_8_UNION,
    #[doc = "LTC PKHA E 57 Register LTC PKHA E3 9 Register"]
    pub ltc0_pke3_9: LTC0_PKE3_9_UNION,
    #[doc = "LTC PKHA E 58 Register LTC PKHA E3 10 Register"]
    pub ltc0_pke_58: LTC0_PKE_58_UNION,
    #[doc = "LTC PKHA E 59 Register LTC PKHA E3 11 Register"]
    pub ltc0_pke_59: LTC0_PKE_59_UNION,
    #[doc = "LTC PKHA E 60 Register LTC PKHA E3 12 Register"]
    pub ltc0_pke_60: LTC0_PKE_60_UNION,
    #[doc = "LTC PKHA E 61 Register LTC PKHA E3 13 Register"]
    pub ltc0_pke_61: LTC0_PKE_61_UNION,
    #[doc = "LTC PKHA E 62 Register LTC PKHA E3 14 Register"]
    pub ltc0_pke_62: LTC0_PKE_62_UNION,
    #[doc = "LTC PKHA E 63 Register LTC PKHA E3 15 Register"]
    pub ltc0_pke_63: LTC0_PKE_63_UNION,
}
#[doc = "LTC Mode Register (PublicKey) LTC Mode Register (non-PKHA/non-RNG use)"]
#[repr(C)]
pub union LTC0_UNION {
    #[doc = "0x00 - LTC Mode Register (PublicKey)"]
    pub ltc0_mdpk: LTC0_MDPK,
    #[doc = "0x00 - LTC Mode Register (non-PKHA/non-RNG use)"]
    pub ltc0_md: LTC0_MD,
}
#[doc = "LTC PKHA A 0 Register LTC PKHA A0 0 Register"]
#[repr(C)]
pub union LTC0_PKA_0_UNION {
    #[doc = "0x800 - LTC PKHA A 0 Register"]
    pub ltc0_pka_0: LTC0_PKA_0,
    #[doc = "0x800 - LTC PKHA A0 0 Register"]
    pub ltc0_pka0_0: LTC0_PKA0_0,
}
#[doc = "LTC PKHA A 1 Register LTC PKHA A0 1 Register"]
#[repr(C)]
pub union LTC0_PKA_1_UNION {
    #[doc = "0x804 - LTC PKHA A 1 Register"]
    pub ltc0_pka_1: LTC0_PKA_1,
    #[doc = "0x804 - LTC PKHA A0 1 Register"]
    pub ltc0_pka0_1: LTC0_PKA0_1,
}
#[doc = "LTC PKHA A 2 Register LTC PKHA A0 2 Register"]
#[repr(C)]
pub union LTC0_PKA_2_UNION {
    #[doc = "0x808 - LTC PKHA A 2 Register"]
    pub ltc0_pka_2: LTC0_PKA_2,
    #[doc = "0x808 - LTC PKHA A0 2 Register"]
    pub ltc0_pka0_2: LTC0_PKA0_2,
}
#[doc = "LTC PKHA A 3 Register LTC PKHA A0 3 Register"]
#[repr(C)]
pub union LTC0_PKA_3_UNION {
    #[doc = "0x80c - LTC PKHA A 3 Register"]
    pub ltc0_pka_3: LTC0_PKA_3,
    #[doc = "0x80c - LTC PKHA A0 3 Register"]
    pub ltc0_pka0_3: LTC0_PKA0_3,
}
#[doc = "LTC PKHA A 4 Register LTC PKHA A0 4 Register"]
#[repr(C)]
pub union LTC0_PKA_4_UNION {
    #[doc = "0x810 - LTC PKHA A 4 Register"]
    pub ltc0_pka_4: LTC0_PKA_4,
    #[doc = "0x810 - LTC PKHA A0 4 Register"]
    pub ltc0_pka0_4: LTC0_PKA0_4,
}
#[doc = "LTC PKHA A 5 Register LTC PKHA A0 5 Register"]
#[repr(C)]
pub union LTC0_PKA_5_UNION {
    #[doc = "0x814 - LTC PKHA A 5 Register"]
    pub ltc0_pka_5: LTC0_PKA_5,
    #[doc = "0x814 - LTC PKHA A0 5 Register"]
    pub ltc0_pka0_5: LTC0_PKA0_5,
}
#[doc = "LTC PKHA A 6 Register LTC PKHA A0 6 Register"]
#[repr(C)]
pub union LTC0_PKA_6_UNION {
    #[doc = "0x818 - LTC PKHA A 6 Register"]
    pub ltc0_pka_6: LTC0_PKA_6,
    #[doc = "0x818 - LTC PKHA A0 6 Register"]
    pub ltc0_pka0_6: LTC0_PKA0_6,
}
#[doc = "LTC PKHA A 7 Register LTC PKHA A0 7 Register"]
#[repr(C)]
pub union LTC0_PKA_7_UNION {
    #[doc = "0x81c - LTC PKHA A 7 Register"]
    pub ltc0_pka_7: LTC0_PKA_7,
    #[doc = "0x81c - LTC PKHA A0 7 Register"]
    pub ltc0_pka0_7: LTC0_PKA0_7,
}
#[doc = "LTC PKHA A 8 Register LTC PKHA A0 8 Register"]
#[repr(C)]
pub union LTC0_PKA_8_UNION {
    #[doc = "0x820 - LTC PKHA A 8 Register"]
    pub ltc0_pka_8: LTC0_PKA_8,
    #[doc = "0x820 - LTC PKHA A0 8 Register"]
    pub ltc0_pka0_8: LTC0_PKA0_8,
}
#[doc = "LTC PKHA A 9 Register LTC PKHA A0 9 Register"]
#[repr(C)]
pub union LTC0_PKA_9_UNION {
    #[doc = "0x824 - LTC PKHA A 9 Register"]
    pub ltc0_pka_9: LTC0_PKA_9,
    #[doc = "0x824 - LTC PKHA A0 9 Register"]
    pub ltc0_pka0_9: LTC0_PKA0_9,
}
#[doc = "LTC PKHA A 10 Register LTC PKHA A0 10 Register"]
#[repr(C)]
pub union LTC0_PKA_10_UNION {
    #[doc = "0x828 - LTC PKHA A 10 Register"]
    pub ltc0_pka_10: LTC0_PKA_10,
    #[doc = "0x828 - LTC PKHA A0 10 Register"]
    pub ltc0_pka0_10: LTC0_PKA0_10,
}
#[doc = "LTC PKHA A 11 Register LTC PKHA A0 11 Register"]
#[repr(C)]
pub union LTC0_PKA_11_UNION {
    #[doc = "0x82c - LTC PKHA A 11 Register"]
    pub ltc0_pka_11: LTC0_PKA_11,
    #[doc = "0x82c - LTC PKHA A0 11 Register"]
    pub ltc0_pka0_11: LTC0_PKA0_11,
}
#[doc = "LTC PKHA A 12 Register LTC PKHA A0 12 Register"]
#[repr(C)]
pub union LTC0_PKA_12_UNION {
    #[doc = "0x830 - LTC PKHA A 12 Register"]
    pub ltc0_pka_12: LTC0_PKA_12,
    #[doc = "0x830 - LTC PKHA A0 12 Register"]
    pub ltc0_pka0_12: LTC0_PKA0_12,
}
#[doc = "LTC PKHA A 13 Register LTC PKHA A0 13 Register"]
#[repr(C)]
pub union LTC0_PKA_13_UNION {
    #[doc = "0x834 - LTC PKHA A 13 Register"]
    pub ltc0_pka_13: LTC0_PKA_13,
    #[doc = "0x834 - LTC PKHA A0 13 Register"]
    pub ltc0_pka0_13: LTC0_PKA0_13,
}
#[doc = "LTC PKHA A 14 Register LTC PKHA A0 14 Register"]
#[repr(C)]
pub union LTC0_PKA_14_UNION {
    #[doc = "0x838 - LTC PKHA A 14 Register"]
    pub ltc0_pka_14: LTC0_PKA_14,
    #[doc = "0x838 - LTC PKHA A0 14 Register"]
    pub ltc0_pka0_14: LTC0_PKA0_14,
}
#[doc = "LTC PKHA A 15 Register LTC PKHA A0 15 Register"]
#[repr(C)]
pub union LTC0_PKA_15_UNION {
    #[doc = "0x83c - LTC PKHA A 15 Register"]
    pub ltc0_pka_15: LTC0_PKA_15,
    #[doc = "0x83c - LTC PKHA A0 15 Register"]
    pub ltc0_pka0_15: LTC0_PKA0_15,
}
#[doc = "LTC PKHA A 16 Register LTC PKHA A1 0 Register"]
#[repr(C)]
pub union LTC0_PKA1_0_UNION {
    #[doc = "0x840 - LTC PKHA A 16 Register"]
    pub ltc0_pka_16: LTC0_PKA_16,
    #[doc = "0x840 - LTC PKHA A1 0 Register"]
    pub ltc0_pka1_0: LTC0_PKA1_0,
}
#[doc = "LTC PKHA A 17 Register LTC PKHA A1 1 Register"]
#[repr(C)]
pub union LTC0_PKA1_1_UNION {
    #[doc = "0x844 - LTC PKHA A 17 Register"]
    pub ltc0_pka_17: LTC0_PKA_17,
    #[doc = "0x844 - LTC PKHA A1 1 Register"]
    pub ltc0_pka1_1: LTC0_PKA1_1,
}
#[doc = "LTC PKHA A 18 Register LTC PKHA A1 2 Register"]
#[repr(C)]
pub union LTC0_PKA1_2_UNION {
    #[doc = "0x848 - LTC PKHA A 18 Register"]
    pub ltc0_pka_18: LTC0_PKA_18,
    #[doc = "0x848 - LTC PKHA A1 2 Register"]
    pub ltc0_pka1_2: LTC0_PKA1_2,
}
#[doc = "LTC PKHA A 19 Register LTC PKHA A1 3 Register"]
#[repr(C)]
pub union LTC0_PKA1_3_UNION {
    #[doc = "0x84c - LTC PKHA A 19 Register"]
    pub ltc0_pka_19: LTC0_PKA_19,
    #[doc = "0x84c - LTC PKHA A1 3 Register"]
    pub ltc0_pka1_3: LTC0_PKA1_3,
}
#[doc = "LTC PKHA A 20 Register LTC PKHA A1 4 Register"]
#[repr(C)]
pub union LTC0_PKA1_4_UNION {
    #[doc = "0x850 - LTC PKHA A 20 Register"]
    pub ltc0_pka_20: LTC0_PKA_20,
    #[doc = "0x850 - LTC PKHA A1 4 Register"]
    pub ltc0_pka1_4: LTC0_PKA1_4,
}
#[doc = "LTC PKHA A 21 Register LTC PKHA A1 5 Register"]
#[repr(C)]
pub union LTC0_PKA1_5_UNION {
    #[doc = "0x854 - LTC PKHA A 21 Register"]
    pub ltc0_pka_21: LTC0_PKA_21,
    #[doc = "0x854 - LTC PKHA A1 5 Register"]
    pub ltc0_pka1_5: LTC0_PKA1_5,
}
#[doc = "LTC PKHA A 22 Register LTC PKHA A1 6 Register"]
#[repr(C)]
pub union LTC0_PKA1_6_UNION {
    #[doc = "0x858 - LTC PKHA A 22 Register"]
    pub ltc0_pka_22: LTC0_PKA_22,
    #[doc = "0x858 - LTC PKHA A1 6 Register"]
    pub ltc0_pka1_6: LTC0_PKA1_6,
}
#[doc = "LTC PKHA A 23 Register LTC PKHA A1 7 Register"]
#[repr(C)]
pub union LTC0_PKA1_7_UNION {
    #[doc = "0x85c - LTC PKHA A 23 Register"]
    pub ltc0_pka_23: LTC0_PKA_23,
    #[doc = "0x85c - LTC PKHA A1 7 Register"]
    pub ltc0_pka1_7: LTC0_PKA1_7,
}
#[doc = "LTC PKHA A 24 Register LTC PKHA A1 8 Register"]
#[repr(C)]
pub union LTC0_PKA1_8_UNION {
    #[doc = "0x860 - LTC PKHA A 24 Register"]
    pub ltc0_pka_24: LTC0_PKA_24,
    #[doc = "0x860 - LTC PKHA A1 8 Register"]
    pub ltc0_pka1_8: LTC0_PKA1_8,
}
#[doc = "LTC PKHA A 25 Register LTC PKHA A1 9 Register"]
#[repr(C)]
pub union LTC0_PKA1_9_UNION {
    #[doc = "0x864 - LTC PKHA A 25 Register"]
    pub ltc0_pka_25: LTC0_PKA_25,
    #[doc = "0x864 - LTC PKHA A1 9 Register"]
    pub ltc0_pka1_9: LTC0_PKA1_9,
}
#[doc = "LTC PKHA A 26 Register LTC PKHA A1 10 Register"]
#[repr(C)]
pub union LTC0_PKA_26_UNION {
    #[doc = "0x868 - LTC PKHA A 26 Register"]
    pub ltc0_pka_26: LTC0_PKA_26,
    #[doc = "0x868 - LTC PKHA A1 10 Register"]
    pub ltc0_pka1_10: LTC0_PKA1_10,
}
#[doc = "LTC PKHA A 27 Register LTC PKHA A1 11 Register"]
#[repr(C)]
pub union LTC0_PKA_27_UNION {
    #[doc = "0x86c - LTC PKHA A 27 Register"]
    pub ltc0_pka_27: LTC0_PKA_27,
    #[doc = "0x86c - LTC PKHA A1 11 Register"]
    pub ltc0_pka1_11: LTC0_PKA1_11,
}
#[doc = "LTC PKHA A 28 Register LTC PKHA A1 12 Register"]
#[repr(C)]
pub union LTC0_PKA_28_UNION {
    #[doc = "0x870 - LTC PKHA A 28 Register"]
    pub ltc0_pka_28: LTC0_PKA_28,
    #[doc = "0x870 - LTC PKHA A1 12 Register"]
    pub ltc0_pka1_12: LTC0_PKA1_12,
}
#[doc = "LTC PKHA A 29 Register LTC PKHA A1 13 Register"]
#[repr(C)]
pub union LTC0_PKA_29_UNION {
    #[doc = "0x874 - LTC PKHA A 29 Register"]
    pub ltc0_pka_29: LTC0_PKA_29,
    #[doc = "0x874 - LTC PKHA A1 13 Register"]
    pub ltc0_pka1_13: LTC0_PKA1_13,
}
#[doc = "LTC PKHA A 30 Register LTC PKHA A1 14 Register"]
#[repr(C)]
pub union LTC0_PKA_30_UNION {
    #[doc = "0x878 - LTC PKHA A 30 Register"]
    pub ltc0_pka_30: LTC0_PKA_30,
    #[doc = "0x878 - LTC PKHA A1 14 Register"]
    pub ltc0_pka1_14: LTC0_PKA1_14,
}
#[doc = "LTC PKHA A 31 Register LTC PKHA A1 15 Register"]
#[repr(C)]
pub union LTC0_PKA_31_UNION {
    #[doc = "0x87c - LTC PKHA A 31 Register"]
    pub ltc0_pka_31: LTC0_PKA_31,
    #[doc = "0x87c - LTC PKHA A1 15 Register"]
    pub ltc0_pka1_15: LTC0_PKA1_15,
}
#[doc = "LTC PKHA A 32 Register LTC PKHA A2 0 Register"]
#[repr(C)]
pub union LTC0_PKA2_0_UNION {
    #[doc = "0x880 - LTC PKHA A 32 Register"]
    pub ltc0_pka_32: LTC0_PKA_32,
    #[doc = "0x880 - LTC PKHA A2 0 Register"]
    pub ltc0_pka2_0: LTC0_PKA2_0,
}
#[doc = "LTC PKHA A 33 Register LTC PKHA A2 1 Register"]
#[repr(C)]
pub union LTC0_PKA2_1_UNION {
    #[doc = "0x884 - LTC PKHA A 33 Register"]
    pub ltc0_pka_33: LTC0_PKA_33,
    #[doc = "0x884 - LTC PKHA A2 1 Register"]
    pub ltc0_pka2_1: LTC0_PKA2_1,
}
#[doc = "LTC PKHA A 34 Register LTC PKHA A2 2 Register"]
#[repr(C)]
pub union LTC0_PKA2_2_UNION {
    #[doc = "0x888 - LTC PKHA A 34 Register"]
    pub ltc0_pka_34: LTC0_PKA_34,
    #[doc = "0x888 - LTC PKHA A2 2 Register"]
    pub ltc0_pka2_2: LTC0_PKA2_2,
}
#[doc = "LTC PKHA A 35 Register LTC PKHA A2 3 Register"]
#[repr(C)]
pub union LTC0_PKA2_3_UNION {
    #[doc = "0x88c - LTC PKHA A 35 Register"]
    pub ltc0_pka_35: LTC0_PKA_35,
    #[doc = "0x88c - LTC PKHA A2 3 Register"]
    pub ltc0_pka2_3: LTC0_PKA2_3,
}
#[doc = "LTC PKHA A 36 Register LTC PKHA A2 4 Register"]
#[repr(C)]
pub union LTC0_PKA2_4_UNION {
    #[doc = "0x890 - LTC PKHA A 36 Register"]
    pub ltc0_pka_36: LTC0_PKA_36,
    #[doc = "0x890 - LTC PKHA A2 4 Register"]
    pub ltc0_pka2_4: LTC0_PKA2_4,
}
#[doc = "LTC PKHA A 37 Register LTC PKHA A2 5 Register"]
#[repr(C)]
pub union LTC0_PKA2_5_UNION {
    #[doc = "0x894 - LTC PKHA A 37 Register"]
    pub ltc0_pka_37: LTC0_PKA_37,
    #[doc = "0x894 - LTC PKHA A2 5 Register"]
    pub ltc0_pka2_5: LTC0_PKA2_5,
}
#[doc = "LTC PKHA A 38 Register LTC PKHA A2 6 Register"]
#[repr(C)]
pub union LTC0_PKA2_6_UNION {
    #[doc = "0x898 - LTC PKHA A 38 Register"]
    pub ltc0_pka_38: LTC0_PKA_38,
    #[doc = "0x898 - LTC PKHA A2 6 Register"]
    pub ltc0_pka2_6: LTC0_PKA2_6,
}
#[doc = "LTC PKHA A 39 Register LTC PKHA A2 7 Register"]
#[repr(C)]
pub union LTC0_PKA2_7_UNION {
    #[doc = "0x89c - LTC PKHA A 39 Register"]
    pub ltc0_pka_39: LTC0_PKA_39,
    #[doc = "0x89c - LTC PKHA A2 7 Register"]
    pub ltc0_pka2_7: LTC0_PKA2_7,
}
#[doc = "LTC PKHA A 40 Register LTC PKHA A2 8 Register"]
#[repr(C)]
pub union LTC0_PKA2_8_UNION {
    #[doc = "0x8a0 - LTC PKHA A 40 Register"]
    pub ltc0_pka_40: LTC0_PKA_40,
    #[doc = "0x8a0 - LTC PKHA A2 8 Register"]
    pub ltc0_pka2_8: LTC0_PKA2_8,
}
#[doc = "LTC PKHA A 41 Register LTC PKHA A2 9 Register"]
#[repr(C)]
pub union LTC0_PKA2_9_UNION {
    #[doc = "0x8a4 - LTC PKHA A 41 Register"]
    pub ltc0_pka_41: LTC0_PKA_41,
    #[doc = "0x8a4 - LTC PKHA A2 9 Register"]
    pub ltc0_pka2_9: LTC0_PKA2_9,
}
#[doc = "LTC PKHA A 42 Register LTC PKHA A2 10 Register"]
#[repr(C)]
pub union LTC0_PKA_42_UNION {
    #[doc = "0x8a8 - LTC PKHA A 42 Register"]
    pub ltc0_pka_42: LTC0_PKA_42,
    #[doc = "0x8a8 - LTC PKHA A2 10 Register"]
    pub ltc0_pka2_10: LTC0_PKA2_10,
}
#[doc = "LTC PKHA A 43 Register LTC PKHA A2 11 Register"]
#[repr(C)]
pub union LTC0_PKA_43_UNION {
    #[doc = "0x8ac - LTC PKHA A 43 Register"]
    pub ltc0_pka_43: LTC0_PKA_43,
    #[doc = "0x8ac - LTC PKHA A2 11 Register"]
    pub ltc0_pka2_11: LTC0_PKA2_11,
}
#[doc = "LTC PKHA A 44 Register LTC PKHA A2 12 Register"]
#[repr(C)]
pub union LTC0_PKA_44_UNION {
    #[doc = "0x8b0 - LTC PKHA A 44 Register"]
    pub ltc0_pka_44: LTC0_PKA_44,
    #[doc = "0x8b0 - LTC PKHA A2 12 Register"]
    pub ltc0_pka2_12: LTC0_PKA2_12,
}
#[doc = "LTC PKHA A 45 Register LTC PKHA A2 13 Register"]
#[repr(C)]
pub union LTC0_PKA_45_UNION {
    #[doc = "0x8b4 - LTC PKHA A 45 Register"]
    pub ltc0_pka_45: LTC0_PKA_45,
    #[doc = "0x8b4 - LTC PKHA A2 13 Register"]
    pub ltc0_pka2_13: LTC0_PKA2_13,
}
#[doc = "LTC PKHA A 46 Register LTC PKHA A2 14 Register"]
#[repr(C)]
pub union LTC0_PKA_46_UNION {
    #[doc = "0x8b8 - LTC PKHA A 46 Register"]
    pub ltc0_pka_46: LTC0_PKA_46,
    #[doc = "0x8b8 - LTC PKHA A2 14 Register"]
    pub ltc0_pka2_14: LTC0_PKA2_14,
}
#[doc = "LTC PKHA A 47 Register LTC PKHA A2 15 Register"]
#[repr(C)]
pub union LTC0_PKA_47_UNION {
    #[doc = "0x8bc - LTC PKHA A 47 Register"]
    pub ltc0_pka_47: LTC0_PKA_47,
    #[doc = "0x8bc - LTC PKHA A2 15 Register"]
    pub ltc0_pka2_15: LTC0_PKA2_15,
}
#[doc = "LTC PKHA A 48 Register LTC PKHA A3 0 Register"]
#[repr(C)]
pub union LTC0_PKA3_0_UNION {
    #[doc = "0x8c0 - LTC PKHA A 48 Register"]
    pub ltc0_pka_48: LTC0_PKA_48,
    #[doc = "0x8c0 - LTC PKHA A3 0 Register"]
    pub ltc0_pka3_0: LTC0_PKA3_0,
}
#[doc = "LTC PKHA A 49 Register LTC PKHA A3 1 Register"]
#[repr(C)]
pub union LTC0_PKA3_1_UNION {
    #[doc = "0x8c4 - LTC PKHA A 49 Register"]
    pub ltc0_pka_49: LTC0_PKA_49,
    #[doc = "0x8c4 - LTC PKHA A3 1 Register"]
    pub ltc0_pka3_1: LTC0_PKA3_1,
}
#[doc = "LTC PKHA A 50 Register LTC PKHA A3 2 Register"]
#[repr(C)]
pub union LTC0_PKA3_2_UNION {
    #[doc = "0x8c8 - LTC PKHA A 50 Register"]
    pub ltc0_pka_50: LTC0_PKA_50,
    #[doc = "0x8c8 - LTC PKHA A3 2 Register"]
    pub ltc0_pka3_2: LTC0_PKA3_2,
}
#[doc = "LTC PKHA A 51 Register LTC PKHA A3 3 Register"]
#[repr(C)]
pub union LTC0_PKA3_3_UNION {
    #[doc = "0x8cc - LTC PKHA A 51 Register"]
    pub ltc0_pka_51: LTC0_PKA_51,
    #[doc = "0x8cc - LTC PKHA A3 3 Register"]
    pub ltc0_pka3_3: LTC0_PKA3_3,
}
#[doc = "LTC PKHA A 52 Register LTC PKHA A3 4 Register"]
#[repr(C)]
pub union LTC0_PKA3_4_UNION {
    #[doc = "0x8d0 - LTC PKHA A 52 Register"]
    pub ltc0_pka_52: LTC0_PKA_52,
    #[doc = "0x8d0 - LTC PKHA A3 4 Register"]
    pub ltc0_pka3_4: LTC0_PKA3_4,
}
#[doc = "LTC PKHA A 53 Register LTC PKHA A3 5 Register"]
#[repr(C)]
pub union LTC0_PKA3_5_UNION {
    #[doc = "0x8d4 - LTC PKHA A 53 Register"]
    pub ltc0_pka_53: LTC0_PKA_53,
    #[doc = "0x8d4 - LTC PKHA A3 5 Register"]
    pub ltc0_pka3_5: LTC0_PKA3_5,
}
#[doc = "LTC PKHA A 54 Register LTC PKHA A3 6 Register"]
#[repr(C)]
pub union LTC0_PKA3_6_UNION {
    #[doc = "0x8d8 - LTC PKHA A 54 Register"]
    pub ltc0_pka_54: LTC0_PKA_54,
    #[doc = "0x8d8 - LTC PKHA A3 6 Register"]
    pub ltc0_pka3_6: LTC0_PKA3_6,
}
#[doc = "LTC PKHA A 55 Register LTC PKHA A3 7 Register"]
#[repr(C)]
pub union LTC0_PKA3_7_UNION {
    #[doc = "0x8dc - LTC PKHA A 55 Register"]
    pub ltc0_pka_55: LTC0_PKA_55,
    #[doc = "0x8dc - LTC PKHA A3 7 Register"]
    pub ltc0_pka3_7: LTC0_PKA3_7,
}
#[doc = "LTC PKHA A 56 Register LTC PKHA A3 8 Register"]
#[repr(C)]
pub union LTC0_PKA3_8_UNION {
    #[doc = "0x8e0 - LTC PKHA A 56 Register"]
    pub ltc0_pka_56: LTC0_PKA_56,
    #[doc = "0x8e0 - LTC PKHA A3 8 Register"]
    pub ltc0_pka3_8: LTC0_PKA3_8,
}
#[doc = "LTC PKHA A 57 Register LTC PKHA A3 9 Register"]
#[repr(C)]
pub union LTC0_PKA3_9_UNION {
    #[doc = "0x8e4 - LTC PKHA A 57 Register"]
    pub ltc0_pka_57: LTC0_PKA_57,
    #[doc = "0x8e4 - LTC PKHA A3 9 Register"]
    pub ltc0_pka3_9: LTC0_PKA3_9,
}
#[doc = "LTC PKHA A 58 Register LTC PKHA A3 10 Register"]
#[repr(C)]
pub union LTC0_PKA_58_UNION {
    #[doc = "0x8e8 - LTC PKHA A 58 Register"]
    pub ltc0_pka_58: LTC0_PKA_58,
    #[doc = "0x8e8 - LTC PKHA A3 10 Register"]
    pub ltc0_pka3_10: LTC0_PKA3_10,
}
#[doc = "LTC PKHA A 59 Register LTC PKHA A3 11 Register"]
#[repr(C)]
pub union LTC0_PKA_59_UNION {
    #[doc = "0x8ec - LTC PKHA A 59 Register"]
    pub ltc0_pka_59: LTC0_PKA_59,
    #[doc = "0x8ec - LTC PKHA A3 11 Register"]
    pub ltc0_pka3_11: LTC0_PKA3_11,
}
#[doc = "LTC PKHA A 60 Register LTC PKHA A3 12 Register"]
#[repr(C)]
pub union LTC0_PKA_60_UNION {
    #[doc = "0x8f0 - LTC PKHA A 60 Register"]
    pub ltc0_pka_60: LTC0_PKA_60,
    #[doc = "0x8f0 - LTC PKHA A3 12 Register"]
    pub ltc0_pka3_12: LTC0_PKA3_12,
}
#[doc = "LTC PKHA A 61 Register LTC PKHA A3 13 Register"]
#[repr(C)]
pub union LTC0_PKA_61_UNION {
    #[doc = "0x8f4 - LTC PKHA A 61 Register"]
    pub ltc0_pka_61: LTC0_PKA_61,
    #[doc = "0x8f4 - LTC PKHA A3 13 Register"]
    pub ltc0_pka3_13: LTC0_PKA3_13,
}
#[doc = "LTC PKHA A 62 Register LTC PKHA A3 14 Register"]
#[repr(C)]
pub union LTC0_PKA_62_UNION {
    #[doc = "0x8f8 - LTC PKHA A 62 Register"]
    pub ltc0_pka_62: LTC0_PKA_62,
    #[doc = "0x8f8 - LTC PKHA A3 14 Register"]
    pub ltc0_pka3_14: LTC0_PKA3_14,
}
#[doc = "LTC PKHA A 63 Register LTC PKHA A3 15 Register"]
#[repr(C)]
pub union LTC0_PKA_63_UNION {
    #[doc = "0x8fc - LTC PKHA A 63 Register"]
    pub ltc0_pka_63: LTC0_PKA_63,
    #[doc = "0x8fc - LTC PKHA A3 15 Register"]
    pub ltc0_pka3_15: LTC0_PKA3_15,
}
#[doc = "LTC PKHA B 0 Register LTC PKHA B0 0 Register"]
#[repr(C)]
pub union LTC0_PKB_0_UNION {
    #[doc = "0xa00 - LTC PKHA B 0 Register"]
    pub ltc0_pkb_0: LTC0_PKB_0,
    #[doc = "0xa00 - LTC PKHA B0 0 Register"]
    pub ltc0_pkb0_0: LTC0_PKB0_0,
}
#[doc = "LTC PKHA B 1 Register LTC PKHA B0 1 Register"]
#[repr(C)]
pub union LTC0_PKB_1_UNION {
    #[doc = "0xa04 - LTC PKHA B 1 Register"]
    pub ltc0_pkb_1: LTC0_PKB_1,
    #[doc = "0xa04 - LTC PKHA B0 1 Register"]
    pub ltc0_pkb0_1: LTC0_PKB0_1,
}
#[doc = "LTC PKHA B 2 Register LTC PKHA B0 2 Register"]
#[repr(C)]
pub union LTC0_PKB_2_UNION {
    #[doc = "0xa08 - LTC PKHA B 2 Register"]
    pub ltc0_pkb_2: LTC0_PKB_2,
    #[doc = "0xa08 - LTC PKHA B0 2 Register"]
    pub ltc0_pkb0_2: LTC0_PKB0_2,
}
#[doc = "LTC PKHA B 3 Register LTC PKHA B0 3 Register"]
#[repr(C)]
pub union LTC0_PKB_3_UNION {
    #[doc = "0xa0c - LTC PKHA B 3 Register"]
    pub ltc0_pkb_3: LTC0_PKB_3,
    #[doc = "0xa0c - LTC PKHA B0 3 Register"]
    pub ltc0_pkb0_3: LTC0_PKB0_3,
}
#[doc = "LTC PKHA B 4 Register LTC PKHA B0 4 Register"]
#[repr(C)]
pub union LTC0_PKB_4_UNION {
    #[doc = "0xa10 - LTC PKHA B 4 Register"]
    pub ltc0_pkb_4: LTC0_PKB_4,
    #[doc = "0xa10 - LTC PKHA B0 4 Register"]
    pub ltc0_pkb0_4: LTC0_PKB0_4,
}
#[doc = "LTC PKHA B 5 Register LTC PKHA B0 5 Register"]
#[repr(C)]
pub union LTC0_PKB_5_UNION {
    #[doc = "0xa14 - LTC PKHA B 5 Register"]
    pub ltc0_pkb_5: LTC0_PKB_5,
    #[doc = "0xa14 - LTC PKHA B0 5 Register"]
    pub ltc0_pkb0_5: LTC0_PKB0_5,
}
#[doc = "LTC PKHA B 6 Register LTC PKHA B0 6 Register"]
#[repr(C)]
pub union LTC0_PKB_6_UNION {
    #[doc = "0xa18 - LTC PKHA B 6 Register"]
    pub ltc0_pkb_6: LTC0_PKB_6,
    #[doc = "0xa18 - LTC PKHA B0 6 Register"]
    pub ltc0_pkb0_6: LTC0_PKB0_6,
}
#[doc = "LTC PKHA B 7 Register LTC PKHA B0 7 Register"]
#[repr(C)]
pub union LTC0_PKB_7_UNION {
    #[doc = "0xa1c - LTC PKHA B 7 Register"]
    pub ltc0_pkb_7: LTC0_PKB_7,
    #[doc = "0xa1c - LTC PKHA B0 7 Register"]
    pub ltc0_pkb0_7: LTC0_PKB0_7,
}
#[doc = "LTC PKHA B 8 Register LTC PKHA B0 8 Register"]
#[repr(C)]
pub union LTC0_PKB_8_UNION {
    #[doc = "0xa20 - LTC PKHA B 8 Register"]
    pub ltc0_pkb_8: LTC0_PKB_8,
    #[doc = "0xa20 - LTC PKHA B0 8 Register"]
    pub ltc0_pkb0_8: LTC0_PKB0_8,
}
#[doc = "LTC PKHA B 9 Register LTC PKHA B0 9 Register"]
#[repr(C)]
pub union LTC0_PKB_9_UNION {
    #[doc = "0xa24 - LTC PKHA B 9 Register"]
    pub ltc0_pkb_9: LTC0_PKB_9,
    #[doc = "0xa24 - LTC PKHA B0 9 Register"]
    pub ltc0_pkb0_9: LTC0_PKB0_9,
}
#[doc = "LTC PKHA B 10 Register LTC PKHA B0 10 Register"]
#[repr(C)]
pub union LTC0_PKB_10_UNION {
    #[doc = "0xa28 - LTC PKHA B 10 Register"]
    pub ltc0_pkb_10: LTC0_PKB_10,
    #[doc = "0xa28 - LTC PKHA B0 10 Register"]
    pub ltc0_pkb0_10: LTC0_PKB0_10,
}
#[doc = "LTC PKHA B 11 Register LTC PKHA B0 11 Register"]
#[repr(C)]
pub union LTC0_PKB_11_UNION {
    #[doc = "0xa2c - LTC PKHA B 11 Register"]
    pub ltc0_pkb_11: LTC0_PKB_11,
    #[doc = "0xa2c - LTC PKHA B0 11 Register"]
    pub ltc0_pkb0_11: LTC0_PKB0_11,
}
#[doc = "LTC PKHA B 12 Register LTC PKHA B0 12 Register"]
#[repr(C)]
pub union LTC0_PKB_12_UNION {
    #[doc = "0xa30 - LTC PKHA B 12 Register"]
    pub ltc0_pkb_12: LTC0_PKB_12,
    #[doc = "0xa30 - LTC PKHA B0 12 Register"]
    pub ltc0_pkb0_12: LTC0_PKB0_12,
}
#[doc = "LTC PKHA B 13 Register LTC PKHA B0 13 Register"]
#[repr(C)]
pub union LTC0_PKB_13_UNION {
    #[doc = "0xa34 - LTC PKHA B 13 Register"]
    pub ltc0_pkb_13: LTC0_PKB_13,
    #[doc = "0xa34 - LTC PKHA B0 13 Register"]
    pub ltc0_pkb0_13: LTC0_PKB0_13,
}
#[doc = "LTC PKHA B 14 Register LTC PKHA B0 14 Register"]
#[repr(C)]
pub union LTC0_PKB_14_UNION {
    #[doc = "0xa38 - LTC PKHA B 14 Register"]
    pub ltc0_pkb_14: LTC0_PKB_14,
    #[doc = "0xa38 - LTC PKHA B0 14 Register"]
    pub ltc0_pkb0_14: LTC0_PKB0_14,
}
#[doc = "LTC PKHA B 15 Register LTC PKHA B0 15 Register"]
#[repr(C)]
pub union LTC0_PKB_15_UNION {
    #[doc = "0xa3c - LTC PKHA B 15 Register"]
    pub ltc0_pkb_15: LTC0_PKB_15,
    #[doc = "0xa3c - LTC PKHA B0 15 Register"]
    pub ltc0_pkb0_15: LTC0_PKB0_15,
}
#[doc = "LTC PKHA B 16 Register LTC PKHA B1 0 Register"]
#[repr(C)]
pub union LTC0_PKB1_0_UNION {
    #[doc = "0xa40 - LTC PKHA B 16 Register"]
    pub ltc0_pkb_16: LTC0_PKB_16,
    #[doc = "0xa40 - LTC PKHA B1 0 Register"]
    pub ltc0_pkb1_0: LTC0_PKB1_0,
}
#[doc = "LTC PKHA B 17 Register LTC PKHA B1 1 Register"]
#[repr(C)]
pub union LTC0_PKB1_1_UNION {
    #[doc = "0xa44 - LTC PKHA B 17 Register"]
    pub ltc0_pkb_17: LTC0_PKB_17,
    #[doc = "0xa44 - LTC PKHA B1 1 Register"]
    pub ltc0_pkb1_1: LTC0_PKB1_1,
}
#[doc = "LTC PKHA B 18 Register LTC PKHA B1 2 Register"]
#[repr(C)]
pub union LTC0_PKB1_2_UNION {
    #[doc = "0xa48 - LTC PKHA B 18 Register"]
    pub ltc0_pkb_18: LTC0_PKB_18,
    #[doc = "0xa48 - LTC PKHA B1 2 Register"]
    pub ltc0_pkb1_2: LTC0_PKB1_2,
}
#[doc = "LTC PKHA B 19 Register LTC PKHA B1 3 Register"]
#[repr(C)]
pub union LTC0_PKB1_3_UNION {
    #[doc = "0xa4c - LTC PKHA B 19 Register"]
    pub ltc0_pkb_19: LTC0_PKB_19,
    #[doc = "0xa4c - LTC PKHA B1 3 Register"]
    pub ltc0_pkb1_3: LTC0_PKB1_3,
}
#[doc = "LTC PKHA B 20 Register LTC PKHA B1 4 Register"]
#[repr(C)]
pub union LTC0_PKB1_4_UNION {
    #[doc = "0xa50 - LTC PKHA B 20 Register"]
    pub ltc0_pkb_20: LTC0_PKB_20,
    #[doc = "0xa50 - LTC PKHA B1 4 Register"]
    pub ltc0_pkb1_4: LTC0_PKB1_4,
}
#[doc = "LTC PKHA B 21 Register LTC PKHA B1 5 Register"]
#[repr(C)]
pub union LTC0_PKB1_5_UNION {
    #[doc = "0xa54 - LTC PKHA B 21 Register"]
    pub ltc0_pkb_21: LTC0_PKB_21,
    #[doc = "0xa54 - LTC PKHA B1 5 Register"]
    pub ltc0_pkb1_5: LTC0_PKB1_5,
}
#[doc = "LTC PKHA B 22 Register LTC PKHA B1 6 Register"]
#[repr(C)]
pub union LTC0_PKB1_6_UNION {
    #[doc = "0xa58 - LTC PKHA B 22 Register"]
    pub ltc0_pkb_22: LTC0_PKB_22,
    #[doc = "0xa58 - LTC PKHA B1 6 Register"]
    pub ltc0_pkb1_6: LTC0_PKB1_6,
}
#[doc = "LTC PKHA B 23 Register LTC PKHA B1 7 Register"]
#[repr(C)]
pub union LTC0_PKB1_7_UNION {
    #[doc = "0xa5c - LTC PKHA B 23 Register"]
    pub ltc0_pkb_23: LTC0_PKB_23,
    #[doc = "0xa5c - LTC PKHA B1 7 Register"]
    pub ltc0_pkb1_7: LTC0_PKB1_7,
}
#[doc = "LTC PKHA B 24 Register LTC PKHA B1 8 Register"]
#[repr(C)]
pub union LTC0_PKB1_8_UNION {
    #[doc = "0xa60 - LTC PKHA B 24 Register"]
    pub ltc0_pkb_24: LTC0_PKB_24,
    #[doc = "0xa60 - LTC PKHA B1 8 Register"]
    pub ltc0_pkb1_8: LTC0_PKB1_8,
}
#[doc = "LTC PKHA B 25 Register LTC PKHA B1 9 Register"]
#[repr(C)]
pub union LTC0_PKB1_9_UNION {
    #[doc = "0xa64 - LTC PKHA B 25 Register"]
    pub ltc0_pkb_25: LTC0_PKB_25,
    #[doc = "0xa64 - LTC PKHA B1 9 Register"]
    pub ltc0_pkb1_9: LTC0_PKB1_9,
}
#[doc = "LTC PKHA B 26 Register LTC PKHA B1 10 Register"]
#[repr(C)]
pub union LTC0_PKB_26_UNION {
    #[doc = "0xa68 - LTC PKHA B 26 Register"]
    pub ltc0_pkb_26: LTC0_PKB_26,
    #[doc = "0xa68 - LTC PKHA B1 10 Register"]
    pub ltc0_pkb1_10: LTC0_PKB1_10,
}
#[doc = "LTC PKHA B 27 Register LTC PKHA B1 11 Register"]
#[repr(C)]
pub union LTC0_PKB_27_UNION {
    #[doc = "0xa6c - LTC PKHA B 27 Register"]
    pub ltc0_pkb_27: LTC0_PKB_27,
    #[doc = "0xa6c - LTC PKHA B1 11 Register"]
    pub ltc0_pkb1_11: LTC0_PKB1_11,
}
#[doc = "LTC PKHA B 28 Register LTC PKHA B1 12 Register"]
#[repr(C)]
pub union LTC0_PKB_28_UNION {
    #[doc = "0xa70 - LTC PKHA B 28 Register"]
    pub ltc0_pkb_28: LTC0_PKB_28,
    #[doc = "0xa70 - LTC PKHA B1 12 Register"]
    pub ltc0_pkb1_12: LTC0_PKB1_12,
}
#[doc = "LTC PKHA B 29 Register LTC PKHA B1 13 Register"]
#[repr(C)]
pub union LTC0_PKB_29_UNION {
    #[doc = "0xa74 - LTC PKHA B 29 Register"]
    pub ltc0_pkb_29: LTC0_PKB_29,
    #[doc = "0xa74 - LTC PKHA B1 13 Register"]
    pub ltc0_pkb1_13: LTC0_PKB1_13,
}
#[doc = "LTC PKHA B 30 Register LTC PKHA B1 14 Register"]
#[repr(C)]
pub union LTC0_PKB_30_UNION {
    #[doc = "0xa78 - LTC PKHA B 30 Register"]
    pub ltc0_pkb_30: LTC0_PKB_30,
    #[doc = "0xa78 - LTC PKHA B1 14 Register"]
    pub ltc0_pkb1_14: LTC0_PKB1_14,
}
#[doc = "LTC PKHA B 31 Register LTC PKHA B1 15 Register"]
#[repr(C)]
pub union LTC0_PKB_31_UNION {
    #[doc = "0xa7c - LTC PKHA B 31 Register"]
    pub ltc0_pkb_31: LTC0_PKB_31,
    #[doc = "0xa7c - LTC PKHA B1 15 Register"]
    pub ltc0_pkb1_15: LTC0_PKB1_15,
}
#[doc = "LTC PKHA B 32 Register LTC PKHA B2 0 Register"]
#[repr(C)]
pub union LTC0_PKB2_0_UNION {
    #[doc = "0xa80 - LTC PKHA B 32 Register"]
    pub ltc0_pkb_32: LTC0_PKB_32,
    #[doc = "0xa80 - LTC PKHA B2 0 Register"]
    pub ltc0_pkb2_0: LTC0_PKB2_0,
}
#[doc = "LTC PKHA B 33 Register LTC PKHA B2 1 Register"]
#[repr(C)]
pub union LTC0_PKB2_1_UNION {
    #[doc = "0xa84 - LTC PKHA B 33 Register"]
    pub ltc0_pkb_33: LTC0_PKB_33,
    #[doc = "0xa84 - LTC PKHA B2 1 Register"]
    pub ltc0_pkb2_1: LTC0_PKB2_1,
}
#[doc = "LTC PKHA B 34 Register LTC PKHA B2 2 Register"]
#[repr(C)]
pub union LTC0_PKB2_2_UNION {
    #[doc = "0xa88 - LTC PKHA B 34 Register"]
    pub ltc0_pkb_34: LTC0_PKB_34,
    #[doc = "0xa88 - LTC PKHA B2 2 Register"]
    pub ltc0_pkb2_2: LTC0_PKB2_2,
}
#[doc = "LTC PKHA B 35 Register LTC PKHA B2 3 Register"]
#[repr(C)]
pub union LTC0_PKB2_3_UNION {
    #[doc = "0xa8c - LTC PKHA B 35 Register"]
    pub ltc0_pkb_35: LTC0_PKB_35,
    #[doc = "0xa8c - LTC PKHA B2 3 Register"]
    pub ltc0_pkb2_3: LTC0_PKB2_3,
}
#[doc = "LTC PKHA B 36 Register LTC PKHA B2 4 Register"]
#[repr(C)]
pub union LTC0_PKB2_4_UNION {
    #[doc = "0xa90 - LTC PKHA B 36 Register"]
    pub ltc0_pkb_36: LTC0_PKB_36,
    #[doc = "0xa90 - LTC PKHA B2 4 Register"]
    pub ltc0_pkb2_4: LTC0_PKB2_4,
}
#[doc = "LTC PKHA B 37 Register LTC PKHA B2 5 Register"]
#[repr(C)]
pub union LTC0_PKB2_5_UNION {
    #[doc = "0xa94 - LTC PKHA B 37 Register"]
    pub ltc0_pkb_37: LTC0_PKB_37,
    #[doc = "0xa94 - LTC PKHA B2 5 Register"]
    pub ltc0_pkb2_5: LTC0_PKB2_5,
}
#[doc = "LTC PKHA B 38 Register LTC PKHA B2 6 Register"]
#[repr(C)]
pub union LTC0_PKB2_6_UNION {
    #[doc = "0xa98 - LTC PKHA B 38 Register"]
    pub ltc0_pkb_38: LTC0_PKB_38,
    #[doc = "0xa98 - LTC PKHA B2 6 Register"]
    pub ltc0_pkb2_6: LTC0_PKB2_6,
}
#[doc = "LTC PKHA B 39 Register LTC PKHA B2 7 Register"]
#[repr(C)]
pub union LTC0_PKB2_7_UNION {
    #[doc = "0xa9c - LTC PKHA B 39 Register"]
    pub ltc0_pkb_39: LTC0_PKB_39,
    #[doc = "0xa9c - LTC PKHA B2 7 Register"]
    pub ltc0_pkb2_7: LTC0_PKB2_7,
}
#[doc = "LTC PKHA B 40 Register LTC PKHA B2 8 Register"]
#[repr(C)]
pub union LTC0_PKB2_8_UNION {
    #[doc = "0xaa0 - LTC PKHA B 40 Register"]
    pub ltc0_pkb_40: LTC0_PKB_40,
    #[doc = "0xaa0 - LTC PKHA B2 8 Register"]
    pub ltc0_pkb2_8: LTC0_PKB2_8,
}
#[doc = "LTC PKHA B 41 Register LTC PKHA B2 9 Register"]
#[repr(C)]
pub union LTC0_PKB2_9_UNION {
    #[doc = "0xaa4 - LTC PKHA B 41 Register"]
    pub ltc0_pkb_41: LTC0_PKB_41,
    #[doc = "0xaa4 - LTC PKHA B2 9 Register"]
    pub ltc0_pkb2_9: LTC0_PKB2_9,
}
#[doc = "LTC PKHA B 42 Register LTC PKHA B2 10 Register"]
#[repr(C)]
pub union LTC0_PKB_42_UNION {
    #[doc = "0xaa8 - LTC PKHA B 42 Register"]
    pub ltc0_pkb_42: LTC0_PKB_42,
    #[doc = "0xaa8 - LTC PKHA B2 10 Register"]
    pub ltc0_pkb2_10: LTC0_PKB2_10,
}
#[doc = "LTC PKHA B 43 Register LTC PKHA B2 11 Register"]
#[repr(C)]
pub union LTC0_PKB_43_UNION {
    #[doc = "0xaac - LTC PKHA B 43 Register"]
    pub ltc0_pkb_43: LTC0_PKB_43,
    #[doc = "0xaac - LTC PKHA B2 11 Register"]
    pub ltc0_pkb2_11: LTC0_PKB2_11,
}
#[doc = "LTC PKHA B 44 Register LTC PKHA B2 12 Register"]
#[repr(C)]
pub union LTC0_PKB_44_UNION {
    #[doc = "0xab0 - LTC PKHA B 44 Register"]
    pub ltc0_pkb_44: LTC0_PKB_44,
    #[doc = "0xab0 - LTC PKHA B2 12 Register"]
    pub ltc0_pkb2_12: LTC0_PKB2_12,
}
#[doc = "LTC PKHA B 45 Register LTC PKHA B2 13 Register"]
#[repr(C)]
pub union LTC0_PKB_45_UNION {
    #[doc = "0xab4 - LTC PKHA B 45 Register"]
    pub ltc0_pkb_45: LTC0_PKB_45,
    #[doc = "0xab4 - LTC PKHA B2 13 Register"]
    pub ltc0_pkb2_13: LTC0_PKB2_13,
}
#[doc = "LTC PKHA B 46 Register LTC PKHA B2 14 Register"]
#[repr(C)]
pub union LTC0_PKB_46_UNION {
    #[doc = "0xab8 - LTC PKHA B 46 Register"]
    pub ltc0_pkb_46: LTC0_PKB_46,
    #[doc = "0xab8 - LTC PKHA B2 14 Register"]
    pub ltc0_pkb2_14: LTC0_PKB2_14,
}
#[doc = "LTC PKHA B 47 Register LTC PKHA B2 15 Register"]
#[repr(C)]
pub union LTC0_PKB_47_UNION {
    #[doc = "0xabc - LTC PKHA B 47 Register"]
    pub ltc0_pkb_47: LTC0_PKB_47,
    #[doc = "0xabc - LTC PKHA B2 15 Register"]
    pub ltc0_pkb2_15: LTC0_PKB2_15,
}
#[doc = "LTC PKHA B 48 Register LTC PKHA B3 0 Register"]
#[repr(C)]
pub union LTC0_PKB3_0_UNION {
    #[doc = "0xac0 - LTC PKHA B 48 Register"]
    pub ltc0_pkb_48: LTC0_PKB_48,
    #[doc = "0xac0 - LTC PKHA B3 0 Register"]
    pub ltc0_pkb3_0: LTC0_PKB3_0,
}
#[doc = "LTC PKHA B 49 Register LTC PKHA B3 1 Register"]
#[repr(C)]
pub union LTC0_PKB3_1_UNION {
    #[doc = "0xac4 - LTC PKHA B 49 Register"]
    pub ltc0_pkb_49: LTC0_PKB_49,
    #[doc = "0xac4 - LTC PKHA B3 1 Register"]
    pub ltc0_pkb3_1: LTC0_PKB3_1,
}
#[doc = "LTC PKHA B 50 Register LTC PKHA B3 2 Register"]
#[repr(C)]
pub union LTC0_PKB3_2_UNION {
    #[doc = "0xac8 - LTC PKHA B 50 Register"]
    pub ltc0_pkb_50: LTC0_PKB_50,
    #[doc = "0xac8 - LTC PKHA B3 2 Register"]
    pub ltc0_pkb3_2: LTC0_PKB3_2,
}
#[doc = "LTC PKHA B 51 Register LTC PKHA B3 3 Register"]
#[repr(C)]
pub union LTC0_PKB3_3_UNION {
    #[doc = "0xacc - LTC PKHA B 51 Register"]
    pub ltc0_pkb_51: LTC0_PKB_51,
    #[doc = "0xacc - LTC PKHA B3 3 Register"]
    pub ltc0_pkb3_3: LTC0_PKB3_3,
}
#[doc = "LTC PKHA B 52 Register LTC PKHA B3 4 Register"]
#[repr(C)]
pub union LTC0_PKB3_4_UNION {
    #[doc = "0xad0 - LTC PKHA B 52 Register"]
    pub ltc0_pkb_52: LTC0_PKB_52,
    #[doc = "0xad0 - LTC PKHA B3 4 Register"]
    pub ltc0_pkb3_4: LTC0_PKB3_4,
}
#[doc = "LTC PKHA B 53 Register LTC PKHA B3 5 Register"]
#[repr(C)]
pub union LTC0_PKB3_5_UNION {
    #[doc = "0xad4 - LTC PKHA B 53 Register"]
    pub ltc0_pkb_53: LTC0_PKB_53,
    #[doc = "0xad4 - LTC PKHA B3 5 Register"]
    pub ltc0_pkb3_5: LTC0_PKB3_5,
}
#[doc = "LTC PKHA B 54 Register LTC PKHA B3 6 Register"]
#[repr(C)]
pub union LTC0_PKB3_6_UNION {
    #[doc = "0xad8 - LTC PKHA B 54 Register"]
    pub ltc0_pkb_54: LTC0_PKB_54,
    #[doc = "0xad8 - LTC PKHA B3 6 Register"]
    pub ltc0_pkb3_6: LTC0_PKB3_6,
}
#[doc = "LTC PKHA B 55 Register LTC PKHA B3 7 Register"]
#[repr(C)]
pub union LTC0_PKB3_7_UNION {
    #[doc = "0xadc - LTC PKHA B 55 Register"]
    pub ltc0_pkb_55: LTC0_PKB_55,
    #[doc = "0xadc - LTC PKHA B3 7 Register"]
    pub ltc0_pkb3_7: LTC0_PKB3_7,
}
#[doc = "LTC PKHA B 56 Register LTC PKHA B3 8 Register"]
#[repr(C)]
pub union LTC0_PKB3_8_UNION {
    #[doc = "0xae0 - LTC PKHA B 56 Register"]
    pub ltc0_pkb_56: LTC0_PKB_56,
    #[doc = "0xae0 - LTC PKHA B3 8 Register"]
    pub ltc0_pkb3_8: LTC0_PKB3_8,
}
#[doc = "LTC PKHA B 57 Register LTC PKHA B3 9 Register"]
#[repr(C)]
pub union LTC0_PKB3_9_UNION {
    #[doc = "0xae4 - LTC PKHA B 57 Register"]
    pub ltc0_pkb_57: LTC0_PKB_57,
    #[doc = "0xae4 - LTC PKHA B3 9 Register"]
    pub ltc0_pkb3_9: LTC0_PKB3_9,
}
#[doc = "LTC PKHA B 58 Register LTC PKHA B3 10 Register"]
#[repr(C)]
pub union LTC0_PKB_58_UNION {
    #[doc = "0xae8 - LTC PKHA B 58 Register"]
    pub ltc0_pkb_58: LTC0_PKB_58,
    #[doc = "0xae8 - LTC PKHA B3 10 Register"]
    pub ltc0_pkb3_10: LTC0_PKB3_10,
}
#[doc = "LTC PKHA B 59 Register LTC PKHA B3 11 Register"]
#[repr(C)]
pub union LTC0_PKB_59_UNION {
    #[doc = "0xaec - LTC PKHA B 59 Register"]
    pub ltc0_pkb_59: LTC0_PKB_59,
    #[doc = "0xaec - LTC PKHA B3 11 Register"]
    pub ltc0_pkb3_11: LTC0_PKB3_11,
}
#[doc = "LTC PKHA B 60 Register LTC PKHA B3 12 Register"]
#[repr(C)]
pub union LTC0_PKB_60_UNION {
    #[doc = "0xaf0 - LTC PKHA B 60 Register"]
    pub ltc0_pkb_60: LTC0_PKB_60,
    #[doc = "0xaf0 - LTC PKHA B3 12 Register"]
    pub ltc0_pkb3_12: LTC0_PKB3_12,
}
#[doc = "LTC PKHA B 61 Register LTC PKHA B3 13 Register"]
#[repr(C)]
pub union LTC0_PKB_61_UNION {
    #[doc = "0xaf4 - LTC PKHA B 61 Register"]
    pub ltc0_pkb_61: LTC0_PKB_61,
    #[doc = "0xaf4 - LTC PKHA B3 13 Register"]
    pub ltc0_pkb3_13: LTC0_PKB3_13,
}
#[doc = "LTC PKHA B 62 Register LTC PKHA B3 14 Register"]
#[repr(C)]
pub union LTC0_PKB_62_UNION {
    #[doc = "0xaf8 - LTC PKHA B 62 Register"]
    pub ltc0_pkb_62: LTC0_PKB_62,
    #[doc = "0xaf8 - LTC PKHA B3 14 Register"]
    pub ltc0_pkb3_14: LTC0_PKB3_14,
}
#[doc = "LTC PKHA B 63 Register LTC PKHA B3 15 Register"]
#[repr(C)]
pub union LTC0_PKB_63_UNION {
    #[doc = "0xafc - LTC PKHA B 63 Register"]
    pub ltc0_pkb_63: LTC0_PKB_63,
    #[doc = "0xafc - LTC PKHA B3 15 Register"]
    pub ltc0_pkb3_15: LTC0_PKB3_15,
}
#[doc = "LTC PKHA N 0 Register LTC PKHA N0 0 Register"]
#[repr(C)]
pub union LTC0_PKN_0_UNION {
    #[doc = "0xc00 - LTC PKHA N 0 Register"]
    pub ltc0_pkn_0: LTC0_PKN_0,
    #[doc = "0xc00 - LTC PKHA N0 0 Register"]
    pub ltc0_pkn0_0: LTC0_PKN0_0,
}
#[doc = "LTC PKHA N 1 Register LTC PKHA N0 1 Register"]
#[repr(C)]
pub union LTC0_PKN_1_UNION {
    #[doc = "0xc04 - LTC PKHA N 1 Register"]
    pub ltc0_pkn_1: LTC0_PKN_1,
    #[doc = "0xc04 - LTC PKHA N0 1 Register"]
    pub ltc0_pkn0_1: LTC0_PKN0_1,
}
#[doc = "LTC PKHA N 2 Register LTC PKHA N0 2 Register"]
#[repr(C)]
pub union LTC0_PKN_2_UNION {
    #[doc = "0xc08 - LTC PKHA N 2 Register"]
    pub ltc0_pkn_2: LTC0_PKN_2,
    #[doc = "0xc08 - LTC PKHA N0 2 Register"]
    pub ltc0_pkn0_2: LTC0_PKN0_2,
}
#[doc = "LTC PKHA N 3 Register LTC PKHA N0 3 Register"]
#[repr(C)]
pub union LTC0_PKN_3_UNION {
    #[doc = "0xc0c - LTC PKHA N 3 Register"]
    pub ltc0_pkn_3: LTC0_PKN_3,
    #[doc = "0xc0c - LTC PKHA N0 3 Register"]
    pub ltc0_pkn0_3: LTC0_PKN0_3,
}
#[doc = "LTC PKHA N 4 Register LTC PKHA N0 4 Register"]
#[repr(C)]
pub union LTC0_PKN_4_UNION {
    #[doc = "0xc10 - LTC PKHA N 4 Register"]
    pub ltc0_pkn_4: LTC0_PKN_4,
    #[doc = "0xc10 - LTC PKHA N0 4 Register"]
    pub ltc0_pkn0_4: LTC0_PKN0_4,
}
#[doc = "LTC PKHA N 5 Register LTC PKHA N0 5 Register"]
#[repr(C)]
pub union LTC0_PKN_5_UNION {
    #[doc = "0xc14 - LTC PKHA N 5 Register"]
    pub ltc0_pkn_5: LTC0_PKN_5,
    #[doc = "0xc14 - LTC PKHA N0 5 Register"]
    pub ltc0_pkn0_5: LTC0_PKN0_5,
}
#[doc = "LTC PKHA N 6 Register LTC PKHA N0 6 Register"]
#[repr(C)]
pub union LTC0_PKN_6_UNION {
    #[doc = "0xc18 - LTC PKHA N 6 Register"]
    pub ltc0_pkn_6: LTC0_PKN_6,
    #[doc = "0xc18 - LTC PKHA N0 6 Register"]
    pub ltc0_pkn0_6: LTC0_PKN0_6,
}
#[doc = "LTC PKHA N 7 Register LTC PKHA N0 7 Register"]
#[repr(C)]
pub union LTC0_PKN_7_UNION {
    #[doc = "0xc1c - LTC PKHA N 7 Register"]
    pub ltc0_pkn_7: LTC0_PKN_7,
    #[doc = "0xc1c - LTC PKHA N0 7 Register"]
    pub ltc0_pkn0_7: LTC0_PKN0_7,
}
#[doc = "LTC PKHA N 8 Register LTC PKHA N0 8 Register"]
#[repr(C)]
pub union LTC0_PKN_8_UNION {
    #[doc = "0xc20 - LTC PKHA N 8 Register"]
    pub ltc0_pkn_8: LTC0_PKN_8,
    #[doc = "0xc20 - LTC PKHA N0 8 Register"]
    pub ltc0_pkn0_8: LTC0_PKN0_8,
}
#[doc = "LTC PKHA N 9 Register LTC PKHA N0 9 Register"]
#[repr(C)]
pub union LTC0_PKN_9_UNION {
    #[doc = "0xc24 - LTC PKHA N 9 Register"]
    pub ltc0_pkn_9: LTC0_PKN_9,
    #[doc = "0xc24 - LTC PKHA N0 9 Register"]
    pub ltc0_pkn0_9: LTC0_PKN0_9,
}
#[doc = "LTC PKHA N 10 Register LTC PKHA N0 10 Register"]
#[repr(C)]
pub union LTC0_PKN_10_UNION {
    #[doc = "0xc28 - LTC PKHA N 10 Register"]
    pub ltc0_pkn_10: LTC0_PKN_10,
    #[doc = "0xc28 - LTC PKHA N0 10 Register"]
    pub ltc0_pkn0_10: LTC0_PKN0_10,
}
#[doc = "LTC PKHA N 11 Register LTC PKHA N0 11 Register"]
#[repr(C)]
pub union LTC0_PKN_11_UNION {
    #[doc = "0xc2c - LTC PKHA N 11 Register"]
    pub ltc0_pkn_11: LTC0_PKN_11,
    #[doc = "0xc2c - LTC PKHA N0 11 Register"]
    pub ltc0_pkn0_11: LTC0_PKN0_11,
}
#[doc = "LTC PKHA N 12 Register LTC PKHA N0 12 Register"]
#[repr(C)]
pub union LTC0_PKN_12_UNION {
    #[doc = "0xc30 - LTC PKHA N 12 Register"]
    pub ltc0_pkn_12: LTC0_PKN_12,
    #[doc = "0xc30 - LTC PKHA N0 12 Register"]
    pub ltc0_pkn0_12: LTC0_PKN0_12,
}
#[doc = "LTC PKHA N 13 Register LTC PKHA N0 13 Register"]
#[repr(C)]
pub union LTC0_PKN_13_UNION {
    #[doc = "0xc34 - LTC PKHA N 13 Register"]
    pub ltc0_pkn_13: LTC0_PKN_13,
    #[doc = "0xc34 - LTC PKHA N0 13 Register"]
    pub ltc0_pkn0_13: LTC0_PKN0_13,
}
#[doc = "LTC PKHA N 14 Register LTC PKHA N0 14 Register"]
#[repr(C)]
pub union LTC0_PKN_14_UNION {
    #[doc = "0xc38 - LTC PKHA N 14 Register"]
    pub ltc0_pkn_14: LTC0_PKN_14,
    #[doc = "0xc38 - LTC PKHA N0 14 Register"]
    pub ltc0_pkn0_14: LTC0_PKN0_14,
}
#[doc = "LTC PKHA N 15 Register LTC PKHA N0 15 Register"]
#[repr(C)]
pub union LTC0_PKN_15_UNION {
    #[doc = "0xc3c - LTC PKHA N 15 Register"]
    pub ltc0_pkn_15: LTC0_PKN_15,
    #[doc = "0xc3c - LTC PKHA N0 15 Register"]
    pub ltc0_pkn0_15: LTC0_PKN0_15,
}
#[doc = "LTC PKHA N 16 Register LTC PKHA N1 0 Register"]
#[repr(C)]
pub union LTC0_PKN1_0_UNION {
    #[doc = "0xc40 - LTC PKHA N 16 Register"]
    pub ltc0_pkn_16: LTC0_PKN_16,
    #[doc = "0xc40 - LTC PKHA N1 0 Register"]
    pub ltc0_pkn1_0: LTC0_PKN1_0,
}
#[doc = "LTC PKHA N 17 Register LTC PKHA N1 1 Register"]
#[repr(C)]
pub union LTC0_PKN1_1_UNION {
    #[doc = "0xc44 - LTC PKHA N 17 Register"]
    pub ltc0_pkn_17: LTC0_PKN_17,
    #[doc = "0xc44 - LTC PKHA N1 1 Register"]
    pub ltc0_pkn1_1: LTC0_PKN1_1,
}
#[doc = "LTC PKHA N 18 Register LTC PKHA N1 2 Register"]
#[repr(C)]
pub union LTC0_PKN1_2_UNION {
    #[doc = "0xc48 - LTC PKHA N 18 Register"]
    pub ltc0_pkn_18: LTC0_PKN_18,
    #[doc = "0xc48 - LTC PKHA N1 2 Register"]
    pub ltc0_pkn1_2: LTC0_PKN1_2,
}
#[doc = "LTC PKHA N 19 Register LTC PKHA N1 3 Register"]
#[repr(C)]
pub union LTC0_PKN1_3_UNION {
    #[doc = "0xc4c - LTC PKHA N 19 Register"]
    pub ltc0_pkn_19: LTC0_PKN_19,
    #[doc = "0xc4c - LTC PKHA N1 3 Register"]
    pub ltc0_pkn1_3: LTC0_PKN1_3,
}
#[doc = "LTC PKHA N 20 Register LTC PKHA N1 4 Register"]
#[repr(C)]
pub union LTC0_PKN1_4_UNION {
    #[doc = "0xc50 - LTC PKHA N 20 Register"]
    pub ltc0_pkn_20: LTC0_PKN_20,
    #[doc = "0xc50 - LTC PKHA N1 4 Register"]
    pub ltc0_pkn1_4: LTC0_PKN1_4,
}
#[doc = "LTC PKHA N 21 Register LTC PKHA N1 5 Register"]
#[repr(C)]
pub union LTC0_PKN1_5_UNION {
    #[doc = "0xc54 - LTC PKHA N 21 Register"]
    pub ltc0_pkn_21: LTC0_PKN_21,
    #[doc = "0xc54 - LTC PKHA N1 5 Register"]
    pub ltc0_pkn1_5: LTC0_PKN1_5,
}
#[doc = "LTC PKHA N 22 Register LTC PKHA N1 6 Register"]
#[repr(C)]
pub union LTC0_PKN1_6_UNION {
    #[doc = "0xc58 - LTC PKHA N 22 Register"]
    pub ltc0_pkn_22: LTC0_PKN_22,
    #[doc = "0xc58 - LTC PKHA N1 6 Register"]
    pub ltc0_pkn1_6: LTC0_PKN1_6,
}
#[doc = "LTC PKHA N 23 Register LTC PKHA N1 7 Register"]
#[repr(C)]
pub union LTC0_PKN1_7_UNION {
    #[doc = "0xc5c - LTC PKHA N 23 Register"]
    pub ltc0_pkn_23: LTC0_PKN_23,
    #[doc = "0xc5c - LTC PKHA N1 7 Register"]
    pub ltc0_pkn1_7: LTC0_PKN1_7,
}
#[doc = "LTC PKHA N 24 Register LTC PKHA N1 8 Register"]
#[repr(C)]
pub union LTC0_PKN1_8_UNION {
    #[doc = "0xc60 - LTC PKHA N 24 Register"]
    pub ltc0_pkn_24: LTC0_PKN_24,
    #[doc = "0xc60 - LTC PKHA N1 8 Register"]
    pub ltc0_pkn1_8: LTC0_PKN1_8,
}
#[doc = "LTC PKHA N 25 Register LTC PKHA N1 9 Register"]
#[repr(C)]
pub union LTC0_PKN1_9_UNION {
    #[doc = "0xc64 - LTC PKHA N 25 Register"]
    pub ltc0_pkn_25: LTC0_PKN_25,
    #[doc = "0xc64 - LTC PKHA N1 9 Register"]
    pub ltc0_pkn1_9: LTC0_PKN1_9,
}
#[doc = "LTC PKHA N 26 Register LTC PKHA N1 10 Register"]
#[repr(C)]
pub union LTC0_PKN_26_UNION {
    #[doc = "0xc68 - LTC PKHA N 26 Register"]
    pub ltc0_pkn_26: LTC0_PKN_26,
    #[doc = "0xc68 - LTC PKHA N1 10 Register"]
    pub ltc0_pkn1_10: LTC0_PKN1_10,
}
#[doc = "LTC PKHA N 27 Register LTC PKHA N1 11 Register"]
#[repr(C)]
pub union LTC0_PKN_27_UNION {
    #[doc = "0xc6c - LTC PKHA N 27 Register"]
    pub ltc0_pkn_27: LTC0_PKN_27,
    #[doc = "0xc6c - LTC PKHA N1 11 Register"]
    pub ltc0_pkn1_11: LTC0_PKN1_11,
}
#[doc = "LTC PKHA N 28 Register LTC PKHA N1 12 Register"]
#[repr(C)]
pub union LTC0_PKN_28_UNION {
    #[doc = "0xc70 - LTC PKHA N 28 Register"]
    pub ltc0_pkn_28: LTC0_PKN_28,
    #[doc = "0xc70 - LTC PKHA N1 12 Register"]
    pub ltc0_pkn1_12: LTC0_PKN1_12,
}
#[doc = "LTC PKHA N 29 Register LTC PKHA N1 13 Register"]
#[repr(C)]
pub union LTC0_PKN_29_UNION {
    #[doc = "0xc74 - LTC PKHA N 29 Register"]
    pub ltc0_pkn_29: LTC0_PKN_29,
    #[doc = "0xc74 - LTC PKHA N1 13 Register"]
    pub ltc0_pkn1_13: LTC0_PKN1_13,
}
#[doc = "LTC PKHA N 30 Register LTC PKHA N1 14 Register"]
#[repr(C)]
pub union LTC0_PKN_30_UNION {
    #[doc = "0xc78 - LTC PKHA N 30 Register"]
    pub ltc0_pkn_30: LTC0_PKN_30,
    #[doc = "0xc78 - LTC PKHA N1 14 Register"]
    pub ltc0_pkn1_14: LTC0_PKN1_14,
}
#[doc = "LTC PKHA N 31 Register LTC PKHA N1 15 Register"]
#[repr(C)]
pub union LTC0_PKN_31_UNION {
    #[doc = "0xc7c - LTC PKHA N 31 Register"]
    pub ltc0_pkn_31: LTC0_PKN_31,
    #[doc = "0xc7c - LTC PKHA N1 15 Register"]
    pub ltc0_pkn1_15: LTC0_PKN1_15,
}
#[doc = "LTC PKHA N 32 Register LTC PKHA N2 0 Register"]
#[repr(C)]
pub union LTC0_PKN2_0_UNION {
    #[doc = "0xc80 - LTC PKHA N 32 Register"]
    pub ltc0_pkn_32: LTC0_PKN_32,
    #[doc = "0xc80 - LTC PKHA N2 0 Register"]
    pub ltc0_pkn2_0: LTC0_PKN2_0,
}
#[doc = "LTC PKHA N 33 Register LTC PKHA N2 1 Register"]
#[repr(C)]
pub union LTC0_PKN2_1_UNION {
    #[doc = "0xc84 - LTC PKHA N 33 Register"]
    pub ltc0_pkn_33: LTC0_PKN_33,
    #[doc = "0xc84 - LTC PKHA N2 1 Register"]
    pub ltc0_pkn2_1: LTC0_PKN2_1,
}
#[doc = "LTC PKHA N 34 Register LTC PKHA N2 2 Register"]
#[repr(C)]
pub union LTC0_PKN2_2_UNION {
    #[doc = "0xc88 - LTC PKHA N 34 Register"]
    pub ltc0_pkn_34: LTC0_PKN_34,
    #[doc = "0xc88 - LTC PKHA N2 2 Register"]
    pub ltc0_pkn2_2: LTC0_PKN2_2,
}
#[doc = "LTC PKHA N 35 Register LTC PKHA N2 3 Register"]
#[repr(C)]
pub union LTC0_PKN2_3_UNION {
    #[doc = "0xc8c - LTC PKHA N 35 Register"]
    pub ltc0_pkn_35: LTC0_PKN_35,
    #[doc = "0xc8c - LTC PKHA N2 3 Register"]
    pub ltc0_pkn2_3: LTC0_PKN2_3,
}
#[doc = "LTC PKHA N 36 Register LTC PKHA N2 4 Register"]
#[repr(C)]
pub union LTC0_PKN2_4_UNION {
    #[doc = "0xc90 - LTC PKHA N 36 Register"]
    pub ltc0_pkn_36: LTC0_PKN_36,
    #[doc = "0xc90 - LTC PKHA N2 4 Register"]
    pub ltc0_pkn2_4: LTC0_PKN2_4,
}
#[doc = "LTC PKHA N 37 Register LTC PKHA N2 5 Register"]
#[repr(C)]
pub union LTC0_PKN2_5_UNION {
    #[doc = "0xc94 - LTC PKHA N 37 Register"]
    pub ltc0_pkn_37: LTC0_PKN_37,
    #[doc = "0xc94 - LTC PKHA N2 5 Register"]
    pub ltc0_pkn2_5: LTC0_PKN2_5,
}
#[doc = "LTC PKHA N 38 Register LTC PKHA N2 6 Register"]
#[repr(C)]
pub union LTC0_PKN2_6_UNION {
    #[doc = "0xc98 - LTC PKHA N 38 Register"]
    pub ltc0_pkn_38: LTC0_PKN_38,
    #[doc = "0xc98 - LTC PKHA N2 6 Register"]
    pub ltc0_pkn2_6: LTC0_PKN2_6,
}
#[doc = "LTC PKHA N 39 Register LTC PKHA N2 7 Register"]
#[repr(C)]
pub union LTC0_PKN2_7_UNION {
    #[doc = "0xc9c - LTC PKHA N 39 Register"]
    pub ltc0_pkn_39: LTC0_PKN_39,
    #[doc = "0xc9c - LTC PKHA N2 7 Register"]
    pub ltc0_pkn2_7: LTC0_PKN2_7,
}
#[doc = "LTC PKHA N 40 Register LTC PKHA N2 8 Register"]
#[repr(C)]
pub union LTC0_PKN2_8_UNION {
    #[doc = "0xca0 - LTC PKHA N 40 Register"]
    pub ltc0_pkn_40: LTC0_PKN_40,
    #[doc = "0xca0 - LTC PKHA N2 8 Register"]
    pub ltc0_pkn2_8: LTC0_PKN2_8,
}
#[doc = "LTC PKHA N 41 Register LTC PKHA N2 9 Register"]
#[repr(C)]
pub union LTC0_PKN2_9_UNION {
    #[doc = "0xca4 - LTC PKHA N 41 Register"]
    pub ltc0_pkn_41: LTC0_PKN_41,
    #[doc = "0xca4 - LTC PKHA N2 9 Register"]
    pub ltc0_pkn2_9: LTC0_PKN2_9,
}
#[doc = "LTC PKHA N 42 Register LTC PKHA N2 10 Register"]
#[repr(C)]
pub union LTC0_PKN_42_UNION {
    #[doc = "0xca8 - LTC PKHA N 42 Register"]
    pub ltc0_pkn_42: LTC0_PKN_42,
    #[doc = "0xca8 - LTC PKHA N2 10 Register"]
    pub ltc0_pkn2_10: LTC0_PKN2_10,
}
#[doc = "LTC PKHA N 43 Register LTC PKHA N2 11 Register"]
#[repr(C)]
pub union LTC0_PKN_43_UNION {
    #[doc = "0xcac - LTC PKHA N 43 Register"]
    pub ltc0_pkn_43: LTC0_PKN_43,
    #[doc = "0xcac - LTC PKHA N2 11 Register"]
    pub ltc0_pkn2_11: LTC0_PKN2_11,
}
#[doc = "LTC PKHA N 44 Register LTC PKHA N2 12 Register"]
#[repr(C)]
pub union LTC0_PKN_44_UNION {
    #[doc = "0xcb0 - LTC PKHA N 44 Register"]
    pub ltc0_pkn_44: LTC0_PKN_44,
    #[doc = "0xcb0 - LTC PKHA N2 12 Register"]
    pub ltc0_pkn2_12: LTC0_PKN2_12,
}
#[doc = "LTC PKHA N 45 Register LTC PKHA N2 13 Register"]
#[repr(C)]
pub union LTC0_PKN_45_UNION {
    #[doc = "0xcb4 - LTC PKHA N 45 Register"]
    pub ltc0_pkn_45: LTC0_PKN_45,
    #[doc = "0xcb4 - LTC PKHA N2 13 Register"]
    pub ltc0_pkn2_13: LTC0_PKN2_13,
}
#[doc = "LTC PKHA N 46 Register LTC PKHA N2 14 Register"]
#[repr(C)]
pub union LTC0_PKN_46_UNION {
    #[doc = "0xcb8 - LTC PKHA N 46 Register"]
    pub ltc0_pkn_46: LTC0_PKN_46,
    #[doc = "0xcb8 - LTC PKHA N2 14 Register"]
    pub ltc0_pkn2_14: LTC0_PKN2_14,
}
#[doc = "LTC PKHA N 47 Register LTC PKHA N2 15 Register"]
#[repr(C)]
pub union LTC0_PKN_47_UNION {
    #[doc = "0xcbc - LTC PKHA N 47 Register"]
    pub ltc0_pkn_47: LTC0_PKN_47,
    #[doc = "0xcbc - LTC PKHA N2 15 Register"]
    pub ltc0_pkn2_15: LTC0_PKN2_15,
}
#[doc = "LTC PKHA N 48 Register LTC PKHA N3 0 Register"]
#[repr(C)]
pub union LTC0_PKN3_0_UNION {
    #[doc = "0xcc0 - LTC PKHA N 48 Register"]
    pub ltc0_pkn_48: LTC0_PKN_48,
    #[doc = "0xcc0 - LTC PKHA N3 0 Register"]
    pub ltc0_pkn3_0: LTC0_PKN3_0,
}
#[doc = "LTC PKHA N 49 Register LTC PKHA N3 1 Register"]
#[repr(C)]
pub union LTC0_PKN3_1_UNION {
    #[doc = "0xcc4 - LTC PKHA N 49 Register"]
    pub ltc0_pkn_49: LTC0_PKN_49,
    #[doc = "0xcc4 - LTC PKHA N3 1 Register"]
    pub ltc0_pkn3_1: LTC0_PKN3_1,
}
#[doc = "LTC PKHA N 50 Register LTC PKHA N3 2 Register"]
#[repr(C)]
pub union LTC0_PKN3_2_UNION {
    #[doc = "0xcc8 - LTC PKHA N 50 Register"]
    pub ltc0_pkn_50: LTC0_PKN_50,
    #[doc = "0xcc8 - LTC PKHA N3 2 Register"]
    pub ltc0_pkn3_2: LTC0_PKN3_2,
}
#[doc = "LTC PKHA N 51 Register LTC PKHA N3 3 Register"]
#[repr(C)]
pub union LTC0_PKN3_3_UNION {
    #[doc = "0xccc - LTC PKHA N 51 Register"]
    pub ltc0_pkn_51: LTC0_PKN_51,
    #[doc = "0xccc - LTC PKHA N3 3 Register"]
    pub ltc0_pkn3_3: LTC0_PKN3_3,
}
#[doc = "LTC PKHA N 52 Register LTC PKHA N3 4 Register"]
#[repr(C)]
pub union LTC0_PKN3_4_UNION {
    #[doc = "0xcd0 - LTC PKHA N 52 Register"]
    pub ltc0_pkn_52: LTC0_PKN_52,
    #[doc = "0xcd0 - LTC PKHA N3 4 Register"]
    pub ltc0_pkn3_4: LTC0_PKN3_4,
}
#[doc = "LTC PKHA N 53 Register LTC PKHA N3 5 Register"]
#[repr(C)]
pub union LTC0_PKN3_5_UNION {
    #[doc = "0xcd4 - LTC PKHA N 53 Register"]
    pub ltc0_pkn_53: LTC0_PKN_53,
    #[doc = "0xcd4 - LTC PKHA N3 5 Register"]
    pub ltc0_pkn3_5: LTC0_PKN3_5,
}
#[doc = "LTC PKHA N 54 Register LTC PKHA N3 6 Register"]
#[repr(C)]
pub union LTC0_PKN3_6_UNION {
    #[doc = "0xcd8 - LTC PKHA N 54 Register"]
    pub ltc0_pkn_54: LTC0_PKN_54,
    #[doc = "0xcd8 - LTC PKHA N3 6 Register"]
    pub ltc0_pkn3_6: LTC0_PKN3_6,
}
#[doc = "LTC PKHA N 55 Register LTC PKHA N3 7 Register"]
#[repr(C)]
pub union LTC0_PKN3_7_UNION {
    #[doc = "0xcdc - LTC PKHA N 55 Register"]
    pub ltc0_pkn_55: LTC0_PKN_55,
    #[doc = "0xcdc - LTC PKHA N3 7 Register"]
    pub ltc0_pkn3_7: LTC0_PKN3_7,
}
#[doc = "LTC PKHA N 56 Register LTC PKHA N3 8 Register"]
#[repr(C)]
pub union LTC0_PKN3_8_UNION {
    #[doc = "0xce0 - LTC PKHA N 56 Register"]
    pub ltc0_pkn_56: LTC0_PKN_56,
    #[doc = "0xce0 - LTC PKHA N3 8 Register"]
    pub ltc0_pkn3_8: LTC0_PKN3_8,
}
#[doc = "LTC PKHA N 57 Register LTC PKHA N3 9 Register"]
#[repr(C)]
pub union LTC0_PKN3_9_UNION {
    #[doc = "0xce4 - LTC PKHA N 57 Register"]
    pub ltc0_pkn_57: LTC0_PKN_57,
    #[doc = "0xce4 - LTC PKHA N3 9 Register"]
    pub ltc0_pkn3_9: LTC0_PKN3_9,
}
#[doc = "LTC PKHA N 58 Register LTC PKHA N3 10 Register"]
#[repr(C)]
pub union LTC0_PKN_58_UNION {
    #[doc = "0xce8 - LTC PKHA N 58 Register"]
    pub ltc0_pkn_58: LTC0_PKN_58,
    #[doc = "0xce8 - LTC PKHA N3 10 Register"]
    pub ltc0_pkn3_10: LTC0_PKN3_10,
}
#[doc = "LTC PKHA N 59 Register LTC PKHA N3 11 Register"]
#[repr(C)]
pub union LTC0_PKN_59_UNION {
    #[doc = "0xcec - LTC PKHA N 59 Register"]
    pub ltc0_pkn_59: LTC0_PKN_59,
    #[doc = "0xcec - LTC PKHA N3 11 Register"]
    pub ltc0_pkn3_11: LTC0_PKN3_11,
}
#[doc = "LTC PKHA N 60 Register LTC PKHA N3 12 Register"]
#[repr(C)]
pub union LTC0_PKN_60_UNION {
    #[doc = "0xcf0 - LTC PKHA N 60 Register"]
    pub ltc0_pkn_60: LTC0_PKN_60,
    #[doc = "0xcf0 - LTC PKHA N3 12 Register"]
    pub ltc0_pkn3_12: LTC0_PKN3_12,
}
#[doc = "LTC PKHA N 61 Register LTC PKHA N3 13 Register"]
#[repr(C)]
pub union LTC0_PKN_61_UNION {
    #[doc = "0xcf4 - LTC PKHA N 61 Register"]
    pub ltc0_pkn_61: LTC0_PKN_61,
    #[doc = "0xcf4 - LTC PKHA N3 13 Register"]
    pub ltc0_pkn3_13: LTC0_PKN3_13,
}
#[doc = "LTC PKHA N 62 Register LTC PKHA N3 14 Register"]
#[repr(C)]
pub union LTC0_PKN_62_UNION {
    #[doc = "0xcf8 - LTC PKHA N 62 Register"]
    pub ltc0_pkn_62: LTC0_PKN_62,
    #[doc = "0xcf8 - LTC PKHA N3 14 Register"]
    pub ltc0_pkn3_14: LTC0_PKN3_14,
}
#[doc = "LTC PKHA N 63 Register LTC PKHA N3 15 Register"]
#[repr(C)]
pub union LTC0_PKN_63_UNION {
    #[doc = "0xcfc - LTC PKHA N 63 Register"]
    pub ltc0_pkn_63: LTC0_PKN_63,
    #[doc = "0xcfc - LTC PKHA N3 15 Register"]
    pub ltc0_pkn3_15: LTC0_PKN3_15,
}
#[doc = "LTC PKHA E 0 Register LTC PKHA E0 0 Register"]
#[repr(C)]
pub union LTC0_PKE_0_UNION {
    #[doc = "0xe00 - LTC PKHA E 0 Register"]
    pub ltc0_pke_0: LTC0_PKE_0,
    #[doc = "0xe00 - LTC PKHA E0 0 Register"]
    pub ltc0_pke0_0: LTC0_PKE0_0,
}
#[doc = "LTC PKHA E 1 Register LTC PKHA E0 1 Register"]
#[repr(C)]
pub union LTC0_PKE_1_UNION {
    #[doc = "0xe04 - LTC PKHA E 1 Register"]
    pub ltc0_pke_1: LTC0_PKE_1,
    #[doc = "0xe04 - LTC PKHA E0 1 Register"]
    pub ltc0_pke0_1: LTC0_PKE0_1,
}
#[doc = "LTC PKHA E 2 Register LTC PKHA E0 2 Register"]
#[repr(C)]
pub union LTC0_PKE_2_UNION {
    #[doc = "0xe08 - LTC PKHA E 2 Register"]
    pub ltc0_pke_2: LTC0_PKE_2,
    #[doc = "0xe08 - LTC PKHA E0 2 Register"]
    pub ltc0_pke0_2: LTC0_PKE0_2,
}
#[doc = "LTC PKHA E 3 Register LTC PKHA E0 3 Register"]
#[repr(C)]
pub union LTC0_PKE_3_UNION {
    #[doc = "0xe0c - LTC PKHA E 3 Register"]
    pub ltc0_pke_3: LTC0_PKE_3,
    #[doc = "0xe0c - LTC PKHA E0 3 Register"]
    pub ltc0_pke0_3: LTC0_PKE0_3,
}
#[doc = "LTC PKHA E 4 Register LTC PKHA E0 4 Register"]
#[repr(C)]
pub union LTC0_PKE_4_UNION {
    #[doc = "0xe10 - LTC PKHA E 4 Register"]
    pub ltc0_pke_4: LTC0_PKE_4,
    #[doc = "0xe10 - LTC PKHA E0 4 Register"]
    pub ltc0_pke0_4: LTC0_PKE0_4,
}
#[doc = "LTC PKHA E 5 Register LTC PKHA E0 5 Register"]
#[repr(C)]
pub union LTC0_PKE_5_UNION {
    #[doc = "0xe14 - LTC PKHA E 5 Register"]
    pub ltc0_pke_5: LTC0_PKE_5,
    #[doc = "0xe14 - LTC PKHA E0 5 Register"]
    pub ltc0_pke0_5: LTC0_PKE0_5,
}
#[doc = "LTC PKHA E 6 Register LTC PKHA E0 6 Register"]
#[repr(C)]
pub union LTC0_PKE_6_UNION {
    #[doc = "0xe18 - LTC PKHA E 6 Register"]
    pub ltc0_pke_6: LTC0_PKE_6,
    #[doc = "0xe18 - LTC PKHA E0 6 Register"]
    pub ltc0_pke0_6: LTC0_PKE0_6,
}
#[doc = "LTC PKHA E 7 Register LTC PKHA E0 7 Register"]
#[repr(C)]
pub union LTC0_PKE_7_UNION {
    #[doc = "0xe1c - LTC PKHA E 7 Register"]
    pub ltc0_pke_7: LTC0_PKE_7,
    #[doc = "0xe1c - LTC PKHA E0 7 Register"]
    pub ltc0_pke0_7: LTC0_PKE0_7,
}
#[doc = "LTC PKHA E 8 Register LTC PKHA E0 8 Register"]
#[repr(C)]
pub union LTC0_PKE_8_UNION {
    #[doc = "0xe20 - LTC PKHA E 8 Register"]
    pub ltc0_pke_8: LTC0_PKE_8,
    #[doc = "0xe20 - LTC PKHA E0 8 Register"]
    pub ltc0_pke0_8: LTC0_PKE0_8,
}
#[doc = "LTC PKHA E 9 Register LTC PKHA E0 9 Register"]
#[repr(C)]
pub union LTC0_PKE_9_UNION {
    #[doc = "0xe24 - LTC PKHA E 9 Register"]
    pub ltc0_pke_9: LTC0_PKE_9,
    #[doc = "0xe24 - LTC PKHA E0 9 Register"]
    pub ltc0_pke0_9: LTC0_PKE0_9,
}
#[doc = "LTC PKHA E 10 Register LTC PKHA E0 10 Register"]
#[repr(C)]
pub union LTC0_PKE_10_UNION {
    #[doc = "0xe28 - LTC PKHA E 10 Register"]
    pub ltc0_pke_10: LTC0_PKE_10,
    #[doc = "0xe28 - LTC PKHA E0 10 Register"]
    pub ltc0_pke0_10: LTC0_PKE0_10,
}
#[doc = "LTC PKHA E 11 Register LTC PKHA E0 11 Register"]
#[repr(C)]
pub union LTC0_PKE_11_UNION {
    #[doc = "0xe2c - LTC PKHA E 11 Register"]
    pub ltc0_pke_11: LTC0_PKE_11,
    #[doc = "0xe2c - LTC PKHA E0 11 Register"]
    pub ltc0_pke0_11: LTC0_PKE0_11,
}
#[doc = "LTC PKHA E 12 Register LTC PKHA E0 12 Register"]
#[repr(C)]
pub union LTC0_PKE_12_UNION {
    #[doc = "0xe30 - LTC PKHA E 12 Register"]
    pub ltc0_pke_12: LTC0_PKE_12,
    #[doc = "0xe30 - LTC PKHA E0 12 Register"]
    pub ltc0_pke0_12: LTC0_PKE0_12,
}
#[doc = "LTC PKHA E 13 Register LTC PKHA E0 13 Register"]
#[repr(C)]
pub union LTC0_PKE_13_UNION {
    #[doc = "0xe34 - LTC PKHA E 13 Register"]
    pub ltc0_pke_13: LTC0_PKE_13,
    #[doc = "0xe34 - LTC PKHA E0 13 Register"]
    pub ltc0_pke0_13: LTC0_PKE0_13,
}
#[doc = "LTC PKHA E 14 Register LTC PKHA E0 14 Register"]
#[repr(C)]
pub union LTC0_PKE_14_UNION {
    #[doc = "0xe38 - LTC PKHA E 14 Register"]
    pub ltc0_pke_14: LTC0_PKE_14,
    #[doc = "0xe38 - LTC PKHA E0 14 Register"]
    pub ltc0_pke0_14: LTC0_PKE0_14,
}
#[doc = "LTC PKHA E 15 Register LTC PKHA E0 15 Register"]
#[repr(C)]
pub union LTC0_PKE_15_UNION {
    #[doc = "0xe3c - LTC PKHA E 15 Register"]
    pub ltc0_pke_15: LTC0_PKE_15,
    #[doc = "0xe3c - LTC PKHA E0 15 Register"]
    pub ltc0_pke0_15: LTC0_PKE0_15,
}
#[doc = "LTC PKHA E 16 Register LTC PKHA E1 0 Register"]
#[repr(C)]
pub union LTC0_PKE1_0_UNION {
    #[doc = "0xe40 - LTC PKHA E 16 Register"]
    pub ltc0_pke_16: LTC0_PKE_16,
    #[doc = "0xe40 - LTC PKHA E1 0 Register"]
    pub ltc0_pke1_0: LTC0_PKE1_0,
}
#[doc = "LTC PKHA E 17 Register LTC PKHA E1 1 Register"]
#[repr(C)]
pub union LTC0_PKE1_1_UNION {
    #[doc = "0xe44 - LTC PKHA E 17 Register"]
    pub ltc0_pke_17: LTC0_PKE_17,
    #[doc = "0xe44 - LTC PKHA E1 1 Register"]
    pub ltc0_pke1_1: LTC0_PKE1_1,
}
#[doc = "LTC PKHA E 18 Register LTC PKHA E1 2 Register"]
#[repr(C)]
pub union LTC0_PKE1_2_UNION {
    #[doc = "0xe48 - LTC PKHA E 18 Register"]
    pub ltc0_pke_18: LTC0_PKE_18,
    #[doc = "0xe48 - LTC PKHA E1 2 Register"]
    pub ltc0_pke1_2: LTC0_PKE1_2,
}
#[doc = "LTC PKHA E 19 Register LTC PKHA E1 3 Register"]
#[repr(C)]
pub union LTC0_PKE1_3_UNION {
    #[doc = "0xe4c - LTC PKHA E 19 Register"]
    pub ltc0_pke_19: LTC0_PKE_19,
    #[doc = "0xe4c - LTC PKHA E1 3 Register"]
    pub ltc0_pke1_3: LTC0_PKE1_3,
}
#[doc = "LTC PKHA E 20 Register LTC PKHA E1 4 Register"]
#[repr(C)]
pub union LTC0_PKE1_4_UNION {
    #[doc = "0xe50 - LTC PKHA E 20 Register"]
    pub ltc0_pke_20: LTC0_PKE_20,
    #[doc = "0xe50 - LTC PKHA E1 4 Register"]
    pub ltc0_pke1_4: LTC0_PKE1_4,
}
#[doc = "LTC PKHA E 21 Register LTC PKHA E1 5 Register"]
#[repr(C)]
pub union LTC0_PKE1_5_UNION {
    #[doc = "0xe54 - LTC PKHA E 21 Register"]
    pub ltc0_pke_21: LTC0_PKE_21,
    #[doc = "0xe54 - LTC PKHA E1 5 Register"]
    pub ltc0_pke1_5: LTC0_PKE1_5,
}
#[doc = "LTC PKHA E 22 Register LTC PKHA E1 6 Register"]
#[repr(C)]
pub union LTC0_PKE1_6_UNION {
    #[doc = "0xe58 - LTC PKHA E 22 Register"]
    pub ltc0_pke_22: LTC0_PKE_22,
    #[doc = "0xe58 - LTC PKHA E1 6 Register"]
    pub ltc0_pke1_6: LTC0_PKE1_6,
}
#[doc = "LTC PKHA E 23 Register LTC PKHA E1 7 Register"]
#[repr(C)]
pub union LTC0_PKE1_7_UNION {
    #[doc = "0xe5c - LTC PKHA E 23 Register"]
    pub ltc0_pke_23: LTC0_PKE_23,
    #[doc = "0xe5c - LTC PKHA E1 7 Register"]
    pub ltc0_pke1_7: LTC0_PKE1_7,
}
#[doc = "LTC PKHA E 24 Register LTC PKHA E1 8 Register"]
#[repr(C)]
pub union LTC0_PKE1_8_UNION {
    #[doc = "0xe60 - LTC PKHA E 24 Register"]
    pub ltc0_pke_24: LTC0_PKE_24,
    #[doc = "0xe60 - LTC PKHA E1 8 Register"]
    pub ltc0_pke1_8: LTC0_PKE1_8,
}
#[doc = "LTC PKHA E 25 Register LTC PKHA E1 9 Register"]
#[repr(C)]
pub union LTC0_PKE1_9_UNION {
    #[doc = "0xe64 - LTC PKHA E 25 Register"]
    pub ltc0_pke_25: LTC0_PKE_25,
    #[doc = "0xe64 - LTC PKHA E1 9 Register"]
    pub ltc0_pke1_9: LTC0_PKE1_9,
}
#[doc = "LTC PKHA E 26 Register LTC PKHA E1 10 Register"]
#[repr(C)]
pub union LTC0_PKE_26_UNION {
    #[doc = "0xe68 - LTC PKHA E 26 Register"]
    pub ltc0_pke_26: LTC0_PKE_26,
    #[doc = "0xe68 - LTC PKHA E1 10 Register"]
    pub ltc0_pke1_10: LTC0_PKE1_10,
}
#[doc = "LTC PKHA E 27 Register LTC PKHA E1 11 Register"]
#[repr(C)]
pub union LTC0_PKE_27_UNION {
    #[doc = "0xe6c - LTC PKHA E 27 Register"]
    pub ltc0_pke_27: LTC0_PKE_27,
    #[doc = "0xe6c - LTC PKHA E1 11 Register"]
    pub ltc0_pke1_11: LTC0_PKE1_11,
}
#[doc = "LTC PKHA E 28 Register LTC PKHA E1 12 Register"]
#[repr(C)]
pub union LTC0_PKE_28_UNION {
    #[doc = "0xe70 - LTC PKHA E 28 Register"]
    pub ltc0_pke_28: LTC0_PKE_28,
    #[doc = "0xe70 - LTC PKHA E1 12 Register"]
    pub ltc0_pke1_12: LTC0_PKE1_12,
}
#[doc = "LTC PKHA E 29 Register LTC PKHA E1 13 Register"]
#[repr(C)]
pub union LTC0_PKE_29_UNION {
    #[doc = "0xe74 - LTC PKHA E 29 Register"]
    pub ltc0_pke_29: LTC0_PKE_29,
    #[doc = "0xe74 - LTC PKHA E1 13 Register"]
    pub ltc0_pke1_13: LTC0_PKE1_13,
}
#[doc = "LTC PKHA E 30 Register LTC PKHA E1 14 Register"]
#[repr(C)]
pub union LTC0_PKE_30_UNION {
    #[doc = "0xe78 - LTC PKHA E 30 Register"]
    pub ltc0_pke_30: LTC0_PKE_30,
    #[doc = "0xe78 - LTC PKHA E1 14 Register"]
    pub ltc0_pke1_14: LTC0_PKE1_14,
}
#[doc = "LTC PKHA E 31 Register LTC PKHA E1 15 Register"]
#[repr(C)]
pub union LTC0_PKE_31_UNION {
    #[doc = "0xe7c - LTC PKHA E 31 Register"]
    pub ltc0_pke_31: LTC0_PKE_31,
    #[doc = "0xe7c - LTC PKHA E1 15 Register"]
    pub ltc0_pke1_15: LTC0_PKE1_15,
}
#[doc = "LTC PKHA E 32 Register LTC PKHA E2 0 Register"]
#[repr(C)]
pub union LTC0_PKE2_0_UNION {
    #[doc = "0xe80 - LTC PKHA E 32 Register"]
    pub ltc0_pke_32: LTC0_PKE_32,
    #[doc = "0xe80 - LTC PKHA E2 0 Register"]
    pub ltc0_pke2_0: LTC0_PKE2_0,
}
#[doc = "LTC PKHA E 33 Register LTC PKHA E2 1 Register"]
#[repr(C)]
pub union LTC0_PKE2_1_UNION {
    #[doc = "0xe84 - LTC PKHA E 33 Register"]
    pub ltc0_pke_33: LTC0_PKE_33,
    #[doc = "0xe84 - LTC PKHA E2 1 Register"]
    pub ltc0_pke2_1: LTC0_PKE2_1,
}
#[doc = "LTC PKHA E 34 Register LTC PKHA E2 2 Register"]
#[repr(C)]
pub union LTC0_PKE2_2_UNION {
    #[doc = "0xe88 - LTC PKHA E 34 Register"]
    pub ltc0_pke_34: LTC0_PKE_34,
    #[doc = "0xe88 - LTC PKHA E2 2 Register"]
    pub ltc0_pke2_2: LTC0_PKE2_2,
}
#[doc = "LTC PKHA E 35 Register LTC PKHA E2 3 Register"]
#[repr(C)]
pub union LTC0_PKE2_3_UNION {
    #[doc = "0xe8c - LTC PKHA E 35 Register"]
    pub ltc0_pke_35: LTC0_PKE_35,
    #[doc = "0xe8c - LTC PKHA E2 3 Register"]
    pub ltc0_pke2_3: LTC0_PKE2_3,
}
#[doc = "LTC PKHA E 36 Register LTC PKHA E2 4 Register"]
#[repr(C)]
pub union LTC0_PKE2_4_UNION {
    #[doc = "0xe90 - LTC PKHA E 36 Register"]
    pub ltc0_pke_36: LTC0_PKE_36,
    #[doc = "0xe90 - LTC PKHA E2 4 Register"]
    pub ltc0_pke2_4: LTC0_PKE2_4,
}
#[doc = "LTC PKHA E 37 Register LTC PKHA E2 5 Register"]
#[repr(C)]
pub union LTC0_PKE2_5_UNION {
    #[doc = "0xe94 - LTC PKHA E 37 Register"]
    pub ltc0_pke_37: LTC0_PKE_37,
    #[doc = "0xe94 - LTC PKHA E2 5 Register"]
    pub ltc0_pke2_5: LTC0_PKE2_5,
}
#[doc = "LTC PKHA E 38 Register LTC PKHA E2 6 Register"]
#[repr(C)]
pub union LTC0_PKE2_6_UNION {
    #[doc = "0xe98 - LTC PKHA E 38 Register"]
    pub ltc0_pke_38: LTC0_PKE_38,
    #[doc = "0xe98 - LTC PKHA E2 6 Register"]
    pub ltc0_pke2_6: LTC0_PKE2_6,
}
#[doc = "LTC PKHA E 39 Register LTC PKHA E2 7 Register"]
#[repr(C)]
pub union LTC0_PKE2_7_UNION {
    #[doc = "0xe9c - LTC PKHA E 39 Register"]
    pub ltc0_pke_39: LTC0_PKE_39,
    #[doc = "0xe9c - LTC PKHA E2 7 Register"]
    pub ltc0_pke2_7: LTC0_PKE2_7,
}
#[doc = "LTC PKHA E 40 Register LTC PKHA E2 8 Register"]
#[repr(C)]
pub union LTC0_PKE2_8_UNION {
    #[doc = "0xea0 - LTC PKHA E 40 Register"]
    pub ltc0_pke_40: LTC0_PKE_40,
    #[doc = "0xea0 - LTC PKHA E2 8 Register"]
    pub ltc0_pke2_8: LTC0_PKE2_8,
}
#[doc = "LTC PKHA E 41 Register LTC PKHA E2 9 Register"]
#[repr(C)]
pub union LTC0_PKE2_9_UNION {
    #[doc = "0xea4 - LTC PKHA E 41 Register"]
    pub ltc0_pke_41: LTC0_PKE_41,
    #[doc = "0xea4 - LTC PKHA E2 9 Register"]
    pub ltc0_pke2_9: LTC0_PKE2_9,
}
#[doc = "LTC PKHA E 42 Register LTC PKHA E2 10 Register"]
#[repr(C)]
pub union LTC0_PKE_42_UNION {
    #[doc = "0xea8 - LTC PKHA E 42 Register"]
    pub ltc0_pke_42: LTC0_PKE_42,
    #[doc = "0xea8 - LTC PKHA E2 10 Register"]
    pub ltc0_pke2_10: LTC0_PKE2_10,
}
#[doc = "LTC PKHA E 43 Register LTC PKHA E2 11 Register"]
#[repr(C)]
pub union LTC0_PKE_43_UNION {
    #[doc = "0xeac - LTC PKHA E 43 Register"]
    pub ltc0_pke_43: LTC0_PKE_43,
    #[doc = "0xeac - LTC PKHA E2 11 Register"]
    pub ltc0_pke2_11: LTC0_PKE2_11,
}
#[doc = "LTC PKHA E 44 Register LTC PKHA E2 12 Register"]
#[repr(C)]
pub union LTC0_PKE_44_UNION {
    #[doc = "0xeb0 - LTC PKHA E 44 Register"]
    pub ltc0_pke_44: LTC0_PKE_44,
    #[doc = "0xeb0 - LTC PKHA E2 12 Register"]
    pub ltc0_pke2_12: LTC0_PKE2_12,
}
#[doc = "LTC PKHA E 45 Register LTC PKHA E2 13 Register"]
#[repr(C)]
pub union LTC0_PKE_45_UNION {
    #[doc = "0xeb4 - LTC PKHA E 45 Register"]
    pub ltc0_pke_45: LTC0_PKE_45,
    #[doc = "0xeb4 - LTC PKHA E2 13 Register"]
    pub ltc0_pke2_13: LTC0_PKE2_13,
}
#[doc = "LTC PKHA E 46 Register LTC PKHA E2 14 Register"]
#[repr(C)]
pub union LTC0_PKE_46_UNION {
    #[doc = "0xeb8 - LTC PKHA E 46 Register"]
    pub ltc0_pke_46: LTC0_PKE_46,
    #[doc = "0xeb8 - LTC PKHA E2 14 Register"]
    pub ltc0_pke2_14: LTC0_PKE2_14,
}
#[doc = "LTC PKHA E 47 Register LTC PKHA E2 15 Register"]
#[repr(C)]
pub union LTC0_PKE_47_UNION {
    #[doc = "0xebc - LTC PKHA E 47 Register"]
    pub ltc0_pke_47: LTC0_PKE_47,
    #[doc = "0xebc - LTC PKHA E2 15 Register"]
    pub ltc0_pke2_15: LTC0_PKE2_15,
}
#[doc = "LTC PKHA E 48 Register LTC PKHA E3 0 Register"]
#[repr(C)]
pub union LTC0_PKE3_0_UNION {
    #[doc = "0xec0 - LTC PKHA E 48 Register"]
    pub ltc0_pke_48: LTC0_PKE_48,
    #[doc = "0xec0 - LTC PKHA E3 0 Register"]
    pub ltc0_pke3_0: LTC0_PKE3_0,
}
#[doc = "LTC PKHA E 49 Register LTC PKHA E3 1 Register"]
#[repr(C)]
pub union LTC0_PKE3_1_UNION {
    #[doc = "0xec4 - LTC PKHA E 49 Register"]
    pub ltc0_pke_49: LTC0_PKE_49,
    #[doc = "0xec4 - LTC PKHA E3 1 Register"]
    pub ltc0_pke3_1: LTC0_PKE3_1,
}
#[doc = "LTC PKHA E 50 Register LTC PKHA E3 2 Register"]
#[repr(C)]
pub union LTC0_PKE3_2_UNION {
    #[doc = "0xec8 - LTC PKHA E 50 Register"]
    pub ltc0_pke_50: LTC0_PKE_50,
    #[doc = "0xec8 - LTC PKHA E3 2 Register"]
    pub ltc0_pke3_2: LTC0_PKE3_2,
}
#[doc = "LTC PKHA E 51 Register LTC PKHA E3 3 Register"]
#[repr(C)]
pub union LTC0_PKE3_3_UNION {
    #[doc = "0xecc - LTC PKHA E 51 Register"]
    pub ltc0_pke_51: LTC0_PKE_51,
    #[doc = "0xecc - LTC PKHA E3 3 Register"]
    pub ltc0_pke3_3: LTC0_PKE3_3,
}
#[doc = "LTC PKHA E 52 Register LTC PKHA E3 4 Register"]
#[repr(C)]
pub union LTC0_PKE3_4_UNION {
    #[doc = "0xed0 - LTC PKHA E 52 Register"]
    pub ltc0_pke_52: LTC0_PKE_52,
    #[doc = "0xed0 - LTC PKHA E3 4 Register"]
    pub ltc0_pke3_4: LTC0_PKE3_4,
}
#[doc = "LTC PKHA E 53 Register LTC PKHA E3 5 Register"]
#[repr(C)]
pub union LTC0_PKE3_5_UNION {
    #[doc = "0xed4 - LTC PKHA E 53 Register"]
    pub ltc0_pke_53: LTC0_PKE_53,
    #[doc = "0xed4 - LTC PKHA E3 5 Register"]
    pub ltc0_pke3_5: LTC0_PKE3_5,
}
#[doc = "LTC PKHA E 54 Register LTC PKHA E3 6 Register"]
#[repr(C)]
pub union LTC0_PKE3_6_UNION {
    #[doc = "0xed8 - LTC PKHA E 54 Register"]
    pub ltc0_pke_54: LTC0_PKE_54,
    #[doc = "0xed8 - LTC PKHA E3 6 Register"]
    pub ltc0_pke3_6: LTC0_PKE3_6,
}
#[doc = "LTC PKHA E 55 Register LTC PKHA E3 7 Register"]
#[repr(C)]
pub union LTC0_PKE3_7_UNION {
    #[doc = "0xedc - LTC PKHA E 55 Register"]
    pub ltc0_pke_55: LTC0_PKE_55,
    #[doc = "0xedc - LTC PKHA E3 7 Register"]
    pub ltc0_pke3_7: LTC0_PKE3_7,
}
#[doc = "LTC PKHA E 56 Register LTC PKHA E3 8 Register"]
#[repr(C)]
pub union LTC0_PKE3_8_UNION {
    #[doc = "0xee0 - LTC PKHA E 56 Register"]
    pub ltc0_pke_56: LTC0_PKE_56,
    #[doc = "0xee0 - LTC PKHA E3 8 Register"]
    pub ltc0_pke3_8: LTC0_PKE3_8,
}
#[doc = "LTC PKHA E 57 Register LTC PKHA E3 9 Register"]
#[repr(C)]
pub union LTC0_PKE3_9_UNION {
    #[doc = "0xee4 - LTC PKHA E 57 Register"]
    pub ltc0_pke_57: LTC0_PKE_57,
    #[doc = "0xee4 - LTC PKHA E3 9 Register"]
    pub ltc0_pke3_9: LTC0_PKE3_9,
}
#[doc = "LTC PKHA E 58 Register LTC PKHA E3 10 Register"]
#[repr(C)]
pub union LTC0_PKE_58_UNION {
    #[doc = "0xee8 - LTC PKHA E 58 Register"]
    pub ltc0_pke_58: LTC0_PKE_58,
    #[doc = "0xee8 - LTC PKHA E3 10 Register"]
    pub ltc0_pke3_10: LTC0_PKE3_10,
}
#[doc = "LTC PKHA E 59 Register LTC PKHA E3 11 Register"]
#[repr(C)]
pub union LTC0_PKE_59_UNION {
    #[doc = "0xeec - LTC PKHA E 59 Register"]
    pub ltc0_pke_59: LTC0_PKE_59,
    #[doc = "0xeec - LTC PKHA E3 11 Register"]
    pub ltc0_pke3_11: LTC0_PKE3_11,
}
#[doc = "LTC PKHA E 60 Register LTC PKHA E3 12 Register"]
#[repr(C)]
pub union LTC0_PKE_60_UNION {
    #[doc = "0xef0 - LTC PKHA E 60 Register"]
    pub ltc0_pke_60: LTC0_PKE_60,
    #[doc = "0xef0 - LTC PKHA E3 12 Register"]
    pub ltc0_pke3_12: LTC0_PKE3_12,
}
#[doc = "LTC PKHA E 61 Register LTC PKHA E3 13 Register"]
#[repr(C)]
pub union LTC0_PKE_61_UNION {
    #[doc = "0xef4 - LTC PKHA E 61 Register"]
    pub ltc0_pke_61: LTC0_PKE_61,
    #[doc = "0xef4 - LTC PKHA E3 13 Register"]
    pub ltc0_pke3_13: LTC0_PKE3_13,
}
#[doc = "LTC PKHA E 62 Register LTC PKHA E3 14 Register"]
#[repr(C)]
pub union LTC0_PKE_62_UNION {
    #[doc = "0xef8 - LTC PKHA E 62 Register"]
    pub ltc0_pke_62: LTC0_PKE_62,
    #[doc = "0xef8 - LTC PKHA E3 14 Register"]
    pub ltc0_pke3_14: LTC0_PKE3_14,
}
#[doc = "LTC PKHA E 63 Register LTC PKHA E3 15 Register"]
#[repr(C)]
pub union LTC0_PKE_63_UNION {
    #[doc = "0xefc - LTC PKHA E 63 Register"]
    pub ltc0_pke_63: LTC0_PKE_63,
    #[doc = "0xefc - LTC PKHA E3 15 Register"]
    pub ltc0_pke3_15: LTC0_PKE3_15,
}
#[doc = "LTC Mode Register (non-PKHA/non-RNG use)"]
pub struct LTC0_MD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Mode Register (non-PKHA/non-RNG use)"]
pub mod ltc0_md;
#[doc = "LTC Mode Register (PublicKey)"]
pub struct LTC0_MDPK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Mode Register (PublicKey)"]
pub mod ltc0_mdpk;
#[doc = "LTC Key Size Register"]
pub struct LTC0_KS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Size Register"]
pub mod ltc0_ks;
#[doc = "LTC Data Size Register"]
pub struct LTC0_DS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Data Size Register"]
pub mod ltc0_ds;
#[doc = "LTC ICV Size Register"]
pub struct LTC0_ICVS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC ICV Size Register"]
pub mod ltc0_icvs;
#[doc = "LTC Command Register"]
pub struct LTC0_COM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Command Register"]
pub mod ltc0_com;
#[doc = "LTC Control Register"]
pub struct LTC0_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Control Register"]
pub mod ltc0_ctl;
#[doc = "LTC Clear Written Register"]
pub struct LTC0_CW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Clear Written Register"]
pub mod ltc0_cw;
#[doc = "LTC Status Register"]
pub struct LTC0_STA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Status Register"]
pub mod ltc0_sta;
#[doc = "LTC Error Status Register"]
pub struct LTC0_ESTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Error Status Register"]
pub mod ltc0_esta;
#[doc = "LTC AAD Size Register"]
pub struct LTC0_AADSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC AAD Size Register"]
pub mod ltc0_aadsz;
#[doc = "LTC IV Size Register"]
pub struct LTC0_IVSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC IV Size Register"]
pub mod ltc0_ivsz;
#[doc = "LTC DPA Mask Seed Register"]
pub struct LTC0_DPAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC DPA Mask Seed Register"]
pub mod ltc0_dpams;
#[doc = "LTC PKHA A Size Register"]
pub struct LTC0_PKASZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A Size Register"]
pub mod ltc0_pkasz;
#[doc = "LTC PKHA B Size Register"]
pub struct LTC0_PKBSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B Size Register"]
pub mod ltc0_pkbsz;
#[doc = "LTC PKHA N Size Register"]
pub struct LTC0_PKNSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N Size Register"]
pub mod ltc0_pknsz;
#[doc = "LTC PKHA E Size Register"]
pub struct LTC0_PKESZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E Size Register"]
pub mod ltc0_pkesz;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_0;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_1;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_2;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_3;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_4;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_5;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_6;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_7;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_8;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_9;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_10;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_11;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_12;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_13;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_14;
#[doc = "LTC Context Register"]
pub struct LTC0_CTX_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_15;
#[doc = "LTC Key Registers"]
pub struct LTC0_KEY_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_0;
#[doc = "LTC Key Registers"]
pub struct LTC0_KEY_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_1;
#[doc = "LTC Key Registers"]
pub struct LTC0_KEY_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_2;
#[doc = "LTC Key Registers"]
pub struct LTC0_KEY_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_3;
#[doc = "LTC Key Registers"]
pub struct LTC0_KEY_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_4;
#[doc = "LTC Key Registers"]
pub struct LTC0_KEY_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_5;
#[doc = "LTC Key Registers"]
pub struct LTC0_KEY_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_6;
#[doc = "LTC Key Registers"]
pub struct LTC0_KEY_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_7;
#[doc = "LTC Version ID Register"]
pub struct LTC0_VID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Version ID Register"]
pub mod ltc0_vid1;
#[doc = "LTC Version ID 2 Register"]
pub struct LTC0_VID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Version ID 2 Register"]
pub mod ltc0_vid2;
#[doc = "LTC CHA Version ID Register"]
pub struct LTC0_CHAVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC CHA Version ID Register"]
pub mod ltc0_chavid;
#[doc = "LTC FIFO Status Register"]
pub struct LTC0_FIFOSTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC FIFO Status Register"]
pub mod ltc0_fifosta;
#[doc = "LTC Input Data FIFO"]
pub struct LTC0_IFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Input Data FIFO"]
pub mod ltc0_ififo;
#[doc = "LTC Output Data FIFO"]
pub struct LTC0_OFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC Output Data FIFO"]
pub mod ltc0_ofifo;
#[doc = "LTC PKHA A0 0 Register"]
pub struct LTC0_PKA0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 0 Register"]
pub mod ltc0_pka0_0;
#[doc = "LTC PKHA A 0 Register"]
pub struct LTC0_PKA_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 0 Register"]
pub mod ltc0_pka_0;
#[doc = "LTC PKHA A0 1 Register"]
pub struct LTC0_PKA0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 1 Register"]
pub mod ltc0_pka0_1;
#[doc = "LTC PKHA A 1 Register"]
pub struct LTC0_PKA_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 1 Register"]
pub mod ltc0_pka_1;
#[doc = "LTC PKHA A0 2 Register"]
pub struct LTC0_PKA0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 2 Register"]
pub mod ltc0_pka0_2;
#[doc = "LTC PKHA A 2 Register"]
pub struct LTC0_PKA_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 2 Register"]
pub mod ltc0_pka_2;
#[doc = "LTC PKHA A0 3 Register"]
pub struct LTC0_PKA0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 3 Register"]
pub mod ltc0_pka0_3;
#[doc = "LTC PKHA A 3 Register"]
pub struct LTC0_PKA_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 3 Register"]
pub mod ltc0_pka_3;
#[doc = "LTC PKHA A0 4 Register"]
pub struct LTC0_PKA0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 4 Register"]
pub mod ltc0_pka0_4;
#[doc = "LTC PKHA A 4 Register"]
pub struct LTC0_PKA_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 4 Register"]
pub mod ltc0_pka_4;
#[doc = "LTC PKHA A0 5 Register"]
pub struct LTC0_PKA0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 5 Register"]
pub mod ltc0_pka0_5;
#[doc = "LTC PKHA A 5 Register"]
pub struct LTC0_PKA_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 5 Register"]
pub mod ltc0_pka_5;
#[doc = "LTC PKHA A0 6 Register"]
pub struct LTC0_PKA0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 6 Register"]
pub mod ltc0_pka0_6;
#[doc = "LTC PKHA A 6 Register"]
pub struct LTC0_PKA_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 6 Register"]
pub mod ltc0_pka_6;
#[doc = "LTC PKHA A0 7 Register"]
pub struct LTC0_PKA0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 7 Register"]
pub mod ltc0_pka0_7;
#[doc = "LTC PKHA A 7 Register"]
pub struct LTC0_PKA_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 7 Register"]
pub mod ltc0_pka_7;
#[doc = "LTC PKHA A0 8 Register"]
pub struct LTC0_PKA0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 8 Register"]
pub mod ltc0_pka0_8;
#[doc = "LTC PKHA A 8 Register"]
pub struct LTC0_PKA_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 8 Register"]
pub mod ltc0_pka_8;
#[doc = "LTC PKHA A0 9 Register"]
pub struct LTC0_PKA0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 9 Register"]
pub mod ltc0_pka0_9;
#[doc = "LTC PKHA A 9 Register"]
pub struct LTC0_PKA_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 9 Register"]
pub mod ltc0_pka_9;
#[doc = "LTC PKHA A0 10 Register"]
pub struct LTC0_PKA0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 10 Register"]
pub mod ltc0_pka0_10;
#[doc = "LTC PKHA A 10 Register"]
pub struct LTC0_PKA_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 10 Register"]
pub mod ltc0_pka_10;
#[doc = "LTC PKHA A0 11 Register"]
pub struct LTC0_PKA0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 11 Register"]
pub mod ltc0_pka0_11;
#[doc = "LTC PKHA A 11 Register"]
pub struct LTC0_PKA_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 11 Register"]
pub mod ltc0_pka_11;
#[doc = "LTC PKHA A0 12 Register"]
pub struct LTC0_PKA0_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 12 Register"]
pub mod ltc0_pka0_12;
#[doc = "LTC PKHA A 12 Register"]
pub struct LTC0_PKA_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 12 Register"]
pub mod ltc0_pka_12;
#[doc = "LTC PKHA A0 13 Register"]
pub struct LTC0_PKA0_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 13 Register"]
pub mod ltc0_pka0_13;
#[doc = "LTC PKHA A 13 Register"]
pub struct LTC0_PKA_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 13 Register"]
pub mod ltc0_pka_13;
#[doc = "LTC PKHA A0 14 Register"]
pub struct LTC0_PKA0_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 14 Register"]
pub mod ltc0_pka0_14;
#[doc = "LTC PKHA A 14 Register"]
pub struct LTC0_PKA_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 14 Register"]
pub mod ltc0_pka_14;
#[doc = "LTC PKHA A0 15 Register"]
pub struct LTC0_PKA0_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A0 15 Register"]
pub mod ltc0_pka0_15;
#[doc = "LTC PKHA A 15 Register"]
pub struct LTC0_PKA_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 15 Register"]
pub mod ltc0_pka_15;
#[doc = "LTC PKHA A1 0 Register"]
pub struct LTC0_PKA1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 0 Register"]
pub mod ltc0_pka1_0;
#[doc = "LTC PKHA A 16 Register"]
pub struct LTC0_PKA_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 16 Register"]
pub mod ltc0_pka_16;
#[doc = "LTC PKHA A1 1 Register"]
pub struct LTC0_PKA1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 1 Register"]
pub mod ltc0_pka1_1;
#[doc = "LTC PKHA A 17 Register"]
pub struct LTC0_PKA_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 17 Register"]
pub mod ltc0_pka_17;
#[doc = "LTC PKHA A1 2 Register"]
pub struct LTC0_PKA1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 2 Register"]
pub mod ltc0_pka1_2;
#[doc = "LTC PKHA A 18 Register"]
pub struct LTC0_PKA_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 18 Register"]
pub mod ltc0_pka_18;
#[doc = "LTC PKHA A1 3 Register"]
pub struct LTC0_PKA1_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 3 Register"]
pub mod ltc0_pka1_3;
#[doc = "LTC PKHA A 19 Register"]
pub struct LTC0_PKA_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 19 Register"]
pub mod ltc0_pka_19;
#[doc = "LTC PKHA A1 4 Register"]
pub struct LTC0_PKA1_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 4 Register"]
pub mod ltc0_pka1_4;
#[doc = "LTC PKHA A 20 Register"]
pub struct LTC0_PKA_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 20 Register"]
pub mod ltc0_pka_20;
#[doc = "LTC PKHA A1 5 Register"]
pub struct LTC0_PKA1_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 5 Register"]
pub mod ltc0_pka1_5;
#[doc = "LTC PKHA A 21 Register"]
pub struct LTC0_PKA_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 21 Register"]
pub mod ltc0_pka_21;
#[doc = "LTC PKHA A1 6 Register"]
pub struct LTC0_PKA1_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 6 Register"]
pub mod ltc0_pka1_6;
#[doc = "LTC PKHA A 22 Register"]
pub struct LTC0_PKA_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 22 Register"]
pub mod ltc0_pka_22;
#[doc = "LTC PKHA A1 7 Register"]
pub struct LTC0_PKA1_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 7 Register"]
pub mod ltc0_pka1_7;
#[doc = "LTC PKHA A 23 Register"]
pub struct LTC0_PKA_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 23 Register"]
pub mod ltc0_pka_23;
#[doc = "LTC PKHA A1 8 Register"]
pub struct LTC0_PKA1_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 8 Register"]
pub mod ltc0_pka1_8;
#[doc = "LTC PKHA A 24 Register"]
pub struct LTC0_PKA_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 24 Register"]
pub mod ltc0_pka_24;
#[doc = "LTC PKHA A1 9 Register"]
pub struct LTC0_PKA1_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 9 Register"]
pub mod ltc0_pka1_9;
#[doc = "LTC PKHA A 25 Register"]
pub struct LTC0_PKA_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 25 Register"]
pub mod ltc0_pka_25;
#[doc = "LTC PKHA A1 10 Register"]
pub struct LTC0_PKA1_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 10 Register"]
pub mod ltc0_pka1_10;
#[doc = "LTC PKHA A 26 Register"]
pub struct LTC0_PKA_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 26 Register"]
pub mod ltc0_pka_26;
#[doc = "LTC PKHA A1 11 Register"]
pub struct LTC0_PKA1_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 11 Register"]
pub mod ltc0_pka1_11;
#[doc = "LTC PKHA A 27 Register"]
pub struct LTC0_PKA_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 27 Register"]
pub mod ltc0_pka_27;
#[doc = "LTC PKHA A1 12 Register"]
pub struct LTC0_PKA1_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 12 Register"]
pub mod ltc0_pka1_12;
#[doc = "LTC PKHA A 28 Register"]
pub struct LTC0_PKA_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 28 Register"]
pub mod ltc0_pka_28;
#[doc = "LTC PKHA A1 13 Register"]
pub struct LTC0_PKA1_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 13 Register"]
pub mod ltc0_pka1_13;
#[doc = "LTC PKHA A 29 Register"]
pub struct LTC0_PKA_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 29 Register"]
pub mod ltc0_pka_29;
#[doc = "LTC PKHA A1 14 Register"]
pub struct LTC0_PKA1_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 14 Register"]
pub mod ltc0_pka1_14;
#[doc = "LTC PKHA A 30 Register"]
pub struct LTC0_PKA_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 30 Register"]
pub mod ltc0_pka_30;
#[doc = "LTC PKHA A1 15 Register"]
pub struct LTC0_PKA1_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A1 15 Register"]
pub mod ltc0_pka1_15;
#[doc = "LTC PKHA A 31 Register"]
pub struct LTC0_PKA_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 31 Register"]
pub mod ltc0_pka_31;
#[doc = "LTC PKHA A2 0 Register"]
pub struct LTC0_PKA2_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 0 Register"]
pub mod ltc0_pka2_0;
#[doc = "LTC PKHA A 32 Register"]
pub struct LTC0_PKA_32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 32 Register"]
pub mod ltc0_pka_32;
#[doc = "LTC PKHA A2 1 Register"]
pub struct LTC0_PKA2_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 1 Register"]
pub mod ltc0_pka2_1;
#[doc = "LTC PKHA A 33 Register"]
pub struct LTC0_PKA_33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 33 Register"]
pub mod ltc0_pka_33;
#[doc = "LTC PKHA A2 2 Register"]
pub struct LTC0_PKA2_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 2 Register"]
pub mod ltc0_pka2_2;
#[doc = "LTC PKHA A 34 Register"]
pub struct LTC0_PKA_34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 34 Register"]
pub mod ltc0_pka_34;
#[doc = "LTC PKHA A2 3 Register"]
pub struct LTC0_PKA2_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 3 Register"]
pub mod ltc0_pka2_3;
#[doc = "LTC PKHA A 35 Register"]
pub struct LTC0_PKA_35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 35 Register"]
pub mod ltc0_pka_35;
#[doc = "LTC PKHA A2 4 Register"]
pub struct LTC0_PKA2_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 4 Register"]
pub mod ltc0_pka2_4;
#[doc = "LTC PKHA A 36 Register"]
pub struct LTC0_PKA_36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 36 Register"]
pub mod ltc0_pka_36;
#[doc = "LTC PKHA A2 5 Register"]
pub struct LTC0_PKA2_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 5 Register"]
pub mod ltc0_pka2_5;
#[doc = "LTC PKHA A 37 Register"]
pub struct LTC0_PKA_37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 37 Register"]
pub mod ltc0_pka_37;
#[doc = "LTC PKHA A2 6 Register"]
pub struct LTC0_PKA2_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 6 Register"]
pub mod ltc0_pka2_6;
#[doc = "LTC PKHA A 38 Register"]
pub struct LTC0_PKA_38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 38 Register"]
pub mod ltc0_pka_38;
#[doc = "LTC PKHA A2 7 Register"]
pub struct LTC0_PKA2_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 7 Register"]
pub mod ltc0_pka2_7;
#[doc = "LTC PKHA A 39 Register"]
pub struct LTC0_PKA_39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 39 Register"]
pub mod ltc0_pka_39;
#[doc = "LTC PKHA A2 8 Register"]
pub struct LTC0_PKA2_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 8 Register"]
pub mod ltc0_pka2_8;
#[doc = "LTC PKHA A 40 Register"]
pub struct LTC0_PKA_40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 40 Register"]
pub mod ltc0_pka_40;
#[doc = "LTC PKHA A2 9 Register"]
pub struct LTC0_PKA2_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 9 Register"]
pub mod ltc0_pka2_9;
#[doc = "LTC PKHA A 41 Register"]
pub struct LTC0_PKA_41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 41 Register"]
pub mod ltc0_pka_41;
#[doc = "LTC PKHA A2 10 Register"]
pub struct LTC0_PKA2_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 10 Register"]
pub mod ltc0_pka2_10;
#[doc = "LTC PKHA A 42 Register"]
pub struct LTC0_PKA_42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 42 Register"]
pub mod ltc0_pka_42;
#[doc = "LTC PKHA A2 11 Register"]
pub struct LTC0_PKA2_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 11 Register"]
pub mod ltc0_pka2_11;
#[doc = "LTC PKHA A 43 Register"]
pub struct LTC0_PKA_43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 43 Register"]
pub mod ltc0_pka_43;
#[doc = "LTC PKHA A2 12 Register"]
pub struct LTC0_PKA2_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 12 Register"]
pub mod ltc0_pka2_12;
#[doc = "LTC PKHA A 44 Register"]
pub struct LTC0_PKA_44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 44 Register"]
pub mod ltc0_pka_44;
#[doc = "LTC PKHA A2 13 Register"]
pub struct LTC0_PKA2_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 13 Register"]
pub mod ltc0_pka2_13;
#[doc = "LTC PKHA A 45 Register"]
pub struct LTC0_PKA_45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 45 Register"]
pub mod ltc0_pka_45;
#[doc = "LTC PKHA A2 14 Register"]
pub struct LTC0_PKA2_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 14 Register"]
pub mod ltc0_pka2_14;
#[doc = "LTC PKHA A 46 Register"]
pub struct LTC0_PKA_46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 46 Register"]
pub mod ltc0_pka_46;
#[doc = "LTC PKHA A2 15 Register"]
pub struct LTC0_PKA2_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A2 15 Register"]
pub mod ltc0_pka2_15;
#[doc = "LTC PKHA A 47 Register"]
pub struct LTC0_PKA_47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 47 Register"]
pub mod ltc0_pka_47;
#[doc = "LTC PKHA A3 0 Register"]
pub struct LTC0_PKA3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 0 Register"]
pub mod ltc0_pka3_0;
#[doc = "LTC PKHA A 48 Register"]
pub struct LTC0_PKA_48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 48 Register"]
pub mod ltc0_pka_48;
#[doc = "LTC PKHA A3 1 Register"]
pub struct LTC0_PKA3_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 1 Register"]
pub mod ltc0_pka3_1;
#[doc = "LTC PKHA A 49 Register"]
pub struct LTC0_PKA_49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 49 Register"]
pub mod ltc0_pka_49;
#[doc = "LTC PKHA A3 2 Register"]
pub struct LTC0_PKA3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 2 Register"]
pub mod ltc0_pka3_2;
#[doc = "LTC PKHA A 50 Register"]
pub struct LTC0_PKA_50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 50 Register"]
pub mod ltc0_pka_50;
#[doc = "LTC PKHA A3 3 Register"]
pub struct LTC0_PKA3_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 3 Register"]
pub mod ltc0_pka3_3;
#[doc = "LTC PKHA A 51 Register"]
pub struct LTC0_PKA_51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 51 Register"]
pub mod ltc0_pka_51;
#[doc = "LTC PKHA A3 4 Register"]
pub struct LTC0_PKA3_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 4 Register"]
pub mod ltc0_pka3_4;
#[doc = "LTC PKHA A 52 Register"]
pub struct LTC0_PKA_52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 52 Register"]
pub mod ltc0_pka_52;
#[doc = "LTC PKHA A3 5 Register"]
pub struct LTC0_PKA3_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 5 Register"]
pub mod ltc0_pka3_5;
#[doc = "LTC PKHA A 53 Register"]
pub struct LTC0_PKA_53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 53 Register"]
pub mod ltc0_pka_53;
#[doc = "LTC PKHA A3 6 Register"]
pub struct LTC0_PKA3_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 6 Register"]
pub mod ltc0_pka3_6;
#[doc = "LTC PKHA A 54 Register"]
pub struct LTC0_PKA_54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 54 Register"]
pub mod ltc0_pka_54;
#[doc = "LTC PKHA A3 7 Register"]
pub struct LTC0_PKA3_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 7 Register"]
pub mod ltc0_pka3_7;
#[doc = "LTC PKHA A 55 Register"]
pub struct LTC0_PKA_55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 55 Register"]
pub mod ltc0_pka_55;
#[doc = "LTC PKHA A3 8 Register"]
pub struct LTC0_PKA3_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 8 Register"]
pub mod ltc0_pka3_8;
#[doc = "LTC PKHA A 56 Register"]
pub struct LTC0_PKA_56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 56 Register"]
pub mod ltc0_pka_56;
#[doc = "LTC PKHA A3 9 Register"]
pub struct LTC0_PKA3_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 9 Register"]
pub mod ltc0_pka3_9;
#[doc = "LTC PKHA A 57 Register"]
pub struct LTC0_PKA_57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 57 Register"]
pub mod ltc0_pka_57;
#[doc = "LTC PKHA A3 10 Register"]
pub struct LTC0_PKA3_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 10 Register"]
pub mod ltc0_pka3_10;
#[doc = "LTC PKHA A 58 Register"]
pub struct LTC0_PKA_58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 58 Register"]
pub mod ltc0_pka_58;
#[doc = "LTC PKHA A3 11 Register"]
pub struct LTC0_PKA3_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 11 Register"]
pub mod ltc0_pka3_11;
#[doc = "LTC PKHA A 59 Register"]
pub struct LTC0_PKA_59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 59 Register"]
pub mod ltc0_pka_59;
#[doc = "LTC PKHA A3 12 Register"]
pub struct LTC0_PKA3_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 12 Register"]
pub mod ltc0_pka3_12;
#[doc = "LTC PKHA A 60 Register"]
pub struct LTC0_PKA_60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 60 Register"]
pub mod ltc0_pka_60;
#[doc = "LTC PKHA A3 13 Register"]
pub struct LTC0_PKA3_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 13 Register"]
pub mod ltc0_pka3_13;
#[doc = "LTC PKHA A 61 Register"]
pub struct LTC0_PKA_61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 61 Register"]
pub mod ltc0_pka_61;
#[doc = "LTC PKHA A3 14 Register"]
pub struct LTC0_PKA3_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 14 Register"]
pub mod ltc0_pka3_14;
#[doc = "LTC PKHA A 62 Register"]
pub struct LTC0_PKA_62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 62 Register"]
pub mod ltc0_pka_62;
#[doc = "LTC PKHA A3 15 Register"]
pub struct LTC0_PKA3_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A3 15 Register"]
pub mod ltc0_pka3_15;
#[doc = "LTC PKHA A 63 Register"]
pub struct LTC0_PKA_63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA A 63 Register"]
pub mod ltc0_pka_63;
#[doc = "LTC PKHA B0 0 Register"]
pub struct LTC0_PKB0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 0 Register"]
pub mod ltc0_pkb0_0;
#[doc = "LTC PKHA B 0 Register"]
pub struct LTC0_PKB_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 0 Register"]
pub mod ltc0_pkb_0;
#[doc = "LTC PKHA B0 1 Register"]
pub struct LTC0_PKB0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 1 Register"]
pub mod ltc0_pkb0_1;
#[doc = "LTC PKHA B 1 Register"]
pub struct LTC0_PKB_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 1 Register"]
pub mod ltc0_pkb_1;
#[doc = "LTC PKHA B0 2 Register"]
pub struct LTC0_PKB0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 2 Register"]
pub mod ltc0_pkb0_2;
#[doc = "LTC PKHA B 2 Register"]
pub struct LTC0_PKB_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 2 Register"]
pub mod ltc0_pkb_2;
#[doc = "LTC PKHA B0 3 Register"]
pub struct LTC0_PKB0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 3 Register"]
pub mod ltc0_pkb0_3;
#[doc = "LTC PKHA B 3 Register"]
pub struct LTC0_PKB_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 3 Register"]
pub mod ltc0_pkb_3;
#[doc = "LTC PKHA B0 4 Register"]
pub struct LTC0_PKB0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 4 Register"]
pub mod ltc0_pkb0_4;
#[doc = "LTC PKHA B 4 Register"]
pub struct LTC0_PKB_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 4 Register"]
pub mod ltc0_pkb_4;
#[doc = "LTC PKHA B0 5 Register"]
pub struct LTC0_PKB0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 5 Register"]
pub mod ltc0_pkb0_5;
#[doc = "LTC PKHA B 5 Register"]
pub struct LTC0_PKB_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 5 Register"]
pub mod ltc0_pkb_5;
#[doc = "LTC PKHA B0 6 Register"]
pub struct LTC0_PKB0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 6 Register"]
pub mod ltc0_pkb0_6;
#[doc = "LTC PKHA B 6 Register"]
pub struct LTC0_PKB_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 6 Register"]
pub mod ltc0_pkb_6;
#[doc = "LTC PKHA B0 7 Register"]
pub struct LTC0_PKB0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 7 Register"]
pub mod ltc0_pkb0_7;
#[doc = "LTC PKHA B 7 Register"]
pub struct LTC0_PKB_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 7 Register"]
pub mod ltc0_pkb_7;
#[doc = "LTC PKHA B0 8 Register"]
pub struct LTC0_PKB0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 8 Register"]
pub mod ltc0_pkb0_8;
#[doc = "LTC PKHA B 8 Register"]
pub struct LTC0_PKB_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 8 Register"]
pub mod ltc0_pkb_8;
#[doc = "LTC PKHA B0 9 Register"]
pub struct LTC0_PKB0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 9 Register"]
pub mod ltc0_pkb0_9;
#[doc = "LTC PKHA B 9 Register"]
pub struct LTC0_PKB_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 9 Register"]
pub mod ltc0_pkb_9;
#[doc = "LTC PKHA B0 10 Register"]
pub struct LTC0_PKB0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 10 Register"]
pub mod ltc0_pkb0_10;
#[doc = "LTC PKHA B 10 Register"]
pub struct LTC0_PKB_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 10 Register"]
pub mod ltc0_pkb_10;
#[doc = "LTC PKHA B0 11 Register"]
pub struct LTC0_PKB0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 11 Register"]
pub mod ltc0_pkb0_11;
#[doc = "LTC PKHA B 11 Register"]
pub struct LTC0_PKB_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 11 Register"]
pub mod ltc0_pkb_11;
#[doc = "LTC PKHA B0 12 Register"]
pub struct LTC0_PKB0_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 12 Register"]
pub mod ltc0_pkb0_12;
#[doc = "LTC PKHA B 12 Register"]
pub struct LTC0_PKB_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 12 Register"]
pub mod ltc0_pkb_12;
#[doc = "LTC PKHA B0 13 Register"]
pub struct LTC0_PKB0_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 13 Register"]
pub mod ltc0_pkb0_13;
#[doc = "LTC PKHA B 13 Register"]
pub struct LTC0_PKB_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 13 Register"]
pub mod ltc0_pkb_13;
#[doc = "LTC PKHA B0 14 Register"]
pub struct LTC0_PKB0_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 14 Register"]
pub mod ltc0_pkb0_14;
#[doc = "LTC PKHA B 14 Register"]
pub struct LTC0_PKB_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 14 Register"]
pub mod ltc0_pkb_14;
#[doc = "LTC PKHA B0 15 Register"]
pub struct LTC0_PKB0_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B0 15 Register"]
pub mod ltc0_pkb0_15;
#[doc = "LTC PKHA B 15 Register"]
pub struct LTC0_PKB_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 15 Register"]
pub mod ltc0_pkb_15;
#[doc = "LTC PKHA B1 0 Register"]
pub struct LTC0_PKB1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 0 Register"]
pub mod ltc0_pkb1_0;
#[doc = "LTC PKHA B 16 Register"]
pub struct LTC0_PKB_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 16 Register"]
pub mod ltc0_pkb_16;
#[doc = "LTC PKHA B1 1 Register"]
pub struct LTC0_PKB1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 1 Register"]
pub mod ltc0_pkb1_1;
#[doc = "LTC PKHA B 17 Register"]
pub struct LTC0_PKB_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 17 Register"]
pub mod ltc0_pkb_17;
#[doc = "LTC PKHA B1 2 Register"]
pub struct LTC0_PKB1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 2 Register"]
pub mod ltc0_pkb1_2;
#[doc = "LTC PKHA B 18 Register"]
pub struct LTC0_PKB_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 18 Register"]
pub mod ltc0_pkb_18;
#[doc = "LTC PKHA B1 3 Register"]
pub struct LTC0_PKB1_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 3 Register"]
pub mod ltc0_pkb1_3;
#[doc = "LTC PKHA B 19 Register"]
pub struct LTC0_PKB_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 19 Register"]
pub mod ltc0_pkb_19;
#[doc = "LTC PKHA B1 4 Register"]
pub struct LTC0_PKB1_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 4 Register"]
pub mod ltc0_pkb1_4;
#[doc = "LTC PKHA B 20 Register"]
pub struct LTC0_PKB_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 20 Register"]
pub mod ltc0_pkb_20;
#[doc = "LTC PKHA B1 5 Register"]
pub struct LTC0_PKB1_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 5 Register"]
pub mod ltc0_pkb1_5;
#[doc = "LTC PKHA B 21 Register"]
pub struct LTC0_PKB_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 21 Register"]
pub mod ltc0_pkb_21;
#[doc = "LTC PKHA B1 6 Register"]
pub struct LTC0_PKB1_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 6 Register"]
pub mod ltc0_pkb1_6;
#[doc = "LTC PKHA B 22 Register"]
pub struct LTC0_PKB_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 22 Register"]
pub mod ltc0_pkb_22;
#[doc = "LTC PKHA B1 7 Register"]
pub struct LTC0_PKB1_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 7 Register"]
pub mod ltc0_pkb1_7;
#[doc = "LTC PKHA B 23 Register"]
pub struct LTC0_PKB_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 23 Register"]
pub mod ltc0_pkb_23;
#[doc = "LTC PKHA B1 8 Register"]
pub struct LTC0_PKB1_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 8 Register"]
pub mod ltc0_pkb1_8;
#[doc = "LTC PKHA B 24 Register"]
pub struct LTC0_PKB_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 24 Register"]
pub mod ltc0_pkb_24;
#[doc = "LTC PKHA B1 9 Register"]
pub struct LTC0_PKB1_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 9 Register"]
pub mod ltc0_pkb1_9;
#[doc = "LTC PKHA B 25 Register"]
pub struct LTC0_PKB_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 25 Register"]
pub mod ltc0_pkb_25;
#[doc = "LTC PKHA B1 10 Register"]
pub struct LTC0_PKB1_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 10 Register"]
pub mod ltc0_pkb1_10;
#[doc = "LTC PKHA B 26 Register"]
pub struct LTC0_PKB_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 26 Register"]
pub mod ltc0_pkb_26;
#[doc = "LTC PKHA B1 11 Register"]
pub struct LTC0_PKB1_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 11 Register"]
pub mod ltc0_pkb1_11;
#[doc = "LTC PKHA B 27 Register"]
pub struct LTC0_PKB_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 27 Register"]
pub mod ltc0_pkb_27;
#[doc = "LTC PKHA B1 12 Register"]
pub struct LTC0_PKB1_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 12 Register"]
pub mod ltc0_pkb1_12;
#[doc = "LTC PKHA B 28 Register"]
pub struct LTC0_PKB_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 28 Register"]
pub mod ltc0_pkb_28;
#[doc = "LTC PKHA B1 13 Register"]
pub struct LTC0_PKB1_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 13 Register"]
pub mod ltc0_pkb1_13;
#[doc = "LTC PKHA B 29 Register"]
pub struct LTC0_PKB_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 29 Register"]
pub mod ltc0_pkb_29;
#[doc = "LTC PKHA B1 14 Register"]
pub struct LTC0_PKB1_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 14 Register"]
pub mod ltc0_pkb1_14;
#[doc = "LTC PKHA B 30 Register"]
pub struct LTC0_PKB_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 30 Register"]
pub mod ltc0_pkb_30;
#[doc = "LTC PKHA B1 15 Register"]
pub struct LTC0_PKB1_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B1 15 Register"]
pub mod ltc0_pkb1_15;
#[doc = "LTC PKHA B 31 Register"]
pub struct LTC0_PKB_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 31 Register"]
pub mod ltc0_pkb_31;
#[doc = "LTC PKHA B2 0 Register"]
pub struct LTC0_PKB2_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 0 Register"]
pub mod ltc0_pkb2_0;
#[doc = "LTC PKHA B 32 Register"]
pub struct LTC0_PKB_32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 32 Register"]
pub mod ltc0_pkb_32;
#[doc = "LTC PKHA B2 1 Register"]
pub struct LTC0_PKB2_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 1 Register"]
pub mod ltc0_pkb2_1;
#[doc = "LTC PKHA B 33 Register"]
pub struct LTC0_PKB_33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 33 Register"]
pub mod ltc0_pkb_33;
#[doc = "LTC PKHA B2 2 Register"]
pub struct LTC0_PKB2_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 2 Register"]
pub mod ltc0_pkb2_2;
#[doc = "LTC PKHA B 34 Register"]
pub struct LTC0_PKB_34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 34 Register"]
pub mod ltc0_pkb_34;
#[doc = "LTC PKHA B2 3 Register"]
pub struct LTC0_PKB2_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 3 Register"]
pub mod ltc0_pkb2_3;
#[doc = "LTC PKHA B 35 Register"]
pub struct LTC0_PKB_35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 35 Register"]
pub mod ltc0_pkb_35;
#[doc = "LTC PKHA B2 4 Register"]
pub struct LTC0_PKB2_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 4 Register"]
pub mod ltc0_pkb2_4;
#[doc = "LTC PKHA B 36 Register"]
pub struct LTC0_PKB_36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 36 Register"]
pub mod ltc0_pkb_36;
#[doc = "LTC PKHA B2 5 Register"]
pub struct LTC0_PKB2_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 5 Register"]
pub mod ltc0_pkb2_5;
#[doc = "LTC PKHA B 37 Register"]
pub struct LTC0_PKB_37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 37 Register"]
pub mod ltc0_pkb_37;
#[doc = "LTC PKHA B2 6 Register"]
pub struct LTC0_PKB2_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 6 Register"]
pub mod ltc0_pkb2_6;
#[doc = "LTC PKHA B 38 Register"]
pub struct LTC0_PKB_38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 38 Register"]
pub mod ltc0_pkb_38;
#[doc = "LTC PKHA B2 7 Register"]
pub struct LTC0_PKB2_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 7 Register"]
pub mod ltc0_pkb2_7;
#[doc = "LTC PKHA B 39 Register"]
pub struct LTC0_PKB_39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 39 Register"]
pub mod ltc0_pkb_39;
#[doc = "LTC PKHA B2 8 Register"]
pub struct LTC0_PKB2_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 8 Register"]
pub mod ltc0_pkb2_8;
#[doc = "LTC PKHA B 40 Register"]
pub struct LTC0_PKB_40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 40 Register"]
pub mod ltc0_pkb_40;
#[doc = "LTC PKHA B2 9 Register"]
pub struct LTC0_PKB2_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 9 Register"]
pub mod ltc0_pkb2_9;
#[doc = "LTC PKHA B 41 Register"]
pub struct LTC0_PKB_41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 41 Register"]
pub mod ltc0_pkb_41;
#[doc = "LTC PKHA B2 10 Register"]
pub struct LTC0_PKB2_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 10 Register"]
pub mod ltc0_pkb2_10;
#[doc = "LTC PKHA B 42 Register"]
pub struct LTC0_PKB_42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 42 Register"]
pub mod ltc0_pkb_42;
#[doc = "LTC PKHA B2 11 Register"]
pub struct LTC0_PKB2_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 11 Register"]
pub mod ltc0_pkb2_11;
#[doc = "LTC PKHA B 43 Register"]
pub struct LTC0_PKB_43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 43 Register"]
pub mod ltc0_pkb_43;
#[doc = "LTC PKHA B2 12 Register"]
pub struct LTC0_PKB2_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 12 Register"]
pub mod ltc0_pkb2_12;
#[doc = "LTC PKHA B 44 Register"]
pub struct LTC0_PKB_44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 44 Register"]
pub mod ltc0_pkb_44;
#[doc = "LTC PKHA B2 13 Register"]
pub struct LTC0_PKB2_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 13 Register"]
pub mod ltc0_pkb2_13;
#[doc = "LTC PKHA B 45 Register"]
pub struct LTC0_PKB_45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 45 Register"]
pub mod ltc0_pkb_45;
#[doc = "LTC PKHA B2 14 Register"]
pub struct LTC0_PKB2_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 14 Register"]
pub mod ltc0_pkb2_14;
#[doc = "LTC PKHA B 46 Register"]
pub struct LTC0_PKB_46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 46 Register"]
pub mod ltc0_pkb_46;
#[doc = "LTC PKHA B2 15 Register"]
pub struct LTC0_PKB2_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B2 15 Register"]
pub mod ltc0_pkb2_15;
#[doc = "LTC PKHA B 47 Register"]
pub struct LTC0_PKB_47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 47 Register"]
pub mod ltc0_pkb_47;
#[doc = "LTC PKHA B3 0 Register"]
pub struct LTC0_PKB3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 0 Register"]
pub mod ltc0_pkb3_0;
#[doc = "LTC PKHA B 48 Register"]
pub struct LTC0_PKB_48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 48 Register"]
pub mod ltc0_pkb_48;
#[doc = "LTC PKHA B3 1 Register"]
pub struct LTC0_PKB3_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 1 Register"]
pub mod ltc0_pkb3_1;
#[doc = "LTC PKHA B 49 Register"]
pub struct LTC0_PKB_49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 49 Register"]
pub mod ltc0_pkb_49;
#[doc = "LTC PKHA B3 2 Register"]
pub struct LTC0_PKB3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 2 Register"]
pub mod ltc0_pkb3_2;
#[doc = "LTC PKHA B 50 Register"]
pub struct LTC0_PKB_50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 50 Register"]
pub mod ltc0_pkb_50;
#[doc = "LTC PKHA B3 3 Register"]
pub struct LTC0_PKB3_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 3 Register"]
pub mod ltc0_pkb3_3;
#[doc = "LTC PKHA B 51 Register"]
pub struct LTC0_PKB_51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 51 Register"]
pub mod ltc0_pkb_51;
#[doc = "LTC PKHA B3 4 Register"]
pub struct LTC0_PKB3_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 4 Register"]
pub mod ltc0_pkb3_4;
#[doc = "LTC PKHA B 52 Register"]
pub struct LTC0_PKB_52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 52 Register"]
pub mod ltc0_pkb_52;
#[doc = "LTC PKHA B3 5 Register"]
pub struct LTC0_PKB3_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 5 Register"]
pub mod ltc0_pkb3_5;
#[doc = "LTC PKHA B 53 Register"]
pub struct LTC0_PKB_53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 53 Register"]
pub mod ltc0_pkb_53;
#[doc = "LTC PKHA B3 6 Register"]
pub struct LTC0_PKB3_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 6 Register"]
pub mod ltc0_pkb3_6;
#[doc = "LTC PKHA B 54 Register"]
pub struct LTC0_PKB_54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 54 Register"]
pub mod ltc0_pkb_54;
#[doc = "LTC PKHA B3 7 Register"]
pub struct LTC0_PKB3_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 7 Register"]
pub mod ltc0_pkb3_7;
#[doc = "LTC PKHA B 55 Register"]
pub struct LTC0_PKB_55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 55 Register"]
pub mod ltc0_pkb_55;
#[doc = "LTC PKHA B3 8 Register"]
pub struct LTC0_PKB3_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 8 Register"]
pub mod ltc0_pkb3_8;
#[doc = "LTC PKHA B 56 Register"]
pub struct LTC0_PKB_56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 56 Register"]
pub mod ltc0_pkb_56;
#[doc = "LTC PKHA B3 9 Register"]
pub struct LTC0_PKB3_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 9 Register"]
pub mod ltc0_pkb3_9;
#[doc = "LTC PKHA B 57 Register"]
pub struct LTC0_PKB_57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 57 Register"]
pub mod ltc0_pkb_57;
#[doc = "LTC PKHA B3 10 Register"]
pub struct LTC0_PKB3_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 10 Register"]
pub mod ltc0_pkb3_10;
#[doc = "LTC PKHA B 58 Register"]
pub struct LTC0_PKB_58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 58 Register"]
pub mod ltc0_pkb_58;
#[doc = "LTC PKHA B3 11 Register"]
pub struct LTC0_PKB3_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 11 Register"]
pub mod ltc0_pkb3_11;
#[doc = "LTC PKHA B 59 Register"]
pub struct LTC0_PKB_59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 59 Register"]
pub mod ltc0_pkb_59;
#[doc = "LTC PKHA B3 12 Register"]
pub struct LTC0_PKB3_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 12 Register"]
pub mod ltc0_pkb3_12;
#[doc = "LTC PKHA B 60 Register"]
pub struct LTC0_PKB_60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 60 Register"]
pub mod ltc0_pkb_60;
#[doc = "LTC PKHA B3 13 Register"]
pub struct LTC0_PKB3_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 13 Register"]
pub mod ltc0_pkb3_13;
#[doc = "LTC PKHA B 61 Register"]
pub struct LTC0_PKB_61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 61 Register"]
pub mod ltc0_pkb_61;
#[doc = "LTC PKHA B3 14 Register"]
pub struct LTC0_PKB3_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 14 Register"]
pub mod ltc0_pkb3_14;
#[doc = "LTC PKHA B 62 Register"]
pub struct LTC0_PKB_62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 62 Register"]
pub mod ltc0_pkb_62;
#[doc = "LTC PKHA B3 15 Register"]
pub struct LTC0_PKB3_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B3 15 Register"]
pub mod ltc0_pkb3_15;
#[doc = "LTC PKHA B 63 Register"]
pub struct LTC0_PKB_63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA B 63 Register"]
pub mod ltc0_pkb_63;
#[doc = "LTC PKHA N0 0 Register"]
pub struct LTC0_PKN0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 0 Register"]
pub mod ltc0_pkn0_0;
#[doc = "LTC PKHA N 0 Register"]
pub struct LTC0_PKN_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 0 Register"]
pub mod ltc0_pkn_0;
#[doc = "LTC PKHA N0 1 Register"]
pub struct LTC0_PKN0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 1 Register"]
pub mod ltc0_pkn0_1;
#[doc = "LTC PKHA N 1 Register"]
pub struct LTC0_PKN_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 1 Register"]
pub mod ltc0_pkn_1;
#[doc = "LTC PKHA N0 2 Register"]
pub struct LTC0_PKN0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 2 Register"]
pub mod ltc0_pkn0_2;
#[doc = "LTC PKHA N 2 Register"]
pub struct LTC0_PKN_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 2 Register"]
pub mod ltc0_pkn_2;
#[doc = "LTC PKHA N0 3 Register"]
pub struct LTC0_PKN0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 3 Register"]
pub mod ltc0_pkn0_3;
#[doc = "LTC PKHA N 3 Register"]
pub struct LTC0_PKN_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 3 Register"]
pub mod ltc0_pkn_3;
#[doc = "LTC PKHA N0 4 Register"]
pub struct LTC0_PKN0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 4 Register"]
pub mod ltc0_pkn0_4;
#[doc = "LTC PKHA N 4 Register"]
pub struct LTC0_PKN_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 4 Register"]
pub mod ltc0_pkn_4;
#[doc = "LTC PKHA N0 5 Register"]
pub struct LTC0_PKN0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 5 Register"]
pub mod ltc0_pkn0_5;
#[doc = "LTC PKHA N 5 Register"]
pub struct LTC0_PKN_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 5 Register"]
pub mod ltc0_pkn_5;
#[doc = "LTC PKHA N0 6 Register"]
pub struct LTC0_PKN0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 6 Register"]
pub mod ltc0_pkn0_6;
#[doc = "LTC PKHA N 6 Register"]
pub struct LTC0_PKN_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 6 Register"]
pub mod ltc0_pkn_6;
#[doc = "LTC PKHA N0 7 Register"]
pub struct LTC0_PKN0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 7 Register"]
pub mod ltc0_pkn0_7;
#[doc = "LTC PKHA N 7 Register"]
pub struct LTC0_PKN_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 7 Register"]
pub mod ltc0_pkn_7;
#[doc = "LTC PKHA N0 8 Register"]
pub struct LTC0_PKN0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 8 Register"]
pub mod ltc0_pkn0_8;
#[doc = "LTC PKHA N 8 Register"]
pub struct LTC0_PKN_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 8 Register"]
pub mod ltc0_pkn_8;
#[doc = "LTC PKHA N0 9 Register"]
pub struct LTC0_PKN0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 9 Register"]
pub mod ltc0_pkn0_9;
#[doc = "LTC PKHA N 9 Register"]
pub struct LTC0_PKN_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 9 Register"]
pub mod ltc0_pkn_9;
#[doc = "LTC PKHA N0 10 Register"]
pub struct LTC0_PKN0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 10 Register"]
pub mod ltc0_pkn0_10;
#[doc = "LTC PKHA N 10 Register"]
pub struct LTC0_PKN_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 10 Register"]
pub mod ltc0_pkn_10;
#[doc = "LTC PKHA N0 11 Register"]
pub struct LTC0_PKN0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 11 Register"]
pub mod ltc0_pkn0_11;
#[doc = "LTC PKHA N 11 Register"]
pub struct LTC0_PKN_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 11 Register"]
pub mod ltc0_pkn_11;
#[doc = "LTC PKHA N0 12 Register"]
pub struct LTC0_PKN0_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 12 Register"]
pub mod ltc0_pkn0_12;
#[doc = "LTC PKHA N 12 Register"]
pub struct LTC0_PKN_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 12 Register"]
pub mod ltc0_pkn_12;
#[doc = "LTC PKHA N0 13 Register"]
pub struct LTC0_PKN0_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 13 Register"]
pub mod ltc0_pkn0_13;
#[doc = "LTC PKHA N 13 Register"]
pub struct LTC0_PKN_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 13 Register"]
pub mod ltc0_pkn_13;
#[doc = "LTC PKHA N0 14 Register"]
pub struct LTC0_PKN0_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 14 Register"]
pub mod ltc0_pkn0_14;
#[doc = "LTC PKHA N 14 Register"]
pub struct LTC0_PKN_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 14 Register"]
pub mod ltc0_pkn_14;
#[doc = "LTC PKHA N0 15 Register"]
pub struct LTC0_PKN0_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N0 15 Register"]
pub mod ltc0_pkn0_15;
#[doc = "LTC PKHA N 15 Register"]
pub struct LTC0_PKN_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 15 Register"]
pub mod ltc0_pkn_15;
#[doc = "LTC PKHA N1 0 Register"]
pub struct LTC0_PKN1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 0 Register"]
pub mod ltc0_pkn1_0;
#[doc = "LTC PKHA N 16 Register"]
pub struct LTC0_PKN_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 16 Register"]
pub mod ltc0_pkn_16;
#[doc = "LTC PKHA N1 1 Register"]
pub struct LTC0_PKN1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 1 Register"]
pub mod ltc0_pkn1_1;
#[doc = "LTC PKHA N 17 Register"]
pub struct LTC0_PKN_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 17 Register"]
pub mod ltc0_pkn_17;
#[doc = "LTC PKHA N1 2 Register"]
pub struct LTC0_PKN1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 2 Register"]
pub mod ltc0_pkn1_2;
#[doc = "LTC PKHA N 18 Register"]
pub struct LTC0_PKN_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 18 Register"]
pub mod ltc0_pkn_18;
#[doc = "LTC PKHA N1 3 Register"]
pub struct LTC0_PKN1_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 3 Register"]
pub mod ltc0_pkn1_3;
#[doc = "LTC PKHA N 19 Register"]
pub struct LTC0_PKN_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 19 Register"]
pub mod ltc0_pkn_19;
#[doc = "LTC PKHA N1 4 Register"]
pub struct LTC0_PKN1_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 4 Register"]
pub mod ltc0_pkn1_4;
#[doc = "LTC PKHA N 20 Register"]
pub struct LTC0_PKN_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 20 Register"]
pub mod ltc0_pkn_20;
#[doc = "LTC PKHA N1 5 Register"]
pub struct LTC0_PKN1_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 5 Register"]
pub mod ltc0_pkn1_5;
#[doc = "LTC PKHA N 21 Register"]
pub struct LTC0_PKN_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 21 Register"]
pub mod ltc0_pkn_21;
#[doc = "LTC PKHA N1 6 Register"]
pub struct LTC0_PKN1_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 6 Register"]
pub mod ltc0_pkn1_6;
#[doc = "LTC PKHA N 22 Register"]
pub struct LTC0_PKN_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 22 Register"]
pub mod ltc0_pkn_22;
#[doc = "LTC PKHA N1 7 Register"]
pub struct LTC0_PKN1_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 7 Register"]
pub mod ltc0_pkn1_7;
#[doc = "LTC PKHA N 23 Register"]
pub struct LTC0_PKN_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 23 Register"]
pub mod ltc0_pkn_23;
#[doc = "LTC PKHA N1 8 Register"]
pub struct LTC0_PKN1_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 8 Register"]
pub mod ltc0_pkn1_8;
#[doc = "LTC PKHA N 24 Register"]
pub struct LTC0_PKN_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 24 Register"]
pub mod ltc0_pkn_24;
#[doc = "LTC PKHA N1 9 Register"]
pub struct LTC0_PKN1_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 9 Register"]
pub mod ltc0_pkn1_9;
#[doc = "LTC PKHA N 25 Register"]
pub struct LTC0_PKN_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 25 Register"]
pub mod ltc0_pkn_25;
#[doc = "LTC PKHA N1 10 Register"]
pub struct LTC0_PKN1_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 10 Register"]
pub mod ltc0_pkn1_10;
#[doc = "LTC PKHA N 26 Register"]
pub struct LTC0_PKN_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 26 Register"]
pub mod ltc0_pkn_26;
#[doc = "LTC PKHA N1 11 Register"]
pub struct LTC0_PKN1_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 11 Register"]
pub mod ltc0_pkn1_11;
#[doc = "LTC PKHA N 27 Register"]
pub struct LTC0_PKN_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 27 Register"]
pub mod ltc0_pkn_27;
#[doc = "LTC PKHA N1 12 Register"]
pub struct LTC0_PKN1_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 12 Register"]
pub mod ltc0_pkn1_12;
#[doc = "LTC PKHA N 28 Register"]
pub struct LTC0_PKN_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 28 Register"]
pub mod ltc0_pkn_28;
#[doc = "LTC PKHA N1 13 Register"]
pub struct LTC0_PKN1_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 13 Register"]
pub mod ltc0_pkn1_13;
#[doc = "LTC PKHA N 29 Register"]
pub struct LTC0_PKN_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 29 Register"]
pub mod ltc0_pkn_29;
#[doc = "LTC PKHA N1 14 Register"]
pub struct LTC0_PKN1_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 14 Register"]
pub mod ltc0_pkn1_14;
#[doc = "LTC PKHA N 30 Register"]
pub struct LTC0_PKN_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 30 Register"]
pub mod ltc0_pkn_30;
#[doc = "LTC PKHA N1 15 Register"]
pub struct LTC0_PKN1_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N1 15 Register"]
pub mod ltc0_pkn1_15;
#[doc = "LTC PKHA N 31 Register"]
pub struct LTC0_PKN_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 31 Register"]
pub mod ltc0_pkn_31;
#[doc = "LTC PKHA N2 0 Register"]
pub struct LTC0_PKN2_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 0 Register"]
pub mod ltc0_pkn2_0;
#[doc = "LTC PKHA N 32 Register"]
pub struct LTC0_PKN_32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 32 Register"]
pub mod ltc0_pkn_32;
#[doc = "LTC PKHA N2 1 Register"]
pub struct LTC0_PKN2_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 1 Register"]
pub mod ltc0_pkn2_1;
#[doc = "LTC PKHA N 33 Register"]
pub struct LTC0_PKN_33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 33 Register"]
pub mod ltc0_pkn_33;
#[doc = "LTC PKHA N2 2 Register"]
pub struct LTC0_PKN2_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 2 Register"]
pub mod ltc0_pkn2_2;
#[doc = "LTC PKHA N 34 Register"]
pub struct LTC0_PKN_34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 34 Register"]
pub mod ltc0_pkn_34;
#[doc = "LTC PKHA N2 3 Register"]
pub struct LTC0_PKN2_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 3 Register"]
pub mod ltc0_pkn2_3;
#[doc = "LTC PKHA N 35 Register"]
pub struct LTC0_PKN_35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 35 Register"]
pub mod ltc0_pkn_35;
#[doc = "LTC PKHA N2 4 Register"]
pub struct LTC0_PKN2_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 4 Register"]
pub mod ltc0_pkn2_4;
#[doc = "LTC PKHA N 36 Register"]
pub struct LTC0_PKN_36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 36 Register"]
pub mod ltc0_pkn_36;
#[doc = "LTC PKHA N2 5 Register"]
pub struct LTC0_PKN2_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 5 Register"]
pub mod ltc0_pkn2_5;
#[doc = "LTC PKHA N 37 Register"]
pub struct LTC0_PKN_37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 37 Register"]
pub mod ltc0_pkn_37;
#[doc = "LTC PKHA N2 6 Register"]
pub struct LTC0_PKN2_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 6 Register"]
pub mod ltc0_pkn2_6;
#[doc = "LTC PKHA N 38 Register"]
pub struct LTC0_PKN_38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 38 Register"]
pub mod ltc0_pkn_38;
#[doc = "LTC PKHA N2 7 Register"]
pub struct LTC0_PKN2_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 7 Register"]
pub mod ltc0_pkn2_7;
#[doc = "LTC PKHA N 39 Register"]
pub struct LTC0_PKN_39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 39 Register"]
pub mod ltc0_pkn_39;
#[doc = "LTC PKHA N2 8 Register"]
pub struct LTC0_PKN2_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 8 Register"]
pub mod ltc0_pkn2_8;
#[doc = "LTC PKHA N 40 Register"]
pub struct LTC0_PKN_40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 40 Register"]
pub mod ltc0_pkn_40;
#[doc = "LTC PKHA N2 9 Register"]
pub struct LTC0_PKN2_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 9 Register"]
pub mod ltc0_pkn2_9;
#[doc = "LTC PKHA N 41 Register"]
pub struct LTC0_PKN_41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 41 Register"]
pub mod ltc0_pkn_41;
#[doc = "LTC PKHA N2 10 Register"]
pub struct LTC0_PKN2_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 10 Register"]
pub mod ltc0_pkn2_10;
#[doc = "LTC PKHA N 42 Register"]
pub struct LTC0_PKN_42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 42 Register"]
pub mod ltc0_pkn_42;
#[doc = "LTC PKHA N2 11 Register"]
pub struct LTC0_PKN2_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 11 Register"]
pub mod ltc0_pkn2_11;
#[doc = "LTC PKHA N 43 Register"]
pub struct LTC0_PKN_43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 43 Register"]
pub mod ltc0_pkn_43;
#[doc = "LTC PKHA N2 12 Register"]
pub struct LTC0_PKN2_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 12 Register"]
pub mod ltc0_pkn2_12;
#[doc = "LTC PKHA N 44 Register"]
pub struct LTC0_PKN_44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 44 Register"]
pub mod ltc0_pkn_44;
#[doc = "LTC PKHA N2 13 Register"]
pub struct LTC0_PKN2_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 13 Register"]
pub mod ltc0_pkn2_13;
#[doc = "LTC PKHA N 45 Register"]
pub struct LTC0_PKN_45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 45 Register"]
pub mod ltc0_pkn_45;
#[doc = "LTC PKHA N2 14 Register"]
pub struct LTC0_PKN2_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 14 Register"]
pub mod ltc0_pkn2_14;
#[doc = "LTC PKHA N 46 Register"]
pub struct LTC0_PKN_46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 46 Register"]
pub mod ltc0_pkn_46;
#[doc = "LTC PKHA N2 15 Register"]
pub struct LTC0_PKN2_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N2 15 Register"]
pub mod ltc0_pkn2_15;
#[doc = "LTC PKHA N 47 Register"]
pub struct LTC0_PKN_47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 47 Register"]
pub mod ltc0_pkn_47;
#[doc = "LTC PKHA N3 0 Register"]
pub struct LTC0_PKN3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 0 Register"]
pub mod ltc0_pkn3_0;
#[doc = "LTC PKHA N 48 Register"]
pub struct LTC0_PKN_48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 48 Register"]
pub mod ltc0_pkn_48;
#[doc = "LTC PKHA N3 1 Register"]
pub struct LTC0_PKN3_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 1 Register"]
pub mod ltc0_pkn3_1;
#[doc = "LTC PKHA N 49 Register"]
pub struct LTC0_PKN_49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 49 Register"]
pub mod ltc0_pkn_49;
#[doc = "LTC PKHA N3 2 Register"]
pub struct LTC0_PKN3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 2 Register"]
pub mod ltc0_pkn3_2;
#[doc = "LTC PKHA N 50 Register"]
pub struct LTC0_PKN_50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 50 Register"]
pub mod ltc0_pkn_50;
#[doc = "LTC PKHA N3 3 Register"]
pub struct LTC0_PKN3_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 3 Register"]
pub mod ltc0_pkn3_3;
#[doc = "LTC PKHA N 51 Register"]
pub struct LTC0_PKN_51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 51 Register"]
pub mod ltc0_pkn_51;
#[doc = "LTC PKHA N3 4 Register"]
pub struct LTC0_PKN3_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 4 Register"]
pub mod ltc0_pkn3_4;
#[doc = "LTC PKHA N 52 Register"]
pub struct LTC0_PKN_52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 52 Register"]
pub mod ltc0_pkn_52;
#[doc = "LTC PKHA N3 5 Register"]
pub struct LTC0_PKN3_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 5 Register"]
pub mod ltc0_pkn3_5;
#[doc = "LTC PKHA N 53 Register"]
pub struct LTC0_PKN_53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 53 Register"]
pub mod ltc0_pkn_53;
#[doc = "LTC PKHA N3 6 Register"]
pub struct LTC0_PKN3_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 6 Register"]
pub mod ltc0_pkn3_6;
#[doc = "LTC PKHA N 54 Register"]
pub struct LTC0_PKN_54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 54 Register"]
pub mod ltc0_pkn_54;
#[doc = "LTC PKHA N3 7 Register"]
pub struct LTC0_PKN3_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 7 Register"]
pub mod ltc0_pkn3_7;
#[doc = "LTC PKHA N 55 Register"]
pub struct LTC0_PKN_55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 55 Register"]
pub mod ltc0_pkn_55;
#[doc = "LTC PKHA N3 8 Register"]
pub struct LTC0_PKN3_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 8 Register"]
pub mod ltc0_pkn3_8;
#[doc = "LTC PKHA N 56 Register"]
pub struct LTC0_PKN_56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 56 Register"]
pub mod ltc0_pkn_56;
#[doc = "LTC PKHA N3 9 Register"]
pub struct LTC0_PKN3_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 9 Register"]
pub mod ltc0_pkn3_9;
#[doc = "LTC PKHA N 57 Register"]
pub struct LTC0_PKN_57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 57 Register"]
pub mod ltc0_pkn_57;
#[doc = "LTC PKHA N3 10 Register"]
pub struct LTC0_PKN3_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 10 Register"]
pub mod ltc0_pkn3_10;
#[doc = "LTC PKHA N 58 Register"]
pub struct LTC0_PKN_58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 58 Register"]
pub mod ltc0_pkn_58;
#[doc = "LTC PKHA N3 11 Register"]
pub struct LTC0_PKN3_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 11 Register"]
pub mod ltc0_pkn3_11;
#[doc = "LTC PKHA N 59 Register"]
pub struct LTC0_PKN_59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 59 Register"]
pub mod ltc0_pkn_59;
#[doc = "LTC PKHA N3 12 Register"]
pub struct LTC0_PKN3_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 12 Register"]
pub mod ltc0_pkn3_12;
#[doc = "LTC PKHA N 60 Register"]
pub struct LTC0_PKN_60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 60 Register"]
pub mod ltc0_pkn_60;
#[doc = "LTC PKHA N3 13 Register"]
pub struct LTC0_PKN3_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 13 Register"]
pub mod ltc0_pkn3_13;
#[doc = "LTC PKHA N 61 Register"]
pub struct LTC0_PKN_61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 61 Register"]
pub mod ltc0_pkn_61;
#[doc = "LTC PKHA N3 14 Register"]
pub struct LTC0_PKN3_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 14 Register"]
pub mod ltc0_pkn3_14;
#[doc = "LTC PKHA N 62 Register"]
pub struct LTC0_PKN_62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 62 Register"]
pub mod ltc0_pkn_62;
#[doc = "LTC PKHA N3 15 Register"]
pub struct LTC0_PKN3_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N3 15 Register"]
pub mod ltc0_pkn3_15;
#[doc = "LTC PKHA N 63 Register"]
pub struct LTC0_PKN_63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA N 63 Register"]
pub mod ltc0_pkn_63;
#[doc = "LTC PKHA E0 0 Register"]
pub struct LTC0_PKE0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 0 Register"]
pub mod ltc0_pke0_0;
#[doc = "LTC PKHA E 0 Register"]
pub struct LTC0_PKE_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 0 Register"]
pub mod ltc0_pke_0;
#[doc = "LTC PKHA E0 1 Register"]
pub struct LTC0_PKE0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 1 Register"]
pub mod ltc0_pke0_1;
#[doc = "LTC PKHA E 1 Register"]
pub struct LTC0_PKE_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 1 Register"]
pub mod ltc0_pke_1;
#[doc = "LTC PKHA E0 2 Register"]
pub struct LTC0_PKE0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 2 Register"]
pub mod ltc0_pke0_2;
#[doc = "LTC PKHA E 2 Register"]
pub struct LTC0_PKE_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 2 Register"]
pub mod ltc0_pke_2;
#[doc = "LTC PKHA E0 3 Register"]
pub struct LTC0_PKE0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 3 Register"]
pub mod ltc0_pke0_3;
#[doc = "LTC PKHA E 3 Register"]
pub struct LTC0_PKE_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 3 Register"]
pub mod ltc0_pke_3;
#[doc = "LTC PKHA E0 4 Register"]
pub struct LTC0_PKE0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 4 Register"]
pub mod ltc0_pke0_4;
#[doc = "LTC PKHA E 4 Register"]
pub struct LTC0_PKE_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 4 Register"]
pub mod ltc0_pke_4;
#[doc = "LTC PKHA E0 5 Register"]
pub struct LTC0_PKE0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 5 Register"]
pub mod ltc0_pke0_5;
#[doc = "LTC PKHA E 5 Register"]
pub struct LTC0_PKE_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 5 Register"]
pub mod ltc0_pke_5;
#[doc = "LTC PKHA E0 6 Register"]
pub struct LTC0_PKE0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 6 Register"]
pub mod ltc0_pke0_6;
#[doc = "LTC PKHA E 6 Register"]
pub struct LTC0_PKE_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 6 Register"]
pub mod ltc0_pke_6;
#[doc = "LTC PKHA E0 7 Register"]
pub struct LTC0_PKE0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 7 Register"]
pub mod ltc0_pke0_7;
#[doc = "LTC PKHA E 7 Register"]
pub struct LTC0_PKE_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 7 Register"]
pub mod ltc0_pke_7;
#[doc = "LTC PKHA E0 8 Register"]
pub struct LTC0_PKE0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 8 Register"]
pub mod ltc0_pke0_8;
#[doc = "LTC PKHA E 8 Register"]
pub struct LTC0_PKE_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 8 Register"]
pub mod ltc0_pke_8;
#[doc = "LTC PKHA E0 9 Register"]
pub struct LTC0_PKE0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 9 Register"]
pub mod ltc0_pke0_9;
#[doc = "LTC PKHA E 9 Register"]
pub struct LTC0_PKE_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 9 Register"]
pub mod ltc0_pke_9;
#[doc = "LTC PKHA E0 10 Register"]
pub struct LTC0_PKE0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 10 Register"]
pub mod ltc0_pke0_10;
#[doc = "LTC PKHA E 10 Register"]
pub struct LTC0_PKE_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 10 Register"]
pub mod ltc0_pke_10;
#[doc = "LTC PKHA E0 11 Register"]
pub struct LTC0_PKE0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 11 Register"]
pub mod ltc0_pke0_11;
#[doc = "LTC PKHA E 11 Register"]
pub struct LTC0_PKE_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 11 Register"]
pub mod ltc0_pke_11;
#[doc = "LTC PKHA E0 12 Register"]
pub struct LTC0_PKE0_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 12 Register"]
pub mod ltc0_pke0_12;
#[doc = "LTC PKHA E 12 Register"]
pub struct LTC0_PKE_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 12 Register"]
pub mod ltc0_pke_12;
#[doc = "LTC PKHA E0 13 Register"]
pub struct LTC0_PKE0_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 13 Register"]
pub mod ltc0_pke0_13;
#[doc = "LTC PKHA E 13 Register"]
pub struct LTC0_PKE_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 13 Register"]
pub mod ltc0_pke_13;
#[doc = "LTC PKHA E0 14 Register"]
pub struct LTC0_PKE0_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 14 Register"]
pub mod ltc0_pke0_14;
#[doc = "LTC PKHA E 14 Register"]
pub struct LTC0_PKE_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 14 Register"]
pub mod ltc0_pke_14;
#[doc = "LTC PKHA E0 15 Register"]
pub struct LTC0_PKE0_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E0 15 Register"]
pub mod ltc0_pke0_15;
#[doc = "LTC PKHA E 15 Register"]
pub struct LTC0_PKE_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 15 Register"]
pub mod ltc0_pke_15;
#[doc = "LTC PKHA E1 0 Register"]
pub struct LTC0_PKE1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 0 Register"]
pub mod ltc0_pke1_0;
#[doc = "LTC PKHA E 16 Register"]
pub struct LTC0_PKE_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 16 Register"]
pub mod ltc0_pke_16;
#[doc = "LTC PKHA E1 1 Register"]
pub struct LTC0_PKE1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 1 Register"]
pub mod ltc0_pke1_1;
#[doc = "LTC PKHA E 17 Register"]
pub struct LTC0_PKE_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 17 Register"]
pub mod ltc0_pke_17;
#[doc = "LTC PKHA E1 2 Register"]
pub struct LTC0_PKE1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 2 Register"]
pub mod ltc0_pke1_2;
#[doc = "LTC PKHA E 18 Register"]
pub struct LTC0_PKE_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 18 Register"]
pub mod ltc0_pke_18;
#[doc = "LTC PKHA E1 3 Register"]
pub struct LTC0_PKE1_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 3 Register"]
pub mod ltc0_pke1_3;
#[doc = "LTC PKHA E 19 Register"]
pub struct LTC0_PKE_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 19 Register"]
pub mod ltc0_pke_19;
#[doc = "LTC PKHA E1 4 Register"]
pub struct LTC0_PKE1_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 4 Register"]
pub mod ltc0_pke1_4;
#[doc = "LTC PKHA E 20 Register"]
pub struct LTC0_PKE_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 20 Register"]
pub mod ltc0_pke_20;
#[doc = "LTC PKHA E1 5 Register"]
pub struct LTC0_PKE1_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 5 Register"]
pub mod ltc0_pke1_5;
#[doc = "LTC PKHA E 21 Register"]
pub struct LTC0_PKE_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 21 Register"]
pub mod ltc0_pke_21;
#[doc = "LTC PKHA E1 6 Register"]
pub struct LTC0_PKE1_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 6 Register"]
pub mod ltc0_pke1_6;
#[doc = "LTC PKHA E 22 Register"]
pub struct LTC0_PKE_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 22 Register"]
pub mod ltc0_pke_22;
#[doc = "LTC PKHA E1 7 Register"]
pub struct LTC0_PKE1_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 7 Register"]
pub mod ltc0_pke1_7;
#[doc = "LTC PKHA E 23 Register"]
pub struct LTC0_PKE_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 23 Register"]
pub mod ltc0_pke_23;
#[doc = "LTC PKHA E1 8 Register"]
pub struct LTC0_PKE1_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 8 Register"]
pub mod ltc0_pke1_8;
#[doc = "LTC PKHA E 24 Register"]
pub struct LTC0_PKE_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 24 Register"]
pub mod ltc0_pke_24;
#[doc = "LTC PKHA E1 9 Register"]
pub struct LTC0_PKE1_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 9 Register"]
pub mod ltc0_pke1_9;
#[doc = "LTC PKHA E 25 Register"]
pub struct LTC0_PKE_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 25 Register"]
pub mod ltc0_pke_25;
#[doc = "LTC PKHA E1 10 Register"]
pub struct LTC0_PKE1_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 10 Register"]
pub mod ltc0_pke1_10;
#[doc = "LTC PKHA E 26 Register"]
pub struct LTC0_PKE_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 26 Register"]
pub mod ltc0_pke_26;
#[doc = "LTC PKHA E1 11 Register"]
pub struct LTC0_PKE1_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 11 Register"]
pub mod ltc0_pke1_11;
#[doc = "LTC PKHA E 27 Register"]
pub struct LTC0_PKE_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 27 Register"]
pub mod ltc0_pke_27;
#[doc = "LTC PKHA E1 12 Register"]
pub struct LTC0_PKE1_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 12 Register"]
pub mod ltc0_pke1_12;
#[doc = "LTC PKHA E 28 Register"]
pub struct LTC0_PKE_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 28 Register"]
pub mod ltc0_pke_28;
#[doc = "LTC PKHA E1 13 Register"]
pub struct LTC0_PKE1_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 13 Register"]
pub mod ltc0_pke1_13;
#[doc = "LTC PKHA E 29 Register"]
pub struct LTC0_PKE_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 29 Register"]
pub mod ltc0_pke_29;
#[doc = "LTC PKHA E1 14 Register"]
pub struct LTC0_PKE1_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 14 Register"]
pub mod ltc0_pke1_14;
#[doc = "LTC PKHA E 30 Register"]
pub struct LTC0_PKE_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 30 Register"]
pub mod ltc0_pke_30;
#[doc = "LTC PKHA E1 15 Register"]
pub struct LTC0_PKE1_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E1 15 Register"]
pub mod ltc0_pke1_15;
#[doc = "LTC PKHA E 31 Register"]
pub struct LTC0_PKE_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 31 Register"]
pub mod ltc0_pke_31;
#[doc = "LTC PKHA E2 0 Register"]
pub struct LTC0_PKE2_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 0 Register"]
pub mod ltc0_pke2_0;
#[doc = "LTC PKHA E 32 Register"]
pub struct LTC0_PKE_32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 32 Register"]
pub mod ltc0_pke_32;
#[doc = "LTC PKHA E2 1 Register"]
pub struct LTC0_PKE2_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 1 Register"]
pub mod ltc0_pke2_1;
#[doc = "LTC PKHA E 33 Register"]
pub struct LTC0_PKE_33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 33 Register"]
pub mod ltc0_pke_33;
#[doc = "LTC PKHA E2 2 Register"]
pub struct LTC0_PKE2_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 2 Register"]
pub mod ltc0_pke2_2;
#[doc = "LTC PKHA E 34 Register"]
pub struct LTC0_PKE_34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 34 Register"]
pub mod ltc0_pke_34;
#[doc = "LTC PKHA E2 3 Register"]
pub struct LTC0_PKE2_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 3 Register"]
pub mod ltc0_pke2_3;
#[doc = "LTC PKHA E 35 Register"]
pub struct LTC0_PKE_35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 35 Register"]
pub mod ltc0_pke_35;
#[doc = "LTC PKHA E2 4 Register"]
pub struct LTC0_PKE2_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 4 Register"]
pub mod ltc0_pke2_4;
#[doc = "LTC PKHA E 36 Register"]
pub struct LTC0_PKE_36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 36 Register"]
pub mod ltc0_pke_36;
#[doc = "LTC PKHA E2 5 Register"]
pub struct LTC0_PKE2_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 5 Register"]
pub mod ltc0_pke2_5;
#[doc = "LTC PKHA E 37 Register"]
pub struct LTC0_PKE_37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 37 Register"]
pub mod ltc0_pke_37;
#[doc = "LTC PKHA E2 6 Register"]
pub struct LTC0_PKE2_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 6 Register"]
pub mod ltc0_pke2_6;
#[doc = "LTC PKHA E 38 Register"]
pub struct LTC0_PKE_38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 38 Register"]
pub mod ltc0_pke_38;
#[doc = "LTC PKHA E2 7 Register"]
pub struct LTC0_PKE2_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 7 Register"]
pub mod ltc0_pke2_7;
#[doc = "LTC PKHA E 39 Register"]
pub struct LTC0_PKE_39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 39 Register"]
pub mod ltc0_pke_39;
#[doc = "LTC PKHA E2 8 Register"]
pub struct LTC0_PKE2_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 8 Register"]
pub mod ltc0_pke2_8;
#[doc = "LTC PKHA E 40 Register"]
pub struct LTC0_PKE_40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 40 Register"]
pub mod ltc0_pke_40;
#[doc = "LTC PKHA E2 9 Register"]
pub struct LTC0_PKE2_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 9 Register"]
pub mod ltc0_pke2_9;
#[doc = "LTC PKHA E 41 Register"]
pub struct LTC0_PKE_41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 41 Register"]
pub mod ltc0_pke_41;
#[doc = "LTC PKHA E2 10 Register"]
pub struct LTC0_PKE2_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 10 Register"]
pub mod ltc0_pke2_10;
#[doc = "LTC PKHA E 42 Register"]
pub struct LTC0_PKE_42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 42 Register"]
pub mod ltc0_pke_42;
#[doc = "LTC PKHA E2 11 Register"]
pub struct LTC0_PKE2_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 11 Register"]
pub mod ltc0_pke2_11;
#[doc = "LTC PKHA E 43 Register"]
pub struct LTC0_PKE_43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 43 Register"]
pub mod ltc0_pke_43;
#[doc = "LTC PKHA E2 12 Register"]
pub struct LTC0_PKE2_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 12 Register"]
pub mod ltc0_pke2_12;
#[doc = "LTC PKHA E 44 Register"]
pub struct LTC0_PKE_44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 44 Register"]
pub mod ltc0_pke_44;
#[doc = "LTC PKHA E2 13 Register"]
pub struct LTC0_PKE2_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 13 Register"]
pub mod ltc0_pke2_13;
#[doc = "LTC PKHA E 45 Register"]
pub struct LTC0_PKE_45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 45 Register"]
pub mod ltc0_pke_45;
#[doc = "LTC PKHA E2 14 Register"]
pub struct LTC0_PKE2_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 14 Register"]
pub mod ltc0_pke2_14;
#[doc = "LTC PKHA E 46 Register"]
pub struct LTC0_PKE_46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 46 Register"]
pub mod ltc0_pke_46;
#[doc = "LTC PKHA E2 15 Register"]
pub struct LTC0_PKE2_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E2 15 Register"]
pub mod ltc0_pke2_15;
#[doc = "LTC PKHA E 47 Register"]
pub struct LTC0_PKE_47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 47 Register"]
pub mod ltc0_pke_47;
#[doc = "LTC PKHA E3 0 Register"]
pub struct LTC0_PKE3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 0 Register"]
pub mod ltc0_pke3_0;
#[doc = "LTC PKHA E 48 Register"]
pub struct LTC0_PKE_48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 48 Register"]
pub mod ltc0_pke_48;
#[doc = "LTC PKHA E3 1 Register"]
pub struct LTC0_PKE3_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 1 Register"]
pub mod ltc0_pke3_1;
#[doc = "LTC PKHA E 49 Register"]
pub struct LTC0_PKE_49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 49 Register"]
pub mod ltc0_pke_49;
#[doc = "LTC PKHA E3 2 Register"]
pub struct LTC0_PKE3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 2 Register"]
pub mod ltc0_pke3_2;
#[doc = "LTC PKHA E 50 Register"]
pub struct LTC0_PKE_50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 50 Register"]
pub mod ltc0_pke_50;
#[doc = "LTC PKHA E3 3 Register"]
pub struct LTC0_PKE3_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 3 Register"]
pub mod ltc0_pke3_3;
#[doc = "LTC PKHA E 51 Register"]
pub struct LTC0_PKE_51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 51 Register"]
pub mod ltc0_pke_51;
#[doc = "LTC PKHA E3 4 Register"]
pub struct LTC0_PKE3_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 4 Register"]
pub mod ltc0_pke3_4;
#[doc = "LTC PKHA E 52 Register"]
pub struct LTC0_PKE_52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 52 Register"]
pub mod ltc0_pke_52;
#[doc = "LTC PKHA E3 5 Register"]
pub struct LTC0_PKE3_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 5 Register"]
pub mod ltc0_pke3_5;
#[doc = "LTC PKHA E 53 Register"]
pub struct LTC0_PKE_53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 53 Register"]
pub mod ltc0_pke_53;
#[doc = "LTC PKHA E3 6 Register"]
pub struct LTC0_PKE3_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 6 Register"]
pub mod ltc0_pke3_6;
#[doc = "LTC PKHA E 54 Register"]
pub struct LTC0_PKE_54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 54 Register"]
pub mod ltc0_pke_54;
#[doc = "LTC PKHA E3 7 Register"]
pub struct LTC0_PKE3_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 7 Register"]
pub mod ltc0_pke3_7;
#[doc = "LTC PKHA E 55 Register"]
pub struct LTC0_PKE_55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 55 Register"]
pub mod ltc0_pke_55;
#[doc = "LTC PKHA E3 8 Register"]
pub struct LTC0_PKE3_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 8 Register"]
pub mod ltc0_pke3_8;
#[doc = "LTC PKHA E 56 Register"]
pub struct LTC0_PKE_56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 56 Register"]
pub mod ltc0_pke_56;
#[doc = "LTC PKHA E3 9 Register"]
pub struct LTC0_PKE3_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 9 Register"]
pub mod ltc0_pke3_9;
#[doc = "LTC PKHA E 57 Register"]
pub struct LTC0_PKE_57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 57 Register"]
pub mod ltc0_pke_57;
#[doc = "LTC PKHA E3 10 Register"]
pub struct LTC0_PKE3_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 10 Register"]
pub mod ltc0_pke3_10;
#[doc = "LTC PKHA E 58 Register"]
pub struct LTC0_PKE_58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 58 Register"]
pub mod ltc0_pke_58;
#[doc = "LTC PKHA E3 11 Register"]
pub struct LTC0_PKE3_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 11 Register"]
pub mod ltc0_pke3_11;
#[doc = "LTC PKHA E 59 Register"]
pub struct LTC0_PKE_59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 59 Register"]
pub mod ltc0_pke_59;
#[doc = "LTC PKHA E3 12 Register"]
pub struct LTC0_PKE3_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 12 Register"]
pub mod ltc0_pke3_12;
#[doc = "LTC PKHA E 60 Register"]
pub struct LTC0_PKE_60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 60 Register"]
pub mod ltc0_pke_60;
#[doc = "LTC PKHA E3 13 Register"]
pub struct LTC0_PKE3_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 13 Register"]
pub mod ltc0_pke3_13;
#[doc = "LTC PKHA E 61 Register"]
pub struct LTC0_PKE_61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 61 Register"]
pub mod ltc0_pke_61;
#[doc = "LTC PKHA E3 14 Register"]
pub struct LTC0_PKE3_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 14 Register"]
pub mod ltc0_pke3_14;
#[doc = "LTC PKHA E 62 Register"]
pub struct LTC0_PKE_62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 62 Register"]
pub mod ltc0_pke_62;
#[doc = "LTC PKHA E3 15 Register"]
pub struct LTC0_PKE3_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E3 15 Register"]
pub mod ltc0_pke3_15;
#[doc = "LTC PKHA E 63 Register"]
pub struct LTC0_PKE_63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LTC PKHA E 63 Register"]
pub mod ltc0_pke_63;
