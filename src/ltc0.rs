#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ltc0_ltc0: [u8; 0x04],
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - LTC Key Size Register"]
    pub ltc0_ks: crate::Reg<ltc0_ks::LTC0_KS_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - LTC Data Size Register"]
    pub ltc0_ds: crate::Reg<ltc0_ds::LTC0_DS_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - LTC ICV Size Register"]
    pub ltc0_icvs: crate::Reg<ltc0_icvs::LTC0_ICVS_SPEC>,
    _reserved4: [u8; 0x14],
    #[doc = "0x30 - LTC Command Register"]
    pub ltc0_com: crate::Reg<ltc0_com::LTC0_COM_SPEC>,
    #[doc = "0x34 - LTC Control Register"]
    pub ltc0_ctl: crate::Reg<ltc0_ctl::LTC0_CTL_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x40 - LTC Clear Written Register"]
    pub ltc0_cw: crate::Reg<ltc0_cw::LTC0_CW_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x48 - LTC Status Register"]
    pub ltc0_sta: crate::Reg<ltc0_sta::LTC0_STA_SPEC>,
    #[doc = "0x4c - LTC Error Status Register"]
    pub ltc0_esta: crate::Reg<ltc0_esta::LTC0_ESTA_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x58 - LTC AAD Size Register"]
    pub ltc0_aadsz: crate::Reg<ltc0_aadsz::LTC0_AADSZ_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x60 - LTC IV Size Register"]
    pub ltc0_ivsz: crate::Reg<ltc0_ivsz::LTC0_IVSZ_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x68 - LTC DPA Mask Seed Register"]
    pub ltc0_dpams: crate::Reg<ltc0_dpams::LTC0_DPAMS_SPEC>,
    _reserved12: [u8; 0x14],
    #[doc = "0x80 - LTC PKHA A Size Register"]
    pub ltc0_pkasz: crate::Reg<ltc0_pkasz::LTC0_PKASZ_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x88 - LTC PKHA B Size Register"]
    pub ltc0_pkbsz: crate::Reg<ltc0_pkbsz::LTC0_PKBSZ_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x90 - LTC PKHA N Size Register"]
    pub ltc0_pknsz: crate::Reg<ltc0_pknsz::LTC0_PKNSZ_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x98 - LTC PKHA E Size Register"]
    pub ltc0_pkesz: crate::Reg<ltc0_pkesz::LTC0_PKESZ_SPEC>,
    _reserved16: [u8; 0x64],
    #[doc = "0x100 - LTC Context Register"]
    pub ltc0_ctx_0: crate::Reg<ltc0_ctx_0::LTC0_CTX_0_SPEC>,
    #[doc = "0x104 - LTC Context Register"]
    pub ltc0_ctx_1: crate::Reg<ltc0_ctx_1::LTC0_CTX_1_SPEC>,
    #[doc = "0x108 - LTC Context Register"]
    pub ltc0_ctx_2: crate::Reg<ltc0_ctx_2::LTC0_CTX_2_SPEC>,
    #[doc = "0x10c - LTC Context Register"]
    pub ltc0_ctx_3: crate::Reg<ltc0_ctx_3::LTC0_CTX_3_SPEC>,
    #[doc = "0x110 - LTC Context Register"]
    pub ltc0_ctx_4: crate::Reg<ltc0_ctx_4::LTC0_CTX_4_SPEC>,
    #[doc = "0x114 - LTC Context Register"]
    pub ltc0_ctx_5: crate::Reg<ltc0_ctx_5::LTC0_CTX_5_SPEC>,
    #[doc = "0x118 - LTC Context Register"]
    pub ltc0_ctx_6: crate::Reg<ltc0_ctx_6::LTC0_CTX_6_SPEC>,
    #[doc = "0x11c - LTC Context Register"]
    pub ltc0_ctx_7: crate::Reg<ltc0_ctx_7::LTC0_CTX_7_SPEC>,
    #[doc = "0x120 - LTC Context Register"]
    pub ltc0_ctx_8: crate::Reg<ltc0_ctx_8::LTC0_CTX_8_SPEC>,
    #[doc = "0x124 - LTC Context Register"]
    pub ltc0_ctx_9: crate::Reg<ltc0_ctx_9::LTC0_CTX_9_SPEC>,
    #[doc = "0x128 - LTC Context Register"]
    pub ltc0_ctx_10: crate::Reg<ltc0_ctx_10::LTC0_CTX_10_SPEC>,
    #[doc = "0x12c - LTC Context Register"]
    pub ltc0_ctx_11: crate::Reg<ltc0_ctx_11::LTC0_CTX_11_SPEC>,
    #[doc = "0x130 - LTC Context Register"]
    pub ltc0_ctx_12: crate::Reg<ltc0_ctx_12::LTC0_CTX_12_SPEC>,
    #[doc = "0x134 - LTC Context Register"]
    pub ltc0_ctx_13: crate::Reg<ltc0_ctx_13::LTC0_CTX_13_SPEC>,
    #[doc = "0x138 - LTC Context Register"]
    pub ltc0_ctx_14: crate::Reg<ltc0_ctx_14::LTC0_CTX_14_SPEC>,
    #[doc = "0x13c - LTC Context Register"]
    pub ltc0_ctx_15: crate::Reg<ltc0_ctx_15::LTC0_CTX_15_SPEC>,
    _reserved32: [u8; 0xc0],
    #[doc = "0x200 - LTC Key Registers"]
    pub ltc0_key_0: crate::Reg<ltc0_key_0::LTC0_KEY_0_SPEC>,
    #[doc = "0x204 - LTC Key Registers"]
    pub ltc0_key_1: crate::Reg<ltc0_key_1::LTC0_KEY_1_SPEC>,
    #[doc = "0x208 - LTC Key Registers"]
    pub ltc0_key_2: crate::Reg<ltc0_key_2::LTC0_KEY_2_SPEC>,
    #[doc = "0x20c - LTC Key Registers"]
    pub ltc0_key_3: crate::Reg<ltc0_key_3::LTC0_KEY_3_SPEC>,
    #[doc = "0x210 - LTC Key Registers"]
    pub ltc0_key_4: crate::Reg<ltc0_key_4::LTC0_KEY_4_SPEC>,
    #[doc = "0x214 - LTC Key Registers"]
    pub ltc0_key_5: crate::Reg<ltc0_key_5::LTC0_KEY_5_SPEC>,
    #[doc = "0x218 - LTC Key Registers"]
    pub ltc0_key_6: crate::Reg<ltc0_key_6::LTC0_KEY_6_SPEC>,
    #[doc = "0x21c - LTC Key Registers"]
    pub ltc0_key_7: crate::Reg<ltc0_key_7::LTC0_KEY_7_SPEC>,
    _reserved40: [u8; 0x02d0],
    #[doc = "0x4f0 - LTC Version ID Register"]
    pub ltc0_vid1: crate::Reg<ltc0_vid1::LTC0_VID1_SPEC>,
    #[doc = "0x4f4 - LTC Version ID 2 Register"]
    pub ltc0_vid2: crate::Reg<ltc0_vid2::LTC0_VID2_SPEC>,
    #[doc = "0x4f8 - LTC CHA Version ID Register"]
    pub ltc0_chavid: crate::Reg<ltc0_chavid::LTC0_CHAVID_SPEC>,
    _reserved43: [u8; 0x02c4],
    #[doc = "0x7c0 - LTC FIFO Status Register"]
    pub ltc0_fifosta: crate::Reg<ltc0_fifosta::LTC0_FIFOSTA_SPEC>,
    _reserved44: [u8; 0x1c],
    #[doc = "0x7e0 - LTC Input Data FIFO"]
    pub ltc0_ififo: crate::Reg<ltc0_ififo::LTC0_IFIFO_SPEC>,
    _reserved45: [u8; 0x0c],
    #[doc = "0x7f0 - LTC Output Data FIFO"]
    pub ltc0_ofifo: crate::Reg<ltc0_ofifo::LTC0_OFIFO_SPEC>,
    _reserved46: [u8; 0x0c],
    _reserved_46_ltc0_ltc0_: [u8; 0x04],
    _reserved_47_ltc0_ltc0_: [u8; 0x04],
    _reserved_48_ltc0_ltc0_: [u8; 0x04],
    _reserved_49_ltc0_ltc0_: [u8; 0x04],
    _reserved_50_ltc0_ltc0_: [u8; 0x04],
    _reserved_51_ltc0_ltc0_: [u8; 0x04],
    _reserved_52_ltc0_ltc0_: [u8; 0x04],
    _reserved_53_ltc0_ltc0_: [u8; 0x04],
    _reserved_54_ltc0_ltc0_: [u8; 0x04],
    _reserved_55_ltc0_ltc0_: [u8; 0x04],
    _reserved_56_ltc0_ltc0_: [u8; 0x04],
    _reserved_57_ltc0_ltc0_: [u8; 0x04],
    _reserved_58_ltc0_ltc0_: [u8; 0x04],
    _reserved_59_ltc0_ltc0_: [u8; 0x04],
    _reserved_60_ltc0_ltc0_: [u8; 0x04],
    _reserved_61_ltc0_ltc0_: [u8; 0x04],
    _reserved_62_ltc0_ltc0_: [u8; 0x04],
    _reserved_63_ltc0_ltc0_: [u8; 0x04],
    _reserved_64_ltc0_ltc0_: [u8; 0x04],
    _reserved_65_ltc0_ltc0_: [u8; 0x04],
    _reserved_66_ltc0_ltc0_: [u8; 0x04],
    _reserved_67_ltc0_ltc0_: [u8; 0x04],
    _reserved_68_ltc0_ltc0_: [u8; 0x04],
    _reserved_69_ltc0_ltc0_: [u8; 0x04],
    _reserved_70_ltc0_ltc0_: [u8; 0x04],
    _reserved_71_ltc0_ltc0_: [u8; 0x04],
    _reserved_72_ltc0_ltc0_: [u8; 0x04],
    _reserved_73_ltc0_ltc0_: [u8; 0x04],
    _reserved_74_ltc0_ltc0_: [u8; 0x04],
    _reserved_75_ltc0_ltc0_: [u8; 0x04],
    _reserved_76_ltc0_ltc0_: [u8; 0x04],
    _reserved_77_ltc0_ltc0_: [u8; 0x04],
    _reserved_78_ltc0_ltc0_: [u8; 0x04],
    _reserved_79_ltc0_ltc0_: [u8; 0x04],
    _reserved_80_ltc0_ltc0_: [u8; 0x04],
    _reserved_81_ltc0_ltc0_: [u8; 0x04],
    _reserved_82_ltc0_ltc0_: [u8; 0x04],
    _reserved_83_ltc0_ltc0_: [u8; 0x04],
    _reserved_84_ltc0_ltc0_: [u8; 0x04],
    _reserved_85_ltc0_ltc0_: [u8; 0x04],
    _reserved_86_ltc0_ltc0_: [u8; 0x04],
    _reserved_87_ltc0_ltc0_: [u8; 0x04],
    _reserved_88_ltc0_ltc0_: [u8; 0x04],
    _reserved_89_ltc0_ltc0_: [u8; 0x04],
    _reserved_90_ltc0_ltc0_: [u8; 0x04],
    _reserved_91_ltc0_ltc0_: [u8; 0x04],
    _reserved_92_ltc0_ltc0_: [u8; 0x04],
    _reserved_93_ltc0_ltc0_: [u8; 0x04],
    _reserved_94_ltc0_ltc0_: [u8; 0x04],
    _reserved_95_ltc0_ltc0_: [u8; 0x04],
    _reserved_96_ltc0_ltc0_: [u8; 0x04],
    _reserved_97_ltc0_ltc0_: [u8; 0x04],
    _reserved_98_ltc0_ltc0_: [u8; 0x04],
    _reserved_99_ltc0_ltc0_: [u8; 0x04],
    _reserved_100_ltc0_ltc0_: [u8; 0x04],
    _reserved_101_ltc0_ltc0_: [u8; 0x04],
    _reserved_102_ltc0_ltc0_: [u8; 0x04],
    _reserved_103_ltc0_ltc0_: [u8; 0x04],
    _reserved_104_ltc0_ltc0_: [u8; 0x04],
    _reserved_105_ltc0_ltc0_: [u8; 0x04],
    _reserved_106_ltc0_ltc0_: [u8; 0x04],
    _reserved_107_ltc0_ltc0_: [u8; 0x04],
    _reserved_108_ltc0_ltc0_: [u8; 0x04],
    _reserved_109_ltc0_ltc0_: [u8; 0x04],
    _reserved110: [u8; 0x0100],
    _reserved_110_ltc0_ltc0_: [u8; 0x04],
    _reserved_111_ltc0_ltc0_: [u8; 0x04],
    _reserved_112_ltc0_ltc0_: [u8; 0x04],
    _reserved_113_ltc0_ltc0_: [u8; 0x04],
    _reserved_114_ltc0_ltc0_: [u8; 0x04],
    _reserved_115_ltc0_ltc0_: [u8; 0x04],
    _reserved_116_ltc0_ltc0_: [u8; 0x04],
    _reserved_117_ltc0_ltc0_: [u8; 0x04],
    _reserved_118_ltc0_ltc0_: [u8; 0x04],
    _reserved_119_ltc0_ltc0_: [u8; 0x04],
    _reserved_120_ltc0_ltc0_: [u8; 0x04],
    _reserved_121_ltc0_ltc0_: [u8; 0x04],
    _reserved_122_ltc0_ltc0_: [u8; 0x04],
    _reserved_123_ltc0_ltc0_: [u8; 0x04],
    _reserved_124_ltc0_ltc0_: [u8; 0x04],
    _reserved_125_ltc0_ltc0_: [u8; 0x04],
    _reserved_126_ltc0_ltc0_: [u8; 0x04],
    _reserved_127_ltc0_ltc0_: [u8; 0x04],
    _reserved_128_ltc0_ltc0_: [u8; 0x04],
    _reserved_129_ltc0_ltc0_: [u8; 0x04],
    _reserved_130_ltc0_ltc0_: [u8; 0x04],
    _reserved_131_ltc0_ltc0_: [u8; 0x04],
    _reserved_132_ltc0_ltc0_: [u8; 0x04],
    _reserved_133_ltc0_ltc0_: [u8; 0x04],
    _reserved_134_ltc0_ltc0_: [u8; 0x04],
    _reserved_135_ltc0_ltc0_: [u8; 0x04],
    _reserved_136_ltc0_ltc0_: [u8; 0x04],
    _reserved_137_ltc0_ltc0_: [u8; 0x04],
    _reserved_138_ltc0_ltc0_: [u8; 0x04],
    _reserved_139_ltc0_ltc0_: [u8; 0x04],
    _reserved_140_ltc0_ltc0_: [u8; 0x04],
    _reserved_141_ltc0_ltc0_: [u8; 0x04],
    _reserved_142_ltc0_ltc0_: [u8; 0x04],
    _reserved_143_ltc0_ltc0_: [u8; 0x04],
    _reserved_144_ltc0_ltc0_: [u8; 0x04],
    _reserved_145_ltc0_ltc0_: [u8; 0x04],
    _reserved_146_ltc0_ltc0_: [u8; 0x04],
    _reserved_147_ltc0_ltc0_: [u8; 0x04],
    _reserved_148_ltc0_ltc0_: [u8; 0x04],
    _reserved_149_ltc0_ltc0_: [u8; 0x04],
    _reserved_150_ltc0_ltc0_: [u8; 0x04],
    _reserved_151_ltc0_ltc0_: [u8; 0x04],
    _reserved_152_ltc0_ltc0_: [u8; 0x04],
    _reserved_153_ltc0_ltc0_: [u8; 0x04],
    _reserved_154_ltc0_ltc0_: [u8; 0x04],
    _reserved_155_ltc0_ltc0_: [u8; 0x04],
    _reserved_156_ltc0_ltc0_: [u8; 0x04],
    _reserved_157_ltc0_ltc0_: [u8; 0x04],
    _reserved_158_ltc0_ltc0_: [u8; 0x04],
    _reserved_159_ltc0_ltc0_: [u8; 0x04],
    _reserved_160_ltc0_ltc0_: [u8; 0x04],
    _reserved_161_ltc0_ltc0_: [u8; 0x04],
    _reserved_162_ltc0_ltc0_: [u8; 0x04],
    _reserved_163_ltc0_ltc0_: [u8; 0x04],
    _reserved_164_ltc0_ltc0_: [u8; 0x04],
    _reserved_165_ltc0_ltc0_: [u8; 0x04],
    _reserved_166_ltc0_ltc0_: [u8; 0x04],
    _reserved_167_ltc0_ltc0_: [u8; 0x04],
    _reserved_168_ltc0_ltc0_: [u8; 0x04],
    _reserved_169_ltc0_ltc0_: [u8; 0x04],
    _reserved_170_ltc0_ltc0_: [u8; 0x04],
    _reserved_171_ltc0_ltc0_: [u8; 0x04],
    _reserved_172_ltc0_ltc0_: [u8; 0x04],
    _reserved_173_ltc0_ltc0_: [u8; 0x04],
    _reserved174: [u8; 0x0100],
    _reserved_174_ltc0_ltc0_: [u8; 0x04],
    _reserved_175_ltc0_ltc0_: [u8; 0x04],
    _reserved_176_ltc0_ltc0_: [u8; 0x04],
    _reserved_177_ltc0_ltc0_: [u8; 0x04],
    _reserved_178_ltc0_ltc0_: [u8; 0x04],
    _reserved_179_ltc0_ltc0_: [u8; 0x04],
    _reserved_180_ltc0_ltc0_: [u8; 0x04],
    _reserved_181_ltc0_ltc0_: [u8; 0x04],
    _reserved_182_ltc0_ltc0_: [u8; 0x04],
    _reserved_183_ltc0_ltc0_: [u8; 0x04],
    _reserved_184_ltc0_ltc0_: [u8; 0x04],
    _reserved_185_ltc0_ltc0_: [u8; 0x04],
    _reserved_186_ltc0_ltc0_: [u8; 0x04],
    _reserved_187_ltc0_ltc0_: [u8; 0x04],
    _reserved_188_ltc0_ltc0_: [u8; 0x04],
    _reserved_189_ltc0_ltc0_: [u8; 0x04],
    _reserved_190_ltc0_ltc0_: [u8; 0x04],
    _reserved_191_ltc0_ltc0_: [u8; 0x04],
    _reserved_192_ltc0_ltc0_: [u8; 0x04],
    _reserved_193_ltc0_ltc0_: [u8; 0x04],
    _reserved_194_ltc0_ltc0_: [u8; 0x04],
    _reserved_195_ltc0_ltc0_: [u8; 0x04],
    _reserved_196_ltc0_ltc0_: [u8; 0x04],
    _reserved_197_ltc0_ltc0_: [u8; 0x04],
    _reserved_198_ltc0_ltc0_: [u8; 0x04],
    _reserved_199_ltc0_ltc0_: [u8; 0x04],
    _reserved_200_ltc0_ltc0_: [u8; 0x04],
    _reserved_201_ltc0_ltc0_: [u8; 0x04],
    _reserved_202_ltc0_ltc0_: [u8; 0x04],
    _reserved_203_ltc0_ltc0_: [u8; 0x04],
    _reserved_204_ltc0_ltc0_: [u8; 0x04],
    _reserved_205_ltc0_ltc0_: [u8; 0x04],
    _reserved_206_ltc0_ltc0_: [u8; 0x04],
    _reserved_207_ltc0_ltc0_: [u8; 0x04],
    _reserved_208_ltc0_ltc0_: [u8; 0x04],
    _reserved_209_ltc0_ltc0_: [u8; 0x04],
    _reserved_210_ltc0_ltc0_: [u8; 0x04],
    _reserved_211_ltc0_ltc0_: [u8; 0x04],
    _reserved_212_ltc0_ltc0_: [u8; 0x04],
    _reserved_213_ltc0_ltc0_: [u8; 0x04],
    _reserved_214_ltc0_ltc0_: [u8; 0x04],
    _reserved_215_ltc0_ltc0_: [u8; 0x04],
    _reserved_216_ltc0_ltc0_: [u8; 0x04],
    _reserved_217_ltc0_ltc0_: [u8; 0x04],
    _reserved_218_ltc0_ltc0_: [u8; 0x04],
    _reserved_219_ltc0_ltc0_: [u8; 0x04],
    _reserved_220_ltc0_ltc0_: [u8; 0x04],
    _reserved_221_ltc0_ltc0_: [u8; 0x04],
    _reserved_222_ltc0_ltc0_: [u8; 0x04],
    _reserved_223_ltc0_ltc0_: [u8; 0x04],
    _reserved_224_ltc0_ltc0_: [u8; 0x04],
    _reserved_225_ltc0_ltc0_: [u8; 0x04],
    _reserved_226_ltc0_ltc0_: [u8; 0x04],
    _reserved_227_ltc0_ltc0_: [u8; 0x04],
    _reserved_228_ltc0_ltc0_: [u8; 0x04],
    _reserved_229_ltc0_ltc0_: [u8; 0x04],
    _reserved_230_ltc0_ltc0_: [u8; 0x04],
    _reserved_231_ltc0_ltc0_: [u8; 0x04],
    _reserved_232_ltc0_ltc0_: [u8; 0x04],
    _reserved_233_ltc0_ltc0_: [u8; 0x04],
    _reserved_234_ltc0_ltc0_: [u8; 0x04],
    _reserved_235_ltc0_ltc0_: [u8; 0x04],
    _reserved_236_ltc0_ltc0_: [u8; 0x04],
    _reserved_237_ltc0_ltc0_: [u8; 0x04],
    _reserved238: [u8; 0x0100],
    _reserved_238_ltc0_ltc0_: [u8; 0x04],
    _reserved_239_ltc0_ltc0_: [u8; 0x04],
    _reserved_240_ltc0_ltc0_: [u8; 0x04],
    _reserved_241_ltc0_ltc0_: [u8; 0x04],
    _reserved_242_ltc0_ltc0_: [u8; 0x04],
    _reserved_243_ltc0_ltc0_: [u8; 0x04],
    _reserved_244_ltc0_ltc0_: [u8; 0x04],
    _reserved_245_ltc0_ltc0_: [u8; 0x04],
    _reserved_246_ltc0_ltc0_: [u8; 0x04],
    _reserved_247_ltc0_ltc0_: [u8; 0x04],
    _reserved_248_ltc0_ltc0_: [u8; 0x04],
    _reserved_249_ltc0_ltc0_: [u8; 0x04],
    _reserved_250_ltc0_ltc0_: [u8; 0x04],
    _reserved_251_ltc0_ltc0_: [u8; 0x04],
    _reserved_252_ltc0_ltc0_: [u8; 0x04],
    _reserved_253_ltc0_ltc0_: [u8; 0x04],
    _reserved_254_ltc0_ltc0_: [u8; 0x04],
    _reserved_255_ltc0_ltc0_: [u8; 0x04],
    _reserved_256_ltc0_ltc0_: [u8; 0x04],
    _reserved_257_ltc0_ltc0_: [u8; 0x04],
    _reserved_258_ltc0_ltc0_: [u8; 0x04],
    _reserved_259_ltc0_ltc0_: [u8; 0x04],
    _reserved_260_ltc0_ltc0_: [u8; 0x04],
    _reserved_261_ltc0_ltc0_: [u8; 0x04],
    _reserved_262_ltc0_ltc0_: [u8; 0x04],
    _reserved_263_ltc0_ltc0_: [u8; 0x04],
    _reserved_264_ltc0_ltc0_: [u8; 0x04],
    _reserved_265_ltc0_ltc0_: [u8; 0x04],
    _reserved_266_ltc0_ltc0_: [u8; 0x04],
    _reserved_267_ltc0_ltc0_: [u8; 0x04],
    _reserved_268_ltc0_ltc0_: [u8; 0x04],
    _reserved_269_ltc0_ltc0_: [u8; 0x04],
    _reserved_270_ltc0_ltc0_: [u8; 0x04],
    _reserved_271_ltc0_ltc0_: [u8; 0x04],
    _reserved_272_ltc0_ltc0_: [u8; 0x04],
    _reserved_273_ltc0_ltc0_: [u8; 0x04],
    _reserved_274_ltc0_ltc0_: [u8; 0x04],
    _reserved_275_ltc0_ltc0_: [u8; 0x04],
    _reserved_276_ltc0_ltc0_: [u8; 0x04],
    _reserved_277_ltc0_ltc0_: [u8; 0x04],
    _reserved_278_ltc0_ltc0_: [u8; 0x04],
    _reserved_279_ltc0_ltc0_: [u8; 0x04],
    _reserved_280_ltc0_ltc0_: [u8; 0x04],
    _reserved_281_ltc0_ltc0_: [u8; 0x04],
    _reserved_282_ltc0_ltc0_: [u8; 0x04],
    _reserved_283_ltc0_ltc0_: [u8; 0x04],
    _reserved_284_ltc0_ltc0_: [u8; 0x04],
    _reserved_285_ltc0_ltc0_: [u8; 0x04],
    _reserved_286_ltc0_ltc0_: [u8; 0x04],
    _reserved_287_ltc0_ltc0_: [u8; 0x04],
    _reserved_288_ltc0_ltc0_: [u8; 0x04],
    _reserved_289_ltc0_ltc0_: [u8; 0x04],
    _reserved_290_ltc0_ltc0_: [u8; 0x04],
    _reserved_291_ltc0_ltc0_: [u8; 0x04],
    _reserved_292_ltc0_ltc0_: [u8; 0x04],
    _reserved_293_ltc0_ltc0_: [u8; 0x04],
    _reserved_294_ltc0_ltc0_: [u8; 0x04],
    _reserved_295_ltc0_ltc0_: [u8; 0x04],
    _reserved_296_ltc0_ltc0_: [u8; 0x04],
    _reserved_297_ltc0_ltc0_: [u8; 0x04],
    _reserved_298_ltc0_ltc0_: [u8; 0x04],
    _reserved_299_ltc0_ltc0_: [u8; 0x04],
    _reserved_300_ltc0_ltc0_: [u8; 0x04],
    _reserved_301_ltc0_ltc0_: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - LTC Mode Register (PublicKey)"]
    #[inline(always)]
    pub fn ltc0_ltc0_mdpk(&self) -> &crate::Reg<ltc0_ltc0_mdpk::LTC0_LTC0_MDPK_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<ltc0_ltc0_mdpk::LTC0_LTC0_MDPK_SPEC>)
        }
    }
    #[doc = "0x00 - LTC Mode Register (non-PKHA/non-RNG use)"]
    #[inline(always)]
    pub fn ltc0_ltc0_md(&self) -> &crate::Reg<ltc0_ltc0_md::LTC0_LTC0_MD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<ltc0_ltc0_md::LTC0_LTC0_MD_SPEC>)
        }
    }
    #[doc = "0x800 - LTC PKHA A 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_0(&self) -> &crate::Reg<ltc0_ltc0_pka_0::LTC0_LTC0_PKA_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2048usize)
                as *const crate::Reg<ltc0_ltc0_pka_0::LTC0_LTC0_PKA_0_SPEC>)
        }
    }
    #[doc = "0x800 - LTC PKHA A0 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_0(&self) -> &crate::Reg<ltc0_ltc0_pka0_0::LTC0_LTC0_PKA0_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2048usize)
                as *const crate::Reg<ltc0_ltc0_pka0_0::LTC0_LTC0_PKA0_0_SPEC>)
        }
    }
    #[doc = "0x804 - LTC PKHA A 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_1(&self) -> &crate::Reg<ltc0_ltc0_pka_1::LTC0_LTC0_PKA_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2052usize)
                as *const crate::Reg<ltc0_ltc0_pka_1::LTC0_LTC0_PKA_1_SPEC>)
        }
    }
    #[doc = "0x804 - LTC PKHA A0 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_1(&self) -> &crate::Reg<ltc0_ltc0_pka0_1::LTC0_LTC0_PKA0_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2052usize)
                as *const crate::Reg<ltc0_ltc0_pka0_1::LTC0_LTC0_PKA0_1_SPEC>)
        }
    }
    #[doc = "0x808 - LTC PKHA A 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_2(&self) -> &crate::Reg<ltc0_ltc0_pka_2::LTC0_LTC0_PKA_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2056usize)
                as *const crate::Reg<ltc0_ltc0_pka_2::LTC0_LTC0_PKA_2_SPEC>)
        }
    }
    #[doc = "0x808 - LTC PKHA A0 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_2(&self) -> &crate::Reg<ltc0_ltc0_pka0_2::LTC0_LTC0_PKA0_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2056usize)
                as *const crate::Reg<ltc0_ltc0_pka0_2::LTC0_LTC0_PKA0_2_SPEC>)
        }
    }
    #[doc = "0x80c - LTC PKHA A 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_3(&self) -> &crate::Reg<ltc0_ltc0_pka_3::LTC0_LTC0_PKA_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2060usize)
                as *const crate::Reg<ltc0_ltc0_pka_3::LTC0_LTC0_PKA_3_SPEC>)
        }
    }
    #[doc = "0x80c - LTC PKHA A0 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_3(&self) -> &crate::Reg<ltc0_ltc0_pka0_3::LTC0_LTC0_PKA0_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2060usize)
                as *const crate::Reg<ltc0_ltc0_pka0_3::LTC0_LTC0_PKA0_3_SPEC>)
        }
    }
    #[doc = "0x810 - LTC PKHA A 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_4(&self) -> &crate::Reg<ltc0_ltc0_pka_4::LTC0_LTC0_PKA_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2064usize)
                as *const crate::Reg<ltc0_ltc0_pka_4::LTC0_LTC0_PKA_4_SPEC>)
        }
    }
    #[doc = "0x810 - LTC PKHA A0 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_4(&self) -> &crate::Reg<ltc0_ltc0_pka0_4::LTC0_LTC0_PKA0_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2064usize)
                as *const crate::Reg<ltc0_ltc0_pka0_4::LTC0_LTC0_PKA0_4_SPEC>)
        }
    }
    #[doc = "0x814 - LTC PKHA A 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_5(&self) -> &crate::Reg<ltc0_ltc0_pka_5::LTC0_LTC0_PKA_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2068usize)
                as *const crate::Reg<ltc0_ltc0_pka_5::LTC0_LTC0_PKA_5_SPEC>)
        }
    }
    #[doc = "0x814 - LTC PKHA A0 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_5(&self) -> &crate::Reg<ltc0_ltc0_pka0_5::LTC0_LTC0_PKA0_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2068usize)
                as *const crate::Reg<ltc0_ltc0_pka0_5::LTC0_LTC0_PKA0_5_SPEC>)
        }
    }
    #[doc = "0x818 - LTC PKHA A 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_6(&self) -> &crate::Reg<ltc0_ltc0_pka_6::LTC0_LTC0_PKA_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2072usize)
                as *const crate::Reg<ltc0_ltc0_pka_6::LTC0_LTC0_PKA_6_SPEC>)
        }
    }
    #[doc = "0x818 - LTC PKHA A0 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_6(&self) -> &crate::Reg<ltc0_ltc0_pka0_6::LTC0_LTC0_PKA0_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2072usize)
                as *const crate::Reg<ltc0_ltc0_pka0_6::LTC0_LTC0_PKA0_6_SPEC>)
        }
    }
    #[doc = "0x81c - LTC PKHA A 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_7(&self) -> &crate::Reg<ltc0_ltc0_pka_7::LTC0_LTC0_PKA_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2076usize)
                as *const crate::Reg<ltc0_ltc0_pka_7::LTC0_LTC0_PKA_7_SPEC>)
        }
    }
    #[doc = "0x81c - LTC PKHA A0 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_7(&self) -> &crate::Reg<ltc0_ltc0_pka0_7::LTC0_LTC0_PKA0_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2076usize)
                as *const crate::Reg<ltc0_ltc0_pka0_7::LTC0_LTC0_PKA0_7_SPEC>)
        }
    }
    #[doc = "0x820 - LTC PKHA A 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_8(&self) -> &crate::Reg<ltc0_ltc0_pka_8::LTC0_LTC0_PKA_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2080usize)
                as *const crate::Reg<ltc0_ltc0_pka_8::LTC0_LTC0_PKA_8_SPEC>)
        }
    }
    #[doc = "0x820 - LTC PKHA A0 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_8(&self) -> &crate::Reg<ltc0_ltc0_pka0_8::LTC0_LTC0_PKA0_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2080usize)
                as *const crate::Reg<ltc0_ltc0_pka0_8::LTC0_LTC0_PKA0_8_SPEC>)
        }
    }
    #[doc = "0x824 - LTC PKHA A 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_9(&self) -> &crate::Reg<ltc0_ltc0_pka_9::LTC0_LTC0_PKA_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2084usize)
                as *const crate::Reg<ltc0_ltc0_pka_9::LTC0_LTC0_PKA_9_SPEC>)
        }
    }
    #[doc = "0x824 - LTC PKHA A0 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_9(&self) -> &crate::Reg<ltc0_ltc0_pka0_9::LTC0_LTC0_PKA0_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2084usize)
                as *const crate::Reg<ltc0_ltc0_pka0_9::LTC0_LTC0_PKA0_9_SPEC>)
        }
    }
    #[doc = "0x828 - LTC PKHA A 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_10(&self) -> &crate::Reg<ltc0_ltc0_pka_10::LTC0_LTC0_PKA_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2088usize)
                as *const crate::Reg<ltc0_ltc0_pka_10::LTC0_LTC0_PKA_10_SPEC>)
        }
    }
    #[doc = "0x828 - LTC PKHA A0 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_10(&self) -> &crate::Reg<ltc0_ltc0_pka0_10::LTC0_LTC0_PKA0_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2088usize)
                as *const crate::Reg<ltc0_ltc0_pka0_10::LTC0_LTC0_PKA0_10_SPEC>)
        }
    }
    #[doc = "0x82c - LTC PKHA A 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_11(&self) -> &crate::Reg<ltc0_ltc0_pka_11::LTC0_LTC0_PKA_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2092usize)
                as *const crate::Reg<ltc0_ltc0_pka_11::LTC0_LTC0_PKA_11_SPEC>)
        }
    }
    #[doc = "0x82c - LTC PKHA A0 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_11(&self) -> &crate::Reg<ltc0_ltc0_pka0_11::LTC0_LTC0_PKA0_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2092usize)
                as *const crate::Reg<ltc0_ltc0_pka0_11::LTC0_LTC0_PKA0_11_SPEC>)
        }
    }
    #[doc = "0x830 - LTC PKHA A 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_12(&self) -> &crate::Reg<ltc0_ltc0_pka_12::LTC0_LTC0_PKA_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2096usize)
                as *const crate::Reg<ltc0_ltc0_pka_12::LTC0_LTC0_PKA_12_SPEC>)
        }
    }
    #[doc = "0x830 - LTC PKHA A0 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_12(&self) -> &crate::Reg<ltc0_ltc0_pka0_12::LTC0_LTC0_PKA0_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2096usize)
                as *const crate::Reg<ltc0_ltc0_pka0_12::LTC0_LTC0_PKA0_12_SPEC>)
        }
    }
    #[doc = "0x834 - LTC PKHA A 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_13(&self) -> &crate::Reg<ltc0_ltc0_pka_13::LTC0_LTC0_PKA_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2100usize)
                as *const crate::Reg<ltc0_ltc0_pka_13::LTC0_LTC0_PKA_13_SPEC>)
        }
    }
    #[doc = "0x834 - LTC PKHA A0 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_13(&self) -> &crate::Reg<ltc0_ltc0_pka0_13::LTC0_LTC0_PKA0_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2100usize)
                as *const crate::Reg<ltc0_ltc0_pka0_13::LTC0_LTC0_PKA0_13_SPEC>)
        }
    }
    #[doc = "0x838 - LTC PKHA A 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_14(&self) -> &crate::Reg<ltc0_ltc0_pka_14::LTC0_LTC0_PKA_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2104usize)
                as *const crate::Reg<ltc0_ltc0_pka_14::LTC0_LTC0_PKA_14_SPEC>)
        }
    }
    #[doc = "0x838 - LTC PKHA A0 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_14(&self) -> &crate::Reg<ltc0_ltc0_pka0_14::LTC0_LTC0_PKA0_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2104usize)
                as *const crate::Reg<ltc0_ltc0_pka0_14::LTC0_LTC0_PKA0_14_SPEC>)
        }
    }
    #[doc = "0x83c - LTC PKHA A 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_15(&self) -> &crate::Reg<ltc0_ltc0_pka_15::LTC0_LTC0_PKA_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2108usize)
                as *const crate::Reg<ltc0_ltc0_pka_15::LTC0_LTC0_PKA_15_SPEC>)
        }
    }
    #[doc = "0x83c - LTC PKHA A0 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka0_15(&self) -> &crate::Reg<ltc0_ltc0_pka0_15::LTC0_LTC0_PKA0_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2108usize)
                as *const crate::Reg<ltc0_ltc0_pka0_15::LTC0_LTC0_PKA0_15_SPEC>)
        }
    }
    #[doc = "0x840 - LTC PKHA A 16 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_16(&self) -> &crate::Reg<ltc0_ltc0_pka_16::LTC0_LTC0_PKA_16_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2112usize)
                as *const crate::Reg<ltc0_ltc0_pka_16::LTC0_LTC0_PKA_16_SPEC>)
        }
    }
    #[doc = "0x840 - LTC PKHA A1 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_0(&self) -> &crate::Reg<ltc0_ltc0_pka1_0::LTC0_LTC0_PKA1_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2112usize)
                as *const crate::Reg<ltc0_ltc0_pka1_0::LTC0_LTC0_PKA1_0_SPEC>)
        }
    }
    #[doc = "0x844 - LTC PKHA A 17 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_17(&self) -> &crate::Reg<ltc0_ltc0_pka_17::LTC0_LTC0_PKA_17_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2116usize)
                as *const crate::Reg<ltc0_ltc0_pka_17::LTC0_LTC0_PKA_17_SPEC>)
        }
    }
    #[doc = "0x844 - LTC PKHA A1 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_1(&self) -> &crate::Reg<ltc0_ltc0_pka1_1::LTC0_LTC0_PKA1_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2116usize)
                as *const crate::Reg<ltc0_ltc0_pka1_1::LTC0_LTC0_PKA1_1_SPEC>)
        }
    }
    #[doc = "0x848 - LTC PKHA A 18 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_18(&self) -> &crate::Reg<ltc0_ltc0_pka_18::LTC0_LTC0_PKA_18_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2120usize)
                as *const crate::Reg<ltc0_ltc0_pka_18::LTC0_LTC0_PKA_18_SPEC>)
        }
    }
    #[doc = "0x848 - LTC PKHA A1 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_2(&self) -> &crate::Reg<ltc0_ltc0_pka1_2::LTC0_LTC0_PKA1_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2120usize)
                as *const crate::Reg<ltc0_ltc0_pka1_2::LTC0_LTC0_PKA1_2_SPEC>)
        }
    }
    #[doc = "0x84c - LTC PKHA A 19 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_19(&self) -> &crate::Reg<ltc0_ltc0_pka_19::LTC0_LTC0_PKA_19_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2124usize)
                as *const crate::Reg<ltc0_ltc0_pka_19::LTC0_LTC0_PKA_19_SPEC>)
        }
    }
    #[doc = "0x84c - LTC PKHA A1 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_3(&self) -> &crate::Reg<ltc0_ltc0_pka1_3::LTC0_LTC0_PKA1_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2124usize)
                as *const crate::Reg<ltc0_ltc0_pka1_3::LTC0_LTC0_PKA1_3_SPEC>)
        }
    }
    #[doc = "0x850 - LTC PKHA A 20 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_20(&self) -> &crate::Reg<ltc0_ltc0_pka_20::LTC0_LTC0_PKA_20_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2128usize)
                as *const crate::Reg<ltc0_ltc0_pka_20::LTC0_LTC0_PKA_20_SPEC>)
        }
    }
    #[doc = "0x850 - LTC PKHA A1 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_4(&self) -> &crate::Reg<ltc0_ltc0_pka1_4::LTC0_LTC0_PKA1_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2128usize)
                as *const crate::Reg<ltc0_ltc0_pka1_4::LTC0_LTC0_PKA1_4_SPEC>)
        }
    }
    #[doc = "0x854 - LTC PKHA A 21 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_21(&self) -> &crate::Reg<ltc0_ltc0_pka_21::LTC0_LTC0_PKA_21_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2132usize)
                as *const crate::Reg<ltc0_ltc0_pka_21::LTC0_LTC0_PKA_21_SPEC>)
        }
    }
    #[doc = "0x854 - LTC PKHA A1 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_5(&self) -> &crate::Reg<ltc0_ltc0_pka1_5::LTC0_LTC0_PKA1_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2132usize)
                as *const crate::Reg<ltc0_ltc0_pka1_5::LTC0_LTC0_PKA1_5_SPEC>)
        }
    }
    #[doc = "0x858 - LTC PKHA A 22 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_22(&self) -> &crate::Reg<ltc0_ltc0_pka_22::LTC0_LTC0_PKA_22_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2136usize)
                as *const crate::Reg<ltc0_ltc0_pka_22::LTC0_LTC0_PKA_22_SPEC>)
        }
    }
    #[doc = "0x858 - LTC PKHA A1 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_6(&self) -> &crate::Reg<ltc0_ltc0_pka1_6::LTC0_LTC0_PKA1_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2136usize)
                as *const crate::Reg<ltc0_ltc0_pka1_6::LTC0_LTC0_PKA1_6_SPEC>)
        }
    }
    #[doc = "0x85c - LTC PKHA A 23 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_23(&self) -> &crate::Reg<ltc0_ltc0_pka_23::LTC0_LTC0_PKA_23_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2140usize)
                as *const crate::Reg<ltc0_ltc0_pka_23::LTC0_LTC0_PKA_23_SPEC>)
        }
    }
    #[doc = "0x85c - LTC PKHA A1 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_7(&self) -> &crate::Reg<ltc0_ltc0_pka1_7::LTC0_LTC0_PKA1_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2140usize)
                as *const crate::Reg<ltc0_ltc0_pka1_7::LTC0_LTC0_PKA1_7_SPEC>)
        }
    }
    #[doc = "0x860 - LTC PKHA A 24 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_24(&self) -> &crate::Reg<ltc0_ltc0_pka_24::LTC0_LTC0_PKA_24_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2144usize)
                as *const crate::Reg<ltc0_ltc0_pka_24::LTC0_LTC0_PKA_24_SPEC>)
        }
    }
    #[doc = "0x860 - LTC PKHA A1 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_8(&self) -> &crate::Reg<ltc0_ltc0_pka1_8::LTC0_LTC0_PKA1_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2144usize)
                as *const crate::Reg<ltc0_ltc0_pka1_8::LTC0_LTC0_PKA1_8_SPEC>)
        }
    }
    #[doc = "0x864 - LTC PKHA A 25 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_25(&self) -> &crate::Reg<ltc0_ltc0_pka_25::LTC0_LTC0_PKA_25_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2148usize)
                as *const crate::Reg<ltc0_ltc0_pka_25::LTC0_LTC0_PKA_25_SPEC>)
        }
    }
    #[doc = "0x864 - LTC PKHA A1 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_9(&self) -> &crate::Reg<ltc0_ltc0_pka1_9::LTC0_LTC0_PKA1_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2148usize)
                as *const crate::Reg<ltc0_ltc0_pka1_9::LTC0_LTC0_PKA1_9_SPEC>)
        }
    }
    #[doc = "0x868 - LTC PKHA A 26 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_26(&self) -> &crate::Reg<ltc0_ltc0_pka_26::LTC0_LTC0_PKA_26_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2152usize)
                as *const crate::Reg<ltc0_ltc0_pka_26::LTC0_LTC0_PKA_26_SPEC>)
        }
    }
    #[doc = "0x868 - LTC PKHA A1 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_10(&self) -> &crate::Reg<ltc0_ltc0_pka1_10::LTC0_LTC0_PKA1_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2152usize)
                as *const crate::Reg<ltc0_ltc0_pka1_10::LTC0_LTC0_PKA1_10_SPEC>)
        }
    }
    #[doc = "0x86c - LTC PKHA A 27 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_27(&self) -> &crate::Reg<ltc0_ltc0_pka_27::LTC0_LTC0_PKA_27_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2156usize)
                as *const crate::Reg<ltc0_ltc0_pka_27::LTC0_LTC0_PKA_27_SPEC>)
        }
    }
    #[doc = "0x86c - LTC PKHA A1 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_11(&self) -> &crate::Reg<ltc0_ltc0_pka1_11::LTC0_LTC0_PKA1_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2156usize)
                as *const crate::Reg<ltc0_ltc0_pka1_11::LTC0_LTC0_PKA1_11_SPEC>)
        }
    }
    #[doc = "0x870 - LTC PKHA A 28 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_28(&self) -> &crate::Reg<ltc0_ltc0_pka_28::LTC0_LTC0_PKA_28_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2160usize)
                as *const crate::Reg<ltc0_ltc0_pka_28::LTC0_LTC0_PKA_28_SPEC>)
        }
    }
    #[doc = "0x870 - LTC PKHA A1 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_12(&self) -> &crate::Reg<ltc0_ltc0_pka1_12::LTC0_LTC0_PKA1_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2160usize)
                as *const crate::Reg<ltc0_ltc0_pka1_12::LTC0_LTC0_PKA1_12_SPEC>)
        }
    }
    #[doc = "0x874 - LTC PKHA A 29 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_29(&self) -> &crate::Reg<ltc0_ltc0_pka_29::LTC0_LTC0_PKA_29_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2164usize)
                as *const crate::Reg<ltc0_ltc0_pka_29::LTC0_LTC0_PKA_29_SPEC>)
        }
    }
    #[doc = "0x874 - LTC PKHA A1 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_13(&self) -> &crate::Reg<ltc0_ltc0_pka1_13::LTC0_LTC0_PKA1_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2164usize)
                as *const crate::Reg<ltc0_ltc0_pka1_13::LTC0_LTC0_PKA1_13_SPEC>)
        }
    }
    #[doc = "0x878 - LTC PKHA A 30 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_30(&self) -> &crate::Reg<ltc0_ltc0_pka_30::LTC0_LTC0_PKA_30_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2168usize)
                as *const crate::Reg<ltc0_ltc0_pka_30::LTC0_LTC0_PKA_30_SPEC>)
        }
    }
    #[doc = "0x878 - LTC PKHA A1 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_14(&self) -> &crate::Reg<ltc0_ltc0_pka1_14::LTC0_LTC0_PKA1_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2168usize)
                as *const crate::Reg<ltc0_ltc0_pka1_14::LTC0_LTC0_PKA1_14_SPEC>)
        }
    }
    #[doc = "0x87c - LTC PKHA A 31 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_31(&self) -> &crate::Reg<ltc0_ltc0_pka_31::LTC0_LTC0_PKA_31_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2172usize)
                as *const crate::Reg<ltc0_ltc0_pka_31::LTC0_LTC0_PKA_31_SPEC>)
        }
    }
    #[doc = "0x87c - LTC PKHA A1 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka1_15(&self) -> &crate::Reg<ltc0_ltc0_pka1_15::LTC0_LTC0_PKA1_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2172usize)
                as *const crate::Reg<ltc0_ltc0_pka1_15::LTC0_LTC0_PKA1_15_SPEC>)
        }
    }
    #[doc = "0x880 - LTC PKHA A 32 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_32(&self) -> &crate::Reg<ltc0_ltc0_pka_32::LTC0_LTC0_PKA_32_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2176usize)
                as *const crate::Reg<ltc0_ltc0_pka_32::LTC0_LTC0_PKA_32_SPEC>)
        }
    }
    #[doc = "0x880 - LTC PKHA A2 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_0(&self) -> &crate::Reg<ltc0_ltc0_pka2_0::LTC0_LTC0_PKA2_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2176usize)
                as *const crate::Reg<ltc0_ltc0_pka2_0::LTC0_LTC0_PKA2_0_SPEC>)
        }
    }
    #[doc = "0x884 - LTC PKHA A 33 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_33(&self) -> &crate::Reg<ltc0_ltc0_pka_33::LTC0_LTC0_PKA_33_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2180usize)
                as *const crate::Reg<ltc0_ltc0_pka_33::LTC0_LTC0_PKA_33_SPEC>)
        }
    }
    #[doc = "0x884 - LTC PKHA A2 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_1(&self) -> &crate::Reg<ltc0_ltc0_pka2_1::LTC0_LTC0_PKA2_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2180usize)
                as *const crate::Reg<ltc0_ltc0_pka2_1::LTC0_LTC0_PKA2_1_SPEC>)
        }
    }
    #[doc = "0x888 - LTC PKHA A 34 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_34(&self) -> &crate::Reg<ltc0_ltc0_pka_34::LTC0_LTC0_PKA_34_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2184usize)
                as *const crate::Reg<ltc0_ltc0_pka_34::LTC0_LTC0_PKA_34_SPEC>)
        }
    }
    #[doc = "0x888 - LTC PKHA A2 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_2(&self) -> &crate::Reg<ltc0_ltc0_pka2_2::LTC0_LTC0_PKA2_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2184usize)
                as *const crate::Reg<ltc0_ltc0_pka2_2::LTC0_LTC0_PKA2_2_SPEC>)
        }
    }
    #[doc = "0x88c - LTC PKHA A 35 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_35(&self) -> &crate::Reg<ltc0_ltc0_pka_35::LTC0_LTC0_PKA_35_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2188usize)
                as *const crate::Reg<ltc0_ltc0_pka_35::LTC0_LTC0_PKA_35_SPEC>)
        }
    }
    #[doc = "0x88c - LTC PKHA A2 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_3(&self) -> &crate::Reg<ltc0_ltc0_pka2_3::LTC0_LTC0_PKA2_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2188usize)
                as *const crate::Reg<ltc0_ltc0_pka2_3::LTC0_LTC0_PKA2_3_SPEC>)
        }
    }
    #[doc = "0x890 - LTC PKHA A 36 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_36(&self) -> &crate::Reg<ltc0_ltc0_pka_36::LTC0_LTC0_PKA_36_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2192usize)
                as *const crate::Reg<ltc0_ltc0_pka_36::LTC0_LTC0_PKA_36_SPEC>)
        }
    }
    #[doc = "0x890 - LTC PKHA A2 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_4(&self) -> &crate::Reg<ltc0_ltc0_pka2_4::LTC0_LTC0_PKA2_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2192usize)
                as *const crate::Reg<ltc0_ltc0_pka2_4::LTC0_LTC0_PKA2_4_SPEC>)
        }
    }
    #[doc = "0x894 - LTC PKHA A 37 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_37(&self) -> &crate::Reg<ltc0_ltc0_pka_37::LTC0_LTC0_PKA_37_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2196usize)
                as *const crate::Reg<ltc0_ltc0_pka_37::LTC0_LTC0_PKA_37_SPEC>)
        }
    }
    #[doc = "0x894 - LTC PKHA A2 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_5(&self) -> &crate::Reg<ltc0_ltc0_pka2_5::LTC0_LTC0_PKA2_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2196usize)
                as *const crate::Reg<ltc0_ltc0_pka2_5::LTC0_LTC0_PKA2_5_SPEC>)
        }
    }
    #[doc = "0x898 - LTC PKHA A 38 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_38(&self) -> &crate::Reg<ltc0_ltc0_pka_38::LTC0_LTC0_PKA_38_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2200usize)
                as *const crate::Reg<ltc0_ltc0_pka_38::LTC0_LTC0_PKA_38_SPEC>)
        }
    }
    #[doc = "0x898 - LTC PKHA A2 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_6(&self) -> &crate::Reg<ltc0_ltc0_pka2_6::LTC0_LTC0_PKA2_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2200usize)
                as *const crate::Reg<ltc0_ltc0_pka2_6::LTC0_LTC0_PKA2_6_SPEC>)
        }
    }
    #[doc = "0x89c - LTC PKHA A 39 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_39(&self) -> &crate::Reg<ltc0_ltc0_pka_39::LTC0_LTC0_PKA_39_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2204usize)
                as *const crate::Reg<ltc0_ltc0_pka_39::LTC0_LTC0_PKA_39_SPEC>)
        }
    }
    #[doc = "0x89c - LTC PKHA A2 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_7(&self) -> &crate::Reg<ltc0_ltc0_pka2_7::LTC0_LTC0_PKA2_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2204usize)
                as *const crate::Reg<ltc0_ltc0_pka2_7::LTC0_LTC0_PKA2_7_SPEC>)
        }
    }
    #[doc = "0x8a0 - LTC PKHA A 40 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_40(&self) -> &crate::Reg<ltc0_ltc0_pka_40::LTC0_LTC0_PKA_40_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2208usize)
                as *const crate::Reg<ltc0_ltc0_pka_40::LTC0_LTC0_PKA_40_SPEC>)
        }
    }
    #[doc = "0x8a0 - LTC PKHA A2 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_8(&self) -> &crate::Reg<ltc0_ltc0_pka2_8::LTC0_LTC0_PKA2_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2208usize)
                as *const crate::Reg<ltc0_ltc0_pka2_8::LTC0_LTC0_PKA2_8_SPEC>)
        }
    }
    #[doc = "0x8a4 - LTC PKHA A 41 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_41(&self) -> &crate::Reg<ltc0_ltc0_pka_41::LTC0_LTC0_PKA_41_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2212usize)
                as *const crate::Reg<ltc0_ltc0_pka_41::LTC0_LTC0_PKA_41_SPEC>)
        }
    }
    #[doc = "0x8a4 - LTC PKHA A2 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_9(&self) -> &crate::Reg<ltc0_ltc0_pka2_9::LTC0_LTC0_PKA2_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2212usize)
                as *const crate::Reg<ltc0_ltc0_pka2_9::LTC0_LTC0_PKA2_9_SPEC>)
        }
    }
    #[doc = "0x8a8 - LTC PKHA A 42 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_42(&self) -> &crate::Reg<ltc0_ltc0_pka_42::LTC0_LTC0_PKA_42_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2216usize)
                as *const crate::Reg<ltc0_ltc0_pka_42::LTC0_LTC0_PKA_42_SPEC>)
        }
    }
    #[doc = "0x8a8 - LTC PKHA A2 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_10(&self) -> &crate::Reg<ltc0_ltc0_pka2_10::LTC0_LTC0_PKA2_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2216usize)
                as *const crate::Reg<ltc0_ltc0_pka2_10::LTC0_LTC0_PKA2_10_SPEC>)
        }
    }
    #[doc = "0x8ac - LTC PKHA A 43 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_43(&self) -> &crate::Reg<ltc0_ltc0_pka_43::LTC0_LTC0_PKA_43_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2220usize)
                as *const crate::Reg<ltc0_ltc0_pka_43::LTC0_LTC0_PKA_43_SPEC>)
        }
    }
    #[doc = "0x8ac - LTC PKHA A2 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_11(&self) -> &crate::Reg<ltc0_ltc0_pka2_11::LTC0_LTC0_PKA2_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2220usize)
                as *const crate::Reg<ltc0_ltc0_pka2_11::LTC0_LTC0_PKA2_11_SPEC>)
        }
    }
    #[doc = "0x8b0 - LTC PKHA A 44 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_44(&self) -> &crate::Reg<ltc0_ltc0_pka_44::LTC0_LTC0_PKA_44_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2224usize)
                as *const crate::Reg<ltc0_ltc0_pka_44::LTC0_LTC0_PKA_44_SPEC>)
        }
    }
    #[doc = "0x8b0 - LTC PKHA A2 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_12(&self) -> &crate::Reg<ltc0_ltc0_pka2_12::LTC0_LTC0_PKA2_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2224usize)
                as *const crate::Reg<ltc0_ltc0_pka2_12::LTC0_LTC0_PKA2_12_SPEC>)
        }
    }
    #[doc = "0x8b4 - LTC PKHA A 45 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_45(&self) -> &crate::Reg<ltc0_ltc0_pka_45::LTC0_LTC0_PKA_45_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2228usize)
                as *const crate::Reg<ltc0_ltc0_pka_45::LTC0_LTC0_PKA_45_SPEC>)
        }
    }
    #[doc = "0x8b4 - LTC PKHA A2 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_13(&self) -> &crate::Reg<ltc0_ltc0_pka2_13::LTC0_LTC0_PKA2_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2228usize)
                as *const crate::Reg<ltc0_ltc0_pka2_13::LTC0_LTC0_PKA2_13_SPEC>)
        }
    }
    #[doc = "0x8b8 - LTC PKHA A 46 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_46(&self) -> &crate::Reg<ltc0_ltc0_pka_46::LTC0_LTC0_PKA_46_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2232usize)
                as *const crate::Reg<ltc0_ltc0_pka_46::LTC0_LTC0_PKA_46_SPEC>)
        }
    }
    #[doc = "0x8b8 - LTC PKHA A2 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_14(&self) -> &crate::Reg<ltc0_ltc0_pka2_14::LTC0_LTC0_PKA2_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2232usize)
                as *const crate::Reg<ltc0_ltc0_pka2_14::LTC0_LTC0_PKA2_14_SPEC>)
        }
    }
    #[doc = "0x8bc - LTC PKHA A 47 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_47(&self) -> &crate::Reg<ltc0_ltc0_pka_47::LTC0_LTC0_PKA_47_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2236usize)
                as *const crate::Reg<ltc0_ltc0_pka_47::LTC0_LTC0_PKA_47_SPEC>)
        }
    }
    #[doc = "0x8bc - LTC PKHA A2 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka2_15(&self) -> &crate::Reg<ltc0_ltc0_pka2_15::LTC0_LTC0_PKA2_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2236usize)
                as *const crate::Reg<ltc0_ltc0_pka2_15::LTC0_LTC0_PKA2_15_SPEC>)
        }
    }
    #[doc = "0x8c0 - LTC PKHA A 48 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_48(&self) -> &crate::Reg<ltc0_ltc0_pka_48::LTC0_LTC0_PKA_48_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2240usize)
                as *const crate::Reg<ltc0_ltc0_pka_48::LTC0_LTC0_PKA_48_SPEC>)
        }
    }
    #[doc = "0x8c0 - LTC PKHA A3 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_0(&self) -> &crate::Reg<ltc0_ltc0_pka3_0::LTC0_LTC0_PKA3_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2240usize)
                as *const crate::Reg<ltc0_ltc0_pka3_0::LTC0_LTC0_PKA3_0_SPEC>)
        }
    }
    #[doc = "0x8c4 - LTC PKHA A 49 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_49(&self) -> &crate::Reg<ltc0_ltc0_pka_49::LTC0_LTC0_PKA_49_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2244usize)
                as *const crate::Reg<ltc0_ltc0_pka_49::LTC0_LTC0_PKA_49_SPEC>)
        }
    }
    #[doc = "0x8c4 - LTC PKHA A3 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_1(&self) -> &crate::Reg<ltc0_ltc0_pka3_1::LTC0_LTC0_PKA3_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2244usize)
                as *const crate::Reg<ltc0_ltc0_pka3_1::LTC0_LTC0_PKA3_1_SPEC>)
        }
    }
    #[doc = "0x8c8 - LTC PKHA A 50 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_50(&self) -> &crate::Reg<ltc0_ltc0_pka_50::LTC0_LTC0_PKA_50_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2248usize)
                as *const crate::Reg<ltc0_ltc0_pka_50::LTC0_LTC0_PKA_50_SPEC>)
        }
    }
    #[doc = "0x8c8 - LTC PKHA A3 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_2(&self) -> &crate::Reg<ltc0_ltc0_pka3_2::LTC0_LTC0_PKA3_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2248usize)
                as *const crate::Reg<ltc0_ltc0_pka3_2::LTC0_LTC0_PKA3_2_SPEC>)
        }
    }
    #[doc = "0x8cc - LTC PKHA A 51 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_51(&self) -> &crate::Reg<ltc0_ltc0_pka_51::LTC0_LTC0_PKA_51_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2252usize)
                as *const crate::Reg<ltc0_ltc0_pka_51::LTC0_LTC0_PKA_51_SPEC>)
        }
    }
    #[doc = "0x8cc - LTC PKHA A3 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_3(&self) -> &crate::Reg<ltc0_ltc0_pka3_3::LTC0_LTC0_PKA3_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2252usize)
                as *const crate::Reg<ltc0_ltc0_pka3_3::LTC0_LTC0_PKA3_3_SPEC>)
        }
    }
    #[doc = "0x8d0 - LTC PKHA A 52 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_52(&self) -> &crate::Reg<ltc0_ltc0_pka_52::LTC0_LTC0_PKA_52_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2256usize)
                as *const crate::Reg<ltc0_ltc0_pka_52::LTC0_LTC0_PKA_52_SPEC>)
        }
    }
    #[doc = "0x8d0 - LTC PKHA A3 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_4(&self) -> &crate::Reg<ltc0_ltc0_pka3_4::LTC0_LTC0_PKA3_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2256usize)
                as *const crate::Reg<ltc0_ltc0_pka3_4::LTC0_LTC0_PKA3_4_SPEC>)
        }
    }
    #[doc = "0x8d4 - LTC PKHA A 53 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_53(&self) -> &crate::Reg<ltc0_ltc0_pka_53::LTC0_LTC0_PKA_53_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2260usize)
                as *const crate::Reg<ltc0_ltc0_pka_53::LTC0_LTC0_PKA_53_SPEC>)
        }
    }
    #[doc = "0x8d4 - LTC PKHA A3 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_5(&self) -> &crate::Reg<ltc0_ltc0_pka3_5::LTC0_LTC0_PKA3_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2260usize)
                as *const crate::Reg<ltc0_ltc0_pka3_5::LTC0_LTC0_PKA3_5_SPEC>)
        }
    }
    #[doc = "0x8d8 - LTC PKHA A 54 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_54(&self) -> &crate::Reg<ltc0_ltc0_pka_54::LTC0_LTC0_PKA_54_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2264usize)
                as *const crate::Reg<ltc0_ltc0_pka_54::LTC0_LTC0_PKA_54_SPEC>)
        }
    }
    #[doc = "0x8d8 - LTC PKHA A3 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_6(&self) -> &crate::Reg<ltc0_ltc0_pka3_6::LTC0_LTC0_PKA3_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2264usize)
                as *const crate::Reg<ltc0_ltc0_pka3_6::LTC0_LTC0_PKA3_6_SPEC>)
        }
    }
    #[doc = "0x8dc - LTC PKHA A 55 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_55(&self) -> &crate::Reg<ltc0_ltc0_pka_55::LTC0_LTC0_PKA_55_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2268usize)
                as *const crate::Reg<ltc0_ltc0_pka_55::LTC0_LTC0_PKA_55_SPEC>)
        }
    }
    #[doc = "0x8dc - LTC PKHA A3 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_7(&self) -> &crate::Reg<ltc0_ltc0_pka3_7::LTC0_LTC0_PKA3_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2268usize)
                as *const crate::Reg<ltc0_ltc0_pka3_7::LTC0_LTC0_PKA3_7_SPEC>)
        }
    }
    #[doc = "0x8e0 - LTC PKHA A 56 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_56(&self) -> &crate::Reg<ltc0_ltc0_pka_56::LTC0_LTC0_PKA_56_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2272usize)
                as *const crate::Reg<ltc0_ltc0_pka_56::LTC0_LTC0_PKA_56_SPEC>)
        }
    }
    #[doc = "0x8e0 - LTC PKHA A3 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_8(&self) -> &crate::Reg<ltc0_ltc0_pka3_8::LTC0_LTC0_PKA3_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2272usize)
                as *const crate::Reg<ltc0_ltc0_pka3_8::LTC0_LTC0_PKA3_8_SPEC>)
        }
    }
    #[doc = "0x8e4 - LTC PKHA A 57 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_57(&self) -> &crate::Reg<ltc0_ltc0_pka_57::LTC0_LTC0_PKA_57_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2276usize)
                as *const crate::Reg<ltc0_ltc0_pka_57::LTC0_LTC0_PKA_57_SPEC>)
        }
    }
    #[doc = "0x8e4 - LTC PKHA A3 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_9(&self) -> &crate::Reg<ltc0_ltc0_pka3_9::LTC0_LTC0_PKA3_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2276usize)
                as *const crate::Reg<ltc0_ltc0_pka3_9::LTC0_LTC0_PKA3_9_SPEC>)
        }
    }
    #[doc = "0x8e8 - LTC PKHA A 58 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_58(&self) -> &crate::Reg<ltc0_ltc0_pka_58::LTC0_LTC0_PKA_58_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2280usize)
                as *const crate::Reg<ltc0_ltc0_pka_58::LTC0_LTC0_PKA_58_SPEC>)
        }
    }
    #[doc = "0x8e8 - LTC PKHA A3 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_10(&self) -> &crate::Reg<ltc0_ltc0_pka3_10::LTC0_LTC0_PKA3_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2280usize)
                as *const crate::Reg<ltc0_ltc0_pka3_10::LTC0_LTC0_PKA3_10_SPEC>)
        }
    }
    #[doc = "0x8ec - LTC PKHA A 59 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_59(&self) -> &crate::Reg<ltc0_ltc0_pka_59::LTC0_LTC0_PKA_59_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2284usize)
                as *const crate::Reg<ltc0_ltc0_pka_59::LTC0_LTC0_PKA_59_SPEC>)
        }
    }
    #[doc = "0x8ec - LTC PKHA A3 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_11(&self) -> &crate::Reg<ltc0_ltc0_pka3_11::LTC0_LTC0_PKA3_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2284usize)
                as *const crate::Reg<ltc0_ltc0_pka3_11::LTC0_LTC0_PKA3_11_SPEC>)
        }
    }
    #[doc = "0x8f0 - LTC PKHA A 60 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_60(&self) -> &crate::Reg<ltc0_ltc0_pka_60::LTC0_LTC0_PKA_60_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2288usize)
                as *const crate::Reg<ltc0_ltc0_pka_60::LTC0_LTC0_PKA_60_SPEC>)
        }
    }
    #[doc = "0x8f0 - LTC PKHA A3 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_12(&self) -> &crate::Reg<ltc0_ltc0_pka3_12::LTC0_LTC0_PKA3_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2288usize)
                as *const crate::Reg<ltc0_ltc0_pka3_12::LTC0_LTC0_PKA3_12_SPEC>)
        }
    }
    #[doc = "0x8f4 - LTC PKHA A 61 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_61(&self) -> &crate::Reg<ltc0_ltc0_pka_61::LTC0_LTC0_PKA_61_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2292usize)
                as *const crate::Reg<ltc0_ltc0_pka_61::LTC0_LTC0_PKA_61_SPEC>)
        }
    }
    #[doc = "0x8f4 - LTC PKHA A3 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_13(&self) -> &crate::Reg<ltc0_ltc0_pka3_13::LTC0_LTC0_PKA3_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2292usize)
                as *const crate::Reg<ltc0_ltc0_pka3_13::LTC0_LTC0_PKA3_13_SPEC>)
        }
    }
    #[doc = "0x8f8 - LTC PKHA A 62 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_62(&self) -> &crate::Reg<ltc0_ltc0_pka_62::LTC0_LTC0_PKA_62_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2296usize)
                as *const crate::Reg<ltc0_ltc0_pka_62::LTC0_LTC0_PKA_62_SPEC>)
        }
    }
    #[doc = "0x8f8 - LTC PKHA A3 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_14(&self) -> &crate::Reg<ltc0_ltc0_pka3_14::LTC0_LTC0_PKA3_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2296usize)
                as *const crate::Reg<ltc0_ltc0_pka3_14::LTC0_LTC0_PKA3_14_SPEC>)
        }
    }
    #[doc = "0x8fc - LTC PKHA A 63 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka_63(&self) -> &crate::Reg<ltc0_ltc0_pka_63::LTC0_LTC0_PKA_63_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2300usize)
                as *const crate::Reg<ltc0_ltc0_pka_63::LTC0_LTC0_PKA_63_SPEC>)
        }
    }
    #[doc = "0x8fc - LTC PKHA A3 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pka3_15(&self) -> &crate::Reg<ltc0_ltc0_pka3_15::LTC0_LTC0_PKA3_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2300usize)
                as *const crate::Reg<ltc0_ltc0_pka3_15::LTC0_LTC0_PKA3_15_SPEC>)
        }
    }
    #[doc = "0xa00 - LTC PKHA B 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_0(&self) -> &crate::Reg<ltc0_ltc0_pkb_0::LTC0_LTC0_PKB_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2560usize)
                as *const crate::Reg<ltc0_ltc0_pkb_0::LTC0_LTC0_PKB_0_SPEC>)
        }
    }
    #[doc = "0xa00 - LTC PKHA B0 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_0(&self) -> &crate::Reg<ltc0_ltc0_pkb0_0::LTC0_LTC0_PKB0_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2560usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_0::LTC0_LTC0_PKB0_0_SPEC>)
        }
    }
    #[doc = "0xa04 - LTC PKHA B 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_1(&self) -> &crate::Reg<ltc0_ltc0_pkb_1::LTC0_LTC0_PKB_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2564usize)
                as *const crate::Reg<ltc0_ltc0_pkb_1::LTC0_LTC0_PKB_1_SPEC>)
        }
    }
    #[doc = "0xa04 - LTC PKHA B0 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_1(&self) -> &crate::Reg<ltc0_ltc0_pkb0_1::LTC0_LTC0_PKB0_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2564usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_1::LTC0_LTC0_PKB0_1_SPEC>)
        }
    }
    #[doc = "0xa08 - LTC PKHA B 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_2(&self) -> &crate::Reg<ltc0_ltc0_pkb_2::LTC0_LTC0_PKB_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2568usize)
                as *const crate::Reg<ltc0_ltc0_pkb_2::LTC0_LTC0_PKB_2_SPEC>)
        }
    }
    #[doc = "0xa08 - LTC PKHA B0 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_2(&self) -> &crate::Reg<ltc0_ltc0_pkb0_2::LTC0_LTC0_PKB0_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2568usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_2::LTC0_LTC0_PKB0_2_SPEC>)
        }
    }
    #[doc = "0xa0c - LTC PKHA B 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_3(&self) -> &crate::Reg<ltc0_ltc0_pkb_3::LTC0_LTC0_PKB_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2572usize)
                as *const crate::Reg<ltc0_ltc0_pkb_3::LTC0_LTC0_PKB_3_SPEC>)
        }
    }
    #[doc = "0xa0c - LTC PKHA B0 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_3(&self) -> &crate::Reg<ltc0_ltc0_pkb0_3::LTC0_LTC0_PKB0_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2572usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_3::LTC0_LTC0_PKB0_3_SPEC>)
        }
    }
    #[doc = "0xa10 - LTC PKHA B 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_4(&self) -> &crate::Reg<ltc0_ltc0_pkb_4::LTC0_LTC0_PKB_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2576usize)
                as *const crate::Reg<ltc0_ltc0_pkb_4::LTC0_LTC0_PKB_4_SPEC>)
        }
    }
    #[doc = "0xa10 - LTC PKHA B0 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_4(&self) -> &crate::Reg<ltc0_ltc0_pkb0_4::LTC0_LTC0_PKB0_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2576usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_4::LTC0_LTC0_PKB0_4_SPEC>)
        }
    }
    #[doc = "0xa14 - LTC PKHA B 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_5(&self) -> &crate::Reg<ltc0_ltc0_pkb_5::LTC0_LTC0_PKB_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2580usize)
                as *const crate::Reg<ltc0_ltc0_pkb_5::LTC0_LTC0_PKB_5_SPEC>)
        }
    }
    #[doc = "0xa14 - LTC PKHA B0 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_5(&self) -> &crate::Reg<ltc0_ltc0_pkb0_5::LTC0_LTC0_PKB0_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2580usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_5::LTC0_LTC0_PKB0_5_SPEC>)
        }
    }
    #[doc = "0xa18 - LTC PKHA B 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_6(&self) -> &crate::Reg<ltc0_ltc0_pkb_6::LTC0_LTC0_PKB_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2584usize)
                as *const crate::Reg<ltc0_ltc0_pkb_6::LTC0_LTC0_PKB_6_SPEC>)
        }
    }
    #[doc = "0xa18 - LTC PKHA B0 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_6(&self) -> &crate::Reg<ltc0_ltc0_pkb0_6::LTC0_LTC0_PKB0_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2584usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_6::LTC0_LTC0_PKB0_6_SPEC>)
        }
    }
    #[doc = "0xa1c - LTC PKHA B 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_7(&self) -> &crate::Reg<ltc0_ltc0_pkb_7::LTC0_LTC0_PKB_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2588usize)
                as *const crate::Reg<ltc0_ltc0_pkb_7::LTC0_LTC0_PKB_7_SPEC>)
        }
    }
    #[doc = "0xa1c - LTC PKHA B0 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_7(&self) -> &crate::Reg<ltc0_ltc0_pkb0_7::LTC0_LTC0_PKB0_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2588usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_7::LTC0_LTC0_PKB0_7_SPEC>)
        }
    }
    #[doc = "0xa20 - LTC PKHA B 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_8(&self) -> &crate::Reg<ltc0_ltc0_pkb_8::LTC0_LTC0_PKB_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2592usize)
                as *const crate::Reg<ltc0_ltc0_pkb_8::LTC0_LTC0_PKB_8_SPEC>)
        }
    }
    #[doc = "0xa20 - LTC PKHA B0 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_8(&self) -> &crate::Reg<ltc0_ltc0_pkb0_8::LTC0_LTC0_PKB0_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2592usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_8::LTC0_LTC0_PKB0_8_SPEC>)
        }
    }
    #[doc = "0xa24 - LTC PKHA B 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_9(&self) -> &crate::Reg<ltc0_ltc0_pkb_9::LTC0_LTC0_PKB_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2596usize)
                as *const crate::Reg<ltc0_ltc0_pkb_9::LTC0_LTC0_PKB_9_SPEC>)
        }
    }
    #[doc = "0xa24 - LTC PKHA B0 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_9(&self) -> &crate::Reg<ltc0_ltc0_pkb0_9::LTC0_LTC0_PKB0_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2596usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_9::LTC0_LTC0_PKB0_9_SPEC>)
        }
    }
    #[doc = "0xa28 - LTC PKHA B 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_10(&self) -> &crate::Reg<ltc0_ltc0_pkb_10::LTC0_LTC0_PKB_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2600usize)
                as *const crate::Reg<ltc0_ltc0_pkb_10::LTC0_LTC0_PKB_10_SPEC>)
        }
    }
    #[doc = "0xa28 - LTC PKHA B0 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_10(&self) -> &crate::Reg<ltc0_ltc0_pkb0_10::LTC0_LTC0_PKB0_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2600usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_10::LTC0_LTC0_PKB0_10_SPEC>)
        }
    }
    #[doc = "0xa2c - LTC PKHA B 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_11(&self) -> &crate::Reg<ltc0_ltc0_pkb_11::LTC0_LTC0_PKB_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2604usize)
                as *const crate::Reg<ltc0_ltc0_pkb_11::LTC0_LTC0_PKB_11_SPEC>)
        }
    }
    #[doc = "0xa2c - LTC PKHA B0 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_11(&self) -> &crate::Reg<ltc0_ltc0_pkb0_11::LTC0_LTC0_PKB0_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2604usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_11::LTC0_LTC0_PKB0_11_SPEC>)
        }
    }
    #[doc = "0xa30 - LTC PKHA B 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_12(&self) -> &crate::Reg<ltc0_ltc0_pkb_12::LTC0_LTC0_PKB_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2608usize)
                as *const crate::Reg<ltc0_ltc0_pkb_12::LTC0_LTC0_PKB_12_SPEC>)
        }
    }
    #[doc = "0xa30 - LTC PKHA B0 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_12(&self) -> &crate::Reg<ltc0_ltc0_pkb0_12::LTC0_LTC0_PKB0_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2608usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_12::LTC0_LTC0_PKB0_12_SPEC>)
        }
    }
    #[doc = "0xa34 - LTC PKHA B 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_13(&self) -> &crate::Reg<ltc0_ltc0_pkb_13::LTC0_LTC0_PKB_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2612usize)
                as *const crate::Reg<ltc0_ltc0_pkb_13::LTC0_LTC0_PKB_13_SPEC>)
        }
    }
    #[doc = "0xa34 - LTC PKHA B0 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_13(&self) -> &crate::Reg<ltc0_ltc0_pkb0_13::LTC0_LTC0_PKB0_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2612usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_13::LTC0_LTC0_PKB0_13_SPEC>)
        }
    }
    #[doc = "0xa38 - LTC PKHA B 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_14(&self) -> &crate::Reg<ltc0_ltc0_pkb_14::LTC0_LTC0_PKB_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2616usize)
                as *const crate::Reg<ltc0_ltc0_pkb_14::LTC0_LTC0_PKB_14_SPEC>)
        }
    }
    #[doc = "0xa38 - LTC PKHA B0 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_14(&self) -> &crate::Reg<ltc0_ltc0_pkb0_14::LTC0_LTC0_PKB0_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2616usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_14::LTC0_LTC0_PKB0_14_SPEC>)
        }
    }
    #[doc = "0xa3c - LTC PKHA B 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_15(&self) -> &crate::Reg<ltc0_ltc0_pkb_15::LTC0_LTC0_PKB_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2620usize)
                as *const crate::Reg<ltc0_ltc0_pkb_15::LTC0_LTC0_PKB_15_SPEC>)
        }
    }
    #[doc = "0xa3c - LTC PKHA B0 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb0_15(&self) -> &crate::Reg<ltc0_ltc0_pkb0_15::LTC0_LTC0_PKB0_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2620usize)
                as *const crate::Reg<ltc0_ltc0_pkb0_15::LTC0_LTC0_PKB0_15_SPEC>)
        }
    }
    #[doc = "0xa40 - LTC PKHA B 16 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_16(&self) -> &crate::Reg<ltc0_ltc0_pkb_16::LTC0_LTC0_PKB_16_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2624usize)
                as *const crate::Reg<ltc0_ltc0_pkb_16::LTC0_LTC0_PKB_16_SPEC>)
        }
    }
    #[doc = "0xa40 - LTC PKHA B1 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_0(&self) -> &crate::Reg<ltc0_ltc0_pkb1_0::LTC0_LTC0_PKB1_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2624usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_0::LTC0_LTC0_PKB1_0_SPEC>)
        }
    }
    #[doc = "0xa44 - LTC PKHA B 17 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_17(&self) -> &crate::Reg<ltc0_ltc0_pkb_17::LTC0_LTC0_PKB_17_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2628usize)
                as *const crate::Reg<ltc0_ltc0_pkb_17::LTC0_LTC0_PKB_17_SPEC>)
        }
    }
    #[doc = "0xa44 - LTC PKHA B1 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_1(&self) -> &crate::Reg<ltc0_ltc0_pkb1_1::LTC0_LTC0_PKB1_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2628usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_1::LTC0_LTC0_PKB1_1_SPEC>)
        }
    }
    #[doc = "0xa48 - LTC PKHA B 18 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_18(&self) -> &crate::Reg<ltc0_ltc0_pkb_18::LTC0_LTC0_PKB_18_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2632usize)
                as *const crate::Reg<ltc0_ltc0_pkb_18::LTC0_LTC0_PKB_18_SPEC>)
        }
    }
    #[doc = "0xa48 - LTC PKHA B1 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_2(&self) -> &crate::Reg<ltc0_ltc0_pkb1_2::LTC0_LTC0_PKB1_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2632usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_2::LTC0_LTC0_PKB1_2_SPEC>)
        }
    }
    #[doc = "0xa4c - LTC PKHA B 19 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_19(&self) -> &crate::Reg<ltc0_ltc0_pkb_19::LTC0_LTC0_PKB_19_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2636usize)
                as *const crate::Reg<ltc0_ltc0_pkb_19::LTC0_LTC0_PKB_19_SPEC>)
        }
    }
    #[doc = "0xa4c - LTC PKHA B1 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_3(&self) -> &crate::Reg<ltc0_ltc0_pkb1_3::LTC0_LTC0_PKB1_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2636usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_3::LTC0_LTC0_PKB1_3_SPEC>)
        }
    }
    #[doc = "0xa50 - LTC PKHA B 20 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_20(&self) -> &crate::Reg<ltc0_ltc0_pkb_20::LTC0_LTC0_PKB_20_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2640usize)
                as *const crate::Reg<ltc0_ltc0_pkb_20::LTC0_LTC0_PKB_20_SPEC>)
        }
    }
    #[doc = "0xa50 - LTC PKHA B1 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_4(&self) -> &crate::Reg<ltc0_ltc0_pkb1_4::LTC0_LTC0_PKB1_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2640usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_4::LTC0_LTC0_PKB1_4_SPEC>)
        }
    }
    #[doc = "0xa54 - LTC PKHA B 21 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_21(&self) -> &crate::Reg<ltc0_ltc0_pkb_21::LTC0_LTC0_PKB_21_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2644usize)
                as *const crate::Reg<ltc0_ltc0_pkb_21::LTC0_LTC0_PKB_21_SPEC>)
        }
    }
    #[doc = "0xa54 - LTC PKHA B1 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_5(&self) -> &crate::Reg<ltc0_ltc0_pkb1_5::LTC0_LTC0_PKB1_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2644usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_5::LTC0_LTC0_PKB1_5_SPEC>)
        }
    }
    #[doc = "0xa58 - LTC PKHA B 22 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_22(&self) -> &crate::Reg<ltc0_ltc0_pkb_22::LTC0_LTC0_PKB_22_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2648usize)
                as *const crate::Reg<ltc0_ltc0_pkb_22::LTC0_LTC0_PKB_22_SPEC>)
        }
    }
    #[doc = "0xa58 - LTC PKHA B1 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_6(&self) -> &crate::Reg<ltc0_ltc0_pkb1_6::LTC0_LTC0_PKB1_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2648usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_6::LTC0_LTC0_PKB1_6_SPEC>)
        }
    }
    #[doc = "0xa5c - LTC PKHA B 23 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_23(&self) -> &crate::Reg<ltc0_ltc0_pkb_23::LTC0_LTC0_PKB_23_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2652usize)
                as *const crate::Reg<ltc0_ltc0_pkb_23::LTC0_LTC0_PKB_23_SPEC>)
        }
    }
    #[doc = "0xa5c - LTC PKHA B1 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_7(&self) -> &crate::Reg<ltc0_ltc0_pkb1_7::LTC0_LTC0_PKB1_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2652usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_7::LTC0_LTC0_PKB1_7_SPEC>)
        }
    }
    #[doc = "0xa60 - LTC PKHA B 24 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_24(&self) -> &crate::Reg<ltc0_ltc0_pkb_24::LTC0_LTC0_PKB_24_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2656usize)
                as *const crate::Reg<ltc0_ltc0_pkb_24::LTC0_LTC0_PKB_24_SPEC>)
        }
    }
    #[doc = "0xa60 - LTC PKHA B1 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_8(&self) -> &crate::Reg<ltc0_ltc0_pkb1_8::LTC0_LTC0_PKB1_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2656usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_8::LTC0_LTC0_PKB1_8_SPEC>)
        }
    }
    #[doc = "0xa64 - LTC PKHA B 25 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_25(&self) -> &crate::Reg<ltc0_ltc0_pkb_25::LTC0_LTC0_PKB_25_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2660usize)
                as *const crate::Reg<ltc0_ltc0_pkb_25::LTC0_LTC0_PKB_25_SPEC>)
        }
    }
    #[doc = "0xa64 - LTC PKHA B1 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_9(&self) -> &crate::Reg<ltc0_ltc0_pkb1_9::LTC0_LTC0_PKB1_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2660usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_9::LTC0_LTC0_PKB1_9_SPEC>)
        }
    }
    #[doc = "0xa68 - LTC PKHA B 26 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_26(&self) -> &crate::Reg<ltc0_ltc0_pkb_26::LTC0_LTC0_PKB_26_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2664usize)
                as *const crate::Reg<ltc0_ltc0_pkb_26::LTC0_LTC0_PKB_26_SPEC>)
        }
    }
    #[doc = "0xa68 - LTC PKHA B1 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_10(&self) -> &crate::Reg<ltc0_ltc0_pkb1_10::LTC0_LTC0_PKB1_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2664usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_10::LTC0_LTC0_PKB1_10_SPEC>)
        }
    }
    #[doc = "0xa6c - LTC PKHA B 27 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_27(&self) -> &crate::Reg<ltc0_ltc0_pkb_27::LTC0_LTC0_PKB_27_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2668usize)
                as *const crate::Reg<ltc0_ltc0_pkb_27::LTC0_LTC0_PKB_27_SPEC>)
        }
    }
    #[doc = "0xa6c - LTC PKHA B1 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_11(&self) -> &crate::Reg<ltc0_ltc0_pkb1_11::LTC0_LTC0_PKB1_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2668usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_11::LTC0_LTC0_PKB1_11_SPEC>)
        }
    }
    #[doc = "0xa70 - LTC PKHA B 28 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_28(&self) -> &crate::Reg<ltc0_ltc0_pkb_28::LTC0_LTC0_PKB_28_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2672usize)
                as *const crate::Reg<ltc0_ltc0_pkb_28::LTC0_LTC0_PKB_28_SPEC>)
        }
    }
    #[doc = "0xa70 - LTC PKHA B1 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_12(&self) -> &crate::Reg<ltc0_ltc0_pkb1_12::LTC0_LTC0_PKB1_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2672usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_12::LTC0_LTC0_PKB1_12_SPEC>)
        }
    }
    #[doc = "0xa74 - LTC PKHA B 29 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_29(&self) -> &crate::Reg<ltc0_ltc0_pkb_29::LTC0_LTC0_PKB_29_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2676usize)
                as *const crate::Reg<ltc0_ltc0_pkb_29::LTC0_LTC0_PKB_29_SPEC>)
        }
    }
    #[doc = "0xa74 - LTC PKHA B1 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_13(&self) -> &crate::Reg<ltc0_ltc0_pkb1_13::LTC0_LTC0_PKB1_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2676usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_13::LTC0_LTC0_PKB1_13_SPEC>)
        }
    }
    #[doc = "0xa78 - LTC PKHA B 30 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_30(&self) -> &crate::Reg<ltc0_ltc0_pkb_30::LTC0_LTC0_PKB_30_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2680usize)
                as *const crate::Reg<ltc0_ltc0_pkb_30::LTC0_LTC0_PKB_30_SPEC>)
        }
    }
    #[doc = "0xa78 - LTC PKHA B1 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_14(&self) -> &crate::Reg<ltc0_ltc0_pkb1_14::LTC0_LTC0_PKB1_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2680usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_14::LTC0_LTC0_PKB1_14_SPEC>)
        }
    }
    #[doc = "0xa7c - LTC PKHA B 31 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_31(&self) -> &crate::Reg<ltc0_ltc0_pkb_31::LTC0_LTC0_PKB_31_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2684usize)
                as *const crate::Reg<ltc0_ltc0_pkb_31::LTC0_LTC0_PKB_31_SPEC>)
        }
    }
    #[doc = "0xa7c - LTC PKHA B1 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb1_15(&self) -> &crate::Reg<ltc0_ltc0_pkb1_15::LTC0_LTC0_PKB1_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2684usize)
                as *const crate::Reg<ltc0_ltc0_pkb1_15::LTC0_LTC0_PKB1_15_SPEC>)
        }
    }
    #[doc = "0xa80 - LTC PKHA B 32 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_32(&self) -> &crate::Reg<ltc0_ltc0_pkb_32::LTC0_LTC0_PKB_32_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2688usize)
                as *const crate::Reg<ltc0_ltc0_pkb_32::LTC0_LTC0_PKB_32_SPEC>)
        }
    }
    #[doc = "0xa80 - LTC PKHA B2 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_0(&self) -> &crate::Reg<ltc0_ltc0_pkb2_0::LTC0_LTC0_PKB2_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2688usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_0::LTC0_LTC0_PKB2_0_SPEC>)
        }
    }
    #[doc = "0xa84 - LTC PKHA B 33 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_33(&self) -> &crate::Reg<ltc0_ltc0_pkb_33::LTC0_LTC0_PKB_33_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2692usize)
                as *const crate::Reg<ltc0_ltc0_pkb_33::LTC0_LTC0_PKB_33_SPEC>)
        }
    }
    #[doc = "0xa84 - LTC PKHA B2 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_1(&self) -> &crate::Reg<ltc0_ltc0_pkb2_1::LTC0_LTC0_PKB2_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2692usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_1::LTC0_LTC0_PKB2_1_SPEC>)
        }
    }
    #[doc = "0xa88 - LTC PKHA B 34 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_34(&self) -> &crate::Reg<ltc0_ltc0_pkb_34::LTC0_LTC0_PKB_34_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2696usize)
                as *const crate::Reg<ltc0_ltc0_pkb_34::LTC0_LTC0_PKB_34_SPEC>)
        }
    }
    #[doc = "0xa88 - LTC PKHA B2 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_2(&self) -> &crate::Reg<ltc0_ltc0_pkb2_2::LTC0_LTC0_PKB2_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2696usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_2::LTC0_LTC0_PKB2_2_SPEC>)
        }
    }
    #[doc = "0xa8c - LTC PKHA B 35 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_35(&self) -> &crate::Reg<ltc0_ltc0_pkb_35::LTC0_LTC0_PKB_35_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2700usize)
                as *const crate::Reg<ltc0_ltc0_pkb_35::LTC0_LTC0_PKB_35_SPEC>)
        }
    }
    #[doc = "0xa8c - LTC PKHA B2 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_3(&self) -> &crate::Reg<ltc0_ltc0_pkb2_3::LTC0_LTC0_PKB2_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2700usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_3::LTC0_LTC0_PKB2_3_SPEC>)
        }
    }
    #[doc = "0xa90 - LTC PKHA B 36 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_36(&self) -> &crate::Reg<ltc0_ltc0_pkb_36::LTC0_LTC0_PKB_36_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2704usize)
                as *const crate::Reg<ltc0_ltc0_pkb_36::LTC0_LTC0_PKB_36_SPEC>)
        }
    }
    #[doc = "0xa90 - LTC PKHA B2 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_4(&self) -> &crate::Reg<ltc0_ltc0_pkb2_4::LTC0_LTC0_PKB2_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2704usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_4::LTC0_LTC0_PKB2_4_SPEC>)
        }
    }
    #[doc = "0xa94 - LTC PKHA B 37 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_37(&self) -> &crate::Reg<ltc0_ltc0_pkb_37::LTC0_LTC0_PKB_37_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2708usize)
                as *const crate::Reg<ltc0_ltc0_pkb_37::LTC0_LTC0_PKB_37_SPEC>)
        }
    }
    #[doc = "0xa94 - LTC PKHA B2 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_5(&self) -> &crate::Reg<ltc0_ltc0_pkb2_5::LTC0_LTC0_PKB2_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2708usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_5::LTC0_LTC0_PKB2_5_SPEC>)
        }
    }
    #[doc = "0xa98 - LTC PKHA B 38 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_38(&self) -> &crate::Reg<ltc0_ltc0_pkb_38::LTC0_LTC0_PKB_38_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2712usize)
                as *const crate::Reg<ltc0_ltc0_pkb_38::LTC0_LTC0_PKB_38_SPEC>)
        }
    }
    #[doc = "0xa98 - LTC PKHA B2 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_6(&self) -> &crate::Reg<ltc0_ltc0_pkb2_6::LTC0_LTC0_PKB2_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2712usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_6::LTC0_LTC0_PKB2_6_SPEC>)
        }
    }
    #[doc = "0xa9c - LTC PKHA B 39 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_39(&self) -> &crate::Reg<ltc0_ltc0_pkb_39::LTC0_LTC0_PKB_39_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2716usize)
                as *const crate::Reg<ltc0_ltc0_pkb_39::LTC0_LTC0_PKB_39_SPEC>)
        }
    }
    #[doc = "0xa9c - LTC PKHA B2 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_7(&self) -> &crate::Reg<ltc0_ltc0_pkb2_7::LTC0_LTC0_PKB2_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2716usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_7::LTC0_LTC0_PKB2_7_SPEC>)
        }
    }
    #[doc = "0xaa0 - LTC PKHA B 40 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_40(&self) -> &crate::Reg<ltc0_ltc0_pkb_40::LTC0_LTC0_PKB_40_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2720usize)
                as *const crate::Reg<ltc0_ltc0_pkb_40::LTC0_LTC0_PKB_40_SPEC>)
        }
    }
    #[doc = "0xaa0 - LTC PKHA B2 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_8(&self) -> &crate::Reg<ltc0_ltc0_pkb2_8::LTC0_LTC0_PKB2_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2720usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_8::LTC0_LTC0_PKB2_8_SPEC>)
        }
    }
    #[doc = "0xaa4 - LTC PKHA B 41 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_41(&self) -> &crate::Reg<ltc0_ltc0_pkb_41::LTC0_LTC0_PKB_41_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2724usize)
                as *const crate::Reg<ltc0_ltc0_pkb_41::LTC0_LTC0_PKB_41_SPEC>)
        }
    }
    #[doc = "0xaa4 - LTC PKHA B2 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_9(&self) -> &crate::Reg<ltc0_ltc0_pkb2_9::LTC0_LTC0_PKB2_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2724usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_9::LTC0_LTC0_PKB2_9_SPEC>)
        }
    }
    #[doc = "0xaa8 - LTC PKHA B 42 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_42(&self) -> &crate::Reg<ltc0_ltc0_pkb_42::LTC0_LTC0_PKB_42_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2728usize)
                as *const crate::Reg<ltc0_ltc0_pkb_42::LTC0_LTC0_PKB_42_SPEC>)
        }
    }
    #[doc = "0xaa8 - LTC PKHA B2 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_10(&self) -> &crate::Reg<ltc0_ltc0_pkb2_10::LTC0_LTC0_PKB2_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2728usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_10::LTC0_LTC0_PKB2_10_SPEC>)
        }
    }
    #[doc = "0xaac - LTC PKHA B 43 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_43(&self) -> &crate::Reg<ltc0_ltc0_pkb_43::LTC0_LTC0_PKB_43_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2732usize)
                as *const crate::Reg<ltc0_ltc0_pkb_43::LTC0_LTC0_PKB_43_SPEC>)
        }
    }
    #[doc = "0xaac - LTC PKHA B2 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_11(&self) -> &crate::Reg<ltc0_ltc0_pkb2_11::LTC0_LTC0_PKB2_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2732usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_11::LTC0_LTC0_PKB2_11_SPEC>)
        }
    }
    #[doc = "0xab0 - LTC PKHA B 44 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_44(&self) -> &crate::Reg<ltc0_ltc0_pkb_44::LTC0_LTC0_PKB_44_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2736usize)
                as *const crate::Reg<ltc0_ltc0_pkb_44::LTC0_LTC0_PKB_44_SPEC>)
        }
    }
    #[doc = "0xab0 - LTC PKHA B2 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_12(&self) -> &crate::Reg<ltc0_ltc0_pkb2_12::LTC0_LTC0_PKB2_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2736usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_12::LTC0_LTC0_PKB2_12_SPEC>)
        }
    }
    #[doc = "0xab4 - LTC PKHA B 45 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_45(&self) -> &crate::Reg<ltc0_ltc0_pkb_45::LTC0_LTC0_PKB_45_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2740usize)
                as *const crate::Reg<ltc0_ltc0_pkb_45::LTC0_LTC0_PKB_45_SPEC>)
        }
    }
    #[doc = "0xab4 - LTC PKHA B2 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_13(&self) -> &crate::Reg<ltc0_ltc0_pkb2_13::LTC0_LTC0_PKB2_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2740usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_13::LTC0_LTC0_PKB2_13_SPEC>)
        }
    }
    #[doc = "0xab8 - LTC PKHA B 46 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_46(&self) -> &crate::Reg<ltc0_ltc0_pkb_46::LTC0_LTC0_PKB_46_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2744usize)
                as *const crate::Reg<ltc0_ltc0_pkb_46::LTC0_LTC0_PKB_46_SPEC>)
        }
    }
    #[doc = "0xab8 - LTC PKHA B2 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_14(&self) -> &crate::Reg<ltc0_ltc0_pkb2_14::LTC0_LTC0_PKB2_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2744usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_14::LTC0_LTC0_PKB2_14_SPEC>)
        }
    }
    #[doc = "0xabc - LTC PKHA B 47 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_47(&self) -> &crate::Reg<ltc0_ltc0_pkb_47::LTC0_LTC0_PKB_47_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2748usize)
                as *const crate::Reg<ltc0_ltc0_pkb_47::LTC0_LTC0_PKB_47_SPEC>)
        }
    }
    #[doc = "0xabc - LTC PKHA B2 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb2_15(&self) -> &crate::Reg<ltc0_ltc0_pkb2_15::LTC0_LTC0_PKB2_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2748usize)
                as *const crate::Reg<ltc0_ltc0_pkb2_15::LTC0_LTC0_PKB2_15_SPEC>)
        }
    }
    #[doc = "0xac0 - LTC PKHA B 48 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_48(&self) -> &crate::Reg<ltc0_ltc0_pkb_48::LTC0_LTC0_PKB_48_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2752usize)
                as *const crate::Reg<ltc0_ltc0_pkb_48::LTC0_LTC0_PKB_48_SPEC>)
        }
    }
    #[doc = "0xac0 - LTC PKHA B3 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_0(&self) -> &crate::Reg<ltc0_ltc0_pkb3_0::LTC0_LTC0_PKB3_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2752usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_0::LTC0_LTC0_PKB3_0_SPEC>)
        }
    }
    #[doc = "0xac4 - LTC PKHA B 49 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_49(&self) -> &crate::Reg<ltc0_ltc0_pkb_49::LTC0_LTC0_PKB_49_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2756usize)
                as *const crate::Reg<ltc0_ltc0_pkb_49::LTC0_LTC0_PKB_49_SPEC>)
        }
    }
    #[doc = "0xac4 - LTC PKHA B3 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_1(&self) -> &crate::Reg<ltc0_ltc0_pkb3_1::LTC0_LTC0_PKB3_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2756usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_1::LTC0_LTC0_PKB3_1_SPEC>)
        }
    }
    #[doc = "0xac8 - LTC PKHA B 50 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_50(&self) -> &crate::Reg<ltc0_ltc0_pkb_50::LTC0_LTC0_PKB_50_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2760usize)
                as *const crate::Reg<ltc0_ltc0_pkb_50::LTC0_LTC0_PKB_50_SPEC>)
        }
    }
    #[doc = "0xac8 - LTC PKHA B3 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_2(&self) -> &crate::Reg<ltc0_ltc0_pkb3_2::LTC0_LTC0_PKB3_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2760usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_2::LTC0_LTC0_PKB3_2_SPEC>)
        }
    }
    #[doc = "0xacc - LTC PKHA B 51 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_51(&self) -> &crate::Reg<ltc0_ltc0_pkb_51::LTC0_LTC0_PKB_51_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2764usize)
                as *const crate::Reg<ltc0_ltc0_pkb_51::LTC0_LTC0_PKB_51_SPEC>)
        }
    }
    #[doc = "0xacc - LTC PKHA B3 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_3(&self) -> &crate::Reg<ltc0_ltc0_pkb3_3::LTC0_LTC0_PKB3_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2764usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_3::LTC0_LTC0_PKB3_3_SPEC>)
        }
    }
    #[doc = "0xad0 - LTC PKHA B 52 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_52(&self) -> &crate::Reg<ltc0_ltc0_pkb_52::LTC0_LTC0_PKB_52_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2768usize)
                as *const crate::Reg<ltc0_ltc0_pkb_52::LTC0_LTC0_PKB_52_SPEC>)
        }
    }
    #[doc = "0xad0 - LTC PKHA B3 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_4(&self) -> &crate::Reg<ltc0_ltc0_pkb3_4::LTC0_LTC0_PKB3_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2768usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_4::LTC0_LTC0_PKB3_4_SPEC>)
        }
    }
    #[doc = "0xad4 - LTC PKHA B 53 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_53(&self) -> &crate::Reg<ltc0_ltc0_pkb_53::LTC0_LTC0_PKB_53_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2772usize)
                as *const crate::Reg<ltc0_ltc0_pkb_53::LTC0_LTC0_PKB_53_SPEC>)
        }
    }
    #[doc = "0xad4 - LTC PKHA B3 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_5(&self) -> &crate::Reg<ltc0_ltc0_pkb3_5::LTC0_LTC0_PKB3_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2772usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_5::LTC0_LTC0_PKB3_5_SPEC>)
        }
    }
    #[doc = "0xad8 - LTC PKHA B 54 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_54(&self) -> &crate::Reg<ltc0_ltc0_pkb_54::LTC0_LTC0_PKB_54_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2776usize)
                as *const crate::Reg<ltc0_ltc0_pkb_54::LTC0_LTC0_PKB_54_SPEC>)
        }
    }
    #[doc = "0xad8 - LTC PKHA B3 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_6(&self) -> &crate::Reg<ltc0_ltc0_pkb3_6::LTC0_LTC0_PKB3_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2776usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_6::LTC0_LTC0_PKB3_6_SPEC>)
        }
    }
    #[doc = "0xadc - LTC PKHA B 55 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_55(&self) -> &crate::Reg<ltc0_ltc0_pkb_55::LTC0_LTC0_PKB_55_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2780usize)
                as *const crate::Reg<ltc0_ltc0_pkb_55::LTC0_LTC0_PKB_55_SPEC>)
        }
    }
    #[doc = "0xadc - LTC PKHA B3 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_7(&self) -> &crate::Reg<ltc0_ltc0_pkb3_7::LTC0_LTC0_PKB3_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2780usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_7::LTC0_LTC0_PKB3_7_SPEC>)
        }
    }
    #[doc = "0xae0 - LTC PKHA B 56 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_56(&self) -> &crate::Reg<ltc0_ltc0_pkb_56::LTC0_LTC0_PKB_56_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2784usize)
                as *const crate::Reg<ltc0_ltc0_pkb_56::LTC0_LTC0_PKB_56_SPEC>)
        }
    }
    #[doc = "0xae0 - LTC PKHA B3 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_8(&self) -> &crate::Reg<ltc0_ltc0_pkb3_8::LTC0_LTC0_PKB3_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2784usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_8::LTC0_LTC0_PKB3_8_SPEC>)
        }
    }
    #[doc = "0xae4 - LTC PKHA B 57 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_57(&self) -> &crate::Reg<ltc0_ltc0_pkb_57::LTC0_LTC0_PKB_57_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2788usize)
                as *const crate::Reg<ltc0_ltc0_pkb_57::LTC0_LTC0_PKB_57_SPEC>)
        }
    }
    #[doc = "0xae4 - LTC PKHA B3 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_9(&self) -> &crate::Reg<ltc0_ltc0_pkb3_9::LTC0_LTC0_PKB3_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2788usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_9::LTC0_LTC0_PKB3_9_SPEC>)
        }
    }
    #[doc = "0xae8 - LTC PKHA B 58 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_58(&self) -> &crate::Reg<ltc0_ltc0_pkb_58::LTC0_LTC0_PKB_58_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2792usize)
                as *const crate::Reg<ltc0_ltc0_pkb_58::LTC0_LTC0_PKB_58_SPEC>)
        }
    }
    #[doc = "0xae8 - LTC PKHA B3 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_10(&self) -> &crate::Reg<ltc0_ltc0_pkb3_10::LTC0_LTC0_PKB3_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2792usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_10::LTC0_LTC0_PKB3_10_SPEC>)
        }
    }
    #[doc = "0xaec - LTC PKHA B 59 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_59(&self) -> &crate::Reg<ltc0_ltc0_pkb_59::LTC0_LTC0_PKB_59_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2796usize)
                as *const crate::Reg<ltc0_ltc0_pkb_59::LTC0_LTC0_PKB_59_SPEC>)
        }
    }
    #[doc = "0xaec - LTC PKHA B3 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_11(&self) -> &crate::Reg<ltc0_ltc0_pkb3_11::LTC0_LTC0_PKB3_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2796usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_11::LTC0_LTC0_PKB3_11_SPEC>)
        }
    }
    #[doc = "0xaf0 - LTC PKHA B 60 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_60(&self) -> &crate::Reg<ltc0_ltc0_pkb_60::LTC0_LTC0_PKB_60_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2800usize)
                as *const crate::Reg<ltc0_ltc0_pkb_60::LTC0_LTC0_PKB_60_SPEC>)
        }
    }
    #[doc = "0xaf0 - LTC PKHA B3 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_12(&self) -> &crate::Reg<ltc0_ltc0_pkb3_12::LTC0_LTC0_PKB3_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2800usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_12::LTC0_LTC0_PKB3_12_SPEC>)
        }
    }
    #[doc = "0xaf4 - LTC PKHA B 61 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_61(&self) -> &crate::Reg<ltc0_ltc0_pkb_61::LTC0_LTC0_PKB_61_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2804usize)
                as *const crate::Reg<ltc0_ltc0_pkb_61::LTC0_LTC0_PKB_61_SPEC>)
        }
    }
    #[doc = "0xaf4 - LTC PKHA B3 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_13(&self) -> &crate::Reg<ltc0_ltc0_pkb3_13::LTC0_LTC0_PKB3_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2804usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_13::LTC0_LTC0_PKB3_13_SPEC>)
        }
    }
    #[doc = "0xaf8 - LTC PKHA B 62 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_62(&self) -> &crate::Reg<ltc0_ltc0_pkb_62::LTC0_LTC0_PKB_62_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2808usize)
                as *const crate::Reg<ltc0_ltc0_pkb_62::LTC0_LTC0_PKB_62_SPEC>)
        }
    }
    #[doc = "0xaf8 - LTC PKHA B3 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_14(&self) -> &crate::Reg<ltc0_ltc0_pkb3_14::LTC0_LTC0_PKB3_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2808usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_14::LTC0_LTC0_PKB3_14_SPEC>)
        }
    }
    #[doc = "0xafc - LTC PKHA B 63 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb_63(&self) -> &crate::Reg<ltc0_ltc0_pkb_63::LTC0_LTC0_PKB_63_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2812usize)
                as *const crate::Reg<ltc0_ltc0_pkb_63::LTC0_LTC0_PKB_63_SPEC>)
        }
    }
    #[doc = "0xafc - LTC PKHA B3 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkb3_15(&self) -> &crate::Reg<ltc0_ltc0_pkb3_15::LTC0_LTC0_PKB3_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2812usize)
                as *const crate::Reg<ltc0_ltc0_pkb3_15::LTC0_LTC0_PKB3_15_SPEC>)
        }
    }
    #[doc = "0xc00 - LTC PKHA N 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_0(&self) -> &crate::Reg<ltc0_ltc0_pkn_0::LTC0_LTC0_PKN_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3072usize)
                as *const crate::Reg<ltc0_ltc0_pkn_0::LTC0_LTC0_PKN_0_SPEC>)
        }
    }
    #[doc = "0xc00 - LTC PKHA N0 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_0(&self) -> &crate::Reg<ltc0_ltc0_pkn0_0::LTC0_LTC0_PKN0_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3072usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_0::LTC0_LTC0_PKN0_0_SPEC>)
        }
    }
    #[doc = "0xc04 - LTC PKHA N 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_1(&self) -> &crate::Reg<ltc0_ltc0_pkn_1::LTC0_LTC0_PKN_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3076usize)
                as *const crate::Reg<ltc0_ltc0_pkn_1::LTC0_LTC0_PKN_1_SPEC>)
        }
    }
    #[doc = "0xc04 - LTC PKHA N0 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_1(&self) -> &crate::Reg<ltc0_ltc0_pkn0_1::LTC0_LTC0_PKN0_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3076usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_1::LTC0_LTC0_PKN0_1_SPEC>)
        }
    }
    #[doc = "0xc08 - LTC PKHA N 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_2(&self) -> &crate::Reg<ltc0_ltc0_pkn_2::LTC0_LTC0_PKN_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3080usize)
                as *const crate::Reg<ltc0_ltc0_pkn_2::LTC0_LTC0_PKN_2_SPEC>)
        }
    }
    #[doc = "0xc08 - LTC PKHA N0 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_2(&self) -> &crate::Reg<ltc0_ltc0_pkn0_2::LTC0_LTC0_PKN0_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3080usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_2::LTC0_LTC0_PKN0_2_SPEC>)
        }
    }
    #[doc = "0xc0c - LTC PKHA N 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_3(&self) -> &crate::Reg<ltc0_ltc0_pkn_3::LTC0_LTC0_PKN_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3084usize)
                as *const crate::Reg<ltc0_ltc0_pkn_3::LTC0_LTC0_PKN_3_SPEC>)
        }
    }
    #[doc = "0xc0c - LTC PKHA N0 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_3(&self) -> &crate::Reg<ltc0_ltc0_pkn0_3::LTC0_LTC0_PKN0_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3084usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_3::LTC0_LTC0_PKN0_3_SPEC>)
        }
    }
    #[doc = "0xc10 - LTC PKHA N 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_4(&self) -> &crate::Reg<ltc0_ltc0_pkn_4::LTC0_LTC0_PKN_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3088usize)
                as *const crate::Reg<ltc0_ltc0_pkn_4::LTC0_LTC0_PKN_4_SPEC>)
        }
    }
    #[doc = "0xc10 - LTC PKHA N0 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_4(&self) -> &crate::Reg<ltc0_ltc0_pkn0_4::LTC0_LTC0_PKN0_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3088usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_4::LTC0_LTC0_PKN0_4_SPEC>)
        }
    }
    #[doc = "0xc14 - LTC PKHA N 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_5(&self) -> &crate::Reg<ltc0_ltc0_pkn_5::LTC0_LTC0_PKN_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3092usize)
                as *const crate::Reg<ltc0_ltc0_pkn_5::LTC0_LTC0_PKN_5_SPEC>)
        }
    }
    #[doc = "0xc14 - LTC PKHA N0 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_5(&self) -> &crate::Reg<ltc0_ltc0_pkn0_5::LTC0_LTC0_PKN0_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3092usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_5::LTC0_LTC0_PKN0_5_SPEC>)
        }
    }
    #[doc = "0xc18 - LTC PKHA N 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_6(&self) -> &crate::Reg<ltc0_ltc0_pkn_6::LTC0_LTC0_PKN_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3096usize)
                as *const crate::Reg<ltc0_ltc0_pkn_6::LTC0_LTC0_PKN_6_SPEC>)
        }
    }
    #[doc = "0xc18 - LTC PKHA N0 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_6(&self) -> &crate::Reg<ltc0_ltc0_pkn0_6::LTC0_LTC0_PKN0_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3096usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_6::LTC0_LTC0_PKN0_6_SPEC>)
        }
    }
    #[doc = "0xc1c - LTC PKHA N 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_7(&self) -> &crate::Reg<ltc0_ltc0_pkn_7::LTC0_LTC0_PKN_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3100usize)
                as *const crate::Reg<ltc0_ltc0_pkn_7::LTC0_LTC0_PKN_7_SPEC>)
        }
    }
    #[doc = "0xc1c - LTC PKHA N0 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_7(&self) -> &crate::Reg<ltc0_ltc0_pkn0_7::LTC0_LTC0_PKN0_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3100usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_7::LTC0_LTC0_PKN0_7_SPEC>)
        }
    }
    #[doc = "0xc20 - LTC PKHA N 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_8(&self) -> &crate::Reg<ltc0_ltc0_pkn_8::LTC0_LTC0_PKN_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3104usize)
                as *const crate::Reg<ltc0_ltc0_pkn_8::LTC0_LTC0_PKN_8_SPEC>)
        }
    }
    #[doc = "0xc20 - LTC PKHA N0 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_8(&self) -> &crate::Reg<ltc0_ltc0_pkn0_8::LTC0_LTC0_PKN0_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3104usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_8::LTC0_LTC0_PKN0_8_SPEC>)
        }
    }
    #[doc = "0xc24 - LTC PKHA N 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_9(&self) -> &crate::Reg<ltc0_ltc0_pkn_9::LTC0_LTC0_PKN_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3108usize)
                as *const crate::Reg<ltc0_ltc0_pkn_9::LTC0_LTC0_PKN_9_SPEC>)
        }
    }
    #[doc = "0xc24 - LTC PKHA N0 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_9(&self) -> &crate::Reg<ltc0_ltc0_pkn0_9::LTC0_LTC0_PKN0_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3108usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_9::LTC0_LTC0_PKN0_9_SPEC>)
        }
    }
    #[doc = "0xc28 - LTC PKHA N 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_10(&self) -> &crate::Reg<ltc0_ltc0_pkn_10::LTC0_LTC0_PKN_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3112usize)
                as *const crate::Reg<ltc0_ltc0_pkn_10::LTC0_LTC0_PKN_10_SPEC>)
        }
    }
    #[doc = "0xc28 - LTC PKHA N0 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_10(&self) -> &crate::Reg<ltc0_ltc0_pkn0_10::LTC0_LTC0_PKN0_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3112usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_10::LTC0_LTC0_PKN0_10_SPEC>)
        }
    }
    #[doc = "0xc2c - LTC PKHA N 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_11(&self) -> &crate::Reg<ltc0_ltc0_pkn_11::LTC0_LTC0_PKN_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3116usize)
                as *const crate::Reg<ltc0_ltc0_pkn_11::LTC0_LTC0_PKN_11_SPEC>)
        }
    }
    #[doc = "0xc2c - LTC PKHA N0 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_11(&self) -> &crate::Reg<ltc0_ltc0_pkn0_11::LTC0_LTC0_PKN0_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3116usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_11::LTC0_LTC0_PKN0_11_SPEC>)
        }
    }
    #[doc = "0xc30 - LTC PKHA N 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_12(&self) -> &crate::Reg<ltc0_ltc0_pkn_12::LTC0_LTC0_PKN_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3120usize)
                as *const crate::Reg<ltc0_ltc0_pkn_12::LTC0_LTC0_PKN_12_SPEC>)
        }
    }
    #[doc = "0xc30 - LTC PKHA N0 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_12(&self) -> &crate::Reg<ltc0_ltc0_pkn0_12::LTC0_LTC0_PKN0_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3120usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_12::LTC0_LTC0_PKN0_12_SPEC>)
        }
    }
    #[doc = "0xc34 - LTC PKHA N 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_13(&self) -> &crate::Reg<ltc0_ltc0_pkn_13::LTC0_LTC0_PKN_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3124usize)
                as *const crate::Reg<ltc0_ltc0_pkn_13::LTC0_LTC0_PKN_13_SPEC>)
        }
    }
    #[doc = "0xc34 - LTC PKHA N0 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_13(&self) -> &crate::Reg<ltc0_ltc0_pkn0_13::LTC0_LTC0_PKN0_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3124usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_13::LTC0_LTC0_PKN0_13_SPEC>)
        }
    }
    #[doc = "0xc38 - LTC PKHA N 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_14(&self) -> &crate::Reg<ltc0_ltc0_pkn_14::LTC0_LTC0_PKN_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3128usize)
                as *const crate::Reg<ltc0_ltc0_pkn_14::LTC0_LTC0_PKN_14_SPEC>)
        }
    }
    #[doc = "0xc38 - LTC PKHA N0 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_14(&self) -> &crate::Reg<ltc0_ltc0_pkn0_14::LTC0_LTC0_PKN0_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3128usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_14::LTC0_LTC0_PKN0_14_SPEC>)
        }
    }
    #[doc = "0xc3c - LTC PKHA N 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_15(&self) -> &crate::Reg<ltc0_ltc0_pkn_15::LTC0_LTC0_PKN_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3132usize)
                as *const crate::Reg<ltc0_ltc0_pkn_15::LTC0_LTC0_PKN_15_SPEC>)
        }
    }
    #[doc = "0xc3c - LTC PKHA N0 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn0_15(&self) -> &crate::Reg<ltc0_ltc0_pkn0_15::LTC0_LTC0_PKN0_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3132usize)
                as *const crate::Reg<ltc0_ltc0_pkn0_15::LTC0_LTC0_PKN0_15_SPEC>)
        }
    }
    #[doc = "0xc40 - LTC PKHA N 16 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_16(&self) -> &crate::Reg<ltc0_ltc0_pkn_16::LTC0_LTC0_PKN_16_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3136usize)
                as *const crate::Reg<ltc0_ltc0_pkn_16::LTC0_LTC0_PKN_16_SPEC>)
        }
    }
    #[doc = "0xc40 - LTC PKHA N1 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_0(&self) -> &crate::Reg<ltc0_ltc0_pkn1_0::LTC0_LTC0_PKN1_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3136usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_0::LTC0_LTC0_PKN1_0_SPEC>)
        }
    }
    #[doc = "0xc44 - LTC PKHA N 17 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_17(&self) -> &crate::Reg<ltc0_ltc0_pkn_17::LTC0_LTC0_PKN_17_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3140usize)
                as *const crate::Reg<ltc0_ltc0_pkn_17::LTC0_LTC0_PKN_17_SPEC>)
        }
    }
    #[doc = "0xc44 - LTC PKHA N1 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_1(&self) -> &crate::Reg<ltc0_ltc0_pkn1_1::LTC0_LTC0_PKN1_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3140usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_1::LTC0_LTC0_PKN1_1_SPEC>)
        }
    }
    #[doc = "0xc48 - LTC PKHA N 18 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_18(&self) -> &crate::Reg<ltc0_ltc0_pkn_18::LTC0_LTC0_PKN_18_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3144usize)
                as *const crate::Reg<ltc0_ltc0_pkn_18::LTC0_LTC0_PKN_18_SPEC>)
        }
    }
    #[doc = "0xc48 - LTC PKHA N1 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_2(&self) -> &crate::Reg<ltc0_ltc0_pkn1_2::LTC0_LTC0_PKN1_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3144usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_2::LTC0_LTC0_PKN1_2_SPEC>)
        }
    }
    #[doc = "0xc4c - LTC PKHA N 19 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_19(&self) -> &crate::Reg<ltc0_ltc0_pkn_19::LTC0_LTC0_PKN_19_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3148usize)
                as *const crate::Reg<ltc0_ltc0_pkn_19::LTC0_LTC0_PKN_19_SPEC>)
        }
    }
    #[doc = "0xc4c - LTC PKHA N1 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_3(&self) -> &crate::Reg<ltc0_ltc0_pkn1_3::LTC0_LTC0_PKN1_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3148usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_3::LTC0_LTC0_PKN1_3_SPEC>)
        }
    }
    #[doc = "0xc50 - LTC PKHA N 20 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_20(&self) -> &crate::Reg<ltc0_ltc0_pkn_20::LTC0_LTC0_PKN_20_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3152usize)
                as *const crate::Reg<ltc0_ltc0_pkn_20::LTC0_LTC0_PKN_20_SPEC>)
        }
    }
    #[doc = "0xc50 - LTC PKHA N1 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_4(&self) -> &crate::Reg<ltc0_ltc0_pkn1_4::LTC0_LTC0_PKN1_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3152usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_4::LTC0_LTC0_PKN1_4_SPEC>)
        }
    }
    #[doc = "0xc54 - LTC PKHA N 21 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_21(&self) -> &crate::Reg<ltc0_ltc0_pkn_21::LTC0_LTC0_PKN_21_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3156usize)
                as *const crate::Reg<ltc0_ltc0_pkn_21::LTC0_LTC0_PKN_21_SPEC>)
        }
    }
    #[doc = "0xc54 - LTC PKHA N1 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_5(&self) -> &crate::Reg<ltc0_ltc0_pkn1_5::LTC0_LTC0_PKN1_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3156usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_5::LTC0_LTC0_PKN1_5_SPEC>)
        }
    }
    #[doc = "0xc58 - LTC PKHA N 22 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_22(&self) -> &crate::Reg<ltc0_ltc0_pkn_22::LTC0_LTC0_PKN_22_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3160usize)
                as *const crate::Reg<ltc0_ltc0_pkn_22::LTC0_LTC0_PKN_22_SPEC>)
        }
    }
    #[doc = "0xc58 - LTC PKHA N1 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_6(&self) -> &crate::Reg<ltc0_ltc0_pkn1_6::LTC0_LTC0_PKN1_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3160usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_6::LTC0_LTC0_PKN1_6_SPEC>)
        }
    }
    #[doc = "0xc5c - LTC PKHA N 23 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_23(&self) -> &crate::Reg<ltc0_ltc0_pkn_23::LTC0_LTC0_PKN_23_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3164usize)
                as *const crate::Reg<ltc0_ltc0_pkn_23::LTC0_LTC0_PKN_23_SPEC>)
        }
    }
    #[doc = "0xc5c - LTC PKHA N1 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_7(&self) -> &crate::Reg<ltc0_ltc0_pkn1_7::LTC0_LTC0_PKN1_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3164usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_7::LTC0_LTC0_PKN1_7_SPEC>)
        }
    }
    #[doc = "0xc60 - LTC PKHA N 24 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_24(&self) -> &crate::Reg<ltc0_ltc0_pkn_24::LTC0_LTC0_PKN_24_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3168usize)
                as *const crate::Reg<ltc0_ltc0_pkn_24::LTC0_LTC0_PKN_24_SPEC>)
        }
    }
    #[doc = "0xc60 - LTC PKHA N1 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_8(&self) -> &crate::Reg<ltc0_ltc0_pkn1_8::LTC0_LTC0_PKN1_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3168usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_8::LTC0_LTC0_PKN1_8_SPEC>)
        }
    }
    #[doc = "0xc64 - LTC PKHA N 25 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_25(&self) -> &crate::Reg<ltc0_ltc0_pkn_25::LTC0_LTC0_PKN_25_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3172usize)
                as *const crate::Reg<ltc0_ltc0_pkn_25::LTC0_LTC0_PKN_25_SPEC>)
        }
    }
    #[doc = "0xc64 - LTC PKHA N1 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_9(&self) -> &crate::Reg<ltc0_ltc0_pkn1_9::LTC0_LTC0_PKN1_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3172usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_9::LTC0_LTC0_PKN1_9_SPEC>)
        }
    }
    #[doc = "0xc68 - LTC PKHA N 26 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_26(&self) -> &crate::Reg<ltc0_ltc0_pkn_26::LTC0_LTC0_PKN_26_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3176usize)
                as *const crate::Reg<ltc0_ltc0_pkn_26::LTC0_LTC0_PKN_26_SPEC>)
        }
    }
    #[doc = "0xc68 - LTC PKHA N1 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_10(&self) -> &crate::Reg<ltc0_ltc0_pkn1_10::LTC0_LTC0_PKN1_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3176usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_10::LTC0_LTC0_PKN1_10_SPEC>)
        }
    }
    #[doc = "0xc6c - LTC PKHA N 27 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_27(&self) -> &crate::Reg<ltc0_ltc0_pkn_27::LTC0_LTC0_PKN_27_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3180usize)
                as *const crate::Reg<ltc0_ltc0_pkn_27::LTC0_LTC0_PKN_27_SPEC>)
        }
    }
    #[doc = "0xc6c - LTC PKHA N1 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_11(&self) -> &crate::Reg<ltc0_ltc0_pkn1_11::LTC0_LTC0_PKN1_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3180usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_11::LTC0_LTC0_PKN1_11_SPEC>)
        }
    }
    #[doc = "0xc70 - LTC PKHA N 28 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_28(&self) -> &crate::Reg<ltc0_ltc0_pkn_28::LTC0_LTC0_PKN_28_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3184usize)
                as *const crate::Reg<ltc0_ltc0_pkn_28::LTC0_LTC0_PKN_28_SPEC>)
        }
    }
    #[doc = "0xc70 - LTC PKHA N1 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_12(&self) -> &crate::Reg<ltc0_ltc0_pkn1_12::LTC0_LTC0_PKN1_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3184usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_12::LTC0_LTC0_PKN1_12_SPEC>)
        }
    }
    #[doc = "0xc74 - LTC PKHA N 29 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_29(&self) -> &crate::Reg<ltc0_ltc0_pkn_29::LTC0_LTC0_PKN_29_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3188usize)
                as *const crate::Reg<ltc0_ltc0_pkn_29::LTC0_LTC0_PKN_29_SPEC>)
        }
    }
    #[doc = "0xc74 - LTC PKHA N1 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_13(&self) -> &crate::Reg<ltc0_ltc0_pkn1_13::LTC0_LTC0_PKN1_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3188usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_13::LTC0_LTC0_PKN1_13_SPEC>)
        }
    }
    #[doc = "0xc78 - LTC PKHA N 30 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_30(&self) -> &crate::Reg<ltc0_ltc0_pkn_30::LTC0_LTC0_PKN_30_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3192usize)
                as *const crate::Reg<ltc0_ltc0_pkn_30::LTC0_LTC0_PKN_30_SPEC>)
        }
    }
    #[doc = "0xc78 - LTC PKHA N1 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_14(&self) -> &crate::Reg<ltc0_ltc0_pkn1_14::LTC0_LTC0_PKN1_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3192usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_14::LTC0_LTC0_PKN1_14_SPEC>)
        }
    }
    #[doc = "0xc7c - LTC PKHA N 31 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_31(&self) -> &crate::Reg<ltc0_ltc0_pkn_31::LTC0_LTC0_PKN_31_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3196usize)
                as *const crate::Reg<ltc0_ltc0_pkn_31::LTC0_LTC0_PKN_31_SPEC>)
        }
    }
    #[doc = "0xc7c - LTC PKHA N1 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn1_15(&self) -> &crate::Reg<ltc0_ltc0_pkn1_15::LTC0_LTC0_PKN1_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3196usize)
                as *const crate::Reg<ltc0_ltc0_pkn1_15::LTC0_LTC0_PKN1_15_SPEC>)
        }
    }
    #[doc = "0xc80 - LTC PKHA N 32 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_32(&self) -> &crate::Reg<ltc0_ltc0_pkn_32::LTC0_LTC0_PKN_32_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3200usize)
                as *const crate::Reg<ltc0_ltc0_pkn_32::LTC0_LTC0_PKN_32_SPEC>)
        }
    }
    #[doc = "0xc80 - LTC PKHA N2 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_0(&self) -> &crate::Reg<ltc0_ltc0_pkn2_0::LTC0_LTC0_PKN2_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3200usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_0::LTC0_LTC0_PKN2_0_SPEC>)
        }
    }
    #[doc = "0xc84 - LTC PKHA N 33 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_33(&self) -> &crate::Reg<ltc0_ltc0_pkn_33::LTC0_LTC0_PKN_33_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3204usize)
                as *const crate::Reg<ltc0_ltc0_pkn_33::LTC0_LTC0_PKN_33_SPEC>)
        }
    }
    #[doc = "0xc84 - LTC PKHA N2 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_1(&self) -> &crate::Reg<ltc0_ltc0_pkn2_1::LTC0_LTC0_PKN2_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3204usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_1::LTC0_LTC0_PKN2_1_SPEC>)
        }
    }
    #[doc = "0xc88 - LTC PKHA N 34 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_34(&self) -> &crate::Reg<ltc0_ltc0_pkn_34::LTC0_LTC0_PKN_34_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3208usize)
                as *const crate::Reg<ltc0_ltc0_pkn_34::LTC0_LTC0_PKN_34_SPEC>)
        }
    }
    #[doc = "0xc88 - LTC PKHA N2 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_2(&self) -> &crate::Reg<ltc0_ltc0_pkn2_2::LTC0_LTC0_PKN2_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3208usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_2::LTC0_LTC0_PKN2_2_SPEC>)
        }
    }
    #[doc = "0xc8c - LTC PKHA N 35 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_35(&self) -> &crate::Reg<ltc0_ltc0_pkn_35::LTC0_LTC0_PKN_35_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3212usize)
                as *const crate::Reg<ltc0_ltc0_pkn_35::LTC0_LTC0_PKN_35_SPEC>)
        }
    }
    #[doc = "0xc8c - LTC PKHA N2 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_3(&self) -> &crate::Reg<ltc0_ltc0_pkn2_3::LTC0_LTC0_PKN2_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3212usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_3::LTC0_LTC0_PKN2_3_SPEC>)
        }
    }
    #[doc = "0xc90 - LTC PKHA N 36 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_36(&self) -> &crate::Reg<ltc0_ltc0_pkn_36::LTC0_LTC0_PKN_36_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3216usize)
                as *const crate::Reg<ltc0_ltc0_pkn_36::LTC0_LTC0_PKN_36_SPEC>)
        }
    }
    #[doc = "0xc90 - LTC PKHA N2 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_4(&self) -> &crate::Reg<ltc0_ltc0_pkn2_4::LTC0_LTC0_PKN2_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3216usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_4::LTC0_LTC0_PKN2_4_SPEC>)
        }
    }
    #[doc = "0xc94 - LTC PKHA N 37 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_37(&self) -> &crate::Reg<ltc0_ltc0_pkn_37::LTC0_LTC0_PKN_37_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3220usize)
                as *const crate::Reg<ltc0_ltc0_pkn_37::LTC0_LTC0_PKN_37_SPEC>)
        }
    }
    #[doc = "0xc94 - LTC PKHA N2 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_5(&self) -> &crate::Reg<ltc0_ltc0_pkn2_5::LTC0_LTC0_PKN2_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3220usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_5::LTC0_LTC0_PKN2_5_SPEC>)
        }
    }
    #[doc = "0xc98 - LTC PKHA N 38 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_38(&self) -> &crate::Reg<ltc0_ltc0_pkn_38::LTC0_LTC0_PKN_38_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3224usize)
                as *const crate::Reg<ltc0_ltc0_pkn_38::LTC0_LTC0_PKN_38_SPEC>)
        }
    }
    #[doc = "0xc98 - LTC PKHA N2 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_6(&self) -> &crate::Reg<ltc0_ltc0_pkn2_6::LTC0_LTC0_PKN2_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3224usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_6::LTC0_LTC0_PKN2_6_SPEC>)
        }
    }
    #[doc = "0xc9c - LTC PKHA N 39 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_39(&self) -> &crate::Reg<ltc0_ltc0_pkn_39::LTC0_LTC0_PKN_39_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3228usize)
                as *const crate::Reg<ltc0_ltc0_pkn_39::LTC0_LTC0_PKN_39_SPEC>)
        }
    }
    #[doc = "0xc9c - LTC PKHA N2 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_7(&self) -> &crate::Reg<ltc0_ltc0_pkn2_7::LTC0_LTC0_PKN2_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3228usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_7::LTC0_LTC0_PKN2_7_SPEC>)
        }
    }
    #[doc = "0xca0 - LTC PKHA N 40 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_40(&self) -> &crate::Reg<ltc0_ltc0_pkn_40::LTC0_LTC0_PKN_40_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3232usize)
                as *const crate::Reg<ltc0_ltc0_pkn_40::LTC0_LTC0_PKN_40_SPEC>)
        }
    }
    #[doc = "0xca0 - LTC PKHA N2 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_8(&self) -> &crate::Reg<ltc0_ltc0_pkn2_8::LTC0_LTC0_PKN2_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3232usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_8::LTC0_LTC0_PKN2_8_SPEC>)
        }
    }
    #[doc = "0xca4 - LTC PKHA N 41 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_41(&self) -> &crate::Reg<ltc0_ltc0_pkn_41::LTC0_LTC0_PKN_41_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3236usize)
                as *const crate::Reg<ltc0_ltc0_pkn_41::LTC0_LTC0_PKN_41_SPEC>)
        }
    }
    #[doc = "0xca4 - LTC PKHA N2 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_9(&self) -> &crate::Reg<ltc0_ltc0_pkn2_9::LTC0_LTC0_PKN2_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3236usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_9::LTC0_LTC0_PKN2_9_SPEC>)
        }
    }
    #[doc = "0xca8 - LTC PKHA N 42 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_42(&self) -> &crate::Reg<ltc0_ltc0_pkn_42::LTC0_LTC0_PKN_42_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3240usize)
                as *const crate::Reg<ltc0_ltc0_pkn_42::LTC0_LTC0_PKN_42_SPEC>)
        }
    }
    #[doc = "0xca8 - LTC PKHA N2 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_10(&self) -> &crate::Reg<ltc0_ltc0_pkn2_10::LTC0_LTC0_PKN2_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3240usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_10::LTC0_LTC0_PKN2_10_SPEC>)
        }
    }
    #[doc = "0xcac - LTC PKHA N 43 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_43(&self) -> &crate::Reg<ltc0_ltc0_pkn_43::LTC0_LTC0_PKN_43_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3244usize)
                as *const crate::Reg<ltc0_ltc0_pkn_43::LTC0_LTC0_PKN_43_SPEC>)
        }
    }
    #[doc = "0xcac - LTC PKHA N2 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_11(&self) -> &crate::Reg<ltc0_ltc0_pkn2_11::LTC0_LTC0_PKN2_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3244usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_11::LTC0_LTC0_PKN2_11_SPEC>)
        }
    }
    #[doc = "0xcb0 - LTC PKHA N 44 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_44(&self) -> &crate::Reg<ltc0_ltc0_pkn_44::LTC0_LTC0_PKN_44_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3248usize)
                as *const crate::Reg<ltc0_ltc0_pkn_44::LTC0_LTC0_PKN_44_SPEC>)
        }
    }
    #[doc = "0xcb0 - LTC PKHA N2 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_12(&self) -> &crate::Reg<ltc0_ltc0_pkn2_12::LTC0_LTC0_PKN2_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3248usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_12::LTC0_LTC0_PKN2_12_SPEC>)
        }
    }
    #[doc = "0xcb4 - LTC PKHA N 45 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_45(&self) -> &crate::Reg<ltc0_ltc0_pkn_45::LTC0_LTC0_PKN_45_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3252usize)
                as *const crate::Reg<ltc0_ltc0_pkn_45::LTC0_LTC0_PKN_45_SPEC>)
        }
    }
    #[doc = "0xcb4 - LTC PKHA N2 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_13(&self) -> &crate::Reg<ltc0_ltc0_pkn2_13::LTC0_LTC0_PKN2_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3252usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_13::LTC0_LTC0_PKN2_13_SPEC>)
        }
    }
    #[doc = "0xcb8 - LTC PKHA N 46 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_46(&self) -> &crate::Reg<ltc0_ltc0_pkn_46::LTC0_LTC0_PKN_46_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3256usize)
                as *const crate::Reg<ltc0_ltc0_pkn_46::LTC0_LTC0_PKN_46_SPEC>)
        }
    }
    #[doc = "0xcb8 - LTC PKHA N2 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_14(&self) -> &crate::Reg<ltc0_ltc0_pkn2_14::LTC0_LTC0_PKN2_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3256usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_14::LTC0_LTC0_PKN2_14_SPEC>)
        }
    }
    #[doc = "0xcbc - LTC PKHA N 47 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_47(&self) -> &crate::Reg<ltc0_ltc0_pkn_47::LTC0_LTC0_PKN_47_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3260usize)
                as *const crate::Reg<ltc0_ltc0_pkn_47::LTC0_LTC0_PKN_47_SPEC>)
        }
    }
    #[doc = "0xcbc - LTC PKHA N2 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn2_15(&self) -> &crate::Reg<ltc0_ltc0_pkn2_15::LTC0_LTC0_PKN2_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3260usize)
                as *const crate::Reg<ltc0_ltc0_pkn2_15::LTC0_LTC0_PKN2_15_SPEC>)
        }
    }
    #[doc = "0xcc0 - LTC PKHA N 48 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_48(&self) -> &crate::Reg<ltc0_ltc0_pkn_48::LTC0_LTC0_PKN_48_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3264usize)
                as *const crate::Reg<ltc0_ltc0_pkn_48::LTC0_LTC0_PKN_48_SPEC>)
        }
    }
    #[doc = "0xcc0 - LTC PKHA N3 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_0(&self) -> &crate::Reg<ltc0_ltc0_pkn3_0::LTC0_LTC0_PKN3_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3264usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_0::LTC0_LTC0_PKN3_0_SPEC>)
        }
    }
    #[doc = "0xcc4 - LTC PKHA N 49 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_49(&self) -> &crate::Reg<ltc0_ltc0_pkn_49::LTC0_LTC0_PKN_49_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3268usize)
                as *const crate::Reg<ltc0_ltc0_pkn_49::LTC0_LTC0_PKN_49_SPEC>)
        }
    }
    #[doc = "0xcc4 - LTC PKHA N3 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_1(&self) -> &crate::Reg<ltc0_ltc0_pkn3_1::LTC0_LTC0_PKN3_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3268usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_1::LTC0_LTC0_PKN3_1_SPEC>)
        }
    }
    #[doc = "0xcc8 - LTC PKHA N 50 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_50(&self) -> &crate::Reg<ltc0_ltc0_pkn_50::LTC0_LTC0_PKN_50_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3272usize)
                as *const crate::Reg<ltc0_ltc0_pkn_50::LTC0_LTC0_PKN_50_SPEC>)
        }
    }
    #[doc = "0xcc8 - LTC PKHA N3 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_2(&self) -> &crate::Reg<ltc0_ltc0_pkn3_2::LTC0_LTC0_PKN3_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3272usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_2::LTC0_LTC0_PKN3_2_SPEC>)
        }
    }
    #[doc = "0xccc - LTC PKHA N 51 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_51(&self) -> &crate::Reg<ltc0_ltc0_pkn_51::LTC0_LTC0_PKN_51_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3276usize)
                as *const crate::Reg<ltc0_ltc0_pkn_51::LTC0_LTC0_PKN_51_SPEC>)
        }
    }
    #[doc = "0xccc - LTC PKHA N3 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_3(&self) -> &crate::Reg<ltc0_ltc0_pkn3_3::LTC0_LTC0_PKN3_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3276usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_3::LTC0_LTC0_PKN3_3_SPEC>)
        }
    }
    #[doc = "0xcd0 - LTC PKHA N 52 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_52(&self) -> &crate::Reg<ltc0_ltc0_pkn_52::LTC0_LTC0_PKN_52_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3280usize)
                as *const crate::Reg<ltc0_ltc0_pkn_52::LTC0_LTC0_PKN_52_SPEC>)
        }
    }
    #[doc = "0xcd0 - LTC PKHA N3 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_4(&self) -> &crate::Reg<ltc0_ltc0_pkn3_4::LTC0_LTC0_PKN3_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3280usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_4::LTC0_LTC0_PKN3_4_SPEC>)
        }
    }
    #[doc = "0xcd4 - LTC PKHA N 53 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_53(&self) -> &crate::Reg<ltc0_ltc0_pkn_53::LTC0_LTC0_PKN_53_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3284usize)
                as *const crate::Reg<ltc0_ltc0_pkn_53::LTC0_LTC0_PKN_53_SPEC>)
        }
    }
    #[doc = "0xcd4 - LTC PKHA N3 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_5(&self) -> &crate::Reg<ltc0_ltc0_pkn3_5::LTC0_LTC0_PKN3_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3284usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_5::LTC0_LTC0_PKN3_5_SPEC>)
        }
    }
    #[doc = "0xcd8 - LTC PKHA N 54 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_54(&self) -> &crate::Reg<ltc0_ltc0_pkn_54::LTC0_LTC0_PKN_54_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3288usize)
                as *const crate::Reg<ltc0_ltc0_pkn_54::LTC0_LTC0_PKN_54_SPEC>)
        }
    }
    #[doc = "0xcd8 - LTC PKHA N3 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_6(&self) -> &crate::Reg<ltc0_ltc0_pkn3_6::LTC0_LTC0_PKN3_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3288usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_6::LTC0_LTC0_PKN3_6_SPEC>)
        }
    }
    #[doc = "0xcdc - LTC PKHA N 55 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_55(&self) -> &crate::Reg<ltc0_ltc0_pkn_55::LTC0_LTC0_PKN_55_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3292usize)
                as *const crate::Reg<ltc0_ltc0_pkn_55::LTC0_LTC0_PKN_55_SPEC>)
        }
    }
    #[doc = "0xcdc - LTC PKHA N3 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_7(&self) -> &crate::Reg<ltc0_ltc0_pkn3_7::LTC0_LTC0_PKN3_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3292usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_7::LTC0_LTC0_PKN3_7_SPEC>)
        }
    }
    #[doc = "0xce0 - LTC PKHA N 56 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_56(&self) -> &crate::Reg<ltc0_ltc0_pkn_56::LTC0_LTC0_PKN_56_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3296usize)
                as *const crate::Reg<ltc0_ltc0_pkn_56::LTC0_LTC0_PKN_56_SPEC>)
        }
    }
    #[doc = "0xce0 - LTC PKHA N3 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_8(&self) -> &crate::Reg<ltc0_ltc0_pkn3_8::LTC0_LTC0_PKN3_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3296usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_8::LTC0_LTC0_PKN3_8_SPEC>)
        }
    }
    #[doc = "0xce4 - LTC PKHA N 57 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_57(&self) -> &crate::Reg<ltc0_ltc0_pkn_57::LTC0_LTC0_PKN_57_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3300usize)
                as *const crate::Reg<ltc0_ltc0_pkn_57::LTC0_LTC0_PKN_57_SPEC>)
        }
    }
    #[doc = "0xce4 - LTC PKHA N3 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_9(&self) -> &crate::Reg<ltc0_ltc0_pkn3_9::LTC0_LTC0_PKN3_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3300usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_9::LTC0_LTC0_PKN3_9_SPEC>)
        }
    }
    #[doc = "0xce8 - LTC PKHA N 58 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_58(&self) -> &crate::Reg<ltc0_ltc0_pkn_58::LTC0_LTC0_PKN_58_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3304usize)
                as *const crate::Reg<ltc0_ltc0_pkn_58::LTC0_LTC0_PKN_58_SPEC>)
        }
    }
    #[doc = "0xce8 - LTC PKHA N3 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_10(&self) -> &crate::Reg<ltc0_ltc0_pkn3_10::LTC0_LTC0_PKN3_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3304usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_10::LTC0_LTC0_PKN3_10_SPEC>)
        }
    }
    #[doc = "0xcec - LTC PKHA N 59 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_59(&self) -> &crate::Reg<ltc0_ltc0_pkn_59::LTC0_LTC0_PKN_59_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3308usize)
                as *const crate::Reg<ltc0_ltc0_pkn_59::LTC0_LTC0_PKN_59_SPEC>)
        }
    }
    #[doc = "0xcec - LTC PKHA N3 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_11(&self) -> &crate::Reg<ltc0_ltc0_pkn3_11::LTC0_LTC0_PKN3_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3308usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_11::LTC0_LTC0_PKN3_11_SPEC>)
        }
    }
    #[doc = "0xcf0 - LTC PKHA N 60 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_60(&self) -> &crate::Reg<ltc0_ltc0_pkn_60::LTC0_LTC0_PKN_60_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3312usize)
                as *const crate::Reg<ltc0_ltc0_pkn_60::LTC0_LTC0_PKN_60_SPEC>)
        }
    }
    #[doc = "0xcf0 - LTC PKHA N3 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_12(&self) -> &crate::Reg<ltc0_ltc0_pkn3_12::LTC0_LTC0_PKN3_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3312usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_12::LTC0_LTC0_PKN3_12_SPEC>)
        }
    }
    #[doc = "0xcf4 - LTC PKHA N 61 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_61(&self) -> &crate::Reg<ltc0_ltc0_pkn_61::LTC0_LTC0_PKN_61_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3316usize)
                as *const crate::Reg<ltc0_ltc0_pkn_61::LTC0_LTC0_PKN_61_SPEC>)
        }
    }
    #[doc = "0xcf4 - LTC PKHA N3 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_13(&self) -> &crate::Reg<ltc0_ltc0_pkn3_13::LTC0_LTC0_PKN3_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3316usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_13::LTC0_LTC0_PKN3_13_SPEC>)
        }
    }
    #[doc = "0xcf8 - LTC PKHA N 62 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_62(&self) -> &crate::Reg<ltc0_ltc0_pkn_62::LTC0_LTC0_PKN_62_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3320usize)
                as *const crate::Reg<ltc0_ltc0_pkn_62::LTC0_LTC0_PKN_62_SPEC>)
        }
    }
    #[doc = "0xcf8 - LTC PKHA N3 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_14(&self) -> &crate::Reg<ltc0_ltc0_pkn3_14::LTC0_LTC0_PKN3_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3320usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_14::LTC0_LTC0_PKN3_14_SPEC>)
        }
    }
    #[doc = "0xcfc - LTC PKHA N 63 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn_63(&self) -> &crate::Reg<ltc0_ltc0_pkn_63::LTC0_LTC0_PKN_63_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3324usize)
                as *const crate::Reg<ltc0_ltc0_pkn_63::LTC0_LTC0_PKN_63_SPEC>)
        }
    }
    #[doc = "0xcfc - LTC PKHA N3 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pkn3_15(&self) -> &crate::Reg<ltc0_ltc0_pkn3_15::LTC0_LTC0_PKN3_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3324usize)
                as *const crate::Reg<ltc0_ltc0_pkn3_15::LTC0_LTC0_PKN3_15_SPEC>)
        }
    }
    #[doc = "0xe00 - LTC PKHA E 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_0(&self) -> &crate::Reg<ltc0_ltc0_pke_0::LTC0_LTC0_PKE_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3584usize)
                as *const crate::Reg<ltc0_ltc0_pke_0::LTC0_LTC0_PKE_0_SPEC>)
        }
    }
    #[doc = "0xe00 - LTC PKHA E0 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_0(&self) -> &crate::Reg<ltc0_ltc0_pke0_0::LTC0_LTC0_PKE0_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3584usize)
                as *const crate::Reg<ltc0_ltc0_pke0_0::LTC0_LTC0_PKE0_0_SPEC>)
        }
    }
    #[doc = "0xe04 - LTC PKHA E 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_1(&self) -> &crate::Reg<ltc0_ltc0_pke_1::LTC0_LTC0_PKE_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3588usize)
                as *const crate::Reg<ltc0_ltc0_pke_1::LTC0_LTC0_PKE_1_SPEC>)
        }
    }
    #[doc = "0xe04 - LTC PKHA E0 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_1(&self) -> &crate::Reg<ltc0_ltc0_pke0_1::LTC0_LTC0_PKE0_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3588usize)
                as *const crate::Reg<ltc0_ltc0_pke0_1::LTC0_LTC0_PKE0_1_SPEC>)
        }
    }
    #[doc = "0xe08 - LTC PKHA E 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_2(&self) -> &crate::Reg<ltc0_ltc0_pke_2::LTC0_LTC0_PKE_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3592usize)
                as *const crate::Reg<ltc0_ltc0_pke_2::LTC0_LTC0_PKE_2_SPEC>)
        }
    }
    #[doc = "0xe08 - LTC PKHA E0 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_2(&self) -> &crate::Reg<ltc0_ltc0_pke0_2::LTC0_LTC0_PKE0_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3592usize)
                as *const crate::Reg<ltc0_ltc0_pke0_2::LTC0_LTC0_PKE0_2_SPEC>)
        }
    }
    #[doc = "0xe0c - LTC PKHA E 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_3(&self) -> &crate::Reg<ltc0_ltc0_pke_3::LTC0_LTC0_PKE_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3596usize)
                as *const crate::Reg<ltc0_ltc0_pke_3::LTC0_LTC0_PKE_3_SPEC>)
        }
    }
    #[doc = "0xe0c - LTC PKHA E0 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_3(&self) -> &crate::Reg<ltc0_ltc0_pke0_3::LTC0_LTC0_PKE0_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3596usize)
                as *const crate::Reg<ltc0_ltc0_pke0_3::LTC0_LTC0_PKE0_3_SPEC>)
        }
    }
    #[doc = "0xe10 - LTC PKHA E 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_4(&self) -> &crate::Reg<ltc0_ltc0_pke_4::LTC0_LTC0_PKE_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3600usize)
                as *const crate::Reg<ltc0_ltc0_pke_4::LTC0_LTC0_PKE_4_SPEC>)
        }
    }
    #[doc = "0xe10 - LTC PKHA E0 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_4(&self) -> &crate::Reg<ltc0_ltc0_pke0_4::LTC0_LTC0_PKE0_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3600usize)
                as *const crate::Reg<ltc0_ltc0_pke0_4::LTC0_LTC0_PKE0_4_SPEC>)
        }
    }
    #[doc = "0xe14 - LTC PKHA E 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_5(&self) -> &crate::Reg<ltc0_ltc0_pke_5::LTC0_LTC0_PKE_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3604usize)
                as *const crate::Reg<ltc0_ltc0_pke_5::LTC0_LTC0_PKE_5_SPEC>)
        }
    }
    #[doc = "0xe14 - LTC PKHA E0 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_5(&self) -> &crate::Reg<ltc0_ltc0_pke0_5::LTC0_LTC0_PKE0_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3604usize)
                as *const crate::Reg<ltc0_ltc0_pke0_5::LTC0_LTC0_PKE0_5_SPEC>)
        }
    }
    #[doc = "0xe18 - LTC PKHA E 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_6(&self) -> &crate::Reg<ltc0_ltc0_pke_6::LTC0_LTC0_PKE_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3608usize)
                as *const crate::Reg<ltc0_ltc0_pke_6::LTC0_LTC0_PKE_6_SPEC>)
        }
    }
    #[doc = "0xe18 - LTC PKHA E0 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_6(&self) -> &crate::Reg<ltc0_ltc0_pke0_6::LTC0_LTC0_PKE0_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3608usize)
                as *const crate::Reg<ltc0_ltc0_pke0_6::LTC0_LTC0_PKE0_6_SPEC>)
        }
    }
    #[doc = "0xe1c - LTC PKHA E 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_7(&self) -> &crate::Reg<ltc0_ltc0_pke_7::LTC0_LTC0_PKE_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3612usize)
                as *const crate::Reg<ltc0_ltc0_pke_7::LTC0_LTC0_PKE_7_SPEC>)
        }
    }
    #[doc = "0xe1c - LTC PKHA E0 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_7(&self) -> &crate::Reg<ltc0_ltc0_pke0_7::LTC0_LTC0_PKE0_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3612usize)
                as *const crate::Reg<ltc0_ltc0_pke0_7::LTC0_LTC0_PKE0_7_SPEC>)
        }
    }
    #[doc = "0xe20 - LTC PKHA E 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_8(&self) -> &crate::Reg<ltc0_ltc0_pke_8::LTC0_LTC0_PKE_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3616usize)
                as *const crate::Reg<ltc0_ltc0_pke_8::LTC0_LTC0_PKE_8_SPEC>)
        }
    }
    #[doc = "0xe20 - LTC PKHA E0 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_8(&self) -> &crate::Reg<ltc0_ltc0_pke0_8::LTC0_LTC0_PKE0_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3616usize)
                as *const crate::Reg<ltc0_ltc0_pke0_8::LTC0_LTC0_PKE0_8_SPEC>)
        }
    }
    #[doc = "0xe24 - LTC PKHA E 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_9(&self) -> &crate::Reg<ltc0_ltc0_pke_9::LTC0_LTC0_PKE_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3620usize)
                as *const crate::Reg<ltc0_ltc0_pke_9::LTC0_LTC0_PKE_9_SPEC>)
        }
    }
    #[doc = "0xe24 - LTC PKHA E0 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_9(&self) -> &crate::Reg<ltc0_ltc0_pke0_9::LTC0_LTC0_PKE0_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3620usize)
                as *const crate::Reg<ltc0_ltc0_pke0_9::LTC0_LTC0_PKE0_9_SPEC>)
        }
    }
    #[doc = "0xe28 - LTC PKHA E 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_10(&self) -> &crate::Reg<ltc0_ltc0_pke_10::LTC0_LTC0_PKE_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3624usize)
                as *const crate::Reg<ltc0_ltc0_pke_10::LTC0_LTC0_PKE_10_SPEC>)
        }
    }
    #[doc = "0xe28 - LTC PKHA E0 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_10(&self) -> &crate::Reg<ltc0_ltc0_pke0_10::LTC0_LTC0_PKE0_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3624usize)
                as *const crate::Reg<ltc0_ltc0_pke0_10::LTC0_LTC0_PKE0_10_SPEC>)
        }
    }
    #[doc = "0xe2c - LTC PKHA E 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_11(&self) -> &crate::Reg<ltc0_ltc0_pke_11::LTC0_LTC0_PKE_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3628usize)
                as *const crate::Reg<ltc0_ltc0_pke_11::LTC0_LTC0_PKE_11_SPEC>)
        }
    }
    #[doc = "0xe2c - LTC PKHA E0 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_11(&self) -> &crate::Reg<ltc0_ltc0_pke0_11::LTC0_LTC0_PKE0_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3628usize)
                as *const crate::Reg<ltc0_ltc0_pke0_11::LTC0_LTC0_PKE0_11_SPEC>)
        }
    }
    #[doc = "0xe30 - LTC PKHA E 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_12(&self) -> &crate::Reg<ltc0_ltc0_pke_12::LTC0_LTC0_PKE_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3632usize)
                as *const crate::Reg<ltc0_ltc0_pke_12::LTC0_LTC0_PKE_12_SPEC>)
        }
    }
    #[doc = "0xe30 - LTC PKHA E0 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_12(&self) -> &crate::Reg<ltc0_ltc0_pke0_12::LTC0_LTC0_PKE0_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3632usize)
                as *const crate::Reg<ltc0_ltc0_pke0_12::LTC0_LTC0_PKE0_12_SPEC>)
        }
    }
    #[doc = "0xe34 - LTC PKHA E 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_13(&self) -> &crate::Reg<ltc0_ltc0_pke_13::LTC0_LTC0_PKE_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3636usize)
                as *const crate::Reg<ltc0_ltc0_pke_13::LTC0_LTC0_PKE_13_SPEC>)
        }
    }
    #[doc = "0xe34 - LTC PKHA E0 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_13(&self) -> &crate::Reg<ltc0_ltc0_pke0_13::LTC0_LTC0_PKE0_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3636usize)
                as *const crate::Reg<ltc0_ltc0_pke0_13::LTC0_LTC0_PKE0_13_SPEC>)
        }
    }
    #[doc = "0xe38 - LTC PKHA E 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_14(&self) -> &crate::Reg<ltc0_ltc0_pke_14::LTC0_LTC0_PKE_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3640usize)
                as *const crate::Reg<ltc0_ltc0_pke_14::LTC0_LTC0_PKE_14_SPEC>)
        }
    }
    #[doc = "0xe38 - LTC PKHA E0 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_14(&self) -> &crate::Reg<ltc0_ltc0_pke0_14::LTC0_LTC0_PKE0_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3640usize)
                as *const crate::Reg<ltc0_ltc0_pke0_14::LTC0_LTC0_PKE0_14_SPEC>)
        }
    }
    #[doc = "0xe3c - LTC PKHA E 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_15(&self) -> &crate::Reg<ltc0_ltc0_pke_15::LTC0_LTC0_PKE_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3644usize)
                as *const crate::Reg<ltc0_ltc0_pke_15::LTC0_LTC0_PKE_15_SPEC>)
        }
    }
    #[doc = "0xe3c - LTC PKHA E0 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke0_15(&self) -> &crate::Reg<ltc0_ltc0_pke0_15::LTC0_LTC0_PKE0_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3644usize)
                as *const crate::Reg<ltc0_ltc0_pke0_15::LTC0_LTC0_PKE0_15_SPEC>)
        }
    }
    #[doc = "0xe40 - LTC PKHA E 16 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_16(&self) -> &crate::Reg<ltc0_ltc0_pke_16::LTC0_LTC0_PKE_16_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3648usize)
                as *const crate::Reg<ltc0_ltc0_pke_16::LTC0_LTC0_PKE_16_SPEC>)
        }
    }
    #[doc = "0xe40 - LTC PKHA E1 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_0(&self) -> &crate::Reg<ltc0_ltc0_pke1_0::LTC0_LTC0_PKE1_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3648usize)
                as *const crate::Reg<ltc0_ltc0_pke1_0::LTC0_LTC0_PKE1_0_SPEC>)
        }
    }
    #[doc = "0xe44 - LTC PKHA E 17 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_17(&self) -> &crate::Reg<ltc0_ltc0_pke_17::LTC0_LTC0_PKE_17_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3652usize)
                as *const crate::Reg<ltc0_ltc0_pke_17::LTC0_LTC0_PKE_17_SPEC>)
        }
    }
    #[doc = "0xe44 - LTC PKHA E1 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_1(&self) -> &crate::Reg<ltc0_ltc0_pke1_1::LTC0_LTC0_PKE1_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3652usize)
                as *const crate::Reg<ltc0_ltc0_pke1_1::LTC0_LTC0_PKE1_1_SPEC>)
        }
    }
    #[doc = "0xe48 - LTC PKHA E 18 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_18(&self) -> &crate::Reg<ltc0_ltc0_pke_18::LTC0_LTC0_PKE_18_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3656usize)
                as *const crate::Reg<ltc0_ltc0_pke_18::LTC0_LTC0_PKE_18_SPEC>)
        }
    }
    #[doc = "0xe48 - LTC PKHA E1 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_2(&self) -> &crate::Reg<ltc0_ltc0_pke1_2::LTC0_LTC0_PKE1_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3656usize)
                as *const crate::Reg<ltc0_ltc0_pke1_2::LTC0_LTC0_PKE1_2_SPEC>)
        }
    }
    #[doc = "0xe4c - LTC PKHA E 19 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_19(&self) -> &crate::Reg<ltc0_ltc0_pke_19::LTC0_LTC0_PKE_19_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3660usize)
                as *const crate::Reg<ltc0_ltc0_pke_19::LTC0_LTC0_PKE_19_SPEC>)
        }
    }
    #[doc = "0xe4c - LTC PKHA E1 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_3(&self) -> &crate::Reg<ltc0_ltc0_pke1_3::LTC0_LTC0_PKE1_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3660usize)
                as *const crate::Reg<ltc0_ltc0_pke1_3::LTC0_LTC0_PKE1_3_SPEC>)
        }
    }
    #[doc = "0xe50 - LTC PKHA E 20 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_20(&self) -> &crate::Reg<ltc0_ltc0_pke_20::LTC0_LTC0_PKE_20_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3664usize)
                as *const crate::Reg<ltc0_ltc0_pke_20::LTC0_LTC0_PKE_20_SPEC>)
        }
    }
    #[doc = "0xe50 - LTC PKHA E1 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_4(&self) -> &crate::Reg<ltc0_ltc0_pke1_4::LTC0_LTC0_PKE1_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3664usize)
                as *const crate::Reg<ltc0_ltc0_pke1_4::LTC0_LTC0_PKE1_4_SPEC>)
        }
    }
    #[doc = "0xe54 - LTC PKHA E 21 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_21(&self) -> &crate::Reg<ltc0_ltc0_pke_21::LTC0_LTC0_PKE_21_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3668usize)
                as *const crate::Reg<ltc0_ltc0_pke_21::LTC0_LTC0_PKE_21_SPEC>)
        }
    }
    #[doc = "0xe54 - LTC PKHA E1 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_5(&self) -> &crate::Reg<ltc0_ltc0_pke1_5::LTC0_LTC0_PKE1_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3668usize)
                as *const crate::Reg<ltc0_ltc0_pke1_5::LTC0_LTC0_PKE1_5_SPEC>)
        }
    }
    #[doc = "0xe58 - LTC PKHA E 22 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_22(&self) -> &crate::Reg<ltc0_ltc0_pke_22::LTC0_LTC0_PKE_22_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3672usize)
                as *const crate::Reg<ltc0_ltc0_pke_22::LTC0_LTC0_PKE_22_SPEC>)
        }
    }
    #[doc = "0xe58 - LTC PKHA E1 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_6(&self) -> &crate::Reg<ltc0_ltc0_pke1_6::LTC0_LTC0_PKE1_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3672usize)
                as *const crate::Reg<ltc0_ltc0_pke1_6::LTC0_LTC0_PKE1_6_SPEC>)
        }
    }
    #[doc = "0xe5c - LTC PKHA E 23 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_23(&self) -> &crate::Reg<ltc0_ltc0_pke_23::LTC0_LTC0_PKE_23_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3676usize)
                as *const crate::Reg<ltc0_ltc0_pke_23::LTC0_LTC0_PKE_23_SPEC>)
        }
    }
    #[doc = "0xe5c - LTC PKHA E1 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_7(&self) -> &crate::Reg<ltc0_ltc0_pke1_7::LTC0_LTC0_PKE1_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3676usize)
                as *const crate::Reg<ltc0_ltc0_pke1_7::LTC0_LTC0_PKE1_7_SPEC>)
        }
    }
    #[doc = "0xe60 - LTC PKHA E 24 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_24(&self) -> &crate::Reg<ltc0_ltc0_pke_24::LTC0_LTC0_PKE_24_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3680usize)
                as *const crate::Reg<ltc0_ltc0_pke_24::LTC0_LTC0_PKE_24_SPEC>)
        }
    }
    #[doc = "0xe60 - LTC PKHA E1 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_8(&self) -> &crate::Reg<ltc0_ltc0_pke1_8::LTC0_LTC0_PKE1_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3680usize)
                as *const crate::Reg<ltc0_ltc0_pke1_8::LTC0_LTC0_PKE1_8_SPEC>)
        }
    }
    #[doc = "0xe64 - LTC PKHA E 25 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_25(&self) -> &crate::Reg<ltc0_ltc0_pke_25::LTC0_LTC0_PKE_25_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3684usize)
                as *const crate::Reg<ltc0_ltc0_pke_25::LTC0_LTC0_PKE_25_SPEC>)
        }
    }
    #[doc = "0xe64 - LTC PKHA E1 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_9(&self) -> &crate::Reg<ltc0_ltc0_pke1_9::LTC0_LTC0_PKE1_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3684usize)
                as *const crate::Reg<ltc0_ltc0_pke1_9::LTC0_LTC0_PKE1_9_SPEC>)
        }
    }
    #[doc = "0xe68 - LTC PKHA E 26 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_26(&self) -> &crate::Reg<ltc0_ltc0_pke_26::LTC0_LTC0_PKE_26_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3688usize)
                as *const crate::Reg<ltc0_ltc0_pke_26::LTC0_LTC0_PKE_26_SPEC>)
        }
    }
    #[doc = "0xe68 - LTC PKHA E1 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_10(&self) -> &crate::Reg<ltc0_ltc0_pke1_10::LTC0_LTC0_PKE1_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3688usize)
                as *const crate::Reg<ltc0_ltc0_pke1_10::LTC0_LTC0_PKE1_10_SPEC>)
        }
    }
    #[doc = "0xe6c - LTC PKHA E 27 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_27(&self) -> &crate::Reg<ltc0_ltc0_pke_27::LTC0_LTC0_PKE_27_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3692usize)
                as *const crate::Reg<ltc0_ltc0_pke_27::LTC0_LTC0_PKE_27_SPEC>)
        }
    }
    #[doc = "0xe6c - LTC PKHA E1 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_11(&self) -> &crate::Reg<ltc0_ltc0_pke1_11::LTC0_LTC0_PKE1_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3692usize)
                as *const crate::Reg<ltc0_ltc0_pke1_11::LTC0_LTC0_PKE1_11_SPEC>)
        }
    }
    #[doc = "0xe70 - LTC PKHA E 28 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_28(&self) -> &crate::Reg<ltc0_ltc0_pke_28::LTC0_LTC0_PKE_28_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3696usize)
                as *const crate::Reg<ltc0_ltc0_pke_28::LTC0_LTC0_PKE_28_SPEC>)
        }
    }
    #[doc = "0xe70 - LTC PKHA E1 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_12(&self) -> &crate::Reg<ltc0_ltc0_pke1_12::LTC0_LTC0_PKE1_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3696usize)
                as *const crate::Reg<ltc0_ltc0_pke1_12::LTC0_LTC0_PKE1_12_SPEC>)
        }
    }
    #[doc = "0xe74 - LTC PKHA E 29 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_29(&self) -> &crate::Reg<ltc0_ltc0_pke_29::LTC0_LTC0_PKE_29_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3700usize)
                as *const crate::Reg<ltc0_ltc0_pke_29::LTC0_LTC0_PKE_29_SPEC>)
        }
    }
    #[doc = "0xe74 - LTC PKHA E1 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_13(&self) -> &crate::Reg<ltc0_ltc0_pke1_13::LTC0_LTC0_PKE1_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3700usize)
                as *const crate::Reg<ltc0_ltc0_pke1_13::LTC0_LTC0_PKE1_13_SPEC>)
        }
    }
    #[doc = "0xe78 - LTC PKHA E 30 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_30(&self) -> &crate::Reg<ltc0_ltc0_pke_30::LTC0_LTC0_PKE_30_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3704usize)
                as *const crate::Reg<ltc0_ltc0_pke_30::LTC0_LTC0_PKE_30_SPEC>)
        }
    }
    #[doc = "0xe78 - LTC PKHA E1 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_14(&self) -> &crate::Reg<ltc0_ltc0_pke1_14::LTC0_LTC0_PKE1_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3704usize)
                as *const crate::Reg<ltc0_ltc0_pke1_14::LTC0_LTC0_PKE1_14_SPEC>)
        }
    }
    #[doc = "0xe7c - LTC PKHA E 31 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_31(&self) -> &crate::Reg<ltc0_ltc0_pke_31::LTC0_LTC0_PKE_31_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3708usize)
                as *const crate::Reg<ltc0_ltc0_pke_31::LTC0_LTC0_PKE_31_SPEC>)
        }
    }
    #[doc = "0xe7c - LTC PKHA E1 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke1_15(&self) -> &crate::Reg<ltc0_ltc0_pke1_15::LTC0_LTC0_PKE1_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3708usize)
                as *const crate::Reg<ltc0_ltc0_pke1_15::LTC0_LTC0_PKE1_15_SPEC>)
        }
    }
    #[doc = "0xe80 - LTC PKHA E 32 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_32(&self) -> &crate::Reg<ltc0_ltc0_pke_32::LTC0_LTC0_PKE_32_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3712usize)
                as *const crate::Reg<ltc0_ltc0_pke_32::LTC0_LTC0_PKE_32_SPEC>)
        }
    }
    #[doc = "0xe80 - LTC PKHA E2 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_0(&self) -> &crate::Reg<ltc0_ltc0_pke2_0::LTC0_LTC0_PKE2_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3712usize)
                as *const crate::Reg<ltc0_ltc0_pke2_0::LTC0_LTC0_PKE2_0_SPEC>)
        }
    }
    #[doc = "0xe84 - LTC PKHA E 33 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_33(&self) -> &crate::Reg<ltc0_ltc0_pke_33::LTC0_LTC0_PKE_33_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3716usize)
                as *const crate::Reg<ltc0_ltc0_pke_33::LTC0_LTC0_PKE_33_SPEC>)
        }
    }
    #[doc = "0xe84 - LTC PKHA E2 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_1(&self) -> &crate::Reg<ltc0_ltc0_pke2_1::LTC0_LTC0_PKE2_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3716usize)
                as *const crate::Reg<ltc0_ltc0_pke2_1::LTC0_LTC0_PKE2_1_SPEC>)
        }
    }
    #[doc = "0xe88 - LTC PKHA E 34 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_34(&self) -> &crate::Reg<ltc0_ltc0_pke_34::LTC0_LTC0_PKE_34_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3720usize)
                as *const crate::Reg<ltc0_ltc0_pke_34::LTC0_LTC0_PKE_34_SPEC>)
        }
    }
    #[doc = "0xe88 - LTC PKHA E2 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_2(&self) -> &crate::Reg<ltc0_ltc0_pke2_2::LTC0_LTC0_PKE2_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3720usize)
                as *const crate::Reg<ltc0_ltc0_pke2_2::LTC0_LTC0_PKE2_2_SPEC>)
        }
    }
    #[doc = "0xe8c - LTC PKHA E 35 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_35(&self) -> &crate::Reg<ltc0_ltc0_pke_35::LTC0_LTC0_PKE_35_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3724usize)
                as *const crate::Reg<ltc0_ltc0_pke_35::LTC0_LTC0_PKE_35_SPEC>)
        }
    }
    #[doc = "0xe8c - LTC PKHA E2 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_3(&self) -> &crate::Reg<ltc0_ltc0_pke2_3::LTC0_LTC0_PKE2_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3724usize)
                as *const crate::Reg<ltc0_ltc0_pke2_3::LTC0_LTC0_PKE2_3_SPEC>)
        }
    }
    #[doc = "0xe90 - LTC PKHA E 36 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_36(&self) -> &crate::Reg<ltc0_ltc0_pke_36::LTC0_LTC0_PKE_36_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3728usize)
                as *const crate::Reg<ltc0_ltc0_pke_36::LTC0_LTC0_PKE_36_SPEC>)
        }
    }
    #[doc = "0xe90 - LTC PKHA E2 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_4(&self) -> &crate::Reg<ltc0_ltc0_pke2_4::LTC0_LTC0_PKE2_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3728usize)
                as *const crate::Reg<ltc0_ltc0_pke2_4::LTC0_LTC0_PKE2_4_SPEC>)
        }
    }
    #[doc = "0xe94 - LTC PKHA E 37 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_37(&self) -> &crate::Reg<ltc0_ltc0_pke_37::LTC0_LTC0_PKE_37_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3732usize)
                as *const crate::Reg<ltc0_ltc0_pke_37::LTC0_LTC0_PKE_37_SPEC>)
        }
    }
    #[doc = "0xe94 - LTC PKHA E2 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_5(&self) -> &crate::Reg<ltc0_ltc0_pke2_5::LTC0_LTC0_PKE2_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3732usize)
                as *const crate::Reg<ltc0_ltc0_pke2_5::LTC0_LTC0_PKE2_5_SPEC>)
        }
    }
    #[doc = "0xe98 - LTC PKHA E 38 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_38(&self) -> &crate::Reg<ltc0_ltc0_pke_38::LTC0_LTC0_PKE_38_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3736usize)
                as *const crate::Reg<ltc0_ltc0_pke_38::LTC0_LTC0_PKE_38_SPEC>)
        }
    }
    #[doc = "0xe98 - LTC PKHA E2 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_6(&self) -> &crate::Reg<ltc0_ltc0_pke2_6::LTC0_LTC0_PKE2_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3736usize)
                as *const crate::Reg<ltc0_ltc0_pke2_6::LTC0_LTC0_PKE2_6_SPEC>)
        }
    }
    #[doc = "0xe9c - LTC PKHA E 39 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_39(&self) -> &crate::Reg<ltc0_ltc0_pke_39::LTC0_LTC0_PKE_39_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3740usize)
                as *const crate::Reg<ltc0_ltc0_pke_39::LTC0_LTC0_PKE_39_SPEC>)
        }
    }
    #[doc = "0xe9c - LTC PKHA E2 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_7(&self) -> &crate::Reg<ltc0_ltc0_pke2_7::LTC0_LTC0_PKE2_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3740usize)
                as *const crate::Reg<ltc0_ltc0_pke2_7::LTC0_LTC0_PKE2_7_SPEC>)
        }
    }
    #[doc = "0xea0 - LTC PKHA E 40 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_40(&self) -> &crate::Reg<ltc0_ltc0_pke_40::LTC0_LTC0_PKE_40_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3744usize)
                as *const crate::Reg<ltc0_ltc0_pke_40::LTC0_LTC0_PKE_40_SPEC>)
        }
    }
    #[doc = "0xea0 - LTC PKHA E2 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_8(&self) -> &crate::Reg<ltc0_ltc0_pke2_8::LTC0_LTC0_PKE2_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3744usize)
                as *const crate::Reg<ltc0_ltc0_pke2_8::LTC0_LTC0_PKE2_8_SPEC>)
        }
    }
    #[doc = "0xea4 - LTC PKHA E 41 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_41(&self) -> &crate::Reg<ltc0_ltc0_pke_41::LTC0_LTC0_PKE_41_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3748usize)
                as *const crate::Reg<ltc0_ltc0_pke_41::LTC0_LTC0_PKE_41_SPEC>)
        }
    }
    #[doc = "0xea4 - LTC PKHA E2 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_9(&self) -> &crate::Reg<ltc0_ltc0_pke2_9::LTC0_LTC0_PKE2_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3748usize)
                as *const crate::Reg<ltc0_ltc0_pke2_9::LTC0_LTC0_PKE2_9_SPEC>)
        }
    }
    #[doc = "0xea8 - LTC PKHA E 42 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_42(&self) -> &crate::Reg<ltc0_ltc0_pke_42::LTC0_LTC0_PKE_42_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3752usize)
                as *const crate::Reg<ltc0_ltc0_pke_42::LTC0_LTC0_PKE_42_SPEC>)
        }
    }
    #[doc = "0xea8 - LTC PKHA E2 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_10(&self) -> &crate::Reg<ltc0_ltc0_pke2_10::LTC0_LTC0_PKE2_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3752usize)
                as *const crate::Reg<ltc0_ltc0_pke2_10::LTC0_LTC0_PKE2_10_SPEC>)
        }
    }
    #[doc = "0xeac - LTC PKHA E 43 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_43(&self) -> &crate::Reg<ltc0_ltc0_pke_43::LTC0_LTC0_PKE_43_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3756usize)
                as *const crate::Reg<ltc0_ltc0_pke_43::LTC0_LTC0_PKE_43_SPEC>)
        }
    }
    #[doc = "0xeac - LTC PKHA E2 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_11(&self) -> &crate::Reg<ltc0_ltc0_pke2_11::LTC0_LTC0_PKE2_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3756usize)
                as *const crate::Reg<ltc0_ltc0_pke2_11::LTC0_LTC0_PKE2_11_SPEC>)
        }
    }
    #[doc = "0xeb0 - LTC PKHA E 44 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_44(&self) -> &crate::Reg<ltc0_ltc0_pke_44::LTC0_LTC0_PKE_44_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3760usize)
                as *const crate::Reg<ltc0_ltc0_pke_44::LTC0_LTC0_PKE_44_SPEC>)
        }
    }
    #[doc = "0xeb0 - LTC PKHA E2 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_12(&self) -> &crate::Reg<ltc0_ltc0_pke2_12::LTC0_LTC0_PKE2_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3760usize)
                as *const crate::Reg<ltc0_ltc0_pke2_12::LTC0_LTC0_PKE2_12_SPEC>)
        }
    }
    #[doc = "0xeb4 - LTC PKHA E 45 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_45(&self) -> &crate::Reg<ltc0_ltc0_pke_45::LTC0_LTC0_PKE_45_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3764usize)
                as *const crate::Reg<ltc0_ltc0_pke_45::LTC0_LTC0_PKE_45_SPEC>)
        }
    }
    #[doc = "0xeb4 - LTC PKHA E2 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_13(&self) -> &crate::Reg<ltc0_ltc0_pke2_13::LTC0_LTC0_PKE2_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3764usize)
                as *const crate::Reg<ltc0_ltc0_pke2_13::LTC0_LTC0_PKE2_13_SPEC>)
        }
    }
    #[doc = "0xeb8 - LTC PKHA E 46 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_46(&self) -> &crate::Reg<ltc0_ltc0_pke_46::LTC0_LTC0_PKE_46_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3768usize)
                as *const crate::Reg<ltc0_ltc0_pke_46::LTC0_LTC0_PKE_46_SPEC>)
        }
    }
    #[doc = "0xeb8 - LTC PKHA E2 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_14(&self) -> &crate::Reg<ltc0_ltc0_pke2_14::LTC0_LTC0_PKE2_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3768usize)
                as *const crate::Reg<ltc0_ltc0_pke2_14::LTC0_LTC0_PKE2_14_SPEC>)
        }
    }
    #[doc = "0xebc - LTC PKHA E 47 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_47(&self) -> &crate::Reg<ltc0_ltc0_pke_47::LTC0_LTC0_PKE_47_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3772usize)
                as *const crate::Reg<ltc0_ltc0_pke_47::LTC0_LTC0_PKE_47_SPEC>)
        }
    }
    #[doc = "0xebc - LTC PKHA E2 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke2_15(&self) -> &crate::Reg<ltc0_ltc0_pke2_15::LTC0_LTC0_PKE2_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3772usize)
                as *const crate::Reg<ltc0_ltc0_pke2_15::LTC0_LTC0_PKE2_15_SPEC>)
        }
    }
    #[doc = "0xec0 - LTC PKHA E 48 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_48(&self) -> &crate::Reg<ltc0_ltc0_pke_48::LTC0_LTC0_PKE_48_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3776usize)
                as *const crate::Reg<ltc0_ltc0_pke_48::LTC0_LTC0_PKE_48_SPEC>)
        }
    }
    #[doc = "0xec0 - LTC PKHA E3 0 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_0(&self) -> &crate::Reg<ltc0_ltc0_pke3_0::LTC0_LTC0_PKE3_0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3776usize)
                as *const crate::Reg<ltc0_ltc0_pke3_0::LTC0_LTC0_PKE3_0_SPEC>)
        }
    }
    #[doc = "0xec4 - LTC PKHA E 49 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_49(&self) -> &crate::Reg<ltc0_ltc0_pke_49::LTC0_LTC0_PKE_49_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3780usize)
                as *const crate::Reg<ltc0_ltc0_pke_49::LTC0_LTC0_PKE_49_SPEC>)
        }
    }
    #[doc = "0xec4 - LTC PKHA E3 1 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_1(&self) -> &crate::Reg<ltc0_ltc0_pke3_1::LTC0_LTC0_PKE3_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3780usize)
                as *const crate::Reg<ltc0_ltc0_pke3_1::LTC0_LTC0_PKE3_1_SPEC>)
        }
    }
    #[doc = "0xec8 - LTC PKHA E 50 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_50(&self) -> &crate::Reg<ltc0_ltc0_pke_50::LTC0_LTC0_PKE_50_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3784usize)
                as *const crate::Reg<ltc0_ltc0_pke_50::LTC0_LTC0_PKE_50_SPEC>)
        }
    }
    #[doc = "0xec8 - LTC PKHA E3 2 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_2(&self) -> &crate::Reg<ltc0_ltc0_pke3_2::LTC0_LTC0_PKE3_2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3784usize)
                as *const crate::Reg<ltc0_ltc0_pke3_2::LTC0_LTC0_PKE3_2_SPEC>)
        }
    }
    #[doc = "0xecc - LTC PKHA E 51 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_51(&self) -> &crate::Reg<ltc0_ltc0_pke_51::LTC0_LTC0_PKE_51_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3788usize)
                as *const crate::Reg<ltc0_ltc0_pke_51::LTC0_LTC0_PKE_51_SPEC>)
        }
    }
    #[doc = "0xecc - LTC PKHA E3 3 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_3(&self) -> &crate::Reg<ltc0_ltc0_pke3_3::LTC0_LTC0_PKE3_3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3788usize)
                as *const crate::Reg<ltc0_ltc0_pke3_3::LTC0_LTC0_PKE3_3_SPEC>)
        }
    }
    #[doc = "0xed0 - LTC PKHA E 52 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_52(&self) -> &crate::Reg<ltc0_ltc0_pke_52::LTC0_LTC0_PKE_52_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3792usize)
                as *const crate::Reg<ltc0_ltc0_pke_52::LTC0_LTC0_PKE_52_SPEC>)
        }
    }
    #[doc = "0xed0 - LTC PKHA E3 4 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_4(&self) -> &crate::Reg<ltc0_ltc0_pke3_4::LTC0_LTC0_PKE3_4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3792usize)
                as *const crate::Reg<ltc0_ltc0_pke3_4::LTC0_LTC0_PKE3_4_SPEC>)
        }
    }
    #[doc = "0xed4 - LTC PKHA E 53 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_53(&self) -> &crate::Reg<ltc0_ltc0_pke_53::LTC0_LTC0_PKE_53_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3796usize)
                as *const crate::Reg<ltc0_ltc0_pke_53::LTC0_LTC0_PKE_53_SPEC>)
        }
    }
    #[doc = "0xed4 - LTC PKHA E3 5 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_5(&self) -> &crate::Reg<ltc0_ltc0_pke3_5::LTC0_LTC0_PKE3_5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3796usize)
                as *const crate::Reg<ltc0_ltc0_pke3_5::LTC0_LTC0_PKE3_5_SPEC>)
        }
    }
    #[doc = "0xed8 - LTC PKHA E 54 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_54(&self) -> &crate::Reg<ltc0_ltc0_pke_54::LTC0_LTC0_PKE_54_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3800usize)
                as *const crate::Reg<ltc0_ltc0_pke_54::LTC0_LTC0_PKE_54_SPEC>)
        }
    }
    #[doc = "0xed8 - LTC PKHA E3 6 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_6(&self) -> &crate::Reg<ltc0_ltc0_pke3_6::LTC0_LTC0_PKE3_6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3800usize)
                as *const crate::Reg<ltc0_ltc0_pke3_6::LTC0_LTC0_PKE3_6_SPEC>)
        }
    }
    #[doc = "0xedc - LTC PKHA E 55 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_55(&self) -> &crate::Reg<ltc0_ltc0_pke_55::LTC0_LTC0_PKE_55_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3804usize)
                as *const crate::Reg<ltc0_ltc0_pke_55::LTC0_LTC0_PKE_55_SPEC>)
        }
    }
    #[doc = "0xedc - LTC PKHA E3 7 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_7(&self) -> &crate::Reg<ltc0_ltc0_pke3_7::LTC0_LTC0_PKE3_7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3804usize)
                as *const crate::Reg<ltc0_ltc0_pke3_7::LTC0_LTC0_PKE3_7_SPEC>)
        }
    }
    #[doc = "0xee0 - LTC PKHA E 56 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_56(&self) -> &crate::Reg<ltc0_ltc0_pke_56::LTC0_LTC0_PKE_56_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3808usize)
                as *const crate::Reg<ltc0_ltc0_pke_56::LTC0_LTC0_PKE_56_SPEC>)
        }
    }
    #[doc = "0xee0 - LTC PKHA E3 8 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_8(&self) -> &crate::Reg<ltc0_ltc0_pke3_8::LTC0_LTC0_PKE3_8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3808usize)
                as *const crate::Reg<ltc0_ltc0_pke3_8::LTC0_LTC0_PKE3_8_SPEC>)
        }
    }
    #[doc = "0xee4 - LTC PKHA E 57 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_57(&self) -> &crate::Reg<ltc0_ltc0_pke_57::LTC0_LTC0_PKE_57_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3812usize)
                as *const crate::Reg<ltc0_ltc0_pke_57::LTC0_LTC0_PKE_57_SPEC>)
        }
    }
    #[doc = "0xee4 - LTC PKHA E3 9 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_9(&self) -> &crate::Reg<ltc0_ltc0_pke3_9::LTC0_LTC0_PKE3_9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3812usize)
                as *const crate::Reg<ltc0_ltc0_pke3_9::LTC0_LTC0_PKE3_9_SPEC>)
        }
    }
    #[doc = "0xee8 - LTC PKHA E 58 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_58(&self) -> &crate::Reg<ltc0_ltc0_pke_58::LTC0_LTC0_PKE_58_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3816usize)
                as *const crate::Reg<ltc0_ltc0_pke_58::LTC0_LTC0_PKE_58_SPEC>)
        }
    }
    #[doc = "0xee8 - LTC PKHA E3 10 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_10(&self) -> &crate::Reg<ltc0_ltc0_pke3_10::LTC0_LTC0_PKE3_10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3816usize)
                as *const crate::Reg<ltc0_ltc0_pke3_10::LTC0_LTC0_PKE3_10_SPEC>)
        }
    }
    #[doc = "0xeec - LTC PKHA E 59 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_59(&self) -> &crate::Reg<ltc0_ltc0_pke_59::LTC0_LTC0_PKE_59_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3820usize)
                as *const crate::Reg<ltc0_ltc0_pke_59::LTC0_LTC0_PKE_59_SPEC>)
        }
    }
    #[doc = "0xeec - LTC PKHA E3 11 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_11(&self) -> &crate::Reg<ltc0_ltc0_pke3_11::LTC0_LTC0_PKE3_11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3820usize)
                as *const crate::Reg<ltc0_ltc0_pke3_11::LTC0_LTC0_PKE3_11_SPEC>)
        }
    }
    #[doc = "0xef0 - LTC PKHA E 60 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_60(&self) -> &crate::Reg<ltc0_ltc0_pke_60::LTC0_LTC0_PKE_60_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3824usize)
                as *const crate::Reg<ltc0_ltc0_pke_60::LTC0_LTC0_PKE_60_SPEC>)
        }
    }
    #[doc = "0xef0 - LTC PKHA E3 12 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_12(&self) -> &crate::Reg<ltc0_ltc0_pke3_12::LTC0_LTC0_PKE3_12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3824usize)
                as *const crate::Reg<ltc0_ltc0_pke3_12::LTC0_LTC0_PKE3_12_SPEC>)
        }
    }
    #[doc = "0xef4 - LTC PKHA E 61 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_61(&self) -> &crate::Reg<ltc0_ltc0_pke_61::LTC0_LTC0_PKE_61_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3828usize)
                as *const crate::Reg<ltc0_ltc0_pke_61::LTC0_LTC0_PKE_61_SPEC>)
        }
    }
    #[doc = "0xef4 - LTC PKHA E3 13 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_13(&self) -> &crate::Reg<ltc0_ltc0_pke3_13::LTC0_LTC0_PKE3_13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3828usize)
                as *const crate::Reg<ltc0_ltc0_pke3_13::LTC0_LTC0_PKE3_13_SPEC>)
        }
    }
    #[doc = "0xef8 - LTC PKHA E 62 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_62(&self) -> &crate::Reg<ltc0_ltc0_pke_62::LTC0_LTC0_PKE_62_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3832usize)
                as *const crate::Reg<ltc0_ltc0_pke_62::LTC0_LTC0_PKE_62_SPEC>)
        }
    }
    #[doc = "0xef8 - LTC PKHA E3 14 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_14(&self) -> &crate::Reg<ltc0_ltc0_pke3_14::LTC0_LTC0_PKE3_14_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3832usize)
                as *const crate::Reg<ltc0_ltc0_pke3_14::LTC0_LTC0_PKE3_14_SPEC>)
        }
    }
    #[doc = "0xefc - LTC PKHA E 63 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke_63(&self) -> &crate::Reg<ltc0_ltc0_pke_63::LTC0_LTC0_PKE_63_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3836usize)
                as *const crate::Reg<ltc0_ltc0_pke_63::LTC0_LTC0_PKE_63_SPEC>)
        }
    }
    #[doc = "0xefc - LTC PKHA E3 15 Register"]
    #[inline(always)]
    pub fn ltc0_ltc0_pke3_15(&self) -> &crate::Reg<ltc0_ltc0_pke3_15::LTC0_LTC0_PKE3_15_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3836usize)
                as *const crate::Reg<ltc0_ltc0_pke3_15::LTC0_LTC0_PKE3_15_SPEC>)
        }
    }
}
#[doc = "LTC0_LTC0_MD register accessor: an alias for `Reg<LTC0_LTC0_MD_SPEC>`"]
pub type LTC0_LTC0_MD = crate::Reg<ltc0_ltc0_md::LTC0_LTC0_MD_SPEC>;
#[doc = "LTC Mode Register (non-PKHA/non-RNG use)"]
pub mod ltc0_ltc0_md;
#[doc = "LTC0_LTC0_MDPK register accessor: an alias for `Reg<LTC0_LTC0_MDPK_SPEC>`"]
pub type LTC0_LTC0_MDPK = crate::Reg<ltc0_ltc0_mdpk::LTC0_LTC0_MDPK_SPEC>;
#[doc = "LTC Mode Register (PublicKey)"]
pub mod ltc0_ltc0_mdpk;
#[doc = "LTC0_KS register accessor: an alias for `Reg<LTC0_KS_SPEC>`"]
pub type LTC0_KS = crate::Reg<ltc0_ks::LTC0_KS_SPEC>;
#[doc = "LTC Key Size Register"]
pub mod ltc0_ks;
#[doc = "LTC0_DS register accessor: an alias for `Reg<LTC0_DS_SPEC>`"]
pub type LTC0_DS = crate::Reg<ltc0_ds::LTC0_DS_SPEC>;
#[doc = "LTC Data Size Register"]
pub mod ltc0_ds;
#[doc = "LTC0_ICVS register accessor: an alias for `Reg<LTC0_ICVS_SPEC>`"]
pub type LTC0_ICVS = crate::Reg<ltc0_icvs::LTC0_ICVS_SPEC>;
#[doc = "LTC ICV Size Register"]
pub mod ltc0_icvs;
#[doc = "LTC0_COM register accessor: an alias for `Reg<LTC0_COM_SPEC>`"]
pub type LTC0_COM = crate::Reg<ltc0_com::LTC0_COM_SPEC>;
#[doc = "LTC Command Register"]
pub mod ltc0_com;
#[doc = "LTC0_CTL register accessor: an alias for `Reg<LTC0_CTL_SPEC>`"]
pub type LTC0_CTL = crate::Reg<ltc0_ctl::LTC0_CTL_SPEC>;
#[doc = "LTC Control Register"]
pub mod ltc0_ctl;
#[doc = "LTC0_CW register accessor: an alias for `Reg<LTC0_CW_SPEC>`"]
pub type LTC0_CW = crate::Reg<ltc0_cw::LTC0_CW_SPEC>;
#[doc = "LTC Clear Written Register"]
pub mod ltc0_cw;
#[doc = "LTC0_STA register accessor: an alias for `Reg<LTC0_STA_SPEC>`"]
pub type LTC0_STA = crate::Reg<ltc0_sta::LTC0_STA_SPEC>;
#[doc = "LTC Status Register"]
pub mod ltc0_sta;
#[doc = "LTC0_ESTA register accessor: an alias for `Reg<LTC0_ESTA_SPEC>`"]
pub type LTC0_ESTA = crate::Reg<ltc0_esta::LTC0_ESTA_SPEC>;
#[doc = "LTC Error Status Register"]
pub mod ltc0_esta;
#[doc = "LTC0_AADSZ register accessor: an alias for `Reg<LTC0_AADSZ_SPEC>`"]
pub type LTC0_AADSZ = crate::Reg<ltc0_aadsz::LTC0_AADSZ_SPEC>;
#[doc = "LTC AAD Size Register"]
pub mod ltc0_aadsz;
#[doc = "LTC0_IVSZ register accessor: an alias for `Reg<LTC0_IVSZ_SPEC>`"]
pub type LTC0_IVSZ = crate::Reg<ltc0_ivsz::LTC0_IVSZ_SPEC>;
#[doc = "LTC IV Size Register"]
pub mod ltc0_ivsz;
#[doc = "LTC0_DPAMS register accessor: an alias for `Reg<LTC0_DPAMS_SPEC>`"]
pub type LTC0_DPAMS = crate::Reg<ltc0_dpams::LTC0_DPAMS_SPEC>;
#[doc = "LTC DPA Mask Seed Register"]
pub mod ltc0_dpams;
#[doc = "LTC0_PKASZ register accessor: an alias for `Reg<LTC0_PKASZ_SPEC>`"]
pub type LTC0_PKASZ = crate::Reg<ltc0_pkasz::LTC0_PKASZ_SPEC>;
#[doc = "LTC PKHA A Size Register"]
pub mod ltc0_pkasz;
#[doc = "LTC0_PKBSZ register accessor: an alias for `Reg<LTC0_PKBSZ_SPEC>`"]
pub type LTC0_PKBSZ = crate::Reg<ltc0_pkbsz::LTC0_PKBSZ_SPEC>;
#[doc = "LTC PKHA B Size Register"]
pub mod ltc0_pkbsz;
#[doc = "LTC0_PKNSZ register accessor: an alias for `Reg<LTC0_PKNSZ_SPEC>`"]
pub type LTC0_PKNSZ = crate::Reg<ltc0_pknsz::LTC0_PKNSZ_SPEC>;
#[doc = "LTC PKHA N Size Register"]
pub mod ltc0_pknsz;
#[doc = "LTC0_PKESZ register accessor: an alias for `Reg<LTC0_PKESZ_SPEC>`"]
pub type LTC0_PKESZ = crate::Reg<ltc0_pkesz::LTC0_PKESZ_SPEC>;
#[doc = "LTC PKHA E Size Register"]
pub mod ltc0_pkesz;
#[doc = "LTC0_CTX_0 register accessor: an alias for `Reg<LTC0_CTX_0_SPEC>`"]
pub type LTC0_CTX_0 = crate::Reg<ltc0_ctx_0::LTC0_CTX_0_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_0;
#[doc = "LTC0_CTX_1 register accessor: an alias for `Reg<LTC0_CTX_1_SPEC>`"]
pub type LTC0_CTX_1 = crate::Reg<ltc0_ctx_1::LTC0_CTX_1_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_1;
#[doc = "LTC0_CTX_2 register accessor: an alias for `Reg<LTC0_CTX_2_SPEC>`"]
pub type LTC0_CTX_2 = crate::Reg<ltc0_ctx_2::LTC0_CTX_2_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_2;
#[doc = "LTC0_CTX_3 register accessor: an alias for `Reg<LTC0_CTX_3_SPEC>`"]
pub type LTC0_CTX_3 = crate::Reg<ltc0_ctx_3::LTC0_CTX_3_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_3;
#[doc = "LTC0_CTX_4 register accessor: an alias for `Reg<LTC0_CTX_4_SPEC>`"]
pub type LTC0_CTX_4 = crate::Reg<ltc0_ctx_4::LTC0_CTX_4_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_4;
#[doc = "LTC0_CTX_5 register accessor: an alias for `Reg<LTC0_CTX_5_SPEC>`"]
pub type LTC0_CTX_5 = crate::Reg<ltc0_ctx_5::LTC0_CTX_5_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_5;
#[doc = "LTC0_CTX_6 register accessor: an alias for `Reg<LTC0_CTX_6_SPEC>`"]
pub type LTC0_CTX_6 = crate::Reg<ltc0_ctx_6::LTC0_CTX_6_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_6;
#[doc = "LTC0_CTX_7 register accessor: an alias for `Reg<LTC0_CTX_7_SPEC>`"]
pub type LTC0_CTX_7 = crate::Reg<ltc0_ctx_7::LTC0_CTX_7_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_7;
#[doc = "LTC0_CTX_8 register accessor: an alias for `Reg<LTC0_CTX_8_SPEC>`"]
pub type LTC0_CTX_8 = crate::Reg<ltc0_ctx_8::LTC0_CTX_8_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_8;
#[doc = "LTC0_CTX_9 register accessor: an alias for `Reg<LTC0_CTX_9_SPEC>`"]
pub type LTC0_CTX_9 = crate::Reg<ltc0_ctx_9::LTC0_CTX_9_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_9;
#[doc = "LTC0_CTX_10 register accessor: an alias for `Reg<LTC0_CTX_10_SPEC>`"]
pub type LTC0_CTX_10 = crate::Reg<ltc0_ctx_10::LTC0_CTX_10_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_10;
#[doc = "LTC0_CTX_11 register accessor: an alias for `Reg<LTC0_CTX_11_SPEC>`"]
pub type LTC0_CTX_11 = crate::Reg<ltc0_ctx_11::LTC0_CTX_11_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_11;
#[doc = "LTC0_CTX_12 register accessor: an alias for `Reg<LTC0_CTX_12_SPEC>`"]
pub type LTC0_CTX_12 = crate::Reg<ltc0_ctx_12::LTC0_CTX_12_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_12;
#[doc = "LTC0_CTX_13 register accessor: an alias for `Reg<LTC0_CTX_13_SPEC>`"]
pub type LTC0_CTX_13 = crate::Reg<ltc0_ctx_13::LTC0_CTX_13_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_13;
#[doc = "LTC0_CTX_14 register accessor: an alias for `Reg<LTC0_CTX_14_SPEC>`"]
pub type LTC0_CTX_14 = crate::Reg<ltc0_ctx_14::LTC0_CTX_14_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_14;
#[doc = "LTC0_CTX_15 register accessor: an alias for `Reg<LTC0_CTX_15_SPEC>`"]
pub type LTC0_CTX_15 = crate::Reg<ltc0_ctx_15::LTC0_CTX_15_SPEC>;
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_15;
#[doc = "LTC0_KEY_0 register accessor: an alias for `Reg<LTC0_KEY_0_SPEC>`"]
pub type LTC0_KEY_0 = crate::Reg<ltc0_key_0::LTC0_KEY_0_SPEC>;
#[doc = "LTC Key Registers"]
pub mod ltc0_key_0;
#[doc = "LTC0_KEY_1 register accessor: an alias for `Reg<LTC0_KEY_1_SPEC>`"]
pub type LTC0_KEY_1 = crate::Reg<ltc0_key_1::LTC0_KEY_1_SPEC>;
#[doc = "LTC Key Registers"]
pub mod ltc0_key_1;
#[doc = "LTC0_KEY_2 register accessor: an alias for `Reg<LTC0_KEY_2_SPEC>`"]
pub type LTC0_KEY_2 = crate::Reg<ltc0_key_2::LTC0_KEY_2_SPEC>;
#[doc = "LTC Key Registers"]
pub mod ltc0_key_2;
#[doc = "LTC0_KEY_3 register accessor: an alias for `Reg<LTC0_KEY_3_SPEC>`"]
pub type LTC0_KEY_3 = crate::Reg<ltc0_key_3::LTC0_KEY_3_SPEC>;
#[doc = "LTC Key Registers"]
pub mod ltc0_key_3;
#[doc = "LTC0_KEY_4 register accessor: an alias for `Reg<LTC0_KEY_4_SPEC>`"]
pub type LTC0_KEY_4 = crate::Reg<ltc0_key_4::LTC0_KEY_4_SPEC>;
#[doc = "LTC Key Registers"]
pub mod ltc0_key_4;
#[doc = "LTC0_KEY_5 register accessor: an alias for `Reg<LTC0_KEY_5_SPEC>`"]
pub type LTC0_KEY_5 = crate::Reg<ltc0_key_5::LTC0_KEY_5_SPEC>;
#[doc = "LTC Key Registers"]
pub mod ltc0_key_5;
#[doc = "LTC0_KEY_6 register accessor: an alias for `Reg<LTC0_KEY_6_SPEC>`"]
pub type LTC0_KEY_6 = crate::Reg<ltc0_key_6::LTC0_KEY_6_SPEC>;
#[doc = "LTC Key Registers"]
pub mod ltc0_key_6;
#[doc = "LTC0_KEY_7 register accessor: an alias for `Reg<LTC0_KEY_7_SPEC>`"]
pub type LTC0_KEY_7 = crate::Reg<ltc0_key_7::LTC0_KEY_7_SPEC>;
#[doc = "LTC Key Registers"]
pub mod ltc0_key_7;
#[doc = "LTC0_VID1 register accessor: an alias for `Reg<LTC0_VID1_SPEC>`"]
pub type LTC0_VID1 = crate::Reg<ltc0_vid1::LTC0_VID1_SPEC>;
#[doc = "LTC Version ID Register"]
pub mod ltc0_vid1;
#[doc = "LTC0_VID2 register accessor: an alias for `Reg<LTC0_VID2_SPEC>`"]
pub type LTC0_VID2 = crate::Reg<ltc0_vid2::LTC0_VID2_SPEC>;
#[doc = "LTC Version ID 2 Register"]
pub mod ltc0_vid2;
#[doc = "LTC0_CHAVID register accessor: an alias for `Reg<LTC0_CHAVID_SPEC>`"]
pub type LTC0_CHAVID = crate::Reg<ltc0_chavid::LTC0_CHAVID_SPEC>;
#[doc = "LTC CHA Version ID Register"]
pub mod ltc0_chavid;
#[doc = "LTC0_FIFOSTA register accessor: an alias for `Reg<LTC0_FIFOSTA_SPEC>`"]
pub type LTC0_FIFOSTA = crate::Reg<ltc0_fifosta::LTC0_FIFOSTA_SPEC>;
#[doc = "LTC FIFO Status Register"]
pub mod ltc0_fifosta;
#[doc = "LTC0_IFIFO register accessor: an alias for `Reg<LTC0_IFIFO_SPEC>`"]
pub type LTC0_IFIFO = crate::Reg<ltc0_ififo::LTC0_IFIFO_SPEC>;
#[doc = "LTC Input Data FIFO"]
pub mod ltc0_ififo;
#[doc = "LTC0_OFIFO register accessor: an alias for `Reg<LTC0_OFIFO_SPEC>`"]
pub type LTC0_OFIFO = crate::Reg<ltc0_ofifo::LTC0_OFIFO_SPEC>;
#[doc = "LTC Output Data FIFO"]
pub mod ltc0_ofifo;
#[doc = "LTC0_LTC0_PKA0_0 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_0_SPEC>`"]
pub type LTC0_LTC0_PKA0_0 = crate::Reg<ltc0_ltc0_pka0_0::LTC0_LTC0_PKA0_0_SPEC>;
#[doc = "LTC PKHA A0 0 Register"]
pub mod ltc0_ltc0_pka0_0;
#[doc = "LTC0_LTC0_PKA_0 register accessor: an alias for `Reg<LTC0_LTC0_PKA_0_SPEC>`"]
pub type LTC0_LTC0_PKA_0 = crate::Reg<ltc0_ltc0_pka_0::LTC0_LTC0_PKA_0_SPEC>;
#[doc = "LTC PKHA A 0 Register"]
pub mod ltc0_ltc0_pka_0;
#[doc = "LTC0_LTC0_PKA0_1 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_1_SPEC>`"]
pub type LTC0_LTC0_PKA0_1 = crate::Reg<ltc0_ltc0_pka0_1::LTC0_LTC0_PKA0_1_SPEC>;
#[doc = "LTC PKHA A0 1 Register"]
pub mod ltc0_ltc0_pka0_1;
#[doc = "LTC0_LTC0_PKA_1 register accessor: an alias for `Reg<LTC0_LTC0_PKA_1_SPEC>`"]
pub type LTC0_LTC0_PKA_1 = crate::Reg<ltc0_ltc0_pka_1::LTC0_LTC0_PKA_1_SPEC>;
#[doc = "LTC PKHA A 1 Register"]
pub mod ltc0_ltc0_pka_1;
#[doc = "LTC0_LTC0_PKA0_2 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_2_SPEC>`"]
pub type LTC0_LTC0_PKA0_2 = crate::Reg<ltc0_ltc0_pka0_2::LTC0_LTC0_PKA0_2_SPEC>;
#[doc = "LTC PKHA A0 2 Register"]
pub mod ltc0_ltc0_pka0_2;
#[doc = "LTC0_LTC0_PKA_2 register accessor: an alias for `Reg<LTC0_LTC0_PKA_2_SPEC>`"]
pub type LTC0_LTC0_PKA_2 = crate::Reg<ltc0_ltc0_pka_2::LTC0_LTC0_PKA_2_SPEC>;
#[doc = "LTC PKHA A 2 Register"]
pub mod ltc0_ltc0_pka_2;
#[doc = "LTC0_LTC0_PKA0_3 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_3_SPEC>`"]
pub type LTC0_LTC0_PKA0_3 = crate::Reg<ltc0_ltc0_pka0_3::LTC0_LTC0_PKA0_3_SPEC>;
#[doc = "LTC PKHA A0 3 Register"]
pub mod ltc0_ltc0_pka0_3;
#[doc = "LTC0_LTC0_PKA_3 register accessor: an alias for `Reg<LTC0_LTC0_PKA_3_SPEC>`"]
pub type LTC0_LTC0_PKA_3 = crate::Reg<ltc0_ltc0_pka_3::LTC0_LTC0_PKA_3_SPEC>;
#[doc = "LTC PKHA A 3 Register"]
pub mod ltc0_ltc0_pka_3;
#[doc = "LTC0_LTC0_PKA0_4 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_4_SPEC>`"]
pub type LTC0_LTC0_PKA0_4 = crate::Reg<ltc0_ltc0_pka0_4::LTC0_LTC0_PKA0_4_SPEC>;
#[doc = "LTC PKHA A0 4 Register"]
pub mod ltc0_ltc0_pka0_4;
#[doc = "LTC0_LTC0_PKA_4 register accessor: an alias for `Reg<LTC0_LTC0_PKA_4_SPEC>`"]
pub type LTC0_LTC0_PKA_4 = crate::Reg<ltc0_ltc0_pka_4::LTC0_LTC0_PKA_4_SPEC>;
#[doc = "LTC PKHA A 4 Register"]
pub mod ltc0_ltc0_pka_4;
#[doc = "LTC0_LTC0_PKA0_5 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_5_SPEC>`"]
pub type LTC0_LTC0_PKA0_5 = crate::Reg<ltc0_ltc0_pka0_5::LTC0_LTC0_PKA0_5_SPEC>;
#[doc = "LTC PKHA A0 5 Register"]
pub mod ltc0_ltc0_pka0_5;
#[doc = "LTC0_LTC0_PKA_5 register accessor: an alias for `Reg<LTC0_LTC0_PKA_5_SPEC>`"]
pub type LTC0_LTC0_PKA_5 = crate::Reg<ltc0_ltc0_pka_5::LTC0_LTC0_PKA_5_SPEC>;
#[doc = "LTC PKHA A 5 Register"]
pub mod ltc0_ltc0_pka_5;
#[doc = "LTC0_LTC0_PKA0_6 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_6_SPEC>`"]
pub type LTC0_LTC0_PKA0_6 = crate::Reg<ltc0_ltc0_pka0_6::LTC0_LTC0_PKA0_6_SPEC>;
#[doc = "LTC PKHA A0 6 Register"]
pub mod ltc0_ltc0_pka0_6;
#[doc = "LTC0_LTC0_PKA_6 register accessor: an alias for `Reg<LTC0_LTC0_PKA_6_SPEC>`"]
pub type LTC0_LTC0_PKA_6 = crate::Reg<ltc0_ltc0_pka_6::LTC0_LTC0_PKA_6_SPEC>;
#[doc = "LTC PKHA A 6 Register"]
pub mod ltc0_ltc0_pka_6;
#[doc = "LTC0_LTC0_PKA0_7 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_7_SPEC>`"]
pub type LTC0_LTC0_PKA0_7 = crate::Reg<ltc0_ltc0_pka0_7::LTC0_LTC0_PKA0_7_SPEC>;
#[doc = "LTC PKHA A0 7 Register"]
pub mod ltc0_ltc0_pka0_7;
#[doc = "LTC0_LTC0_PKA_7 register accessor: an alias for `Reg<LTC0_LTC0_PKA_7_SPEC>`"]
pub type LTC0_LTC0_PKA_7 = crate::Reg<ltc0_ltc0_pka_7::LTC0_LTC0_PKA_7_SPEC>;
#[doc = "LTC PKHA A 7 Register"]
pub mod ltc0_ltc0_pka_7;
#[doc = "LTC0_LTC0_PKA0_8 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_8_SPEC>`"]
pub type LTC0_LTC0_PKA0_8 = crate::Reg<ltc0_ltc0_pka0_8::LTC0_LTC0_PKA0_8_SPEC>;
#[doc = "LTC PKHA A0 8 Register"]
pub mod ltc0_ltc0_pka0_8;
#[doc = "LTC0_LTC0_PKA_8 register accessor: an alias for `Reg<LTC0_LTC0_PKA_8_SPEC>`"]
pub type LTC0_LTC0_PKA_8 = crate::Reg<ltc0_ltc0_pka_8::LTC0_LTC0_PKA_8_SPEC>;
#[doc = "LTC PKHA A 8 Register"]
pub mod ltc0_ltc0_pka_8;
#[doc = "LTC0_LTC0_PKA0_9 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_9_SPEC>`"]
pub type LTC0_LTC0_PKA0_9 = crate::Reg<ltc0_ltc0_pka0_9::LTC0_LTC0_PKA0_9_SPEC>;
#[doc = "LTC PKHA A0 9 Register"]
pub mod ltc0_ltc0_pka0_9;
#[doc = "LTC0_LTC0_PKA_9 register accessor: an alias for `Reg<LTC0_LTC0_PKA_9_SPEC>`"]
pub type LTC0_LTC0_PKA_9 = crate::Reg<ltc0_ltc0_pka_9::LTC0_LTC0_PKA_9_SPEC>;
#[doc = "LTC PKHA A 9 Register"]
pub mod ltc0_ltc0_pka_9;
#[doc = "LTC0_LTC0_PKA0_10 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_10_SPEC>`"]
pub type LTC0_LTC0_PKA0_10 = crate::Reg<ltc0_ltc0_pka0_10::LTC0_LTC0_PKA0_10_SPEC>;
#[doc = "LTC PKHA A0 10 Register"]
pub mod ltc0_ltc0_pka0_10;
#[doc = "LTC0_LTC0_PKA_10 register accessor: an alias for `Reg<LTC0_LTC0_PKA_10_SPEC>`"]
pub type LTC0_LTC0_PKA_10 = crate::Reg<ltc0_ltc0_pka_10::LTC0_LTC0_PKA_10_SPEC>;
#[doc = "LTC PKHA A 10 Register"]
pub mod ltc0_ltc0_pka_10;
#[doc = "LTC0_LTC0_PKA0_11 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_11_SPEC>`"]
pub type LTC0_LTC0_PKA0_11 = crate::Reg<ltc0_ltc0_pka0_11::LTC0_LTC0_PKA0_11_SPEC>;
#[doc = "LTC PKHA A0 11 Register"]
pub mod ltc0_ltc0_pka0_11;
#[doc = "LTC0_LTC0_PKA_11 register accessor: an alias for `Reg<LTC0_LTC0_PKA_11_SPEC>`"]
pub type LTC0_LTC0_PKA_11 = crate::Reg<ltc0_ltc0_pka_11::LTC0_LTC0_PKA_11_SPEC>;
#[doc = "LTC PKHA A 11 Register"]
pub mod ltc0_ltc0_pka_11;
#[doc = "LTC0_LTC0_PKA0_12 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_12_SPEC>`"]
pub type LTC0_LTC0_PKA0_12 = crate::Reg<ltc0_ltc0_pka0_12::LTC0_LTC0_PKA0_12_SPEC>;
#[doc = "LTC PKHA A0 12 Register"]
pub mod ltc0_ltc0_pka0_12;
#[doc = "LTC0_LTC0_PKA_12 register accessor: an alias for `Reg<LTC0_LTC0_PKA_12_SPEC>`"]
pub type LTC0_LTC0_PKA_12 = crate::Reg<ltc0_ltc0_pka_12::LTC0_LTC0_PKA_12_SPEC>;
#[doc = "LTC PKHA A 12 Register"]
pub mod ltc0_ltc0_pka_12;
#[doc = "LTC0_LTC0_PKA0_13 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_13_SPEC>`"]
pub type LTC0_LTC0_PKA0_13 = crate::Reg<ltc0_ltc0_pka0_13::LTC0_LTC0_PKA0_13_SPEC>;
#[doc = "LTC PKHA A0 13 Register"]
pub mod ltc0_ltc0_pka0_13;
#[doc = "LTC0_LTC0_PKA_13 register accessor: an alias for `Reg<LTC0_LTC0_PKA_13_SPEC>`"]
pub type LTC0_LTC0_PKA_13 = crate::Reg<ltc0_ltc0_pka_13::LTC0_LTC0_PKA_13_SPEC>;
#[doc = "LTC PKHA A 13 Register"]
pub mod ltc0_ltc0_pka_13;
#[doc = "LTC0_LTC0_PKA0_14 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_14_SPEC>`"]
pub type LTC0_LTC0_PKA0_14 = crate::Reg<ltc0_ltc0_pka0_14::LTC0_LTC0_PKA0_14_SPEC>;
#[doc = "LTC PKHA A0 14 Register"]
pub mod ltc0_ltc0_pka0_14;
#[doc = "LTC0_LTC0_PKA_14 register accessor: an alias for `Reg<LTC0_LTC0_PKA_14_SPEC>`"]
pub type LTC0_LTC0_PKA_14 = crate::Reg<ltc0_ltc0_pka_14::LTC0_LTC0_PKA_14_SPEC>;
#[doc = "LTC PKHA A 14 Register"]
pub mod ltc0_ltc0_pka_14;
#[doc = "LTC0_LTC0_PKA0_15 register accessor: an alias for `Reg<LTC0_LTC0_PKA0_15_SPEC>`"]
pub type LTC0_LTC0_PKA0_15 = crate::Reg<ltc0_ltc0_pka0_15::LTC0_LTC0_PKA0_15_SPEC>;
#[doc = "LTC PKHA A0 15 Register"]
pub mod ltc0_ltc0_pka0_15;
#[doc = "LTC0_LTC0_PKA_15 register accessor: an alias for `Reg<LTC0_LTC0_PKA_15_SPEC>`"]
pub type LTC0_LTC0_PKA_15 = crate::Reg<ltc0_ltc0_pka_15::LTC0_LTC0_PKA_15_SPEC>;
#[doc = "LTC PKHA A 15 Register"]
pub mod ltc0_ltc0_pka_15;
#[doc = "LTC0_LTC0_PKA1_0 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_0_SPEC>`"]
pub type LTC0_LTC0_PKA1_0 = crate::Reg<ltc0_ltc0_pka1_0::LTC0_LTC0_PKA1_0_SPEC>;
#[doc = "LTC PKHA A1 0 Register"]
pub mod ltc0_ltc0_pka1_0;
#[doc = "LTC0_LTC0_PKA_16 register accessor: an alias for `Reg<LTC0_LTC0_PKA_16_SPEC>`"]
pub type LTC0_LTC0_PKA_16 = crate::Reg<ltc0_ltc0_pka_16::LTC0_LTC0_PKA_16_SPEC>;
#[doc = "LTC PKHA A 16 Register"]
pub mod ltc0_ltc0_pka_16;
#[doc = "LTC0_LTC0_PKA1_1 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_1_SPEC>`"]
pub type LTC0_LTC0_PKA1_1 = crate::Reg<ltc0_ltc0_pka1_1::LTC0_LTC0_PKA1_1_SPEC>;
#[doc = "LTC PKHA A1 1 Register"]
pub mod ltc0_ltc0_pka1_1;
#[doc = "LTC0_LTC0_PKA_17 register accessor: an alias for `Reg<LTC0_LTC0_PKA_17_SPEC>`"]
pub type LTC0_LTC0_PKA_17 = crate::Reg<ltc0_ltc0_pka_17::LTC0_LTC0_PKA_17_SPEC>;
#[doc = "LTC PKHA A 17 Register"]
pub mod ltc0_ltc0_pka_17;
#[doc = "LTC0_LTC0_PKA1_2 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_2_SPEC>`"]
pub type LTC0_LTC0_PKA1_2 = crate::Reg<ltc0_ltc0_pka1_2::LTC0_LTC0_PKA1_2_SPEC>;
#[doc = "LTC PKHA A1 2 Register"]
pub mod ltc0_ltc0_pka1_2;
#[doc = "LTC0_LTC0_PKA_18 register accessor: an alias for `Reg<LTC0_LTC0_PKA_18_SPEC>`"]
pub type LTC0_LTC0_PKA_18 = crate::Reg<ltc0_ltc0_pka_18::LTC0_LTC0_PKA_18_SPEC>;
#[doc = "LTC PKHA A 18 Register"]
pub mod ltc0_ltc0_pka_18;
#[doc = "LTC0_LTC0_PKA1_3 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_3_SPEC>`"]
pub type LTC0_LTC0_PKA1_3 = crate::Reg<ltc0_ltc0_pka1_3::LTC0_LTC0_PKA1_3_SPEC>;
#[doc = "LTC PKHA A1 3 Register"]
pub mod ltc0_ltc0_pka1_3;
#[doc = "LTC0_LTC0_PKA_19 register accessor: an alias for `Reg<LTC0_LTC0_PKA_19_SPEC>`"]
pub type LTC0_LTC0_PKA_19 = crate::Reg<ltc0_ltc0_pka_19::LTC0_LTC0_PKA_19_SPEC>;
#[doc = "LTC PKHA A 19 Register"]
pub mod ltc0_ltc0_pka_19;
#[doc = "LTC0_LTC0_PKA1_4 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_4_SPEC>`"]
pub type LTC0_LTC0_PKA1_4 = crate::Reg<ltc0_ltc0_pka1_4::LTC0_LTC0_PKA1_4_SPEC>;
#[doc = "LTC PKHA A1 4 Register"]
pub mod ltc0_ltc0_pka1_4;
#[doc = "LTC0_LTC0_PKA_20 register accessor: an alias for `Reg<LTC0_LTC0_PKA_20_SPEC>`"]
pub type LTC0_LTC0_PKA_20 = crate::Reg<ltc0_ltc0_pka_20::LTC0_LTC0_PKA_20_SPEC>;
#[doc = "LTC PKHA A 20 Register"]
pub mod ltc0_ltc0_pka_20;
#[doc = "LTC0_LTC0_PKA1_5 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_5_SPEC>`"]
pub type LTC0_LTC0_PKA1_5 = crate::Reg<ltc0_ltc0_pka1_5::LTC0_LTC0_PKA1_5_SPEC>;
#[doc = "LTC PKHA A1 5 Register"]
pub mod ltc0_ltc0_pka1_5;
#[doc = "LTC0_LTC0_PKA_21 register accessor: an alias for `Reg<LTC0_LTC0_PKA_21_SPEC>`"]
pub type LTC0_LTC0_PKA_21 = crate::Reg<ltc0_ltc0_pka_21::LTC0_LTC0_PKA_21_SPEC>;
#[doc = "LTC PKHA A 21 Register"]
pub mod ltc0_ltc0_pka_21;
#[doc = "LTC0_LTC0_PKA1_6 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_6_SPEC>`"]
pub type LTC0_LTC0_PKA1_6 = crate::Reg<ltc0_ltc0_pka1_6::LTC0_LTC0_PKA1_6_SPEC>;
#[doc = "LTC PKHA A1 6 Register"]
pub mod ltc0_ltc0_pka1_6;
#[doc = "LTC0_LTC0_PKA_22 register accessor: an alias for `Reg<LTC0_LTC0_PKA_22_SPEC>`"]
pub type LTC0_LTC0_PKA_22 = crate::Reg<ltc0_ltc0_pka_22::LTC0_LTC0_PKA_22_SPEC>;
#[doc = "LTC PKHA A 22 Register"]
pub mod ltc0_ltc0_pka_22;
#[doc = "LTC0_LTC0_PKA1_7 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_7_SPEC>`"]
pub type LTC0_LTC0_PKA1_7 = crate::Reg<ltc0_ltc0_pka1_7::LTC0_LTC0_PKA1_7_SPEC>;
#[doc = "LTC PKHA A1 7 Register"]
pub mod ltc0_ltc0_pka1_7;
#[doc = "LTC0_LTC0_PKA_23 register accessor: an alias for `Reg<LTC0_LTC0_PKA_23_SPEC>`"]
pub type LTC0_LTC0_PKA_23 = crate::Reg<ltc0_ltc0_pka_23::LTC0_LTC0_PKA_23_SPEC>;
#[doc = "LTC PKHA A 23 Register"]
pub mod ltc0_ltc0_pka_23;
#[doc = "LTC0_LTC0_PKA1_8 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_8_SPEC>`"]
pub type LTC0_LTC0_PKA1_8 = crate::Reg<ltc0_ltc0_pka1_8::LTC0_LTC0_PKA1_8_SPEC>;
#[doc = "LTC PKHA A1 8 Register"]
pub mod ltc0_ltc0_pka1_8;
#[doc = "LTC0_LTC0_PKA_24 register accessor: an alias for `Reg<LTC0_LTC0_PKA_24_SPEC>`"]
pub type LTC0_LTC0_PKA_24 = crate::Reg<ltc0_ltc0_pka_24::LTC0_LTC0_PKA_24_SPEC>;
#[doc = "LTC PKHA A 24 Register"]
pub mod ltc0_ltc0_pka_24;
#[doc = "LTC0_LTC0_PKA1_9 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_9_SPEC>`"]
pub type LTC0_LTC0_PKA1_9 = crate::Reg<ltc0_ltc0_pka1_9::LTC0_LTC0_PKA1_9_SPEC>;
#[doc = "LTC PKHA A1 9 Register"]
pub mod ltc0_ltc0_pka1_9;
#[doc = "LTC0_LTC0_PKA_25 register accessor: an alias for `Reg<LTC0_LTC0_PKA_25_SPEC>`"]
pub type LTC0_LTC0_PKA_25 = crate::Reg<ltc0_ltc0_pka_25::LTC0_LTC0_PKA_25_SPEC>;
#[doc = "LTC PKHA A 25 Register"]
pub mod ltc0_ltc0_pka_25;
#[doc = "LTC0_LTC0_PKA1_10 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_10_SPEC>`"]
pub type LTC0_LTC0_PKA1_10 = crate::Reg<ltc0_ltc0_pka1_10::LTC0_LTC0_PKA1_10_SPEC>;
#[doc = "LTC PKHA A1 10 Register"]
pub mod ltc0_ltc0_pka1_10;
#[doc = "LTC0_LTC0_PKA_26 register accessor: an alias for `Reg<LTC0_LTC0_PKA_26_SPEC>`"]
pub type LTC0_LTC0_PKA_26 = crate::Reg<ltc0_ltc0_pka_26::LTC0_LTC0_PKA_26_SPEC>;
#[doc = "LTC PKHA A 26 Register"]
pub mod ltc0_ltc0_pka_26;
#[doc = "LTC0_LTC0_PKA1_11 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_11_SPEC>`"]
pub type LTC0_LTC0_PKA1_11 = crate::Reg<ltc0_ltc0_pka1_11::LTC0_LTC0_PKA1_11_SPEC>;
#[doc = "LTC PKHA A1 11 Register"]
pub mod ltc0_ltc0_pka1_11;
#[doc = "LTC0_LTC0_PKA_27 register accessor: an alias for `Reg<LTC0_LTC0_PKA_27_SPEC>`"]
pub type LTC0_LTC0_PKA_27 = crate::Reg<ltc0_ltc0_pka_27::LTC0_LTC0_PKA_27_SPEC>;
#[doc = "LTC PKHA A 27 Register"]
pub mod ltc0_ltc0_pka_27;
#[doc = "LTC0_LTC0_PKA1_12 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_12_SPEC>`"]
pub type LTC0_LTC0_PKA1_12 = crate::Reg<ltc0_ltc0_pka1_12::LTC0_LTC0_PKA1_12_SPEC>;
#[doc = "LTC PKHA A1 12 Register"]
pub mod ltc0_ltc0_pka1_12;
#[doc = "LTC0_LTC0_PKA_28 register accessor: an alias for `Reg<LTC0_LTC0_PKA_28_SPEC>`"]
pub type LTC0_LTC0_PKA_28 = crate::Reg<ltc0_ltc0_pka_28::LTC0_LTC0_PKA_28_SPEC>;
#[doc = "LTC PKHA A 28 Register"]
pub mod ltc0_ltc0_pka_28;
#[doc = "LTC0_LTC0_PKA1_13 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_13_SPEC>`"]
pub type LTC0_LTC0_PKA1_13 = crate::Reg<ltc0_ltc0_pka1_13::LTC0_LTC0_PKA1_13_SPEC>;
#[doc = "LTC PKHA A1 13 Register"]
pub mod ltc0_ltc0_pka1_13;
#[doc = "LTC0_LTC0_PKA_29 register accessor: an alias for `Reg<LTC0_LTC0_PKA_29_SPEC>`"]
pub type LTC0_LTC0_PKA_29 = crate::Reg<ltc0_ltc0_pka_29::LTC0_LTC0_PKA_29_SPEC>;
#[doc = "LTC PKHA A 29 Register"]
pub mod ltc0_ltc0_pka_29;
#[doc = "LTC0_LTC0_PKA1_14 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_14_SPEC>`"]
pub type LTC0_LTC0_PKA1_14 = crate::Reg<ltc0_ltc0_pka1_14::LTC0_LTC0_PKA1_14_SPEC>;
#[doc = "LTC PKHA A1 14 Register"]
pub mod ltc0_ltc0_pka1_14;
#[doc = "LTC0_LTC0_PKA_30 register accessor: an alias for `Reg<LTC0_LTC0_PKA_30_SPEC>`"]
pub type LTC0_LTC0_PKA_30 = crate::Reg<ltc0_ltc0_pka_30::LTC0_LTC0_PKA_30_SPEC>;
#[doc = "LTC PKHA A 30 Register"]
pub mod ltc0_ltc0_pka_30;
#[doc = "LTC0_LTC0_PKA1_15 register accessor: an alias for `Reg<LTC0_LTC0_PKA1_15_SPEC>`"]
pub type LTC0_LTC0_PKA1_15 = crate::Reg<ltc0_ltc0_pka1_15::LTC0_LTC0_PKA1_15_SPEC>;
#[doc = "LTC PKHA A1 15 Register"]
pub mod ltc0_ltc0_pka1_15;
#[doc = "LTC0_LTC0_PKA_31 register accessor: an alias for `Reg<LTC0_LTC0_PKA_31_SPEC>`"]
pub type LTC0_LTC0_PKA_31 = crate::Reg<ltc0_ltc0_pka_31::LTC0_LTC0_PKA_31_SPEC>;
#[doc = "LTC PKHA A 31 Register"]
pub mod ltc0_ltc0_pka_31;
#[doc = "LTC0_LTC0_PKA2_0 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_0_SPEC>`"]
pub type LTC0_LTC0_PKA2_0 = crate::Reg<ltc0_ltc0_pka2_0::LTC0_LTC0_PKA2_0_SPEC>;
#[doc = "LTC PKHA A2 0 Register"]
pub mod ltc0_ltc0_pka2_0;
#[doc = "LTC0_LTC0_PKA_32 register accessor: an alias for `Reg<LTC0_LTC0_PKA_32_SPEC>`"]
pub type LTC0_LTC0_PKA_32 = crate::Reg<ltc0_ltc0_pka_32::LTC0_LTC0_PKA_32_SPEC>;
#[doc = "LTC PKHA A 32 Register"]
pub mod ltc0_ltc0_pka_32;
#[doc = "LTC0_LTC0_PKA2_1 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_1_SPEC>`"]
pub type LTC0_LTC0_PKA2_1 = crate::Reg<ltc0_ltc0_pka2_1::LTC0_LTC0_PKA2_1_SPEC>;
#[doc = "LTC PKHA A2 1 Register"]
pub mod ltc0_ltc0_pka2_1;
#[doc = "LTC0_LTC0_PKA_33 register accessor: an alias for `Reg<LTC0_LTC0_PKA_33_SPEC>`"]
pub type LTC0_LTC0_PKA_33 = crate::Reg<ltc0_ltc0_pka_33::LTC0_LTC0_PKA_33_SPEC>;
#[doc = "LTC PKHA A 33 Register"]
pub mod ltc0_ltc0_pka_33;
#[doc = "LTC0_LTC0_PKA2_2 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_2_SPEC>`"]
pub type LTC0_LTC0_PKA2_2 = crate::Reg<ltc0_ltc0_pka2_2::LTC0_LTC0_PKA2_2_SPEC>;
#[doc = "LTC PKHA A2 2 Register"]
pub mod ltc0_ltc0_pka2_2;
#[doc = "LTC0_LTC0_PKA_34 register accessor: an alias for `Reg<LTC0_LTC0_PKA_34_SPEC>`"]
pub type LTC0_LTC0_PKA_34 = crate::Reg<ltc0_ltc0_pka_34::LTC0_LTC0_PKA_34_SPEC>;
#[doc = "LTC PKHA A 34 Register"]
pub mod ltc0_ltc0_pka_34;
#[doc = "LTC0_LTC0_PKA2_3 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_3_SPEC>`"]
pub type LTC0_LTC0_PKA2_3 = crate::Reg<ltc0_ltc0_pka2_3::LTC0_LTC0_PKA2_3_SPEC>;
#[doc = "LTC PKHA A2 3 Register"]
pub mod ltc0_ltc0_pka2_3;
#[doc = "LTC0_LTC0_PKA_35 register accessor: an alias for `Reg<LTC0_LTC0_PKA_35_SPEC>`"]
pub type LTC0_LTC0_PKA_35 = crate::Reg<ltc0_ltc0_pka_35::LTC0_LTC0_PKA_35_SPEC>;
#[doc = "LTC PKHA A 35 Register"]
pub mod ltc0_ltc0_pka_35;
#[doc = "LTC0_LTC0_PKA2_4 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_4_SPEC>`"]
pub type LTC0_LTC0_PKA2_4 = crate::Reg<ltc0_ltc0_pka2_4::LTC0_LTC0_PKA2_4_SPEC>;
#[doc = "LTC PKHA A2 4 Register"]
pub mod ltc0_ltc0_pka2_4;
#[doc = "LTC0_LTC0_PKA_36 register accessor: an alias for `Reg<LTC0_LTC0_PKA_36_SPEC>`"]
pub type LTC0_LTC0_PKA_36 = crate::Reg<ltc0_ltc0_pka_36::LTC0_LTC0_PKA_36_SPEC>;
#[doc = "LTC PKHA A 36 Register"]
pub mod ltc0_ltc0_pka_36;
#[doc = "LTC0_LTC0_PKA2_5 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_5_SPEC>`"]
pub type LTC0_LTC0_PKA2_5 = crate::Reg<ltc0_ltc0_pka2_5::LTC0_LTC0_PKA2_5_SPEC>;
#[doc = "LTC PKHA A2 5 Register"]
pub mod ltc0_ltc0_pka2_5;
#[doc = "LTC0_LTC0_PKA_37 register accessor: an alias for `Reg<LTC0_LTC0_PKA_37_SPEC>`"]
pub type LTC0_LTC0_PKA_37 = crate::Reg<ltc0_ltc0_pka_37::LTC0_LTC0_PKA_37_SPEC>;
#[doc = "LTC PKHA A 37 Register"]
pub mod ltc0_ltc0_pka_37;
#[doc = "LTC0_LTC0_PKA2_6 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_6_SPEC>`"]
pub type LTC0_LTC0_PKA2_6 = crate::Reg<ltc0_ltc0_pka2_6::LTC0_LTC0_PKA2_6_SPEC>;
#[doc = "LTC PKHA A2 6 Register"]
pub mod ltc0_ltc0_pka2_6;
#[doc = "LTC0_LTC0_PKA_38 register accessor: an alias for `Reg<LTC0_LTC0_PKA_38_SPEC>`"]
pub type LTC0_LTC0_PKA_38 = crate::Reg<ltc0_ltc0_pka_38::LTC0_LTC0_PKA_38_SPEC>;
#[doc = "LTC PKHA A 38 Register"]
pub mod ltc0_ltc0_pka_38;
#[doc = "LTC0_LTC0_PKA2_7 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_7_SPEC>`"]
pub type LTC0_LTC0_PKA2_7 = crate::Reg<ltc0_ltc0_pka2_7::LTC0_LTC0_PKA2_7_SPEC>;
#[doc = "LTC PKHA A2 7 Register"]
pub mod ltc0_ltc0_pka2_7;
#[doc = "LTC0_LTC0_PKA_39 register accessor: an alias for `Reg<LTC0_LTC0_PKA_39_SPEC>`"]
pub type LTC0_LTC0_PKA_39 = crate::Reg<ltc0_ltc0_pka_39::LTC0_LTC0_PKA_39_SPEC>;
#[doc = "LTC PKHA A 39 Register"]
pub mod ltc0_ltc0_pka_39;
#[doc = "LTC0_LTC0_PKA2_8 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_8_SPEC>`"]
pub type LTC0_LTC0_PKA2_8 = crate::Reg<ltc0_ltc0_pka2_8::LTC0_LTC0_PKA2_8_SPEC>;
#[doc = "LTC PKHA A2 8 Register"]
pub mod ltc0_ltc0_pka2_8;
#[doc = "LTC0_LTC0_PKA_40 register accessor: an alias for `Reg<LTC0_LTC0_PKA_40_SPEC>`"]
pub type LTC0_LTC0_PKA_40 = crate::Reg<ltc0_ltc0_pka_40::LTC0_LTC0_PKA_40_SPEC>;
#[doc = "LTC PKHA A 40 Register"]
pub mod ltc0_ltc0_pka_40;
#[doc = "LTC0_LTC0_PKA2_9 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_9_SPEC>`"]
pub type LTC0_LTC0_PKA2_9 = crate::Reg<ltc0_ltc0_pka2_9::LTC0_LTC0_PKA2_9_SPEC>;
#[doc = "LTC PKHA A2 9 Register"]
pub mod ltc0_ltc0_pka2_9;
#[doc = "LTC0_LTC0_PKA_41 register accessor: an alias for `Reg<LTC0_LTC0_PKA_41_SPEC>`"]
pub type LTC0_LTC0_PKA_41 = crate::Reg<ltc0_ltc0_pka_41::LTC0_LTC0_PKA_41_SPEC>;
#[doc = "LTC PKHA A 41 Register"]
pub mod ltc0_ltc0_pka_41;
#[doc = "LTC0_LTC0_PKA2_10 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_10_SPEC>`"]
pub type LTC0_LTC0_PKA2_10 = crate::Reg<ltc0_ltc0_pka2_10::LTC0_LTC0_PKA2_10_SPEC>;
#[doc = "LTC PKHA A2 10 Register"]
pub mod ltc0_ltc0_pka2_10;
#[doc = "LTC0_LTC0_PKA_42 register accessor: an alias for `Reg<LTC0_LTC0_PKA_42_SPEC>`"]
pub type LTC0_LTC0_PKA_42 = crate::Reg<ltc0_ltc0_pka_42::LTC0_LTC0_PKA_42_SPEC>;
#[doc = "LTC PKHA A 42 Register"]
pub mod ltc0_ltc0_pka_42;
#[doc = "LTC0_LTC0_PKA2_11 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_11_SPEC>`"]
pub type LTC0_LTC0_PKA2_11 = crate::Reg<ltc0_ltc0_pka2_11::LTC0_LTC0_PKA2_11_SPEC>;
#[doc = "LTC PKHA A2 11 Register"]
pub mod ltc0_ltc0_pka2_11;
#[doc = "LTC0_LTC0_PKA_43 register accessor: an alias for `Reg<LTC0_LTC0_PKA_43_SPEC>`"]
pub type LTC0_LTC0_PKA_43 = crate::Reg<ltc0_ltc0_pka_43::LTC0_LTC0_PKA_43_SPEC>;
#[doc = "LTC PKHA A 43 Register"]
pub mod ltc0_ltc0_pka_43;
#[doc = "LTC0_LTC0_PKA2_12 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_12_SPEC>`"]
pub type LTC0_LTC0_PKA2_12 = crate::Reg<ltc0_ltc0_pka2_12::LTC0_LTC0_PKA2_12_SPEC>;
#[doc = "LTC PKHA A2 12 Register"]
pub mod ltc0_ltc0_pka2_12;
#[doc = "LTC0_LTC0_PKA_44 register accessor: an alias for `Reg<LTC0_LTC0_PKA_44_SPEC>`"]
pub type LTC0_LTC0_PKA_44 = crate::Reg<ltc0_ltc0_pka_44::LTC0_LTC0_PKA_44_SPEC>;
#[doc = "LTC PKHA A 44 Register"]
pub mod ltc0_ltc0_pka_44;
#[doc = "LTC0_LTC0_PKA2_13 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_13_SPEC>`"]
pub type LTC0_LTC0_PKA2_13 = crate::Reg<ltc0_ltc0_pka2_13::LTC0_LTC0_PKA2_13_SPEC>;
#[doc = "LTC PKHA A2 13 Register"]
pub mod ltc0_ltc0_pka2_13;
#[doc = "LTC0_LTC0_PKA_45 register accessor: an alias for `Reg<LTC0_LTC0_PKA_45_SPEC>`"]
pub type LTC0_LTC0_PKA_45 = crate::Reg<ltc0_ltc0_pka_45::LTC0_LTC0_PKA_45_SPEC>;
#[doc = "LTC PKHA A 45 Register"]
pub mod ltc0_ltc0_pka_45;
#[doc = "LTC0_LTC0_PKA2_14 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_14_SPEC>`"]
pub type LTC0_LTC0_PKA2_14 = crate::Reg<ltc0_ltc0_pka2_14::LTC0_LTC0_PKA2_14_SPEC>;
#[doc = "LTC PKHA A2 14 Register"]
pub mod ltc0_ltc0_pka2_14;
#[doc = "LTC0_LTC0_PKA_46 register accessor: an alias for `Reg<LTC0_LTC0_PKA_46_SPEC>`"]
pub type LTC0_LTC0_PKA_46 = crate::Reg<ltc0_ltc0_pka_46::LTC0_LTC0_PKA_46_SPEC>;
#[doc = "LTC PKHA A 46 Register"]
pub mod ltc0_ltc0_pka_46;
#[doc = "LTC0_LTC0_PKA2_15 register accessor: an alias for `Reg<LTC0_LTC0_PKA2_15_SPEC>`"]
pub type LTC0_LTC0_PKA2_15 = crate::Reg<ltc0_ltc0_pka2_15::LTC0_LTC0_PKA2_15_SPEC>;
#[doc = "LTC PKHA A2 15 Register"]
pub mod ltc0_ltc0_pka2_15;
#[doc = "LTC0_LTC0_PKA_47 register accessor: an alias for `Reg<LTC0_LTC0_PKA_47_SPEC>`"]
pub type LTC0_LTC0_PKA_47 = crate::Reg<ltc0_ltc0_pka_47::LTC0_LTC0_PKA_47_SPEC>;
#[doc = "LTC PKHA A 47 Register"]
pub mod ltc0_ltc0_pka_47;
#[doc = "LTC0_LTC0_PKA3_0 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_0_SPEC>`"]
pub type LTC0_LTC0_PKA3_0 = crate::Reg<ltc0_ltc0_pka3_0::LTC0_LTC0_PKA3_0_SPEC>;
#[doc = "LTC PKHA A3 0 Register"]
pub mod ltc0_ltc0_pka3_0;
#[doc = "LTC0_LTC0_PKA_48 register accessor: an alias for `Reg<LTC0_LTC0_PKA_48_SPEC>`"]
pub type LTC0_LTC0_PKA_48 = crate::Reg<ltc0_ltc0_pka_48::LTC0_LTC0_PKA_48_SPEC>;
#[doc = "LTC PKHA A 48 Register"]
pub mod ltc0_ltc0_pka_48;
#[doc = "LTC0_LTC0_PKA3_1 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_1_SPEC>`"]
pub type LTC0_LTC0_PKA3_1 = crate::Reg<ltc0_ltc0_pka3_1::LTC0_LTC0_PKA3_1_SPEC>;
#[doc = "LTC PKHA A3 1 Register"]
pub mod ltc0_ltc0_pka3_1;
#[doc = "LTC0_LTC0_PKA_49 register accessor: an alias for `Reg<LTC0_LTC0_PKA_49_SPEC>`"]
pub type LTC0_LTC0_PKA_49 = crate::Reg<ltc0_ltc0_pka_49::LTC0_LTC0_PKA_49_SPEC>;
#[doc = "LTC PKHA A 49 Register"]
pub mod ltc0_ltc0_pka_49;
#[doc = "LTC0_LTC0_PKA3_2 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_2_SPEC>`"]
pub type LTC0_LTC0_PKA3_2 = crate::Reg<ltc0_ltc0_pka3_2::LTC0_LTC0_PKA3_2_SPEC>;
#[doc = "LTC PKHA A3 2 Register"]
pub mod ltc0_ltc0_pka3_2;
#[doc = "LTC0_LTC0_PKA_50 register accessor: an alias for `Reg<LTC0_LTC0_PKA_50_SPEC>`"]
pub type LTC0_LTC0_PKA_50 = crate::Reg<ltc0_ltc0_pka_50::LTC0_LTC0_PKA_50_SPEC>;
#[doc = "LTC PKHA A 50 Register"]
pub mod ltc0_ltc0_pka_50;
#[doc = "LTC0_LTC0_PKA3_3 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_3_SPEC>`"]
pub type LTC0_LTC0_PKA3_3 = crate::Reg<ltc0_ltc0_pka3_3::LTC0_LTC0_PKA3_3_SPEC>;
#[doc = "LTC PKHA A3 3 Register"]
pub mod ltc0_ltc0_pka3_3;
#[doc = "LTC0_LTC0_PKA_51 register accessor: an alias for `Reg<LTC0_LTC0_PKA_51_SPEC>`"]
pub type LTC0_LTC0_PKA_51 = crate::Reg<ltc0_ltc0_pka_51::LTC0_LTC0_PKA_51_SPEC>;
#[doc = "LTC PKHA A 51 Register"]
pub mod ltc0_ltc0_pka_51;
#[doc = "LTC0_LTC0_PKA3_4 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_4_SPEC>`"]
pub type LTC0_LTC0_PKA3_4 = crate::Reg<ltc0_ltc0_pka3_4::LTC0_LTC0_PKA3_4_SPEC>;
#[doc = "LTC PKHA A3 4 Register"]
pub mod ltc0_ltc0_pka3_4;
#[doc = "LTC0_LTC0_PKA_52 register accessor: an alias for `Reg<LTC0_LTC0_PKA_52_SPEC>`"]
pub type LTC0_LTC0_PKA_52 = crate::Reg<ltc0_ltc0_pka_52::LTC0_LTC0_PKA_52_SPEC>;
#[doc = "LTC PKHA A 52 Register"]
pub mod ltc0_ltc0_pka_52;
#[doc = "LTC0_LTC0_PKA3_5 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_5_SPEC>`"]
pub type LTC0_LTC0_PKA3_5 = crate::Reg<ltc0_ltc0_pka3_5::LTC0_LTC0_PKA3_5_SPEC>;
#[doc = "LTC PKHA A3 5 Register"]
pub mod ltc0_ltc0_pka3_5;
#[doc = "LTC0_LTC0_PKA_53 register accessor: an alias for `Reg<LTC0_LTC0_PKA_53_SPEC>`"]
pub type LTC0_LTC0_PKA_53 = crate::Reg<ltc0_ltc0_pka_53::LTC0_LTC0_PKA_53_SPEC>;
#[doc = "LTC PKHA A 53 Register"]
pub mod ltc0_ltc0_pka_53;
#[doc = "LTC0_LTC0_PKA3_6 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_6_SPEC>`"]
pub type LTC0_LTC0_PKA3_6 = crate::Reg<ltc0_ltc0_pka3_6::LTC0_LTC0_PKA3_6_SPEC>;
#[doc = "LTC PKHA A3 6 Register"]
pub mod ltc0_ltc0_pka3_6;
#[doc = "LTC0_LTC0_PKA_54 register accessor: an alias for `Reg<LTC0_LTC0_PKA_54_SPEC>`"]
pub type LTC0_LTC0_PKA_54 = crate::Reg<ltc0_ltc0_pka_54::LTC0_LTC0_PKA_54_SPEC>;
#[doc = "LTC PKHA A 54 Register"]
pub mod ltc0_ltc0_pka_54;
#[doc = "LTC0_LTC0_PKA3_7 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_7_SPEC>`"]
pub type LTC0_LTC0_PKA3_7 = crate::Reg<ltc0_ltc0_pka3_7::LTC0_LTC0_PKA3_7_SPEC>;
#[doc = "LTC PKHA A3 7 Register"]
pub mod ltc0_ltc0_pka3_7;
#[doc = "LTC0_LTC0_PKA_55 register accessor: an alias for `Reg<LTC0_LTC0_PKA_55_SPEC>`"]
pub type LTC0_LTC0_PKA_55 = crate::Reg<ltc0_ltc0_pka_55::LTC0_LTC0_PKA_55_SPEC>;
#[doc = "LTC PKHA A 55 Register"]
pub mod ltc0_ltc0_pka_55;
#[doc = "LTC0_LTC0_PKA3_8 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_8_SPEC>`"]
pub type LTC0_LTC0_PKA3_8 = crate::Reg<ltc0_ltc0_pka3_8::LTC0_LTC0_PKA3_8_SPEC>;
#[doc = "LTC PKHA A3 8 Register"]
pub mod ltc0_ltc0_pka3_8;
#[doc = "LTC0_LTC0_PKA_56 register accessor: an alias for `Reg<LTC0_LTC0_PKA_56_SPEC>`"]
pub type LTC0_LTC0_PKA_56 = crate::Reg<ltc0_ltc0_pka_56::LTC0_LTC0_PKA_56_SPEC>;
#[doc = "LTC PKHA A 56 Register"]
pub mod ltc0_ltc0_pka_56;
#[doc = "LTC0_LTC0_PKA3_9 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_9_SPEC>`"]
pub type LTC0_LTC0_PKA3_9 = crate::Reg<ltc0_ltc0_pka3_9::LTC0_LTC0_PKA3_9_SPEC>;
#[doc = "LTC PKHA A3 9 Register"]
pub mod ltc0_ltc0_pka3_9;
#[doc = "LTC0_LTC0_PKA_57 register accessor: an alias for `Reg<LTC0_LTC0_PKA_57_SPEC>`"]
pub type LTC0_LTC0_PKA_57 = crate::Reg<ltc0_ltc0_pka_57::LTC0_LTC0_PKA_57_SPEC>;
#[doc = "LTC PKHA A 57 Register"]
pub mod ltc0_ltc0_pka_57;
#[doc = "LTC0_LTC0_PKA3_10 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_10_SPEC>`"]
pub type LTC0_LTC0_PKA3_10 = crate::Reg<ltc0_ltc0_pka3_10::LTC0_LTC0_PKA3_10_SPEC>;
#[doc = "LTC PKHA A3 10 Register"]
pub mod ltc0_ltc0_pka3_10;
#[doc = "LTC0_LTC0_PKA_58 register accessor: an alias for `Reg<LTC0_LTC0_PKA_58_SPEC>`"]
pub type LTC0_LTC0_PKA_58 = crate::Reg<ltc0_ltc0_pka_58::LTC0_LTC0_PKA_58_SPEC>;
#[doc = "LTC PKHA A 58 Register"]
pub mod ltc0_ltc0_pka_58;
#[doc = "LTC0_LTC0_PKA3_11 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_11_SPEC>`"]
pub type LTC0_LTC0_PKA3_11 = crate::Reg<ltc0_ltc0_pka3_11::LTC0_LTC0_PKA3_11_SPEC>;
#[doc = "LTC PKHA A3 11 Register"]
pub mod ltc0_ltc0_pka3_11;
#[doc = "LTC0_LTC0_PKA_59 register accessor: an alias for `Reg<LTC0_LTC0_PKA_59_SPEC>`"]
pub type LTC0_LTC0_PKA_59 = crate::Reg<ltc0_ltc0_pka_59::LTC0_LTC0_PKA_59_SPEC>;
#[doc = "LTC PKHA A 59 Register"]
pub mod ltc0_ltc0_pka_59;
#[doc = "LTC0_LTC0_PKA3_12 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_12_SPEC>`"]
pub type LTC0_LTC0_PKA3_12 = crate::Reg<ltc0_ltc0_pka3_12::LTC0_LTC0_PKA3_12_SPEC>;
#[doc = "LTC PKHA A3 12 Register"]
pub mod ltc0_ltc0_pka3_12;
#[doc = "LTC0_LTC0_PKA_60 register accessor: an alias for `Reg<LTC0_LTC0_PKA_60_SPEC>`"]
pub type LTC0_LTC0_PKA_60 = crate::Reg<ltc0_ltc0_pka_60::LTC0_LTC0_PKA_60_SPEC>;
#[doc = "LTC PKHA A 60 Register"]
pub mod ltc0_ltc0_pka_60;
#[doc = "LTC0_LTC0_PKA3_13 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_13_SPEC>`"]
pub type LTC0_LTC0_PKA3_13 = crate::Reg<ltc0_ltc0_pka3_13::LTC0_LTC0_PKA3_13_SPEC>;
#[doc = "LTC PKHA A3 13 Register"]
pub mod ltc0_ltc0_pka3_13;
#[doc = "LTC0_LTC0_PKA_61 register accessor: an alias for `Reg<LTC0_LTC0_PKA_61_SPEC>`"]
pub type LTC0_LTC0_PKA_61 = crate::Reg<ltc0_ltc0_pka_61::LTC0_LTC0_PKA_61_SPEC>;
#[doc = "LTC PKHA A 61 Register"]
pub mod ltc0_ltc0_pka_61;
#[doc = "LTC0_LTC0_PKA3_14 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_14_SPEC>`"]
pub type LTC0_LTC0_PKA3_14 = crate::Reg<ltc0_ltc0_pka3_14::LTC0_LTC0_PKA3_14_SPEC>;
#[doc = "LTC PKHA A3 14 Register"]
pub mod ltc0_ltc0_pka3_14;
#[doc = "LTC0_LTC0_PKA_62 register accessor: an alias for `Reg<LTC0_LTC0_PKA_62_SPEC>`"]
pub type LTC0_LTC0_PKA_62 = crate::Reg<ltc0_ltc0_pka_62::LTC0_LTC0_PKA_62_SPEC>;
#[doc = "LTC PKHA A 62 Register"]
pub mod ltc0_ltc0_pka_62;
#[doc = "LTC0_LTC0_PKA3_15 register accessor: an alias for `Reg<LTC0_LTC0_PKA3_15_SPEC>`"]
pub type LTC0_LTC0_PKA3_15 = crate::Reg<ltc0_ltc0_pka3_15::LTC0_LTC0_PKA3_15_SPEC>;
#[doc = "LTC PKHA A3 15 Register"]
pub mod ltc0_ltc0_pka3_15;
#[doc = "LTC0_LTC0_PKA_63 register accessor: an alias for `Reg<LTC0_LTC0_PKA_63_SPEC>`"]
pub type LTC0_LTC0_PKA_63 = crate::Reg<ltc0_ltc0_pka_63::LTC0_LTC0_PKA_63_SPEC>;
#[doc = "LTC PKHA A 63 Register"]
pub mod ltc0_ltc0_pka_63;
#[doc = "LTC0_LTC0_PKB0_0 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_0_SPEC>`"]
pub type LTC0_LTC0_PKB0_0 = crate::Reg<ltc0_ltc0_pkb0_0::LTC0_LTC0_PKB0_0_SPEC>;
#[doc = "LTC PKHA B0 0 Register"]
pub mod ltc0_ltc0_pkb0_0;
#[doc = "LTC0_LTC0_PKB_0 register accessor: an alias for `Reg<LTC0_LTC0_PKB_0_SPEC>`"]
pub type LTC0_LTC0_PKB_0 = crate::Reg<ltc0_ltc0_pkb_0::LTC0_LTC0_PKB_0_SPEC>;
#[doc = "LTC PKHA B 0 Register"]
pub mod ltc0_ltc0_pkb_0;
#[doc = "LTC0_LTC0_PKB0_1 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_1_SPEC>`"]
pub type LTC0_LTC0_PKB0_1 = crate::Reg<ltc0_ltc0_pkb0_1::LTC0_LTC0_PKB0_1_SPEC>;
#[doc = "LTC PKHA B0 1 Register"]
pub mod ltc0_ltc0_pkb0_1;
#[doc = "LTC0_LTC0_PKB_1 register accessor: an alias for `Reg<LTC0_LTC0_PKB_1_SPEC>`"]
pub type LTC0_LTC0_PKB_1 = crate::Reg<ltc0_ltc0_pkb_1::LTC0_LTC0_PKB_1_SPEC>;
#[doc = "LTC PKHA B 1 Register"]
pub mod ltc0_ltc0_pkb_1;
#[doc = "LTC0_LTC0_PKB0_2 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_2_SPEC>`"]
pub type LTC0_LTC0_PKB0_2 = crate::Reg<ltc0_ltc0_pkb0_2::LTC0_LTC0_PKB0_2_SPEC>;
#[doc = "LTC PKHA B0 2 Register"]
pub mod ltc0_ltc0_pkb0_2;
#[doc = "LTC0_LTC0_PKB_2 register accessor: an alias for `Reg<LTC0_LTC0_PKB_2_SPEC>`"]
pub type LTC0_LTC0_PKB_2 = crate::Reg<ltc0_ltc0_pkb_2::LTC0_LTC0_PKB_2_SPEC>;
#[doc = "LTC PKHA B 2 Register"]
pub mod ltc0_ltc0_pkb_2;
#[doc = "LTC0_LTC0_PKB0_3 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_3_SPEC>`"]
pub type LTC0_LTC0_PKB0_3 = crate::Reg<ltc0_ltc0_pkb0_3::LTC0_LTC0_PKB0_3_SPEC>;
#[doc = "LTC PKHA B0 3 Register"]
pub mod ltc0_ltc0_pkb0_3;
#[doc = "LTC0_LTC0_PKB_3 register accessor: an alias for `Reg<LTC0_LTC0_PKB_3_SPEC>`"]
pub type LTC0_LTC0_PKB_3 = crate::Reg<ltc0_ltc0_pkb_3::LTC0_LTC0_PKB_3_SPEC>;
#[doc = "LTC PKHA B 3 Register"]
pub mod ltc0_ltc0_pkb_3;
#[doc = "LTC0_LTC0_PKB0_4 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_4_SPEC>`"]
pub type LTC0_LTC0_PKB0_4 = crate::Reg<ltc0_ltc0_pkb0_4::LTC0_LTC0_PKB0_4_SPEC>;
#[doc = "LTC PKHA B0 4 Register"]
pub mod ltc0_ltc0_pkb0_4;
#[doc = "LTC0_LTC0_PKB_4 register accessor: an alias for `Reg<LTC0_LTC0_PKB_4_SPEC>`"]
pub type LTC0_LTC0_PKB_4 = crate::Reg<ltc0_ltc0_pkb_4::LTC0_LTC0_PKB_4_SPEC>;
#[doc = "LTC PKHA B 4 Register"]
pub mod ltc0_ltc0_pkb_4;
#[doc = "LTC0_LTC0_PKB0_5 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_5_SPEC>`"]
pub type LTC0_LTC0_PKB0_5 = crate::Reg<ltc0_ltc0_pkb0_5::LTC0_LTC0_PKB0_5_SPEC>;
#[doc = "LTC PKHA B0 5 Register"]
pub mod ltc0_ltc0_pkb0_5;
#[doc = "LTC0_LTC0_PKB_5 register accessor: an alias for `Reg<LTC0_LTC0_PKB_5_SPEC>`"]
pub type LTC0_LTC0_PKB_5 = crate::Reg<ltc0_ltc0_pkb_5::LTC0_LTC0_PKB_5_SPEC>;
#[doc = "LTC PKHA B 5 Register"]
pub mod ltc0_ltc0_pkb_5;
#[doc = "LTC0_LTC0_PKB0_6 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_6_SPEC>`"]
pub type LTC0_LTC0_PKB0_6 = crate::Reg<ltc0_ltc0_pkb0_6::LTC0_LTC0_PKB0_6_SPEC>;
#[doc = "LTC PKHA B0 6 Register"]
pub mod ltc0_ltc0_pkb0_6;
#[doc = "LTC0_LTC0_PKB_6 register accessor: an alias for `Reg<LTC0_LTC0_PKB_6_SPEC>`"]
pub type LTC0_LTC0_PKB_6 = crate::Reg<ltc0_ltc0_pkb_6::LTC0_LTC0_PKB_6_SPEC>;
#[doc = "LTC PKHA B 6 Register"]
pub mod ltc0_ltc0_pkb_6;
#[doc = "LTC0_LTC0_PKB0_7 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_7_SPEC>`"]
pub type LTC0_LTC0_PKB0_7 = crate::Reg<ltc0_ltc0_pkb0_7::LTC0_LTC0_PKB0_7_SPEC>;
#[doc = "LTC PKHA B0 7 Register"]
pub mod ltc0_ltc0_pkb0_7;
#[doc = "LTC0_LTC0_PKB_7 register accessor: an alias for `Reg<LTC0_LTC0_PKB_7_SPEC>`"]
pub type LTC0_LTC0_PKB_7 = crate::Reg<ltc0_ltc0_pkb_7::LTC0_LTC0_PKB_7_SPEC>;
#[doc = "LTC PKHA B 7 Register"]
pub mod ltc0_ltc0_pkb_7;
#[doc = "LTC0_LTC0_PKB0_8 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_8_SPEC>`"]
pub type LTC0_LTC0_PKB0_8 = crate::Reg<ltc0_ltc0_pkb0_8::LTC0_LTC0_PKB0_8_SPEC>;
#[doc = "LTC PKHA B0 8 Register"]
pub mod ltc0_ltc0_pkb0_8;
#[doc = "LTC0_LTC0_PKB_8 register accessor: an alias for `Reg<LTC0_LTC0_PKB_8_SPEC>`"]
pub type LTC0_LTC0_PKB_8 = crate::Reg<ltc0_ltc0_pkb_8::LTC0_LTC0_PKB_8_SPEC>;
#[doc = "LTC PKHA B 8 Register"]
pub mod ltc0_ltc0_pkb_8;
#[doc = "LTC0_LTC0_PKB0_9 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_9_SPEC>`"]
pub type LTC0_LTC0_PKB0_9 = crate::Reg<ltc0_ltc0_pkb0_9::LTC0_LTC0_PKB0_9_SPEC>;
#[doc = "LTC PKHA B0 9 Register"]
pub mod ltc0_ltc0_pkb0_9;
#[doc = "LTC0_LTC0_PKB_9 register accessor: an alias for `Reg<LTC0_LTC0_PKB_9_SPEC>`"]
pub type LTC0_LTC0_PKB_9 = crate::Reg<ltc0_ltc0_pkb_9::LTC0_LTC0_PKB_9_SPEC>;
#[doc = "LTC PKHA B 9 Register"]
pub mod ltc0_ltc0_pkb_9;
#[doc = "LTC0_LTC0_PKB0_10 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_10_SPEC>`"]
pub type LTC0_LTC0_PKB0_10 = crate::Reg<ltc0_ltc0_pkb0_10::LTC0_LTC0_PKB0_10_SPEC>;
#[doc = "LTC PKHA B0 10 Register"]
pub mod ltc0_ltc0_pkb0_10;
#[doc = "LTC0_LTC0_PKB_10 register accessor: an alias for `Reg<LTC0_LTC0_PKB_10_SPEC>`"]
pub type LTC0_LTC0_PKB_10 = crate::Reg<ltc0_ltc0_pkb_10::LTC0_LTC0_PKB_10_SPEC>;
#[doc = "LTC PKHA B 10 Register"]
pub mod ltc0_ltc0_pkb_10;
#[doc = "LTC0_LTC0_PKB0_11 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_11_SPEC>`"]
pub type LTC0_LTC0_PKB0_11 = crate::Reg<ltc0_ltc0_pkb0_11::LTC0_LTC0_PKB0_11_SPEC>;
#[doc = "LTC PKHA B0 11 Register"]
pub mod ltc0_ltc0_pkb0_11;
#[doc = "LTC0_LTC0_PKB_11 register accessor: an alias for `Reg<LTC0_LTC0_PKB_11_SPEC>`"]
pub type LTC0_LTC0_PKB_11 = crate::Reg<ltc0_ltc0_pkb_11::LTC0_LTC0_PKB_11_SPEC>;
#[doc = "LTC PKHA B 11 Register"]
pub mod ltc0_ltc0_pkb_11;
#[doc = "LTC0_LTC0_PKB0_12 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_12_SPEC>`"]
pub type LTC0_LTC0_PKB0_12 = crate::Reg<ltc0_ltc0_pkb0_12::LTC0_LTC0_PKB0_12_SPEC>;
#[doc = "LTC PKHA B0 12 Register"]
pub mod ltc0_ltc0_pkb0_12;
#[doc = "LTC0_LTC0_PKB_12 register accessor: an alias for `Reg<LTC0_LTC0_PKB_12_SPEC>`"]
pub type LTC0_LTC0_PKB_12 = crate::Reg<ltc0_ltc0_pkb_12::LTC0_LTC0_PKB_12_SPEC>;
#[doc = "LTC PKHA B 12 Register"]
pub mod ltc0_ltc0_pkb_12;
#[doc = "LTC0_LTC0_PKB0_13 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_13_SPEC>`"]
pub type LTC0_LTC0_PKB0_13 = crate::Reg<ltc0_ltc0_pkb0_13::LTC0_LTC0_PKB0_13_SPEC>;
#[doc = "LTC PKHA B0 13 Register"]
pub mod ltc0_ltc0_pkb0_13;
#[doc = "LTC0_LTC0_PKB_13 register accessor: an alias for `Reg<LTC0_LTC0_PKB_13_SPEC>`"]
pub type LTC0_LTC0_PKB_13 = crate::Reg<ltc0_ltc0_pkb_13::LTC0_LTC0_PKB_13_SPEC>;
#[doc = "LTC PKHA B 13 Register"]
pub mod ltc0_ltc0_pkb_13;
#[doc = "LTC0_LTC0_PKB0_14 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_14_SPEC>`"]
pub type LTC0_LTC0_PKB0_14 = crate::Reg<ltc0_ltc0_pkb0_14::LTC0_LTC0_PKB0_14_SPEC>;
#[doc = "LTC PKHA B0 14 Register"]
pub mod ltc0_ltc0_pkb0_14;
#[doc = "LTC0_LTC0_PKB_14 register accessor: an alias for `Reg<LTC0_LTC0_PKB_14_SPEC>`"]
pub type LTC0_LTC0_PKB_14 = crate::Reg<ltc0_ltc0_pkb_14::LTC0_LTC0_PKB_14_SPEC>;
#[doc = "LTC PKHA B 14 Register"]
pub mod ltc0_ltc0_pkb_14;
#[doc = "LTC0_LTC0_PKB0_15 register accessor: an alias for `Reg<LTC0_LTC0_PKB0_15_SPEC>`"]
pub type LTC0_LTC0_PKB0_15 = crate::Reg<ltc0_ltc0_pkb0_15::LTC0_LTC0_PKB0_15_SPEC>;
#[doc = "LTC PKHA B0 15 Register"]
pub mod ltc0_ltc0_pkb0_15;
#[doc = "LTC0_LTC0_PKB_15 register accessor: an alias for `Reg<LTC0_LTC0_PKB_15_SPEC>`"]
pub type LTC0_LTC0_PKB_15 = crate::Reg<ltc0_ltc0_pkb_15::LTC0_LTC0_PKB_15_SPEC>;
#[doc = "LTC PKHA B 15 Register"]
pub mod ltc0_ltc0_pkb_15;
#[doc = "LTC0_LTC0_PKB1_0 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_0_SPEC>`"]
pub type LTC0_LTC0_PKB1_0 = crate::Reg<ltc0_ltc0_pkb1_0::LTC0_LTC0_PKB1_0_SPEC>;
#[doc = "LTC PKHA B1 0 Register"]
pub mod ltc0_ltc0_pkb1_0;
#[doc = "LTC0_LTC0_PKB_16 register accessor: an alias for `Reg<LTC0_LTC0_PKB_16_SPEC>`"]
pub type LTC0_LTC0_PKB_16 = crate::Reg<ltc0_ltc0_pkb_16::LTC0_LTC0_PKB_16_SPEC>;
#[doc = "LTC PKHA B 16 Register"]
pub mod ltc0_ltc0_pkb_16;
#[doc = "LTC0_LTC0_PKB1_1 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_1_SPEC>`"]
pub type LTC0_LTC0_PKB1_1 = crate::Reg<ltc0_ltc0_pkb1_1::LTC0_LTC0_PKB1_1_SPEC>;
#[doc = "LTC PKHA B1 1 Register"]
pub mod ltc0_ltc0_pkb1_1;
#[doc = "LTC0_LTC0_PKB_17 register accessor: an alias for `Reg<LTC0_LTC0_PKB_17_SPEC>`"]
pub type LTC0_LTC0_PKB_17 = crate::Reg<ltc0_ltc0_pkb_17::LTC0_LTC0_PKB_17_SPEC>;
#[doc = "LTC PKHA B 17 Register"]
pub mod ltc0_ltc0_pkb_17;
#[doc = "LTC0_LTC0_PKB1_2 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_2_SPEC>`"]
pub type LTC0_LTC0_PKB1_2 = crate::Reg<ltc0_ltc0_pkb1_2::LTC0_LTC0_PKB1_2_SPEC>;
#[doc = "LTC PKHA B1 2 Register"]
pub mod ltc0_ltc0_pkb1_2;
#[doc = "LTC0_LTC0_PKB_18 register accessor: an alias for `Reg<LTC0_LTC0_PKB_18_SPEC>`"]
pub type LTC0_LTC0_PKB_18 = crate::Reg<ltc0_ltc0_pkb_18::LTC0_LTC0_PKB_18_SPEC>;
#[doc = "LTC PKHA B 18 Register"]
pub mod ltc0_ltc0_pkb_18;
#[doc = "LTC0_LTC0_PKB1_3 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_3_SPEC>`"]
pub type LTC0_LTC0_PKB1_3 = crate::Reg<ltc0_ltc0_pkb1_3::LTC0_LTC0_PKB1_3_SPEC>;
#[doc = "LTC PKHA B1 3 Register"]
pub mod ltc0_ltc0_pkb1_3;
#[doc = "LTC0_LTC0_PKB_19 register accessor: an alias for `Reg<LTC0_LTC0_PKB_19_SPEC>`"]
pub type LTC0_LTC0_PKB_19 = crate::Reg<ltc0_ltc0_pkb_19::LTC0_LTC0_PKB_19_SPEC>;
#[doc = "LTC PKHA B 19 Register"]
pub mod ltc0_ltc0_pkb_19;
#[doc = "LTC0_LTC0_PKB1_4 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_4_SPEC>`"]
pub type LTC0_LTC0_PKB1_4 = crate::Reg<ltc0_ltc0_pkb1_4::LTC0_LTC0_PKB1_4_SPEC>;
#[doc = "LTC PKHA B1 4 Register"]
pub mod ltc0_ltc0_pkb1_4;
#[doc = "LTC0_LTC0_PKB_20 register accessor: an alias for `Reg<LTC0_LTC0_PKB_20_SPEC>`"]
pub type LTC0_LTC0_PKB_20 = crate::Reg<ltc0_ltc0_pkb_20::LTC0_LTC0_PKB_20_SPEC>;
#[doc = "LTC PKHA B 20 Register"]
pub mod ltc0_ltc0_pkb_20;
#[doc = "LTC0_LTC0_PKB1_5 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_5_SPEC>`"]
pub type LTC0_LTC0_PKB1_5 = crate::Reg<ltc0_ltc0_pkb1_5::LTC0_LTC0_PKB1_5_SPEC>;
#[doc = "LTC PKHA B1 5 Register"]
pub mod ltc0_ltc0_pkb1_5;
#[doc = "LTC0_LTC0_PKB_21 register accessor: an alias for `Reg<LTC0_LTC0_PKB_21_SPEC>`"]
pub type LTC0_LTC0_PKB_21 = crate::Reg<ltc0_ltc0_pkb_21::LTC0_LTC0_PKB_21_SPEC>;
#[doc = "LTC PKHA B 21 Register"]
pub mod ltc0_ltc0_pkb_21;
#[doc = "LTC0_LTC0_PKB1_6 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_6_SPEC>`"]
pub type LTC0_LTC0_PKB1_6 = crate::Reg<ltc0_ltc0_pkb1_6::LTC0_LTC0_PKB1_6_SPEC>;
#[doc = "LTC PKHA B1 6 Register"]
pub mod ltc0_ltc0_pkb1_6;
#[doc = "LTC0_LTC0_PKB_22 register accessor: an alias for `Reg<LTC0_LTC0_PKB_22_SPEC>`"]
pub type LTC0_LTC0_PKB_22 = crate::Reg<ltc0_ltc0_pkb_22::LTC0_LTC0_PKB_22_SPEC>;
#[doc = "LTC PKHA B 22 Register"]
pub mod ltc0_ltc0_pkb_22;
#[doc = "LTC0_LTC0_PKB1_7 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_7_SPEC>`"]
pub type LTC0_LTC0_PKB1_7 = crate::Reg<ltc0_ltc0_pkb1_7::LTC0_LTC0_PKB1_7_SPEC>;
#[doc = "LTC PKHA B1 7 Register"]
pub mod ltc0_ltc0_pkb1_7;
#[doc = "LTC0_LTC0_PKB_23 register accessor: an alias for `Reg<LTC0_LTC0_PKB_23_SPEC>`"]
pub type LTC0_LTC0_PKB_23 = crate::Reg<ltc0_ltc0_pkb_23::LTC0_LTC0_PKB_23_SPEC>;
#[doc = "LTC PKHA B 23 Register"]
pub mod ltc0_ltc0_pkb_23;
#[doc = "LTC0_LTC0_PKB1_8 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_8_SPEC>`"]
pub type LTC0_LTC0_PKB1_8 = crate::Reg<ltc0_ltc0_pkb1_8::LTC0_LTC0_PKB1_8_SPEC>;
#[doc = "LTC PKHA B1 8 Register"]
pub mod ltc0_ltc0_pkb1_8;
#[doc = "LTC0_LTC0_PKB_24 register accessor: an alias for `Reg<LTC0_LTC0_PKB_24_SPEC>`"]
pub type LTC0_LTC0_PKB_24 = crate::Reg<ltc0_ltc0_pkb_24::LTC0_LTC0_PKB_24_SPEC>;
#[doc = "LTC PKHA B 24 Register"]
pub mod ltc0_ltc0_pkb_24;
#[doc = "LTC0_LTC0_PKB1_9 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_9_SPEC>`"]
pub type LTC0_LTC0_PKB1_9 = crate::Reg<ltc0_ltc0_pkb1_9::LTC0_LTC0_PKB1_9_SPEC>;
#[doc = "LTC PKHA B1 9 Register"]
pub mod ltc0_ltc0_pkb1_9;
#[doc = "LTC0_LTC0_PKB_25 register accessor: an alias for `Reg<LTC0_LTC0_PKB_25_SPEC>`"]
pub type LTC0_LTC0_PKB_25 = crate::Reg<ltc0_ltc0_pkb_25::LTC0_LTC0_PKB_25_SPEC>;
#[doc = "LTC PKHA B 25 Register"]
pub mod ltc0_ltc0_pkb_25;
#[doc = "LTC0_LTC0_PKB1_10 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_10_SPEC>`"]
pub type LTC0_LTC0_PKB1_10 = crate::Reg<ltc0_ltc0_pkb1_10::LTC0_LTC0_PKB1_10_SPEC>;
#[doc = "LTC PKHA B1 10 Register"]
pub mod ltc0_ltc0_pkb1_10;
#[doc = "LTC0_LTC0_PKB_26 register accessor: an alias for `Reg<LTC0_LTC0_PKB_26_SPEC>`"]
pub type LTC0_LTC0_PKB_26 = crate::Reg<ltc0_ltc0_pkb_26::LTC0_LTC0_PKB_26_SPEC>;
#[doc = "LTC PKHA B 26 Register"]
pub mod ltc0_ltc0_pkb_26;
#[doc = "LTC0_LTC0_PKB1_11 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_11_SPEC>`"]
pub type LTC0_LTC0_PKB1_11 = crate::Reg<ltc0_ltc0_pkb1_11::LTC0_LTC0_PKB1_11_SPEC>;
#[doc = "LTC PKHA B1 11 Register"]
pub mod ltc0_ltc0_pkb1_11;
#[doc = "LTC0_LTC0_PKB_27 register accessor: an alias for `Reg<LTC0_LTC0_PKB_27_SPEC>`"]
pub type LTC0_LTC0_PKB_27 = crate::Reg<ltc0_ltc0_pkb_27::LTC0_LTC0_PKB_27_SPEC>;
#[doc = "LTC PKHA B 27 Register"]
pub mod ltc0_ltc0_pkb_27;
#[doc = "LTC0_LTC0_PKB1_12 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_12_SPEC>`"]
pub type LTC0_LTC0_PKB1_12 = crate::Reg<ltc0_ltc0_pkb1_12::LTC0_LTC0_PKB1_12_SPEC>;
#[doc = "LTC PKHA B1 12 Register"]
pub mod ltc0_ltc0_pkb1_12;
#[doc = "LTC0_LTC0_PKB_28 register accessor: an alias for `Reg<LTC0_LTC0_PKB_28_SPEC>`"]
pub type LTC0_LTC0_PKB_28 = crate::Reg<ltc0_ltc0_pkb_28::LTC0_LTC0_PKB_28_SPEC>;
#[doc = "LTC PKHA B 28 Register"]
pub mod ltc0_ltc0_pkb_28;
#[doc = "LTC0_LTC0_PKB1_13 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_13_SPEC>`"]
pub type LTC0_LTC0_PKB1_13 = crate::Reg<ltc0_ltc0_pkb1_13::LTC0_LTC0_PKB1_13_SPEC>;
#[doc = "LTC PKHA B1 13 Register"]
pub mod ltc0_ltc0_pkb1_13;
#[doc = "LTC0_LTC0_PKB_29 register accessor: an alias for `Reg<LTC0_LTC0_PKB_29_SPEC>`"]
pub type LTC0_LTC0_PKB_29 = crate::Reg<ltc0_ltc0_pkb_29::LTC0_LTC0_PKB_29_SPEC>;
#[doc = "LTC PKHA B 29 Register"]
pub mod ltc0_ltc0_pkb_29;
#[doc = "LTC0_LTC0_PKB1_14 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_14_SPEC>`"]
pub type LTC0_LTC0_PKB1_14 = crate::Reg<ltc0_ltc0_pkb1_14::LTC0_LTC0_PKB1_14_SPEC>;
#[doc = "LTC PKHA B1 14 Register"]
pub mod ltc0_ltc0_pkb1_14;
#[doc = "LTC0_LTC0_PKB_30 register accessor: an alias for `Reg<LTC0_LTC0_PKB_30_SPEC>`"]
pub type LTC0_LTC0_PKB_30 = crate::Reg<ltc0_ltc0_pkb_30::LTC0_LTC0_PKB_30_SPEC>;
#[doc = "LTC PKHA B 30 Register"]
pub mod ltc0_ltc0_pkb_30;
#[doc = "LTC0_LTC0_PKB1_15 register accessor: an alias for `Reg<LTC0_LTC0_PKB1_15_SPEC>`"]
pub type LTC0_LTC0_PKB1_15 = crate::Reg<ltc0_ltc0_pkb1_15::LTC0_LTC0_PKB1_15_SPEC>;
#[doc = "LTC PKHA B1 15 Register"]
pub mod ltc0_ltc0_pkb1_15;
#[doc = "LTC0_LTC0_PKB_31 register accessor: an alias for `Reg<LTC0_LTC0_PKB_31_SPEC>`"]
pub type LTC0_LTC0_PKB_31 = crate::Reg<ltc0_ltc0_pkb_31::LTC0_LTC0_PKB_31_SPEC>;
#[doc = "LTC PKHA B 31 Register"]
pub mod ltc0_ltc0_pkb_31;
#[doc = "LTC0_LTC0_PKB2_0 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_0_SPEC>`"]
pub type LTC0_LTC0_PKB2_0 = crate::Reg<ltc0_ltc0_pkb2_0::LTC0_LTC0_PKB2_0_SPEC>;
#[doc = "LTC PKHA B2 0 Register"]
pub mod ltc0_ltc0_pkb2_0;
#[doc = "LTC0_LTC0_PKB_32 register accessor: an alias for `Reg<LTC0_LTC0_PKB_32_SPEC>`"]
pub type LTC0_LTC0_PKB_32 = crate::Reg<ltc0_ltc0_pkb_32::LTC0_LTC0_PKB_32_SPEC>;
#[doc = "LTC PKHA B 32 Register"]
pub mod ltc0_ltc0_pkb_32;
#[doc = "LTC0_LTC0_PKB2_1 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_1_SPEC>`"]
pub type LTC0_LTC0_PKB2_1 = crate::Reg<ltc0_ltc0_pkb2_1::LTC0_LTC0_PKB2_1_SPEC>;
#[doc = "LTC PKHA B2 1 Register"]
pub mod ltc0_ltc0_pkb2_1;
#[doc = "LTC0_LTC0_PKB_33 register accessor: an alias for `Reg<LTC0_LTC0_PKB_33_SPEC>`"]
pub type LTC0_LTC0_PKB_33 = crate::Reg<ltc0_ltc0_pkb_33::LTC0_LTC0_PKB_33_SPEC>;
#[doc = "LTC PKHA B 33 Register"]
pub mod ltc0_ltc0_pkb_33;
#[doc = "LTC0_LTC0_PKB2_2 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_2_SPEC>`"]
pub type LTC0_LTC0_PKB2_2 = crate::Reg<ltc0_ltc0_pkb2_2::LTC0_LTC0_PKB2_2_SPEC>;
#[doc = "LTC PKHA B2 2 Register"]
pub mod ltc0_ltc0_pkb2_2;
#[doc = "LTC0_LTC0_PKB_34 register accessor: an alias for `Reg<LTC0_LTC0_PKB_34_SPEC>`"]
pub type LTC0_LTC0_PKB_34 = crate::Reg<ltc0_ltc0_pkb_34::LTC0_LTC0_PKB_34_SPEC>;
#[doc = "LTC PKHA B 34 Register"]
pub mod ltc0_ltc0_pkb_34;
#[doc = "LTC0_LTC0_PKB2_3 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_3_SPEC>`"]
pub type LTC0_LTC0_PKB2_3 = crate::Reg<ltc0_ltc0_pkb2_3::LTC0_LTC0_PKB2_3_SPEC>;
#[doc = "LTC PKHA B2 3 Register"]
pub mod ltc0_ltc0_pkb2_3;
#[doc = "LTC0_LTC0_PKB_35 register accessor: an alias for `Reg<LTC0_LTC0_PKB_35_SPEC>`"]
pub type LTC0_LTC0_PKB_35 = crate::Reg<ltc0_ltc0_pkb_35::LTC0_LTC0_PKB_35_SPEC>;
#[doc = "LTC PKHA B 35 Register"]
pub mod ltc0_ltc0_pkb_35;
#[doc = "LTC0_LTC0_PKB2_4 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_4_SPEC>`"]
pub type LTC0_LTC0_PKB2_4 = crate::Reg<ltc0_ltc0_pkb2_4::LTC0_LTC0_PKB2_4_SPEC>;
#[doc = "LTC PKHA B2 4 Register"]
pub mod ltc0_ltc0_pkb2_4;
#[doc = "LTC0_LTC0_PKB_36 register accessor: an alias for `Reg<LTC0_LTC0_PKB_36_SPEC>`"]
pub type LTC0_LTC0_PKB_36 = crate::Reg<ltc0_ltc0_pkb_36::LTC0_LTC0_PKB_36_SPEC>;
#[doc = "LTC PKHA B 36 Register"]
pub mod ltc0_ltc0_pkb_36;
#[doc = "LTC0_LTC0_PKB2_5 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_5_SPEC>`"]
pub type LTC0_LTC0_PKB2_5 = crate::Reg<ltc0_ltc0_pkb2_5::LTC0_LTC0_PKB2_5_SPEC>;
#[doc = "LTC PKHA B2 5 Register"]
pub mod ltc0_ltc0_pkb2_5;
#[doc = "LTC0_LTC0_PKB_37 register accessor: an alias for `Reg<LTC0_LTC0_PKB_37_SPEC>`"]
pub type LTC0_LTC0_PKB_37 = crate::Reg<ltc0_ltc0_pkb_37::LTC0_LTC0_PKB_37_SPEC>;
#[doc = "LTC PKHA B 37 Register"]
pub mod ltc0_ltc0_pkb_37;
#[doc = "LTC0_LTC0_PKB2_6 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_6_SPEC>`"]
pub type LTC0_LTC0_PKB2_6 = crate::Reg<ltc0_ltc0_pkb2_6::LTC0_LTC0_PKB2_6_SPEC>;
#[doc = "LTC PKHA B2 6 Register"]
pub mod ltc0_ltc0_pkb2_6;
#[doc = "LTC0_LTC0_PKB_38 register accessor: an alias for `Reg<LTC0_LTC0_PKB_38_SPEC>`"]
pub type LTC0_LTC0_PKB_38 = crate::Reg<ltc0_ltc0_pkb_38::LTC0_LTC0_PKB_38_SPEC>;
#[doc = "LTC PKHA B 38 Register"]
pub mod ltc0_ltc0_pkb_38;
#[doc = "LTC0_LTC0_PKB2_7 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_7_SPEC>`"]
pub type LTC0_LTC0_PKB2_7 = crate::Reg<ltc0_ltc0_pkb2_7::LTC0_LTC0_PKB2_7_SPEC>;
#[doc = "LTC PKHA B2 7 Register"]
pub mod ltc0_ltc0_pkb2_7;
#[doc = "LTC0_LTC0_PKB_39 register accessor: an alias for `Reg<LTC0_LTC0_PKB_39_SPEC>`"]
pub type LTC0_LTC0_PKB_39 = crate::Reg<ltc0_ltc0_pkb_39::LTC0_LTC0_PKB_39_SPEC>;
#[doc = "LTC PKHA B 39 Register"]
pub mod ltc0_ltc0_pkb_39;
#[doc = "LTC0_LTC0_PKB2_8 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_8_SPEC>`"]
pub type LTC0_LTC0_PKB2_8 = crate::Reg<ltc0_ltc0_pkb2_8::LTC0_LTC0_PKB2_8_SPEC>;
#[doc = "LTC PKHA B2 8 Register"]
pub mod ltc0_ltc0_pkb2_8;
#[doc = "LTC0_LTC0_PKB_40 register accessor: an alias for `Reg<LTC0_LTC0_PKB_40_SPEC>`"]
pub type LTC0_LTC0_PKB_40 = crate::Reg<ltc0_ltc0_pkb_40::LTC0_LTC0_PKB_40_SPEC>;
#[doc = "LTC PKHA B 40 Register"]
pub mod ltc0_ltc0_pkb_40;
#[doc = "LTC0_LTC0_PKB2_9 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_9_SPEC>`"]
pub type LTC0_LTC0_PKB2_9 = crate::Reg<ltc0_ltc0_pkb2_9::LTC0_LTC0_PKB2_9_SPEC>;
#[doc = "LTC PKHA B2 9 Register"]
pub mod ltc0_ltc0_pkb2_9;
#[doc = "LTC0_LTC0_PKB_41 register accessor: an alias for `Reg<LTC0_LTC0_PKB_41_SPEC>`"]
pub type LTC0_LTC0_PKB_41 = crate::Reg<ltc0_ltc0_pkb_41::LTC0_LTC0_PKB_41_SPEC>;
#[doc = "LTC PKHA B 41 Register"]
pub mod ltc0_ltc0_pkb_41;
#[doc = "LTC0_LTC0_PKB2_10 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_10_SPEC>`"]
pub type LTC0_LTC0_PKB2_10 = crate::Reg<ltc0_ltc0_pkb2_10::LTC0_LTC0_PKB2_10_SPEC>;
#[doc = "LTC PKHA B2 10 Register"]
pub mod ltc0_ltc0_pkb2_10;
#[doc = "LTC0_LTC0_PKB_42 register accessor: an alias for `Reg<LTC0_LTC0_PKB_42_SPEC>`"]
pub type LTC0_LTC0_PKB_42 = crate::Reg<ltc0_ltc0_pkb_42::LTC0_LTC0_PKB_42_SPEC>;
#[doc = "LTC PKHA B 42 Register"]
pub mod ltc0_ltc0_pkb_42;
#[doc = "LTC0_LTC0_PKB2_11 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_11_SPEC>`"]
pub type LTC0_LTC0_PKB2_11 = crate::Reg<ltc0_ltc0_pkb2_11::LTC0_LTC0_PKB2_11_SPEC>;
#[doc = "LTC PKHA B2 11 Register"]
pub mod ltc0_ltc0_pkb2_11;
#[doc = "LTC0_LTC0_PKB_43 register accessor: an alias for `Reg<LTC0_LTC0_PKB_43_SPEC>`"]
pub type LTC0_LTC0_PKB_43 = crate::Reg<ltc0_ltc0_pkb_43::LTC0_LTC0_PKB_43_SPEC>;
#[doc = "LTC PKHA B 43 Register"]
pub mod ltc0_ltc0_pkb_43;
#[doc = "LTC0_LTC0_PKB2_12 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_12_SPEC>`"]
pub type LTC0_LTC0_PKB2_12 = crate::Reg<ltc0_ltc0_pkb2_12::LTC0_LTC0_PKB2_12_SPEC>;
#[doc = "LTC PKHA B2 12 Register"]
pub mod ltc0_ltc0_pkb2_12;
#[doc = "LTC0_LTC0_PKB_44 register accessor: an alias for `Reg<LTC0_LTC0_PKB_44_SPEC>`"]
pub type LTC0_LTC0_PKB_44 = crate::Reg<ltc0_ltc0_pkb_44::LTC0_LTC0_PKB_44_SPEC>;
#[doc = "LTC PKHA B 44 Register"]
pub mod ltc0_ltc0_pkb_44;
#[doc = "LTC0_LTC0_PKB2_13 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_13_SPEC>`"]
pub type LTC0_LTC0_PKB2_13 = crate::Reg<ltc0_ltc0_pkb2_13::LTC0_LTC0_PKB2_13_SPEC>;
#[doc = "LTC PKHA B2 13 Register"]
pub mod ltc0_ltc0_pkb2_13;
#[doc = "LTC0_LTC0_PKB_45 register accessor: an alias for `Reg<LTC0_LTC0_PKB_45_SPEC>`"]
pub type LTC0_LTC0_PKB_45 = crate::Reg<ltc0_ltc0_pkb_45::LTC0_LTC0_PKB_45_SPEC>;
#[doc = "LTC PKHA B 45 Register"]
pub mod ltc0_ltc0_pkb_45;
#[doc = "LTC0_LTC0_PKB2_14 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_14_SPEC>`"]
pub type LTC0_LTC0_PKB2_14 = crate::Reg<ltc0_ltc0_pkb2_14::LTC0_LTC0_PKB2_14_SPEC>;
#[doc = "LTC PKHA B2 14 Register"]
pub mod ltc0_ltc0_pkb2_14;
#[doc = "LTC0_LTC0_PKB_46 register accessor: an alias for `Reg<LTC0_LTC0_PKB_46_SPEC>`"]
pub type LTC0_LTC0_PKB_46 = crate::Reg<ltc0_ltc0_pkb_46::LTC0_LTC0_PKB_46_SPEC>;
#[doc = "LTC PKHA B 46 Register"]
pub mod ltc0_ltc0_pkb_46;
#[doc = "LTC0_LTC0_PKB2_15 register accessor: an alias for `Reg<LTC0_LTC0_PKB2_15_SPEC>`"]
pub type LTC0_LTC0_PKB2_15 = crate::Reg<ltc0_ltc0_pkb2_15::LTC0_LTC0_PKB2_15_SPEC>;
#[doc = "LTC PKHA B2 15 Register"]
pub mod ltc0_ltc0_pkb2_15;
#[doc = "LTC0_LTC0_PKB_47 register accessor: an alias for `Reg<LTC0_LTC0_PKB_47_SPEC>`"]
pub type LTC0_LTC0_PKB_47 = crate::Reg<ltc0_ltc0_pkb_47::LTC0_LTC0_PKB_47_SPEC>;
#[doc = "LTC PKHA B 47 Register"]
pub mod ltc0_ltc0_pkb_47;
#[doc = "LTC0_LTC0_PKB3_0 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_0_SPEC>`"]
pub type LTC0_LTC0_PKB3_0 = crate::Reg<ltc0_ltc0_pkb3_0::LTC0_LTC0_PKB3_0_SPEC>;
#[doc = "LTC PKHA B3 0 Register"]
pub mod ltc0_ltc0_pkb3_0;
#[doc = "LTC0_LTC0_PKB_48 register accessor: an alias for `Reg<LTC0_LTC0_PKB_48_SPEC>`"]
pub type LTC0_LTC0_PKB_48 = crate::Reg<ltc0_ltc0_pkb_48::LTC0_LTC0_PKB_48_SPEC>;
#[doc = "LTC PKHA B 48 Register"]
pub mod ltc0_ltc0_pkb_48;
#[doc = "LTC0_LTC0_PKB3_1 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_1_SPEC>`"]
pub type LTC0_LTC0_PKB3_1 = crate::Reg<ltc0_ltc0_pkb3_1::LTC0_LTC0_PKB3_1_SPEC>;
#[doc = "LTC PKHA B3 1 Register"]
pub mod ltc0_ltc0_pkb3_1;
#[doc = "LTC0_LTC0_PKB_49 register accessor: an alias for `Reg<LTC0_LTC0_PKB_49_SPEC>`"]
pub type LTC0_LTC0_PKB_49 = crate::Reg<ltc0_ltc0_pkb_49::LTC0_LTC0_PKB_49_SPEC>;
#[doc = "LTC PKHA B 49 Register"]
pub mod ltc0_ltc0_pkb_49;
#[doc = "LTC0_LTC0_PKB3_2 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_2_SPEC>`"]
pub type LTC0_LTC0_PKB3_2 = crate::Reg<ltc0_ltc0_pkb3_2::LTC0_LTC0_PKB3_2_SPEC>;
#[doc = "LTC PKHA B3 2 Register"]
pub mod ltc0_ltc0_pkb3_2;
#[doc = "LTC0_LTC0_PKB_50 register accessor: an alias for `Reg<LTC0_LTC0_PKB_50_SPEC>`"]
pub type LTC0_LTC0_PKB_50 = crate::Reg<ltc0_ltc0_pkb_50::LTC0_LTC0_PKB_50_SPEC>;
#[doc = "LTC PKHA B 50 Register"]
pub mod ltc0_ltc0_pkb_50;
#[doc = "LTC0_LTC0_PKB3_3 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_3_SPEC>`"]
pub type LTC0_LTC0_PKB3_3 = crate::Reg<ltc0_ltc0_pkb3_3::LTC0_LTC0_PKB3_3_SPEC>;
#[doc = "LTC PKHA B3 3 Register"]
pub mod ltc0_ltc0_pkb3_3;
#[doc = "LTC0_LTC0_PKB_51 register accessor: an alias for `Reg<LTC0_LTC0_PKB_51_SPEC>`"]
pub type LTC0_LTC0_PKB_51 = crate::Reg<ltc0_ltc0_pkb_51::LTC0_LTC0_PKB_51_SPEC>;
#[doc = "LTC PKHA B 51 Register"]
pub mod ltc0_ltc0_pkb_51;
#[doc = "LTC0_LTC0_PKB3_4 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_4_SPEC>`"]
pub type LTC0_LTC0_PKB3_4 = crate::Reg<ltc0_ltc0_pkb3_4::LTC0_LTC0_PKB3_4_SPEC>;
#[doc = "LTC PKHA B3 4 Register"]
pub mod ltc0_ltc0_pkb3_4;
#[doc = "LTC0_LTC0_PKB_52 register accessor: an alias for `Reg<LTC0_LTC0_PKB_52_SPEC>`"]
pub type LTC0_LTC0_PKB_52 = crate::Reg<ltc0_ltc0_pkb_52::LTC0_LTC0_PKB_52_SPEC>;
#[doc = "LTC PKHA B 52 Register"]
pub mod ltc0_ltc0_pkb_52;
#[doc = "LTC0_LTC0_PKB3_5 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_5_SPEC>`"]
pub type LTC0_LTC0_PKB3_5 = crate::Reg<ltc0_ltc0_pkb3_5::LTC0_LTC0_PKB3_5_SPEC>;
#[doc = "LTC PKHA B3 5 Register"]
pub mod ltc0_ltc0_pkb3_5;
#[doc = "LTC0_LTC0_PKB_53 register accessor: an alias for `Reg<LTC0_LTC0_PKB_53_SPEC>`"]
pub type LTC0_LTC0_PKB_53 = crate::Reg<ltc0_ltc0_pkb_53::LTC0_LTC0_PKB_53_SPEC>;
#[doc = "LTC PKHA B 53 Register"]
pub mod ltc0_ltc0_pkb_53;
#[doc = "LTC0_LTC0_PKB3_6 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_6_SPEC>`"]
pub type LTC0_LTC0_PKB3_6 = crate::Reg<ltc0_ltc0_pkb3_6::LTC0_LTC0_PKB3_6_SPEC>;
#[doc = "LTC PKHA B3 6 Register"]
pub mod ltc0_ltc0_pkb3_6;
#[doc = "LTC0_LTC0_PKB_54 register accessor: an alias for `Reg<LTC0_LTC0_PKB_54_SPEC>`"]
pub type LTC0_LTC0_PKB_54 = crate::Reg<ltc0_ltc0_pkb_54::LTC0_LTC0_PKB_54_SPEC>;
#[doc = "LTC PKHA B 54 Register"]
pub mod ltc0_ltc0_pkb_54;
#[doc = "LTC0_LTC0_PKB3_7 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_7_SPEC>`"]
pub type LTC0_LTC0_PKB3_7 = crate::Reg<ltc0_ltc0_pkb3_7::LTC0_LTC0_PKB3_7_SPEC>;
#[doc = "LTC PKHA B3 7 Register"]
pub mod ltc0_ltc0_pkb3_7;
#[doc = "LTC0_LTC0_PKB_55 register accessor: an alias for `Reg<LTC0_LTC0_PKB_55_SPEC>`"]
pub type LTC0_LTC0_PKB_55 = crate::Reg<ltc0_ltc0_pkb_55::LTC0_LTC0_PKB_55_SPEC>;
#[doc = "LTC PKHA B 55 Register"]
pub mod ltc0_ltc0_pkb_55;
#[doc = "LTC0_LTC0_PKB3_8 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_8_SPEC>`"]
pub type LTC0_LTC0_PKB3_8 = crate::Reg<ltc0_ltc0_pkb3_8::LTC0_LTC0_PKB3_8_SPEC>;
#[doc = "LTC PKHA B3 8 Register"]
pub mod ltc0_ltc0_pkb3_8;
#[doc = "LTC0_LTC0_PKB_56 register accessor: an alias for `Reg<LTC0_LTC0_PKB_56_SPEC>`"]
pub type LTC0_LTC0_PKB_56 = crate::Reg<ltc0_ltc0_pkb_56::LTC0_LTC0_PKB_56_SPEC>;
#[doc = "LTC PKHA B 56 Register"]
pub mod ltc0_ltc0_pkb_56;
#[doc = "LTC0_LTC0_PKB3_9 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_9_SPEC>`"]
pub type LTC0_LTC0_PKB3_9 = crate::Reg<ltc0_ltc0_pkb3_9::LTC0_LTC0_PKB3_9_SPEC>;
#[doc = "LTC PKHA B3 9 Register"]
pub mod ltc0_ltc0_pkb3_9;
#[doc = "LTC0_LTC0_PKB_57 register accessor: an alias for `Reg<LTC0_LTC0_PKB_57_SPEC>`"]
pub type LTC0_LTC0_PKB_57 = crate::Reg<ltc0_ltc0_pkb_57::LTC0_LTC0_PKB_57_SPEC>;
#[doc = "LTC PKHA B 57 Register"]
pub mod ltc0_ltc0_pkb_57;
#[doc = "LTC0_LTC0_PKB3_10 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_10_SPEC>`"]
pub type LTC0_LTC0_PKB3_10 = crate::Reg<ltc0_ltc0_pkb3_10::LTC0_LTC0_PKB3_10_SPEC>;
#[doc = "LTC PKHA B3 10 Register"]
pub mod ltc0_ltc0_pkb3_10;
#[doc = "LTC0_LTC0_PKB_58 register accessor: an alias for `Reg<LTC0_LTC0_PKB_58_SPEC>`"]
pub type LTC0_LTC0_PKB_58 = crate::Reg<ltc0_ltc0_pkb_58::LTC0_LTC0_PKB_58_SPEC>;
#[doc = "LTC PKHA B 58 Register"]
pub mod ltc0_ltc0_pkb_58;
#[doc = "LTC0_LTC0_PKB3_11 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_11_SPEC>`"]
pub type LTC0_LTC0_PKB3_11 = crate::Reg<ltc0_ltc0_pkb3_11::LTC0_LTC0_PKB3_11_SPEC>;
#[doc = "LTC PKHA B3 11 Register"]
pub mod ltc0_ltc0_pkb3_11;
#[doc = "LTC0_LTC0_PKB_59 register accessor: an alias for `Reg<LTC0_LTC0_PKB_59_SPEC>`"]
pub type LTC0_LTC0_PKB_59 = crate::Reg<ltc0_ltc0_pkb_59::LTC0_LTC0_PKB_59_SPEC>;
#[doc = "LTC PKHA B 59 Register"]
pub mod ltc0_ltc0_pkb_59;
#[doc = "LTC0_LTC0_PKB3_12 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_12_SPEC>`"]
pub type LTC0_LTC0_PKB3_12 = crate::Reg<ltc0_ltc0_pkb3_12::LTC0_LTC0_PKB3_12_SPEC>;
#[doc = "LTC PKHA B3 12 Register"]
pub mod ltc0_ltc0_pkb3_12;
#[doc = "LTC0_LTC0_PKB_60 register accessor: an alias for `Reg<LTC0_LTC0_PKB_60_SPEC>`"]
pub type LTC0_LTC0_PKB_60 = crate::Reg<ltc0_ltc0_pkb_60::LTC0_LTC0_PKB_60_SPEC>;
#[doc = "LTC PKHA B 60 Register"]
pub mod ltc0_ltc0_pkb_60;
#[doc = "LTC0_LTC0_PKB3_13 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_13_SPEC>`"]
pub type LTC0_LTC0_PKB3_13 = crate::Reg<ltc0_ltc0_pkb3_13::LTC0_LTC0_PKB3_13_SPEC>;
#[doc = "LTC PKHA B3 13 Register"]
pub mod ltc0_ltc0_pkb3_13;
#[doc = "LTC0_LTC0_PKB_61 register accessor: an alias for `Reg<LTC0_LTC0_PKB_61_SPEC>`"]
pub type LTC0_LTC0_PKB_61 = crate::Reg<ltc0_ltc0_pkb_61::LTC0_LTC0_PKB_61_SPEC>;
#[doc = "LTC PKHA B 61 Register"]
pub mod ltc0_ltc0_pkb_61;
#[doc = "LTC0_LTC0_PKB3_14 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_14_SPEC>`"]
pub type LTC0_LTC0_PKB3_14 = crate::Reg<ltc0_ltc0_pkb3_14::LTC0_LTC0_PKB3_14_SPEC>;
#[doc = "LTC PKHA B3 14 Register"]
pub mod ltc0_ltc0_pkb3_14;
#[doc = "LTC0_LTC0_PKB_62 register accessor: an alias for `Reg<LTC0_LTC0_PKB_62_SPEC>`"]
pub type LTC0_LTC0_PKB_62 = crate::Reg<ltc0_ltc0_pkb_62::LTC0_LTC0_PKB_62_SPEC>;
#[doc = "LTC PKHA B 62 Register"]
pub mod ltc0_ltc0_pkb_62;
#[doc = "LTC0_LTC0_PKB3_15 register accessor: an alias for `Reg<LTC0_LTC0_PKB3_15_SPEC>`"]
pub type LTC0_LTC0_PKB3_15 = crate::Reg<ltc0_ltc0_pkb3_15::LTC0_LTC0_PKB3_15_SPEC>;
#[doc = "LTC PKHA B3 15 Register"]
pub mod ltc0_ltc0_pkb3_15;
#[doc = "LTC0_LTC0_PKB_63 register accessor: an alias for `Reg<LTC0_LTC0_PKB_63_SPEC>`"]
pub type LTC0_LTC0_PKB_63 = crate::Reg<ltc0_ltc0_pkb_63::LTC0_LTC0_PKB_63_SPEC>;
#[doc = "LTC PKHA B 63 Register"]
pub mod ltc0_ltc0_pkb_63;
#[doc = "LTC0_LTC0_PKN0_0 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_0_SPEC>`"]
pub type LTC0_LTC0_PKN0_0 = crate::Reg<ltc0_ltc0_pkn0_0::LTC0_LTC0_PKN0_0_SPEC>;
#[doc = "LTC PKHA N0 0 Register"]
pub mod ltc0_ltc0_pkn0_0;
#[doc = "LTC0_LTC0_PKN_0 register accessor: an alias for `Reg<LTC0_LTC0_PKN_0_SPEC>`"]
pub type LTC0_LTC0_PKN_0 = crate::Reg<ltc0_ltc0_pkn_0::LTC0_LTC0_PKN_0_SPEC>;
#[doc = "LTC PKHA N 0 Register"]
pub mod ltc0_ltc0_pkn_0;
#[doc = "LTC0_LTC0_PKN0_1 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_1_SPEC>`"]
pub type LTC0_LTC0_PKN0_1 = crate::Reg<ltc0_ltc0_pkn0_1::LTC0_LTC0_PKN0_1_SPEC>;
#[doc = "LTC PKHA N0 1 Register"]
pub mod ltc0_ltc0_pkn0_1;
#[doc = "LTC0_LTC0_PKN_1 register accessor: an alias for `Reg<LTC0_LTC0_PKN_1_SPEC>`"]
pub type LTC0_LTC0_PKN_1 = crate::Reg<ltc0_ltc0_pkn_1::LTC0_LTC0_PKN_1_SPEC>;
#[doc = "LTC PKHA N 1 Register"]
pub mod ltc0_ltc0_pkn_1;
#[doc = "LTC0_LTC0_PKN0_2 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_2_SPEC>`"]
pub type LTC0_LTC0_PKN0_2 = crate::Reg<ltc0_ltc0_pkn0_2::LTC0_LTC0_PKN0_2_SPEC>;
#[doc = "LTC PKHA N0 2 Register"]
pub mod ltc0_ltc0_pkn0_2;
#[doc = "LTC0_LTC0_PKN_2 register accessor: an alias for `Reg<LTC0_LTC0_PKN_2_SPEC>`"]
pub type LTC0_LTC0_PKN_2 = crate::Reg<ltc0_ltc0_pkn_2::LTC0_LTC0_PKN_2_SPEC>;
#[doc = "LTC PKHA N 2 Register"]
pub mod ltc0_ltc0_pkn_2;
#[doc = "LTC0_LTC0_PKN0_3 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_3_SPEC>`"]
pub type LTC0_LTC0_PKN0_3 = crate::Reg<ltc0_ltc0_pkn0_3::LTC0_LTC0_PKN0_3_SPEC>;
#[doc = "LTC PKHA N0 3 Register"]
pub mod ltc0_ltc0_pkn0_3;
#[doc = "LTC0_LTC0_PKN_3 register accessor: an alias for `Reg<LTC0_LTC0_PKN_3_SPEC>`"]
pub type LTC0_LTC0_PKN_3 = crate::Reg<ltc0_ltc0_pkn_3::LTC0_LTC0_PKN_3_SPEC>;
#[doc = "LTC PKHA N 3 Register"]
pub mod ltc0_ltc0_pkn_3;
#[doc = "LTC0_LTC0_PKN0_4 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_4_SPEC>`"]
pub type LTC0_LTC0_PKN0_4 = crate::Reg<ltc0_ltc0_pkn0_4::LTC0_LTC0_PKN0_4_SPEC>;
#[doc = "LTC PKHA N0 4 Register"]
pub mod ltc0_ltc0_pkn0_4;
#[doc = "LTC0_LTC0_PKN_4 register accessor: an alias for `Reg<LTC0_LTC0_PKN_4_SPEC>`"]
pub type LTC0_LTC0_PKN_4 = crate::Reg<ltc0_ltc0_pkn_4::LTC0_LTC0_PKN_4_SPEC>;
#[doc = "LTC PKHA N 4 Register"]
pub mod ltc0_ltc0_pkn_4;
#[doc = "LTC0_LTC0_PKN0_5 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_5_SPEC>`"]
pub type LTC0_LTC0_PKN0_5 = crate::Reg<ltc0_ltc0_pkn0_5::LTC0_LTC0_PKN0_5_SPEC>;
#[doc = "LTC PKHA N0 5 Register"]
pub mod ltc0_ltc0_pkn0_5;
#[doc = "LTC0_LTC0_PKN_5 register accessor: an alias for `Reg<LTC0_LTC0_PKN_5_SPEC>`"]
pub type LTC0_LTC0_PKN_5 = crate::Reg<ltc0_ltc0_pkn_5::LTC0_LTC0_PKN_5_SPEC>;
#[doc = "LTC PKHA N 5 Register"]
pub mod ltc0_ltc0_pkn_5;
#[doc = "LTC0_LTC0_PKN0_6 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_6_SPEC>`"]
pub type LTC0_LTC0_PKN0_6 = crate::Reg<ltc0_ltc0_pkn0_6::LTC0_LTC0_PKN0_6_SPEC>;
#[doc = "LTC PKHA N0 6 Register"]
pub mod ltc0_ltc0_pkn0_6;
#[doc = "LTC0_LTC0_PKN_6 register accessor: an alias for `Reg<LTC0_LTC0_PKN_6_SPEC>`"]
pub type LTC0_LTC0_PKN_6 = crate::Reg<ltc0_ltc0_pkn_6::LTC0_LTC0_PKN_6_SPEC>;
#[doc = "LTC PKHA N 6 Register"]
pub mod ltc0_ltc0_pkn_6;
#[doc = "LTC0_LTC0_PKN0_7 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_7_SPEC>`"]
pub type LTC0_LTC0_PKN0_7 = crate::Reg<ltc0_ltc0_pkn0_7::LTC0_LTC0_PKN0_7_SPEC>;
#[doc = "LTC PKHA N0 7 Register"]
pub mod ltc0_ltc0_pkn0_7;
#[doc = "LTC0_LTC0_PKN_7 register accessor: an alias for `Reg<LTC0_LTC0_PKN_7_SPEC>`"]
pub type LTC0_LTC0_PKN_7 = crate::Reg<ltc0_ltc0_pkn_7::LTC0_LTC0_PKN_7_SPEC>;
#[doc = "LTC PKHA N 7 Register"]
pub mod ltc0_ltc0_pkn_7;
#[doc = "LTC0_LTC0_PKN0_8 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_8_SPEC>`"]
pub type LTC0_LTC0_PKN0_8 = crate::Reg<ltc0_ltc0_pkn0_8::LTC0_LTC0_PKN0_8_SPEC>;
#[doc = "LTC PKHA N0 8 Register"]
pub mod ltc0_ltc0_pkn0_8;
#[doc = "LTC0_LTC0_PKN_8 register accessor: an alias for `Reg<LTC0_LTC0_PKN_8_SPEC>`"]
pub type LTC0_LTC0_PKN_8 = crate::Reg<ltc0_ltc0_pkn_8::LTC0_LTC0_PKN_8_SPEC>;
#[doc = "LTC PKHA N 8 Register"]
pub mod ltc0_ltc0_pkn_8;
#[doc = "LTC0_LTC0_PKN0_9 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_9_SPEC>`"]
pub type LTC0_LTC0_PKN0_9 = crate::Reg<ltc0_ltc0_pkn0_9::LTC0_LTC0_PKN0_9_SPEC>;
#[doc = "LTC PKHA N0 9 Register"]
pub mod ltc0_ltc0_pkn0_9;
#[doc = "LTC0_LTC0_PKN_9 register accessor: an alias for `Reg<LTC0_LTC0_PKN_9_SPEC>`"]
pub type LTC0_LTC0_PKN_9 = crate::Reg<ltc0_ltc0_pkn_9::LTC0_LTC0_PKN_9_SPEC>;
#[doc = "LTC PKHA N 9 Register"]
pub mod ltc0_ltc0_pkn_9;
#[doc = "LTC0_LTC0_PKN0_10 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_10_SPEC>`"]
pub type LTC0_LTC0_PKN0_10 = crate::Reg<ltc0_ltc0_pkn0_10::LTC0_LTC0_PKN0_10_SPEC>;
#[doc = "LTC PKHA N0 10 Register"]
pub mod ltc0_ltc0_pkn0_10;
#[doc = "LTC0_LTC0_PKN_10 register accessor: an alias for `Reg<LTC0_LTC0_PKN_10_SPEC>`"]
pub type LTC0_LTC0_PKN_10 = crate::Reg<ltc0_ltc0_pkn_10::LTC0_LTC0_PKN_10_SPEC>;
#[doc = "LTC PKHA N 10 Register"]
pub mod ltc0_ltc0_pkn_10;
#[doc = "LTC0_LTC0_PKN0_11 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_11_SPEC>`"]
pub type LTC0_LTC0_PKN0_11 = crate::Reg<ltc0_ltc0_pkn0_11::LTC0_LTC0_PKN0_11_SPEC>;
#[doc = "LTC PKHA N0 11 Register"]
pub mod ltc0_ltc0_pkn0_11;
#[doc = "LTC0_LTC0_PKN_11 register accessor: an alias for `Reg<LTC0_LTC0_PKN_11_SPEC>`"]
pub type LTC0_LTC0_PKN_11 = crate::Reg<ltc0_ltc0_pkn_11::LTC0_LTC0_PKN_11_SPEC>;
#[doc = "LTC PKHA N 11 Register"]
pub mod ltc0_ltc0_pkn_11;
#[doc = "LTC0_LTC0_PKN0_12 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_12_SPEC>`"]
pub type LTC0_LTC0_PKN0_12 = crate::Reg<ltc0_ltc0_pkn0_12::LTC0_LTC0_PKN0_12_SPEC>;
#[doc = "LTC PKHA N0 12 Register"]
pub mod ltc0_ltc0_pkn0_12;
#[doc = "LTC0_LTC0_PKN_12 register accessor: an alias for `Reg<LTC0_LTC0_PKN_12_SPEC>`"]
pub type LTC0_LTC0_PKN_12 = crate::Reg<ltc0_ltc0_pkn_12::LTC0_LTC0_PKN_12_SPEC>;
#[doc = "LTC PKHA N 12 Register"]
pub mod ltc0_ltc0_pkn_12;
#[doc = "LTC0_LTC0_PKN0_13 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_13_SPEC>`"]
pub type LTC0_LTC0_PKN0_13 = crate::Reg<ltc0_ltc0_pkn0_13::LTC0_LTC0_PKN0_13_SPEC>;
#[doc = "LTC PKHA N0 13 Register"]
pub mod ltc0_ltc0_pkn0_13;
#[doc = "LTC0_LTC0_PKN_13 register accessor: an alias for `Reg<LTC0_LTC0_PKN_13_SPEC>`"]
pub type LTC0_LTC0_PKN_13 = crate::Reg<ltc0_ltc0_pkn_13::LTC0_LTC0_PKN_13_SPEC>;
#[doc = "LTC PKHA N 13 Register"]
pub mod ltc0_ltc0_pkn_13;
#[doc = "LTC0_LTC0_PKN0_14 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_14_SPEC>`"]
pub type LTC0_LTC0_PKN0_14 = crate::Reg<ltc0_ltc0_pkn0_14::LTC0_LTC0_PKN0_14_SPEC>;
#[doc = "LTC PKHA N0 14 Register"]
pub mod ltc0_ltc0_pkn0_14;
#[doc = "LTC0_LTC0_PKN_14 register accessor: an alias for `Reg<LTC0_LTC0_PKN_14_SPEC>`"]
pub type LTC0_LTC0_PKN_14 = crate::Reg<ltc0_ltc0_pkn_14::LTC0_LTC0_PKN_14_SPEC>;
#[doc = "LTC PKHA N 14 Register"]
pub mod ltc0_ltc0_pkn_14;
#[doc = "LTC0_LTC0_PKN0_15 register accessor: an alias for `Reg<LTC0_LTC0_PKN0_15_SPEC>`"]
pub type LTC0_LTC0_PKN0_15 = crate::Reg<ltc0_ltc0_pkn0_15::LTC0_LTC0_PKN0_15_SPEC>;
#[doc = "LTC PKHA N0 15 Register"]
pub mod ltc0_ltc0_pkn0_15;
#[doc = "LTC0_LTC0_PKN_15 register accessor: an alias for `Reg<LTC0_LTC0_PKN_15_SPEC>`"]
pub type LTC0_LTC0_PKN_15 = crate::Reg<ltc0_ltc0_pkn_15::LTC0_LTC0_PKN_15_SPEC>;
#[doc = "LTC PKHA N 15 Register"]
pub mod ltc0_ltc0_pkn_15;
#[doc = "LTC0_LTC0_PKN1_0 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_0_SPEC>`"]
pub type LTC0_LTC0_PKN1_0 = crate::Reg<ltc0_ltc0_pkn1_0::LTC0_LTC0_PKN1_0_SPEC>;
#[doc = "LTC PKHA N1 0 Register"]
pub mod ltc0_ltc0_pkn1_0;
#[doc = "LTC0_LTC0_PKN_16 register accessor: an alias for `Reg<LTC0_LTC0_PKN_16_SPEC>`"]
pub type LTC0_LTC0_PKN_16 = crate::Reg<ltc0_ltc0_pkn_16::LTC0_LTC0_PKN_16_SPEC>;
#[doc = "LTC PKHA N 16 Register"]
pub mod ltc0_ltc0_pkn_16;
#[doc = "LTC0_LTC0_PKN1_1 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_1_SPEC>`"]
pub type LTC0_LTC0_PKN1_1 = crate::Reg<ltc0_ltc0_pkn1_1::LTC0_LTC0_PKN1_1_SPEC>;
#[doc = "LTC PKHA N1 1 Register"]
pub mod ltc0_ltc0_pkn1_1;
#[doc = "LTC0_LTC0_PKN_17 register accessor: an alias for `Reg<LTC0_LTC0_PKN_17_SPEC>`"]
pub type LTC0_LTC0_PKN_17 = crate::Reg<ltc0_ltc0_pkn_17::LTC0_LTC0_PKN_17_SPEC>;
#[doc = "LTC PKHA N 17 Register"]
pub mod ltc0_ltc0_pkn_17;
#[doc = "LTC0_LTC0_PKN1_2 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_2_SPEC>`"]
pub type LTC0_LTC0_PKN1_2 = crate::Reg<ltc0_ltc0_pkn1_2::LTC0_LTC0_PKN1_2_SPEC>;
#[doc = "LTC PKHA N1 2 Register"]
pub mod ltc0_ltc0_pkn1_2;
#[doc = "LTC0_LTC0_PKN_18 register accessor: an alias for `Reg<LTC0_LTC0_PKN_18_SPEC>`"]
pub type LTC0_LTC0_PKN_18 = crate::Reg<ltc0_ltc0_pkn_18::LTC0_LTC0_PKN_18_SPEC>;
#[doc = "LTC PKHA N 18 Register"]
pub mod ltc0_ltc0_pkn_18;
#[doc = "LTC0_LTC0_PKN1_3 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_3_SPEC>`"]
pub type LTC0_LTC0_PKN1_3 = crate::Reg<ltc0_ltc0_pkn1_3::LTC0_LTC0_PKN1_3_SPEC>;
#[doc = "LTC PKHA N1 3 Register"]
pub mod ltc0_ltc0_pkn1_3;
#[doc = "LTC0_LTC0_PKN_19 register accessor: an alias for `Reg<LTC0_LTC0_PKN_19_SPEC>`"]
pub type LTC0_LTC0_PKN_19 = crate::Reg<ltc0_ltc0_pkn_19::LTC0_LTC0_PKN_19_SPEC>;
#[doc = "LTC PKHA N 19 Register"]
pub mod ltc0_ltc0_pkn_19;
#[doc = "LTC0_LTC0_PKN1_4 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_4_SPEC>`"]
pub type LTC0_LTC0_PKN1_4 = crate::Reg<ltc0_ltc0_pkn1_4::LTC0_LTC0_PKN1_4_SPEC>;
#[doc = "LTC PKHA N1 4 Register"]
pub mod ltc0_ltc0_pkn1_4;
#[doc = "LTC0_LTC0_PKN_20 register accessor: an alias for `Reg<LTC0_LTC0_PKN_20_SPEC>`"]
pub type LTC0_LTC0_PKN_20 = crate::Reg<ltc0_ltc0_pkn_20::LTC0_LTC0_PKN_20_SPEC>;
#[doc = "LTC PKHA N 20 Register"]
pub mod ltc0_ltc0_pkn_20;
#[doc = "LTC0_LTC0_PKN1_5 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_5_SPEC>`"]
pub type LTC0_LTC0_PKN1_5 = crate::Reg<ltc0_ltc0_pkn1_5::LTC0_LTC0_PKN1_5_SPEC>;
#[doc = "LTC PKHA N1 5 Register"]
pub mod ltc0_ltc0_pkn1_5;
#[doc = "LTC0_LTC0_PKN_21 register accessor: an alias for `Reg<LTC0_LTC0_PKN_21_SPEC>`"]
pub type LTC0_LTC0_PKN_21 = crate::Reg<ltc0_ltc0_pkn_21::LTC0_LTC0_PKN_21_SPEC>;
#[doc = "LTC PKHA N 21 Register"]
pub mod ltc0_ltc0_pkn_21;
#[doc = "LTC0_LTC0_PKN1_6 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_6_SPEC>`"]
pub type LTC0_LTC0_PKN1_6 = crate::Reg<ltc0_ltc0_pkn1_6::LTC0_LTC0_PKN1_6_SPEC>;
#[doc = "LTC PKHA N1 6 Register"]
pub mod ltc0_ltc0_pkn1_6;
#[doc = "LTC0_LTC0_PKN_22 register accessor: an alias for `Reg<LTC0_LTC0_PKN_22_SPEC>`"]
pub type LTC0_LTC0_PKN_22 = crate::Reg<ltc0_ltc0_pkn_22::LTC0_LTC0_PKN_22_SPEC>;
#[doc = "LTC PKHA N 22 Register"]
pub mod ltc0_ltc0_pkn_22;
#[doc = "LTC0_LTC0_PKN1_7 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_7_SPEC>`"]
pub type LTC0_LTC0_PKN1_7 = crate::Reg<ltc0_ltc0_pkn1_7::LTC0_LTC0_PKN1_7_SPEC>;
#[doc = "LTC PKHA N1 7 Register"]
pub mod ltc0_ltc0_pkn1_7;
#[doc = "LTC0_LTC0_PKN_23 register accessor: an alias for `Reg<LTC0_LTC0_PKN_23_SPEC>`"]
pub type LTC0_LTC0_PKN_23 = crate::Reg<ltc0_ltc0_pkn_23::LTC0_LTC0_PKN_23_SPEC>;
#[doc = "LTC PKHA N 23 Register"]
pub mod ltc0_ltc0_pkn_23;
#[doc = "LTC0_LTC0_PKN1_8 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_8_SPEC>`"]
pub type LTC0_LTC0_PKN1_8 = crate::Reg<ltc0_ltc0_pkn1_8::LTC0_LTC0_PKN1_8_SPEC>;
#[doc = "LTC PKHA N1 8 Register"]
pub mod ltc0_ltc0_pkn1_8;
#[doc = "LTC0_LTC0_PKN_24 register accessor: an alias for `Reg<LTC0_LTC0_PKN_24_SPEC>`"]
pub type LTC0_LTC0_PKN_24 = crate::Reg<ltc0_ltc0_pkn_24::LTC0_LTC0_PKN_24_SPEC>;
#[doc = "LTC PKHA N 24 Register"]
pub mod ltc0_ltc0_pkn_24;
#[doc = "LTC0_LTC0_PKN1_9 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_9_SPEC>`"]
pub type LTC0_LTC0_PKN1_9 = crate::Reg<ltc0_ltc0_pkn1_9::LTC0_LTC0_PKN1_9_SPEC>;
#[doc = "LTC PKHA N1 9 Register"]
pub mod ltc0_ltc0_pkn1_9;
#[doc = "LTC0_LTC0_PKN_25 register accessor: an alias for `Reg<LTC0_LTC0_PKN_25_SPEC>`"]
pub type LTC0_LTC0_PKN_25 = crate::Reg<ltc0_ltc0_pkn_25::LTC0_LTC0_PKN_25_SPEC>;
#[doc = "LTC PKHA N 25 Register"]
pub mod ltc0_ltc0_pkn_25;
#[doc = "LTC0_LTC0_PKN1_10 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_10_SPEC>`"]
pub type LTC0_LTC0_PKN1_10 = crate::Reg<ltc0_ltc0_pkn1_10::LTC0_LTC0_PKN1_10_SPEC>;
#[doc = "LTC PKHA N1 10 Register"]
pub mod ltc0_ltc0_pkn1_10;
#[doc = "LTC0_LTC0_PKN_26 register accessor: an alias for `Reg<LTC0_LTC0_PKN_26_SPEC>`"]
pub type LTC0_LTC0_PKN_26 = crate::Reg<ltc0_ltc0_pkn_26::LTC0_LTC0_PKN_26_SPEC>;
#[doc = "LTC PKHA N 26 Register"]
pub mod ltc0_ltc0_pkn_26;
#[doc = "LTC0_LTC0_PKN1_11 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_11_SPEC>`"]
pub type LTC0_LTC0_PKN1_11 = crate::Reg<ltc0_ltc0_pkn1_11::LTC0_LTC0_PKN1_11_SPEC>;
#[doc = "LTC PKHA N1 11 Register"]
pub mod ltc0_ltc0_pkn1_11;
#[doc = "LTC0_LTC0_PKN_27 register accessor: an alias for `Reg<LTC0_LTC0_PKN_27_SPEC>`"]
pub type LTC0_LTC0_PKN_27 = crate::Reg<ltc0_ltc0_pkn_27::LTC0_LTC0_PKN_27_SPEC>;
#[doc = "LTC PKHA N 27 Register"]
pub mod ltc0_ltc0_pkn_27;
#[doc = "LTC0_LTC0_PKN1_12 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_12_SPEC>`"]
pub type LTC0_LTC0_PKN1_12 = crate::Reg<ltc0_ltc0_pkn1_12::LTC0_LTC0_PKN1_12_SPEC>;
#[doc = "LTC PKHA N1 12 Register"]
pub mod ltc0_ltc0_pkn1_12;
#[doc = "LTC0_LTC0_PKN_28 register accessor: an alias for `Reg<LTC0_LTC0_PKN_28_SPEC>`"]
pub type LTC0_LTC0_PKN_28 = crate::Reg<ltc0_ltc0_pkn_28::LTC0_LTC0_PKN_28_SPEC>;
#[doc = "LTC PKHA N 28 Register"]
pub mod ltc0_ltc0_pkn_28;
#[doc = "LTC0_LTC0_PKN1_13 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_13_SPEC>`"]
pub type LTC0_LTC0_PKN1_13 = crate::Reg<ltc0_ltc0_pkn1_13::LTC0_LTC0_PKN1_13_SPEC>;
#[doc = "LTC PKHA N1 13 Register"]
pub mod ltc0_ltc0_pkn1_13;
#[doc = "LTC0_LTC0_PKN_29 register accessor: an alias for `Reg<LTC0_LTC0_PKN_29_SPEC>`"]
pub type LTC0_LTC0_PKN_29 = crate::Reg<ltc0_ltc0_pkn_29::LTC0_LTC0_PKN_29_SPEC>;
#[doc = "LTC PKHA N 29 Register"]
pub mod ltc0_ltc0_pkn_29;
#[doc = "LTC0_LTC0_PKN1_14 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_14_SPEC>`"]
pub type LTC0_LTC0_PKN1_14 = crate::Reg<ltc0_ltc0_pkn1_14::LTC0_LTC0_PKN1_14_SPEC>;
#[doc = "LTC PKHA N1 14 Register"]
pub mod ltc0_ltc0_pkn1_14;
#[doc = "LTC0_LTC0_PKN_30 register accessor: an alias for `Reg<LTC0_LTC0_PKN_30_SPEC>`"]
pub type LTC0_LTC0_PKN_30 = crate::Reg<ltc0_ltc0_pkn_30::LTC0_LTC0_PKN_30_SPEC>;
#[doc = "LTC PKHA N 30 Register"]
pub mod ltc0_ltc0_pkn_30;
#[doc = "LTC0_LTC0_PKN1_15 register accessor: an alias for `Reg<LTC0_LTC0_PKN1_15_SPEC>`"]
pub type LTC0_LTC0_PKN1_15 = crate::Reg<ltc0_ltc0_pkn1_15::LTC0_LTC0_PKN1_15_SPEC>;
#[doc = "LTC PKHA N1 15 Register"]
pub mod ltc0_ltc0_pkn1_15;
#[doc = "LTC0_LTC0_PKN_31 register accessor: an alias for `Reg<LTC0_LTC0_PKN_31_SPEC>`"]
pub type LTC0_LTC0_PKN_31 = crate::Reg<ltc0_ltc0_pkn_31::LTC0_LTC0_PKN_31_SPEC>;
#[doc = "LTC PKHA N 31 Register"]
pub mod ltc0_ltc0_pkn_31;
#[doc = "LTC0_LTC0_PKN2_0 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_0_SPEC>`"]
pub type LTC0_LTC0_PKN2_0 = crate::Reg<ltc0_ltc0_pkn2_0::LTC0_LTC0_PKN2_0_SPEC>;
#[doc = "LTC PKHA N2 0 Register"]
pub mod ltc0_ltc0_pkn2_0;
#[doc = "LTC0_LTC0_PKN_32 register accessor: an alias for `Reg<LTC0_LTC0_PKN_32_SPEC>`"]
pub type LTC0_LTC0_PKN_32 = crate::Reg<ltc0_ltc0_pkn_32::LTC0_LTC0_PKN_32_SPEC>;
#[doc = "LTC PKHA N 32 Register"]
pub mod ltc0_ltc0_pkn_32;
#[doc = "LTC0_LTC0_PKN2_1 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_1_SPEC>`"]
pub type LTC0_LTC0_PKN2_1 = crate::Reg<ltc0_ltc0_pkn2_1::LTC0_LTC0_PKN2_1_SPEC>;
#[doc = "LTC PKHA N2 1 Register"]
pub mod ltc0_ltc0_pkn2_1;
#[doc = "LTC0_LTC0_PKN_33 register accessor: an alias for `Reg<LTC0_LTC0_PKN_33_SPEC>`"]
pub type LTC0_LTC0_PKN_33 = crate::Reg<ltc0_ltc0_pkn_33::LTC0_LTC0_PKN_33_SPEC>;
#[doc = "LTC PKHA N 33 Register"]
pub mod ltc0_ltc0_pkn_33;
#[doc = "LTC0_LTC0_PKN2_2 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_2_SPEC>`"]
pub type LTC0_LTC0_PKN2_2 = crate::Reg<ltc0_ltc0_pkn2_2::LTC0_LTC0_PKN2_2_SPEC>;
#[doc = "LTC PKHA N2 2 Register"]
pub mod ltc0_ltc0_pkn2_2;
#[doc = "LTC0_LTC0_PKN_34 register accessor: an alias for `Reg<LTC0_LTC0_PKN_34_SPEC>`"]
pub type LTC0_LTC0_PKN_34 = crate::Reg<ltc0_ltc0_pkn_34::LTC0_LTC0_PKN_34_SPEC>;
#[doc = "LTC PKHA N 34 Register"]
pub mod ltc0_ltc0_pkn_34;
#[doc = "LTC0_LTC0_PKN2_3 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_3_SPEC>`"]
pub type LTC0_LTC0_PKN2_3 = crate::Reg<ltc0_ltc0_pkn2_3::LTC0_LTC0_PKN2_3_SPEC>;
#[doc = "LTC PKHA N2 3 Register"]
pub mod ltc0_ltc0_pkn2_3;
#[doc = "LTC0_LTC0_PKN_35 register accessor: an alias for `Reg<LTC0_LTC0_PKN_35_SPEC>`"]
pub type LTC0_LTC0_PKN_35 = crate::Reg<ltc0_ltc0_pkn_35::LTC0_LTC0_PKN_35_SPEC>;
#[doc = "LTC PKHA N 35 Register"]
pub mod ltc0_ltc0_pkn_35;
#[doc = "LTC0_LTC0_PKN2_4 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_4_SPEC>`"]
pub type LTC0_LTC0_PKN2_4 = crate::Reg<ltc0_ltc0_pkn2_4::LTC0_LTC0_PKN2_4_SPEC>;
#[doc = "LTC PKHA N2 4 Register"]
pub mod ltc0_ltc0_pkn2_4;
#[doc = "LTC0_LTC0_PKN_36 register accessor: an alias for `Reg<LTC0_LTC0_PKN_36_SPEC>`"]
pub type LTC0_LTC0_PKN_36 = crate::Reg<ltc0_ltc0_pkn_36::LTC0_LTC0_PKN_36_SPEC>;
#[doc = "LTC PKHA N 36 Register"]
pub mod ltc0_ltc0_pkn_36;
#[doc = "LTC0_LTC0_PKN2_5 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_5_SPEC>`"]
pub type LTC0_LTC0_PKN2_5 = crate::Reg<ltc0_ltc0_pkn2_5::LTC0_LTC0_PKN2_5_SPEC>;
#[doc = "LTC PKHA N2 5 Register"]
pub mod ltc0_ltc0_pkn2_5;
#[doc = "LTC0_LTC0_PKN_37 register accessor: an alias for `Reg<LTC0_LTC0_PKN_37_SPEC>`"]
pub type LTC0_LTC0_PKN_37 = crate::Reg<ltc0_ltc0_pkn_37::LTC0_LTC0_PKN_37_SPEC>;
#[doc = "LTC PKHA N 37 Register"]
pub mod ltc0_ltc0_pkn_37;
#[doc = "LTC0_LTC0_PKN2_6 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_6_SPEC>`"]
pub type LTC0_LTC0_PKN2_6 = crate::Reg<ltc0_ltc0_pkn2_6::LTC0_LTC0_PKN2_6_SPEC>;
#[doc = "LTC PKHA N2 6 Register"]
pub mod ltc0_ltc0_pkn2_6;
#[doc = "LTC0_LTC0_PKN_38 register accessor: an alias for `Reg<LTC0_LTC0_PKN_38_SPEC>`"]
pub type LTC0_LTC0_PKN_38 = crate::Reg<ltc0_ltc0_pkn_38::LTC0_LTC0_PKN_38_SPEC>;
#[doc = "LTC PKHA N 38 Register"]
pub mod ltc0_ltc0_pkn_38;
#[doc = "LTC0_LTC0_PKN2_7 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_7_SPEC>`"]
pub type LTC0_LTC0_PKN2_7 = crate::Reg<ltc0_ltc0_pkn2_7::LTC0_LTC0_PKN2_7_SPEC>;
#[doc = "LTC PKHA N2 7 Register"]
pub mod ltc0_ltc0_pkn2_7;
#[doc = "LTC0_LTC0_PKN_39 register accessor: an alias for `Reg<LTC0_LTC0_PKN_39_SPEC>`"]
pub type LTC0_LTC0_PKN_39 = crate::Reg<ltc0_ltc0_pkn_39::LTC0_LTC0_PKN_39_SPEC>;
#[doc = "LTC PKHA N 39 Register"]
pub mod ltc0_ltc0_pkn_39;
#[doc = "LTC0_LTC0_PKN2_8 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_8_SPEC>`"]
pub type LTC0_LTC0_PKN2_8 = crate::Reg<ltc0_ltc0_pkn2_8::LTC0_LTC0_PKN2_8_SPEC>;
#[doc = "LTC PKHA N2 8 Register"]
pub mod ltc0_ltc0_pkn2_8;
#[doc = "LTC0_LTC0_PKN_40 register accessor: an alias for `Reg<LTC0_LTC0_PKN_40_SPEC>`"]
pub type LTC0_LTC0_PKN_40 = crate::Reg<ltc0_ltc0_pkn_40::LTC0_LTC0_PKN_40_SPEC>;
#[doc = "LTC PKHA N 40 Register"]
pub mod ltc0_ltc0_pkn_40;
#[doc = "LTC0_LTC0_PKN2_9 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_9_SPEC>`"]
pub type LTC0_LTC0_PKN2_9 = crate::Reg<ltc0_ltc0_pkn2_9::LTC0_LTC0_PKN2_9_SPEC>;
#[doc = "LTC PKHA N2 9 Register"]
pub mod ltc0_ltc0_pkn2_9;
#[doc = "LTC0_LTC0_PKN_41 register accessor: an alias for `Reg<LTC0_LTC0_PKN_41_SPEC>`"]
pub type LTC0_LTC0_PKN_41 = crate::Reg<ltc0_ltc0_pkn_41::LTC0_LTC0_PKN_41_SPEC>;
#[doc = "LTC PKHA N 41 Register"]
pub mod ltc0_ltc0_pkn_41;
#[doc = "LTC0_LTC0_PKN2_10 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_10_SPEC>`"]
pub type LTC0_LTC0_PKN2_10 = crate::Reg<ltc0_ltc0_pkn2_10::LTC0_LTC0_PKN2_10_SPEC>;
#[doc = "LTC PKHA N2 10 Register"]
pub mod ltc0_ltc0_pkn2_10;
#[doc = "LTC0_LTC0_PKN_42 register accessor: an alias for `Reg<LTC0_LTC0_PKN_42_SPEC>`"]
pub type LTC0_LTC0_PKN_42 = crate::Reg<ltc0_ltc0_pkn_42::LTC0_LTC0_PKN_42_SPEC>;
#[doc = "LTC PKHA N 42 Register"]
pub mod ltc0_ltc0_pkn_42;
#[doc = "LTC0_LTC0_PKN2_11 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_11_SPEC>`"]
pub type LTC0_LTC0_PKN2_11 = crate::Reg<ltc0_ltc0_pkn2_11::LTC0_LTC0_PKN2_11_SPEC>;
#[doc = "LTC PKHA N2 11 Register"]
pub mod ltc0_ltc0_pkn2_11;
#[doc = "LTC0_LTC0_PKN_43 register accessor: an alias for `Reg<LTC0_LTC0_PKN_43_SPEC>`"]
pub type LTC0_LTC0_PKN_43 = crate::Reg<ltc0_ltc0_pkn_43::LTC0_LTC0_PKN_43_SPEC>;
#[doc = "LTC PKHA N 43 Register"]
pub mod ltc0_ltc0_pkn_43;
#[doc = "LTC0_LTC0_PKN2_12 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_12_SPEC>`"]
pub type LTC0_LTC0_PKN2_12 = crate::Reg<ltc0_ltc0_pkn2_12::LTC0_LTC0_PKN2_12_SPEC>;
#[doc = "LTC PKHA N2 12 Register"]
pub mod ltc0_ltc0_pkn2_12;
#[doc = "LTC0_LTC0_PKN_44 register accessor: an alias for `Reg<LTC0_LTC0_PKN_44_SPEC>`"]
pub type LTC0_LTC0_PKN_44 = crate::Reg<ltc0_ltc0_pkn_44::LTC0_LTC0_PKN_44_SPEC>;
#[doc = "LTC PKHA N 44 Register"]
pub mod ltc0_ltc0_pkn_44;
#[doc = "LTC0_LTC0_PKN2_13 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_13_SPEC>`"]
pub type LTC0_LTC0_PKN2_13 = crate::Reg<ltc0_ltc0_pkn2_13::LTC0_LTC0_PKN2_13_SPEC>;
#[doc = "LTC PKHA N2 13 Register"]
pub mod ltc0_ltc0_pkn2_13;
#[doc = "LTC0_LTC0_PKN_45 register accessor: an alias for `Reg<LTC0_LTC0_PKN_45_SPEC>`"]
pub type LTC0_LTC0_PKN_45 = crate::Reg<ltc0_ltc0_pkn_45::LTC0_LTC0_PKN_45_SPEC>;
#[doc = "LTC PKHA N 45 Register"]
pub mod ltc0_ltc0_pkn_45;
#[doc = "LTC0_LTC0_PKN2_14 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_14_SPEC>`"]
pub type LTC0_LTC0_PKN2_14 = crate::Reg<ltc0_ltc0_pkn2_14::LTC0_LTC0_PKN2_14_SPEC>;
#[doc = "LTC PKHA N2 14 Register"]
pub mod ltc0_ltc0_pkn2_14;
#[doc = "LTC0_LTC0_PKN_46 register accessor: an alias for `Reg<LTC0_LTC0_PKN_46_SPEC>`"]
pub type LTC0_LTC0_PKN_46 = crate::Reg<ltc0_ltc0_pkn_46::LTC0_LTC0_PKN_46_SPEC>;
#[doc = "LTC PKHA N 46 Register"]
pub mod ltc0_ltc0_pkn_46;
#[doc = "LTC0_LTC0_PKN2_15 register accessor: an alias for `Reg<LTC0_LTC0_PKN2_15_SPEC>`"]
pub type LTC0_LTC0_PKN2_15 = crate::Reg<ltc0_ltc0_pkn2_15::LTC0_LTC0_PKN2_15_SPEC>;
#[doc = "LTC PKHA N2 15 Register"]
pub mod ltc0_ltc0_pkn2_15;
#[doc = "LTC0_LTC0_PKN_47 register accessor: an alias for `Reg<LTC0_LTC0_PKN_47_SPEC>`"]
pub type LTC0_LTC0_PKN_47 = crate::Reg<ltc0_ltc0_pkn_47::LTC0_LTC0_PKN_47_SPEC>;
#[doc = "LTC PKHA N 47 Register"]
pub mod ltc0_ltc0_pkn_47;
#[doc = "LTC0_LTC0_PKN3_0 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_0_SPEC>`"]
pub type LTC0_LTC0_PKN3_0 = crate::Reg<ltc0_ltc0_pkn3_0::LTC0_LTC0_PKN3_0_SPEC>;
#[doc = "LTC PKHA N3 0 Register"]
pub mod ltc0_ltc0_pkn3_0;
#[doc = "LTC0_LTC0_PKN_48 register accessor: an alias for `Reg<LTC0_LTC0_PKN_48_SPEC>`"]
pub type LTC0_LTC0_PKN_48 = crate::Reg<ltc0_ltc0_pkn_48::LTC0_LTC0_PKN_48_SPEC>;
#[doc = "LTC PKHA N 48 Register"]
pub mod ltc0_ltc0_pkn_48;
#[doc = "LTC0_LTC0_PKN3_1 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_1_SPEC>`"]
pub type LTC0_LTC0_PKN3_1 = crate::Reg<ltc0_ltc0_pkn3_1::LTC0_LTC0_PKN3_1_SPEC>;
#[doc = "LTC PKHA N3 1 Register"]
pub mod ltc0_ltc0_pkn3_1;
#[doc = "LTC0_LTC0_PKN_49 register accessor: an alias for `Reg<LTC0_LTC0_PKN_49_SPEC>`"]
pub type LTC0_LTC0_PKN_49 = crate::Reg<ltc0_ltc0_pkn_49::LTC0_LTC0_PKN_49_SPEC>;
#[doc = "LTC PKHA N 49 Register"]
pub mod ltc0_ltc0_pkn_49;
#[doc = "LTC0_LTC0_PKN3_2 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_2_SPEC>`"]
pub type LTC0_LTC0_PKN3_2 = crate::Reg<ltc0_ltc0_pkn3_2::LTC0_LTC0_PKN3_2_SPEC>;
#[doc = "LTC PKHA N3 2 Register"]
pub mod ltc0_ltc0_pkn3_2;
#[doc = "LTC0_LTC0_PKN_50 register accessor: an alias for `Reg<LTC0_LTC0_PKN_50_SPEC>`"]
pub type LTC0_LTC0_PKN_50 = crate::Reg<ltc0_ltc0_pkn_50::LTC0_LTC0_PKN_50_SPEC>;
#[doc = "LTC PKHA N 50 Register"]
pub mod ltc0_ltc0_pkn_50;
#[doc = "LTC0_LTC0_PKN3_3 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_3_SPEC>`"]
pub type LTC0_LTC0_PKN3_3 = crate::Reg<ltc0_ltc0_pkn3_3::LTC0_LTC0_PKN3_3_SPEC>;
#[doc = "LTC PKHA N3 3 Register"]
pub mod ltc0_ltc0_pkn3_3;
#[doc = "LTC0_LTC0_PKN_51 register accessor: an alias for `Reg<LTC0_LTC0_PKN_51_SPEC>`"]
pub type LTC0_LTC0_PKN_51 = crate::Reg<ltc0_ltc0_pkn_51::LTC0_LTC0_PKN_51_SPEC>;
#[doc = "LTC PKHA N 51 Register"]
pub mod ltc0_ltc0_pkn_51;
#[doc = "LTC0_LTC0_PKN3_4 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_4_SPEC>`"]
pub type LTC0_LTC0_PKN3_4 = crate::Reg<ltc0_ltc0_pkn3_4::LTC0_LTC0_PKN3_4_SPEC>;
#[doc = "LTC PKHA N3 4 Register"]
pub mod ltc0_ltc0_pkn3_4;
#[doc = "LTC0_LTC0_PKN_52 register accessor: an alias for `Reg<LTC0_LTC0_PKN_52_SPEC>`"]
pub type LTC0_LTC0_PKN_52 = crate::Reg<ltc0_ltc0_pkn_52::LTC0_LTC0_PKN_52_SPEC>;
#[doc = "LTC PKHA N 52 Register"]
pub mod ltc0_ltc0_pkn_52;
#[doc = "LTC0_LTC0_PKN3_5 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_5_SPEC>`"]
pub type LTC0_LTC0_PKN3_5 = crate::Reg<ltc0_ltc0_pkn3_5::LTC0_LTC0_PKN3_5_SPEC>;
#[doc = "LTC PKHA N3 5 Register"]
pub mod ltc0_ltc0_pkn3_5;
#[doc = "LTC0_LTC0_PKN_53 register accessor: an alias for `Reg<LTC0_LTC0_PKN_53_SPEC>`"]
pub type LTC0_LTC0_PKN_53 = crate::Reg<ltc0_ltc0_pkn_53::LTC0_LTC0_PKN_53_SPEC>;
#[doc = "LTC PKHA N 53 Register"]
pub mod ltc0_ltc0_pkn_53;
#[doc = "LTC0_LTC0_PKN3_6 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_6_SPEC>`"]
pub type LTC0_LTC0_PKN3_6 = crate::Reg<ltc0_ltc0_pkn3_6::LTC0_LTC0_PKN3_6_SPEC>;
#[doc = "LTC PKHA N3 6 Register"]
pub mod ltc0_ltc0_pkn3_6;
#[doc = "LTC0_LTC0_PKN_54 register accessor: an alias for `Reg<LTC0_LTC0_PKN_54_SPEC>`"]
pub type LTC0_LTC0_PKN_54 = crate::Reg<ltc0_ltc0_pkn_54::LTC0_LTC0_PKN_54_SPEC>;
#[doc = "LTC PKHA N 54 Register"]
pub mod ltc0_ltc0_pkn_54;
#[doc = "LTC0_LTC0_PKN3_7 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_7_SPEC>`"]
pub type LTC0_LTC0_PKN3_7 = crate::Reg<ltc0_ltc0_pkn3_7::LTC0_LTC0_PKN3_7_SPEC>;
#[doc = "LTC PKHA N3 7 Register"]
pub mod ltc0_ltc0_pkn3_7;
#[doc = "LTC0_LTC0_PKN_55 register accessor: an alias for `Reg<LTC0_LTC0_PKN_55_SPEC>`"]
pub type LTC0_LTC0_PKN_55 = crate::Reg<ltc0_ltc0_pkn_55::LTC0_LTC0_PKN_55_SPEC>;
#[doc = "LTC PKHA N 55 Register"]
pub mod ltc0_ltc0_pkn_55;
#[doc = "LTC0_LTC0_PKN3_8 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_8_SPEC>`"]
pub type LTC0_LTC0_PKN3_8 = crate::Reg<ltc0_ltc0_pkn3_8::LTC0_LTC0_PKN3_8_SPEC>;
#[doc = "LTC PKHA N3 8 Register"]
pub mod ltc0_ltc0_pkn3_8;
#[doc = "LTC0_LTC0_PKN_56 register accessor: an alias for `Reg<LTC0_LTC0_PKN_56_SPEC>`"]
pub type LTC0_LTC0_PKN_56 = crate::Reg<ltc0_ltc0_pkn_56::LTC0_LTC0_PKN_56_SPEC>;
#[doc = "LTC PKHA N 56 Register"]
pub mod ltc0_ltc0_pkn_56;
#[doc = "LTC0_LTC0_PKN3_9 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_9_SPEC>`"]
pub type LTC0_LTC0_PKN3_9 = crate::Reg<ltc0_ltc0_pkn3_9::LTC0_LTC0_PKN3_9_SPEC>;
#[doc = "LTC PKHA N3 9 Register"]
pub mod ltc0_ltc0_pkn3_9;
#[doc = "LTC0_LTC0_PKN_57 register accessor: an alias for `Reg<LTC0_LTC0_PKN_57_SPEC>`"]
pub type LTC0_LTC0_PKN_57 = crate::Reg<ltc0_ltc0_pkn_57::LTC0_LTC0_PKN_57_SPEC>;
#[doc = "LTC PKHA N 57 Register"]
pub mod ltc0_ltc0_pkn_57;
#[doc = "LTC0_LTC0_PKN3_10 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_10_SPEC>`"]
pub type LTC0_LTC0_PKN3_10 = crate::Reg<ltc0_ltc0_pkn3_10::LTC0_LTC0_PKN3_10_SPEC>;
#[doc = "LTC PKHA N3 10 Register"]
pub mod ltc0_ltc0_pkn3_10;
#[doc = "LTC0_LTC0_PKN_58 register accessor: an alias for `Reg<LTC0_LTC0_PKN_58_SPEC>`"]
pub type LTC0_LTC0_PKN_58 = crate::Reg<ltc0_ltc0_pkn_58::LTC0_LTC0_PKN_58_SPEC>;
#[doc = "LTC PKHA N 58 Register"]
pub mod ltc0_ltc0_pkn_58;
#[doc = "LTC0_LTC0_PKN3_11 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_11_SPEC>`"]
pub type LTC0_LTC0_PKN3_11 = crate::Reg<ltc0_ltc0_pkn3_11::LTC0_LTC0_PKN3_11_SPEC>;
#[doc = "LTC PKHA N3 11 Register"]
pub mod ltc0_ltc0_pkn3_11;
#[doc = "LTC0_LTC0_PKN_59 register accessor: an alias for `Reg<LTC0_LTC0_PKN_59_SPEC>`"]
pub type LTC0_LTC0_PKN_59 = crate::Reg<ltc0_ltc0_pkn_59::LTC0_LTC0_PKN_59_SPEC>;
#[doc = "LTC PKHA N 59 Register"]
pub mod ltc0_ltc0_pkn_59;
#[doc = "LTC0_LTC0_PKN3_12 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_12_SPEC>`"]
pub type LTC0_LTC0_PKN3_12 = crate::Reg<ltc0_ltc0_pkn3_12::LTC0_LTC0_PKN3_12_SPEC>;
#[doc = "LTC PKHA N3 12 Register"]
pub mod ltc0_ltc0_pkn3_12;
#[doc = "LTC0_LTC0_PKN_60 register accessor: an alias for `Reg<LTC0_LTC0_PKN_60_SPEC>`"]
pub type LTC0_LTC0_PKN_60 = crate::Reg<ltc0_ltc0_pkn_60::LTC0_LTC0_PKN_60_SPEC>;
#[doc = "LTC PKHA N 60 Register"]
pub mod ltc0_ltc0_pkn_60;
#[doc = "LTC0_LTC0_PKN3_13 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_13_SPEC>`"]
pub type LTC0_LTC0_PKN3_13 = crate::Reg<ltc0_ltc0_pkn3_13::LTC0_LTC0_PKN3_13_SPEC>;
#[doc = "LTC PKHA N3 13 Register"]
pub mod ltc0_ltc0_pkn3_13;
#[doc = "LTC0_LTC0_PKN_61 register accessor: an alias for `Reg<LTC0_LTC0_PKN_61_SPEC>`"]
pub type LTC0_LTC0_PKN_61 = crate::Reg<ltc0_ltc0_pkn_61::LTC0_LTC0_PKN_61_SPEC>;
#[doc = "LTC PKHA N 61 Register"]
pub mod ltc0_ltc0_pkn_61;
#[doc = "LTC0_LTC0_PKN3_14 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_14_SPEC>`"]
pub type LTC0_LTC0_PKN3_14 = crate::Reg<ltc0_ltc0_pkn3_14::LTC0_LTC0_PKN3_14_SPEC>;
#[doc = "LTC PKHA N3 14 Register"]
pub mod ltc0_ltc0_pkn3_14;
#[doc = "LTC0_LTC0_PKN_62 register accessor: an alias for `Reg<LTC0_LTC0_PKN_62_SPEC>`"]
pub type LTC0_LTC0_PKN_62 = crate::Reg<ltc0_ltc0_pkn_62::LTC0_LTC0_PKN_62_SPEC>;
#[doc = "LTC PKHA N 62 Register"]
pub mod ltc0_ltc0_pkn_62;
#[doc = "LTC0_LTC0_PKN3_15 register accessor: an alias for `Reg<LTC0_LTC0_PKN3_15_SPEC>`"]
pub type LTC0_LTC0_PKN3_15 = crate::Reg<ltc0_ltc0_pkn3_15::LTC0_LTC0_PKN3_15_SPEC>;
#[doc = "LTC PKHA N3 15 Register"]
pub mod ltc0_ltc0_pkn3_15;
#[doc = "LTC0_LTC0_PKN_63 register accessor: an alias for `Reg<LTC0_LTC0_PKN_63_SPEC>`"]
pub type LTC0_LTC0_PKN_63 = crate::Reg<ltc0_ltc0_pkn_63::LTC0_LTC0_PKN_63_SPEC>;
#[doc = "LTC PKHA N 63 Register"]
pub mod ltc0_ltc0_pkn_63;
#[doc = "LTC0_LTC0_PKE0_0 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_0_SPEC>`"]
pub type LTC0_LTC0_PKE0_0 = crate::Reg<ltc0_ltc0_pke0_0::LTC0_LTC0_PKE0_0_SPEC>;
#[doc = "LTC PKHA E0 0 Register"]
pub mod ltc0_ltc0_pke0_0;
#[doc = "LTC0_LTC0_PKE_0 register accessor: an alias for `Reg<LTC0_LTC0_PKE_0_SPEC>`"]
pub type LTC0_LTC0_PKE_0 = crate::Reg<ltc0_ltc0_pke_0::LTC0_LTC0_PKE_0_SPEC>;
#[doc = "LTC PKHA E 0 Register"]
pub mod ltc0_ltc0_pke_0;
#[doc = "LTC0_LTC0_PKE0_1 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_1_SPEC>`"]
pub type LTC0_LTC0_PKE0_1 = crate::Reg<ltc0_ltc0_pke0_1::LTC0_LTC0_PKE0_1_SPEC>;
#[doc = "LTC PKHA E0 1 Register"]
pub mod ltc0_ltc0_pke0_1;
#[doc = "LTC0_LTC0_PKE_1 register accessor: an alias for `Reg<LTC0_LTC0_PKE_1_SPEC>`"]
pub type LTC0_LTC0_PKE_1 = crate::Reg<ltc0_ltc0_pke_1::LTC0_LTC0_PKE_1_SPEC>;
#[doc = "LTC PKHA E 1 Register"]
pub mod ltc0_ltc0_pke_1;
#[doc = "LTC0_LTC0_PKE0_2 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_2_SPEC>`"]
pub type LTC0_LTC0_PKE0_2 = crate::Reg<ltc0_ltc0_pke0_2::LTC0_LTC0_PKE0_2_SPEC>;
#[doc = "LTC PKHA E0 2 Register"]
pub mod ltc0_ltc0_pke0_2;
#[doc = "LTC0_LTC0_PKE_2 register accessor: an alias for `Reg<LTC0_LTC0_PKE_2_SPEC>`"]
pub type LTC0_LTC0_PKE_2 = crate::Reg<ltc0_ltc0_pke_2::LTC0_LTC0_PKE_2_SPEC>;
#[doc = "LTC PKHA E 2 Register"]
pub mod ltc0_ltc0_pke_2;
#[doc = "LTC0_LTC0_PKE0_3 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_3_SPEC>`"]
pub type LTC0_LTC0_PKE0_3 = crate::Reg<ltc0_ltc0_pke0_3::LTC0_LTC0_PKE0_3_SPEC>;
#[doc = "LTC PKHA E0 3 Register"]
pub mod ltc0_ltc0_pke0_3;
#[doc = "LTC0_LTC0_PKE_3 register accessor: an alias for `Reg<LTC0_LTC0_PKE_3_SPEC>`"]
pub type LTC0_LTC0_PKE_3 = crate::Reg<ltc0_ltc0_pke_3::LTC0_LTC0_PKE_3_SPEC>;
#[doc = "LTC PKHA E 3 Register"]
pub mod ltc0_ltc0_pke_3;
#[doc = "LTC0_LTC0_PKE0_4 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_4_SPEC>`"]
pub type LTC0_LTC0_PKE0_4 = crate::Reg<ltc0_ltc0_pke0_4::LTC0_LTC0_PKE0_4_SPEC>;
#[doc = "LTC PKHA E0 4 Register"]
pub mod ltc0_ltc0_pke0_4;
#[doc = "LTC0_LTC0_PKE_4 register accessor: an alias for `Reg<LTC0_LTC0_PKE_4_SPEC>`"]
pub type LTC0_LTC0_PKE_4 = crate::Reg<ltc0_ltc0_pke_4::LTC0_LTC0_PKE_4_SPEC>;
#[doc = "LTC PKHA E 4 Register"]
pub mod ltc0_ltc0_pke_4;
#[doc = "LTC0_LTC0_PKE0_5 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_5_SPEC>`"]
pub type LTC0_LTC0_PKE0_5 = crate::Reg<ltc0_ltc0_pke0_5::LTC0_LTC0_PKE0_5_SPEC>;
#[doc = "LTC PKHA E0 5 Register"]
pub mod ltc0_ltc0_pke0_5;
#[doc = "LTC0_LTC0_PKE_5 register accessor: an alias for `Reg<LTC0_LTC0_PKE_5_SPEC>`"]
pub type LTC0_LTC0_PKE_5 = crate::Reg<ltc0_ltc0_pke_5::LTC0_LTC0_PKE_5_SPEC>;
#[doc = "LTC PKHA E 5 Register"]
pub mod ltc0_ltc0_pke_5;
#[doc = "LTC0_LTC0_PKE0_6 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_6_SPEC>`"]
pub type LTC0_LTC0_PKE0_6 = crate::Reg<ltc0_ltc0_pke0_6::LTC0_LTC0_PKE0_6_SPEC>;
#[doc = "LTC PKHA E0 6 Register"]
pub mod ltc0_ltc0_pke0_6;
#[doc = "LTC0_LTC0_PKE_6 register accessor: an alias for `Reg<LTC0_LTC0_PKE_6_SPEC>`"]
pub type LTC0_LTC0_PKE_6 = crate::Reg<ltc0_ltc0_pke_6::LTC0_LTC0_PKE_6_SPEC>;
#[doc = "LTC PKHA E 6 Register"]
pub mod ltc0_ltc0_pke_6;
#[doc = "LTC0_LTC0_PKE0_7 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_7_SPEC>`"]
pub type LTC0_LTC0_PKE0_7 = crate::Reg<ltc0_ltc0_pke0_7::LTC0_LTC0_PKE0_7_SPEC>;
#[doc = "LTC PKHA E0 7 Register"]
pub mod ltc0_ltc0_pke0_7;
#[doc = "LTC0_LTC0_PKE_7 register accessor: an alias for `Reg<LTC0_LTC0_PKE_7_SPEC>`"]
pub type LTC0_LTC0_PKE_7 = crate::Reg<ltc0_ltc0_pke_7::LTC0_LTC0_PKE_7_SPEC>;
#[doc = "LTC PKHA E 7 Register"]
pub mod ltc0_ltc0_pke_7;
#[doc = "LTC0_LTC0_PKE0_8 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_8_SPEC>`"]
pub type LTC0_LTC0_PKE0_8 = crate::Reg<ltc0_ltc0_pke0_8::LTC0_LTC0_PKE0_8_SPEC>;
#[doc = "LTC PKHA E0 8 Register"]
pub mod ltc0_ltc0_pke0_8;
#[doc = "LTC0_LTC0_PKE_8 register accessor: an alias for `Reg<LTC0_LTC0_PKE_8_SPEC>`"]
pub type LTC0_LTC0_PKE_8 = crate::Reg<ltc0_ltc0_pke_8::LTC0_LTC0_PKE_8_SPEC>;
#[doc = "LTC PKHA E 8 Register"]
pub mod ltc0_ltc0_pke_8;
#[doc = "LTC0_LTC0_PKE0_9 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_9_SPEC>`"]
pub type LTC0_LTC0_PKE0_9 = crate::Reg<ltc0_ltc0_pke0_9::LTC0_LTC0_PKE0_9_SPEC>;
#[doc = "LTC PKHA E0 9 Register"]
pub mod ltc0_ltc0_pke0_9;
#[doc = "LTC0_LTC0_PKE_9 register accessor: an alias for `Reg<LTC0_LTC0_PKE_9_SPEC>`"]
pub type LTC0_LTC0_PKE_9 = crate::Reg<ltc0_ltc0_pke_9::LTC0_LTC0_PKE_9_SPEC>;
#[doc = "LTC PKHA E 9 Register"]
pub mod ltc0_ltc0_pke_9;
#[doc = "LTC0_LTC0_PKE0_10 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_10_SPEC>`"]
pub type LTC0_LTC0_PKE0_10 = crate::Reg<ltc0_ltc0_pke0_10::LTC0_LTC0_PKE0_10_SPEC>;
#[doc = "LTC PKHA E0 10 Register"]
pub mod ltc0_ltc0_pke0_10;
#[doc = "LTC0_LTC0_PKE_10 register accessor: an alias for `Reg<LTC0_LTC0_PKE_10_SPEC>`"]
pub type LTC0_LTC0_PKE_10 = crate::Reg<ltc0_ltc0_pke_10::LTC0_LTC0_PKE_10_SPEC>;
#[doc = "LTC PKHA E 10 Register"]
pub mod ltc0_ltc0_pke_10;
#[doc = "LTC0_LTC0_PKE0_11 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_11_SPEC>`"]
pub type LTC0_LTC0_PKE0_11 = crate::Reg<ltc0_ltc0_pke0_11::LTC0_LTC0_PKE0_11_SPEC>;
#[doc = "LTC PKHA E0 11 Register"]
pub mod ltc0_ltc0_pke0_11;
#[doc = "LTC0_LTC0_PKE_11 register accessor: an alias for `Reg<LTC0_LTC0_PKE_11_SPEC>`"]
pub type LTC0_LTC0_PKE_11 = crate::Reg<ltc0_ltc0_pke_11::LTC0_LTC0_PKE_11_SPEC>;
#[doc = "LTC PKHA E 11 Register"]
pub mod ltc0_ltc0_pke_11;
#[doc = "LTC0_LTC0_PKE0_12 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_12_SPEC>`"]
pub type LTC0_LTC0_PKE0_12 = crate::Reg<ltc0_ltc0_pke0_12::LTC0_LTC0_PKE0_12_SPEC>;
#[doc = "LTC PKHA E0 12 Register"]
pub mod ltc0_ltc0_pke0_12;
#[doc = "LTC0_LTC0_PKE_12 register accessor: an alias for `Reg<LTC0_LTC0_PKE_12_SPEC>`"]
pub type LTC0_LTC0_PKE_12 = crate::Reg<ltc0_ltc0_pke_12::LTC0_LTC0_PKE_12_SPEC>;
#[doc = "LTC PKHA E 12 Register"]
pub mod ltc0_ltc0_pke_12;
#[doc = "LTC0_LTC0_PKE0_13 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_13_SPEC>`"]
pub type LTC0_LTC0_PKE0_13 = crate::Reg<ltc0_ltc0_pke0_13::LTC0_LTC0_PKE0_13_SPEC>;
#[doc = "LTC PKHA E0 13 Register"]
pub mod ltc0_ltc0_pke0_13;
#[doc = "LTC0_LTC0_PKE_13 register accessor: an alias for `Reg<LTC0_LTC0_PKE_13_SPEC>`"]
pub type LTC0_LTC0_PKE_13 = crate::Reg<ltc0_ltc0_pke_13::LTC0_LTC0_PKE_13_SPEC>;
#[doc = "LTC PKHA E 13 Register"]
pub mod ltc0_ltc0_pke_13;
#[doc = "LTC0_LTC0_PKE0_14 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_14_SPEC>`"]
pub type LTC0_LTC0_PKE0_14 = crate::Reg<ltc0_ltc0_pke0_14::LTC0_LTC0_PKE0_14_SPEC>;
#[doc = "LTC PKHA E0 14 Register"]
pub mod ltc0_ltc0_pke0_14;
#[doc = "LTC0_LTC0_PKE_14 register accessor: an alias for `Reg<LTC0_LTC0_PKE_14_SPEC>`"]
pub type LTC0_LTC0_PKE_14 = crate::Reg<ltc0_ltc0_pke_14::LTC0_LTC0_PKE_14_SPEC>;
#[doc = "LTC PKHA E 14 Register"]
pub mod ltc0_ltc0_pke_14;
#[doc = "LTC0_LTC0_PKE0_15 register accessor: an alias for `Reg<LTC0_LTC0_PKE0_15_SPEC>`"]
pub type LTC0_LTC0_PKE0_15 = crate::Reg<ltc0_ltc0_pke0_15::LTC0_LTC0_PKE0_15_SPEC>;
#[doc = "LTC PKHA E0 15 Register"]
pub mod ltc0_ltc0_pke0_15;
#[doc = "LTC0_LTC0_PKE_15 register accessor: an alias for `Reg<LTC0_LTC0_PKE_15_SPEC>`"]
pub type LTC0_LTC0_PKE_15 = crate::Reg<ltc0_ltc0_pke_15::LTC0_LTC0_PKE_15_SPEC>;
#[doc = "LTC PKHA E 15 Register"]
pub mod ltc0_ltc0_pke_15;
#[doc = "LTC0_LTC0_PKE1_0 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_0_SPEC>`"]
pub type LTC0_LTC0_PKE1_0 = crate::Reg<ltc0_ltc0_pke1_0::LTC0_LTC0_PKE1_0_SPEC>;
#[doc = "LTC PKHA E1 0 Register"]
pub mod ltc0_ltc0_pke1_0;
#[doc = "LTC0_LTC0_PKE_16 register accessor: an alias for `Reg<LTC0_LTC0_PKE_16_SPEC>`"]
pub type LTC0_LTC0_PKE_16 = crate::Reg<ltc0_ltc0_pke_16::LTC0_LTC0_PKE_16_SPEC>;
#[doc = "LTC PKHA E 16 Register"]
pub mod ltc0_ltc0_pke_16;
#[doc = "LTC0_LTC0_PKE1_1 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_1_SPEC>`"]
pub type LTC0_LTC0_PKE1_1 = crate::Reg<ltc0_ltc0_pke1_1::LTC0_LTC0_PKE1_1_SPEC>;
#[doc = "LTC PKHA E1 1 Register"]
pub mod ltc0_ltc0_pke1_1;
#[doc = "LTC0_LTC0_PKE_17 register accessor: an alias for `Reg<LTC0_LTC0_PKE_17_SPEC>`"]
pub type LTC0_LTC0_PKE_17 = crate::Reg<ltc0_ltc0_pke_17::LTC0_LTC0_PKE_17_SPEC>;
#[doc = "LTC PKHA E 17 Register"]
pub mod ltc0_ltc0_pke_17;
#[doc = "LTC0_LTC0_PKE1_2 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_2_SPEC>`"]
pub type LTC0_LTC0_PKE1_2 = crate::Reg<ltc0_ltc0_pke1_2::LTC0_LTC0_PKE1_2_SPEC>;
#[doc = "LTC PKHA E1 2 Register"]
pub mod ltc0_ltc0_pke1_2;
#[doc = "LTC0_LTC0_PKE_18 register accessor: an alias for `Reg<LTC0_LTC0_PKE_18_SPEC>`"]
pub type LTC0_LTC0_PKE_18 = crate::Reg<ltc0_ltc0_pke_18::LTC0_LTC0_PKE_18_SPEC>;
#[doc = "LTC PKHA E 18 Register"]
pub mod ltc0_ltc0_pke_18;
#[doc = "LTC0_LTC0_PKE1_3 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_3_SPEC>`"]
pub type LTC0_LTC0_PKE1_3 = crate::Reg<ltc0_ltc0_pke1_3::LTC0_LTC0_PKE1_3_SPEC>;
#[doc = "LTC PKHA E1 3 Register"]
pub mod ltc0_ltc0_pke1_3;
#[doc = "LTC0_LTC0_PKE_19 register accessor: an alias for `Reg<LTC0_LTC0_PKE_19_SPEC>`"]
pub type LTC0_LTC0_PKE_19 = crate::Reg<ltc0_ltc0_pke_19::LTC0_LTC0_PKE_19_SPEC>;
#[doc = "LTC PKHA E 19 Register"]
pub mod ltc0_ltc0_pke_19;
#[doc = "LTC0_LTC0_PKE1_4 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_4_SPEC>`"]
pub type LTC0_LTC0_PKE1_4 = crate::Reg<ltc0_ltc0_pke1_4::LTC0_LTC0_PKE1_4_SPEC>;
#[doc = "LTC PKHA E1 4 Register"]
pub mod ltc0_ltc0_pke1_4;
#[doc = "LTC0_LTC0_PKE_20 register accessor: an alias for `Reg<LTC0_LTC0_PKE_20_SPEC>`"]
pub type LTC0_LTC0_PKE_20 = crate::Reg<ltc0_ltc0_pke_20::LTC0_LTC0_PKE_20_SPEC>;
#[doc = "LTC PKHA E 20 Register"]
pub mod ltc0_ltc0_pke_20;
#[doc = "LTC0_LTC0_PKE1_5 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_5_SPEC>`"]
pub type LTC0_LTC0_PKE1_5 = crate::Reg<ltc0_ltc0_pke1_5::LTC0_LTC0_PKE1_5_SPEC>;
#[doc = "LTC PKHA E1 5 Register"]
pub mod ltc0_ltc0_pke1_5;
#[doc = "LTC0_LTC0_PKE_21 register accessor: an alias for `Reg<LTC0_LTC0_PKE_21_SPEC>`"]
pub type LTC0_LTC0_PKE_21 = crate::Reg<ltc0_ltc0_pke_21::LTC0_LTC0_PKE_21_SPEC>;
#[doc = "LTC PKHA E 21 Register"]
pub mod ltc0_ltc0_pke_21;
#[doc = "LTC0_LTC0_PKE1_6 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_6_SPEC>`"]
pub type LTC0_LTC0_PKE1_6 = crate::Reg<ltc0_ltc0_pke1_6::LTC0_LTC0_PKE1_6_SPEC>;
#[doc = "LTC PKHA E1 6 Register"]
pub mod ltc0_ltc0_pke1_6;
#[doc = "LTC0_LTC0_PKE_22 register accessor: an alias for `Reg<LTC0_LTC0_PKE_22_SPEC>`"]
pub type LTC0_LTC0_PKE_22 = crate::Reg<ltc0_ltc0_pke_22::LTC0_LTC0_PKE_22_SPEC>;
#[doc = "LTC PKHA E 22 Register"]
pub mod ltc0_ltc0_pke_22;
#[doc = "LTC0_LTC0_PKE1_7 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_7_SPEC>`"]
pub type LTC0_LTC0_PKE1_7 = crate::Reg<ltc0_ltc0_pke1_7::LTC0_LTC0_PKE1_7_SPEC>;
#[doc = "LTC PKHA E1 7 Register"]
pub mod ltc0_ltc0_pke1_7;
#[doc = "LTC0_LTC0_PKE_23 register accessor: an alias for `Reg<LTC0_LTC0_PKE_23_SPEC>`"]
pub type LTC0_LTC0_PKE_23 = crate::Reg<ltc0_ltc0_pke_23::LTC0_LTC0_PKE_23_SPEC>;
#[doc = "LTC PKHA E 23 Register"]
pub mod ltc0_ltc0_pke_23;
#[doc = "LTC0_LTC0_PKE1_8 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_8_SPEC>`"]
pub type LTC0_LTC0_PKE1_8 = crate::Reg<ltc0_ltc0_pke1_8::LTC0_LTC0_PKE1_8_SPEC>;
#[doc = "LTC PKHA E1 8 Register"]
pub mod ltc0_ltc0_pke1_8;
#[doc = "LTC0_LTC0_PKE_24 register accessor: an alias for `Reg<LTC0_LTC0_PKE_24_SPEC>`"]
pub type LTC0_LTC0_PKE_24 = crate::Reg<ltc0_ltc0_pke_24::LTC0_LTC0_PKE_24_SPEC>;
#[doc = "LTC PKHA E 24 Register"]
pub mod ltc0_ltc0_pke_24;
#[doc = "LTC0_LTC0_PKE1_9 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_9_SPEC>`"]
pub type LTC0_LTC0_PKE1_9 = crate::Reg<ltc0_ltc0_pke1_9::LTC0_LTC0_PKE1_9_SPEC>;
#[doc = "LTC PKHA E1 9 Register"]
pub mod ltc0_ltc0_pke1_9;
#[doc = "LTC0_LTC0_PKE_25 register accessor: an alias for `Reg<LTC0_LTC0_PKE_25_SPEC>`"]
pub type LTC0_LTC0_PKE_25 = crate::Reg<ltc0_ltc0_pke_25::LTC0_LTC0_PKE_25_SPEC>;
#[doc = "LTC PKHA E 25 Register"]
pub mod ltc0_ltc0_pke_25;
#[doc = "LTC0_LTC0_PKE1_10 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_10_SPEC>`"]
pub type LTC0_LTC0_PKE1_10 = crate::Reg<ltc0_ltc0_pke1_10::LTC0_LTC0_PKE1_10_SPEC>;
#[doc = "LTC PKHA E1 10 Register"]
pub mod ltc0_ltc0_pke1_10;
#[doc = "LTC0_LTC0_PKE_26 register accessor: an alias for `Reg<LTC0_LTC0_PKE_26_SPEC>`"]
pub type LTC0_LTC0_PKE_26 = crate::Reg<ltc0_ltc0_pke_26::LTC0_LTC0_PKE_26_SPEC>;
#[doc = "LTC PKHA E 26 Register"]
pub mod ltc0_ltc0_pke_26;
#[doc = "LTC0_LTC0_PKE1_11 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_11_SPEC>`"]
pub type LTC0_LTC0_PKE1_11 = crate::Reg<ltc0_ltc0_pke1_11::LTC0_LTC0_PKE1_11_SPEC>;
#[doc = "LTC PKHA E1 11 Register"]
pub mod ltc0_ltc0_pke1_11;
#[doc = "LTC0_LTC0_PKE_27 register accessor: an alias for `Reg<LTC0_LTC0_PKE_27_SPEC>`"]
pub type LTC0_LTC0_PKE_27 = crate::Reg<ltc0_ltc0_pke_27::LTC0_LTC0_PKE_27_SPEC>;
#[doc = "LTC PKHA E 27 Register"]
pub mod ltc0_ltc0_pke_27;
#[doc = "LTC0_LTC0_PKE1_12 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_12_SPEC>`"]
pub type LTC0_LTC0_PKE1_12 = crate::Reg<ltc0_ltc0_pke1_12::LTC0_LTC0_PKE1_12_SPEC>;
#[doc = "LTC PKHA E1 12 Register"]
pub mod ltc0_ltc0_pke1_12;
#[doc = "LTC0_LTC0_PKE_28 register accessor: an alias for `Reg<LTC0_LTC0_PKE_28_SPEC>`"]
pub type LTC0_LTC0_PKE_28 = crate::Reg<ltc0_ltc0_pke_28::LTC0_LTC0_PKE_28_SPEC>;
#[doc = "LTC PKHA E 28 Register"]
pub mod ltc0_ltc0_pke_28;
#[doc = "LTC0_LTC0_PKE1_13 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_13_SPEC>`"]
pub type LTC0_LTC0_PKE1_13 = crate::Reg<ltc0_ltc0_pke1_13::LTC0_LTC0_PKE1_13_SPEC>;
#[doc = "LTC PKHA E1 13 Register"]
pub mod ltc0_ltc0_pke1_13;
#[doc = "LTC0_LTC0_PKE_29 register accessor: an alias for `Reg<LTC0_LTC0_PKE_29_SPEC>`"]
pub type LTC0_LTC0_PKE_29 = crate::Reg<ltc0_ltc0_pke_29::LTC0_LTC0_PKE_29_SPEC>;
#[doc = "LTC PKHA E 29 Register"]
pub mod ltc0_ltc0_pke_29;
#[doc = "LTC0_LTC0_PKE1_14 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_14_SPEC>`"]
pub type LTC0_LTC0_PKE1_14 = crate::Reg<ltc0_ltc0_pke1_14::LTC0_LTC0_PKE1_14_SPEC>;
#[doc = "LTC PKHA E1 14 Register"]
pub mod ltc0_ltc0_pke1_14;
#[doc = "LTC0_LTC0_PKE_30 register accessor: an alias for `Reg<LTC0_LTC0_PKE_30_SPEC>`"]
pub type LTC0_LTC0_PKE_30 = crate::Reg<ltc0_ltc0_pke_30::LTC0_LTC0_PKE_30_SPEC>;
#[doc = "LTC PKHA E 30 Register"]
pub mod ltc0_ltc0_pke_30;
#[doc = "LTC0_LTC0_PKE1_15 register accessor: an alias for `Reg<LTC0_LTC0_PKE1_15_SPEC>`"]
pub type LTC0_LTC0_PKE1_15 = crate::Reg<ltc0_ltc0_pke1_15::LTC0_LTC0_PKE1_15_SPEC>;
#[doc = "LTC PKHA E1 15 Register"]
pub mod ltc0_ltc0_pke1_15;
#[doc = "LTC0_LTC0_PKE_31 register accessor: an alias for `Reg<LTC0_LTC0_PKE_31_SPEC>`"]
pub type LTC0_LTC0_PKE_31 = crate::Reg<ltc0_ltc0_pke_31::LTC0_LTC0_PKE_31_SPEC>;
#[doc = "LTC PKHA E 31 Register"]
pub mod ltc0_ltc0_pke_31;
#[doc = "LTC0_LTC0_PKE2_0 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_0_SPEC>`"]
pub type LTC0_LTC0_PKE2_0 = crate::Reg<ltc0_ltc0_pke2_0::LTC0_LTC0_PKE2_0_SPEC>;
#[doc = "LTC PKHA E2 0 Register"]
pub mod ltc0_ltc0_pke2_0;
#[doc = "LTC0_LTC0_PKE_32 register accessor: an alias for `Reg<LTC0_LTC0_PKE_32_SPEC>`"]
pub type LTC0_LTC0_PKE_32 = crate::Reg<ltc0_ltc0_pke_32::LTC0_LTC0_PKE_32_SPEC>;
#[doc = "LTC PKHA E 32 Register"]
pub mod ltc0_ltc0_pke_32;
#[doc = "LTC0_LTC0_PKE2_1 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_1_SPEC>`"]
pub type LTC0_LTC0_PKE2_1 = crate::Reg<ltc0_ltc0_pke2_1::LTC0_LTC0_PKE2_1_SPEC>;
#[doc = "LTC PKHA E2 1 Register"]
pub mod ltc0_ltc0_pke2_1;
#[doc = "LTC0_LTC0_PKE_33 register accessor: an alias for `Reg<LTC0_LTC0_PKE_33_SPEC>`"]
pub type LTC0_LTC0_PKE_33 = crate::Reg<ltc0_ltc0_pke_33::LTC0_LTC0_PKE_33_SPEC>;
#[doc = "LTC PKHA E 33 Register"]
pub mod ltc0_ltc0_pke_33;
#[doc = "LTC0_LTC0_PKE2_2 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_2_SPEC>`"]
pub type LTC0_LTC0_PKE2_2 = crate::Reg<ltc0_ltc0_pke2_2::LTC0_LTC0_PKE2_2_SPEC>;
#[doc = "LTC PKHA E2 2 Register"]
pub mod ltc0_ltc0_pke2_2;
#[doc = "LTC0_LTC0_PKE_34 register accessor: an alias for `Reg<LTC0_LTC0_PKE_34_SPEC>`"]
pub type LTC0_LTC0_PKE_34 = crate::Reg<ltc0_ltc0_pke_34::LTC0_LTC0_PKE_34_SPEC>;
#[doc = "LTC PKHA E 34 Register"]
pub mod ltc0_ltc0_pke_34;
#[doc = "LTC0_LTC0_PKE2_3 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_3_SPEC>`"]
pub type LTC0_LTC0_PKE2_3 = crate::Reg<ltc0_ltc0_pke2_3::LTC0_LTC0_PKE2_3_SPEC>;
#[doc = "LTC PKHA E2 3 Register"]
pub mod ltc0_ltc0_pke2_3;
#[doc = "LTC0_LTC0_PKE_35 register accessor: an alias for `Reg<LTC0_LTC0_PKE_35_SPEC>`"]
pub type LTC0_LTC0_PKE_35 = crate::Reg<ltc0_ltc0_pke_35::LTC0_LTC0_PKE_35_SPEC>;
#[doc = "LTC PKHA E 35 Register"]
pub mod ltc0_ltc0_pke_35;
#[doc = "LTC0_LTC0_PKE2_4 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_4_SPEC>`"]
pub type LTC0_LTC0_PKE2_4 = crate::Reg<ltc0_ltc0_pke2_4::LTC0_LTC0_PKE2_4_SPEC>;
#[doc = "LTC PKHA E2 4 Register"]
pub mod ltc0_ltc0_pke2_4;
#[doc = "LTC0_LTC0_PKE_36 register accessor: an alias for `Reg<LTC0_LTC0_PKE_36_SPEC>`"]
pub type LTC0_LTC0_PKE_36 = crate::Reg<ltc0_ltc0_pke_36::LTC0_LTC0_PKE_36_SPEC>;
#[doc = "LTC PKHA E 36 Register"]
pub mod ltc0_ltc0_pke_36;
#[doc = "LTC0_LTC0_PKE2_5 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_5_SPEC>`"]
pub type LTC0_LTC0_PKE2_5 = crate::Reg<ltc0_ltc0_pke2_5::LTC0_LTC0_PKE2_5_SPEC>;
#[doc = "LTC PKHA E2 5 Register"]
pub mod ltc0_ltc0_pke2_5;
#[doc = "LTC0_LTC0_PKE_37 register accessor: an alias for `Reg<LTC0_LTC0_PKE_37_SPEC>`"]
pub type LTC0_LTC0_PKE_37 = crate::Reg<ltc0_ltc0_pke_37::LTC0_LTC0_PKE_37_SPEC>;
#[doc = "LTC PKHA E 37 Register"]
pub mod ltc0_ltc0_pke_37;
#[doc = "LTC0_LTC0_PKE2_6 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_6_SPEC>`"]
pub type LTC0_LTC0_PKE2_6 = crate::Reg<ltc0_ltc0_pke2_6::LTC0_LTC0_PKE2_6_SPEC>;
#[doc = "LTC PKHA E2 6 Register"]
pub mod ltc0_ltc0_pke2_6;
#[doc = "LTC0_LTC0_PKE_38 register accessor: an alias for `Reg<LTC0_LTC0_PKE_38_SPEC>`"]
pub type LTC0_LTC0_PKE_38 = crate::Reg<ltc0_ltc0_pke_38::LTC0_LTC0_PKE_38_SPEC>;
#[doc = "LTC PKHA E 38 Register"]
pub mod ltc0_ltc0_pke_38;
#[doc = "LTC0_LTC0_PKE2_7 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_7_SPEC>`"]
pub type LTC0_LTC0_PKE2_7 = crate::Reg<ltc0_ltc0_pke2_7::LTC0_LTC0_PKE2_7_SPEC>;
#[doc = "LTC PKHA E2 7 Register"]
pub mod ltc0_ltc0_pke2_7;
#[doc = "LTC0_LTC0_PKE_39 register accessor: an alias for `Reg<LTC0_LTC0_PKE_39_SPEC>`"]
pub type LTC0_LTC0_PKE_39 = crate::Reg<ltc0_ltc0_pke_39::LTC0_LTC0_PKE_39_SPEC>;
#[doc = "LTC PKHA E 39 Register"]
pub mod ltc0_ltc0_pke_39;
#[doc = "LTC0_LTC0_PKE2_8 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_8_SPEC>`"]
pub type LTC0_LTC0_PKE2_8 = crate::Reg<ltc0_ltc0_pke2_8::LTC0_LTC0_PKE2_8_SPEC>;
#[doc = "LTC PKHA E2 8 Register"]
pub mod ltc0_ltc0_pke2_8;
#[doc = "LTC0_LTC0_PKE_40 register accessor: an alias for `Reg<LTC0_LTC0_PKE_40_SPEC>`"]
pub type LTC0_LTC0_PKE_40 = crate::Reg<ltc0_ltc0_pke_40::LTC0_LTC0_PKE_40_SPEC>;
#[doc = "LTC PKHA E 40 Register"]
pub mod ltc0_ltc0_pke_40;
#[doc = "LTC0_LTC0_PKE2_9 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_9_SPEC>`"]
pub type LTC0_LTC0_PKE2_9 = crate::Reg<ltc0_ltc0_pke2_9::LTC0_LTC0_PKE2_9_SPEC>;
#[doc = "LTC PKHA E2 9 Register"]
pub mod ltc0_ltc0_pke2_9;
#[doc = "LTC0_LTC0_PKE_41 register accessor: an alias for `Reg<LTC0_LTC0_PKE_41_SPEC>`"]
pub type LTC0_LTC0_PKE_41 = crate::Reg<ltc0_ltc0_pke_41::LTC0_LTC0_PKE_41_SPEC>;
#[doc = "LTC PKHA E 41 Register"]
pub mod ltc0_ltc0_pke_41;
#[doc = "LTC0_LTC0_PKE2_10 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_10_SPEC>`"]
pub type LTC0_LTC0_PKE2_10 = crate::Reg<ltc0_ltc0_pke2_10::LTC0_LTC0_PKE2_10_SPEC>;
#[doc = "LTC PKHA E2 10 Register"]
pub mod ltc0_ltc0_pke2_10;
#[doc = "LTC0_LTC0_PKE_42 register accessor: an alias for `Reg<LTC0_LTC0_PKE_42_SPEC>`"]
pub type LTC0_LTC0_PKE_42 = crate::Reg<ltc0_ltc0_pke_42::LTC0_LTC0_PKE_42_SPEC>;
#[doc = "LTC PKHA E 42 Register"]
pub mod ltc0_ltc0_pke_42;
#[doc = "LTC0_LTC0_PKE2_11 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_11_SPEC>`"]
pub type LTC0_LTC0_PKE2_11 = crate::Reg<ltc0_ltc0_pke2_11::LTC0_LTC0_PKE2_11_SPEC>;
#[doc = "LTC PKHA E2 11 Register"]
pub mod ltc0_ltc0_pke2_11;
#[doc = "LTC0_LTC0_PKE_43 register accessor: an alias for `Reg<LTC0_LTC0_PKE_43_SPEC>`"]
pub type LTC0_LTC0_PKE_43 = crate::Reg<ltc0_ltc0_pke_43::LTC0_LTC0_PKE_43_SPEC>;
#[doc = "LTC PKHA E 43 Register"]
pub mod ltc0_ltc0_pke_43;
#[doc = "LTC0_LTC0_PKE2_12 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_12_SPEC>`"]
pub type LTC0_LTC0_PKE2_12 = crate::Reg<ltc0_ltc0_pke2_12::LTC0_LTC0_PKE2_12_SPEC>;
#[doc = "LTC PKHA E2 12 Register"]
pub mod ltc0_ltc0_pke2_12;
#[doc = "LTC0_LTC0_PKE_44 register accessor: an alias for `Reg<LTC0_LTC0_PKE_44_SPEC>`"]
pub type LTC0_LTC0_PKE_44 = crate::Reg<ltc0_ltc0_pke_44::LTC0_LTC0_PKE_44_SPEC>;
#[doc = "LTC PKHA E 44 Register"]
pub mod ltc0_ltc0_pke_44;
#[doc = "LTC0_LTC0_PKE2_13 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_13_SPEC>`"]
pub type LTC0_LTC0_PKE2_13 = crate::Reg<ltc0_ltc0_pke2_13::LTC0_LTC0_PKE2_13_SPEC>;
#[doc = "LTC PKHA E2 13 Register"]
pub mod ltc0_ltc0_pke2_13;
#[doc = "LTC0_LTC0_PKE_45 register accessor: an alias for `Reg<LTC0_LTC0_PKE_45_SPEC>`"]
pub type LTC0_LTC0_PKE_45 = crate::Reg<ltc0_ltc0_pke_45::LTC0_LTC0_PKE_45_SPEC>;
#[doc = "LTC PKHA E 45 Register"]
pub mod ltc0_ltc0_pke_45;
#[doc = "LTC0_LTC0_PKE2_14 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_14_SPEC>`"]
pub type LTC0_LTC0_PKE2_14 = crate::Reg<ltc0_ltc0_pke2_14::LTC0_LTC0_PKE2_14_SPEC>;
#[doc = "LTC PKHA E2 14 Register"]
pub mod ltc0_ltc0_pke2_14;
#[doc = "LTC0_LTC0_PKE_46 register accessor: an alias for `Reg<LTC0_LTC0_PKE_46_SPEC>`"]
pub type LTC0_LTC0_PKE_46 = crate::Reg<ltc0_ltc0_pke_46::LTC0_LTC0_PKE_46_SPEC>;
#[doc = "LTC PKHA E 46 Register"]
pub mod ltc0_ltc0_pke_46;
#[doc = "LTC0_LTC0_PKE2_15 register accessor: an alias for `Reg<LTC0_LTC0_PKE2_15_SPEC>`"]
pub type LTC0_LTC0_PKE2_15 = crate::Reg<ltc0_ltc0_pke2_15::LTC0_LTC0_PKE2_15_SPEC>;
#[doc = "LTC PKHA E2 15 Register"]
pub mod ltc0_ltc0_pke2_15;
#[doc = "LTC0_LTC0_PKE_47 register accessor: an alias for `Reg<LTC0_LTC0_PKE_47_SPEC>`"]
pub type LTC0_LTC0_PKE_47 = crate::Reg<ltc0_ltc0_pke_47::LTC0_LTC0_PKE_47_SPEC>;
#[doc = "LTC PKHA E 47 Register"]
pub mod ltc0_ltc0_pke_47;
#[doc = "LTC0_LTC0_PKE3_0 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_0_SPEC>`"]
pub type LTC0_LTC0_PKE3_0 = crate::Reg<ltc0_ltc0_pke3_0::LTC0_LTC0_PKE3_0_SPEC>;
#[doc = "LTC PKHA E3 0 Register"]
pub mod ltc0_ltc0_pke3_0;
#[doc = "LTC0_LTC0_PKE_48 register accessor: an alias for `Reg<LTC0_LTC0_PKE_48_SPEC>`"]
pub type LTC0_LTC0_PKE_48 = crate::Reg<ltc0_ltc0_pke_48::LTC0_LTC0_PKE_48_SPEC>;
#[doc = "LTC PKHA E 48 Register"]
pub mod ltc0_ltc0_pke_48;
#[doc = "LTC0_LTC0_PKE3_1 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_1_SPEC>`"]
pub type LTC0_LTC0_PKE3_1 = crate::Reg<ltc0_ltc0_pke3_1::LTC0_LTC0_PKE3_1_SPEC>;
#[doc = "LTC PKHA E3 1 Register"]
pub mod ltc0_ltc0_pke3_1;
#[doc = "LTC0_LTC0_PKE_49 register accessor: an alias for `Reg<LTC0_LTC0_PKE_49_SPEC>`"]
pub type LTC0_LTC0_PKE_49 = crate::Reg<ltc0_ltc0_pke_49::LTC0_LTC0_PKE_49_SPEC>;
#[doc = "LTC PKHA E 49 Register"]
pub mod ltc0_ltc0_pke_49;
#[doc = "LTC0_LTC0_PKE3_2 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_2_SPEC>`"]
pub type LTC0_LTC0_PKE3_2 = crate::Reg<ltc0_ltc0_pke3_2::LTC0_LTC0_PKE3_2_SPEC>;
#[doc = "LTC PKHA E3 2 Register"]
pub mod ltc0_ltc0_pke3_2;
#[doc = "LTC0_LTC0_PKE_50 register accessor: an alias for `Reg<LTC0_LTC0_PKE_50_SPEC>`"]
pub type LTC0_LTC0_PKE_50 = crate::Reg<ltc0_ltc0_pke_50::LTC0_LTC0_PKE_50_SPEC>;
#[doc = "LTC PKHA E 50 Register"]
pub mod ltc0_ltc0_pke_50;
#[doc = "LTC0_LTC0_PKE3_3 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_3_SPEC>`"]
pub type LTC0_LTC0_PKE3_3 = crate::Reg<ltc0_ltc0_pke3_3::LTC0_LTC0_PKE3_3_SPEC>;
#[doc = "LTC PKHA E3 3 Register"]
pub mod ltc0_ltc0_pke3_3;
#[doc = "LTC0_LTC0_PKE_51 register accessor: an alias for `Reg<LTC0_LTC0_PKE_51_SPEC>`"]
pub type LTC0_LTC0_PKE_51 = crate::Reg<ltc0_ltc0_pke_51::LTC0_LTC0_PKE_51_SPEC>;
#[doc = "LTC PKHA E 51 Register"]
pub mod ltc0_ltc0_pke_51;
#[doc = "LTC0_LTC0_PKE3_4 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_4_SPEC>`"]
pub type LTC0_LTC0_PKE3_4 = crate::Reg<ltc0_ltc0_pke3_4::LTC0_LTC0_PKE3_4_SPEC>;
#[doc = "LTC PKHA E3 4 Register"]
pub mod ltc0_ltc0_pke3_4;
#[doc = "LTC0_LTC0_PKE_52 register accessor: an alias for `Reg<LTC0_LTC0_PKE_52_SPEC>`"]
pub type LTC0_LTC0_PKE_52 = crate::Reg<ltc0_ltc0_pke_52::LTC0_LTC0_PKE_52_SPEC>;
#[doc = "LTC PKHA E 52 Register"]
pub mod ltc0_ltc0_pke_52;
#[doc = "LTC0_LTC0_PKE3_5 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_5_SPEC>`"]
pub type LTC0_LTC0_PKE3_5 = crate::Reg<ltc0_ltc0_pke3_5::LTC0_LTC0_PKE3_5_SPEC>;
#[doc = "LTC PKHA E3 5 Register"]
pub mod ltc0_ltc0_pke3_5;
#[doc = "LTC0_LTC0_PKE_53 register accessor: an alias for `Reg<LTC0_LTC0_PKE_53_SPEC>`"]
pub type LTC0_LTC0_PKE_53 = crate::Reg<ltc0_ltc0_pke_53::LTC0_LTC0_PKE_53_SPEC>;
#[doc = "LTC PKHA E 53 Register"]
pub mod ltc0_ltc0_pke_53;
#[doc = "LTC0_LTC0_PKE3_6 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_6_SPEC>`"]
pub type LTC0_LTC0_PKE3_6 = crate::Reg<ltc0_ltc0_pke3_6::LTC0_LTC0_PKE3_6_SPEC>;
#[doc = "LTC PKHA E3 6 Register"]
pub mod ltc0_ltc0_pke3_6;
#[doc = "LTC0_LTC0_PKE_54 register accessor: an alias for `Reg<LTC0_LTC0_PKE_54_SPEC>`"]
pub type LTC0_LTC0_PKE_54 = crate::Reg<ltc0_ltc0_pke_54::LTC0_LTC0_PKE_54_SPEC>;
#[doc = "LTC PKHA E 54 Register"]
pub mod ltc0_ltc0_pke_54;
#[doc = "LTC0_LTC0_PKE3_7 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_7_SPEC>`"]
pub type LTC0_LTC0_PKE3_7 = crate::Reg<ltc0_ltc0_pke3_7::LTC0_LTC0_PKE3_7_SPEC>;
#[doc = "LTC PKHA E3 7 Register"]
pub mod ltc0_ltc0_pke3_7;
#[doc = "LTC0_LTC0_PKE_55 register accessor: an alias for `Reg<LTC0_LTC0_PKE_55_SPEC>`"]
pub type LTC0_LTC0_PKE_55 = crate::Reg<ltc0_ltc0_pke_55::LTC0_LTC0_PKE_55_SPEC>;
#[doc = "LTC PKHA E 55 Register"]
pub mod ltc0_ltc0_pke_55;
#[doc = "LTC0_LTC0_PKE3_8 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_8_SPEC>`"]
pub type LTC0_LTC0_PKE3_8 = crate::Reg<ltc0_ltc0_pke3_8::LTC0_LTC0_PKE3_8_SPEC>;
#[doc = "LTC PKHA E3 8 Register"]
pub mod ltc0_ltc0_pke3_8;
#[doc = "LTC0_LTC0_PKE_56 register accessor: an alias for `Reg<LTC0_LTC0_PKE_56_SPEC>`"]
pub type LTC0_LTC0_PKE_56 = crate::Reg<ltc0_ltc0_pke_56::LTC0_LTC0_PKE_56_SPEC>;
#[doc = "LTC PKHA E 56 Register"]
pub mod ltc0_ltc0_pke_56;
#[doc = "LTC0_LTC0_PKE3_9 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_9_SPEC>`"]
pub type LTC0_LTC0_PKE3_9 = crate::Reg<ltc0_ltc0_pke3_9::LTC0_LTC0_PKE3_9_SPEC>;
#[doc = "LTC PKHA E3 9 Register"]
pub mod ltc0_ltc0_pke3_9;
#[doc = "LTC0_LTC0_PKE_57 register accessor: an alias for `Reg<LTC0_LTC0_PKE_57_SPEC>`"]
pub type LTC0_LTC0_PKE_57 = crate::Reg<ltc0_ltc0_pke_57::LTC0_LTC0_PKE_57_SPEC>;
#[doc = "LTC PKHA E 57 Register"]
pub mod ltc0_ltc0_pke_57;
#[doc = "LTC0_LTC0_PKE3_10 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_10_SPEC>`"]
pub type LTC0_LTC0_PKE3_10 = crate::Reg<ltc0_ltc0_pke3_10::LTC0_LTC0_PKE3_10_SPEC>;
#[doc = "LTC PKHA E3 10 Register"]
pub mod ltc0_ltc0_pke3_10;
#[doc = "LTC0_LTC0_PKE_58 register accessor: an alias for `Reg<LTC0_LTC0_PKE_58_SPEC>`"]
pub type LTC0_LTC0_PKE_58 = crate::Reg<ltc0_ltc0_pke_58::LTC0_LTC0_PKE_58_SPEC>;
#[doc = "LTC PKHA E 58 Register"]
pub mod ltc0_ltc0_pke_58;
#[doc = "LTC0_LTC0_PKE3_11 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_11_SPEC>`"]
pub type LTC0_LTC0_PKE3_11 = crate::Reg<ltc0_ltc0_pke3_11::LTC0_LTC0_PKE3_11_SPEC>;
#[doc = "LTC PKHA E3 11 Register"]
pub mod ltc0_ltc0_pke3_11;
#[doc = "LTC0_LTC0_PKE_59 register accessor: an alias for `Reg<LTC0_LTC0_PKE_59_SPEC>`"]
pub type LTC0_LTC0_PKE_59 = crate::Reg<ltc0_ltc0_pke_59::LTC0_LTC0_PKE_59_SPEC>;
#[doc = "LTC PKHA E 59 Register"]
pub mod ltc0_ltc0_pke_59;
#[doc = "LTC0_LTC0_PKE3_12 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_12_SPEC>`"]
pub type LTC0_LTC0_PKE3_12 = crate::Reg<ltc0_ltc0_pke3_12::LTC0_LTC0_PKE3_12_SPEC>;
#[doc = "LTC PKHA E3 12 Register"]
pub mod ltc0_ltc0_pke3_12;
#[doc = "LTC0_LTC0_PKE_60 register accessor: an alias for `Reg<LTC0_LTC0_PKE_60_SPEC>`"]
pub type LTC0_LTC0_PKE_60 = crate::Reg<ltc0_ltc0_pke_60::LTC0_LTC0_PKE_60_SPEC>;
#[doc = "LTC PKHA E 60 Register"]
pub mod ltc0_ltc0_pke_60;
#[doc = "LTC0_LTC0_PKE3_13 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_13_SPEC>`"]
pub type LTC0_LTC0_PKE3_13 = crate::Reg<ltc0_ltc0_pke3_13::LTC0_LTC0_PKE3_13_SPEC>;
#[doc = "LTC PKHA E3 13 Register"]
pub mod ltc0_ltc0_pke3_13;
#[doc = "LTC0_LTC0_PKE_61 register accessor: an alias for `Reg<LTC0_LTC0_PKE_61_SPEC>`"]
pub type LTC0_LTC0_PKE_61 = crate::Reg<ltc0_ltc0_pke_61::LTC0_LTC0_PKE_61_SPEC>;
#[doc = "LTC PKHA E 61 Register"]
pub mod ltc0_ltc0_pke_61;
#[doc = "LTC0_LTC0_PKE3_14 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_14_SPEC>`"]
pub type LTC0_LTC0_PKE3_14 = crate::Reg<ltc0_ltc0_pke3_14::LTC0_LTC0_PKE3_14_SPEC>;
#[doc = "LTC PKHA E3 14 Register"]
pub mod ltc0_ltc0_pke3_14;
#[doc = "LTC0_LTC0_PKE_62 register accessor: an alias for `Reg<LTC0_LTC0_PKE_62_SPEC>`"]
pub type LTC0_LTC0_PKE_62 = crate::Reg<ltc0_ltc0_pke_62::LTC0_LTC0_PKE_62_SPEC>;
#[doc = "LTC PKHA E 62 Register"]
pub mod ltc0_ltc0_pke_62;
#[doc = "LTC0_LTC0_PKE3_15 register accessor: an alias for `Reg<LTC0_LTC0_PKE3_15_SPEC>`"]
pub type LTC0_LTC0_PKE3_15 = crate::Reg<ltc0_ltc0_pke3_15::LTC0_LTC0_PKE3_15_SPEC>;
#[doc = "LTC PKHA E3 15 Register"]
pub mod ltc0_ltc0_pke3_15;
#[doc = "LTC0_LTC0_PKE_63 register accessor: an alias for `Reg<LTC0_LTC0_PKE_63_SPEC>`"]
pub type LTC0_LTC0_PKE_63 = crate::Reg<ltc0_ltc0_pke_63::LTC0_LTC0_PKE_63_SPEC>;
#[doc = "LTC PKHA E 63 Register"]
pub mod ltc0_ltc0_pke_63;
