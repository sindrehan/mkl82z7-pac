#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ltc0: [u8; 4usize],
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
    _reserved_46_ltc0_: [u8; 4usize],
    _reserved_47_ltc0_: [u8; 4usize],
    _reserved_48_ltc0_: [u8; 4usize],
    _reserved_49_ltc0_: [u8; 4usize],
    _reserved_50_ltc0_: [u8; 4usize],
    _reserved_51_ltc0_: [u8; 4usize],
    _reserved_52_ltc0_: [u8; 4usize],
    _reserved_53_ltc0_: [u8; 4usize],
    _reserved_54_ltc0_: [u8; 4usize],
    _reserved_55_ltc0_: [u8; 4usize],
    _reserved_56_ltc0_: [u8; 4usize],
    _reserved_57_ltc0_: [u8; 4usize],
    _reserved_58_ltc0_: [u8; 4usize],
    _reserved_59_ltc0_: [u8; 4usize],
    _reserved_60_ltc0_: [u8; 4usize],
    _reserved_61_ltc0_: [u8; 4usize],
    _reserved_62_ltc0_: [u8; 4usize],
    _reserved_63_ltc0_: [u8; 4usize],
    _reserved_64_ltc0_: [u8; 4usize],
    _reserved_65_ltc0_: [u8; 4usize],
    _reserved_66_ltc0_: [u8; 4usize],
    _reserved_67_ltc0_: [u8; 4usize],
    _reserved_68_ltc0_: [u8; 4usize],
    _reserved_69_ltc0_: [u8; 4usize],
    _reserved_70_ltc0_: [u8; 4usize],
    _reserved_71_ltc0_: [u8; 4usize],
    _reserved_72_ltc0_: [u8; 4usize],
    _reserved_73_ltc0_: [u8; 4usize],
    _reserved_74_ltc0_: [u8; 4usize],
    _reserved_75_ltc0_: [u8; 4usize],
    _reserved_76_ltc0_: [u8; 4usize],
    _reserved_77_ltc0_: [u8; 4usize],
    _reserved_78_ltc0_: [u8; 4usize],
    _reserved_79_ltc0_: [u8; 4usize],
    _reserved_80_ltc0_: [u8; 4usize],
    _reserved_81_ltc0_: [u8; 4usize],
    _reserved_82_ltc0_: [u8; 4usize],
    _reserved_83_ltc0_: [u8; 4usize],
    _reserved_84_ltc0_: [u8; 4usize],
    _reserved_85_ltc0_: [u8; 4usize],
    _reserved_86_ltc0_: [u8; 4usize],
    _reserved_87_ltc0_: [u8; 4usize],
    _reserved_88_ltc0_: [u8; 4usize],
    _reserved_89_ltc0_: [u8; 4usize],
    _reserved_90_ltc0_: [u8; 4usize],
    _reserved_91_ltc0_: [u8; 4usize],
    _reserved_92_ltc0_: [u8; 4usize],
    _reserved_93_ltc0_: [u8; 4usize],
    _reserved_94_ltc0_: [u8; 4usize],
    _reserved_95_ltc0_: [u8; 4usize],
    _reserved_96_ltc0_: [u8; 4usize],
    _reserved_97_ltc0_: [u8; 4usize],
    _reserved_98_ltc0_: [u8; 4usize],
    _reserved_99_ltc0_: [u8; 4usize],
    _reserved_100_ltc0_: [u8; 4usize],
    _reserved_101_ltc0_: [u8; 4usize],
    _reserved_102_ltc0_: [u8; 4usize],
    _reserved_103_ltc0_: [u8; 4usize],
    _reserved_104_ltc0_: [u8; 4usize],
    _reserved_105_ltc0_: [u8; 4usize],
    _reserved_106_ltc0_: [u8; 4usize],
    _reserved_107_ltc0_: [u8; 4usize],
    _reserved_108_ltc0_: [u8; 4usize],
    _reserved_109_ltc0_: [u8; 4usize],
    _reserved110: [u8; 256usize],
    _reserved_110_ltc0_: [u8; 4usize],
    _reserved_111_ltc0_: [u8; 4usize],
    _reserved_112_ltc0_: [u8; 4usize],
    _reserved_113_ltc0_: [u8; 4usize],
    _reserved_114_ltc0_: [u8; 4usize],
    _reserved_115_ltc0_: [u8; 4usize],
    _reserved_116_ltc0_: [u8; 4usize],
    _reserved_117_ltc0_: [u8; 4usize],
    _reserved_118_ltc0_: [u8; 4usize],
    _reserved_119_ltc0_: [u8; 4usize],
    _reserved_120_ltc0_: [u8; 4usize],
    _reserved_121_ltc0_: [u8; 4usize],
    _reserved_122_ltc0_: [u8; 4usize],
    _reserved_123_ltc0_: [u8; 4usize],
    _reserved_124_ltc0_: [u8; 4usize],
    _reserved_125_ltc0_: [u8; 4usize],
    _reserved_126_ltc0_: [u8; 4usize],
    _reserved_127_ltc0_: [u8; 4usize],
    _reserved_128_ltc0_: [u8; 4usize],
    _reserved_129_ltc0_: [u8; 4usize],
    _reserved_130_ltc0_: [u8; 4usize],
    _reserved_131_ltc0_: [u8; 4usize],
    _reserved_132_ltc0_: [u8; 4usize],
    _reserved_133_ltc0_: [u8; 4usize],
    _reserved_134_ltc0_: [u8; 4usize],
    _reserved_135_ltc0_: [u8; 4usize],
    _reserved_136_ltc0_: [u8; 4usize],
    _reserved_137_ltc0_: [u8; 4usize],
    _reserved_138_ltc0_: [u8; 4usize],
    _reserved_139_ltc0_: [u8; 4usize],
    _reserved_140_ltc0_: [u8; 4usize],
    _reserved_141_ltc0_: [u8; 4usize],
    _reserved_142_ltc0_: [u8; 4usize],
    _reserved_143_ltc0_: [u8; 4usize],
    _reserved_144_ltc0_: [u8; 4usize],
    _reserved_145_ltc0_: [u8; 4usize],
    _reserved_146_ltc0_: [u8; 4usize],
    _reserved_147_ltc0_: [u8; 4usize],
    _reserved_148_ltc0_: [u8; 4usize],
    _reserved_149_ltc0_: [u8; 4usize],
    _reserved_150_ltc0_: [u8; 4usize],
    _reserved_151_ltc0_: [u8; 4usize],
    _reserved_152_ltc0_: [u8; 4usize],
    _reserved_153_ltc0_: [u8; 4usize],
    _reserved_154_ltc0_: [u8; 4usize],
    _reserved_155_ltc0_: [u8; 4usize],
    _reserved_156_ltc0_: [u8; 4usize],
    _reserved_157_ltc0_: [u8; 4usize],
    _reserved_158_ltc0_: [u8; 4usize],
    _reserved_159_ltc0_: [u8; 4usize],
    _reserved_160_ltc0_: [u8; 4usize],
    _reserved_161_ltc0_: [u8; 4usize],
    _reserved_162_ltc0_: [u8; 4usize],
    _reserved_163_ltc0_: [u8; 4usize],
    _reserved_164_ltc0_: [u8; 4usize],
    _reserved_165_ltc0_: [u8; 4usize],
    _reserved_166_ltc0_: [u8; 4usize],
    _reserved_167_ltc0_: [u8; 4usize],
    _reserved_168_ltc0_: [u8; 4usize],
    _reserved_169_ltc0_: [u8; 4usize],
    _reserved_170_ltc0_: [u8; 4usize],
    _reserved_171_ltc0_: [u8; 4usize],
    _reserved_172_ltc0_: [u8; 4usize],
    _reserved_173_ltc0_: [u8; 4usize],
    _reserved174: [u8; 256usize],
    _reserved_174_ltc0_: [u8; 4usize],
    _reserved_175_ltc0_: [u8; 4usize],
    _reserved_176_ltc0_: [u8; 4usize],
    _reserved_177_ltc0_: [u8; 4usize],
    _reserved_178_ltc0_: [u8; 4usize],
    _reserved_179_ltc0_: [u8; 4usize],
    _reserved_180_ltc0_: [u8; 4usize],
    _reserved_181_ltc0_: [u8; 4usize],
    _reserved_182_ltc0_: [u8; 4usize],
    _reserved_183_ltc0_: [u8; 4usize],
    _reserved_184_ltc0_: [u8; 4usize],
    _reserved_185_ltc0_: [u8; 4usize],
    _reserved_186_ltc0_: [u8; 4usize],
    _reserved_187_ltc0_: [u8; 4usize],
    _reserved_188_ltc0_: [u8; 4usize],
    _reserved_189_ltc0_: [u8; 4usize],
    _reserved_190_ltc0_: [u8; 4usize],
    _reserved_191_ltc0_: [u8; 4usize],
    _reserved_192_ltc0_: [u8; 4usize],
    _reserved_193_ltc0_: [u8; 4usize],
    _reserved_194_ltc0_: [u8; 4usize],
    _reserved_195_ltc0_: [u8; 4usize],
    _reserved_196_ltc0_: [u8; 4usize],
    _reserved_197_ltc0_: [u8; 4usize],
    _reserved_198_ltc0_: [u8; 4usize],
    _reserved_199_ltc0_: [u8; 4usize],
    _reserved_200_ltc0_: [u8; 4usize],
    _reserved_201_ltc0_: [u8; 4usize],
    _reserved_202_ltc0_: [u8; 4usize],
    _reserved_203_ltc0_: [u8; 4usize],
    _reserved_204_ltc0_: [u8; 4usize],
    _reserved_205_ltc0_: [u8; 4usize],
    _reserved_206_ltc0_: [u8; 4usize],
    _reserved_207_ltc0_: [u8; 4usize],
    _reserved_208_ltc0_: [u8; 4usize],
    _reserved_209_ltc0_: [u8; 4usize],
    _reserved_210_ltc0_: [u8; 4usize],
    _reserved_211_ltc0_: [u8; 4usize],
    _reserved_212_ltc0_: [u8; 4usize],
    _reserved_213_ltc0_: [u8; 4usize],
    _reserved_214_ltc0_: [u8; 4usize],
    _reserved_215_ltc0_: [u8; 4usize],
    _reserved_216_ltc0_: [u8; 4usize],
    _reserved_217_ltc0_: [u8; 4usize],
    _reserved_218_ltc0_: [u8; 4usize],
    _reserved_219_ltc0_: [u8; 4usize],
    _reserved_220_ltc0_: [u8; 4usize],
    _reserved_221_ltc0_: [u8; 4usize],
    _reserved_222_ltc0_: [u8; 4usize],
    _reserved_223_ltc0_: [u8; 4usize],
    _reserved_224_ltc0_: [u8; 4usize],
    _reserved_225_ltc0_: [u8; 4usize],
    _reserved_226_ltc0_: [u8; 4usize],
    _reserved_227_ltc0_: [u8; 4usize],
    _reserved_228_ltc0_: [u8; 4usize],
    _reserved_229_ltc0_: [u8; 4usize],
    _reserved_230_ltc0_: [u8; 4usize],
    _reserved_231_ltc0_: [u8; 4usize],
    _reserved_232_ltc0_: [u8; 4usize],
    _reserved_233_ltc0_: [u8; 4usize],
    _reserved_234_ltc0_: [u8; 4usize],
    _reserved_235_ltc0_: [u8; 4usize],
    _reserved_236_ltc0_: [u8; 4usize],
    _reserved_237_ltc0_: [u8; 4usize],
    _reserved238: [u8; 256usize],
    _reserved_238_ltc0_: [u8; 4usize],
    _reserved_239_ltc0_: [u8; 4usize],
    _reserved_240_ltc0_: [u8; 4usize],
    _reserved_241_ltc0_: [u8; 4usize],
    _reserved_242_ltc0_: [u8; 4usize],
    _reserved_243_ltc0_: [u8; 4usize],
    _reserved_244_ltc0_: [u8; 4usize],
    _reserved_245_ltc0_: [u8; 4usize],
    _reserved_246_ltc0_: [u8; 4usize],
    _reserved_247_ltc0_: [u8; 4usize],
    _reserved_248_ltc0_: [u8; 4usize],
    _reserved_249_ltc0_: [u8; 4usize],
    _reserved_250_ltc0_: [u8; 4usize],
    _reserved_251_ltc0_: [u8; 4usize],
    _reserved_252_ltc0_: [u8; 4usize],
    _reserved_253_ltc0_: [u8; 4usize],
    _reserved_254_ltc0_: [u8; 4usize],
    _reserved_255_ltc0_: [u8; 4usize],
    _reserved_256_ltc0_: [u8; 4usize],
    _reserved_257_ltc0_: [u8; 4usize],
    _reserved_258_ltc0_: [u8; 4usize],
    _reserved_259_ltc0_: [u8; 4usize],
    _reserved_260_ltc0_: [u8; 4usize],
    _reserved_261_ltc0_: [u8; 4usize],
    _reserved_262_ltc0_: [u8; 4usize],
    _reserved_263_ltc0_: [u8; 4usize],
    _reserved_264_ltc0_: [u8; 4usize],
    _reserved_265_ltc0_: [u8; 4usize],
    _reserved_266_ltc0_: [u8; 4usize],
    _reserved_267_ltc0_: [u8; 4usize],
    _reserved_268_ltc0_: [u8; 4usize],
    _reserved_269_ltc0_: [u8; 4usize],
    _reserved_270_ltc0_: [u8; 4usize],
    _reserved_271_ltc0_: [u8; 4usize],
    _reserved_272_ltc0_: [u8; 4usize],
    _reserved_273_ltc0_: [u8; 4usize],
    _reserved_274_ltc0_: [u8; 4usize],
    _reserved_275_ltc0_: [u8; 4usize],
    _reserved_276_ltc0_: [u8; 4usize],
    _reserved_277_ltc0_: [u8; 4usize],
    _reserved_278_ltc0_: [u8; 4usize],
    _reserved_279_ltc0_: [u8; 4usize],
    _reserved_280_ltc0_: [u8; 4usize],
    _reserved_281_ltc0_: [u8; 4usize],
    _reserved_282_ltc0_: [u8; 4usize],
    _reserved_283_ltc0_: [u8; 4usize],
    _reserved_284_ltc0_: [u8; 4usize],
    _reserved_285_ltc0_: [u8; 4usize],
    _reserved_286_ltc0_: [u8; 4usize],
    _reserved_287_ltc0_: [u8; 4usize],
    _reserved_288_ltc0_: [u8; 4usize],
    _reserved_289_ltc0_: [u8; 4usize],
    _reserved_290_ltc0_: [u8; 4usize],
    _reserved_291_ltc0_: [u8; 4usize],
    _reserved_292_ltc0_: [u8; 4usize],
    _reserved_293_ltc0_: [u8; 4usize],
    _reserved_294_ltc0_: [u8; 4usize],
    _reserved_295_ltc0_: [u8; 4usize],
    _reserved_296_ltc0_: [u8; 4usize],
    _reserved_297_ltc0_: [u8; 4usize],
    _reserved_298_ltc0_: [u8; 4usize],
    _reserved_299_ltc0_: [u8; 4usize],
    _reserved_300_ltc0_: [u8; 4usize],
    _reserved_301_ltc0_: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x00 - LTC Mode Register (PublicKey)"]
    #[inline(always)]
    pub fn ltc0_mdpk(&self) -> &LTC0_MDPK {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const LTC0_MDPK) }
    }
    #[doc = "0x00 - LTC Mode Register (PublicKey)"]
    #[inline(always)]
    pub fn ltc0_mdpk_mut(&self) -> &mut LTC0_MDPK {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut LTC0_MDPK) }
    }
    #[doc = "0x00 - LTC Mode Register (non-PKHA/non-RNG use)"]
    #[inline(always)]
    pub fn ltc0_md(&self) -> &LTC0_MD {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const LTC0_MD) }
    }
    #[doc = "0x00 - LTC Mode Register (non-PKHA/non-RNG use)"]
    #[inline(always)]
    pub fn ltc0_md_mut(&self) -> &mut LTC0_MD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut LTC0_MD) }
    }
    #[doc = "0x800 - LTC PKHA A 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka_0(&self) -> &LTC0_PKA_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2048usize) as *const LTC0_PKA_0) }
    }
    #[doc = "0x800 - LTC PKHA A 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka_0_mut(&self) -> &mut LTC0_PKA_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2048usize) as *mut LTC0_PKA_0) }
    }
    #[doc = "0x800 - LTC PKHA A0 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_0(&self) -> &LTC0_PKA0_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2048usize) as *const LTC0_PKA0_0) }
    }
    #[doc = "0x800 - LTC PKHA A0 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_0_mut(&self) -> &mut LTC0_PKA0_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2048usize) as *mut LTC0_PKA0_0) }
    }
    #[doc = "0x804 - LTC PKHA A 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka_1(&self) -> &LTC0_PKA_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2052usize) as *const LTC0_PKA_1) }
    }
    #[doc = "0x804 - LTC PKHA A 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka_1_mut(&self) -> &mut LTC0_PKA_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2052usize) as *mut LTC0_PKA_1) }
    }
    #[doc = "0x804 - LTC PKHA A0 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_1(&self) -> &LTC0_PKA0_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2052usize) as *const LTC0_PKA0_1) }
    }
    #[doc = "0x804 - LTC PKHA A0 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_1_mut(&self) -> &mut LTC0_PKA0_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2052usize) as *mut LTC0_PKA0_1) }
    }
    #[doc = "0x808 - LTC PKHA A 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka_2(&self) -> &LTC0_PKA_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2056usize) as *const LTC0_PKA_2) }
    }
    #[doc = "0x808 - LTC PKHA A 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka_2_mut(&self) -> &mut LTC0_PKA_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2056usize) as *mut LTC0_PKA_2) }
    }
    #[doc = "0x808 - LTC PKHA A0 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_2(&self) -> &LTC0_PKA0_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2056usize) as *const LTC0_PKA0_2) }
    }
    #[doc = "0x808 - LTC PKHA A0 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_2_mut(&self) -> &mut LTC0_PKA0_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2056usize) as *mut LTC0_PKA0_2) }
    }
    #[doc = "0x80c - LTC PKHA A 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka_3(&self) -> &LTC0_PKA_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2060usize) as *const LTC0_PKA_3) }
    }
    #[doc = "0x80c - LTC PKHA A 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka_3_mut(&self) -> &mut LTC0_PKA_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2060usize) as *mut LTC0_PKA_3) }
    }
    #[doc = "0x80c - LTC PKHA A0 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_3(&self) -> &LTC0_PKA0_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2060usize) as *const LTC0_PKA0_3) }
    }
    #[doc = "0x80c - LTC PKHA A0 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_3_mut(&self) -> &mut LTC0_PKA0_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2060usize) as *mut LTC0_PKA0_3) }
    }
    #[doc = "0x810 - LTC PKHA A 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka_4(&self) -> &LTC0_PKA_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2064usize) as *const LTC0_PKA_4) }
    }
    #[doc = "0x810 - LTC PKHA A 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka_4_mut(&self) -> &mut LTC0_PKA_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2064usize) as *mut LTC0_PKA_4) }
    }
    #[doc = "0x810 - LTC PKHA A0 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_4(&self) -> &LTC0_PKA0_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2064usize) as *const LTC0_PKA0_4) }
    }
    #[doc = "0x810 - LTC PKHA A0 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_4_mut(&self) -> &mut LTC0_PKA0_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2064usize) as *mut LTC0_PKA0_4) }
    }
    #[doc = "0x814 - LTC PKHA A 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka_5(&self) -> &LTC0_PKA_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2068usize) as *const LTC0_PKA_5) }
    }
    #[doc = "0x814 - LTC PKHA A 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka_5_mut(&self) -> &mut LTC0_PKA_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2068usize) as *mut LTC0_PKA_5) }
    }
    #[doc = "0x814 - LTC PKHA A0 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_5(&self) -> &LTC0_PKA0_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2068usize) as *const LTC0_PKA0_5) }
    }
    #[doc = "0x814 - LTC PKHA A0 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_5_mut(&self) -> &mut LTC0_PKA0_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2068usize) as *mut LTC0_PKA0_5) }
    }
    #[doc = "0x818 - LTC PKHA A 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka_6(&self) -> &LTC0_PKA_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2072usize) as *const LTC0_PKA_6) }
    }
    #[doc = "0x818 - LTC PKHA A 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka_6_mut(&self) -> &mut LTC0_PKA_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2072usize) as *mut LTC0_PKA_6) }
    }
    #[doc = "0x818 - LTC PKHA A0 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_6(&self) -> &LTC0_PKA0_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2072usize) as *const LTC0_PKA0_6) }
    }
    #[doc = "0x818 - LTC PKHA A0 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_6_mut(&self) -> &mut LTC0_PKA0_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2072usize) as *mut LTC0_PKA0_6) }
    }
    #[doc = "0x81c - LTC PKHA A 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka_7(&self) -> &LTC0_PKA_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2076usize) as *const LTC0_PKA_7) }
    }
    #[doc = "0x81c - LTC PKHA A 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka_7_mut(&self) -> &mut LTC0_PKA_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2076usize) as *mut LTC0_PKA_7) }
    }
    #[doc = "0x81c - LTC PKHA A0 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_7(&self) -> &LTC0_PKA0_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2076usize) as *const LTC0_PKA0_7) }
    }
    #[doc = "0x81c - LTC PKHA A0 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_7_mut(&self) -> &mut LTC0_PKA0_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2076usize) as *mut LTC0_PKA0_7) }
    }
    #[doc = "0x820 - LTC PKHA A 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka_8(&self) -> &LTC0_PKA_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2080usize) as *const LTC0_PKA_8) }
    }
    #[doc = "0x820 - LTC PKHA A 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka_8_mut(&self) -> &mut LTC0_PKA_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2080usize) as *mut LTC0_PKA_8) }
    }
    #[doc = "0x820 - LTC PKHA A0 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_8(&self) -> &LTC0_PKA0_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2080usize) as *const LTC0_PKA0_8) }
    }
    #[doc = "0x820 - LTC PKHA A0 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_8_mut(&self) -> &mut LTC0_PKA0_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2080usize) as *mut LTC0_PKA0_8) }
    }
    #[doc = "0x824 - LTC PKHA A 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka_9(&self) -> &LTC0_PKA_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2084usize) as *const LTC0_PKA_9) }
    }
    #[doc = "0x824 - LTC PKHA A 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka_9_mut(&self) -> &mut LTC0_PKA_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2084usize) as *mut LTC0_PKA_9) }
    }
    #[doc = "0x824 - LTC PKHA A0 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_9(&self) -> &LTC0_PKA0_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2084usize) as *const LTC0_PKA0_9) }
    }
    #[doc = "0x824 - LTC PKHA A0 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_9_mut(&self) -> &mut LTC0_PKA0_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2084usize) as *mut LTC0_PKA0_9) }
    }
    #[doc = "0x828 - LTC PKHA A 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka_10(&self) -> &LTC0_PKA_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2088usize) as *const LTC0_PKA_10) }
    }
    #[doc = "0x828 - LTC PKHA A 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka_10_mut(&self) -> &mut LTC0_PKA_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2088usize) as *mut LTC0_PKA_10) }
    }
    #[doc = "0x828 - LTC PKHA A0 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_10(&self) -> &LTC0_PKA0_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2088usize) as *const LTC0_PKA0_10) }
    }
    #[doc = "0x828 - LTC PKHA A0 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_10_mut(&self) -> &mut LTC0_PKA0_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2088usize) as *mut LTC0_PKA0_10) }
    }
    #[doc = "0x82c - LTC PKHA A 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka_11(&self) -> &LTC0_PKA_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2092usize) as *const LTC0_PKA_11) }
    }
    #[doc = "0x82c - LTC PKHA A 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka_11_mut(&self) -> &mut LTC0_PKA_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2092usize) as *mut LTC0_PKA_11) }
    }
    #[doc = "0x82c - LTC PKHA A0 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_11(&self) -> &LTC0_PKA0_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2092usize) as *const LTC0_PKA0_11) }
    }
    #[doc = "0x82c - LTC PKHA A0 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_11_mut(&self) -> &mut LTC0_PKA0_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2092usize) as *mut LTC0_PKA0_11) }
    }
    #[doc = "0x830 - LTC PKHA A 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka_12(&self) -> &LTC0_PKA_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2096usize) as *const LTC0_PKA_12) }
    }
    #[doc = "0x830 - LTC PKHA A 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka_12_mut(&self) -> &mut LTC0_PKA_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2096usize) as *mut LTC0_PKA_12) }
    }
    #[doc = "0x830 - LTC PKHA A0 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_12(&self) -> &LTC0_PKA0_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2096usize) as *const LTC0_PKA0_12) }
    }
    #[doc = "0x830 - LTC PKHA A0 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_12_mut(&self) -> &mut LTC0_PKA0_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2096usize) as *mut LTC0_PKA0_12) }
    }
    #[doc = "0x834 - LTC PKHA A 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka_13(&self) -> &LTC0_PKA_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2100usize) as *const LTC0_PKA_13) }
    }
    #[doc = "0x834 - LTC PKHA A 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka_13_mut(&self) -> &mut LTC0_PKA_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2100usize) as *mut LTC0_PKA_13) }
    }
    #[doc = "0x834 - LTC PKHA A0 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_13(&self) -> &LTC0_PKA0_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2100usize) as *const LTC0_PKA0_13) }
    }
    #[doc = "0x834 - LTC PKHA A0 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_13_mut(&self) -> &mut LTC0_PKA0_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2100usize) as *mut LTC0_PKA0_13) }
    }
    #[doc = "0x838 - LTC PKHA A 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka_14(&self) -> &LTC0_PKA_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2104usize) as *const LTC0_PKA_14) }
    }
    #[doc = "0x838 - LTC PKHA A 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka_14_mut(&self) -> &mut LTC0_PKA_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2104usize) as *mut LTC0_PKA_14) }
    }
    #[doc = "0x838 - LTC PKHA A0 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_14(&self) -> &LTC0_PKA0_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2104usize) as *const LTC0_PKA0_14) }
    }
    #[doc = "0x838 - LTC PKHA A0 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_14_mut(&self) -> &mut LTC0_PKA0_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2104usize) as *mut LTC0_PKA0_14) }
    }
    #[doc = "0x83c - LTC PKHA A 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka_15(&self) -> &LTC0_PKA_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2108usize) as *const LTC0_PKA_15) }
    }
    #[doc = "0x83c - LTC PKHA A 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka_15_mut(&self) -> &mut LTC0_PKA_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2108usize) as *mut LTC0_PKA_15) }
    }
    #[doc = "0x83c - LTC PKHA A0 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_15(&self) -> &LTC0_PKA0_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2108usize) as *const LTC0_PKA0_15) }
    }
    #[doc = "0x83c - LTC PKHA A0 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka0_15_mut(&self) -> &mut LTC0_PKA0_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2108usize) as *mut LTC0_PKA0_15) }
    }
    #[doc = "0x840 - LTC PKHA A 16 Register"]
    #[inline(always)]
    pub fn ltc0_pka_16(&self) -> &LTC0_PKA_16 {
        unsafe { &*(((self as *const Self) as *const u8).add(2112usize) as *const LTC0_PKA_16) }
    }
    #[doc = "0x840 - LTC PKHA A 16 Register"]
    #[inline(always)]
    pub fn ltc0_pka_16_mut(&self) -> &mut LTC0_PKA_16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2112usize) as *mut LTC0_PKA_16) }
    }
    #[doc = "0x840 - LTC PKHA A1 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_0(&self) -> &LTC0_PKA1_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2112usize) as *const LTC0_PKA1_0) }
    }
    #[doc = "0x840 - LTC PKHA A1 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_0_mut(&self) -> &mut LTC0_PKA1_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2112usize) as *mut LTC0_PKA1_0) }
    }
    #[doc = "0x844 - LTC PKHA A 17 Register"]
    #[inline(always)]
    pub fn ltc0_pka_17(&self) -> &LTC0_PKA_17 {
        unsafe { &*(((self as *const Self) as *const u8).add(2116usize) as *const LTC0_PKA_17) }
    }
    #[doc = "0x844 - LTC PKHA A 17 Register"]
    #[inline(always)]
    pub fn ltc0_pka_17_mut(&self) -> &mut LTC0_PKA_17 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2116usize) as *mut LTC0_PKA_17) }
    }
    #[doc = "0x844 - LTC PKHA A1 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_1(&self) -> &LTC0_PKA1_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2116usize) as *const LTC0_PKA1_1) }
    }
    #[doc = "0x844 - LTC PKHA A1 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_1_mut(&self) -> &mut LTC0_PKA1_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2116usize) as *mut LTC0_PKA1_1) }
    }
    #[doc = "0x848 - LTC PKHA A 18 Register"]
    #[inline(always)]
    pub fn ltc0_pka_18(&self) -> &LTC0_PKA_18 {
        unsafe { &*(((self as *const Self) as *const u8).add(2120usize) as *const LTC0_PKA_18) }
    }
    #[doc = "0x848 - LTC PKHA A 18 Register"]
    #[inline(always)]
    pub fn ltc0_pka_18_mut(&self) -> &mut LTC0_PKA_18 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2120usize) as *mut LTC0_PKA_18) }
    }
    #[doc = "0x848 - LTC PKHA A1 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_2(&self) -> &LTC0_PKA1_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2120usize) as *const LTC0_PKA1_2) }
    }
    #[doc = "0x848 - LTC PKHA A1 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_2_mut(&self) -> &mut LTC0_PKA1_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2120usize) as *mut LTC0_PKA1_2) }
    }
    #[doc = "0x84c - LTC PKHA A 19 Register"]
    #[inline(always)]
    pub fn ltc0_pka_19(&self) -> &LTC0_PKA_19 {
        unsafe { &*(((self as *const Self) as *const u8).add(2124usize) as *const LTC0_PKA_19) }
    }
    #[doc = "0x84c - LTC PKHA A 19 Register"]
    #[inline(always)]
    pub fn ltc0_pka_19_mut(&self) -> &mut LTC0_PKA_19 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2124usize) as *mut LTC0_PKA_19) }
    }
    #[doc = "0x84c - LTC PKHA A1 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_3(&self) -> &LTC0_PKA1_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2124usize) as *const LTC0_PKA1_3) }
    }
    #[doc = "0x84c - LTC PKHA A1 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_3_mut(&self) -> &mut LTC0_PKA1_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2124usize) as *mut LTC0_PKA1_3) }
    }
    #[doc = "0x850 - LTC PKHA A 20 Register"]
    #[inline(always)]
    pub fn ltc0_pka_20(&self) -> &LTC0_PKA_20 {
        unsafe { &*(((self as *const Self) as *const u8).add(2128usize) as *const LTC0_PKA_20) }
    }
    #[doc = "0x850 - LTC PKHA A 20 Register"]
    #[inline(always)]
    pub fn ltc0_pka_20_mut(&self) -> &mut LTC0_PKA_20 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2128usize) as *mut LTC0_PKA_20) }
    }
    #[doc = "0x850 - LTC PKHA A1 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_4(&self) -> &LTC0_PKA1_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2128usize) as *const LTC0_PKA1_4) }
    }
    #[doc = "0x850 - LTC PKHA A1 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_4_mut(&self) -> &mut LTC0_PKA1_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2128usize) as *mut LTC0_PKA1_4) }
    }
    #[doc = "0x854 - LTC PKHA A 21 Register"]
    #[inline(always)]
    pub fn ltc0_pka_21(&self) -> &LTC0_PKA_21 {
        unsafe { &*(((self as *const Self) as *const u8).add(2132usize) as *const LTC0_PKA_21) }
    }
    #[doc = "0x854 - LTC PKHA A 21 Register"]
    #[inline(always)]
    pub fn ltc0_pka_21_mut(&self) -> &mut LTC0_PKA_21 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2132usize) as *mut LTC0_PKA_21) }
    }
    #[doc = "0x854 - LTC PKHA A1 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_5(&self) -> &LTC0_PKA1_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2132usize) as *const LTC0_PKA1_5) }
    }
    #[doc = "0x854 - LTC PKHA A1 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_5_mut(&self) -> &mut LTC0_PKA1_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2132usize) as *mut LTC0_PKA1_5) }
    }
    #[doc = "0x858 - LTC PKHA A 22 Register"]
    #[inline(always)]
    pub fn ltc0_pka_22(&self) -> &LTC0_PKA_22 {
        unsafe { &*(((self as *const Self) as *const u8).add(2136usize) as *const LTC0_PKA_22) }
    }
    #[doc = "0x858 - LTC PKHA A 22 Register"]
    #[inline(always)]
    pub fn ltc0_pka_22_mut(&self) -> &mut LTC0_PKA_22 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2136usize) as *mut LTC0_PKA_22) }
    }
    #[doc = "0x858 - LTC PKHA A1 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_6(&self) -> &LTC0_PKA1_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2136usize) as *const LTC0_PKA1_6) }
    }
    #[doc = "0x858 - LTC PKHA A1 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_6_mut(&self) -> &mut LTC0_PKA1_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2136usize) as *mut LTC0_PKA1_6) }
    }
    #[doc = "0x85c - LTC PKHA A 23 Register"]
    #[inline(always)]
    pub fn ltc0_pka_23(&self) -> &LTC0_PKA_23 {
        unsafe { &*(((self as *const Self) as *const u8).add(2140usize) as *const LTC0_PKA_23) }
    }
    #[doc = "0x85c - LTC PKHA A 23 Register"]
    #[inline(always)]
    pub fn ltc0_pka_23_mut(&self) -> &mut LTC0_PKA_23 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2140usize) as *mut LTC0_PKA_23) }
    }
    #[doc = "0x85c - LTC PKHA A1 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_7(&self) -> &LTC0_PKA1_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2140usize) as *const LTC0_PKA1_7) }
    }
    #[doc = "0x85c - LTC PKHA A1 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_7_mut(&self) -> &mut LTC0_PKA1_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2140usize) as *mut LTC0_PKA1_7) }
    }
    #[doc = "0x860 - LTC PKHA A 24 Register"]
    #[inline(always)]
    pub fn ltc0_pka_24(&self) -> &LTC0_PKA_24 {
        unsafe { &*(((self as *const Self) as *const u8).add(2144usize) as *const LTC0_PKA_24) }
    }
    #[doc = "0x860 - LTC PKHA A 24 Register"]
    #[inline(always)]
    pub fn ltc0_pka_24_mut(&self) -> &mut LTC0_PKA_24 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2144usize) as *mut LTC0_PKA_24) }
    }
    #[doc = "0x860 - LTC PKHA A1 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_8(&self) -> &LTC0_PKA1_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2144usize) as *const LTC0_PKA1_8) }
    }
    #[doc = "0x860 - LTC PKHA A1 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_8_mut(&self) -> &mut LTC0_PKA1_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2144usize) as *mut LTC0_PKA1_8) }
    }
    #[doc = "0x864 - LTC PKHA A 25 Register"]
    #[inline(always)]
    pub fn ltc0_pka_25(&self) -> &LTC0_PKA_25 {
        unsafe { &*(((self as *const Self) as *const u8).add(2148usize) as *const LTC0_PKA_25) }
    }
    #[doc = "0x864 - LTC PKHA A 25 Register"]
    #[inline(always)]
    pub fn ltc0_pka_25_mut(&self) -> &mut LTC0_PKA_25 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2148usize) as *mut LTC0_PKA_25) }
    }
    #[doc = "0x864 - LTC PKHA A1 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_9(&self) -> &LTC0_PKA1_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2148usize) as *const LTC0_PKA1_9) }
    }
    #[doc = "0x864 - LTC PKHA A1 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_9_mut(&self) -> &mut LTC0_PKA1_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2148usize) as *mut LTC0_PKA1_9) }
    }
    #[doc = "0x868 - LTC PKHA A 26 Register"]
    #[inline(always)]
    pub fn ltc0_pka_26(&self) -> &LTC0_PKA_26 {
        unsafe { &*(((self as *const Self) as *const u8).add(2152usize) as *const LTC0_PKA_26) }
    }
    #[doc = "0x868 - LTC PKHA A 26 Register"]
    #[inline(always)]
    pub fn ltc0_pka_26_mut(&self) -> &mut LTC0_PKA_26 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2152usize) as *mut LTC0_PKA_26) }
    }
    #[doc = "0x868 - LTC PKHA A1 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_10(&self) -> &LTC0_PKA1_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2152usize) as *const LTC0_PKA1_10) }
    }
    #[doc = "0x868 - LTC PKHA A1 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_10_mut(&self) -> &mut LTC0_PKA1_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2152usize) as *mut LTC0_PKA1_10) }
    }
    #[doc = "0x86c - LTC PKHA A 27 Register"]
    #[inline(always)]
    pub fn ltc0_pka_27(&self) -> &LTC0_PKA_27 {
        unsafe { &*(((self as *const Self) as *const u8).add(2156usize) as *const LTC0_PKA_27) }
    }
    #[doc = "0x86c - LTC PKHA A 27 Register"]
    #[inline(always)]
    pub fn ltc0_pka_27_mut(&self) -> &mut LTC0_PKA_27 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2156usize) as *mut LTC0_PKA_27) }
    }
    #[doc = "0x86c - LTC PKHA A1 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_11(&self) -> &LTC0_PKA1_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2156usize) as *const LTC0_PKA1_11) }
    }
    #[doc = "0x86c - LTC PKHA A1 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_11_mut(&self) -> &mut LTC0_PKA1_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2156usize) as *mut LTC0_PKA1_11) }
    }
    #[doc = "0x870 - LTC PKHA A 28 Register"]
    #[inline(always)]
    pub fn ltc0_pka_28(&self) -> &LTC0_PKA_28 {
        unsafe { &*(((self as *const Self) as *const u8).add(2160usize) as *const LTC0_PKA_28) }
    }
    #[doc = "0x870 - LTC PKHA A 28 Register"]
    #[inline(always)]
    pub fn ltc0_pka_28_mut(&self) -> &mut LTC0_PKA_28 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2160usize) as *mut LTC0_PKA_28) }
    }
    #[doc = "0x870 - LTC PKHA A1 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_12(&self) -> &LTC0_PKA1_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2160usize) as *const LTC0_PKA1_12) }
    }
    #[doc = "0x870 - LTC PKHA A1 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_12_mut(&self) -> &mut LTC0_PKA1_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2160usize) as *mut LTC0_PKA1_12) }
    }
    #[doc = "0x874 - LTC PKHA A 29 Register"]
    #[inline(always)]
    pub fn ltc0_pka_29(&self) -> &LTC0_PKA_29 {
        unsafe { &*(((self as *const Self) as *const u8).add(2164usize) as *const LTC0_PKA_29) }
    }
    #[doc = "0x874 - LTC PKHA A 29 Register"]
    #[inline(always)]
    pub fn ltc0_pka_29_mut(&self) -> &mut LTC0_PKA_29 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2164usize) as *mut LTC0_PKA_29) }
    }
    #[doc = "0x874 - LTC PKHA A1 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_13(&self) -> &LTC0_PKA1_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2164usize) as *const LTC0_PKA1_13) }
    }
    #[doc = "0x874 - LTC PKHA A1 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_13_mut(&self) -> &mut LTC0_PKA1_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2164usize) as *mut LTC0_PKA1_13) }
    }
    #[doc = "0x878 - LTC PKHA A 30 Register"]
    #[inline(always)]
    pub fn ltc0_pka_30(&self) -> &LTC0_PKA_30 {
        unsafe { &*(((self as *const Self) as *const u8).add(2168usize) as *const LTC0_PKA_30) }
    }
    #[doc = "0x878 - LTC PKHA A 30 Register"]
    #[inline(always)]
    pub fn ltc0_pka_30_mut(&self) -> &mut LTC0_PKA_30 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2168usize) as *mut LTC0_PKA_30) }
    }
    #[doc = "0x878 - LTC PKHA A1 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_14(&self) -> &LTC0_PKA1_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2168usize) as *const LTC0_PKA1_14) }
    }
    #[doc = "0x878 - LTC PKHA A1 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_14_mut(&self) -> &mut LTC0_PKA1_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2168usize) as *mut LTC0_PKA1_14) }
    }
    #[doc = "0x87c - LTC PKHA A 31 Register"]
    #[inline(always)]
    pub fn ltc0_pka_31(&self) -> &LTC0_PKA_31 {
        unsafe { &*(((self as *const Self) as *const u8).add(2172usize) as *const LTC0_PKA_31) }
    }
    #[doc = "0x87c - LTC PKHA A 31 Register"]
    #[inline(always)]
    pub fn ltc0_pka_31_mut(&self) -> &mut LTC0_PKA_31 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2172usize) as *mut LTC0_PKA_31) }
    }
    #[doc = "0x87c - LTC PKHA A1 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_15(&self) -> &LTC0_PKA1_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2172usize) as *const LTC0_PKA1_15) }
    }
    #[doc = "0x87c - LTC PKHA A1 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka1_15_mut(&self) -> &mut LTC0_PKA1_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2172usize) as *mut LTC0_PKA1_15) }
    }
    #[doc = "0x880 - LTC PKHA A 32 Register"]
    #[inline(always)]
    pub fn ltc0_pka_32(&self) -> &LTC0_PKA_32 {
        unsafe { &*(((self as *const Self) as *const u8).add(2176usize) as *const LTC0_PKA_32) }
    }
    #[doc = "0x880 - LTC PKHA A 32 Register"]
    #[inline(always)]
    pub fn ltc0_pka_32_mut(&self) -> &mut LTC0_PKA_32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2176usize) as *mut LTC0_PKA_32) }
    }
    #[doc = "0x880 - LTC PKHA A2 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_0(&self) -> &LTC0_PKA2_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2176usize) as *const LTC0_PKA2_0) }
    }
    #[doc = "0x880 - LTC PKHA A2 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_0_mut(&self) -> &mut LTC0_PKA2_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2176usize) as *mut LTC0_PKA2_0) }
    }
    #[doc = "0x884 - LTC PKHA A 33 Register"]
    #[inline(always)]
    pub fn ltc0_pka_33(&self) -> &LTC0_PKA_33 {
        unsafe { &*(((self as *const Self) as *const u8).add(2180usize) as *const LTC0_PKA_33) }
    }
    #[doc = "0x884 - LTC PKHA A 33 Register"]
    #[inline(always)]
    pub fn ltc0_pka_33_mut(&self) -> &mut LTC0_PKA_33 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2180usize) as *mut LTC0_PKA_33) }
    }
    #[doc = "0x884 - LTC PKHA A2 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_1(&self) -> &LTC0_PKA2_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2180usize) as *const LTC0_PKA2_1) }
    }
    #[doc = "0x884 - LTC PKHA A2 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_1_mut(&self) -> &mut LTC0_PKA2_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2180usize) as *mut LTC0_PKA2_1) }
    }
    #[doc = "0x888 - LTC PKHA A 34 Register"]
    #[inline(always)]
    pub fn ltc0_pka_34(&self) -> &LTC0_PKA_34 {
        unsafe { &*(((self as *const Self) as *const u8).add(2184usize) as *const LTC0_PKA_34) }
    }
    #[doc = "0x888 - LTC PKHA A 34 Register"]
    #[inline(always)]
    pub fn ltc0_pka_34_mut(&self) -> &mut LTC0_PKA_34 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2184usize) as *mut LTC0_PKA_34) }
    }
    #[doc = "0x888 - LTC PKHA A2 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_2(&self) -> &LTC0_PKA2_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2184usize) as *const LTC0_PKA2_2) }
    }
    #[doc = "0x888 - LTC PKHA A2 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_2_mut(&self) -> &mut LTC0_PKA2_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2184usize) as *mut LTC0_PKA2_2) }
    }
    #[doc = "0x88c - LTC PKHA A 35 Register"]
    #[inline(always)]
    pub fn ltc0_pka_35(&self) -> &LTC0_PKA_35 {
        unsafe { &*(((self as *const Self) as *const u8).add(2188usize) as *const LTC0_PKA_35) }
    }
    #[doc = "0x88c - LTC PKHA A 35 Register"]
    #[inline(always)]
    pub fn ltc0_pka_35_mut(&self) -> &mut LTC0_PKA_35 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2188usize) as *mut LTC0_PKA_35) }
    }
    #[doc = "0x88c - LTC PKHA A2 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_3(&self) -> &LTC0_PKA2_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2188usize) as *const LTC0_PKA2_3) }
    }
    #[doc = "0x88c - LTC PKHA A2 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_3_mut(&self) -> &mut LTC0_PKA2_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2188usize) as *mut LTC0_PKA2_3) }
    }
    #[doc = "0x890 - LTC PKHA A 36 Register"]
    #[inline(always)]
    pub fn ltc0_pka_36(&self) -> &LTC0_PKA_36 {
        unsafe { &*(((self as *const Self) as *const u8).add(2192usize) as *const LTC0_PKA_36) }
    }
    #[doc = "0x890 - LTC PKHA A 36 Register"]
    #[inline(always)]
    pub fn ltc0_pka_36_mut(&self) -> &mut LTC0_PKA_36 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2192usize) as *mut LTC0_PKA_36) }
    }
    #[doc = "0x890 - LTC PKHA A2 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_4(&self) -> &LTC0_PKA2_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2192usize) as *const LTC0_PKA2_4) }
    }
    #[doc = "0x890 - LTC PKHA A2 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_4_mut(&self) -> &mut LTC0_PKA2_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2192usize) as *mut LTC0_PKA2_4) }
    }
    #[doc = "0x894 - LTC PKHA A 37 Register"]
    #[inline(always)]
    pub fn ltc0_pka_37(&self) -> &LTC0_PKA_37 {
        unsafe { &*(((self as *const Self) as *const u8).add(2196usize) as *const LTC0_PKA_37) }
    }
    #[doc = "0x894 - LTC PKHA A 37 Register"]
    #[inline(always)]
    pub fn ltc0_pka_37_mut(&self) -> &mut LTC0_PKA_37 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2196usize) as *mut LTC0_PKA_37) }
    }
    #[doc = "0x894 - LTC PKHA A2 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_5(&self) -> &LTC0_PKA2_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2196usize) as *const LTC0_PKA2_5) }
    }
    #[doc = "0x894 - LTC PKHA A2 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_5_mut(&self) -> &mut LTC0_PKA2_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2196usize) as *mut LTC0_PKA2_5) }
    }
    #[doc = "0x898 - LTC PKHA A 38 Register"]
    #[inline(always)]
    pub fn ltc0_pka_38(&self) -> &LTC0_PKA_38 {
        unsafe { &*(((self as *const Self) as *const u8).add(2200usize) as *const LTC0_PKA_38) }
    }
    #[doc = "0x898 - LTC PKHA A 38 Register"]
    #[inline(always)]
    pub fn ltc0_pka_38_mut(&self) -> &mut LTC0_PKA_38 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2200usize) as *mut LTC0_PKA_38) }
    }
    #[doc = "0x898 - LTC PKHA A2 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_6(&self) -> &LTC0_PKA2_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2200usize) as *const LTC0_PKA2_6) }
    }
    #[doc = "0x898 - LTC PKHA A2 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_6_mut(&self) -> &mut LTC0_PKA2_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2200usize) as *mut LTC0_PKA2_6) }
    }
    #[doc = "0x89c - LTC PKHA A 39 Register"]
    #[inline(always)]
    pub fn ltc0_pka_39(&self) -> &LTC0_PKA_39 {
        unsafe { &*(((self as *const Self) as *const u8).add(2204usize) as *const LTC0_PKA_39) }
    }
    #[doc = "0x89c - LTC PKHA A 39 Register"]
    #[inline(always)]
    pub fn ltc0_pka_39_mut(&self) -> &mut LTC0_PKA_39 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2204usize) as *mut LTC0_PKA_39) }
    }
    #[doc = "0x89c - LTC PKHA A2 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_7(&self) -> &LTC0_PKA2_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2204usize) as *const LTC0_PKA2_7) }
    }
    #[doc = "0x89c - LTC PKHA A2 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_7_mut(&self) -> &mut LTC0_PKA2_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2204usize) as *mut LTC0_PKA2_7) }
    }
    #[doc = "0x8a0 - LTC PKHA A 40 Register"]
    #[inline(always)]
    pub fn ltc0_pka_40(&self) -> &LTC0_PKA_40 {
        unsafe { &*(((self as *const Self) as *const u8).add(2208usize) as *const LTC0_PKA_40) }
    }
    #[doc = "0x8a0 - LTC PKHA A 40 Register"]
    #[inline(always)]
    pub fn ltc0_pka_40_mut(&self) -> &mut LTC0_PKA_40 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2208usize) as *mut LTC0_PKA_40) }
    }
    #[doc = "0x8a0 - LTC PKHA A2 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_8(&self) -> &LTC0_PKA2_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2208usize) as *const LTC0_PKA2_8) }
    }
    #[doc = "0x8a0 - LTC PKHA A2 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_8_mut(&self) -> &mut LTC0_PKA2_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2208usize) as *mut LTC0_PKA2_8) }
    }
    #[doc = "0x8a4 - LTC PKHA A 41 Register"]
    #[inline(always)]
    pub fn ltc0_pka_41(&self) -> &LTC0_PKA_41 {
        unsafe { &*(((self as *const Self) as *const u8).add(2212usize) as *const LTC0_PKA_41) }
    }
    #[doc = "0x8a4 - LTC PKHA A 41 Register"]
    #[inline(always)]
    pub fn ltc0_pka_41_mut(&self) -> &mut LTC0_PKA_41 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2212usize) as *mut LTC0_PKA_41) }
    }
    #[doc = "0x8a4 - LTC PKHA A2 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_9(&self) -> &LTC0_PKA2_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2212usize) as *const LTC0_PKA2_9) }
    }
    #[doc = "0x8a4 - LTC PKHA A2 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_9_mut(&self) -> &mut LTC0_PKA2_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2212usize) as *mut LTC0_PKA2_9) }
    }
    #[doc = "0x8a8 - LTC PKHA A 42 Register"]
    #[inline(always)]
    pub fn ltc0_pka_42(&self) -> &LTC0_PKA_42 {
        unsafe { &*(((self as *const Self) as *const u8).add(2216usize) as *const LTC0_PKA_42) }
    }
    #[doc = "0x8a8 - LTC PKHA A 42 Register"]
    #[inline(always)]
    pub fn ltc0_pka_42_mut(&self) -> &mut LTC0_PKA_42 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2216usize) as *mut LTC0_PKA_42) }
    }
    #[doc = "0x8a8 - LTC PKHA A2 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_10(&self) -> &LTC0_PKA2_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2216usize) as *const LTC0_PKA2_10) }
    }
    #[doc = "0x8a8 - LTC PKHA A2 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_10_mut(&self) -> &mut LTC0_PKA2_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2216usize) as *mut LTC0_PKA2_10) }
    }
    #[doc = "0x8ac - LTC PKHA A 43 Register"]
    #[inline(always)]
    pub fn ltc0_pka_43(&self) -> &LTC0_PKA_43 {
        unsafe { &*(((self as *const Self) as *const u8).add(2220usize) as *const LTC0_PKA_43) }
    }
    #[doc = "0x8ac - LTC PKHA A 43 Register"]
    #[inline(always)]
    pub fn ltc0_pka_43_mut(&self) -> &mut LTC0_PKA_43 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2220usize) as *mut LTC0_PKA_43) }
    }
    #[doc = "0x8ac - LTC PKHA A2 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_11(&self) -> &LTC0_PKA2_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2220usize) as *const LTC0_PKA2_11) }
    }
    #[doc = "0x8ac - LTC PKHA A2 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_11_mut(&self) -> &mut LTC0_PKA2_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2220usize) as *mut LTC0_PKA2_11) }
    }
    #[doc = "0x8b0 - LTC PKHA A 44 Register"]
    #[inline(always)]
    pub fn ltc0_pka_44(&self) -> &LTC0_PKA_44 {
        unsafe { &*(((self as *const Self) as *const u8).add(2224usize) as *const LTC0_PKA_44) }
    }
    #[doc = "0x8b0 - LTC PKHA A 44 Register"]
    #[inline(always)]
    pub fn ltc0_pka_44_mut(&self) -> &mut LTC0_PKA_44 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2224usize) as *mut LTC0_PKA_44) }
    }
    #[doc = "0x8b0 - LTC PKHA A2 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_12(&self) -> &LTC0_PKA2_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2224usize) as *const LTC0_PKA2_12) }
    }
    #[doc = "0x8b0 - LTC PKHA A2 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_12_mut(&self) -> &mut LTC0_PKA2_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2224usize) as *mut LTC0_PKA2_12) }
    }
    #[doc = "0x8b4 - LTC PKHA A 45 Register"]
    #[inline(always)]
    pub fn ltc0_pka_45(&self) -> &LTC0_PKA_45 {
        unsafe { &*(((self as *const Self) as *const u8).add(2228usize) as *const LTC0_PKA_45) }
    }
    #[doc = "0x8b4 - LTC PKHA A 45 Register"]
    #[inline(always)]
    pub fn ltc0_pka_45_mut(&self) -> &mut LTC0_PKA_45 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2228usize) as *mut LTC0_PKA_45) }
    }
    #[doc = "0x8b4 - LTC PKHA A2 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_13(&self) -> &LTC0_PKA2_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2228usize) as *const LTC0_PKA2_13) }
    }
    #[doc = "0x8b4 - LTC PKHA A2 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_13_mut(&self) -> &mut LTC0_PKA2_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2228usize) as *mut LTC0_PKA2_13) }
    }
    #[doc = "0x8b8 - LTC PKHA A 46 Register"]
    #[inline(always)]
    pub fn ltc0_pka_46(&self) -> &LTC0_PKA_46 {
        unsafe { &*(((self as *const Self) as *const u8).add(2232usize) as *const LTC0_PKA_46) }
    }
    #[doc = "0x8b8 - LTC PKHA A 46 Register"]
    #[inline(always)]
    pub fn ltc0_pka_46_mut(&self) -> &mut LTC0_PKA_46 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2232usize) as *mut LTC0_PKA_46) }
    }
    #[doc = "0x8b8 - LTC PKHA A2 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_14(&self) -> &LTC0_PKA2_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2232usize) as *const LTC0_PKA2_14) }
    }
    #[doc = "0x8b8 - LTC PKHA A2 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_14_mut(&self) -> &mut LTC0_PKA2_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2232usize) as *mut LTC0_PKA2_14) }
    }
    #[doc = "0x8bc - LTC PKHA A 47 Register"]
    #[inline(always)]
    pub fn ltc0_pka_47(&self) -> &LTC0_PKA_47 {
        unsafe { &*(((self as *const Self) as *const u8).add(2236usize) as *const LTC0_PKA_47) }
    }
    #[doc = "0x8bc - LTC PKHA A 47 Register"]
    #[inline(always)]
    pub fn ltc0_pka_47_mut(&self) -> &mut LTC0_PKA_47 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2236usize) as *mut LTC0_PKA_47) }
    }
    #[doc = "0x8bc - LTC PKHA A2 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_15(&self) -> &LTC0_PKA2_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2236usize) as *const LTC0_PKA2_15) }
    }
    #[doc = "0x8bc - LTC PKHA A2 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka2_15_mut(&self) -> &mut LTC0_PKA2_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2236usize) as *mut LTC0_PKA2_15) }
    }
    #[doc = "0x8c0 - LTC PKHA A 48 Register"]
    #[inline(always)]
    pub fn ltc0_pka_48(&self) -> &LTC0_PKA_48 {
        unsafe { &*(((self as *const Self) as *const u8).add(2240usize) as *const LTC0_PKA_48) }
    }
    #[doc = "0x8c0 - LTC PKHA A 48 Register"]
    #[inline(always)]
    pub fn ltc0_pka_48_mut(&self) -> &mut LTC0_PKA_48 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2240usize) as *mut LTC0_PKA_48) }
    }
    #[doc = "0x8c0 - LTC PKHA A3 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_0(&self) -> &LTC0_PKA3_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2240usize) as *const LTC0_PKA3_0) }
    }
    #[doc = "0x8c0 - LTC PKHA A3 0 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_0_mut(&self) -> &mut LTC0_PKA3_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2240usize) as *mut LTC0_PKA3_0) }
    }
    #[doc = "0x8c4 - LTC PKHA A 49 Register"]
    #[inline(always)]
    pub fn ltc0_pka_49(&self) -> &LTC0_PKA_49 {
        unsafe { &*(((self as *const Self) as *const u8).add(2244usize) as *const LTC0_PKA_49) }
    }
    #[doc = "0x8c4 - LTC PKHA A 49 Register"]
    #[inline(always)]
    pub fn ltc0_pka_49_mut(&self) -> &mut LTC0_PKA_49 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2244usize) as *mut LTC0_PKA_49) }
    }
    #[doc = "0x8c4 - LTC PKHA A3 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_1(&self) -> &LTC0_PKA3_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2244usize) as *const LTC0_PKA3_1) }
    }
    #[doc = "0x8c4 - LTC PKHA A3 1 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_1_mut(&self) -> &mut LTC0_PKA3_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2244usize) as *mut LTC0_PKA3_1) }
    }
    #[doc = "0x8c8 - LTC PKHA A 50 Register"]
    #[inline(always)]
    pub fn ltc0_pka_50(&self) -> &LTC0_PKA_50 {
        unsafe { &*(((self as *const Self) as *const u8).add(2248usize) as *const LTC0_PKA_50) }
    }
    #[doc = "0x8c8 - LTC PKHA A 50 Register"]
    #[inline(always)]
    pub fn ltc0_pka_50_mut(&self) -> &mut LTC0_PKA_50 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2248usize) as *mut LTC0_PKA_50) }
    }
    #[doc = "0x8c8 - LTC PKHA A3 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_2(&self) -> &LTC0_PKA3_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2248usize) as *const LTC0_PKA3_2) }
    }
    #[doc = "0x8c8 - LTC PKHA A3 2 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_2_mut(&self) -> &mut LTC0_PKA3_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2248usize) as *mut LTC0_PKA3_2) }
    }
    #[doc = "0x8cc - LTC PKHA A 51 Register"]
    #[inline(always)]
    pub fn ltc0_pka_51(&self) -> &LTC0_PKA_51 {
        unsafe { &*(((self as *const Self) as *const u8).add(2252usize) as *const LTC0_PKA_51) }
    }
    #[doc = "0x8cc - LTC PKHA A 51 Register"]
    #[inline(always)]
    pub fn ltc0_pka_51_mut(&self) -> &mut LTC0_PKA_51 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2252usize) as *mut LTC0_PKA_51) }
    }
    #[doc = "0x8cc - LTC PKHA A3 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_3(&self) -> &LTC0_PKA3_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2252usize) as *const LTC0_PKA3_3) }
    }
    #[doc = "0x8cc - LTC PKHA A3 3 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_3_mut(&self) -> &mut LTC0_PKA3_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2252usize) as *mut LTC0_PKA3_3) }
    }
    #[doc = "0x8d0 - LTC PKHA A 52 Register"]
    #[inline(always)]
    pub fn ltc0_pka_52(&self) -> &LTC0_PKA_52 {
        unsafe { &*(((self as *const Self) as *const u8).add(2256usize) as *const LTC0_PKA_52) }
    }
    #[doc = "0x8d0 - LTC PKHA A 52 Register"]
    #[inline(always)]
    pub fn ltc0_pka_52_mut(&self) -> &mut LTC0_PKA_52 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2256usize) as *mut LTC0_PKA_52) }
    }
    #[doc = "0x8d0 - LTC PKHA A3 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_4(&self) -> &LTC0_PKA3_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2256usize) as *const LTC0_PKA3_4) }
    }
    #[doc = "0x8d0 - LTC PKHA A3 4 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_4_mut(&self) -> &mut LTC0_PKA3_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2256usize) as *mut LTC0_PKA3_4) }
    }
    #[doc = "0x8d4 - LTC PKHA A 53 Register"]
    #[inline(always)]
    pub fn ltc0_pka_53(&self) -> &LTC0_PKA_53 {
        unsafe { &*(((self as *const Self) as *const u8).add(2260usize) as *const LTC0_PKA_53) }
    }
    #[doc = "0x8d4 - LTC PKHA A 53 Register"]
    #[inline(always)]
    pub fn ltc0_pka_53_mut(&self) -> &mut LTC0_PKA_53 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2260usize) as *mut LTC0_PKA_53) }
    }
    #[doc = "0x8d4 - LTC PKHA A3 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_5(&self) -> &LTC0_PKA3_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2260usize) as *const LTC0_PKA3_5) }
    }
    #[doc = "0x8d4 - LTC PKHA A3 5 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_5_mut(&self) -> &mut LTC0_PKA3_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2260usize) as *mut LTC0_PKA3_5) }
    }
    #[doc = "0x8d8 - LTC PKHA A 54 Register"]
    #[inline(always)]
    pub fn ltc0_pka_54(&self) -> &LTC0_PKA_54 {
        unsafe { &*(((self as *const Self) as *const u8).add(2264usize) as *const LTC0_PKA_54) }
    }
    #[doc = "0x8d8 - LTC PKHA A 54 Register"]
    #[inline(always)]
    pub fn ltc0_pka_54_mut(&self) -> &mut LTC0_PKA_54 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2264usize) as *mut LTC0_PKA_54) }
    }
    #[doc = "0x8d8 - LTC PKHA A3 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_6(&self) -> &LTC0_PKA3_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2264usize) as *const LTC0_PKA3_6) }
    }
    #[doc = "0x8d8 - LTC PKHA A3 6 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_6_mut(&self) -> &mut LTC0_PKA3_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2264usize) as *mut LTC0_PKA3_6) }
    }
    #[doc = "0x8dc - LTC PKHA A 55 Register"]
    #[inline(always)]
    pub fn ltc0_pka_55(&self) -> &LTC0_PKA_55 {
        unsafe { &*(((self as *const Self) as *const u8).add(2268usize) as *const LTC0_PKA_55) }
    }
    #[doc = "0x8dc - LTC PKHA A 55 Register"]
    #[inline(always)]
    pub fn ltc0_pka_55_mut(&self) -> &mut LTC0_PKA_55 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2268usize) as *mut LTC0_PKA_55) }
    }
    #[doc = "0x8dc - LTC PKHA A3 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_7(&self) -> &LTC0_PKA3_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2268usize) as *const LTC0_PKA3_7) }
    }
    #[doc = "0x8dc - LTC PKHA A3 7 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_7_mut(&self) -> &mut LTC0_PKA3_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2268usize) as *mut LTC0_PKA3_7) }
    }
    #[doc = "0x8e0 - LTC PKHA A 56 Register"]
    #[inline(always)]
    pub fn ltc0_pka_56(&self) -> &LTC0_PKA_56 {
        unsafe { &*(((self as *const Self) as *const u8).add(2272usize) as *const LTC0_PKA_56) }
    }
    #[doc = "0x8e0 - LTC PKHA A 56 Register"]
    #[inline(always)]
    pub fn ltc0_pka_56_mut(&self) -> &mut LTC0_PKA_56 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2272usize) as *mut LTC0_PKA_56) }
    }
    #[doc = "0x8e0 - LTC PKHA A3 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_8(&self) -> &LTC0_PKA3_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2272usize) as *const LTC0_PKA3_8) }
    }
    #[doc = "0x8e0 - LTC PKHA A3 8 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_8_mut(&self) -> &mut LTC0_PKA3_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2272usize) as *mut LTC0_PKA3_8) }
    }
    #[doc = "0x8e4 - LTC PKHA A 57 Register"]
    #[inline(always)]
    pub fn ltc0_pka_57(&self) -> &LTC0_PKA_57 {
        unsafe { &*(((self as *const Self) as *const u8).add(2276usize) as *const LTC0_PKA_57) }
    }
    #[doc = "0x8e4 - LTC PKHA A 57 Register"]
    #[inline(always)]
    pub fn ltc0_pka_57_mut(&self) -> &mut LTC0_PKA_57 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2276usize) as *mut LTC0_PKA_57) }
    }
    #[doc = "0x8e4 - LTC PKHA A3 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_9(&self) -> &LTC0_PKA3_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2276usize) as *const LTC0_PKA3_9) }
    }
    #[doc = "0x8e4 - LTC PKHA A3 9 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_9_mut(&self) -> &mut LTC0_PKA3_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2276usize) as *mut LTC0_PKA3_9) }
    }
    #[doc = "0x8e8 - LTC PKHA A 58 Register"]
    #[inline(always)]
    pub fn ltc0_pka_58(&self) -> &LTC0_PKA_58 {
        unsafe { &*(((self as *const Self) as *const u8).add(2280usize) as *const LTC0_PKA_58) }
    }
    #[doc = "0x8e8 - LTC PKHA A 58 Register"]
    #[inline(always)]
    pub fn ltc0_pka_58_mut(&self) -> &mut LTC0_PKA_58 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2280usize) as *mut LTC0_PKA_58) }
    }
    #[doc = "0x8e8 - LTC PKHA A3 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_10(&self) -> &LTC0_PKA3_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2280usize) as *const LTC0_PKA3_10) }
    }
    #[doc = "0x8e8 - LTC PKHA A3 10 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_10_mut(&self) -> &mut LTC0_PKA3_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2280usize) as *mut LTC0_PKA3_10) }
    }
    #[doc = "0x8ec - LTC PKHA A 59 Register"]
    #[inline(always)]
    pub fn ltc0_pka_59(&self) -> &LTC0_PKA_59 {
        unsafe { &*(((self as *const Self) as *const u8).add(2284usize) as *const LTC0_PKA_59) }
    }
    #[doc = "0x8ec - LTC PKHA A 59 Register"]
    #[inline(always)]
    pub fn ltc0_pka_59_mut(&self) -> &mut LTC0_PKA_59 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2284usize) as *mut LTC0_PKA_59) }
    }
    #[doc = "0x8ec - LTC PKHA A3 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_11(&self) -> &LTC0_PKA3_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2284usize) as *const LTC0_PKA3_11) }
    }
    #[doc = "0x8ec - LTC PKHA A3 11 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_11_mut(&self) -> &mut LTC0_PKA3_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2284usize) as *mut LTC0_PKA3_11) }
    }
    #[doc = "0x8f0 - LTC PKHA A 60 Register"]
    #[inline(always)]
    pub fn ltc0_pka_60(&self) -> &LTC0_PKA_60 {
        unsafe { &*(((self as *const Self) as *const u8).add(2288usize) as *const LTC0_PKA_60) }
    }
    #[doc = "0x8f0 - LTC PKHA A 60 Register"]
    #[inline(always)]
    pub fn ltc0_pka_60_mut(&self) -> &mut LTC0_PKA_60 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2288usize) as *mut LTC0_PKA_60) }
    }
    #[doc = "0x8f0 - LTC PKHA A3 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_12(&self) -> &LTC0_PKA3_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2288usize) as *const LTC0_PKA3_12) }
    }
    #[doc = "0x8f0 - LTC PKHA A3 12 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_12_mut(&self) -> &mut LTC0_PKA3_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2288usize) as *mut LTC0_PKA3_12) }
    }
    #[doc = "0x8f4 - LTC PKHA A 61 Register"]
    #[inline(always)]
    pub fn ltc0_pka_61(&self) -> &LTC0_PKA_61 {
        unsafe { &*(((self as *const Self) as *const u8).add(2292usize) as *const LTC0_PKA_61) }
    }
    #[doc = "0x8f4 - LTC PKHA A 61 Register"]
    #[inline(always)]
    pub fn ltc0_pka_61_mut(&self) -> &mut LTC0_PKA_61 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2292usize) as *mut LTC0_PKA_61) }
    }
    #[doc = "0x8f4 - LTC PKHA A3 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_13(&self) -> &LTC0_PKA3_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2292usize) as *const LTC0_PKA3_13) }
    }
    #[doc = "0x8f4 - LTC PKHA A3 13 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_13_mut(&self) -> &mut LTC0_PKA3_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2292usize) as *mut LTC0_PKA3_13) }
    }
    #[doc = "0x8f8 - LTC PKHA A 62 Register"]
    #[inline(always)]
    pub fn ltc0_pka_62(&self) -> &LTC0_PKA_62 {
        unsafe { &*(((self as *const Self) as *const u8).add(2296usize) as *const LTC0_PKA_62) }
    }
    #[doc = "0x8f8 - LTC PKHA A 62 Register"]
    #[inline(always)]
    pub fn ltc0_pka_62_mut(&self) -> &mut LTC0_PKA_62 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2296usize) as *mut LTC0_PKA_62) }
    }
    #[doc = "0x8f8 - LTC PKHA A3 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_14(&self) -> &LTC0_PKA3_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2296usize) as *const LTC0_PKA3_14) }
    }
    #[doc = "0x8f8 - LTC PKHA A3 14 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_14_mut(&self) -> &mut LTC0_PKA3_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2296usize) as *mut LTC0_PKA3_14) }
    }
    #[doc = "0x8fc - LTC PKHA A 63 Register"]
    #[inline(always)]
    pub fn ltc0_pka_63(&self) -> &LTC0_PKA_63 {
        unsafe { &*(((self as *const Self) as *const u8).add(2300usize) as *const LTC0_PKA_63) }
    }
    #[doc = "0x8fc - LTC PKHA A 63 Register"]
    #[inline(always)]
    pub fn ltc0_pka_63_mut(&self) -> &mut LTC0_PKA_63 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2300usize) as *mut LTC0_PKA_63) }
    }
    #[doc = "0x8fc - LTC PKHA A3 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_15(&self) -> &LTC0_PKA3_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2300usize) as *const LTC0_PKA3_15) }
    }
    #[doc = "0x8fc - LTC PKHA A3 15 Register"]
    #[inline(always)]
    pub fn ltc0_pka3_15_mut(&self) -> &mut LTC0_PKA3_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2300usize) as *mut LTC0_PKA3_15) }
    }
    #[doc = "0xa00 - LTC PKHA B 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_0(&self) -> &LTC0_PKB_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2560usize) as *const LTC0_PKB_0) }
    }
    #[doc = "0xa00 - LTC PKHA B 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_0_mut(&self) -> &mut LTC0_PKB_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2560usize) as *mut LTC0_PKB_0) }
    }
    #[doc = "0xa00 - LTC PKHA B0 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_0(&self) -> &LTC0_PKB0_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2560usize) as *const LTC0_PKB0_0) }
    }
    #[doc = "0xa00 - LTC PKHA B0 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_0_mut(&self) -> &mut LTC0_PKB0_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2560usize) as *mut LTC0_PKB0_0) }
    }
    #[doc = "0xa04 - LTC PKHA B 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_1(&self) -> &LTC0_PKB_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2564usize) as *const LTC0_PKB_1) }
    }
    #[doc = "0xa04 - LTC PKHA B 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_1_mut(&self) -> &mut LTC0_PKB_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2564usize) as *mut LTC0_PKB_1) }
    }
    #[doc = "0xa04 - LTC PKHA B0 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_1(&self) -> &LTC0_PKB0_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2564usize) as *const LTC0_PKB0_1) }
    }
    #[doc = "0xa04 - LTC PKHA B0 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_1_mut(&self) -> &mut LTC0_PKB0_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2564usize) as *mut LTC0_PKB0_1) }
    }
    #[doc = "0xa08 - LTC PKHA B 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_2(&self) -> &LTC0_PKB_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2568usize) as *const LTC0_PKB_2) }
    }
    #[doc = "0xa08 - LTC PKHA B 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_2_mut(&self) -> &mut LTC0_PKB_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2568usize) as *mut LTC0_PKB_2) }
    }
    #[doc = "0xa08 - LTC PKHA B0 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_2(&self) -> &LTC0_PKB0_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2568usize) as *const LTC0_PKB0_2) }
    }
    #[doc = "0xa08 - LTC PKHA B0 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_2_mut(&self) -> &mut LTC0_PKB0_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2568usize) as *mut LTC0_PKB0_2) }
    }
    #[doc = "0xa0c - LTC PKHA B 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_3(&self) -> &LTC0_PKB_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2572usize) as *const LTC0_PKB_3) }
    }
    #[doc = "0xa0c - LTC PKHA B 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_3_mut(&self) -> &mut LTC0_PKB_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2572usize) as *mut LTC0_PKB_3) }
    }
    #[doc = "0xa0c - LTC PKHA B0 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_3(&self) -> &LTC0_PKB0_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2572usize) as *const LTC0_PKB0_3) }
    }
    #[doc = "0xa0c - LTC PKHA B0 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_3_mut(&self) -> &mut LTC0_PKB0_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2572usize) as *mut LTC0_PKB0_3) }
    }
    #[doc = "0xa10 - LTC PKHA B 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_4(&self) -> &LTC0_PKB_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2576usize) as *const LTC0_PKB_4) }
    }
    #[doc = "0xa10 - LTC PKHA B 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_4_mut(&self) -> &mut LTC0_PKB_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2576usize) as *mut LTC0_PKB_4) }
    }
    #[doc = "0xa10 - LTC PKHA B0 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_4(&self) -> &LTC0_PKB0_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2576usize) as *const LTC0_PKB0_4) }
    }
    #[doc = "0xa10 - LTC PKHA B0 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_4_mut(&self) -> &mut LTC0_PKB0_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2576usize) as *mut LTC0_PKB0_4) }
    }
    #[doc = "0xa14 - LTC PKHA B 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_5(&self) -> &LTC0_PKB_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2580usize) as *const LTC0_PKB_5) }
    }
    #[doc = "0xa14 - LTC PKHA B 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_5_mut(&self) -> &mut LTC0_PKB_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2580usize) as *mut LTC0_PKB_5) }
    }
    #[doc = "0xa14 - LTC PKHA B0 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_5(&self) -> &LTC0_PKB0_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2580usize) as *const LTC0_PKB0_5) }
    }
    #[doc = "0xa14 - LTC PKHA B0 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_5_mut(&self) -> &mut LTC0_PKB0_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2580usize) as *mut LTC0_PKB0_5) }
    }
    #[doc = "0xa18 - LTC PKHA B 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_6(&self) -> &LTC0_PKB_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2584usize) as *const LTC0_PKB_6) }
    }
    #[doc = "0xa18 - LTC PKHA B 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_6_mut(&self) -> &mut LTC0_PKB_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2584usize) as *mut LTC0_PKB_6) }
    }
    #[doc = "0xa18 - LTC PKHA B0 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_6(&self) -> &LTC0_PKB0_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2584usize) as *const LTC0_PKB0_6) }
    }
    #[doc = "0xa18 - LTC PKHA B0 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_6_mut(&self) -> &mut LTC0_PKB0_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2584usize) as *mut LTC0_PKB0_6) }
    }
    #[doc = "0xa1c - LTC PKHA B 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_7(&self) -> &LTC0_PKB_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2588usize) as *const LTC0_PKB_7) }
    }
    #[doc = "0xa1c - LTC PKHA B 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_7_mut(&self) -> &mut LTC0_PKB_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2588usize) as *mut LTC0_PKB_7) }
    }
    #[doc = "0xa1c - LTC PKHA B0 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_7(&self) -> &LTC0_PKB0_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2588usize) as *const LTC0_PKB0_7) }
    }
    #[doc = "0xa1c - LTC PKHA B0 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_7_mut(&self) -> &mut LTC0_PKB0_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2588usize) as *mut LTC0_PKB0_7) }
    }
    #[doc = "0xa20 - LTC PKHA B 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_8(&self) -> &LTC0_PKB_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2592usize) as *const LTC0_PKB_8) }
    }
    #[doc = "0xa20 - LTC PKHA B 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_8_mut(&self) -> &mut LTC0_PKB_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2592usize) as *mut LTC0_PKB_8) }
    }
    #[doc = "0xa20 - LTC PKHA B0 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_8(&self) -> &LTC0_PKB0_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2592usize) as *const LTC0_PKB0_8) }
    }
    #[doc = "0xa20 - LTC PKHA B0 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_8_mut(&self) -> &mut LTC0_PKB0_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2592usize) as *mut LTC0_PKB0_8) }
    }
    #[doc = "0xa24 - LTC PKHA B 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_9(&self) -> &LTC0_PKB_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2596usize) as *const LTC0_PKB_9) }
    }
    #[doc = "0xa24 - LTC PKHA B 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_9_mut(&self) -> &mut LTC0_PKB_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2596usize) as *mut LTC0_PKB_9) }
    }
    #[doc = "0xa24 - LTC PKHA B0 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_9(&self) -> &LTC0_PKB0_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2596usize) as *const LTC0_PKB0_9) }
    }
    #[doc = "0xa24 - LTC PKHA B0 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_9_mut(&self) -> &mut LTC0_PKB0_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2596usize) as *mut LTC0_PKB0_9) }
    }
    #[doc = "0xa28 - LTC PKHA B 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_10(&self) -> &LTC0_PKB_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2600usize) as *const LTC0_PKB_10) }
    }
    #[doc = "0xa28 - LTC PKHA B 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_10_mut(&self) -> &mut LTC0_PKB_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2600usize) as *mut LTC0_PKB_10) }
    }
    #[doc = "0xa28 - LTC PKHA B0 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_10(&self) -> &LTC0_PKB0_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2600usize) as *const LTC0_PKB0_10) }
    }
    #[doc = "0xa28 - LTC PKHA B0 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_10_mut(&self) -> &mut LTC0_PKB0_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2600usize) as *mut LTC0_PKB0_10) }
    }
    #[doc = "0xa2c - LTC PKHA B 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_11(&self) -> &LTC0_PKB_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2604usize) as *const LTC0_PKB_11) }
    }
    #[doc = "0xa2c - LTC PKHA B 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_11_mut(&self) -> &mut LTC0_PKB_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2604usize) as *mut LTC0_PKB_11) }
    }
    #[doc = "0xa2c - LTC PKHA B0 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_11(&self) -> &LTC0_PKB0_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2604usize) as *const LTC0_PKB0_11) }
    }
    #[doc = "0xa2c - LTC PKHA B0 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_11_mut(&self) -> &mut LTC0_PKB0_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2604usize) as *mut LTC0_PKB0_11) }
    }
    #[doc = "0xa30 - LTC PKHA B 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_12(&self) -> &LTC0_PKB_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2608usize) as *const LTC0_PKB_12) }
    }
    #[doc = "0xa30 - LTC PKHA B 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_12_mut(&self) -> &mut LTC0_PKB_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2608usize) as *mut LTC0_PKB_12) }
    }
    #[doc = "0xa30 - LTC PKHA B0 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_12(&self) -> &LTC0_PKB0_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2608usize) as *const LTC0_PKB0_12) }
    }
    #[doc = "0xa30 - LTC PKHA B0 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_12_mut(&self) -> &mut LTC0_PKB0_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2608usize) as *mut LTC0_PKB0_12) }
    }
    #[doc = "0xa34 - LTC PKHA B 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_13(&self) -> &LTC0_PKB_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2612usize) as *const LTC0_PKB_13) }
    }
    #[doc = "0xa34 - LTC PKHA B 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_13_mut(&self) -> &mut LTC0_PKB_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2612usize) as *mut LTC0_PKB_13) }
    }
    #[doc = "0xa34 - LTC PKHA B0 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_13(&self) -> &LTC0_PKB0_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2612usize) as *const LTC0_PKB0_13) }
    }
    #[doc = "0xa34 - LTC PKHA B0 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_13_mut(&self) -> &mut LTC0_PKB0_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2612usize) as *mut LTC0_PKB0_13) }
    }
    #[doc = "0xa38 - LTC PKHA B 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_14(&self) -> &LTC0_PKB_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2616usize) as *const LTC0_PKB_14) }
    }
    #[doc = "0xa38 - LTC PKHA B 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_14_mut(&self) -> &mut LTC0_PKB_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2616usize) as *mut LTC0_PKB_14) }
    }
    #[doc = "0xa38 - LTC PKHA B0 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_14(&self) -> &LTC0_PKB0_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2616usize) as *const LTC0_PKB0_14) }
    }
    #[doc = "0xa38 - LTC PKHA B0 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_14_mut(&self) -> &mut LTC0_PKB0_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2616usize) as *mut LTC0_PKB0_14) }
    }
    #[doc = "0xa3c - LTC PKHA B 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_15(&self) -> &LTC0_PKB_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2620usize) as *const LTC0_PKB_15) }
    }
    #[doc = "0xa3c - LTC PKHA B 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_15_mut(&self) -> &mut LTC0_PKB_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2620usize) as *mut LTC0_PKB_15) }
    }
    #[doc = "0xa3c - LTC PKHA B0 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_15(&self) -> &LTC0_PKB0_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2620usize) as *const LTC0_PKB0_15) }
    }
    #[doc = "0xa3c - LTC PKHA B0 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb0_15_mut(&self) -> &mut LTC0_PKB0_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2620usize) as *mut LTC0_PKB0_15) }
    }
    #[doc = "0xa40 - LTC PKHA B 16 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_16(&self) -> &LTC0_PKB_16 {
        unsafe { &*(((self as *const Self) as *const u8).add(2624usize) as *const LTC0_PKB_16) }
    }
    #[doc = "0xa40 - LTC PKHA B 16 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_16_mut(&self) -> &mut LTC0_PKB_16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2624usize) as *mut LTC0_PKB_16) }
    }
    #[doc = "0xa40 - LTC PKHA B1 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_0(&self) -> &LTC0_PKB1_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2624usize) as *const LTC0_PKB1_0) }
    }
    #[doc = "0xa40 - LTC PKHA B1 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_0_mut(&self) -> &mut LTC0_PKB1_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2624usize) as *mut LTC0_PKB1_0) }
    }
    #[doc = "0xa44 - LTC PKHA B 17 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_17(&self) -> &LTC0_PKB_17 {
        unsafe { &*(((self as *const Self) as *const u8).add(2628usize) as *const LTC0_PKB_17) }
    }
    #[doc = "0xa44 - LTC PKHA B 17 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_17_mut(&self) -> &mut LTC0_PKB_17 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2628usize) as *mut LTC0_PKB_17) }
    }
    #[doc = "0xa44 - LTC PKHA B1 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_1(&self) -> &LTC0_PKB1_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2628usize) as *const LTC0_PKB1_1) }
    }
    #[doc = "0xa44 - LTC PKHA B1 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_1_mut(&self) -> &mut LTC0_PKB1_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2628usize) as *mut LTC0_PKB1_1) }
    }
    #[doc = "0xa48 - LTC PKHA B 18 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_18(&self) -> &LTC0_PKB_18 {
        unsafe { &*(((self as *const Self) as *const u8).add(2632usize) as *const LTC0_PKB_18) }
    }
    #[doc = "0xa48 - LTC PKHA B 18 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_18_mut(&self) -> &mut LTC0_PKB_18 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2632usize) as *mut LTC0_PKB_18) }
    }
    #[doc = "0xa48 - LTC PKHA B1 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_2(&self) -> &LTC0_PKB1_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2632usize) as *const LTC0_PKB1_2) }
    }
    #[doc = "0xa48 - LTC PKHA B1 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_2_mut(&self) -> &mut LTC0_PKB1_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2632usize) as *mut LTC0_PKB1_2) }
    }
    #[doc = "0xa4c - LTC PKHA B 19 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_19(&self) -> &LTC0_PKB_19 {
        unsafe { &*(((self as *const Self) as *const u8).add(2636usize) as *const LTC0_PKB_19) }
    }
    #[doc = "0xa4c - LTC PKHA B 19 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_19_mut(&self) -> &mut LTC0_PKB_19 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2636usize) as *mut LTC0_PKB_19) }
    }
    #[doc = "0xa4c - LTC PKHA B1 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_3(&self) -> &LTC0_PKB1_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2636usize) as *const LTC0_PKB1_3) }
    }
    #[doc = "0xa4c - LTC PKHA B1 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_3_mut(&self) -> &mut LTC0_PKB1_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2636usize) as *mut LTC0_PKB1_3) }
    }
    #[doc = "0xa50 - LTC PKHA B 20 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_20(&self) -> &LTC0_PKB_20 {
        unsafe { &*(((self as *const Self) as *const u8).add(2640usize) as *const LTC0_PKB_20) }
    }
    #[doc = "0xa50 - LTC PKHA B 20 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_20_mut(&self) -> &mut LTC0_PKB_20 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2640usize) as *mut LTC0_PKB_20) }
    }
    #[doc = "0xa50 - LTC PKHA B1 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_4(&self) -> &LTC0_PKB1_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2640usize) as *const LTC0_PKB1_4) }
    }
    #[doc = "0xa50 - LTC PKHA B1 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_4_mut(&self) -> &mut LTC0_PKB1_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2640usize) as *mut LTC0_PKB1_4) }
    }
    #[doc = "0xa54 - LTC PKHA B 21 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_21(&self) -> &LTC0_PKB_21 {
        unsafe { &*(((self as *const Self) as *const u8).add(2644usize) as *const LTC0_PKB_21) }
    }
    #[doc = "0xa54 - LTC PKHA B 21 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_21_mut(&self) -> &mut LTC0_PKB_21 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2644usize) as *mut LTC0_PKB_21) }
    }
    #[doc = "0xa54 - LTC PKHA B1 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_5(&self) -> &LTC0_PKB1_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2644usize) as *const LTC0_PKB1_5) }
    }
    #[doc = "0xa54 - LTC PKHA B1 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_5_mut(&self) -> &mut LTC0_PKB1_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2644usize) as *mut LTC0_PKB1_5) }
    }
    #[doc = "0xa58 - LTC PKHA B 22 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_22(&self) -> &LTC0_PKB_22 {
        unsafe { &*(((self as *const Self) as *const u8).add(2648usize) as *const LTC0_PKB_22) }
    }
    #[doc = "0xa58 - LTC PKHA B 22 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_22_mut(&self) -> &mut LTC0_PKB_22 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2648usize) as *mut LTC0_PKB_22) }
    }
    #[doc = "0xa58 - LTC PKHA B1 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_6(&self) -> &LTC0_PKB1_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2648usize) as *const LTC0_PKB1_6) }
    }
    #[doc = "0xa58 - LTC PKHA B1 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_6_mut(&self) -> &mut LTC0_PKB1_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2648usize) as *mut LTC0_PKB1_6) }
    }
    #[doc = "0xa5c - LTC PKHA B 23 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_23(&self) -> &LTC0_PKB_23 {
        unsafe { &*(((self as *const Self) as *const u8).add(2652usize) as *const LTC0_PKB_23) }
    }
    #[doc = "0xa5c - LTC PKHA B 23 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_23_mut(&self) -> &mut LTC0_PKB_23 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2652usize) as *mut LTC0_PKB_23) }
    }
    #[doc = "0xa5c - LTC PKHA B1 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_7(&self) -> &LTC0_PKB1_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2652usize) as *const LTC0_PKB1_7) }
    }
    #[doc = "0xa5c - LTC PKHA B1 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_7_mut(&self) -> &mut LTC0_PKB1_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2652usize) as *mut LTC0_PKB1_7) }
    }
    #[doc = "0xa60 - LTC PKHA B 24 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_24(&self) -> &LTC0_PKB_24 {
        unsafe { &*(((self as *const Self) as *const u8).add(2656usize) as *const LTC0_PKB_24) }
    }
    #[doc = "0xa60 - LTC PKHA B 24 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_24_mut(&self) -> &mut LTC0_PKB_24 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2656usize) as *mut LTC0_PKB_24) }
    }
    #[doc = "0xa60 - LTC PKHA B1 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_8(&self) -> &LTC0_PKB1_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2656usize) as *const LTC0_PKB1_8) }
    }
    #[doc = "0xa60 - LTC PKHA B1 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_8_mut(&self) -> &mut LTC0_PKB1_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2656usize) as *mut LTC0_PKB1_8) }
    }
    #[doc = "0xa64 - LTC PKHA B 25 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_25(&self) -> &LTC0_PKB_25 {
        unsafe { &*(((self as *const Self) as *const u8).add(2660usize) as *const LTC0_PKB_25) }
    }
    #[doc = "0xa64 - LTC PKHA B 25 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_25_mut(&self) -> &mut LTC0_PKB_25 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2660usize) as *mut LTC0_PKB_25) }
    }
    #[doc = "0xa64 - LTC PKHA B1 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_9(&self) -> &LTC0_PKB1_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2660usize) as *const LTC0_PKB1_9) }
    }
    #[doc = "0xa64 - LTC PKHA B1 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_9_mut(&self) -> &mut LTC0_PKB1_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2660usize) as *mut LTC0_PKB1_9) }
    }
    #[doc = "0xa68 - LTC PKHA B 26 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_26(&self) -> &LTC0_PKB_26 {
        unsafe { &*(((self as *const Self) as *const u8).add(2664usize) as *const LTC0_PKB_26) }
    }
    #[doc = "0xa68 - LTC PKHA B 26 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_26_mut(&self) -> &mut LTC0_PKB_26 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2664usize) as *mut LTC0_PKB_26) }
    }
    #[doc = "0xa68 - LTC PKHA B1 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_10(&self) -> &LTC0_PKB1_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2664usize) as *const LTC0_PKB1_10) }
    }
    #[doc = "0xa68 - LTC PKHA B1 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_10_mut(&self) -> &mut LTC0_PKB1_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2664usize) as *mut LTC0_PKB1_10) }
    }
    #[doc = "0xa6c - LTC PKHA B 27 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_27(&self) -> &LTC0_PKB_27 {
        unsafe { &*(((self as *const Self) as *const u8).add(2668usize) as *const LTC0_PKB_27) }
    }
    #[doc = "0xa6c - LTC PKHA B 27 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_27_mut(&self) -> &mut LTC0_PKB_27 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2668usize) as *mut LTC0_PKB_27) }
    }
    #[doc = "0xa6c - LTC PKHA B1 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_11(&self) -> &LTC0_PKB1_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2668usize) as *const LTC0_PKB1_11) }
    }
    #[doc = "0xa6c - LTC PKHA B1 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_11_mut(&self) -> &mut LTC0_PKB1_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2668usize) as *mut LTC0_PKB1_11) }
    }
    #[doc = "0xa70 - LTC PKHA B 28 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_28(&self) -> &LTC0_PKB_28 {
        unsafe { &*(((self as *const Self) as *const u8).add(2672usize) as *const LTC0_PKB_28) }
    }
    #[doc = "0xa70 - LTC PKHA B 28 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_28_mut(&self) -> &mut LTC0_PKB_28 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2672usize) as *mut LTC0_PKB_28) }
    }
    #[doc = "0xa70 - LTC PKHA B1 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_12(&self) -> &LTC0_PKB1_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2672usize) as *const LTC0_PKB1_12) }
    }
    #[doc = "0xa70 - LTC PKHA B1 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_12_mut(&self) -> &mut LTC0_PKB1_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2672usize) as *mut LTC0_PKB1_12) }
    }
    #[doc = "0xa74 - LTC PKHA B 29 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_29(&self) -> &LTC0_PKB_29 {
        unsafe { &*(((self as *const Self) as *const u8).add(2676usize) as *const LTC0_PKB_29) }
    }
    #[doc = "0xa74 - LTC PKHA B 29 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_29_mut(&self) -> &mut LTC0_PKB_29 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2676usize) as *mut LTC0_PKB_29) }
    }
    #[doc = "0xa74 - LTC PKHA B1 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_13(&self) -> &LTC0_PKB1_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2676usize) as *const LTC0_PKB1_13) }
    }
    #[doc = "0xa74 - LTC PKHA B1 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_13_mut(&self) -> &mut LTC0_PKB1_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2676usize) as *mut LTC0_PKB1_13) }
    }
    #[doc = "0xa78 - LTC PKHA B 30 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_30(&self) -> &LTC0_PKB_30 {
        unsafe { &*(((self as *const Self) as *const u8).add(2680usize) as *const LTC0_PKB_30) }
    }
    #[doc = "0xa78 - LTC PKHA B 30 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_30_mut(&self) -> &mut LTC0_PKB_30 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2680usize) as *mut LTC0_PKB_30) }
    }
    #[doc = "0xa78 - LTC PKHA B1 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_14(&self) -> &LTC0_PKB1_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2680usize) as *const LTC0_PKB1_14) }
    }
    #[doc = "0xa78 - LTC PKHA B1 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_14_mut(&self) -> &mut LTC0_PKB1_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2680usize) as *mut LTC0_PKB1_14) }
    }
    #[doc = "0xa7c - LTC PKHA B 31 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_31(&self) -> &LTC0_PKB_31 {
        unsafe { &*(((self as *const Self) as *const u8).add(2684usize) as *const LTC0_PKB_31) }
    }
    #[doc = "0xa7c - LTC PKHA B 31 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_31_mut(&self) -> &mut LTC0_PKB_31 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2684usize) as *mut LTC0_PKB_31) }
    }
    #[doc = "0xa7c - LTC PKHA B1 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_15(&self) -> &LTC0_PKB1_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2684usize) as *const LTC0_PKB1_15) }
    }
    #[doc = "0xa7c - LTC PKHA B1 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb1_15_mut(&self) -> &mut LTC0_PKB1_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2684usize) as *mut LTC0_PKB1_15) }
    }
    #[doc = "0xa80 - LTC PKHA B 32 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_32(&self) -> &LTC0_PKB_32 {
        unsafe { &*(((self as *const Self) as *const u8).add(2688usize) as *const LTC0_PKB_32) }
    }
    #[doc = "0xa80 - LTC PKHA B 32 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_32_mut(&self) -> &mut LTC0_PKB_32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2688usize) as *mut LTC0_PKB_32) }
    }
    #[doc = "0xa80 - LTC PKHA B2 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_0(&self) -> &LTC0_PKB2_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2688usize) as *const LTC0_PKB2_0) }
    }
    #[doc = "0xa80 - LTC PKHA B2 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_0_mut(&self) -> &mut LTC0_PKB2_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2688usize) as *mut LTC0_PKB2_0) }
    }
    #[doc = "0xa84 - LTC PKHA B 33 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_33(&self) -> &LTC0_PKB_33 {
        unsafe { &*(((self as *const Self) as *const u8).add(2692usize) as *const LTC0_PKB_33) }
    }
    #[doc = "0xa84 - LTC PKHA B 33 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_33_mut(&self) -> &mut LTC0_PKB_33 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2692usize) as *mut LTC0_PKB_33) }
    }
    #[doc = "0xa84 - LTC PKHA B2 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_1(&self) -> &LTC0_PKB2_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2692usize) as *const LTC0_PKB2_1) }
    }
    #[doc = "0xa84 - LTC PKHA B2 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_1_mut(&self) -> &mut LTC0_PKB2_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2692usize) as *mut LTC0_PKB2_1) }
    }
    #[doc = "0xa88 - LTC PKHA B 34 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_34(&self) -> &LTC0_PKB_34 {
        unsafe { &*(((self as *const Self) as *const u8).add(2696usize) as *const LTC0_PKB_34) }
    }
    #[doc = "0xa88 - LTC PKHA B 34 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_34_mut(&self) -> &mut LTC0_PKB_34 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2696usize) as *mut LTC0_PKB_34) }
    }
    #[doc = "0xa88 - LTC PKHA B2 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_2(&self) -> &LTC0_PKB2_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2696usize) as *const LTC0_PKB2_2) }
    }
    #[doc = "0xa88 - LTC PKHA B2 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_2_mut(&self) -> &mut LTC0_PKB2_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2696usize) as *mut LTC0_PKB2_2) }
    }
    #[doc = "0xa8c - LTC PKHA B 35 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_35(&self) -> &LTC0_PKB_35 {
        unsafe { &*(((self as *const Self) as *const u8).add(2700usize) as *const LTC0_PKB_35) }
    }
    #[doc = "0xa8c - LTC PKHA B 35 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_35_mut(&self) -> &mut LTC0_PKB_35 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2700usize) as *mut LTC0_PKB_35) }
    }
    #[doc = "0xa8c - LTC PKHA B2 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_3(&self) -> &LTC0_PKB2_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2700usize) as *const LTC0_PKB2_3) }
    }
    #[doc = "0xa8c - LTC PKHA B2 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_3_mut(&self) -> &mut LTC0_PKB2_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2700usize) as *mut LTC0_PKB2_3) }
    }
    #[doc = "0xa90 - LTC PKHA B 36 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_36(&self) -> &LTC0_PKB_36 {
        unsafe { &*(((self as *const Self) as *const u8).add(2704usize) as *const LTC0_PKB_36) }
    }
    #[doc = "0xa90 - LTC PKHA B 36 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_36_mut(&self) -> &mut LTC0_PKB_36 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2704usize) as *mut LTC0_PKB_36) }
    }
    #[doc = "0xa90 - LTC PKHA B2 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_4(&self) -> &LTC0_PKB2_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2704usize) as *const LTC0_PKB2_4) }
    }
    #[doc = "0xa90 - LTC PKHA B2 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_4_mut(&self) -> &mut LTC0_PKB2_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2704usize) as *mut LTC0_PKB2_4) }
    }
    #[doc = "0xa94 - LTC PKHA B 37 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_37(&self) -> &LTC0_PKB_37 {
        unsafe { &*(((self as *const Self) as *const u8).add(2708usize) as *const LTC0_PKB_37) }
    }
    #[doc = "0xa94 - LTC PKHA B 37 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_37_mut(&self) -> &mut LTC0_PKB_37 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2708usize) as *mut LTC0_PKB_37) }
    }
    #[doc = "0xa94 - LTC PKHA B2 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_5(&self) -> &LTC0_PKB2_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2708usize) as *const LTC0_PKB2_5) }
    }
    #[doc = "0xa94 - LTC PKHA B2 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_5_mut(&self) -> &mut LTC0_PKB2_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2708usize) as *mut LTC0_PKB2_5) }
    }
    #[doc = "0xa98 - LTC PKHA B 38 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_38(&self) -> &LTC0_PKB_38 {
        unsafe { &*(((self as *const Self) as *const u8).add(2712usize) as *const LTC0_PKB_38) }
    }
    #[doc = "0xa98 - LTC PKHA B 38 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_38_mut(&self) -> &mut LTC0_PKB_38 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2712usize) as *mut LTC0_PKB_38) }
    }
    #[doc = "0xa98 - LTC PKHA B2 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_6(&self) -> &LTC0_PKB2_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2712usize) as *const LTC0_PKB2_6) }
    }
    #[doc = "0xa98 - LTC PKHA B2 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_6_mut(&self) -> &mut LTC0_PKB2_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2712usize) as *mut LTC0_PKB2_6) }
    }
    #[doc = "0xa9c - LTC PKHA B 39 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_39(&self) -> &LTC0_PKB_39 {
        unsafe { &*(((self as *const Self) as *const u8).add(2716usize) as *const LTC0_PKB_39) }
    }
    #[doc = "0xa9c - LTC PKHA B 39 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_39_mut(&self) -> &mut LTC0_PKB_39 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2716usize) as *mut LTC0_PKB_39) }
    }
    #[doc = "0xa9c - LTC PKHA B2 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_7(&self) -> &LTC0_PKB2_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2716usize) as *const LTC0_PKB2_7) }
    }
    #[doc = "0xa9c - LTC PKHA B2 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_7_mut(&self) -> &mut LTC0_PKB2_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2716usize) as *mut LTC0_PKB2_7) }
    }
    #[doc = "0xaa0 - LTC PKHA B 40 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_40(&self) -> &LTC0_PKB_40 {
        unsafe { &*(((self as *const Self) as *const u8).add(2720usize) as *const LTC0_PKB_40) }
    }
    #[doc = "0xaa0 - LTC PKHA B 40 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_40_mut(&self) -> &mut LTC0_PKB_40 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2720usize) as *mut LTC0_PKB_40) }
    }
    #[doc = "0xaa0 - LTC PKHA B2 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_8(&self) -> &LTC0_PKB2_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2720usize) as *const LTC0_PKB2_8) }
    }
    #[doc = "0xaa0 - LTC PKHA B2 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_8_mut(&self) -> &mut LTC0_PKB2_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2720usize) as *mut LTC0_PKB2_8) }
    }
    #[doc = "0xaa4 - LTC PKHA B 41 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_41(&self) -> &LTC0_PKB_41 {
        unsafe { &*(((self as *const Self) as *const u8).add(2724usize) as *const LTC0_PKB_41) }
    }
    #[doc = "0xaa4 - LTC PKHA B 41 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_41_mut(&self) -> &mut LTC0_PKB_41 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2724usize) as *mut LTC0_PKB_41) }
    }
    #[doc = "0xaa4 - LTC PKHA B2 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_9(&self) -> &LTC0_PKB2_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2724usize) as *const LTC0_PKB2_9) }
    }
    #[doc = "0xaa4 - LTC PKHA B2 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_9_mut(&self) -> &mut LTC0_PKB2_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2724usize) as *mut LTC0_PKB2_9) }
    }
    #[doc = "0xaa8 - LTC PKHA B 42 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_42(&self) -> &LTC0_PKB_42 {
        unsafe { &*(((self as *const Self) as *const u8).add(2728usize) as *const LTC0_PKB_42) }
    }
    #[doc = "0xaa8 - LTC PKHA B 42 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_42_mut(&self) -> &mut LTC0_PKB_42 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2728usize) as *mut LTC0_PKB_42) }
    }
    #[doc = "0xaa8 - LTC PKHA B2 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_10(&self) -> &LTC0_PKB2_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2728usize) as *const LTC0_PKB2_10) }
    }
    #[doc = "0xaa8 - LTC PKHA B2 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_10_mut(&self) -> &mut LTC0_PKB2_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2728usize) as *mut LTC0_PKB2_10) }
    }
    #[doc = "0xaac - LTC PKHA B 43 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_43(&self) -> &LTC0_PKB_43 {
        unsafe { &*(((self as *const Self) as *const u8).add(2732usize) as *const LTC0_PKB_43) }
    }
    #[doc = "0xaac - LTC PKHA B 43 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_43_mut(&self) -> &mut LTC0_PKB_43 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2732usize) as *mut LTC0_PKB_43) }
    }
    #[doc = "0xaac - LTC PKHA B2 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_11(&self) -> &LTC0_PKB2_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2732usize) as *const LTC0_PKB2_11) }
    }
    #[doc = "0xaac - LTC PKHA B2 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_11_mut(&self) -> &mut LTC0_PKB2_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2732usize) as *mut LTC0_PKB2_11) }
    }
    #[doc = "0xab0 - LTC PKHA B 44 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_44(&self) -> &LTC0_PKB_44 {
        unsafe { &*(((self as *const Self) as *const u8).add(2736usize) as *const LTC0_PKB_44) }
    }
    #[doc = "0xab0 - LTC PKHA B 44 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_44_mut(&self) -> &mut LTC0_PKB_44 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2736usize) as *mut LTC0_PKB_44) }
    }
    #[doc = "0xab0 - LTC PKHA B2 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_12(&self) -> &LTC0_PKB2_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2736usize) as *const LTC0_PKB2_12) }
    }
    #[doc = "0xab0 - LTC PKHA B2 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_12_mut(&self) -> &mut LTC0_PKB2_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2736usize) as *mut LTC0_PKB2_12) }
    }
    #[doc = "0xab4 - LTC PKHA B 45 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_45(&self) -> &LTC0_PKB_45 {
        unsafe { &*(((self as *const Self) as *const u8).add(2740usize) as *const LTC0_PKB_45) }
    }
    #[doc = "0xab4 - LTC PKHA B 45 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_45_mut(&self) -> &mut LTC0_PKB_45 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2740usize) as *mut LTC0_PKB_45) }
    }
    #[doc = "0xab4 - LTC PKHA B2 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_13(&self) -> &LTC0_PKB2_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2740usize) as *const LTC0_PKB2_13) }
    }
    #[doc = "0xab4 - LTC PKHA B2 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_13_mut(&self) -> &mut LTC0_PKB2_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2740usize) as *mut LTC0_PKB2_13) }
    }
    #[doc = "0xab8 - LTC PKHA B 46 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_46(&self) -> &LTC0_PKB_46 {
        unsafe { &*(((self as *const Self) as *const u8).add(2744usize) as *const LTC0_PKB_46) }
    }
    #[doc = "0xab8 - LTC PKHA B 46 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_46_mut(&self) -> &mut LTC0_PKB_46 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2744usize) as *mut LTC0_PKB_46) }
    }
    #[doc = "0xab8 - LTC PKHA B2 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_14(&self) -> &LTC0_PKB2_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2744usize) as *const LTC0_PKB2_14) }
    }
    #[doc = "0xab8 - LTC PKHA B2 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_14_mut(&self) -> &mut LTC0_PKB2_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2744usize) as *mut LTC0_PKB2_14) }
    }
    #[doc = "0xabc - LTC PKHA B 47 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_47(&self) -> &LTC0_PKB_47 {
        unsafe { &*(((self as *const Self) as *const u8).add(2748usize) as *const LTC0_PKB_47) }
    }
    #[doc = "0xabc - LTC PKHA B 47 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_47_mut(&self) -> &mut LTC0_PKB_47 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2748usize) as *mut LTC0_PKB_47) }
    }
    #[doc = "0xabc - LTC PKHA B2 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_15(&self) -> &LTC0_PKB2_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2748usize) as *const LTC0_PKB2_15) }
    }
    #[doc = "0xabc - LTC PKHA B2 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb2_15_mut(&self) -> &mut LTC0_PKB2_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2748usize) as *mut LTC0_PKB2_15) }
    }
    #[doc = "0xac0 - LTC PKHA B 48 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_48(&self) -> &LTC0_PKB_48 {
        unsafe { &*(((self as *const Self) as *const u8).add(2752usize) as *const LTC0_PKB_48) }
    }
    #[doc = "0xac0 - LTC PKHA B 48 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_48_mut(&self) -> &mut LTC0_PKB_48 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2752usize) as *mut LTC0_PKB_48) }
    }
    #[doc = "0xac0 - LTC PKHA B3 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_0(&self) -> &LTC0_PKB3_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(2752usize) as *const LTC0_PKB3_0) }
    }
    #[doc = "0xac0 - LTC PKHA B3 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_0_mut(&self) -> &mut LTC0_PKB3_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2752usize) as *mut LTC0_PKB3_0) }
    }
    #[doc = "0xac4 - LTC PKHA B 49 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_49(&self) -> &LTC0_PKB_49 {
        unsafe { &*(((self as *const Self) as *const u8).add(2756usize) as *const LTC0_PKB_49) }
    }
    #[doc = "0xac4 - LTC PKHA B 49 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_49_mut(&self) -> &mut LTC0_PKB_49 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2756usize) as *mut LTC0_PKB_49) }
    }
    #[doc = "0xac4 - LTC PKHA B3 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_1(&self) -> &LTC0_PKB3_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(2756usize) as *const LTC0_PKB3_1) }
    }
    #[doc = "0xac4 - LTC PKHA B3 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_1_mut(&self) -> &mut LTC0_PKB3_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2756usize) as *mut LTC0_PKB3_1) }
    }
    #[doc = "0xac8 - LTC PKHA B 50 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_50(&self) -> &LTC0_PKB_50 {
        unsafe { &*(((self as *const Self) as *const u8).add(2760usize) as *const LTC0_PKB_50) }
    }
    #[doc = "0xac8 - LTC PKHA B 50 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_50_mut(&self) -> &mut LTC0_PKB_50 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2760usize) as *mut LTC0_PKB_50) }
    }
    #[doc = "0xac8 - LTC PKHA B3 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_2(&self) -> &LTC0_PKB3_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(2760usize) as *const LTC0_PKB3_2) }
    }
    #[doc = "0xac8 - LTC PKHA B3 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_2_mut(&self) -> &mut LTC0_PKB3_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2760usize) as *mut LTC0_PKB3_2) }
    }
    #[doc = "0xacc - LTC PKHA B 51 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_51(&self) -> &LTC0_PKB_51 {
        unsafe { &*(((self as *const Self) as *const u8).add(2764usize) as *const LTC0_PKB_51) }
    }
    #[doc = "0xacc - LTC PKHA B 51 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_51_mut(&self) -> &mut LTC0_PKB_51 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2764usize) as *mut LTC0_PKB_51) }
    }
    #[doc = "0xacc - LTC PKHA B3 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_3(&self) -> &LTC0_PKB3_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(2764usize) as *const LTC0_PKB3_3) }
    }
    #[doc = "0xacc - LTC PKHA B3 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_3_mut(&self) -> &mut LTC0_PKB3_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2764usize) as *mut LTC0_PKB3_3) }
    }
    #[doc = "0xad0 - LTC PKHA B 52 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_52(&self) -> &LTC0_PKB_52 {
        unsafe { &*(((self as *const Self) as *const u8).add(2768usize) as *const LTC0_PKB_52) }
    }
    #[doc = "0xad0 - LTC PKHA B 52 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_52_mut(&self) -> &mut LTC0_PKB_52 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2768usize) as *mut LTC0_PKB_52) }
    }
    #[doc = "0xad0 - LTC PKHA B3 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_4(&self) -> &LTC0_PKB3_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(2768usize) as *const LTC0_PKB3_4) }
    }
    #[doc = "0xad0 - LTC PKHA B3 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_4_mut(&self) -> &mut LTC0_PKB3_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2768usize) as *mut LTC0_PKB3_4) }
    }
    #[doc = "0xad4 - LTC PKHA B 53 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_53(&self) -> &LTC0_PKB_53 {
        unsafe { &*(((self as *const Self) as *const u8).add(2772usize) as *const LTC0_PKB_53) }
    }
    #[doc = "0xad4 - LTC PKHA B 53 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_53_mut(&self) -> &mut LTC0_PKB_53 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2772usize) as *mut LTC0_PKB_53) }
    }
    #[doc = "0xad4 - LTC PKHA B3 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_5(&self) -> &LTC0_PKB3_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(2772usize) as *const LTC0_PKB3_5) }
    }
    #[doc = "0xad4 - LTC PKHA B3 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_5_mut(&self) -> &mut LTC0_PKB3_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2772usize) as *mut LTC0_PKB3_5) }
    }
    #[doc = "0xad8 - LTC PKHA B 54 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_54(&self) -> &LTC0_PKB_54 {
        unsafe { &*(((self as *const Self) as *const u8).add(2776usize) as *const LTC0_PKB_54) }
    }
    #[doc = "0xad8 - LTC PKHA B 54 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_54_mut(&self) -> &mut LTC0_PKB_54 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2776usize) as *mut LTC0_PKB_54) }
    }
    #[doc = "0xad8 - LTC PKHA B3 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_6(&self) -> &LTC0_PKB3_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(2776usize) as *const LTC0_PKB3_6) }
    }
    #[doc = "0xad8 - LTC PKHA B3 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_6_mut(&self) -> &mut LTC0_PKB3_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2776usize) as *mut LTC0_PKB3_6) }
    }
    #[doc = "0xadc - LTC PKHA B 55 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_55(&self) -> &LTC0_PKB_55 {
        unsafe { &*(((self as *const Self) as *const u8).add(2780usize) as *const LTC0_PKB_55) }
    }
    #[doc = "0xadc - LTC PKHA B 55 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_55_mut(&self) -> &mut LTC0_PKB_55 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2780usize) as *mut LTC0_PKB_55) }
    }
    #[doc = "0xadc - LTC PKHA B3 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_7(&self) -> &LTC0_PKB3_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(2780usize) as *const LTC0_PKB3_7) }
    }
    #[doc = "0xadc - LTC PKHA B3 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_7_mut(&self) -> &mut LTC0_PKB3_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2780usize) as *mut LTC0_PKB3_7) }
    }
    #[doc = "0xae0 - LTC PKHA B 56 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_56(&self) -> &LTC0_PKB_56 {
        unsafe { &*(((self as *const Self) as *const u8).add(2784usize) as *const LTC0_PKB_56) }
    }
    #[doc = "0xae0 - LTC PKHA B 56 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_56_mut(&self) -> &mut LTC0_PKB_56 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2784usize) as *mut LTC0_PKB_56) }
    }
    #[doc = "0xae0 - LTC PKHA B3 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_8(&self) -> &LTC0_PKB3_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(2784usize) as *const LTC0_PKB3_8) }
    }
    #[doc = "0xae0 - LTC PKHA B3 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_8_mut(&self) -> &mut LTC0_PKB3_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2784usize) as *mut LTC0_PKB3_8) }
    }
    #[doc = "0xae4 - LTC PKHA B 57 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_57(&self) -> &LTC0_PKB_57 {
        unsafe { &*(((self as *const Self) as *const u8).add(2788usize) as *const LTC0_PKB_57) }
    }
    #[doc = "0xae4 - LTC PKHA B 57 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_57_mut(&self) -> &mut LTC0_PKB_57 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2788usize) as *mut LTC0_PKB_57) }
    }
    #[doc = "0xae4 - LTC PKHA B3 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_9(&self) -> &LTC0_PKB3_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(2788usize) as *const LTC0_PKB3_9) }
    }
    #[doc = "0xae4 - LTC PKHA B3 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_9_mut(&self) -> &mut LTC0_PKB3_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2788usize) as *mut LTC0_PKB3_9) }
    }
    #[doc = "0xae8 - LTC PKHA B 58 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_58(&self) -> &LTC0_PKB_58 {
        unsafe { &*(((self as *const Self) as *const u8).add(2792usize) as *const LTC0_PKB_58) }
    }
    #[doc = "0xae8 - LTC PKHA B 58 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_58_mut(&self) -> &mut LTC0_PKB_58 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2792usize) as *mut LTC0_PKB_58) }
    }
    #[doc = "0xae8 - LTC PKHA B3 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_10(&self) -> &LTC0_PKB3_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(2792usize) as *const LTC0_PKB3_10) }
    }
    #[doc = "0xae8 - LTC PKHA B3 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_10_mut(&self) -> &mut LTC0_PKB3_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2792usize) as *mut LTC0_PKB3_10) }
    }
    #[doc = "0xaec - LTC PKHA B 59 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_59(&self) -> &LTC0_PKB_59 {
        unsafe { &*(((self as *const Self) as *const u8).add(2796usize) as *const LTC0_PKB_59) }
    }
    #[doc = "0xaec - LTC PKHA B 59 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_59_mut(&self) -> &mut LTC0_PKB_59 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2796usize) as *mut LTC0_PKB_59) }
    }
    #[doc = "0xaec - LTC PKHA B3 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_11(&self) -> &LTC0_PKB3_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(2796usize) as *const LTC0_PKB3_11) }
    }
    #[doc = "0xaec - LTC PKHA B3 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_11_mut(&self) -> &mut LTC0_PKB3_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2796usize) as *mut LTC0_PKB3_11) }
    }
    #[doc = "0xaf0 - LTC PKHA B 60 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_60(&self) -> &LTC0_PKB_60 {
        unsafe { &*(((self as *const Self) as *const u8).add(2800usize) as *const LTC0_PKB_60) }
    }
    #[doc = "0xaf0 - LTC PKHA B 60 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_60_mut(&self) -> &mut LTC0_PKB_60 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2800usize) as *mut LTC0_PKB_60) }
    }
    #[doc = "0xaf0 - LTC PKHA B3 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_12(&self) -> &LTC0_PKB3_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(2800usize) as *const LTC0_PKB3_12) }
    }
    #[doc = "0xaf0 - LTC PKHA B3 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_12_mut(&self) -> &mut LTC0_PKB3_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2800usize) as *mut LTC0_PKB3_12) }
    }
    #[doc = "0xaf4 - LTC PKHA B 61 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_61(&self) -> &LTC0_PKB_61 {
        unsafe { &*(((self as *const Self) as *const u8).add(2804usize) as *const LTC0_PKB_61) }
    }
    #[doc = "0xaf4 - LTC PKHA B 61 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_61_mut(&self) -> &mut LTC0_PKB_61 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2804usize) as *mut LTC0_PKB_61) }
    }
    #[doc = "0xaf4 - LTC PKHA B3 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_13(&self) -> &LTC0_PKB3_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(2804usize) as *const LTC0_PKB3_13) }
    }
    #[doc = "0xaf4 - LTC PKHA B3 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_13_mut(&self) -> &mut LTC0_PKB3_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2804usize) as *mut LTC0_PKB3_13) }
    }
    #[doc = "0xaf8 - LTC PKHA B 62 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_62(&self) -> &LTC0_PKB_62 {
        unsafe { &*(((self as *const Self) as *const u8).add(2808usize) as *const LTC0_PKB_62) }
    }
    #[doc = "0xaf8 - LTC PKHA B 62 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_62_mut(&self) -> &mut LTC0_PKB_62 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2808usize) as *mut LTC0_PKB_62) }
    }
    #[doc = "0xaf8 - LTC PKHA B3 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_14(&self) -> &LTC0_PKB3_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(2808usize) as *const LTC0_PKB3_14) }
    }
    #[doc = "0xaf8 - LTC PKHA B3 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_14_mut(&self) -> &mut LTC0_PKB3_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2808usize) as *mut LTC0_PKB3_14) }
    }
    #[doc = "0xafc - LTC PKHA B 63 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_63(&self) -> &LTC0_PKB_63 {
        unsafe { &*(((self as *const Self) as *const u8).add(2812usize) as *const LTC0_PKB_63) }
    }
    #[doc = "0xafc - LTC PKHA B 63 Register"]
    #[inline(always)]
    pub fn ltc0_pkb_63_mut(&self) -> &mut LTC0_PKB_63 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2812usize) as *mut LTC0_PKB_63) }
    }
    #[doc = "0xafc - LTC PKHA B3 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_15(&self) -> &LTC0_PKB3_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(2812usize) as *const LTC0_PKB3_15) }
    }
    #[doc = "0xafc - LTC PKHA B3 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkb3_15_mut(&self) -> &mut LTC0_PKB3_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2812usize) as *mut LTC0_PKB3_15) }
    }
    #[doc = "0xc00 - LTC PKHA N 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_0(&self) -> &LTC0_PKN_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3072usize) as *const LTC0_PKN_0) }
    }
    #[doc = "0xc00 - LTC PKHA N 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_0_mut(&self) -> &mut LTC0_PKN_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3072usize) as *mut LTC0_PKN_0) }
    }
    #[doc = "0xc00 - LTC PKHA N0 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_0(&self) -> &LTC0_PKN0_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3072usize) as *const LTC0_PKN0_0) }
    }
    #[doc = "0xc00 - LTC PKHA N0 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_0_mut(&self) -> &mut LTC0_PKN0_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3072usize) as *mut LTC0_PKN0_0) }
    }
    #[doc = "0xc04 - LTC PKHA N 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_1(&self) -> &LTC0_PKN_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3076usize) as *const LTC0_PKN_1) }
    }
    #[doc = "0xc04 - LTC PKHA N 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_1_mut(&self) -> &mut LTC0_PKN_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3076usize) as *mut LTC0_PKN_1) }
    }
    #[doc = "0xc04 - LTC PKHA N0 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_1(&self) -> &LTC0_PKN0_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3076usize) as *const LTC0_PKN0_1) }
    }
    #[doc = "0xc04 - LTC PKHA N0 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_1_mut(&self) -> &mut LTC0_PKN0_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3076usize) as *mut LTC0_PKN0_1) }
    }
    #[doc = "0xc08 - LTC PKHA N 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_2(&self) -> &LTC0_PKN_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3080usize) as *const LTC0_PKN_2) }
    }
    #[doc = "0xc08 - LTC PKHA N 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_2_mut(&self) -> &mut LTC0_PKN_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3080usize) as *mut LTC0_PKN_2) }
    }
    #[doc = "0xc08 - LTC PKHA N0 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_2(&self) -> &LTC0_PKN0_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3080usize) as *const LTC0_PKN0_2) }
    }
    #[doc = "0xc08 - LTC PKHA N0 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_2_mut(&self) -> &mut LTC0_PKN0_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3080usize) as *mut LTC0_PKN0_2) }
    }
    #[doc = "0xc0c - LTC PKHA N 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_3(&self) -> &LTC0_PKN_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3084usize) as *const LTC0_PKN_3) }
    }
    #[doc = "0xc0c - LTC PKHA N 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_3_mut(&self) -> &mut LTC0_PKN_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3084usize) as *mut LTC0_PKN_3) }
    }
    #[doc = "0xc0c - LTC PKHA N0 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_3(&self) -> &LTC0_PKN0_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3084usize) as *const LTC0_PKN0_3) }
    }
    #[doc = "0xc0c - LTC PKHA N0 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_3_mut(&self) -> &mut LTC0_PKN0_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3084usize) as *mut LTC0_PKN0_3) }
    }
    #[doc = "0xc10 - LTC PKHA N 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_4(&self) -> &LTC0_PKN_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3088usize) as *const LTC0_PKN_4) }
    }
    #[doc = "0xc10 - LTC PKHA N 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_4_mut(&self) -> &mut LTC0_PKN_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3088usize) as *mut LTC0_PKN_4) }
    }
    #[doc = "0xc10 - LTC PKHA N0 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_4(&self) -> &LTC0_PKN0_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3088usize) as *const LTC0_PKN0_4) }
    }
    #[doc = "0xc10 - LTC PKHA N0 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_4_mut(&self) -> &mut LTC0_PKN0_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3088usize) as *mut LTC0_PKN0_4) }
    }
    #[doc = "0xc14 - LTC PKHA N 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_5(&self) -> &LTC0_PKN_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3092usize) as *const LTC0_PKN_5) }
    }
    #[doc = "0xc14 - LTC PKHA N 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_5_mut(&self) -> &mut LTC0_PKN_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3092usize) as *mut LTC0_PKN_5) }
    }
    #[doc = "0xc14 - LTC PKHA N0 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_5(&self) -> &LTC0_PKN0_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3092usize) as *const LTC0_PKN0_5) }
    }
    #[doc = "0xc14 - LTC PKHA N0 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_5_mut(&self) -> &mut LTC0_PKN0_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3092usize) as *mut LTC0_PKN0_5) }
    }
    #[doc = "0xc18 - LTC PKHA N 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_6(&self) -> &LTC0_PKN_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3096usize) as *const LTC0_PKN_6) }
    }
    #[doc = "0xc18 - LTC PKHA N 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_6_mut(&self) -> &mut LTC0_PKN_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3096usize) as *mut LTC0_PKN_6) }
    }
    #[doc = "0xc18 - LTC PKHA N0 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_6(&self) -> &LTC0_PKN0_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3096usize) as *const LTC0_PKN0_6) }
    }
    #[doc = "0xc18 - LTC PKHA N0 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_6_mut(&self) -> &mut LTC0_PKN0_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3096usize) as *mut LTC0_PKN0_6) }
    }
    #[doc = "0xc1c - LTC PKHA N 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_7(&self) -> &LTC0_PKN_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3100usize) as *const LTC0_PKN_7) }
    }
    #[doc = "0xc1c - LTC PKHA N 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_7_mut(&self) -> &mut LTC0_PKN_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3100usize) as *mut LTC0_PKN_7) }
    }
    #[doc = "0xc1c - LTC PKHA N0 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_7(&self) -> &LTC0_PKN0_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3100usize) as *const LTC0_PKN0_7) }
    }
    #[doc = "0xc1c - LTC PKHA N0 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_7_mut(&self) -> &mut LTC0_PKN0_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3100usize) as *mut LTC0_PKN0_7) }
    }
    #[doc = "0xc20 - LTC PKHA N 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_8(&self) -> &LTC0_PKN_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3104usize) as *const LTC0_PKN_8) }
    }
    #[doc = "0xc20 - LTC PKHA N 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_8_mut(&self) -> &mut LTC0_PKN_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3104usize) as *mut LTC0_PKN_8) }
    }
    #[doc = "0xc20 - LTC PKHA N0 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_8(&self) -> &LTC0_PKN0_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3104usize) as *const LTC0_PKN0_8) }
    }
    #[doc = "0xc20 - LTC PKHA N0 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_8_mut(&self) -> &mut LTC0_PKN0_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3104usize) as *mut LTC0_PKN0_8) }
    }
    #[doc = "0xc24 - LTC PKHA N 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_9(&self) -> &LTC0_PKN_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3108usize) as *const LTC0_PKN_9) }
    }
    #[doc = "0xc24 - LTC PKHA N 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_9_mut(&self) -> &mut LTC0_PKN_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3108usize) as *mut LTC0_PKN_9) }
    }
    #[doc = "0xc24 - LTC PKHA N0 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_9(&self) -> &LTC0_PKN0_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3108usize) as *const LTC0_PKN0_9) }
    }
    #[doc = "0xc24 - LTC PKHA N0 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_9_mut(&self) -> &mut LTC0_PKN0_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3108usize) as *mut LTC0_PKN0_9) }
    }
    #[doc = "0xc28 - LTC PKHA N 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_10(&self) -> &LTC0_PKN_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3112usize) as *const LTC0_PKN_10) }
    }
    #[doc = "0xc28 - LTC PKHA N 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_10_mut(&self) -> &mut LTC0_PKN_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3112usize) as *mut LTC0_PKN_10) }
    }
    #[doc = "0xc28 - LTC PKHA N0 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_10(&self) -> &LTC0_PKN0_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3112usize) as *const LTC0_PKN0_10) }
    }
    #[doc = "0xc28 - LTC PKHA N0 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_10_mut(&self) -> &mut LTC0_PKN0_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3112usize) as *mut LTC0_PKN0_10) }
    }
    #[doc = "0xc2c - LTC PKHA N 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_11(&self) -> &LTC0_PKN_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3116usize) as *const LTC0_PKN_11) }
    }
    #[doc = "0xc2c - LTC PKHA N 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_11_mut(&self) -> &mut LTC0_PKN_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3116usize) as *mut LTC0_PKN_11) }
    }
    #[doc = "0xc2c - LTC PKHA N0 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_11(&self) -> &LTC0_PKN0_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3116usize) as *const LTC0_PKN0_11) }
    }
    #[doc = "0xc2c - LTC PKHA N0 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_11_mut(&self) -> &mut LTC0_PKN0_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3116usize) as *mut LTC0_PKN0_11) }
    }
    #[doc = "0xc30 - LTC PKHA N 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_12(&self) -> &LTC0_PKN_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3120usize) as *const LTC0_PKN_12) }
    }
    #[doc = "0xc30 - LTC PKHA N 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_12_mut(&self) -> &mut LTC0_PKN_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3120usize) as *mut LTC0_PKN_12) }
    }
    #[doc = "0xc30 - LTC PKHA N0 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_12(&self) -> &LTC0_PKN0_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3120usize) as *const LTC0_PKN0_12) }
    }
    #[doc = "0xc30 - LTC PKHA N0 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_12_mut(&self) -> &mut LTC0_PKN0_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3120usize) as *mut LTC0_PKN0_12) }
    }
    #[doc = "0xc34 - LTC PKHA N 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_13(&self) -> &LTC0_PKN_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3124usize) as *const LTC0_PKN_13) }
    }
    #[doc = "0xc34 - LTC PKHA N 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_13_mut(&self) -> &mut LTC0_PKN_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3124usize) as *mut LTC0_PKN_13) }
    }
    #[doc = "0xc34 - LTC PKHA N0 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_13(&self) -> &LTC0_PKN0_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3124usize) as *const LTC0_PKN0_13) }
    }
    #[doc = "0xc34 - LTC PKHA N0 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_13_mut(&self) -> &mut LTC0_PKN0_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3124usize) as *mut LTC0_PKN0_13) }
    }
    #[doc = "0xc38 - LTC PKHA N 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_14(&self) -> &LTC0_PKN_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3128usize) as *const LTC0_PKN_14) }
    }
    #[doc = "0xc38 - LTC PKHA N 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_14_mut(&self) -> &mut LTC0_PKN_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3128usize) as *mut LTC0_PKN_14) }
    }
    #[doc = "0xc38 - LTC PKHA N0 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_14(&self) -> &LTC0_PKN0_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3128usize) as *const LTC0_PKN0_14) }
    }
    #[doc = "0xc38 - LTC PKHA N0 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_14_mut(&self) -> &mut LTC0_PKN0_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3128usize) as *mut LTC0_PKN0_14) }
    }
    #[doc = "0xc3c - LTC PKHA N 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_15(&self) -> &LTC0_PKN_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3132usize) as *const LTC0_PKN_15) }
    }
    #[doc = "0xc3c - LTC PKHA N 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_15_mut(&self) -> &mut LTC0_PKN_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3132usize) as *mut LTC0_PKN_15) }
    }
    #[doc = "0xc3c - LTC PKHA N0 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_15(&self) -> &LTC0_PKN0_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3132usize) as *const LTC0_PKN0_15) }
    }
    #[doc = "0xc3c - LTC PKHA N0 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn0_15_mut(&self) -> &mut LTC0_PKN0_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3132usize) as *mut LTC0_PKN0_15) }
    }
    #[doc = "0xc40 - LTC PKHA N 16 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_16(&self) -> &LTC0_PKN_16 {
        unsafe { &*(((self as *const Self) as *const u8).add(3136usize) as *const LTC0_PKN_16) }
    }
    #[doc = "0xc40 - LTC PKHA N 16 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_16_mut(&self) -> &mut LTC0_PKN_16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3136usize) as *mut LTC0_PKN_16) }
    }
    #[doc = "0xc40 - LTC PKHA N1 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_0(&self) -> &LTC0_PKN1_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3136usize) as *const LTC0_PKN1_0) }
    }
    #[doc = "0xc40 - LTC PKHA N1 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_0_mut(&self) -> &mut LTC0_PKN1_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3136usize) as *mut LTC0_PKN1_0) }
    }
    #[doc = "0xc44 - LTC PKHA N 17 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_17(&self) -> &LTC0_PKN_17 {
        unsafe { &*(((self as *const Self) as *const u8).add(3140usize) as *const LTC0_PKN_17) }
    }
    #[doc = "0xc44 - LTC PKHA N 17 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_17_mut(&self) -> &mut LTC0_PKN_17 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3140usize) as *mut LTC0_PKN_17) }
    }
    #[doc = "0xc44 - LTC PKHA N1 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_1(&self) -> &LTC0_PKN1_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3140usize) as *const LTC0_PKN1_1) }
    }
    #[doc = "0xc44 - LTC PKHA N1 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_1_mut(&self) -> &mut LTC0_PKN1_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3140usize) as *mut LTC0_PKN1_1) }
    }
    #[doc = "0xc48 - LTC PKHA N 18 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_18(&self) -> &LTC0_PKN_18 {
        unsafe { &*(((self as *const Self) as *const u8).add(3144usize) as *const LTC0_PKN_18) }
    }
    #[doc = "0xc48 - LTC PKHA N 18 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_18_mut(&self) -> &mut LTC0_PKN_18 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3144usize) as *mut LTC0_PKN_18) }
    }
    #[doc = "0xc48 - LTC PKHA N1 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_2(&self) -> &LTC0_PKN1_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3144usize) as *const LTC0_PKN1_2) }
    }
    #[doc = "0xc48 - LTC PKHA N1 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_2_mut(&self) -> &mut LTC0_PKN1_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3144usize) as *mut LTC0_PKN1_2) }
    }
    #[doc = "0xc4c - LTC PKHA N 19 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_19(&self) -> &LTC0_PKN_19 {
        unsafe { &*(((self as *const Self) as *const u8).add(3148usize) as *const LTC0_PKN_19) }
    }
    #[doc = "0xc4c - LTC PKHA N 19 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_19_mut(&self) -> &mut LTC0_PKN_19 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3148usize) as *mut LTC0_PKN_19) }
    }
    #[doc = "0xc4c - LTC PKHA N1 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_3(&self) -> &LTC0_PKN1_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3148usize) as *const LTC0_PKN1_3) }
    }
    #[doc = "0xc4c - LTC PKHA N1 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_3_mut(&self) -> &mut LTC0_PKN1_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3148usize) as *mut LTC0_PKN1_3) }
    }
    #[doc = "0xc50 - LTC PKHA N 20 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_20(&self) -> &LTC0_PKN_20 {
        unsafe { &*(((self as *const Self) as *const u8).add(3152usize) as *const LTC0_PKN_20) }
    }
    #[doc = "0xc50 - LTC PKHA N 20 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_20_mut(&self) -> &mut LTC0_PKN_20 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3152usize) as *mut LTC0_PKN_20) }
    }
    #[doc = "0xc50 - LTC PKHA N1 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_4(&self) -> &LTC0_PKN1_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3152usize) as *const LTC0_PKN1_4) }
    }
    #[doc = "0xc50 - LTC PKHA N1 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_4_mut(&self) -> &mut LTC0_PKN1_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3152usize) as *mut LTC0_PKN1_4) }
    }
    #[doc = "0xc54 - LTC PKHA N 21 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_21(&self) -> &LTC0_PKN_21 {
        unsafe { &*(((self as *const Self) as *const u8).add(3156usize) as *const LTC0_PKN_21) }
    }
    #[doc = "0xc54 - LTC PKHA N 21 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_21_mut(&self) -> &mut LTC0_PKN_21 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3156usize) as *mut LTC0_PKN_21) }
    }
    #[doc = "0xc54 - LTC PKHA N1 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_5(&self) -> &LTC0_PKN1_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3156usize) as *const LTC0_PKN1_5) }
    }
    #[doc = "0xc54 - LTC PKHA N1 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_5_mut(&self) -> &mut LTC0_PKN1_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3156usize) as *mut LTC0_PKN1_5) }
    }
    #[doc = "0xc58 - LTC PKHA N 22 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_22(&self) -> &LTC0_PKN_22 {
        unsafe { &*(((self as *const Self) as *const u8).add(3160usize) as *const LTC0_PKN_22) }
    }
    #[doc = "0xc58 - LTC PKHA N 22 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_22_mut(&self) -> &mut LTC0_PKN_22 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3160usize) as *mut LTC0_PKN_22) }
    }
    #[doc = "0xc58 - LTC PKHA N1 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_6(&self) -> &LTC0_PKN1_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3160usize) as *const LTC0_PKN1_6) }
    }
    #[doc = "0xc58 - LTC PKHA N1 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_6_mut(&self) -> &mut LTC0_PKN1_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3160usize) as *mut LTC0_PKN1_6) }
    }
    #[doc = "0xc5c - LTC PKHA N 23 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_23(&self) -> &LTC0_PKN_23 {
        unsafe { &*(((self as *const Self) as *const u8).add(3164usize) as *const LTC0_PKN_23) }
    }
    #[doc = "0xc5c - LTC PKHA N 23 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_23_mut(&self) -> &mut LTC0_PKN_23 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3164usize) as *mut LTC0_PKN_23) }
    }
    #[doc = "0xc5c - LTC PKHA N1 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_7(&self) -> &LTC0_PKN1_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3164usize) as *const LTC0_PKN1_7) }
    }
    #[doc = "0xc5c - LTC PKHA N1 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_7_mut(&self) -> &mut LTC0_PKN1_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3164usize) as *mut LTC0_PKN1_7) }
    }
    #[doc = "0xc60 - LTC PKHA N 24 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_24(&self) -> &LTC0_PKN_24 {
        unsafe { &*(((self as *const Self) as *const u8).add(3168usize) as *const LTC0_PKN_24) }
    }
    #[doc = "0xc60 - LTC PKHA N 24 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_24_mut(&self) -> &mut LTC0_PKN_24 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3168usize) as *mut LTC0_PKN_24) }
    }
    #[doc = "0xc60 - LTC PKHA N1 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_8(&self) -> &LTC0_PKN1_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3168usize) as *const LTC0_PKN1_8) }
    }
    #[doc = "0xc60 - LTC PKHA N1 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_8_mut(&self) -> &mut LTC0_PKN1_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3168usize) as *mut LTC0_PKN1_8) }
    }
    #[doc = "0xc64 - LTC PKHA N 25 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_25(&self) -> &LTC0_PKN_25 {
        unsafe { &*(((self as *const Self) as *const u8).add(3172usize) as *const LTC0_PKN_25) }
    }
    #[doc = "0xc64 - LTC PKHA N 25 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_25_mut(&self) -> &mut LTC0_PKN_25 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3172usize) as *mut LTC0_PKN_25) }
    }
    #[doc = "0xc64 - LTC PKHA N1 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_9(&self) -> &LTC0_PKN1_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3172usize) as *const LTC0_PKN1_9) }
    }
    #[doc = "0xc64 - LTC PKHA N1 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_9_mut(&self) -> &mut LTC0_PKN1_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3172usize) as *mut LTC0_PKN1_9) }
    }
    #[doc = "0xc68 - LTC PKHA N 26 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_26(&self) -> &LTC0_PKN_26 {
        unsafe { &*(((self as *const Self) as *const u8).add(3176usize) as *const LTC0_PKN_26) }
    }
    #[doc = "0xc68 - LTC PKHA N 26 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_26_mut(&self) -> &mut LTC0_PKN_26 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3176usize) as *mut LTC0_PKN_26) }
    }
    #[doc = "0xc68 - LTC PKHA N1 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_10(&self) -> &LTC0_PKN1_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3176usize) as *const LTC0_PKN1_10) }
    }
    #[doc = "0xc68 - LTC PKHA N1 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_10_mut(&self) -> &mut LTC0_PKN1_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3176usize) as *mut LTC0_PKN1_10) }
    }
    #[doc = "0xc6c - LTC PKHA N 27 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_27(&self) -> &LTC0_PKN_27 {
        unsafe { &*(((self as *const Self) as *const u8).add(3180usize) as *const LTC0_PKN_27) }
    }
    #[doc = "0xc6c - LTC PKHA N 27 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_27_mut(&self) -> &mut LTC0_PKN_27 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3180usize) as *mut LTC0_PKN_27) }
    }
    #[doc = "0xc6c - LTC PKHA N1 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_11(&self) -> &LTC0_PKN1_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3180usize) as *const LTC0_PKN1_11) }
    }
    #[doc = "0xc6c - LTC PKHA N1 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_11_mut(&self) -> &mut LTC0_PKN1_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3180usize) as *mut LTC0_PKN1_11) }
    }
    #[doc = "0xc70 - LTC PKHA N 28 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_28(&self) -> &LTC0_PKN_28 {
        unsafe { &*(((self as *const Self) as *const u8).add(3184usize) as *const LTC0_PKN_28) }
    }
    #[doc = "0xc70 - LTC PKHA N 28 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_28_mut(&self) -> &mut LTC0_PKN_28 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3184usize) as *mut LTC0_PKN_28) }
    }
    #[doc = "0xc70 - LTC PKHA N1 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_12(&self) -> &LTC0_PKN1_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3184usize) as *const LTC0_PKN1_12) }
    }
    #[doc = "0xc70 - LTC PKHA N1 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_12_mut(&self) -> &mut LTC0_PKN1_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3184usize) as *mut LTC0_PKN1_12) }
    }
    #[doc = "0xc74 - LTC PKHA N 29 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_29(&self) -> &LTC0_PKN_29 {
        unsafe { &*(((self as *const Self) as *const u8).add(3188usize) as *const LTC0_PKN_29) }
    }
    #[doc = "0xc74 - LTC PKHA N 29 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_29_mut(&self) -> &mut LTC0_PKN_29 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3188usize) as *mut LTC0_PKN_29) }
    }
    #[doc = "0xc74 - LTC PKHA N1 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_13(&self) -> &LTC0_PKN1_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3188usize) as *const LTC0_PKN1_13) }
    }
    #[doc = "0xc74 - LTC PKHA N1 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_13_mut(&self) -> &mut LTC0_PKN1_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3188usize) as *mut LTC0_PKN1_13) }
    }
    #[doc = "0xc78 - LTC PKHA N 30 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_30(&self) -> &LTC0_PKN_30 {
        unsafe { &*(((self as *const Self) as *const u8).add(3192usize) as *const LTC0_PKN_30) }
    }
    #[doc = "0xc78 - LTC PKHA N 30 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_30_mut(&self) -> &mut LTC0_PKN_30 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3192usize) as *mut LTC0_PKN_30) }
    }
    #[doc = "0xc78 - LTC PKHA N1 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_14(&self) -> &LTC0_PKN1_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3192usize) as *const LTC0_PKN1_14) }
    }
    #[doc = "0xc78 - LTC PKHA N1 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_14_mut(&self) -> &mut LTC0_PKN1_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3192usize) as *mut LTC0_PKN1_14) }
    }
    #[doc = "0xc7c - LTC PKHA N 31 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_31(&self) -> &LTC0_PKN_31 {
        unsafe { &*(((self as *const Self) as *const u8).add(3196usize) as *const LTC0_PKN_31) }
    }
    #[doc = "0xc7c - LTC PKHA N 31 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_31_mut(&self) -> &mut LTC0_PKN_31 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3196usize) as *mut LTC0_PKN_31) }
    }
    #[doc = "0xc7c - LTC PKHA N1 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_15(&self) -> &LTC0_PKN1_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3196usize) as *const LTC0_PKN1_15) }
    }
    #[doc = "0xc7c - LTC PKHA N1 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn1_15_mut(&self) -> &mut LTC0_PKN1_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3196usize) as *mut LTC0_PKN1_15) }
    }
    #[doc = "0xc80 - LTC PKHA N 32 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_32(&self) -> &LTC0_PKN_32 {
        unsafe { &*(((self as *const Self) as *const u8).add(3200usize) as *const LTC0_PKN_32) }
    }
    #[doc = "0xc80 - LTC PKHA N 32 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_32_mut(&self) -> &mut LTC0_PKN_32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3200usize) as *mut LTC0_PKN_32) }
    }
    #[doc = "0xc80 - LTC PKHA N2 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_0(&self) -> &LTC0_PKN2_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3200usize) as *const LTC0_PKN2_0) }
    }
    #[doc = "0xc80 - LTC PKHA N2 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_0_mut(&self) -> &mut LTC0_PKN2_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3200usize) as *mut LTC0_PKN2_0) }
    }
    #[doc = "0xc84 - LTC PKHA N 33 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_33(&self) -> &LTC0_PKN_33 {
        unsafe { &*(((self as *const Self) as *const u8).add(3204usize) as *const LTC0_PKN_33) }
    }
    #[doc = "0xc84 - LTC PKHA N 33 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_33_mut(&self) -> &mut LTC0_PKN_33 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3204usize) as *mut LTC0_PKN_33) }
    }
    #[doc = "0xc84 - LTC PKHA N2 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_1(&self) -> &LTC0_PKN2_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3204usize) as *const LTC0_PKN2_1) }
    }
    #[doc = "0xc84 - LTC PKHA N2 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_1_mut(&self) -> &mut LTC0_PKN2_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3204usize) as *mut LTC0_PKN2_1) }
    }
    #[doc = "0xc88 - LTC PKHA N 34 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_34(&self) -> &LTC0_PKN_34 {
        unsafe { &*(((self as *const Self) as *const u8).add(3208usize) as *const LTC0_PKN_34) }
    }
    #[doc = "0xc88 - LTC PKHA N 34 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_34_mut(&self) -> &mut LTC0_PKN_34 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3208usize) as *mut LTC0_PKN_34) }
    }
    #[doc = "0xc88 - LTC PKHA N2 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_2(&self) -> &LTC0_PKN2_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3208usize) as *const LTC0_PKN2_2) }
    }
    #[doc = "0xc88 - LTC PKHA N2 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_2_mut(&self) -> &mut LTC0_PKN2_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3208usize) as *mut LTC0_PKN2_2) }
    }
    #[doc = "0xc8c - LTC PKHA N 35 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_35(&self) -> &LTC0_PKN_35 {
        unsafe { &*(((self as *const Self) as *const u8).add(3212usize) as *const LTC0_PKN_35) }
    }
    #[doc = "0xc8c - LTC PKHA N 35 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_35_mut(&self) -> &mut LTC0_PKN_35 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3212usize) as *mut LTC0_PKN_35) }
    }
    #[doc = "0xc8c - LTC PKHA N2 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_3(&self) -> &LTC0_PKN2_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3212usize) as *const LTC0_PKN2_3) }
    }
    #[doc = "0xc8c - LTC PKHA N2 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_3_mut(&self) -> &mut LTC0_PKN2_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3212usize) as *mut LTC0_PKN2_3) }
    }
    #[doc = "0xc90 - LTC PKHA N 36 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_36(&self) -> &LTC0_PKN_36 {
        unsafe { &*(((self as *const Self) as *const u8).add(3216usize) as *const LTC0_PKN_36) }
    }
    #[doc = "0xc90 - LTC PKHA N 36 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_36_mut(&self) -> &mut LTC0_PKN_36 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3216usize) as *mut LTC0_PKN_36) }
    }
    #[doc = "0xc90 - LTC PKHA N2 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_4(&self) -> &LTC0_PKN2_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3216usize) as *const LTC0_PKN2_4) }
    }
    #[doc = "0xc90 - LTC PKHA N2 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_4_mut(&self) -> &mut LTC0_PKN2_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3216usize) as *mut LTC0_PKN2_4) }
    }
    #[doc = "0xc94 - LTC PKHA N 37 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_37(&self) -> &LTC0_PKN_37 {
        unsafe { &*(((self as *const Self) as *const u8).add(3220usize) as *const LTC0_PKN_37) }
    }
    #[doc = "0xc94 - LTC PKHA N 37 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_37_mut(&self) -> &mut LTC0_PKN_37 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3220usize) as *mut LTC0_PKN_37) }
    }
    #[doc = "0xc94 - LTC PKHA N2 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_5(&self) -> &LTC0_PKN2_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3220usize) as *const LTC0_PKN2_5) }
    }
    #[doc = "0xc94 - LTC PKHA N2 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_5_mut(&self) -> &mut LTC0_PKN2_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3220usize) as *mut LTC0_PKN2_5) }
    }
    #[doc = "0xc98 - LTC PKHA N 38 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_38(&self) -> &LTC0_PKN_38 {
        unsafe { &*(((self as *const Self) as *const u8).add(3224usize) as *const LTC0_PKN_38) }
    }
    #[doc = "0xc98 - LTC PKHA N 38 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_38_mut(&self) -> &mut LTC0_PKN_38 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3224usize) as *mut LTC0_PKN_38) }
    }
    #[doc = "0xc98 - LTC PKHA N2 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_6(&self) -> &LTC0_PKN2_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3224usize) as *const LTC0_PKN2_6) }
    }
    #[doc = "0xc98 - LTC PKHA N2 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_6_mut(&self) -> &mut LTC0_PKN2_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3224usize) as *mut LTC0_PKN2_6) }
    }
    #[doc = "0xc9c - LTC PKHA N 39 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_39(&self) -> &LTC0_PKN_39 {
        unsafe { &*(((self as *const Self) as *const u8).add(3228usize) as *const LTC0_PKN_39) }
    }
    #[doc = "0xc9c - LTC PKHA N 39 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_39_mut(&self) -> &mut LTC0_PKN_39 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3228usize) as *mut LTC0_PKN_39) }
    }
    #[doc = "0xc9c - LTC PKHA N2 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_7(&self) -> &LTC0_PKN2_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3228usize) as *const LTC0_PKN2_7) }
    }
    #[doc = "0xc9c - LTC PKHA N2 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_7_mut(&self) -> &mut LTC0_PKN2_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3228usize) as *mut LTC0_PKN2_7) }
    }
    #[doc = "0xca0 - LTC PKHA N 40 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_40(&self) -> &LTC0_PKN_40 {
        unsafe { &*(((self as *const Self) as *const u8).add(3232usize) as *const LTC0_PKN_40) }
    }
    #[doc = "0xca0 - LTC PKHA N 40 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_40_mut(&self) -> &mut LTC0_PKN_40 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3232usize) as *mut LTC0_PKN_40) }
    }
    #[doc = "0xca0 - LTC PKHA N2 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_8(&self) -> &LTC0_PKN2_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3232usize) as *const LTC0_PKN2_8) }
    }
    #[doc = "0xca0 - LTC PKHA N2 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_8_mut(&self) -> &mut LTC0_PKN2_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3232usize) as *mut LTC0_PKN2_8) }
    }
    #[doc = "0xca4 - LTC PKHA N 41 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_41(&self) -> &LTC0_PKN_41 {
        unsafe { &*(((self as *const Self) as *const u8).add(3236usize) as *const LTC0_PKN_41) }
    }
    #[doc = "0xca4 - LTC PKHA N 41 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_41_mut(&self) -> &mut LTC0_PKN_41 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3236usize) as *mut LTC0_PKN_41) }
    }
    #[doc = "0xca4 - LTC PKHA N2 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_9(&self) -> &LTC0_PKN2_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3236usize) as *const LTC0_PKN2_9) }
    }
    #[doc = "0xca4 - LTC PKHA N2 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_9_mut(&self) -> &mut LTC0_PKN2_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3236usize) as *mut LTC0_PKN2_9) }
    }
    #[doc = "0xca8 - LTC PKHA N 42 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_42(&self) -> &LTC0_PKN_42 {
        unsafe { &*(((self as *const Self) as *const u8).add(3240usize) as *const LTC0_PKN_42) }
    }
    #[doc = "0xca8 - LTC PKHA N 42 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_42_mut(&self) -> &mut LTC0_PKN_42 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3240usize) as *mut LTC0_PKN_42) }
    }
    #[doc = "0xca8 - LTC PKHA N2 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_10(&self) -> &LTC0_PKN2_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3240usize) as *const LTC0_PKN2_10) }
    }
    #[doc = "0xca8 - LTC PKHA N2 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_10_mut(&self) -> &mut LTC0_PKN2_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3240usize) as *mut LTC0_PKN2_10) }
    }
    #[doc = "0xcac - LTC PKHA N 43 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_43(&self) -> &LTC0_PKN_43 {
        unsafe { &*(((self as *const Self) as *const u8).add(3244usize) as *const LTC0_PKN_43) }
    }
    #[doc = "0xcac - LTC PKHA N 43 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_43_mut(&self) -> &mut LTC0_PKN_43 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3244usize) as *mut LTC0_PKN_43) }
    }
    #[doc = "0xcac - LTC PKHA N2 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_11(&self) -> &LTC0_PKN2_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3244usize) as *const LTC0_PKN2_11) }
    }
    #[doc = "0xcac - LTC PKHA N2 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_11_mut(&self) -> &mut LTC0_PKN2_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3244usize) as *mut LTC0_PKN2_11) }
    }
    #[doc = "0xcb0 - LTC PKHA N 44 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_44(&self) -> &LTC0_PKN_44 {
        unsafe { &*(((self as *const Self) as *const u8).add(3248usize) as *const LTC0_PKN_44) }
    }
    #[doc = "0xcb0 - LTC PKHA N 44 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_44_mut(&self) -> &mut LTC0_PKN_44 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3248usize) as *mut LTC0_PKN_44) }
    }
    #[doc = "0xcb0 - LTC PKHA N2 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_12(&self) -> &LTC0_PKN2_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3248usize) as *const LTC0_PKN2_12) }
    }
    #[doc = "0xcb0 - LTC PKHA N2 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_12_mut(&self) -> &mut LTC0_PKN2_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3248usize) as *mut LTC0_PKN2_12) }
    }
    #[doc = "0xcb4 - LTC PKHA N 45 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_45(&self) -> &LTC0_PKN_45 {
        unsafe { &*(((self as *const Self) as *const u8).add(3252usize) as *const LTC0_PKN_45) }
    }
    #[doc = "0xcb4 - LTC PKHA N 45 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_45_mut(&self) -> &mut LTC0_PKN_45 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3252usize) as *mut LTC0_PKN_45) }
    }
    #[doc = "0xcb4 - LTC PKHA N2 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_13(&self) -> &LTC0_PKN2_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3252usize) as *const LTC0_PKN2_13) }
    }
    #[doc = "0xcb4 - LTC PKHA N2 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_13_mut(&self) -> &mut LTC0_PKN2_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3252usize) as *mut LTC0_PKN2_13) }
    }
    #[doc = "0xcb8 - LTC PKHA N 46 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_46(&self) -> &LTC0_PKN_46 {
        unsafe { &*(((self as *const Self) as *const u8).add(3256usize) as *const LTC0_PKN_46) }
    }
    #[doc = "0xcb8 - LTC PKHA N 46 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_46_mut(&self) -> &mut LTC0_PKN_46 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3256usize) as *mut LTC0_PKN_46) }
    }
    #[doc = "0xcb8 - LTC PKHA N2 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_14(&self) -> &LTC0_PKN2_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3256usize) as *const LTC0_PKN2_14) }
    }
    #[doc = "0xcb8 - LTC PKHA N2 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_14_mut(&self) -> &mut LTC0_PKN2_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3256usize) as *mut LTC0_PKN2_14) }
    }
    #[doc = "0xcbc - LTC PKHA N 47 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_47(&self) -> &LTC0_PKN_47 {
        unsafe { &*(((self as *const Self) as *const u8).add(3260usize) as *const LTC0_PKN_47) }
    }
    #[doc = "0xcbc - LTC PKHA N 47 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_47_mut(&self) -> &mut LTC0_PKN_47 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3260usize) as *mut LTC0_PKN_47) }
    }
    #[doc = "0xcbc - LTC PKHA N2 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_15(&self) -> &LTC0_PKN2_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3260usize) as *const LTC0_PKN2_15) }
    }
    #[doc = "0xcbc - LTC PKHA N2 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn2_15_mut(&self) -> &mut LTC0_PKN2_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3260usize) as *mut LTC0_PKN2_15) }
    }
    #[doc = "0xcc0 - LTC PKHA N 48 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_48(&self) -> &LTC0_PKN_48 {
        unsafe { &*(((self as *const Self) as *const u8).add(3264usize) as *const LTC0_PKN_48) }
    }
    #[doc = "0xcc0 - LTC PKHA N 48 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_48_mut(&self) -> &mut LTC0_PKN_48 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3264usize) as *mut LTC0_PKN_48) }
    }
    #[doc = "0xcc0 - LTC PKHA N3 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_0(&self) -> &LTC0_PKN3_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3264usize) as *const LTC0_PKN3_0) }
    }
    #[doc = "0xcc0 - LTC PKHA N3 0 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_0_mut(&self) -> &mut LTC0_PKN3_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3264usize) as *mut LTC0_PKN3_0) }
    }
    #[doc = "0xcc4 - LTC PKHA N 49 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_49(&self) -> &LTC0_PKN_49 {
        unsafe { &*(((self as *const Self) as *const u8).add(3268usize) as *const LTC0_PKN_49) }
    }
    #[doc = "0xcc4 - LTC PKHA N 49 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_49_mut(&self) -> &mut LTC0_PKN_49 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3268usize) as *mut LTC0_PKN_49) }
    }
    #[doc = "0xcc4 - LTC PKHA N3 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_1(&self) -> &LTC0_PKN3_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3268usize) as *const LTC0_PKN3_1) }
    }
    #[doc = "0xcc4 - LTC PKHA N3 1 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_1_mut(&self) -> &mut LTC0_PKN3_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3268usize) as *mut LTC0_PKN3_1) }
    }
    #[doc = "0xcc8 - LTC PKHA N 50 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_50(&self) -> &LTC0_PKN_50 {
        unsafe { &*(((self as *const Self) as *const u8).add(3272usize) as *const LTC0_PKN_50) }
    }
    #[doc = "0xcc8 - LTC PKHA N 50 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_50_mut(&self) -> &mut LTC0_PKN_50 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3272usize) as *mut LTC0_PKN_50) }
    }
    #[doc = "0xcc8 - LTC PKHA N3 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_2(&self) -> &LTC0_PKN3_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3272usize) as *const LTC0_PKN3_2) }
    }
    #[doc = "0xcc8 - LTC PKHA N3 2 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_2_mut(&self) -> &mut LTC0_PKN3_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3272usize) as *mut LTC0_PKN3_2) }
    }
    #[doc = "0xccc - LTC PKHA N 51 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_51(&self) -> &LTC0_PKN_51 {
        unsafe { &*(((self as *const Self) as *const u8).add(3276usize) as *const LTC0_PKN_51) }
    }
    #[doc = "0xccc - LTC PKHA N 51 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_51_mut(&self) -> &mut LTC0_PKN_51 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3276usize) as *mut LTC0_PKN_51) }
    }
    #[doc = "0xccc - LTC PKHA N3 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_3(&self) -> &LTC0_PKN3_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3276usize) as *const LTC0_PKN3_3) }
    }
    #[doc = "0xccc - LTC PKHA N3 3 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_3_mut(&self) -> &mut LTC0_PKN3_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3276usize) as *mut LTC0_PKN3_3) }
    }
    #[doc = "0xcd0 - LTC PKHA N 52 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_52(&self) -> &LTC0_PKN_52 {
        unsafe { &*(((self as *const Self) as *const u8).add(3280usize) as *const LTC0_PKN_52) }
    }
    #[doc = "0xcd0 - LTC PKHA N 52 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_52_mut(&self) -> &mut LTC0_PKN_52 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3280usize) as *mut LTC0_PKN_52) }
    }
    #[doc = "0xcd0 - LTC PKHA N3 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_4(&self) -> &LTC0_PKN3_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3280usize) as *const LTC0_PKN3_4) }
    }
    #[doc = "0xcd0 - LTC PKHA N3 4 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_4_mut(&self) -> &mut LTC0_PKN3_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3280usize) as *mut LTC0_PKN3_4) }
    }
    #[doc = "0xcd4 - LTC PKHA N 53 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_53(&self) -> &LTC0_PKN_53 {
        unsafe { &*(((self as *const Self) as *const u8).add(3284usize) as *const LTC0_PKN_53) }
    }
    #[doc = "0xcd4 - LTC PKHA N 53 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_53_mut(&self) -> &mut LTC0_PKN_53 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3284usize) as *mut LTC0_PKN_53) }
    }
    #[doc = "0xcd4 - LTC PKHA N3 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_5(&self) -> &LTC0_PKN3_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3284usize) as *const LTC0_PKN3_5) }
    }
    #[doc = "0xcd4 - LTC PKHA N3 5 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_5_mut(&self) -> &mut LTC0_PKN3_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3284usize) as *mut LTC0_PKN3_5) }
    }
    #[doc = "0xcd8 - LTC PKHA N 54 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_54(&self) -> &LTC0_PKN_54 {
        unsafe { &*(((self as *const Self) as *const u8).add(3288usize) as *const LTC0_PKN_54) }
    }
    #[doc = "0xcd8 - LTC PKHA N 54 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_54_mut(&self) -> &mut LTC0_PKN_54 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3288usize) as *mut LTC0_PKN_54) }
    }
    #[doc = "0xcd8 - LTC PKHA N3 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_6(&self) -> &LTC0_PKN3_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3288usize) as *const LTC0_PKN3_6) }
    }
    #[doc = "0xcd8 - LTC PKHA N3 6 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_6_mut(&self) -> &mut LTC0_PKN3_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3288usize) as *mut LTC0_PKN3_6) }
    }
    #[doc = "0xcdc - LTC PKHA N 55 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_55(&self) -> &LTC0_PKN_55 {
        unsafe { &*(((self as *const Self) as *const u8).add(3292usize) as *const LTC0_PKN_55) }
    }
    #[doc = "0xcdc - LTC PKHA N 55 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_55_mut(&self) -> &mut LTC0_PKN_55 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3292usize) as *mut LTC0_PKN_55) }
    }
    #[doc = "0xcdc - LTC PKHA N3 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_7(&self) -> &LTC0_PKN3_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3292usize) as *const LTC0_PKN3_7) }
    }
    #[doc = "0xcdc - LTC PKHA N3 7 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_7_mut(&self) -> &mut LTC0_PKN3_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3292usize) as *mut LTC0_PKN3_7) }
    }
    #[doc = "0xce0 - LTC PKHA N 56 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_56(&self) -> &LTC0_PKN_56 {
        unsafe { &*(((self as *const Self) as *const u8).add(3296usize) as *const LTC0_PKN_56) }
    }
    #[doc = "0xce0 - LTC PKHA N 56 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_56_mut(&self) -> &mut LTC0_PKN_56 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3296usize) as *mut LTC0_PKN_56) }
    }
    #[doc = "0xce0 - LTC PKHA N3 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_8(&self) -> &LTC0_PKN3_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3296usize) as *const LTC0_PKN3_8) }
    }
    #[doc = "0xce0 - LTC PKHA N3 8 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_8_mut(&self) -> &mut LTC0_PKN3_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3296usize) as *mut LTC0_PKN3_8) }
    }
    #[doc = "0xce4 - LTC PKHA N 57 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_57(&self) -> &LTC0_PKN_57 {
        unsafe { &*(((self as *const Self) as *const u8).add(3300usize) as *const LTC0_PKN_57) }
    }
    #[doc = "0xce4 - LTC PKHA N 57 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_57_mut(&self) -> &mut LTC0_PKN_57 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3300usize) as *mut LTC0_PKN_57) }
    }
    #[doc = "0xce4 - LTC PKHA N3 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_9(&self) -> &LTC0_PKN3_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3300usize) as *const LTC0_PKN3_9) }
    }
    #[doc = "0xce4 - LTC PKHA N3 9 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_9_mut(&self) -> &mut LTC0_PKN3_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3300usize) as *mut LTC0_PKN3_9) }
    }
    #[doc = "0xce8 - LTC PKHA N 58 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_58(&self) -> &LTC0_PKN_58 {
        unsafe { &*(((self as *const Self) as *const u8).add(3304usize) as *const LTC0_PKN_58) }
    }
    #[doc = "0xce8 - LTC PKHA N 58 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_58_mut(&self) -> &mut LTC0_PKN_58 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3304usize) as *mut LTC0_PKN_58) }
    }
    #[doc = "0xce8 - LTC PKHA N3 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_10(&self) -> &LTC0_PKN3_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3304usize) as *const LTC0_PKN3_10) }
    }
    #[doc = "0xce8 - LTC PKHA N3 10 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_10_mut(&self) -> &mut LTC0_PKN3_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3304usize) as *mut LTC0_PKN3_10) }
    }
    #[doc = "0xcec - LTC PKHA N 59 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_59(&self) -> &LTC0_PKN_59 {
        unsafe { &*(((self as *const Self) as *const u8).add(3308usize) as *const LTC0_PKN_59) }
    }
    #[doc = "0xcec - LTC PKHA N 59 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_59_mut(&self) -> &mut LTC0_PKN_59 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3308usize) as *mut LTC0_PKN_59) }
    }
    #[doc = "0xcec - LTC PKHA N3 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_11(&self) -> &LTC0_PKN3_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3308usize) as *const LTC0_PKN3_11) }
    }
    #[doc = "0xcec - LTC PKHA N3 11 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_11_mut(&self) -> &mut LTC0_PKN3_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3308usize) as *mut LTC0_PKN3_11) }
    }
    #[doc = "0xcf0 - LTC PKHA N 60 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_60(&self) -> &LTC0_PKN_60 {
        unsafe { &*(((self as *const Self) as *const u8).add(3312usize) as *const LTC0_PKN_60) }
    }
    #[doc = "0xcf0 - LTC PKHA N 60 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_60_mut(&self) -> &mut LTC0_PKN_60 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3312usize) as *mut LTC0_PKN_60) }
    }
    #[doc = "0xcf0 - LTC PKHA N3 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_12(&self) -> &LTC0_PKN3_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3312usize) as *const LTC0_PKN3_12) }
    }
    #[doc = "0xcf0 - LTC PKHA N3 12 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_12_mut(&self) -> &mut LTC0_PKN3_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3312usize) as *mut LTC0_PKN3_12) }
    }
    #[doc = "0xcf4 - LTC PKHA N 61 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_61(&self) -> &LTC0_PKN_61 {
        unsafe { &*(((self as *const Self) as *const u8).add(3316usize) as *const LTC0_PKN_61) }
    }
    #[doc = "0xcf4 - LTC PKHA N 61 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_61_mut(&self) -> &mut LTC0_PKN_61 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3316usize) as *mut LTC0_PKN_61) }
    }
    #[doc = "0xcf4 - LTC PKHA N3 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_13(&self) -> &LTC0_PKN3_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3316usize) as *const LTC0_PKN3_13) }
    }
    #[doc = "0xcf4 - LTC PKHA N3 13 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_13_mut(&self) -> &mut LTC0_PKN3_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3316usize) as *mut LTC0_PKN3_13) }
    }
    #[doc = "0xcf8 - LTC PKHA N 62 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_62(&self) -> &LTC0_PKN_62 {
        unsafe { &*(((self as *const Self) as *const u8).add(3320usize) as *const LTC0_PKN_62) }
    }
    #[doc = "0xcf8 - LTC PKHA N 62 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_62_mut(&self) -> &mut LTC0_PKN_62 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3320usize) as *mut LTC0_PKN_62) }
    }
    #[doc = "0xcf8 - LTC PKHA N3 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_14(&self) -> &LTC0_PKN3_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3320usize) as *const LTC0_PKN3_14) }
    }
    #[doc = "0xcf8 - LTC PKHA N3 14 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_14_mut(&self) -> &mut LTC0_PKN3_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3320usize) as *mut LTC0_PKN3_14) }
    }
    #[doc = "0xcfc - LTC PKHA N 63 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_63(&self) -> &LTC0_PKN_63 {
        unsafe { &*(((self as *const Self) as *const u8).add(3324usize) as *const LTC0_PKN_63) }
    }
    #[doc = "0xcfc - LTC PKHA N 63 Register"]
    #[inline(always)]
    pub fn ltc0_pkn_63_mut(&self) -> &mut LTC0_PKN_63 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3324usize) as *mut LTC0_PKN_63) }
    }
    #[doc = "0xcfc - LTC PKHA N3 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_15(&self) -> &LTC0_PKN3_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3324usize) as *const LTC0_PKN3_15) }
    }
    #[doc = "0xcfc - LTC PKHA N3 15 Register"]
    #[inline(always)]
    pub fn ltc0_pkn3_15_mut(&self) -> &mut LTC0_PKN3_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3324usize) as *mut LTC0_PKN3_15) }
    }
    #[doc = "0xe00 - LTC PKHA E 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke_0(&self) -> &LTC0_PKE_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3584usize) as *const LTC0_PKE_0) }
    }
    #[doc = "0xe00 - LTC PKHA E 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke_0_mut(&self) -> &mut LTC0_PKE_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3584usize) as *mut LTC0_PKE_0) }
    }
    #[doc = "0xe00 - LTC PKHA E0 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_0(&self) -> &LTC0_PKE0_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3584usize) as *const LTC0_PKE0_0) }
    }
    #[doc = "0xe00 - LTC PKHA E0 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_0_mut(&self) -> &mut LTC0_PKE0_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3584usize) as *mut LTC0_PKE0_0) }
    }
    #[doc = "0xe04 - LTC PKHA E 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke_1(&self) -> &LTC0_PKE_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3588usize) as *const LTC0_PKE_1) }
    }
    #[doc = "0xe04 - LTC PKHA E 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke_1_mut(&self) -> &mut LTC0_PKE_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3588usize) as *mut LTC0_PKE_1) }
    }
    #[doc = "0xe04 - LTC PKHA E0 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_1(&self) -> &LTC0_PKE0_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3588usize) as *const LTC0_PKE0_1) }
    }
    #[doc = "0xe04 - LTC PKHA E0 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_1_mut(&self) -> &mut LTC0_PKE0_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3588usize) as *mut LTC0_PKE0_1) }
    }
    #[doc = "0xe08 - LTC PKHA E 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke_2(&self) -> &LTC0_PKE_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3592usize) as *const LTC0_PKE_2) }
    }
    #[doc = "0xe08 - LTC PKHA E 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke_2_mut(&self) -> &mut LTC0_PKE_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3592usize) as *mut LTC0_PKE_2) }
    }
    #[doc = "0xe08 - LTC PKHA E0 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_2(&self) -> &LTC0_PKE0_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3592usize) as *const LTC0_PKE0_2) }
    }
    #[doc = "0xe08 - LTC PKHA E0 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_2_mut(&self) -> &mut LTC0_PKE0_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3592usize) as *mut LTC0_PKE0_2) }
    }
    #[doc = "0xe0c - LTC PKHA E 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke_3(&self) -> &LTC0_PKE_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3596usize) as *const LTC0_PKE_3) }
    }
    #[doc = "0xe0c - LTC PKHA E 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke_3_mut(&self) -> &mut LTC0_PKE_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3596usize) as *mut LTC0_PKE_3) }
    }
    #[doc = "0xe0c - LTC PKHA E0 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_3(&self) -> &LTC0_PKE0_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3596usize) as *const LTC0_PKE0_3) }
    }
    #[doc = "0xe0c - LTC PKHA E0 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_3_mut(&self) -> &mut LTC0_PKE0_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3596usize) as *mut LTC0_PKE0_3) }
    }
    #[doc = "0xe10 - LTC PKHA E 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke_4(&self) -> &LTC0_PKE_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3600usize) as *const LTC0_PKE_4) }
    }
    #[doc = "0xe10 - LTC PKHA E 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke_4_mut(&self) -> &mut LTC0_PKE_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3600usize) as *mut LTC0_PKE_4) }
    }
    #[doc = "0xe10 - LTC PKHA E0 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_4(&self) -> &LTC0_PKE0_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3600usize) as *const LTC0_PKE0_4) }
    }
    #[doc = "0xe10 - LTC PKHA E0 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_4_mut(&self) -> &mut LTC0_PKE0_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3600usize) as *mut LTC0_PKE0_4) }
    }
    #[doc = "0xe14 - LTC PKHA E 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke_5(&self) -> &LTC0_PKE_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3604usize) as *const LTC0_PKE_5) }
    }
    #[doc = "0xe14 - LTC PKHA E 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke_5_mut(&self) -> &mut LTC0_PKE_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3604usize) as *mut LTC0_PKE_5) }
    }
    #[doc = "0xe14 - LTC PKHA E0 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_5(&self) -> &LTC0_PKE0_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3604usize) as *const LTC0_PKE0_5) }
    }
    #[doc = "0xe14 - LTC PKHA E0 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_5_mut(&self) -> &mut LTC0_PKE0_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3604usize) as *mut LTC0_PKE0_5) }
    }
    #[doc = "0xe18 - LTC PKHA E 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke_6(&self) -> &LTC0_PKE_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3608usize) as *const LTC0_PKE_6) }
    }
    #[doc = "0xe18 - LTC PKHA E 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke_6_mut(&self) -> &mut LTC0_PKE_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3608usize) as *mut LTC0_PKE_6) }
    }
    #[doc = "0xe18 - LTC PKHA E0 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_6(&self) -> &LTC0_PKE0_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3608usize) as *const LTC0_PKE0_6) }
    }
    #[doc = "0xe18 - LTC PKHA E0 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_6_mut(&self) -> &mut LTC0_PKE0_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3608usize) as *mut LTC0_PKE0_6) }
    }
    #[doc = "0xe1c - LTC PKHA E 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke_7(&self) -> &LTC0_PKE_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3612usize) as *const LTC0_PKE_7) }
    }
    #[doc = "0xe1c - LTC PKHA E 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke_7_mut(&self) -> &mut LTC0_PKE_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3612usize) as *mut LTC0_PKE_7) }
    }
    #[doc = "0xe1c - LTC PKHA E0 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_7(&self) -> &LTC0_PKE0_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3612usize) as *const LTC0_PKE0_7) }
    }
    #[doc = "0xe1c - LTC PKHA E0 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_7_mut(&self) -> &mut LTC0_PKE0_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3612usize) as *mut LTC0_PKE0_7) }
    }
    #[doc = "0xe20 - LTC PKHA E 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke_8(&self) -> &LTC0_PKE_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3616usize) as *const LTC0_PKE_8) }
    }
    #[doc = "0xe20 - LTC PKHA E 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke_8_mut(&self) -> &mut LTC0_PKE_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3616usize) as *mut LTC0_PKE_8) }
    }
    #[doc = "0xe20 - LTC PKHA E0 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_8(&self) -> &LTC0_PKE0_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3616usize) as *const LTC0_PKE0_8) }
    }
    #[doc = "0xe20 - LTC PKHA E0 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_8_mut(&self) -> &mut LTC0_PKE0_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3616usize) as *mut LTC0_PKE0_8) }
    }
    #[doc = "0xe24 - LTC PKHA E 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke_9(&self) -> &LTC0_PKE_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3620usize) as *const LTC0_PKE_9) }
    }
    #[doc = "0xe24 - LTC PKHA E 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke_9_mut(&self) -> &mut LTC0_PKE_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3620usize) as *mut LTC0_PKE_9) }
    }
    #[doc = "0xe24 - LTC PKHA E0 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_9(&self) -> &LTC0_PKE0_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3620usize) as *const LTC0_PKE0_9) }
    }
    #[doc = "0xe24 - LTC PKHA E0 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_9_mut(&self) -> &mut LTC0_PKE0_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3620usize) as *mut LTC0_PKE0_9) }
    }
    #[doc = "0xe28 - LTC PKHA E 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke_10(&self) -> &LTC0_PKE_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3624usize) as *const LTC0_PKE_10) }
    }
    #[doc = "0xe28 - LTC PKHA E 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke_10_mut(&self) -> &mut LTC0_PKE_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3624usize) as *mut LTC0_PKE_10) }
    }
    #[doc = "0xe28 - LTC PKHA E0 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_10(&self) -> &LTC0_PKE0_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3624usize) as *const LTC0_PKE0_10) }
    }
    #[doc = "0xe28 - LTC PKHA E0 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_10_mut(&self) -> &mut LTC0_PKE0_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3624usize) as *mut LTC0_PKE0_10) }
    }
    #[doc = "0xe2c - LTC PKHA E 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke_11(&self) -> &LTC0_PKE_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3628usize) as *const LTC0_PKE_11) }
    }
    #[doc = "0xe2c - LTC PKHA E 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke_11_mut(&self) -> &mut LTC0_PKE_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3628usize) as *mut LTC0_PKE_11) }
    }
    #[doc = "0xe2c - LTC PKHA E0 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_11(&self) -> &LTC0_PKE0_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3628usize) as *const LTC0_PKE0_11) }
    }
    #[doc = "0xe2c - LTC PKHA E0 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_11_mut(&self) -> &mut LTC0_PKE0_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3628usize) as *mut LTC0_PKE0_11) }
    }
    #[doc = "0xe30 - LTC PKHA E 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke_12(&self) -> &LTC0_PKE_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3632usize) as *const LTC0_PKE_12) }
    }
    #[doc = "0xe30 - LTC PKHA E 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke_12_mut(&self) -> &mut LTC0_PKE_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3632usize) as *mut LTC0_PKE_12) }
    }
    #[doc = "0xe30 - LTC PKHA E0 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_12(&self) -> &LTC0_PKE0_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3632usize) as *const LTC0_PKE0_12) }
    }
    #[doc = "0xe30 - LTC PKHA E0 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_12_mut(&self) -> &mut LTC0_PKE0_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3632usize) as *mut LTC0_PKE0_12) }
    }
    #[doc = "0xe34 - LTC PKHA E 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke_13(&self) -> &LTC0_PKE_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3636usize) as *const LTC0_PKE_13) }
    }
    #[doc = "0xe34 - LTC PKHA E 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke_13_mut(&self) -> &mut LTC0_PKE_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3636usize) as *mut LTC0_PKE_13) }
    }
    #[doc = "0xe34 - LTC PKHA E0 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_13(&self) -> &LTC0_PKE0_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3636usize) as *const LTC0_PKE0_13) }
    }
    #[doc = "0xe34 - LTC PKHA E0 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_13_mut(&self) -> &mut LTC0_PKE0_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3636usize) as *mut LTC0_PKE0_13) }
    }
    #[doc = "0xe38 - LTC PKHA E 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke_14(&self) -> &LTC0_PKE_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3640usize) as *const LTC0_PKE_14) }
    }
    #[doc = "0xe38 - LTC PKHA E 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke_14_mut(&self) -> &mut LTC0_PKE_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3640usize) as *mut LTC0_PKE_14) }
    }
    #[doc = "0xe38 - LTC PKHA E0 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_14(&self) -> &LTC0_PKE0_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3640usize) as *const LTC0_PKE0_14) }
    }
    #[doc = "0xe38 - LTC PKHA E0 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_14_mut(&self) -> &mut LTC0_PKE0_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3640usize) as *mut LTC0_PKE0_14) }
    }
    #[doc = "0xe3c - LTC PKHA E 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke_15(&self) -> &LTC0_PKE_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3644usize) as *const LTC0_PKE_15) }
    }
    #[doc = "0xe3c - LTC PKHA E 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke_15_mut(&self) -> &mut LTC0_PKE_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3644usize) as *mut LTC0_PKE_15) }
    }
    #[doc = "0xe3c - LTC PKHA E0 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_15(&self) -> &LTC0_PKE0_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3644usize) as *const LTC0_PKE0_15) }
    }
    #[doc = "0xe3c - LTC PKHA E0 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke0_15_mut(&self) -> &mut LTC0_PKE0_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3644usize) as *mut LTC0_PKE0_15) }
    }
    #[doc = "0xe40 - LTC PKHA E 16 Register"]
    #[inline(always)]
    pub fn ltc0_pke_16(&self) -> &LTC0_PKE_16 {
        unsafe { &*(((self as *const Self) as *const u8).add(3648usize) as *const LTC0_PKE_16) }
    }
    #[doc = "0xe40 - LTC PKHA E 16 Register"]
    #[inline(always)]
    pub fn ltc0_pke_16_mut(&self) -> &mut LTC0_PKE_16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3648usize) as *mut LTC0_PKE_16) }
    }
    #[doc = "0xe40 - LTC PKHA E1 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_0(&self) -> &LTC0_PKE1_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3648usize) as *const LTC0_PKE1_0) }
    }
    #[doc = "0xe40 - LTC PKHA E1 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_0_mut(&self) -> &mut LTC0_PKE1_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3648usize) as *mut LTC0_PKE1_0) }
    }
    #[doc = "0xe44 - LTC PKHA E 17 Register"]
    #[inline(always)]
    pub fn ltc0_pke_17(&self) -> &LTC0_PKE_17 {
        unsafe { &*(((self as *const Self) as *const u8).add(3652usize) as *const LTC0_PKE_17) }
    }
    #[doc = "0xe44 - LTC PKHA E 17 Register"]
    #[inline(always)]
    pub fn ltc0_pke_17_mut(&self) -> &mut LTC0_PKE_17 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3652usize) as *mut LTC0_PKE_17) }
    }
    #[doc = "0xe44 - LTC PKHA E1 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_1(&self) -> &LTC0_PKE1_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3652usize) as *const LTC0_PKE1_1) }
    }
    #[doc = "0xe44 - LTC PKHA E1 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_1_mut(&self) -> &mut LTC0_PKE1_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3652usize) as *mut LTC0_PKE1_1) }
    }
    #[doc = "0xe48 - LTC PKHA E 18 Register"]
    #[inline(always)]
    pub fn ltc0_pke_18(&self) -> &LTC0_PKE_18 {
        unsafe { &*(((self as *const Self) as *const u8).add(3656usize) as *const LTC0_PKE_18) }
    }
    #[doc = "0xe48 - LTC PKHA E 18 Register"]
    #[inline(always)]
    pub fn ltc0_pke_18_mut(&self) -> &mut LTC0_PKE_18 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3656usize) as *mut LTC0_PKE_18) }
    }
    #[doc = "0xe48 - LTC PKHA E1 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_2(&self) -> &LTC0_PKE1_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3656usize) as *const LTC0_PKE1_2) }
    }
    #[doc = "0xe48 - LTC PKHA E1 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_2_mut(&self) -> &mut LTC0_PKE1_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3656usize) as *mut LTC0_PKE1_2) }
    }
    #[doc = "0xe4c - LTC PKHA E 19 Register"]
    #[inline(always)]
    pub fn ltc0_pke_19(&self) -> &LTC0_PKE_19 {
        unsafe { &*(((self as *const Self) as *const u8).add(3660usize) as *const LTC0_PKE_19) }
    }
    #[doc = "0xe4c - LTC PKHA E 19 Register"]
    #[inline(always)]
    pub fn ltc0_pke_19_mut(&self) -> &mut LTC0_PKE_19 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3660usize) as *mut LTC0_PKE_19) }
    }
    #[doc = "0xe4c - LTC PKHA E1 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_3(&self) -> &LTC0_PKE1_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3660usize) as *const LTC0_PKE1_3) }
    }
    #[doc = "0xe4c - LTC PKHA E1 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_3_mut(&self) -> &mut LTC0_PKE1_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3660usize) as *mut LTC0_PKE1_3) }
    }
    #[doc = "0xe50 - LTC PKHA E 20 Register"]
    #[inline(always)]
    pub fn ltc0_pke_20(&self) -> &LTC0_PKE_20 {
        unsafe { &*(((self as *const Self) as *const u8).add(3664usize) as *const LTC0_PKE_20) }
    }
    #[doc = "0xe50 - LTC PKHA E 20 Register"]
    #[inline(always)]
    pub fn ltc0_pke_20_mut(&self) -> &mut LTC0_PKE_20 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3664usize) as *mut LTC0_PKE_20) }
    }
    #[doc = "0xe50 - LTC PKHA E1 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_4(&self) -> &LTC0_PKE1_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3664usize) as *const LTC0_PKE1_4) }
    }
    #[doc = "0xe50 - LTC PKHA E1 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_4_mut(&self) -> &mut LTC0_PKE1_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3664usize) as *mut LTC0_PKE1_4) }
    }
    #[doc = "0xe54 - LTC PKHA E 21 Register"]
    #[inline(always)]
    pub fn ltc0_pke_21(&self) -> &LTC0_PKE_21 {
        unsafe { &*(((self as *const Self) as *const u8).add(3668usize) as *const LTC0_PKE_21) }
    }
    #[doc = "0xe54 - LTC PKHA E 21 Register"]
    #[inline(always)]
    pub fn ltc0_pke_21_mut(&self) -> &mut LTC0_PKE_21 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3668usize) as *mut LTC0_PKE_21) }
    }
    #[doc = "0xe54 - LTC PKHA E1 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_5(&self) -> &LTC0_PKE1_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3668usize) as *const LTC0_PKE1_5) }
    }
    #[doc = "0xe54 - LTC PKHA E1 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_5_mut(&self) -> &mut LTC0_PKE1_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3668usize) as *mut LTC0_PKE1_5) }
    }
    #[doc = "0xe58 - LTC PKHA E 22 Register"]
    #[inline(always)]
    pub fn ltc0_pke_22(&self) -> &LTC0_PKE_22 {
        unsafe { &*(((self as *const Self) as *const u8).add(3672usize) as *const LTC0_PKE_22) }
    }
    #[doc = "0xe58 - LTC PKHA E 22 Register"]
    #[inline(always)]
    pub fn ltc0_pke_22_mut(&self) -> &mut LTC0_PKE_22 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3672usize) as *mut LTC0_PKE_22) }
    }
    #[doc = "0xe58 - LTC PKHA E1 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_6(&self) -> &LTC0_PKE1_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3672usize) as *const LTC0_PKE1_6) }
    }
    #[doc = "0xe58 - LTC PKHA E1 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_6_mut(&self) -> &mut LTC0_PKE1_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3672usize) as *mut LTC0_PKE1_6) }
    }
    #[doc = "0xe5c - LTC PKHA E 23 Register"]
    #[inline(always)]
    pub fn ltc0_pke_23(&self) -> &LTC0_PKE_23 {
        unsafe { &*(((self as *const Self) as *const u8).add(3676usize) as *const LTC0_PKE_23) }
    }
    #[doc = "0xe5c - LTC PKHA E 23 Register"]
    #[inline(always)]
    pub fn ltc0_pke_23_mut(&self) -> &mut LTC0_PKE_23 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3676usize) as *mut LTC0_PKE_23) }
    }
    #[doc = "0xe5c - LTC PKHA E1 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_7(&self) -> &LTC0_PKE1_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3676usize) as *const LTC0_PKE1_7) }
    }
    #[doc = "0xe5c - LTC PKHA E1 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_7_mut(&self) -> &mut LTC0_PKE1_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3676usize) as *mut LTC0_PKE1_7) }
    }
    #[doc = "0xe60 - LTC PKHA E 24 Register"]
    #[inline(always)]
    pub fn ltc0_pke_24(&self) -> &LTC0_PKE_24 {
        unsafe { &*(((self as *const Self) as *const u8).add(3680usize) as *const LTC0_PKE_24) }
    }
    #[doc = "0xe60 - LTC PKHA E 24 Register"]
    #[inline(always)]
    pub fn ltc0_pke_24_mut(&self) -> &mut LTC0_PKE_24 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3680usize) as *mut LTC0_PKE_24) }
    }
    #[doc = "0xe60 - LTC PKHA E1 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_8(&self) -> &LTC0_PKE1_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3680usize) as *const LTC0_PKE1_8) }
    }
    #[doc = "0xe60 - LTC PKHA E1 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_8_mut(&self) -> &mut LTC0_PKE1_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3680usize) as *mut LTC0_PKE1_8) }
    }
    #[doc = "0xe64 - LTC PKHA E 25 Register"]
    #[inline(always)]
    pub fn ltc0_pke_25(&self) -> &LTC0_PKE_25 {
        unsafe { &*(((self as *const Self) as *const u8).add(3684usize) as *const LTC0_PKE_25) }
    }
    #[doc = "0xe64 - LTC PKHA E 25 Register"]
    #[inline(always)]
    pub fn ltc0_pke_25_mut(&self) -> &mut LTC0_PKE_25 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3684usize) as *mut LTC0_PKE_25) }
    }
    #[doc = "0xe64 - LTC PKHA E1 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_9(&self) -> &LTC0_PKE1_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3684usize) as *const LTC0_PKE1_9) }
    }
    #[doc = "0xe64 - LTC PKHA E1 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_9_mut(&self) -> &mut LTC0_PKE1_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3684usize) as *mut LTC0_PKE1_9) }
    }
    #[doc = "0xe68 - LTC PKHA E 26 Register"]
    #[inline(always)]
    pub fn ltc0_pke_26(&self) -> &LTC0_PKE_26 {
        unsafe { &*(((self as *const Self) as *const u8).add(3688usize) as *const LTC0_PKE_26) }
    }
    #[doc = "0xe68 - LTC PKHA E 26 Register"]
    #[inline(always)]
    pub fn ltc0_pke_26_mut(&self) -> &mut LTC0_PKE_26 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3688usize) as *mut LTC0_PKE_26) }
    }
    #[doc = "0xe68 - LTC PKHA E1 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_10(&self) -> &LTC0_PKE1_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3688usize) as *const LTC0_PKE1_10) }
    }
    #[doc = "0xe68 - LTC PKHA E1 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_10_mut(&self) -> &mut LTC0_PKE1_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3688usize) as *mut LTC0_PKE1_10) }
    }
    #[doc = "0xe6c - LTC PKHA E 27 Register"]
    #[inline(always)]
    pub fn ltc0_pke_27(&self) -> &LTC0_PKE_27 {
        unsafe { &*(((self as *const Self) as *const u8).add(3692usize) as *const LTC0_PKE_27) }
    }
    #[doc = "0xe6c - LTC PKHA E 27 Register"]
    #[inline(always)]
    pub fn ltc0_pke_27_mut(&self) -> &mut LTC0_PKE_27 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3692usize) as *mut LTC0_PKE_27) }
    }
    #[doc = "0xe6c - LTC PKHA E1 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_11(&self) -> &LTC0_PKE1_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3692usize) as *const LTC0_PKE1_11) }
    }
    #[doc = "0xe6c - LTC PKHA E1 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_11_mut(&self) -> &mut LTC0_PKE1_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3692usize) as *mut LTC0_PKE1_11) }
    }
    #[doc = "0xe70 - LTC PKHA E 28 Register"]
    #[inline(always)]
    pub fn ltc0_pke_28(&self) -> &LTC0_PKE_28 {
        unsafe { &*(((self as *const Self) as *const u8).add(3696usize) as *const LTC0_PKE_28) }
    }
    #[doc = "0xe70 - LTC PKHA E 28 Register"]
    #[inline(always)]
    pub fn ltc0_pke_28_mut(&self) -> &mut LTC0_PKE_28 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3696usize) as *mut LTC0_PKE_28) }
    }
    #[doc = "0xe70 - LTC PKHA E1 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_12(&self) -> &LTC0_PKE1_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3696usize) as *const LTC0_PKE1_12) }
    }
    #[doc = "0xe70 - LTC PKHA E1 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_12_mut(&self) -> &mut LTC0_PKE1_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3696usize) as *mut LTC0_PKE1_12) }
    }
    #[doc = "0xe74 - LTC PKHA E 29 Register"]
    #[inline(always)]
    pub fn ltc0_pke_29(&self) -> &LTC0_PKE_29 {
        unsafe { &*(((self as *const Self) as *const u8).add(3700usize) as *const LTC0_PKE_29) }
    }
    #[doc = "0xe74 - LTC PKHA E 29 Register"]
    #[inline(always)]
    pub fn ltc0_pke_29_mut(&self) -> &mut LTC0_PKE_29 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3700usize) as *mut LTC0_PKE_29) }
    }
    #[doc = "0xe74 - LTC PKHA E1 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_13(&self) -> &LTC0_PKE1_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3700usize) as *const LTC0_PKE1_13) }
    }
    #[doc = "0xe74 - LTC PKHA E1 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_13_mut(&self) -> &mut LTC0_PKE1_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3700usize) as *mut LTC0_PKE1_13) }
    }
    #[doc = "0xe78 - LTC PKHA E 30 Register"]
    #[inline(always)]
    pub fn ltc0_pke_30(&self) -> &LTC0_PKE_30 {
        unsafe { &*(((self as *const Self) as *const u8).add(3704usize) as *const LTC0_PKE_30) }
    }
    #[doc = "0xe78 - LTC PKHA E 30 Register"]
    #[inline(always)]
    pub fn ltc0_pke_30_mut(&self) -> &mut LTC0_PKE_30 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3704usize) as *mut LTC0_PKE_30) }
    }
    #[doc = "0xe78 - LTC PKHA E1 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_14(&self) -> &LTC0_PKE1_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3704usize) as *const LTC0_PKE1_14) }
    }
    #[doc = "0xe78 - LTC PKHA E1 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_14_mut(&self) -> &mut LTC0_PKE1_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3704usize) as *mut LTC0_PKE1_14) }
    }
    #[doc = "0xe7c - LTC PKHA E 31 Register"]
    #[inline(always)]
    pub fn ltc0_pke_31(&self) -> &LTC0_PKE_31 {
        unsafe { &*(((self as *const Self) as *const u8).add(3708usize) as *const LTC0_PKE_31) }
    }
    #[doc = "0xe7c - LTC PKHA E 31 Register"]
    #[inline(always)]
    pub fn ltc0_pke_31_mut(&self) -> &mut LTC0_PKE_31 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3708usize) as *mut LTC0_PKE_31) }
    }
    #[doc = "0xe7c - LTC PKHA E1 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_15(&self) -> &LTC0_PKE1_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3708usize) as *const LTC0_PKE1_15) }
    }
    #[doc = "0xe7c - LTC PKHA E1 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke1_15_mut(&self) -> &mut LTC0_PKE1_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3708usize) as *mut LTC0_PKE1_15) }
    }
    #[doc = "0xe80 - LTC PKHA E 32 Register"]
    #[inline(always)]
    pub fn ltc0_pke_32(&self) -> &LTC0_PKE_32 {
        unsafe { &*(((self as *const Self) as *const u8).add(3712usize) as *const LTC0_PKE_32) }
    }
    #[doc = "0xe80 - LTC PKHA E 32 Register"]
    #[inline(always)]
    pub fn ltc0_pke_32_mut(&self) -> &mut LTC0_PKE_32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3712usize) as *mut LTC0_PKE_32) }
    }
    #[doc = "0xe80 - LTC PKHA E2 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_0(&self) -> &LTC0_PKE2_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3712usize) as *const LTC0_PKE2_0) }
    }
    #[doc = "0xe80 - LTC PKHA E2 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_0_mut(&self) -> &mut LTC0_PKE2_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3712usize) as *mut LTC0_PKE2_0) }
    }
    #[doc = "0xe84 - LTC PKHA E 33 Register"]
    #[inline(always)]
    pub fn ltc0_pke_33(&self) -> &LTC0_PKE_33 {
        unsafe { &*(((self as *const Self) as *const u8).add(3716usize) as *const LTC0_PKE_33) }
    }
    #[doc = "0xe84 - LTC PKHA E 33 Register"]
    #[inline(always)]
    pub fn ltc0_pke_33_mut(&self) -> &mut LTC0_PKE_33 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3716usize) as *mut LTC0_PKE_33) }
    }
    #[doc = "0xe84 - LTC PKHA E2 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_1(&self) -> &LTC0_PKE2_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3716usize) as *const LTC0_PKE2_1) }
    }
    #[doc = "0xe84 - LTC PKHA E2 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_1_mut(&self) -> &mut LTC0_PKE2_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3716usize) as *mut LTC0_PKE2_1) }
    }
    #[doc = "0xe88 - LTC PKHA E 34 Register"]
    #[inline(always)]
    pub fn ltc0_pke_34(&self) -> &LTC0_PKE_34 {
        unsafe { &*(((self as *const Self) as *const u8).add(3720usize) as *const LTC0_PKE_34) }
    }
    #[doc = "0xe88 - LTC PKHA E 34 Register"]
    #[inline(always)]
    pub fn ltc0_pke_34_mut(&self) -> &mut LTC0_PKE_34 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3720usize) as *mut LTC0_PKE_34) }
    }
    #[doc = "0xe88 - LTC PKHA E2 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_2(&self) -> &LTC0_PKE2_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3720usize) as *const LTC0_PKE2_2) }
    }
    #[doc = "0xe88 - LTC PKHA E2 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_2_mut(&self) -> &mut LTC0_PKE2_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3720usize) as *mut LTC0_PKE2_2) }
    }
    #[doc = "0xe8c - LTC PKHA E 35 Register"]
    #[inline(always)]
    pub fn ltc0_pke_35(&self) -> &LTC0_PKE_35 {
        unsafe { &*(((self as *const Self) as *const u8).add(3724usize) as *const LTC0_PKE_35) }
    }
    #[doc = "0xe8c - LTC PKHA E 35 Register"]
    #[inline(always)]
    pub fn ltc0_pke_35_mut(&self) -> &mut LTC0_PKE_35 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3724usize) as *mut LTC0_PKE_35) }
    }
    #[doc = "0xe8c - LTC PKHA E2 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_3(&self) -> &LTC0_PKE2_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3724usize) as *const LTC0_PKE2_3) }
    }
    #[doc = "0xe8c - LTC PKHA E2 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_3_mut(&self) -> &mut LTC0_PKE2_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3724usize) as *mut LTC0_PKE2_3) }
    }
    #[doc = "0xe90 - LTC PKHA E 36 Register"]
    #[inline(always)]
    pub fn ltc0_pke_36(&self) -> &LTC0_PKE_36 {
        unsafe { &*(((self as *const Self) as *const u8).add(3728usize) as *const LTC0_PKE_36) }
    }
    #[doc = "0xe90 - LTC PKHA E 36 Register"]
    #[inline(always)]
    pub fn ltc0_pke_36_mut(&self) -> &mut LTC0_PKE_36 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3728usize) as *mut LTC0_PKE_36) }
    }
    #[doc = "0xe90 - LTC PKHA E2 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_4(&self) -> &LTC0_PKE2_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3728usize) as *const LTC0_PKE2_4) }
    }
    #[doc = "0xe90 - LTC PKHA E2 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_4_mut(&self) -> &mut LTC0_PKE2_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3728usize) as *mut LTC0_PKE2_4) }
    }
    #[doc = "0xe94 - LTC PKHA E 37 Register"]
    #[inline(always)]
    pub fn ltc0_pke_37(&self) -> &LTC0_PKE_37 {
        unsafe { &*(((self as *const Self) as *const u8).add(3732usize) as *const LTC0_PKE_37) }
    }
    #[doc = "0xe94 - LTC PKHA E 37 Register"]
    #[inline(always)]
    pub fn ltc0_pke_37_mut(&self) -> &mut LTC0_PKE_37 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3732usize) as *mut LTC0_PKE_37) }
    }
    #[doc = "0xe94 - LTC PKHA E2 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_5(&self) -> &LTC0_PKE2_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3732usize) as *const LTC0_PKE2_5) }
    }
    #[doc = "0xe94 - LTC PKHA E2 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_5_mut(&self) -> &mut LTC0_PKE2_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3732usize) as *mut LTC0_PKE2_5) }
    }
    #[doc = "0xe98 - LTC PKHA E 38 Register"]
    #[inline(always)]
    pub fn ltc0_pke_38(&self) -> &LTC0_PKE_38 {
        unsafe { &*(((self as *const Self) as *const u8).add(3736usize) as *const LTC0_PKE_38) }
    }
    #[doc = "0xe98 - LTC PKHA E 38 Register"]
    #[inline(always)]
    pub fn ltc0_pke_38_mut(&self) -> &mut LTC0_PKE_38 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3736usize) as *mut LTC0_PKE_38) }
    }
    #[doc = "0xe98 - LTC PKHA E2 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_6(&self) -> &LTC0_PKE2_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3736usize) as *const LTC0_PKE2_6) }
    }
    #[doc = "0xe98 - LTC PKHA E2 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_6_mut(&self) -> &mut LTC0_PKE2_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3736usize) as *mut LTC0_PKE2_6) }
    }
    #[doc = "0xe9c - LTC PKHA E 39 Register"]
    #[inline(always)]
    pub fn ltc0_pke_39(&self) -> &LTC0_PKE_39 {
        unsafe { &*(((self as *const Self) as *const u8).add(3740usize) as *const LTC0_PKE_39) }
    }
    #[doc = "0xe9c - LTC PKHA E 39 Register"]
    #[inline(always)]
    pub fn ltc0_pke_39_mut(&self) -> &mut LTC0_PKE_39 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3740usize) as *mut LTC0_PKE_39) }
    }
    #[doc = "0xe9c - LTC PKHA E2 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_7(&self) -> &LTC0_PKE2_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3740usize) as *const LTC0_PKE2_7) }
    }
    #[doc = "0xe9c - LTC PKHA E2 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_7_mut(&self) -> &mut LTC0_PKE2_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3740usize) as *mut LTC0_PKE2_7) }
    }
    #[doc = "0xea0 - LTC PKHA E 40 Register"]
    #[inline(always)]
    pub fn ltc0_pke_40(&self) -> &LTC0_PKE_40 {
        unsafe { &*(((self as *const Self) as *const u8).add(3744usize) as *const LTC0_PKE_40) }
    }
    #[doc = "0xea0 - LTC PKHA E 40 Register"]
    #[inline(always)]
    pub fn ltc0_pke_40_mut(&self) -> &mut LTC0_PKE_40 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3744usize) as *mut LTC0_PKE_40) }
    }
    #[doc = "0xea0 - LTC PKHA E2 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_8(&self) -> &LTC0_PKE2_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3744usize) as *const LTC0_PKE2_8) }
    }
    #[doc = "0xea0 - LTC PKHA E2 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_8_mut(&self) -> &mut LTC0_PKE2_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3744usize) as *mut LTC0_PKE2_8) }
    }
    #[doc = "0xea4 - LTC PKHA E 41 Register"]
    #[inline(always)]
    pub fn ltc0_pke_41(&self) -> &LTC0_PKE_41 {
        unsafe { &*(((self as *const Self) as *const u8).add(3748usize) as *const LTC0_PKE_41) }
    }
    #[doc = "0xea4 - LTC PKHA E 41 Register"]
    #[inline(always)]
    pub fn ltc0_pke_41_mut(&self) -> &mut LTC0_PKE_41 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3748usize) as *mut LTC0_PKE_41) }
    }
    #[doc = "0xea4 - LTC PKHA E2 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_9(&self) -> &LTC0_PKE2_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3748usize) as *const LTC0_PKE2_9) }
    }
    #[doc = "0xea4 - LTC PKHA E2 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_9_mut(&self) -> &mut LTC0_PKE2_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3748usize) as *mut LTC0_PKE2_9) }
    }
    #[doc = "0xea8 - LTC PKHA E 42 Register"]
    #[inline(always)]
    pub fn ltc0_pke_42(&self) -> &LTC0_PKE_42 {
        unsafe { &*(((self as *const Self) as *const u8).add(3752usize) as *const LTC0_PKE_42) }
    }
    #[doc = "0xea8 - LTC PKHA E 42 Register"]
    #[inline(always)]
    pub fn ltc0_pke_42_mut(&self) -> &mut LTC0_PKE_42 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3752usize) as *mut LTC0_PKE_42) }
    }
    #[doc = "0xea8 - LTC PKHA E2 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_10(&self) -> &LTC0_PKE2_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3752usize) as *const LTC0_PKE2_10) }
    }
    #[doc = "0xea8 - LTC PKHA E2 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_10_mut(&self) -> &mut LTC0_PKE2_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3752usize) as *mut LTC0_PKE2_10) }
    }
    #[doc = "0xeac - LTC PKHA E 43 Register"]
    #[inline(always)]
    pub fn ltc0_pke_43(&self) -> &LTC0_PKE_43 {
        unsafe { &*(((self as *const Self) as *const u8).add(3756usize) as *const LTC0_PKE_43) }
    }
    #[doc = "0xeac - LTC PKHA E 43 Register"]
    #[inline(always)]
    pub fn ltc0_pke_43_mut(&self) -> &mut LTC0_PKE_43 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3756usize) as *mut LTC0_PKE_43) }
    }
    #[doc = "0xeac - LTC PKHA E2 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_11(&self) -> &LTC0_PKE2_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3756usize) as *const LTC0_PKE2_11) }
    }
    #[doc = "0xeac - LTC PKHA E2 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_11_mut(&self) -> &mut LTC0_PKE2_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3756usize) as *mut LTC0_PKE2_11) }
    }
    #[doc = "0xeb0 - LTC PKHA E 44 Register"]
    #[inline(always)]
    pub fn ltc0_pke_44(&self) -> &LTC0_PKE_44 {
        unsafe { &*(((self as *const Self) as *const u8).add(3760usize) as *const LTC0_PKE_44) }
    }
    #[doc = "0xeb0 - LTC PKHA E 44 Register"]
    #[inline(always)]
    pub fn ltc0_pke_44_mut(&self) -> &mut LTC0_PKE_44 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3760usize) as *mut LTC0_PKE_44) }
    }
    #[doc = "0xeb0 - LTC PKHA E2 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_12(&self) -> &LTC0_PKE2_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3760usize) as *const LTC0_PKE2_12) }
    }
    #[doc = "0xeb0 - LTC PKHA E2 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_12_mut(&self) -> &mut LTC0_PKE2_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3760usize) as *mut LTC0_PKE2_12) }
    }
    #[doc = "0xeb4 - LTC PKHA E 45 Register"]
    #[inline(always)]
    pub fn ltc0_pke_45(&self) -> &LTC0_PKE_45 {
        unsafe { &*(((self as *const Self) as *const u8).add(3764usize) as *const LTC0_PKE_45) }
    }
    #[doc = "0xeb4 - LTC PKHA E 45 Register"]
    #[inline(always)]
    pub fn ltc0_pke_45_mut(&self) -> &mut LTC0_PKE_45 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3764usize) as *mut LTC0_PKE_45) }
    }
    #[doc = "0xeb4 - LTC PKHA E2 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_13(&self) -> &LTC0_PKE2_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3764usize) as *const LTC0_PKE2_13) }
    }
    #[doc = "0xeb4 - LTC PKHA E2 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_13_mut(&self) -> &mut LTC0_PKE2_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3764usize) as *mut LTC0_PKE2_13) }
    }
    #[doc = "0xeb8 - LTC PKHA E 46 Register"]
    #[inline(always)]
    pub fn ltc0_pke_46(&self) -> &LTC0_PKE_46 {
        unsafe { &*(((self as *const Self) as *const u8).add(3768usize) as *const LTC0_PKE_46) }
    }
    #[doc = "0xeb8 - LTC PKHA E 46 Register"]
    #[inline(always)]
    pub fn ltc0_pke_46_mut(&self) -> &mut LTC0_PKE_46 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3768usize) as *mut LTC0_PKE_46) }
    }
    #[doc = "0xeb8 - LTC PKHA E2 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_14(&self) -> &LTC0_PKE2_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3768usize) as *const LTC0_PKE2_14) }
    }
    #[doc = "0xeb8 - LTC PKHA E2 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_14_mut(&self) -> &mut LTC0_PKE2_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3768usize) as *mut LTC0_PKE2_14) }
    }
    #[doc = "0xebc - LTC PKHA E 47 Register"]
    #[inline(always)]
    pub fn ltc0_pke_47(&self) -> &LTC0_PKE_47 {
        unsafe { &*(((self as *const Self) as *const u8).add(3772usize) as *const LTC0_PKE_47) }
    }
    #[doc = "0xebc - LTC PKHA E 47 Register"]
    #[inline(always)]
    pub fn ltc0_pke_47_mut(&self) -> &mut LTC0_PKE_47 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3772usize) as *mut LTC0_PKE_47) }
    }
    #[doc = "0xebc - LTC PKHA E2 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_15(&self) -> &LTC0_PKE2_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3772usize) as *const LTC0_PKE2_15) }
    }
    #[doc = "0xebc - LTC PKHA E2 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke2_15_mut(&self) -> &mut LTC0_PKE2_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3772usize) as *mut LTC0_PKE2_15) }
    }
    #[doc = "0xec0 - LTC PKHA E 48 Register"]
    #[inline(always)]
    pub fn ltc0_pke_48(&self) -> &LTC0_PKE_48 {
        unsafe { &*(((self as *const Self) as *const u8).add(3776usize) as *const LTC0_PKE_48) }
    }
    #[doc = "0xec0 - LTC PKHA E 48 Register"]
    #[inline(always)]
    pub fn ltc0_pke_48_mut(&self) -> &mut LTC0_PKE_48 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3776usize) as *mut LTC0_PKE_48) }
    }
    #[doc = "0xec0 - LTC PKHA E3 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_0(&self) -> &LTC0_PKE3_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(3776usize) as *const LTC0_PKE3_0) }
    }
    #[doc = "0xec0 - LTC PKHA E3 0 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_0_mut(&self) -> &mut LTC0_PKE3_0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3776usize) as *mut LTC0_PKE3_0) }
    }
    #[doc = "0xec4 - LTC PKHA E 49 Register"]
    #[inline(always)]
    pub fn ltc0_pke_49(&self) -> &LTC0_PKE_49 {
        unsafe { &*(((self as *const Self) as *const u8).add(3780usize) as *const LTC0_PKE_49) }
    }
    #[doc = "0xec4 - LTC PKHA E 49 Register"]
    #[inline(always)]
    pub fn ltc0_pke_49_mut(&self) -> &mut LTC0_PKE_49 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3780usize) as *mut LTC0_PKE_49) }
    }
    #[doc = "0xec4 - LTC PKHA E3 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_1(&self) -> &LTC0_PKE3_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3780usize) as *const LTC0_PKE3_1) }
    }
    #[doc = "0xec4 - LTC PKHA E3 1 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_1_mut(&self) -> &mut LTC0_PKE3_1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3780usize) as *mut LTC0_PKE3_1) }
    }
    #[doc = "0xec8 - LTC PKHA E 50 Register"]
    #[inline(always)]
    pub fn ltc0_pke_50(&self) -> &LTC0_PKE_50 {
        unsafe { &*(((self as *const Self) as *const u8).add(3784usize) as *const LTC0_PKE_50) }
    }
    #[doc = "0xec8 - LTC PKHA E 50 Register"]
    #[inline(always)]
    pub fn ltc0_pke_50_mut(&self) -> &mut LTC0_PKE_50 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3784usize) as *mut LTC0_PKE_50) }
    }
    #[doc = "0xec8 - LTC PKHA E3 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_2(&self) -> &LTC0_PKE3_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3784usize) as *const LTC0_PKE3_2) }
    }
    #[doc = "0xec8 - LTC PKHA E3 2 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_2_mut(&self) -> &mut LTC0_PKE3_2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3784usize) as *mut LTC0_PKE3_2) }
    }
    #[doc = "0xecc - LTC PKHA E 51 Register"]
    #[inline(always)]
    pub fn ltc0_pke_51(&self) -> &LTC0_PKE_51 {
        unsafe { &*(((self as *const Self) as *const u8).add(3788usize) as *const LTC0_PKE_51) }
    }
    #[doc = "0xecc - LTC PKHA E 51 Register"]
    #[inline(always)]
    pub fn ltc0_pke_51_mut(&self) -> &mut LTC0_PKE_51 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3788usize) as *mut LTC0_PKE_51) }
    }
    #[doc = "0xecc - LTC PKHA E3 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_3(&self) -> &LTC0_PKE3_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(3788usize) as *const LTC0_PKE3_3) }
    }
    #[doc = "0xecc - LTC PKHA E3 3 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_3_mut(&self) -> &mut LTC0_PKE3_3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3788usize) as *mut LTC0_PKE3_3) }
    }
    #[doc = "0xed0 - LTC PKHA E 52 Register"]
    #[inline(always)]
    pub fn ltc0_pke_52(&self) -> &LTC0_PKE_52 {
        unsafe { &*(((self as *const Self) as *const u8).add(3792usize) as *const LTC0_PKE_52) }
    }
    #[doc = "0xed0 - LTC PKHA E 52 Register"]
    #[inline(always)]
    pub fn ltc0_pke_52_mut(&self) -> &mut LTC0_PKE_52 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3792usize) as *mut LTC0_PKE_52) }
    }
    #[doc = "0xed0 - LTC PKHA E3 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_4(&self) -> &LTC0_PKE3_4 {
        unsafe { &*(((self as *const Self) as *const u8).add(3792usize) as *const LTC0_PKE3_4) }
    }
    #[doc = "0xed0 - LTC PKHA E3 4 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_4_mut(&self) -> &mut LTC0_PKE3_4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3792usize) as *mut LTC0_PKE3_4) }
    }
    #[doc = "0xed4 - LTC PKHA E 53 Register"]
    #[inline(always)]
    pub fn ltc0_pke_53(&self) -> &LTC0_PKE_53 {
        unsafe { &*(((self as *const Self) as *const u8).add(3796usize) as *const LTC0_PKE_53) }
    }
    #[doc = "0xed4 - LTC PKHA E 53 Register"]
    #[inline(always)]
    pub fn ltc0_pke_53_mut(&self) -> &mut LTC0_PKE_53 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3796usize) as *mut LTC0_PKE_53) }
    }
    #[doc = "0xed4 - LTC PKHA E3 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_5(&self) -> &LTC0_PKE3_5 {
        unsafe { &*(((self as *const Self) as *const u8).add(3796usize) as *const LTC0_PKE3_5) }
    }
    #[doc = "0xed4 - LTC PKHA E3 5 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_5_mut(&self) -> &mut LTC0_PKE3_5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3796usize) as *mut LTC0_PKE3_5) }
    }
    #[doc = "0xed8 - LTC PKHA E 54 Register"]
    #[inline(always)]
    pub fn ltc0_pke_54(&self) -> &LTC0_PKE_54 {
        unsafe { &*(((self as *const Self) as *const u8).add(3800usize) as *const LTC0_PKE_54) }
    }
    #[doc = "0xed8 - LTC PKHA E 54 Register"]
    #[inline(always)]
    pub fn ltc0_pke_54_mut(&self) -> &mut LTC0_PKE_54 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3800usize) as *mut LTC0_PKE_54) }
    }
    #[doc = "0xed8 - LTC PKHA E3 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_6(&self) -> &LTC0_PKE3_6 {
        unsafe { &*(((self as *const Self) as *const u8).add(3800usize) as *const LTC0_PKE3_6) }
    }
    #[doc = "0xed8 - LTC PKHA E3 6 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_6_mut(&self) -> &mut LTC0_PKE3_6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3800usize) as *mut LTC0_PKE3_6) }
    }
    #[doc = "0xedc - LTC PKHA E 55 Register"]
    #[inline(always)]
    pub fn ltc0_pke_55(&self) -> &LTC0_PKE_55 {
        unsafe { &*(((self as *const Self) as *const u8).add(3804usize) as *const LTC0_PKE_55) }
    }
    #[doc = "0xedc - LTC PKHA E 55 Register"]
    #[inline(always)]
    pub fn ltc0_pke_55_mut(&self) -> &mut LTC0_PKE_55 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3804usize) as *mut LTC0_PKE_55) }
    }
    #[doc = "0xedc - LTC PKHA E3 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_7(&self) -> &LTC0_PKE3_7 {
        unsafe { &*(((self as *const Self) as *const u8).add(3804usize) as *const LTC0_PKE3_7) }
    }
    #[doc = "0xedc - LTC PKHA E3 7 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_7_mut(&self) -> &mut LTC0_PKE3_7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3804usize) as *mut LTC0_PKE3_7) }
    }
    #[doc = "0xee0 - LTC PKHA E 56 Register"]
    #[inline(always)]
    pub fn ltc0_pke_56(&self) -> &LTC0_PKE_56 {
        unsafe { &*(((self as *const Self) as *const u8).add(3808usize) as *const LTC0_PKE_56) }
    }
    #[doc = "0xee0 - LTC PKHA E 56 Register"]
    #[inline(always)]
    pub fn ltc0_pke_56_mut(&self) -> &mut LTC0_PKE_56 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3808usize) as *mut LTC0_PKE_56) }
    }
    #[doc = "0xee0 - LTC PKHA E3 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_8(&self) -> &LTC0_PKE3_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(3808usize) as *const LTC0_PKE3_8) }
    }
    #[doc = "0xee0 - LTC PKHA E3 8 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_8_mut(&self) -> &mut LTC0_PKE3_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3808usize) as *mut LTC0_PKE3_8) }
    }
    #[doc = "0xee4 - LTC PKHA E 57 Register"]
    #[inline(always)]
    pub fn ltc0_pke_57(&self) -> &LTC0_PKE_57 {
        unsafe { &*(((self as *const Self) as *const u8).add(3812usize) as *const LTC0_PKE_57) }
    }
    #[doc = "0xee4 - LTC PKHA E 57 Register"]
    #[inline(always)]
    pub fn ltc0_pke_57_mut(&self) -> &mut LTC0_PKE_57 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3812usize) as *mut LTC0_PKE_57) }
    }
    #[doc = "0xee4 - LTC PKHA E3 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_9(&self) -> &LTC0_PKE3_9 {
        unsafe { &*(((self as *const Self) as *const u8).add(3812usize) as *const LTC0_PKE3_9) }
    }
    #[doc = "0xee4 - LTC PKHA E3 9 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_9_mut(&self) -> &mut LTC0_PKE3_9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3812usize) as *mut LTC0_PKE3_9) }
    }
    #[doc = "0xee8 - LTC PKHA E 58 Register"]
    #[inline(always)]
    pub fn ltc0_pke_58(&self) -> &LTC0_PKE_58 {
        unsafe { &*(((self as *const Self) as *const u8).add(3816usize) as *const LTC0_PKE_58) }
    }
    #[doc = "0xee8 - LTC PKHA E 58 Register"]
    #[inline(always)]
    pub fn ltc0_pke_58_mut(&self) -> &mut LTC0_PKE_58 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3816usize) as *mut LTC0_PKE_58) }
    }
    #[doc = "0xee8 - LTC PKHA E3 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_10(&self) -> &LTC0_PKE3_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(3816usize) as *const LTC0_PKE3_10) }
    }
    #[doc = "0xee8 - LTC PKHA E3 10 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_10_mut(&self) -> &mut LTC0_PKE3_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3816usize) as *mut LTC0_PKE3_10) }
    }
    #[doc = "0xeec - LTC PKHA E 59 Register"]
    #[inline(always)]
    pub fn ltc0_pke_59(&self) -> &LTC0_PKE_59 {
        unsafe { &*(((self as *const Self) as *const u8).add(3820usize) as *const LTC0_PKE_59) }
    }
    #[doc = "0xeec - LTC PKHA E 59 Register"]
    #[inline(always)]
    pub fn ltc0_pke_59_mut(&self) -> &mut LTC0_PKE_59 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3820usize) as *mut LTC0_PKE_59) }
    }
    #[doc = "0xeec - LTC PKHA E3 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_11(&self) -> &LTC0_PKE3_11 {
        unsafe { &*(((self as *const Self) as *const u8).add(3820usize) as *const LTC0_PKE3_11) }
    }
    #[doc = "0xeec - LTC PKHA E3 11 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_11_mut(&self) -> &mut LTC0_PKE3_11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3820usize) as *mut LTC0_PKE3_11) }
    }
    #[doc = "0xef0 - LTC PKHA E 60 Register"]
    #[inline(always)]
    pub fn ltc0_pke_60(&self) -> &LTC0_PKE_60 {
        unsafe { &*(((self as *const Self) as *const u8).add(3824usize) as *const LTC0_PKE_60) }
    }
    #[doc = "0xef0 - LTC PKHA E 60 Register"]
    #[inline(always)]
    pub fn ltc0_pke_60_mut(&self) -> &mut LTC0_PKE_60 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3824usize) as *mut LTC0_PKE_60) }
    }
    #[doc = "0xef0 - LTC PKHA E3 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_12(&self) -> &LTC0_PKE3_12 {
        unsafe { &*(((self as *const Self) as *const u8).add(3824usize) as *const LTC0_PKE3_12) }
    }
    #[doc = "0xef0 - LTC PKHA E3 12 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_12_mut(&self) -> &mut LTC0_PKE3_12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3824usize) as *mut LTC0_PKE3_12) }
    }
    #[doc = "0xef4 - LTC PKHA E 61 Register"]
    #[inline(always)]
    pub fn ltc0_pke_61(&self) -> &LTC0_PKE_61 {
        unsafe { &*(((self as *const Self) as *const u8).add(3828usize) as *const LTC0_PKE_61) }
    }
    #[doc = "0xef4 - LTC PKHA E 61 Register"]
    #[inline(always)]
    pub fn ltc0_pke_61_mut(&self) -> &mut LTC0_PKE_61 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3828usize) as *mut LTC0_PKE_61) }
    }
    #[doc = "0xef4 - LTC PKHA E3 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_13(&self) -> &LTC0_PKE3_13 {
        unsafe { &*(((self as *const Self) as *const u8).add(3828usize) as *const LTC0_PKE3_13) }
    }
    #[doc = "0xef4 - LTC PKHA E3 13 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_13_mut(&self) -> &mut LTC0_PKE3_13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3828usize) as *mut LTC0_PKE3_13) }
    }
    #[doc = "0xef8 - LTC PKHA E 62 Register"]
    #[inline(always)]
    pub fn ltc0_pke_62(&self) -> &LTC0_PKE_62 {
        unsafe { &*(((self as *const Self) as *const u8).add(3832usize) as *const LTC0_PKE_62) }
    }
    #[doc = "0xef8 - LTC PKHA E 62 Register"]
    #[inline(always)]
    pub fn ltc0_pke_62_mut(&self) -> &mut LTC0_PKE_62 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3832usize) as *mut LTC0_PKE_62) }
    }
    #[doc = "0xef8 - LTC PKHA E3 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_14(&self) -> &LTC0_PKE3_14 {
        unsafe { &*(((self as *const Self) as *const u8).add(3832usize) as *const LTC0_PKE3_14) }
    }
    #[doc = "0xef8 - LTC PKHA E3 14 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_14_mut(&self) -> &mut LTC0_PKE3_14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3832usize) as *mut LTC0_PKE3_14) }
    }
    #[doc = "0xefc - LTC PKHA E 63 Register"]
    #[inline(always)]
    pub fn ltc0_pke_63(&self) -> &LTC0_PKE_63 {
        unsafe { &*(((self as *const Self) as *const u8).add(3836usize) as *const LTC0_PKE_63) }
    }
    #[doc = "0xefc - LTC PKHA E 63 Register"]
    #[inline(always)]
    pub fn ltc0_pke_63_mut(&self) -> &mut LTC0_PKE_63 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3836usize) as *mut LTC0_PKE_63) }
    }
    #[doc = "0xefc - LTC PKHA E3 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_15(&self) -> &LTC0_PKE3_15 {
        unsafe { &*(((self as *const Self) as *const u8).add(3836usize) as *const LTC0_PKE3_15) }
    }
    #[doc = "0xefc - LTC PKHA E3 15 Register"]
    #[inline(always)]
    pub fn ltc0_pke3_15_mut(&self) -> &mut LTC0_PKE3_15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3836usize) as *mut LTC0_PKE3_15) }
    }
}
#[doc = "LTC Mode Register (non-PKHA/non-RNG use)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_md](ltc0_md) module"]
pub type LTC0_MD = crate::Reg<u32, _LTC0_MD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_MD;
#[doc = "`read()` method returns [ltc0_md::R](ltc0_md::R) reader structure"]
impl crate::Readable for LTC0_MD {}
#[doc = "`write(|w| ..)` method takes [ltc0_md::W](ltc0_md::W) writer structure"]
impl crate::Writable for LTC0_MD {}
#[doc = "LTC Mode Register (non-PKHA/non-RNG use)"]
pub mod ltc0_md;
#[doc = "LTC Mode Register (PublicKey)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_mdpk](ltc0_mdpk) module"]
pub type LTC0_MDPK = crate::Reg<u32, _LTC0_MDPK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_MDPK;
#[doc = "`read()` method returns [ltc0_mdpk::R](ltc0_mdpk::R) reader structure"]
impl crate::Readable for LTC0_MDPK {}
#[doc = "`write(|w| ..)` method takes [ltc0_mdpk::W](ltc0_mdpk::W) writer structure"]
impl crate::Writable for LTC0_MDPK {}
#[doc = "LTC Mode Register (PublicKey)"]
pub mod ltc0_mdpk;
#[doc = "LTC Key Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ks](ltc0_ks) module"]
pub type LTC0_KS = crate::Reg<u32, _LTC0_KS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KS;
#[doc = "`read()` method returns [ltc0_ks::R](ltc0_ks::R) reader structure"]
impl crate::Readable for LTC0_KS {}
#[doc = "`write(|w| ..)` method takes [ltc0_ks::W](ltc0_ks::W) writer structure"]
impl crate::Writable for LTC0_KS {}
#[doc = "LTC Key Size Register"]
pub mod ltc0_ks;
#[doc = "LTC Data Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ds](ltc0_ds) module"]
pub type LTC0_DS = crate::Reg<u32, _LTC0_DS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_DS;
#[doc = "`read()` method returns [ltc0_ds::R](ltc0_ds::R) reader structure"]
impl crate::Readable for LTC0_DS {}
#[doc = "`write(|w| ..)` method takes [ltc0_ds::W](ltc0_ds::W) writer structure"]
impl crate::Writable for LTC0_DS {}
#[doc = "LTC Data Size Register"]
pub mod ltc0_ds;
#[doc = "LTC ICV Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_icvs](ltc0_icvs) module"]
pub type LTC0_ICVS = crate::Reg<u32, _LTC0_ICVS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_ICVS;
#[doc = "`read()` method returns [ltc0_icvs::R](ltc0_icvs::R) reader structure"]
impl crate::Readable for LTC0_ICVS {}
#[doc = "`write(|w| ..)` method takes [ltc0_icvs::W](ltc0_icvs::W) writer structure"]
impl crate::Writable for LTC0_ICVS {}
#[doc = "LTC ICV Size Register"]
pub mod ltc0_icvs;
#[doc = "LTC Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_com](ltc0_com) module"]
pub type LTC0_COM = crate::Reg<u32, _LTC0_COM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_COM;
#[doc = "`read()` method returns [ltc0_com::R](ltc0_com::R) reader structure"]
impl crate::Readable for LTC0_COM {}
#[doc = "`write(|w| ..)` method takes [ltc0_com::W](ltc0_com::W) writer structure"]
impl crate::Writable for LTC0_COM {}
#[doc = "LTC Command Register"]
pub mod ltc0_com;
#[doc = "LTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctl](ltc0_ctl) module"]
pub type LTC0_CTL = crate::Reg<u32, _LTC0_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTL;
#[doc = "`read()` method returns [ltc0_ctl::R](ltc0_ctl::R) reader structure"]
impl crate::Readable for LTC0_CTL {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctl::W](ltc0_ctl::W) writer structure"]
impl crate::Writable for LTC0_CTL {}
#[doc = "LTC Control Register"]
pub mod ltc0_ctl;
#[doc = "LTC Clear Written Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_cw](ltc0_cw) module"]
pub type LTC0_CW = crate::Reg<u32, _LTC0_CW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CW;
#[doc = "`read()` method returns [ltc0_cw::R](ltc0_cw::R) reader structure"]
impl crate::Readable for LTC0_CW {}
#[doc = "`write(|w| ..)` method takes [ltc0_cw::W](ltc0_cw::W) writer structure"]
impl crate::Writable for LTC0_CW {}
#[doc = "LTC Clear Written Register"]
pub mod ltc0_cw;
#[doc = "LTC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_sta](ltc0_sta) module"]
pub type LTC0_STA = crate::Reg<u32, _LTC0_STA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_STA;
#[doc = "`read()` method returns [ltc0_sta::R](ltc0_sta::R) reader structure"]
impl crate::Readable for LTC0_STA {}
#[doc = "`write(|w| ..)` method takes [ltc0_sta::W](ltc0_sta::W) writer structure"]
impl crate::Writable for LTC0_STA {}
#[doc = "LTC Status Register"]
pub mod ltc0_sta;
#[doc = "LTC Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_esta](ltc0_esta) module"]
pub type LTC0_ESTA = crate::Reg<u32, _LTC0_ESTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_ESTA;
#[doc = "`read()` method returns [ltc0_esta::R](ltc0_esta::R) reader structure"]
impl crate::Readable for LTC0_ESTA {}
#[doc = "LTC Error Status Register"]
pub mod ltc0_esta;
#[doc = "LTC AAD Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_aadsz](ltc0_aadsz) module"]
pub type LTC0_AADSZ = crate::Reg<u32, _LTC0_AADSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_AADSZ;
#[doc = "`read()` method returns [ltc0_aadsz::R](ltc0_aadsz::R) reader structure"]
impl crate::Readable for LTC0_AADSZ {}
#[doc = "`write(|w| ..)` method takes [ltc0_aadsz::W](ltc0_aadsz::W) writer structure"]
impl crate::Writable for LTC0_AADSZ {}
#[doc = "LTC AAD Size Register"]
pub mod ltc0_aadsz;
#[doc = "LTC IV Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ivsz](ltc0_ivsz) module"]
pub type LTC0_IVSZ = crate::Reg<u32, _LTC0_IVSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_IVSZ;
#[doc = "`read()` method returns [ltc0_ivsz::R](ltc0_ivsz::R) reader structure"]
impl crate::Readable for LTC0_IVSZ {}
#[doc = "`write(|w| ..)` method takes [ltc0_ivsz::W](ltc0_ivsz::W) writer structure"]
impl crate::Writable for LTC0_IVSZ {}
#[doc = "LTC IV Size Register"]
pub mod ltc0_ivsz;
#[doc = "LTC DPA Mask Seed Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_dpams](ltc0_dpams) module"]
pub type LTC0_DPAMS = crate::Reg<u32, _LTC0_DPAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_DPAMS;
#[doc = "`write(|w| ..)` method takes [ltc0_dpams::W](ltc0_dpams::W) writer structure"]
impl crate::Writable for LTC0_DPAMS {}
#[doc = "LTC DPA Mask Seed Register"]
pub mod ltc0_dpams;
#[doc = "LTC PKHA A Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkasz](ltc0_pkasz) module"]
pub type LTC0_PKASZ = crate::Reg<u32, _LTC0_PKASZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKASZ;
#[doc = "`read()` method returns [ltc0_pkasz::R](ltc0_pkasz::R) reader structure"]
impl crate::Readable for LTC0_PKASZ {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkasz::W](ltc0_pkasz::W) writer structure"]
impl crate::Writable for LTC0_PKASZ {}
#[doc = "LTC PKHA A Size Register"]
pub mod ltc0_pkasz;
#[doc = "LTC PKHA B Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkbsz](ltc0_pkbsz) module"]
pub type LTC0_PKBSZ = crate::Reg<u32, _LTC0_PKBSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKBSZ;
#[doc = "`read()` method returns [ltc0_pkbsz::R](ltc0_pkbsz::R) reader structure"]
impl crate::Readable for LTC0_PKBSZ {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkbsz::W](ltc0_pkbsz::W) writer structure"]
impl crate::Writable for LTC0_PKBSZ {}
#[doc = "LTC PKHA B Size Register"]
pub mod ltc0_pkbsz;
#[doc = "LTC PKHA N Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pknsz](ltc0_pknsz) module"]
pub type LTC0_PKNSZ = crate::Reg<u32, _LTC0_PKNSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKNSZ;
#[doc = "`read()` method returns [ltc0_pknsz::R](ltc0_pknsz::R) reader structure"]
impl crate::Readable for LTC0_PKNSZ {}
#[doc = "`write(|w| ..)` method takes [ltc0_pknsz::W](ltc0_pknsz::W) writer structure"]
impl crate::Writable for LTC0_PKNSZ {}
#[doc = "LTC PKHA N Size Register"]
pub mod ltc0_pknsz;
#[doc = "LTC PKHA E Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkesz](ltc0_pkesz) module"]
pub type LTC0_PKESZ = crate::Reg<u32, _LTC0_PKESZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKESZ;
#[doc = "`read()` method returns [ltc0_pkesz::R](ltc0_pkesz::R) reader structure"]
impl crate::Readable for LTC0_PKESZ {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkesz::W](ltc0_pkesz::W) writer structure"]
impl crate::Writable for LTC0_PKESZ {}
#[doc = "LTC PKHA E Size Register"]
pub mod ltc0_pkesz;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_0](ltc0_ctx_0) module"]
pub type LTC0_CTX_0 = crate::Reg<u32, _LTC0_CTX_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_0;
#[doc = "`read()` method returns [ltc0_ctx_0::R](ltc0_ctx_0::R) reader structure"]
impl crate::Readable for LTC0_CTX_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_0::W](ltc0_ctx_0::W) writer structure"]
impl crate::Writable for LTC0_CTX_0 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_0;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_1](ltc0_ctx_1) module"]
pub type LTC0_CTX_1 = crate::Reg<u32, _LTC0_CTX_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_1;
#[doc = "`read()` method returns [ltc0_ctx_1::R](ltc0_ctx_1::R) reader structure"]
impl crate::Readable for LTC0_CTX_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_1::W](ltc0_ctx_1::W) writer structure"]
impl crate::Writable for LTC0_CTX_1 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_1;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_2](ltc0_ctx_2) module"]
pub type LTC0_CTX_2 = crate::Reg<u32, _LTC0_CTX_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_2;
#[doc = "`read()` method returns [ltc0_ctx_2::R](ltc0_ctx_2::R) reader structure"]
impl crate::Readable for LTC0_CTX_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_2::W](ltc0_ctx_2::W) writer structure"]
impl crate::Writable for LTC0_CTX_2 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_2;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_3](ltc0_ctx_3) module"]
pub type LTC0_CTX_3 = crate::Reg<u32, _LTC0_CTX_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_3;
#[doc = "`read()` method returns [ltc0_ctx_3::R](ltc0_ctx_3::R) reader structure"]
impl crate::Readable for LTC0_CTX_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_3::W](ltc0_ctx_3::W) writer structure"]
impl crate::Writable for LTC0_CTX_3 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_3;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_4](ltc0_ctx_4) module"]
pub type LTC0_CTX_4 = crate::Reg<u32, _LTC0_CTX_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_4;
#[doc = "`read()` method returns [ltc0_ctx_4::R](ltc0_ctx_4::R) reader structure"]
impl crate::Readable for LTC0_CTX_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_4::W](ltc0_ctx_4::W) writer structure"]
impl crate::Writable for LTC0_CTX_4 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_4;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_5](ltc0_ctx_5) module"]
pub type LTC0_CTX_5 = crate::Reg<u32, _LTC0_CTX_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_5;
#[doc = "`read()` method returns [ltc0_ctx_5::R](ltc0_ctx_5::R) reader structure"]
impl crate::Readable for LTC0_CTX_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_5::W](ltc0_ctx_5::W) writer structure"]
impl crate::Writable for LTC0_CTX_5 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_5;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_6](ltc0_ctx_6) module"]
pub type LTC0_CTX_6 = crate::Reg<u32, _LTC0_CTX_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_6;
#[doc = "`read()` method returns [ltc0_ctx_6::R](ltc0_ctx_6::R) reader structure"]
impl crate::Readable for LTC0_CTX_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_6::W](ltc0_ctx_6::W) writer structure"]
impl crate::Writable for LTC0_CTX_6 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_6;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_7](ltc0_ctx_7) module"]
pub type LTC0_CTX_7 = crate::Reg<u32, _LTC0_CTX_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_7;
#[doc = "`read()` method returns [ltc0_ctx_7::R](ltc0_ctx_7::R) reader structure"]
impl crate::Readable for LTC0_CTX_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_7::W](ltc0_ctx_7::W) writer structure"]
impl crate::Writable for LTC0_CTX_7 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_7;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_8](ltc0_ctx_8) module"]
pub type LTC0_CTX_8 = crate::Reg<u32, _LTC0_CTX_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_8;
#[doc = "`read()` method returns [ltc0_ctx_8::R](ltc0_ctx_8::R) reader structure"]
impl crate::Readable for LTC0_CTX_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_8::W](ltc0_ctx_8::W) writer structure"]
impl crate::Writable for LTC0_CTX_8 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_8;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_9](ltc0_ctx_9) module"]
pub type LTC0_CTX_9 = crate::Reg<u32, _LTC0_CTX_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_9;
#[doc = "`read()` method returns [ltc0_ctx_9::R](ltc0_ctx_9::R) reader structure"]
impl crate::Readable for LTC0_CTX_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_9::W](ltc0_ctx_9::W) writer structure"]
impl crate::Writable for LTC0_CTX_9 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_9;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_10](ltc0_ctx_10) module"]
pub type LTC0_CTX_10 = crate::Reg<u32, _LTC0_CTX_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_10;
#[doc = "`read()` method returns [ltc0_ctx_10::R](ltc0_ctx_10::R) reader structure"]
impl crate::Readable for LTC0_CTX_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_10::W](ltc0_ctx_10::W) writer structure"]
impl crate::Writable for LTC0_CTX_10 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_10;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_11](ltc0_ctx_11) module"]
pub type LTC0_CTX_11 = crate::Reg<u32, _LTC0_CTX_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_11;
#[doc = "`read()` method returns [ltc0_ctx_11::R](ltc0_ctx_11::R) reader structure"]
impl crate::Readable for LTC0_CTX_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_11::W](ltc0_ctx_11::W) writer structure"]
impl crate::Writable for LTC0_CTX_11 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_11;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_12](ltc0_ctx_12) module"]
pub type LTC0_CTX_12 = crate::Reg<u32, _LTC0_CTX_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_12;
#[doc = "`read()` method returns [ltc0_ctx_12::R](ltc0_ctx_12::R) reader structure"]
impl crate::Readable for LTC0_CTX_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_12::W](ltc0_ctx_12::W) writer structure"]
impl crate::Writable for LTC0_CTX_12 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_12;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_13](ltc0_ctx_13) module"]
pub type LTC0_CTX_13 = crate::Reg<u32, _LTC0_CTX_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_13;
#[doc = "`read()` method returns [ltc0_ctx_13::R](ltc0_ctx_13::R) reader structure"]
impl crate::Readable for LTC0_CTX_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_13::W](ltc0_ctx_13::W) writer structure"]
impl crate::Writable for LTC0_CTX_13 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_13;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_14](ltc0_ctx_14) module"]
pub type LTC0_CTX_14 = crate::Reg<u32, _LTC0_CTX_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_14;
#[doc = "`read()` method returns [ltc0_ctx_14::R](ltc0_ctx_14::R) reader structure"]
impl crate::Readable for LTC0_CTX_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_14::W](ltc0_ctx_14::W) writer structure"]
impl crate::Writable for LTC0_CTX_14 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_14;
#[doc = "LTC Context Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ctx_15](ltc0_ctx_15) module"]
pub type LTC0_CTX_15 = crate::Reg<u32, _LTC0_CTX_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CTX_15;
#[doc = "`read()` method returns [ltc0_ctx_15::R](ltc0_ctx_15::R) reader structure"]
impl crate::Readable for LTC0_CTX_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_ctx_15::W](ltc0_ctx_15::W) writer structure"]
impl crate::Writable for LTC0_CTX_15 {}
#[doc = "LTC Context Register"]
pub mod ltc0_ctx_15;
#[doc = "LTC Key Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_key_0](ltc0_key_0) module"]
pub type LTC0_KEY_0 = crate::Reg<u32, _LTC0_KEY_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KEY_0;
#[doc = "`read()` method returns [ltc0_key_0::R](ltc0_key_0::R) reader structure"]
impl crate::Readable for LTC0_KEY_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_key_0::W](ltc0_key_0::W) writer structure"]
impl crate::Writable for LTC0_KEY_0 {}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_0;
#[doc = "LTC Key Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_key_1](ltc0_key_1) module"]
pub type LTC0_KEY_1 = crate::Reg<u32, _LTC0_KEY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KEY_1;
#[doc = "`read()` method returns [ltc0_key_1::R](ltc0_key_1::R) reader structure"]
impl crate::Readable for LTC0_KEY_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_key_1::W](ltc0_key_1::W) writer structure"]
impl crate::Writable for LTC0_KEY_1 {}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_1;
#[doc = "LTC Key Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_key_2](ltc0_key_2) module"]
pub type LTC0_KEY_2 = crate::Reg<u32, _LTC0_KEY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KEY_2;
#[doc = "`read()` method returns [ltc0_key_2::R](ltc0_key_2::R) reader structure"]
impl crate::Readable for LTC0_KEY_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_key_2::W](ltc0_key_2::W) writer structure"]
impl crate::Writable for LTC0_KEY_2 {}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_2;
#[doc = "LTC Key Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_key_3](ltc0_key_3) module"]
pub type LTC0_KEY_3 = crate::Reg<u32, _LTC0_KEY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KEY_3;
#[doc = "`read()` method returns [ltc0_key_3::R](ltc0_key_3::R) reader structure"]
impl crate::Readable for LTC0_KEY_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_key_3::W](ltc0_key_3::W) writer structure"]
impl crate::Writable for LTC0_KEY_3 {}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_3;
#[doc = "LTC Key Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_key_4](ltc0_key_4) module"]
pub type LTC0_KEY_4 = crate::Reg<u32, _LTC0_KEY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KEY_4;
#[doc = "`read()` method returns [ltc0_key_4::R](ltc0_key_4::R) reader structure"]
impl crate::Readable for LTC0_KEY_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_key_4::W](ltc0_key_4::W) writer structure"]
impl crate::Writable for LTC0_KEY_4 {}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_4;
#[doc = "LTC Key Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_key_5](ltc0_key_5) module"]
pub type LTC0_KEY_5 = crate::Reg<u32, _LTC0_KEY_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KEY_5;
#[doc = "`read()` method returns [ltc0_key_5::R](ltc0_key_5::R) reader structure"]
impl crate::Readable for LTC0_KEY_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_key_5::W](ltc0_key_5::W) writer structure"]
impl crate::Writable for LTC0_KEY_5 {}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_5;
#[doc = "LTC Key Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_key_6](ltc0_key_6) module"]
pub type LTC0_KEY_6 = crate::Reg<u32, _LTC0_KEY_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KEY_6;
#[doc = "`read()` method returns [ltc0_key_6::R](ltc0_key_6::R) reader structure"]
impl crate::Readable for LTC0_KEY_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_key_6::W](ltc0_key_6::W) writer structure"]
impl crate::Writable for LTC0_KEY_6 {}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_6;
#[doc = "LTC Key Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_key_7](ltc0_key_7) module"]
pub type LTC0_KEY_7 = crate::Reg<u32, _LTC0_KEY_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_KEY_7;
#[doc = "`read()` method returns [ltc0_key_7::R](ltc0_key_7::R) reader structure"]
impl crate::Readable for LTC0_KEY_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_key_7::W](ltc0_key_7::W) writer structure"]
impl crate::Writable for LTC0_KEY_7 {}
#[doc = "LTC Key Registers"]
pub mod ltc0_key_7;
#[doc = "LTC Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_vid1](ltc0_vid1) module"]
pub type LTC0_VID1 = crate::Reg<u32, _LTC0_VID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_VID1;
#[doc = "`read()` method returns [ltc0_vid1::R](ltc0_vid1::R) reader structure"]
impl crate::Readable for LTC0_VID1 {}
#[doc = "LTC Version ID Register"]
pub mod ltc0_vid1;
#[doc = "LTC Version ID 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_vid2](ltc0_vid2) module"]
pub type LTC0_VID2 = crate::Reg<u32, _LTC0_VID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_VID2;
#[doc = "`read()` method returns [ltc0_vid2::R](ltc0_vid2::R) reader structure"]
impl crate::Readable for LTC0_VID2 {}
#[doc = "LTC Version ID 2 Register"]
pub mod ltc0_vid2;
#[doc = "LTC CHA Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_chavid](ltc0_chavid) module"]
pub type LTC0_CHAVID = crate::Reg<u32, _LTC0_CHAVID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_CHAVID;
#[doc = "`read()` method returns [ltc0_chavid::R](ltc0_chavid::R) reader structure"]
impl crate::Readable for LTC0_CHAVID {}
#[doc = "LTC CHA Version ID Register"]
pub mod ltc0_chavid;
#[doc = "LTC FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_fifosta](ltc0_fifosta) module"]
pub type LTC0_FIFOSTA = crate::Reg<u32, _LTC0_FIFOSTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_FIFOSTA;
#[doc = "`read()` method returns [ltc0_fifosta::R](ltc0_fifosta::R) reader structure"]
impl crate::Readable for LTC0_FIFOSTA {}
#[doc = "LTC FIFO Status Register"]
pub mod ltc0_fifosta;
#[doc = "LTC Input Data FIFO\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ififo](ltc0_ififo) module"]
pub type LTC0_IFIFO = crate::Reg<u32, _LTC0_IFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_IFIFO;
#[doc = "`write(|w| ..)` method takes [ltc0_ififo::W](ltc0_ififo::W) writer structure"]
impl crate::Writable for LTC0_IFIFO {}
#[doc = "LTC Input Data FIFO"]
pub mod ltc0_ififo;
#[doc = "LTC Output Data FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_ofifo](ltc0_ofifo) module"]
pub type LTC0_OFIFO = crate::Reg<u32, _LTC0_OFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_OFIFO;
#[doc = "`read()` method returns [ltc0_ofifo::R](ltc0_ofifo::R) reader structure"]
impl crate::Readable for LTC0_OFIFO {}
#[doc = "LTC Output Data FIFO"]
pub mod ltc0_ofifo;
#[doc = "LTC PKHA A0 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_0](ltc0_pka0_0) module"]
pub type LTC0_PKA0_0 = crate::Reg<u32, _LTC0_PKA0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_0;
#[doc = "`read()` method returns [ltc0_pka0_0::R](ltc0_pka0_0::R) reader structure"]
impl crate::Readable for LTC0_PKA0_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_0::W](ltc0_pka0_0::W) writer structure"]
impl crate::Writable for LTC0_PKA0_0 {}
#[doc = "LTC PKHA A0 0 Register"]
pub mod ltc0_pka0_0;
#[doc = "LTC PKHA A 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_0](ltc0_pka_0) module"]
pub type LTC0_PKA_0 = crate::Reg<u32, _LTC0_PKA_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_0;
#[doc = "`read()` method returns [ltc0_pka_0::R](ltc0_pka_0::R) reader structure"]
impl crate::Readable for LTC0_PKA_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_0::W](ltc0_pka_0::W) writer structure"]
impl crate::Writable for LTC0_PKA_0 {}
#[doc = "LTC PKHA A 0 Register"]
pub mod ltc0_pka_0;
#[doc = "LTC PKHA A0 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_1](ltc0_pka0_1) module"]
pub type LTC0_PKA0_1 = crate::Reg<u32, _LTC0_PKA0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_1;
#[doc = "`read()` method returns [ltc0_pka0_1::R](ltc0_pka0_1::R) reader structure"]
impl crate::Readable for LTC0_PKA0_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_1::W](ltc0_pka0_1::W) writer structure"]
impl crate::Writable for LTC0_PKA0_1 {}
#[doc = "LTC PKHA A0 1 Register"]
pub mod ltc0_pka0_1;
#[doc = "LTC PKHA A 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_1](ltc0_pka_1) module"]
pub type LTC0_PKA_1 = crate::Reg<u32, _LTC0_PKA_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_1;
#[doc = "`read()` method returns [ltc0_pka_1::R](ltc0_pka_1::R) reader structure"]
impl crate::Readable for LTC0_PKA_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_1::W](ltc0_pka_1::W) writer structure"]
impl crate::Writable for LTC0_PKA_1 {}
#[doc = "LTC PKHA A 1 Register"]
pub mod ltc0_pka_1;
#[doc = "LTC PKHA A0 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_2](ltc0_pka0_2) module"]
pub type LTC0_PKA0_2 = crate::Reg<u32, _LTC0_PKA0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_2;
#[doc = "`read()` method returns [ltc0_pka0_2::R](ltc0_pka0_2::R) reader structure"]
impl crate::Readable for LTC0_PKA0_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_2::W](ltc0_pka0_2::W) writer structure"]
impl crate::Writable for LTC0_PKA0_2 {}
#[doc = "LTC PKHA A0 2 Register"]
pub mod ltc0_pka0_2;
#[doc = "LTC PKHA A 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_2](ltc0_pka_2) module"]
pub type LTC0_PKA_2 = crate::Reg<u32, _LTC0_PKA_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_2;
#[doc = "`read()` method returns [ltc0_pka_2::R](ltc0_pka_2::R) reader structure"]
impl crate::Readable for LTC0_PKA_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_2::W](ltc0_pka_2::W) writer structure"]
impl crate::Writable for LTC0_PKA_2 {}
#[doc = "LTC PKHA A 2 Register"]
pub mod ltc0_pka_2;
#[doc = "LTC PKHA A0 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_3](ltc0_pka0_3) module"]
pub type LTC0_PKA0_3 = crate::Reg<u32, _LTC0_PKA0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_3;
#[doc = "`read()` method returns [ltc0_pka0_3::R](ltc0_pka0_3::R) reader structure"]
impl crate::Readable for LTC0_PKA0_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_3::W](ltc0_pka0_3::W) writer structure"]
impl crate::Writable for LTC0_PKA0_3 {}
#[doc = "LTC PKHA A0 3 Register"]
pub mod ltc0_pka0_3;
#[doc = "LTC PKHA A 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_3](ltc0_pka_3) module"]
pub type LTC0_PKA_3 = crate::Reg<u32, _LTC0_PKA_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_3;
#[doc = "`read()` method returns [ltc0_pka_3::R](ltc0_pka_3::R) reader structure"]
impl crate::Readable for LTC0_PKA_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_3::W](ltc0_pka_3::W) writer structure"]
impl crate::Writable for LTC0_PKA_3 {}
#[doc = "LTC PKHA A 3 Register"]
pub mod ltc0_pka_3;
#[doc = "LTC PKHA A0 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_4](ltc0_pka0_4) module"]
pub type LTC0_PKA0_4 = crate::Reg<u32, _LTC0_PKA0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_4;
#[doc = "`read()` method returns [ltc0_pka0_4::R](ltc0_pka0_4::R) reader structure"]
impl crate::Readable for LTC0_PKA0_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_4::W](ltc0_pka0_4::W) writer structure"]
impl crate::Writable for LTC0_PKA0_4 {}
#[doc = "LTC PKHA A0 4 Register"]
pub mod ltc0_pka0_4;
#[doc = "LTC PKHA A 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_4](ltc0_pka_4) module"]
pub type LTC0_PKA_4 = crate::Reg<u32, _LTC0_PKA_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_4;
#[doc = "`read()` method returns [ltc0_pka_4::R](ltc0_pka_4::R) reader structure"]
impl crate::Readable for LTC0_PKA_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_4::W](ltc0_pka_4::W) writer structure"]
impl crate::Writable for LTC0_PKA_4 {}
#[doc = "LTC PKHA A 4 Register"]
pub mod ltc0_pka_4;
#[doc = "LTC PKHA A0 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_5](ltc0_pka0_5) module"]
pub type LTC0_PKA0_5 = crate::Reg<u32, _LTC0_PKA0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_5;
#[doc = "`read()` method returns [ltc0_pka0_5::R](ltc0_pka0_5::R) reader structure"]
impl crate::Readable for LTC0_PKA0_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_5::W](ltc0_pka0_5::W) writer structure"]
impl crate::Writable for LTC0_PKA0_5 {}
#[doc = "LTC PKHA A0 5 Register"]
pub mod ltc0_pka0_5;
#[doc = "LTC PKHA A 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_5](ltc0_pka_5) module"]
pub type LTC0_PKA_5 = crate::Reg<u32, _LTC0_PKA_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_5;
#[doc = "`read()` method returns [ltc0_pka_5::R](ltc0_pka_5::R) reader structure"]
impl crate::Readable for LTC0_PKA_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_5::W](ltc0_pka_5::W) writer structure"]
impl crate::Writable for LTC0_PKA_5 {}
#[doc = "LTC PKHA A 5 Register"]
pub mod ltc0_pka_5;
#[doc = "LTC PKHA A0 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_6](ltc0_pka0_6) module"]
pub type LTC0_PKA0_6 = crate::Reg<u32, _LTC0_PKA0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_6;
#[doc = "`read()` method returns [ltc0_pka0_6::R](ltc0_pka0_6::R) reader structure"]
impl crate::Readable for LTC0_PKA0_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_6::W](ltc0_pka0_6::W) writer structure"]
impl crate::Writable for LTC0_PKA0_6 {}
#[doc = "LTC PKHA A0 6 Register"]
pub mod ltc0_pka0_6;
#[doc = "LTC PKHA A 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_6](ltc0_pka_6) module"]
pub type LTC0_PKA_6 = crate::Reg<u32, _LTC0_PKA_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_6;
#[doc = "`read()` method returns [ltc0_pka_6::R](ltc0_pka_6::R) reader structure"]
impl crate::Readable for LTC0_PKA_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_6::W](ltc0_pka_6::W) writer structure"]
impl crate::Writable for LTC0_PKA_6 {}
#[doc = "LTC PKHA A 6 Register"]
pub mod ltc0_pka_6;
#[doc = "LTC PKHA A0 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_7](ltc0_pka0_7) module"]
pub type LTC0_PKA0_7 = crate::Reg<u32, _LTC0_PKA0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_7;
#[doc = "`read()` method returns [ltc0_pka0_7::R](ltc0_pka0_7::R) reader structure"]
impl crate::Readable for LTC0_PKA0_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_7::W](ltc0_pka0_7::W) writer structure"]
impl crate::Writable for LTC0_PKA0_7 {}
#[doc = "LTC PKHA A0 7 Register"]
pub mod ltc0_pka0_7;
#[doc = "LTC PKHA A 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_7](ltc0_pka_7) module"]
pub type LTC0_PKA_7 = crate::Reg<u32, _LTC0_PKA_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_7;
#[doc = "`read()` method returns [ltc0_pka_7::R](ltc0_pka_7::R) reader structure"]
impl crate::Readable for LTC0_PKA_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_7::W](ltc0_pka_7::W) writer structure"]
impl crate::Writable for LTC0_PKA_7 {}
#[doc = "LTC PKHA A 7 Register"]
pub mod ltc0_pka_7;
#[doc = "LTC PKHA A0 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_8](ltc0_pka0_8) module"]
pub type LTC0_PKA0_8 = crate::Reg<u32, _LTC0_PKA0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_8;
#[doc = "`read()` method returns [ltc0_pka0_8::R](ltc0_pka0_8::R) reader structure"]
impl crate::Readable for LTC0_PKA0_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_8::W](ltc0_pka0_8::W) writer structure"]
impl crate::Writable for LTC0_PKA0_8 {}
#[doc = "LTC PKHA A0 8 Register"]
pub mod ltc0_pka0_8;
#[doc = "LTC PKHA A 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_8](ltc0_pka_8) module"]
pub type LTC0_PKA_8 = crate::Reg<u32, _LTC0_PKA_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_8;
#[doc = "`read()` method returns [ltc0_pka_8::R](ltc0_pka_8::R) reader structure"]
impl crate::Readable for LTC0_PKA_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_8::W](ltc0_pka_8::W) writer structure"]
impl crate::Writable for LTC0_PKA_8 {}
#[doc = "LTC PKHA A 8 Register"]
pub mod ltc0_pka_8;
#[doc = "LTC PKHA A0 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_9](ltc0_pka0_9) module"]
pub type LTC0_PKA0_9 = crate::Reg<u32, _LTC0_PKA0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_9;
#[doc = "`read()` method returns [ltc0_pka0_9::R](ltc0_pka0_9::R) reader structure"]
impl crate::Readable for LTC0_PKA0_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_9::W](ltc0_pka0_9::W) writer structure"]
impl crate::Writable for LTC0_PKA0_9 {}
#[doc = "LTC PKHA A0 9 Register"]
pub mod ltc0_pka0_9;
#[doc = "LTC PKHA A 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_9](ltc0_pka_9) module"]
pub type LTC0_PKA_9 = crate::Reg<u32, _LTC0_PKA_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_9;
#[doc = "`read()` method returns [ltc0_pka_9::R](ltc0_pka_9::R) reader structure"]
impl crate::Readable for LTC0_PKA_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_9::W](ltc0_pka_9::W) writer structure"]
impl crate::Writable for LTC0_PKA_9 {}
#[doc = "LTC PKHA A 9 Register"]
pub mod ltc0_pka_9;
#[doc = "LTC PKHA A0 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_10](ltc0_pka0_10) module"]
pub type LTC0_PKA0_10 = crate::Reg<u32, _LTC0_PKA0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_10;
#[doc = "`read()` method returns [ltc0_pka0_10::R](ltc0_pka0_10::R) reader structure"]
impl crate::Readable for LTC0_PKA0_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_10::W](ltc0_pka0_10::W) writer structure"]
impl crate::Writable for LTC0_PKA0_10 {}
#[doc = "LTC PKHA A0 10 Register"]
pub mod ltc0_pka0_10;
#[doc = "LTC PKHA A 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_10](ltc0_pka_10) module"]
pub type LTC0_PKA_10 = crate::Reg<u32, _LTC0_PKA_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_10;
#[doc = "`read()` method returns [ltc0_pka_10::R](ltc0_pka_10::R) reader structure"]
impl crate::Readable for LTC0_PKA_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_10::W](ltc0_pka_10::W) writer structure"]
impl crate::Writable for LTC0_PKA_10 {}
#[doc = "LTC PKHA A 10 Register"]
pub mod ltc0_pka_10;
#[doc = "LTC PKHA A0 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_11](ltc0_pka0_11) module"]
pub type LTC0_PKA0_11 = crate::Reg<u32, _LTC0_PKA0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_11;
#[doc = "`read()` method returns [ltc0_pka0_11::R](ltc0_pka0_11::R) reader structure"]
impl crate::Readable for LTC0_PKA0_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_11::W](ltc0_pka0_11::W) writer structure"]
impl crate::Writable for LTC0_PKA0_11 {}
#[doc = "LTC PKHA A0 11 Register"]
pub mod ltc0_pka0_11;
#[doc = "LTC PKHA A 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_11](ltc0_pka_11) module"]
pub type LTC0_PKA_11 = crate::Reg<u32, _LTC0_PKA_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_11;
#[doc = "`read()` method returns [ltc0_pka_11::R](ltc0_pka_11::R) reader structure"]
impl crate::Readable for LTC0_PKA_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_11::W](ltc0_pka_11::W) writer structure"]
impl crate::Writable for LTC0_PKA_11 {}
#[doc = "LTC PKHA A 11 Register"]
pub mod ltc0_pka_11;
#[doc = "LTC PKHA A0 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_12](ltc0_pka0_12) module"]
pub type LTC0_PKA0_12 = crate::Reg<u32, _LTC0_PKA0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_12;
#[doc = "`read()` method returns [ltc0_pka0_12::R](ltc0_pka0_12::R) reader structure"]
impl crate::Readable for LTC0_PKA0_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_12::W](ltc0_pka0_12::W) writer structure"]
impl crate::Writable for LTC0_PKA0_12 {}
#[doc = "LTC PKHA A0 12 Register"]
pub mod ltc0_pka0_12;
#[doc = "LTC PKHA A 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_12](ltc0_pka_12) module"]
pub type LTC0_PKA_12 = crate::Reg<u32, _LTC0_PKA_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_12;
#[doc = "`read()` method returns [ltc0_pka_12::R](ltc0_pka_12::R) reader structure"]
impl crate::Readable for LTC0_PKA_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_12::W](ltc0_pka_12::W) writer structure"]
impl crate::Writable for LTC0_PKA_12 {}
#[doc = "LTC PKHA A 12 Register"]
pub mod ltc0_pka_12;
#[doc = "LTC PKHA A0 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_13](ltc0_pka0_13) module"]
pub type LTC0_PKA0_13 = crate::Reg<u32, _LTC0_PKA0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_13;
#[doc = "`read()` method returns [ltc0_pka0_13::R](ltc0_pka0_13::R) reader structure"]
impl crate::Readable for LTC0_PKA0_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_13::W](ltc0_pka0_13::W) writer structure"]
impl crate::Writable for LTC0_PKA0_13 {}
#[doc = "LTC PKHA A0 13 Register"]
pub mod ltc0_pka0_13;
#[doc = "LTC PKHA A 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_13](ltc0_pka_13) module"]
pub type LTC0_PKA_13 = crate::Reg<u32, _LTC0_PKA_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_13;
#[doc = "`read()` method returns [ltc0_pka_13::R](ltc0_pka_13::R) reader structure"]
impl crate::Readable for LTC0_PKA_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_13::W](ltc0_pka_13::W) writer structure"]
impl crate::Writable for LTC0_PKA_13 {}
#[doc = "LTC PKHA A 13 Register"]
pub mod ltc0_pka_13;
#[doc = "LTC PKHA A0 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_14](ltc0_pka0_14) module"]
pub type LTC0_PKA0_14 = crate::Reg<u32, _LTC0_PKA0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_14;
#[doc = "`read()` method returns [ltc0_pka0_14::R](ltc0_pka0_14::R) reader structure"]
impl crate::Readable for LTC0_PKA0_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_14::W](ltc0_pka0_14::W) writer structure"]
impl crate::Writable for LTC0_PKA0_14 {}
#[doc = "LTC PKHA A0 14 Register"]
pub mod ltc0_pka0_14;
#[doc = "LTC PKHA A 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_14](ltc0_pka_14) module"]
pub type LTC0_PKA_14 = crate::Reg<u32, _LTC0_PKA_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_14;
#[doc = "`read()` method returns [ltc0_pka_14::R](ltc0_pka_14::R) reader structure"]
impl crate::Readable for LTC0_PKA_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_14::W](ltc0_pka_14::W) writer structure"]
impl crate::Writable for LTC0_PKA_14 {}
#[doc = "LTC PKHA A 14 Register"]
pub mod ltc0_pka_14;
#[doc = "LTC PKHA A0 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka0_15](ltc0_pka0_15) module"]
pub type LTC0_PKA0_15 = crate::Reg<u32, _LTC0_PKA0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA0_15;
#[doc = "`read()` method returns [ltc0_pka0_15::R](ltc0_pka0_15::R) reader structure"]
impl crate::Readable for LTC0_PKA0_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka0_15::W](ltc0_pka0_15::W) writer structure"]
impl crate::Writable for LTC0_PKA0_15 {}
#[doc = "LTC PKHA A0 15 Register"]
pub mod ltc0_pka0_15;
#[doc = "LTC PKHA A 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_15](ltc0_pka_15) module"]
pub type LTC0_PKA_15 = crate::Reg<u32, _LTC0_PKA_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_15;
#[doc = "`read()` method returns [ltc0_pka_15::R](ltc0_pka_15::R) reader structure"]
impl crate::Readable for LTC0_PKA_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_15::W](ltc0_pka_15::W) writer structure"]
impl crate::Writable for LTC0_PKA_15 {}
#[doc = "LTC PKHA A 15 Register"]
pub mod ltc0_pka_15;
#[doc = "LTC PKHA A1 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_0](ltc0_pka1_0) module"]
pub type LTC0_PKA1_0 = crate::Reg<u32, _LTC0_PKA1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_0;
#[doc = "`read()` method returns [ltc0_pka1_0::R](ltc0_pka1_0::R) reader structure"]
impl crate::Readable for LTC0_PKA1_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_0::W](ltc0_pka1_0::W) writer structure"]
impl crate::Writable for LTC0_PKA1_0 {}
#[doc = "LTC PKHA A1 0 Register"]
pub mod ltc0_pka1_0;
#[doc = "LTC PKHA A 16 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_16](ltc0_pka_16) module"]
pub type LTC0_PKA_16 = crate::Reg<u32, _LTC0_PKA_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_16;
#[doc = "`read()` method returns [ltc0_pka_16::R](ltc0_pka_16::R) reader structure"]
impl crate::Readable for LTC0_PKA_16 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_16::W](ltc0_pka_16::W) writer structure"]
impl crate::Writable for LTC0_PKA_16 {}
#[doc = "LTC PKHA A 16 Register"]
pub mod ltc0_pka_16;
#[doc = "LTC PKHA A1 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_1](ltc0_pka1_1) module"]
pub type LTC0_PKA1_1 = crate::Reg<u32, _LTC0_PKA1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_1;
#[doc = "`read()` method returns [ltc0_pka1_1::R](ltc0_pka1_1::R) reader structure"]
impl crate::Readable for LTC0_PKA1_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_1::W](ltc0_pka1_1::W) writer structure"]
impl crate::Writable for LTC0_PKA1_1 {}
#[doc = "LTC PKHA A1 1 Register"]
pub mod ltc0_pka1_1;
#[doc = "LTC PKHA A 17 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_17](ltc0_pka_17) module"]
pub type LTC0_PKA_17 = crate::Reg<u32, _LTC0_PKA_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_17;
#[doc = "`read()` method returns [ltc0_pka_17::R](ltc0_pka_17::R) reader structure"]
impl crate::Readable for LTC0_PKA_17 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_17::W](ltc0_pka_17::W) writer structure"]
impl crate::Writable for LTC0_PKA_17 {}
#[doc = "LTC PKHA A 17 Register"]
pub mod ltc0_pka_17;
#[doc = "LTC PKHA A1 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_2](ltc0_pka1_2) module"]
pub type LTC0_PKA1_2 = crate::Reg<u32, _LTC0_PKA1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_2;
#[doc = "`read()` method returns [ltc0_pka1_2::R](ltc0_pka1_2::R) reader structure"]
impl crate::Readable for LTC0_PKA1_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_2::W](ltc0_pka1_2::W) writer structure"]
impl crate::Writable for LTC0_PKA1_2 {}
#[doc = "LTC PKHA A1 2 Register"]
pub mod ltc0_pka1_2;
#[doc = "LTC PKHA A 18 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_18](ltc0_pka_18) module"]
pub type LTC0_PKA_18 = crate::Reg<u32, _LTC0_PKA_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_18;
#[doc = "`read()` method returns [ltc0_pka_18::R](ltc0_pka_18::R) reader structure"]
impl crate::Readable for LTC0_PKA_18 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_18::W](ltc0_pka_18::W) writer structure"]
impl crate::Writable for LTC0_PKA_18 {}
#[doc = "LTC PKHA A 18 Register"]
pub mod ltc0_pka_18;
#[doc = "LTC PKHA A1 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_3](ltc0_pka1_3) module"]
pub type LTC0_PKA1_3 = crate::Reg<u32, _LTC0_PKA1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_3;
#[doc = "`read()` method returns [ltc0_pka1_3::R](ltc0_pka1_3::R) reader structure"]
impl crate::Readable for LTC0_PKA1_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_3::W](ltc0_pka1_3::W) writer structure"]
impl crate::Writable for LTC0_PKA1_3 {}
#[doc = "LTC PKHA A1 3 Register"]
pub mod ltc0_pka1_3;
#[doc = "LTC PKHA A 19 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_19](ltc0_pka_19) module"]
pub type LTC0_PKA_19 = crate::Reg<u32, _LTC0_PKA_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_19;
#[doc = "`read()` method returns [ltc0_pka_19::R](ltc0_pka_19::R) reader structure"]
impl crate::Readable for LTC0_PKA_19 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_19::W](ltc0_pka_19::W) writer structure"]
impl crate::Writable for LTC0_PKA_19 {}
#[doc = "LTC PKHA A 19 Register"]
pub mod ltc0_pka_19;
#[doc = "LTC PKHA A1 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_4](ltc0_pka1_4) module"]
pub type LTC0_PKA1_4 = crate::Reg<u32, _LTC0_PKA1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_4;
#[doc = "`read()` method returns [ltc0_pka1_4::R](ltc0_pka1_4::R) reader structure"]
impl crate::Readable for LTC0_PKA1_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_4::W](ltc0_pka1_4::W) writer structure"]
impl crate::Writable for LTC0_PKA1_4 {}
#[doc = "LTC PKHA A1 4 Register"]
pub mod ltc0_pka1_4;
#[doc = "LTC PKHA A 20 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_20](ltc0_pka_20) module"]
pub type LTC0_PKA_20 = crate::Reg<u32, _LTC0_PKA_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_20;
#[doc = "`read()` method returns [ltc0_pka_20::R](ltc0_pka_20::R) reader structure"]
impl crate::Readable for LTC0_PKA_20 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_20::W](ltc0_pka_20::W) writer structure"]
impl crate::Writable for LTC0_PKA_20 {}
#[doc = "LTC PKHA A 20 Register"]
pub mod ltc0_pka_20;
#[doc = "LTC PKHA A1 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_5](ltc0_pka1_5) module"]
pub type LTC0_PKA1_5 = crate::Reg<u32, _LTC0_PKA1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_5;
#[doc = "`read()` method returns [ltc0_pka1_5::R](ltc0_pka1_5::R) reader structure"]
impl crate::Readable for LTC0_PKA1_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_5::W](ltc0_pka1_5::W) writer structure"]
impl crate::Writable for LTC0_PKA1_5 {}
#[doc = "LTC PKHA A1 5 Register"]
pub mod ltc0_pka1_5;
#[doc = "LTC PKHA A 21 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_21](ltc0_pka_21) module"]
pub type LTC0_PKA_21 = crate::Reg<u32, _LTC0_PKA_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_21;
#[doc = "`read()` method returns [ltc0_pka_21::R](ltc0_pka_21::R) reader structure"]
impl crate::Readable for LTC0_PKA_21 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_21::W](ltc0_pka_21::W) writer structure"]
impl crate::Writable for LTC0_PKA_21 {}
#[doc = "LTC PKHA A 21 Register"]
pub mod ltc0_pka_21;
#[doc = "LTC PKHA A1 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_6](ltc0_pka1_6) module"]
pub type LTC0_PKA1_6 = crate::Reg<u32, _LTC0_PKA1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_6;
#[doc = "`read()` method returns [ltc0_pka1_6::R](ltc0_pka1_6::R) reader structure"]
impl crate::Readable for LTC0_PKA1_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_6::W](ltc0_pka1_6::W) writer structure"]
impl crate::Writable for LTC0_PKA1_6 {}
#[doc = "LTC PKHA A1 6 Register"]
pub mod ltc0_pka1_6;
#[doc = "LTC PKHA A 22 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_22](ltc0_pka_22) module"]
pub type LTC0_PKA_22 = crate::Reg<u32, _LTC0_PKA_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_22;
#[doc = "`read()` method returns [ltc0_pka_22::R](ltc0_pka_22::R) reader structure"]
impl crate::Readable for LTC0_PKA_22 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_22::W](ltc0_pka_22::W) writer structure"]
impl crate::Writable for LTC0_PKA_22 {}
#[doc = "LTC PKHA A 22 Register"]
pub mod ltc0_pka_22;
#[doc = "LTC PKHA A1 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_7](ltc0_pka1_7) module"]
pub type LTC0_PKA1_7 = crate::Reg<u32, _LTC0_PKA1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_7;
#[doc = "`read()` method returns [ltc0_pka1_7::R](ltc0_pka1_7::R) reader structure"]
impl crate::Readable for LTC0_PKA1_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_7::W](ltc0_pka1_7::W) writer structure"]
impl crate::Writable for LTC0_PKA1_7 {}
#[doc = "LTC PKHA A1 7 Register"]
pub mod ltc0_pka1_7;
#[doc = "LTC PKHA A 23 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_23](ltc0_pka_23) module"]
pub type LTC0_PKA_23 = crate::Reg<u32, _LTC0_PKA_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_23;
#[doc = "`read()` method returns [ltc0_pka_23::R](ltc0_pka_23::R) reader structure"]
impl crate::Readable for LTC0_PKA_23 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_23::W](ltc0_pka_23::W) writer structure"]
impl crate::Writable for LTC0_PKA_23 {}
#[doc = "LTC PKHA A 23 Register"]
pub mod ltc0_pka_23;
#[doc = "LTC PKHA A1 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_8](ltc0_pka1_8) module"]
pub type LTC0_PKA1_8 = crate::Reg<u32, _LTC0_PKA1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_8;
#[doc = "`read()` method returns [ltc0_pka1_8::R](ltc0_pka1_8::R) reader structure"]
impl crate::Readable for LTC0_PKA1_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_8::W](ltc0_pka1_8::W) writer structure"]
impl crate::Writable for LTC0_PKA1_8 {}
#[doc = "LTC PKHA A1 8 Register"]
pub mod ltc0_pka1_8;
#[doc = "LTC PKHA A 24 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_24](ltc0_pka_24) module"]
pub type LTC0_PKA_24 = crate::Reg<u32, _LTC0_PKA_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_24;
#[doc = "`read()` method returns [ltc0_pka_24::R](ltc0_pka_24::R) reader structure"]
impl crate::Readable for LTC0_PKA_24 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_24::W](ltc0_pka_24::W) writer structure"]
impl crate::Writable for LTC0_PKA_24 {}
#[doc = "LTC PKHA A 24 Register"]
pub mod ltc0_pka_24;
#[doc = "LTC PKHA A1 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_9](ltc0_pka1_9) module"]
pub type LTC0_PKA1_9 = crate::Reg<u32, _LTC0_PKA1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_9;
#[doc = "`read()` method returns [ltc0_pka1_9::R](ltc0_pka1_9::R) reader structure"]
impl crate::Readable for LTC0_PKA1_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_9::W](ltc0_pka1_9::W) writer structure"]
impl crate::Writable for LTC0_PKA1_9 {}
#[doc = "LTC PKHA A1 9 Register"]
pub mod ltc0_pka1_9;
#[doc = "LTC PKHA A 25 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_25](ltc0_pka_25) module"]
pub type LTC0_PKA_25 = crate::Reg<u32, _LTC0_PKA_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_25;
#[doc = "`read()` method returns [ltc0_pka_25::R](ltc0_pka_25::R) reader structure"]
impl crate::Readable for LTC0_PKA_25 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_25::W](ltc0_pka_25::W) writer structure"]
impl crate::Writable for LTC0_PKA_25 {}
#[doc = "LTC PKHA A 25 Register"]
pub mod ltc0_pka_25;
#[doc = "LTC PKHA A1 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_10](ltc0_pka1_10) module"]
pub type LTC0_PKA1_10 = crate::Reg<u32, _LTC0_PKA1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_10;
#[doc = "`read()` method returns [ltc0_pka1_10::R](ltc0_pka1_10::R) reader structure"]
impl crate::Readable for LTC0_PKA1_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_10::W](ltc0_pka1_10::W) writer structure"]
impl crate::Writable for LTC0_PKA1_10 {}
#[doc = "LTC PKHA A1 10 Register"]
pub mod ltc0_pka1_10;
#[doc = "LTC PKHA A 26 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_26](ltc0_pka_26) module"]
pub type LTC0_PKA_26 = crate::Reg<u32, _LTC0_PKA_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_26;
#[doc = "`read()` method returns [ltc0_pka_26::R](ltc0_pka_26::R) reader structure"]
impl crate::Readable for LTC0_PKA_26 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_26::W](ltc0_pka_26::W) writer structure"]
impl crate::Writable for LTC0_PKA_26 {}
#[doc = "LTC PKHA A 26 Register"]
pub mod ltc0_pka_26;
#[doc = "LTC PKHA A1 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_11](ltc0_pka1_11) module"]
pub type LTC0_PKA1_11 = crate::Reg<u32, _LTC0_PKA1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_11;
#[doc = "`read()` method returns [ltc0_pka1_11::R](ltc0_pka1_11::R) reader structure"]
impl crate::Readable for LTC0_PKA1_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_11::W](ltc0_pka1_11::W) writer structure"]
impl crate::Writable for LTC0_PKA1_11 {}
#[doc = "LTC PKHA A1 11 Register"]
pub mod ltc0_pka1_11;
#[doc = "LTC PKHA A 27 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_27](ltc0_pka_27) module"]
pub type LTC0_PKA_27 = crate::Reg<u32, _LTC0_PKA_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_27;
#[doc = "`read()` method returns [ltc0_pka_27::R](ltc0_pka_27::R) reader structure"]
impl crate::Readable for LTC0_PKA_27 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_27::W](ltc0_pka_27::W) writer structure"]
impl crate::Writable for LTC0_PKA_27 {}
#[doc = "LTC PKHA A 27 Register"]
pub mod ltc0_pka_27;
#[doc = "LTC PKHA A1 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_12](ltc0_pka1_12) module"]
pub type LTC0_PKA1_12 = crate::Reg<u32, _LTC0_PKA1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_12;
#[doc = "`read()` method returns [ltc0_pka1_12::R](ltc0_pka1_12::R) reader structure"]
impl crate::Readable for LTC0_PKA1_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_12::W](ltc0_pka1_12::W) writer structure"]
impl crate::Writable for LTC0_PKA1_12 {}
#[doc = "LTC PKHA A1 12 Register"]
pub mod ltc0_pka1_12;
#[doc = "LTC PKHA A 28 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_28](ltc0_pka_28) module"]
pub type LTC0_PKA_28 = crate::Reg<u32, _LTC0_PKA_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_28;
#[doc = "`read()` method returns [ltc0_pka_28::R](ltc0_pka_28::R) reader structure"]
impl crate::Readable for LTC0_PKA_28 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_28::W](ltc0_pka_28::W) writer structure"]
impl crate::Writable for LTC0_PKA_28 {}
#[doc = "LTC PKHA A 28 Register"]
pub mod ltc0_pka_28;
#[doc = "LTC PKHA A1 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_13](ltc0_pka1_13) module"]
pub type LTC0_PKA1_13 = crate::Reg<u32, _LTC0_PKA1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_13;
#[doc = "`read()` method returns [ltc0_pka1_13::R](ltc0_pka1_13::R) reader structure"]
impl crate::Readable for LTC0_PKA1_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_13::W](ltc0_pka1_13::W) writer structure"]
impl crate::Writable for LTC0_PKA1_13 {}
#[doc = "LTC PKHA A1 13 Register"]
pub mod ltc0_pka1_13;
#[doc = "LTC PKHA A 29 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_29](ltc0_pka_29) module"]
pub type LTC0_PKA_29 = crate::Reg<u32, _LTC0_PKA_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_29;
#[doc = "`read()` method returns [ltc0_pka_29::R](ltc0_pka_29::R) reader structure"]
impl crate::Readable for LTC0_PKA_29 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_29::W](ltc0_pka_29::W) writer structure"]
impl crate::Writable for LTC0_PKA_29 {}
#[doc = "LTC PKHA A 29 Register"]
pub mod ltc0_pka_29;
#[doc = "LTC PKHA A1 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_14](ltc0_pka1_14) module"]
pub type LTC0_PKA1_14 = crate::Reg<u32, _LTC0_PKA1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_14;
#[doc = "`read()` method returns [ltc0_pka1_14::R](ltc0_pka1_14::R) reader structure"]
impl crate::Readable for LTC0_PKA1_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_14::W](ltc0_pka1_14::W) writer structure"]
impl crate::Writable for LTC0_PKA1_14 {}
#[doc = "LTC PKHA A1 14 Register"]
pub mod ltc0_pka1_14;
#[doc = "LTC PKHA A 30 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_30](ltc0_pka_30) module"]
pub type LTC0_PKA_30 = crate::Reg<u32, _LTC0_PKA_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_30;
#[doc = "`read()` method returns [ltc0_pka_30::R](ltc0_pka_30::R) reader structure"]
impl crate::Readable for LTC0_PKA_30 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_30::W](ltc0_pka_30::W) writer structure"]
impl crate::Writable for LTC0_PKA_30 {}
#[doc = "LTC PKHA A 30 Register"]
pub mod ltc0_pka_30;
#[doc = "LTC PKHA A1 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka1_15](ltc0_pka1_15) module"]
pub type LTC0_PKA1_15 = crate::Reg<u32, _LTC0_PKA1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA1_15;
#[doc = "`read()` method returns [ltc0_pka1_15::R](ltc0_pka1_15::R) reader structure"]
impl crate::Readable for LTC0_PKA1_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka1_15::W](ltc0_pka1_15::W) writer structure"]
impl crate::Writable for LTC0_PKA1_15 {}
#[doc = "LTC PKHA A1 15 Register"]
pub mod ltc0_pka1_15;
#[doc = "LTC PKHA A 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_31](ltc0_pka_31) module"]
pub type LTC0_PKA_31 = crate::Reg<u32, _LTC0_PKA_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_31;
#[doc = "`read()` method returns [ltc0_pka_31::R](ltc0_pka_31::R) reader structure"]
impl crate::Readable for LTC0_PKA_31 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_31::W](ltc0_pka_31::W) writer structure"]
impl crate::Writable for LTC0_PKA_31 {}
#[doc = "LTC PKHA A 31 Register"]
pub mod ltc0_pka_31;
#[doc = "LTC PKHA A2 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_0](ltc0_pka2_0) module"]
pub type LTC0_PKA2_0 = crate::Reg<u32, _LTC0_PKA2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_0;
#[doc = "`read()` method returns [ltc0_pka2_0::R](ltc0_pka2_0::R) reader structure"]
impl crate::Readable for LTC0_PKA2_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_0::W](ltc0_pka2_0::W) writer structure"]
impl crate::Writable for LTC0_PKA2_0 {}
#[doc = "LTC PKHA A2 0 Register"]
pub mod ltc0_pka2_0;
#[doc = "LTC PKHA A 32 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_32](ltc0_pka_32) module"]
pub type LTC0_PKA_32 = crate::Reg<u32, _LTC0_PKA_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_32;
#[doc = "`read()` method returns [ltc0_pka_32::R](ltc0_pka_32::R) reader structure"]
impl crate::Readable for LTC0_PKA_32 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_32::W](ltc0_pka_32::W) writer structure"]
impl crate::Writable for LTC0_PKA_32 {}
#[doc = "LTC PKHA A 32 Register"]
pub mod ltc0_pka_32;
#[doc = "LTC PKHA A2 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_1](ltc0_pka2_1) module"]
pub type LTC0_PKA2_1 = crate::Reg<u32, _LTC0_PKA2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_1;
#[doc = "`read()` method returns [ltc0_pka2_1::R](ltc0_pka2_1::R) reader structure"]
impl crate::Readable for LTC0_PKA2_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_1::W](ltc0_pka2_1::W) writer structure"]
impl crate::Writable for LTC0_PKA2_1 {}
#[doc = "LTC PKHA A2 1 Register"]
pub mod ltc0_pka2_1;
#[doc = "LTC PKHA A 33 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_33](ltc0_pka_33) module"]
pub type LTC0_PKA_33 = crate::Reg<u32, _LTC0_PKA_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_33;
#[doc = "`read()` method returns [ltc0_pka_33::R](ltc0_pka_33::R) reader structure"]
impl crate::Readable for LTC0_PKA_33 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_33::W](ltc0_pka_33::W) writer structure"]
impl crate::Writable for LTC0_PKA_33 {}
#[doc = "LTC PKHA A 33 Register"]
pub mod ltc0_pka_33;
#[doc = "LTC PKHA A2 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_2](ltc0_pka2_2) module"]
pub type LTC0_PKA2_2 = crate::Reg<u32, _LTC0_PKA2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_2;
#[doc = "`read()` method returns [ltc0_pka2_2::R](ltc0_pka2_2::R) reader structure"]
impl crate::Readable for LTC0_PKA2_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_2::W](ltc0_pka2_2::W) writer structure"]
impl crate::Writable for LTC0_PKA2_2 {}
#[doc = "LTC PKHA A2 2 Register"]
pub mod ltc0_pka2_2;
#[doc = "LTC PKHA A 34 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_34](ltc0_pka_34) module"]
pub type LTC0_PKA_34 = crate::Reg<u32, _LTC0_PKA_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_34;
#[doc = "`read()` method returns [ltc0_pka_34::R](ltc0_pka_34::R) reader structure"]
impl crate::Readable for LTC0_PKA_34 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_34::W](ltc0_pka_34::W) writer structure"]
impl crate::Writable for LTC0_PKA_34 {}
#[doc = "LTC PKHA A 34 Register"]
pub mod ltc0_pka_34;
#[doc = "LTC PKHA A2 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_3](ltc0_pka2_3) module"]
pub type LTC0_PKA2_3 = crate::Reg<u32, _LTC0_PKA2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_3;
#[doc = "`read()` method returns [ltc0_pka2_3::R](ltc0_pka2_3::R) reader structure"]
impl crate::Readable for LTC0_PKA2_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_3::W](ltc0_pka2_3::W) writer structure"]
impl crate::Writable for LTC0_PKA2_3 {}
#[doc = "LTC PKHA A2 3 Register"]
pub mod ltc0_pka2_3;
#[doc = "LTC PKHA A 35 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_35](ltc0_pka_35) module"]
pub type LTC0_PKA_35 = crate::Reg<u32, _LTC0_PKA_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_35;
#[doc = "`read()` method returns [ltc0_pka_35::R](ltc0_pka_35::R) reader structure"]
impl crate::Readable for LTC0_PKA_35 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_35::W](ltc0_pka_35::W) writer structure"]
impl crate::Writable for LTC0_PKA_35 {}
#[doc = "LTC PKHA A 35 Register"]
pub mod ltc0_pka_35;
#[doc = "LTC PKHA A2 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_4](ltc0_pka2_4) module"]
pub type LTC0_PKA2_4 = crate::Reg<u32, _LTC0_PKA2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_4;
#[doc = "`read()` method returns [ltc0_pka2_4::R](ltc0_pka2_4::R) reader structure"]
impl crate::Readable for LTC0_PKA2_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_4::W](ltc0_pka2_4::W) writer structure"]
impl crate::Writable for LTC0_PKA2_4 {}
#[doc = "LTC PKHA A2 4 Register"]
pub mod ltc0_pka2_4;
#[doc = "LTC PKHA A 36 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_36](ltc0_pka_36) module"]
pub type LTC0_PKA_36 = crate::Reg<u32, _LTC0_PKA_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_36;
#[doc = "`read()` method returns [ltc0_pka_36::R](ltc0_pka_36::R) reader structure"]
impl crate::Readable for LTC0_PKA_36 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_36::W](ltc0_pka_36::W) writer structure"]
impl crate::Writable for LTC0_PKA_36 {}
#[doc = "LTC PKHA A 36 Register"]
pub mod ltc0_pka_36;
#[doc = "LTC PKHA A2 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_5](ltc0_pka2_5) module"]
pub type LTC0_PKA2_5 = crate::Reg<u32, _LTC0_PKA2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_5;
#[doc = "`read()` method returns [ltc0_pka2_5::R](ltc0_pka2_5::R) reader structure"]
impl crate::Readable for LTC0_PKA2_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_5::W](ltc0_pka2_5::W) writer structure"]
impl crate::Writable for LTC0_PKA2_5 {}
#[doc = "LTC PKHA A2 5 Register"]
pub mod ltc0_pka2_5;
#[doc = "LTC PKHA A 37 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_37](ltc0_pka_37) module"]
pub type LTC0_PKA_37 = crate::Reg<u32, _LTC0_PKA_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_37;
#[doc = "`read()` method returns [ltc0_pka_37::R](ltc0_pka_37::R) reader structure"]
impl crate::Readable for LTC0_PKA_37 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_37::W](ltc0_pka_37::W) writer structure"]
impl crate::Writable for LTC0_PKA_37 {}
#[doc = "LTC PKHA A 37 Register"]
pub mod ltc0_pka_37;
#[doc = "LTC PKHA A2 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_6](ltc0_pka2_6) module"]
pub type LTC0_PKA2_6 = crate::Reg<u32, _LTC0_PKA2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_6;
#[doc = "`read()` method returns [ltc0_pka2_6::R](ltc0_pka2_6::R) reader structure"]
impl crate::Readable for LTC0_PKA2_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_6::W](ltc0_pka2_6::W) writer structure"]
impl crate::Writable for LTC0_PKA2_6 {}
#[doc = "LTC PKHA A2 6 Register"]
pub mod ltc0_pka2_6;
#[doc = "LTC PKHA A 38 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_38](ltc0_pka_38) module"]
pub type LTC0_PKA_38 = crate::Reg<u32, _LTC0_PKA_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_38;
#[doc = "`read()` method returns [ltc0_pka_38::R](ltc0_pka_38::R) reader structure"]
impl crate::Readable for LTC0_PKA_38 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_38::W](ltc0_pka_38::W) writer structure"]
impl crate::Writable for LTC0_PKA_38 {}
#[doc = "LTC PKHA A 38 Register"]
pub mod ltc0_pka_38;
#[doc = "LTC PKHA A2 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_7](ltc0_pka2_7) module"]
pub type LTC0_PKA2_7 = crate::Reg<u32, _LTC0_PKA2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_7;
#[doc = "`read()` method returns [ltc0_pka2_7::R](ltc0_pka2_7::R) reader structure"]
impl crate::Readable for LTC0_PKA2_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_7::W](ltc0_pka2_7::W) writer structure"]
impl crate::Writable for LTC0_PKA2_7 {}
#[doc = "LTC PKHA A2 7 Register"]
pub mod ltc0_pka2_7;
#[doc = "LTC PKHA A 39 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_39](ltc0_pka_39) module"]
pub type LTC0_PKA_39 = crate::Reg<u32, _LTC0_PKA_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_39;
#[doc = "`read()` method returns [ltc0_pka_39::R](ltc0_pka_39::R) reader structure"]
impl crate::Readable for LTC0_PKA_39 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_39::W](ltc0_pka_39::W) writer structure"]
impl crate::Writable for LTC0_PKA_39 {}
#[doc = "LTC PKHA A 39 Register"]
pub mod ltc0_pka_39;
#[doc = "LTC PKHA A2 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_8](ltc0_pka2_8) module"]
pub type LTC0_PKA2_8 = crate::Reg<u32, _LTC0_PKA2_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_8;
#[doc = "`read()` method returns [ltc0_pka2_8::R](ltc0_pka2_8::R) reader structure"]
impl crate::Readable for LTC0_PKA2_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_8::W](ltc0_pka2_8::W) writer structure"]
impl crate::Writable for LTC0_PKA2_8 {}
#[doc = "LTC PKHA A2 8 Register"]
pub mod ltc0_pka2_8;
#[doc = "LTC PKHA A 40 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_40](ltc0_pka_40) module"]
pub type LTC0_PKA_40 = crate::Reg<u32, _LTC0_PKA_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_40;
#[doc = "`read()` method returns [ltc0_pka_40::R](ltc0_pka_40::R) reader structure"]
impl crate::Readable for LTC0_PKA_40 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_40::W](ltc0_pka_40::W) writer structure"]
impl crate::Writable for LTC0_PKA_40 {}
#[doc = "LTC PKHA A 40 Register"]
pub mod ltc0_pka_40;
#[doc = "LTC PKHA A2 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_9](ltc0_pka2_9) module"]
pub type LTC0_PKA2_9 = crate::Reg<u32, _LTC0_PKA2_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_9;
#[doc = "`read()` method returns [ltc0_pka2_9::R](ltc0_pka2_9::R) reader structure"]
impl crate::Readable for LTC0_PKA2_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_9::W](ltc0_pka2_9::W) writer structure"]
impl crate::Writable for LTC0_PKA2_9 {}
#[doc = "LTC PKHA A2 9 Register"]
pub mod ltc0_pka2_9;
#[doc = "LTC PKHA A 41 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_41](ltc0_pka_41) module"]
pub type LTC0_PKA_41 = crate::Reg<u32, _LTC0_PKA_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_41;
#[doc = "`read()` method returns [ltc0_pka_41::R](ltc0_pka_41::R) reader structure"]
impl crate::Readable for LTC0_PKA_41 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_41::W](ltc0_pka_41::W) writer structure"]
impl crate::Writable for LTC0_PKA_41 {}
#[doc = "LTC PKHA A 41 Register"]
pub mod ltc0_pka_41;
#[doc = "LTC PKHA A2 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_10](ltc0_pka2_10) module"]
pub type LTC0_PKA2_10 = crate::Reg<u32, _LTC0_PKA2_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_10;
#[doc = "`read()` method returns [ltc0_pka2_10::R](ltc0_pka2_10::R) reader structure"]
impl crate::Readable for LTC0_PKA2_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_10::W](ltc0_pka2_10::W) writer structure"]
impl crate::Writable for LTC0_PKA2_10 {}
#[doc = "LTC PKHA A2 10 Register"]
pub mod ltc0_pka2_10;
#[doc = "LTC PKHA A 42 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_42](ltc0_pka_42) module"]
pub type LTC0_PKA_42 = crate::Reg<u32, _LTC0_PKA_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_42;
#[doc = "`read()` method returns [ltc0_pka_42::R](ltc0_pka_42::R) reader structure"]
impl crate::Readable for LTC0_PKA_42 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_42::W](ltc0_pka_42::W) writer structure"]
impl crate::Writable for LTC0_PKA_42 {}
#[doc = "LTC PKHA A 42 Register"]
pub mod ltc0_pka_42;
#[doc = "LTC PKHA A2 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_11](ltc0_pka2_11) module"]
pub type LTC0_PKA2_11 = crate::Reg<u32, _LTC0_PKA2_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_11;
#[doc = "`read()` method returns [ltc0_pka2_11::R](ltc0_pka2_11::R) reader structure"]
impl crate::Readable for LTC0_PKA2_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_11::W](ltc0_pka2_11::W) writer structure"]
impl crate::Writable for LTC0_PKA2_11 {}
#[doc = "LTC PKHA A2 11 Register"]
pub mod ltc0_pka2_11;
#[doc = "LTC PKHA A 43 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_43](ltc0_pka_43) module"]
pub type LTC0_PKA_43 = crate::Reg<u32, _LTC0_PKA_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_43;
#[doc = "`read()` method returns [ltc0_pka_43::R](ltc0_pka_43::R) reader structure"]
impl crate::Readable for LTC0_PKA_43 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_43::W](ltc0_pka_43::W) writer structure"]
impl crate::Writable for LTC0_PKA_43 {}
#[doc = "LTC PKHA A 43 Register"]
pub mod ltc0_pka_43;
#[doc = "LTC PKHA A2 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_12](ltc0_pka2_12) module"]
pub type LTC0_PKA2_12 = crate::Reg<u32, _LTC0_PKA2_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_12;
#[doc = "`read()` method returns [ltc0_pka2_12::R](ltc0_pka2_12::R) reader structure"]
impl crate::Readable for LTC0_PKA2_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_12::W](ltc0_pka2_12::W) writer structure"]
impl crate::Writable for LTC0_PKA2_12 {}
#[doc = "LTC PKHA A2 12 Register"]
pub mod ltc0_pka2_12;
#[doc = "LTC PKHA A 44 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_44](ltc0_pka_44) module"]
pub type LTC0_PKA_44 = crate::Reg<u32, _LTC0_PKA_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_44;
#[doc = "`read()` method returns [ltc0_pka_44::R](ltc0_pka_44::R) reader structure"]
impl crate::Readable for LTC0_PKA_44 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_44::W](ltc0_pka_44::W) writer structure"]
impl crate::Writable for LTC0_PKA_44 {}
#[doc = "LTC PKHA A 44 Register"]
pub mod ltc0_pka_44;
#[doc = "LTC PKHA A2 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_13](ltc0_pka2_13) module"]
pub type LTC0_PKA2_13 = crate::Reg<u32, _LTC0_PKA2_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_13;
#[doc = "`read()` method returns [ltc0_pka2_13::R](ltc0_pka2_13::R) reader structure"]
impl crate::Readable for LTC0_PKA2_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_13::W](ltc0_pka2_13::W) writer structure"]
impl crate::Writable for LTC0_PKA2_13 {}
#[doc = "LTC PKHA A2 13 Register"]
pub mod ltc0_pka2_13;
#[doc = "LTC PKHA A 45 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_45](ltc0_pka_45) module"]
pub type LTC0_PKA_45 = crate::Reg<u32, _LTC0_PKA_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_45;
#[doc = "`read()` method returns [ltc0_pka_45::R](ltc0_pka_45::R) reader structure"]
impl crate::Readable for LTC0_PKA_45 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_45::W](ltc0_pka_45::W) writer structure"]
impl crate::Writable for LTC0_PKA_45 {}
#[doc = "LTC PKHA A 45 Register"]
pub mod ltc0_pka_45;
#[doc = "LTC PKHA A2 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_14](ltc0_pka2_14) module"]
pub type LTC0_PKA2_14 = crate::Reg<u32, _LTC0_PKA2_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_14;
#[doc = "`read()` method returns [ltc0_pka2_14::R](ltc0_pka2_14::R) reader structure"]
impl crate::Readable for LTC0_PKA2_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_14::W](ltc0_pka2_14::W) writer structure"]
impl crate::Writable for LTC0_PKA2_14 {}
#[doc = "LTC PKHA A2 14 Register"]
pub mod ltc0_pka2_14;
#[doc = "LTC PKHA A 46 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_46](ltc0_pka_46) module"]
pub type LTC0_PKA_46 = crate::Reg<u32, _LTC0_PKA_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_46;
#[doc = "`read()` method returns [ltc0_pka_46::R](ltc0_pka_46::R) reader structure"]
impl crate::Readable for LTC0_PKA_46 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_46::W](ltc0_pka_46::W) writer structure"]
impl crate::Writable for LTC0_PKA_46 {}
#[doc = "LTC PKHA A 46 Register"]
pub mod ltc0_pka_46;
#[doc = "LTC PKHA A2 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka2_15](ltc0_pka2_15) module"]
pub type LTC0_PKA2_15 = crate::Reg<u32, _LTC0_PKA2_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA2_15;
#[doc = "`read()` method returns [ltc0_pka2_15::R](ltc0_pka2_15::R) reader structure"]
impl crate::Readable for LTC0_PKA2_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka2_15::W](ltc0_pka2_15::W) writer structure"]
impl crate::Writable for LTC0_PKA2_15 {}
#[doc = "LTC PKHA A2 15 Register"]
pub mod ltc0_pka2_15;
#[doc = "LTC PKHA A 47 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_47](ltc0_pka_47) module"]
pub type LTC0_PKA_47 = crate::Reg<u32, _LTC0_PKA_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_47;
#[doc = "`read()` method returns [ltc0_pka_47::R](ltc0_pka_47::R) reader structure"]
impl crate::Readable for LTC0_PKA_47 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_47::W](ltc0_pka_47::W) writer structure"]
impl crate::Writable for LTC0_PKA_47 {}
#[doc = "LTC PKHA A 47 Register"]
pub mod ltc0_pka_47;
#[doc = "LTC PKHA A3 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_0](ltc0_pka3_0) module"]
pub type LTC0_PKA3_0 = crate::Reg<u32, _LTC0_PKA3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_0;
#[doc = "`read()` method returns [ltc0_pka3_0::R](ltc0_pka3_0::R) reader structure"]
impl crate::Readable for LTC0_PKA3_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_0::W](ltc0_pka3_0::W) writer structure"]
impl crate::Writable for LTC0_PKA3_0 {}
#[doc = "LTC PKHA A3 0 Register"]
pub mod ltc0_pka3_0;
#[doc = "LTC PKHA A 48 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_48](ltc0_pka_48) module"]
pub type LTC0_PKA_48 = crate::Reg<u32, _LTC0_PKA_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_48;
#[doc = "`read()` method returns [ltc0_pka_48::R](ltc0_pka_48::R) reader structure"]
impl crate::Readable for LTC0_PKA_48 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_48::W](ltc0_pka_48::W) writer structure"]
impl crate::Writable for LTC0_PKA_48 {}
#[doc = "LTC PKHA A 48 Register"]
pub mod ltc0_pka_48;
#[doc = "LTC PKHA A3 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_1](ltc0_pka3_1) module"]
pub type LTC0_PKA3_1 = crate::Reg<u32, _LTC0_PKA3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_1;
#[doc = "`read()` method returns [ltc0_pka3_1::R](ltc0_pka3_1::R) reader structure"]
impl crate::Readable for LTC0_PKA3_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_1::W](ltc0_pka3_1::W) writer structure"]
impl crate::Writable for LTC0_PKA3_1 {}
#[doc = "LTC PKHA A3 1 Register"]
pub mod ltc0_pka3_1;
#[doc = "LTC PKHA A 49 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_49](ltc0_pka_49) module"]
pub type LTC0_PKA_49 = crate::Reg<u32, _LTC0_PKA_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_49;
#[doc = "`read()` method returns [ltc0_pka_49::R](ltc0_pka_49::R) reader structure"]
impl crate::Readable for LTC0_PKA_49 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_49::W](ltc0_pka_49::W) writer structure"]
impl crate::Writable for LTC0_PKA_49 {}
#[doc = "LTC PKHA A 49 Register"]
pub mod ltc0_pka_49;
#[doc = "LTC PKHA A3 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_2](ltc0_pka3_2) module"]
pub type LTC0_PKA3_2 = crate::Reg<u32, _LTC0_PKA3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_2;
#[doc = "`read()` method returns [ltc0_pka3_2::R](ltc0_pka3_2::R) reader structure"]
impl crate::Readable for LTC0_PKA3_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_2::W](ltc0_pka3_2::W) writer structure"]
impl crate::Writable for LTC0_PKA3_2 {}
#[doc = "LTC PKHA A3 2 Register"]
pub mod ltc0_pka3_2;
#[doc = "LTC PKHA A 50 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_50](ltc0_pka_50) module"]
pub type LTC0_PKA_50 = crate::Reg<u32, _LTC0_PKA_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_50;
#[doc = "`read()` method returns [ltc0_pka_50::R](ltc0_pka_50::R) reader structure"]
impl crate::Readable for LTC0_PKA_50 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_50::W](ltc0_pka_50::W) writer structure"]
impl crate::Writable for LTC0_PKA_50 {}
#[doc = "LTC PKHA A 50 Register"]
pub mod ltc0_pka_50;
#[doc = "LTC PKHA A3 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_3](ltc0_pka3_3) module"]
pub type LTC0_PKA3_3 = crate::Reg<u32, _LTC0_PKA3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_3;
#[doc = "`read()` method returns [ltc0_pka3_3::R](ltc0_pka3_3::R) reader structure"]
impl crate::Readable for LTC0_PKA3_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_3::W](ltc0_pka3_3::W) writer structure"]
impl crate::Writable for LTC0_PKA3_3 {}
#[doc = "LTC PKHA A3 3 Register"]
pub mod ltc0_pka3_3;
#[doc = "LTC PKHA A 51 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_51](ltc0_pka_51) module"]
pub type LTC0_PKA_51 = crate::Reg<u32, _LTC0_PKA_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_51;
#[doc = "`read()` method returns [ltc0_pka_51::R](ltc0_pka_51::R) reader structure"]
impl crate::Readable for LTC0_PKA_51 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_51::W](ltc0_pka_51::W) writer structure"]
impl crate::Writable for LTC0_PKA_51 {}
#[doc = "LTC PKHA A 51 Register"]
pub mod ltc0_pka_51;
#[doc = "LTC PKHA A3 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_4](ltc0_pka3_4) module"]
pub type LTC0_PKA3_4 = crate::Reg<u32, _LTC0_PKA3_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_4;
#[doc = "`read()` method returns [ltc0_pka3_4::R](ltc0_pka3_4::R) reader structure"]
impl crate::Readable for LTC0_PKA3_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_4::W](ltc0_pka3_4::W) writer structure"]
impl crate::Writable for LTC0_PKA3_4 {}
#[doc = "LTC PKHA A3 4 Register"]
pub mod ltc0_pka3_4;
#[doc = "LTC PKHA A 52 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_52](ltc0_pka_52) module"]
pub type LTC0_PKA_52 = crate::Reg<u32, _LTC0_PKA_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_52;
#[doc = "`read()` method returns [ltc0_pka_52::R](ltc0_pka_52::R) reader structure"]
impl crate::Readable for LTC0_PKA_52 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_52::W](ltc0_pka_52::W) writer structure"]
impl crate::Writable for LTC0_PKA_52 {}
#[doc = "LTC PKHA A 52 Register"]
pub mod ltc0_pka_52;
#[doc = "LTC PKHA A3 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_5](ltc0_pka3_5) module"]
pub type LTC0_PKA3_5 = crate::Reg<u32, _LTC0_PKA3_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_5;
#[doc = "`read()` method returns [ltc0_pka3_5::R](ltc0_pka3_5::R) reader structure"]
impl crate::Readable for LTC0_PKA3_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_5::W](ltc0_pka3_5::W) writer structure"]
impl crate::Writable for LTC0_PKA3_5 {}
#[doc = "LTC PKHA A3 5 Register"]
pub mod ltc0_pka3_5;
#[doc = "LTC PKHA A 53 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_53](ltc0_pka_53) module"]
pub type LTC0_PKA_53 = crate::Reg<u32, _LTC0_PKA_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_53;
#[doc = "`read()` method returns [ltc0_pka_53::R](ltc0_pka_53::R) reader structure"]
impl crate::Readable for LTC0_PKA_53 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_53::W](ltc0_pka_53::W) writer structure"]
impl crate::Writable for LTC0_PKA_53 {}
#[doc = "LTC PKHA A 53 Register"]
pub mod ltc0_pka_53;
#[doc = "LTC PKHA A3 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_6](ltc0_pka3_6) module"]
pub type LTC0_PKA3_6 = crate::Reg<u32, _LTC0_PKA3_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_6;
#[doc = "`read()` method returns [ltc0_pka3_6::R](ltc0_pka3_6::R) reader structure"]
impl crate::Readable for LTC0_PKA3_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_6::W](ltc0_pka3_6::W) writer structure"]
impl crate::Writable for LTC0_PKA3_6 {}
#[doc = "LTC PKHA A3 6 Register"]
pub mod ltc0_pka3_6;
#[doc = "LTC PKHA A 54 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_54](ltc0_pka_54) module"]
pub type LTC0_PKA_54 = crate::Reg<u32, _LTC0_PKA_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_54;
#[doc = "`read()` method returns [ltc0_pka_54::R](ltc0_pka_54::R) reader structure"]
impl crate::Readable for LTC0_PKA_54 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_54::W](ltc0_pka_54::W) writer structure"]
impl crate::Writable for LTC0_PKA_54 {}
#[doc = "LTC PKHA A 54 Register"]
pub mod ltc0_pka_54;
#[doc = "LTC PKHA A3 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_7](ltc0_pka3_7) module"]
pub type LTC0_PKA3_7 = crate::Reg<u32, _LTC0_PKA3_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_7;
#[doc = "`read()` method returns [ltc0_pka3_7::R](ltc0_pka3_7::R) reader structure"]
impl crate::Readable for LTC0_PKA3_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_7::W](ltc0_pka3_7::W) writer structure"]
impl crate::Writable for LTC0_PKA3_7 {}
#[doc = "LTC PKHA A3 7 Register"]
pub mod ltc0_pka3_7;
#[doc = "LTC PKHA A 55 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_55](ltc0_pka_55) module"]
pub type LTC0_PKA_55 = crate::Reg<u32, _LTC0_PKA_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_55;
#[doc = "`read()` method returns [ltc0_pka_55::R](ltc0_pka_55::R) reader structure"]
impl crate::Readable for LTC0_PKA_55 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_55::W](ltc0_pka_55::W) writer structure"]
impl crate::Writable for LTC0_PKA_55 {}
#[doc = "LTC PKHA A 55 Register"]
pub mod ltc0_pka_55;
#[doc = "LTC PKHA A3 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_8](ltc0_pka3_8) module"]
pub type LTC0_PKA3_8 = crate::Reg<u32, _LTC0_PKA3_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_8;
#[doc = "`read()` method returns [ltc0_pka3_8::R](ltc0_pka3_8::R) reader structure"]
impl crate::Readable for LTC0_PKA3_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_8::W](ltc0_pka3_8::W) writer structure"]
impl crate::Writable for LTC0_PKA3_8 {}
#[doc = "LTC PKHA A3 8 Register"]
pub mod ltc0_pka3_8;
#[doc = "LTC PKHA A 56 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_56](ltc0_pka_56) module"]
pub type LTC0_PKA_56 = crate::Reg<u32, _LTC0_PKA_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_56;
#[doc = "`read()` method returns [ltc0_pka_56::R](ltc0_pka_56::R) reader structure"]
impl crate::Readable for LTC0_PKA_56 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_56::W](ltc0_pka_56::W) writer structure"]
impl crate::Writable for LTC0_PKA_56 {}
#[doc = "LTC PKHA A 56 Register"]
pub mod ltc0_pka_56;
#[doc = "LTC PKHA A3 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_9](ltc0_pka3_9) module"]
pub type LTC0_PKA3_9 = crate::Reg<u32, _LTC0_PKA3_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_9;
#[doc = "`read()` method returns [ltc0_pka3_9::R](ltc0_pka3_9::R) reader structure"]
impl crate::Readable for LTC0_PKA3_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_9::W](ltc0_pka3_9::W) writer structure"]
impl crate::Writable for LTC0_PKA3_9 {}
#[doc = "LTC PKHA A3 9 Register"]
pub mod ltc0_pka3_9;
#[doc = "LTC PKHA A 57 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_57](ltc0_pka_57) module"]
pub type LTC0_PKA_57 = crate::Reg<u32, _LTC0_PKA_57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_57;
#[doc = "`read()` method returns [ltc0_pka_57::R](ltc0_pka_57::R) reader structure"]
impl crate::Readable for LTC0_PKA_57 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_57::W](ltc0_pka_57::W) writer structure"]
impl crate::Writable for LTC0_PKA_57 {}
#[doc = "LTC PKHA A 57 Register"]
pub mod ltc0_pka_57;
#[doc = "LTC PKHA A3 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_10](ltc0_pka3_10) module"]
pub type LTC0_PKA3_10 = crate::Reg<u32, _LTC0_PKA3_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_10;
#[doc = "`read()` method returns [ltc0_pka3_10::R](ltc0_pka3_10::R) reader structure"]
impl crate::Readable for LTC0_PKA3_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_10::W](ltc0_pka3_10::W) writer structure"]
impl crate::Writable for LTC0_PKA3_10 {}
#[doc = "LTC PKHA A3 10 Register"]
pub mod ltc0_pka3_10;
#[doc = "LTC PKHA A 58 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_58](ltc0_pka_58) module"]
pub type LTC0_PKA_58 = crate::Reg<u32, _LTC0_PKA_58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_58;
#[doc = "`read()` method returns [ltc0_pka_58::R](ltc0_pka_58::R) reader structure"]
impl crate::Readable for LTC0_PKA_58 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_58::W](ltc0_pka_58::W) writer structure"]
impl crate::Writable for LTC0_PKA_58 {}
#[doc = "LTC PKHA A 58 Register"]
pub mod ltc0_pka_58;
#[doc = "LTC PKHA A3 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_11](ltc0_pka3_11) module"]
pub type LTC0_PKA3_11 = crate::Reg<u32, _LTC0_PKA3_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_11;
#[doc = "`read()` method returns [ltc0_pka3_11::R](ltc0_pka3_11::R) reader structure"]
impl crate::Readable for LTC0_PKA3_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_11::W](ltc0_pka3_11::W) writer structure"]
impl crate::Writable for LTC0_PKA3_11 {}
#[doc = "LTC PKHA A3 11 Register"]
pub mod ltc0_pka3_11;
#[doc = "LTC PKHA A 59 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_59](ltc0_pka_59) module"]
pub type LTC0_PKA_59 = crate::Reg<u32, _LTC0_PKA_59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_59;
#[doc = "`read()` method returns [ltc0_pka_59::R](ltc0_pka_59::R) reader structure"]
impl crate::Readable for LTC0_PKA_59 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_59::W](ltc0_pka_59::W) writer structure"]
impl crate::Writable for LTC0_PKA_59 {}
#[doc = "LTC PKHA A 59 Register"]
pub mod ltc0_pka_59;
#[doc = "LTC PKHA A3 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_12](ltc0_pka3_12) module"]
pub type LTC0_PKA3_12 = crate::Reg<u32, _LTC0_PKA3_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_12;
#[doc = "`read()` method returns [ltc0_pka3_12::R](ltc0_pka3_12::R) reader structure"]
impl crate::Readable for LTC0_PKA3_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_12::W](ltc0_pka3_12::W) writer structure"]
impl crate::Writable for LTC0_PKA3_12 {}
#[doc = "LTC PKHA A3 12 Register"]
pub mod ltc0_pka3_12;
#[doc = "LTC PKHA A 60 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_60](ltc0_pka_60) module"]
pub type LTC0_PKA_60 = crate::Reg<u32, _LTC0_PKA_60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_60;
#[doc = "`read()` method returns [ltc0_pka_60::R](ltc0_pka_60::R) reader structure"]
impl crate::Readable for LTC0_PKA_60 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_60::W](ltc0_pka_60::W) writer structure"]
impl crate::Writable for LTC0_PKA_60 {}
#[doc = "LTC PKHA A 60 Register"]
pub mod ltc0_pka_60;
#[doc = "LTC PKHA A3 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_13](ltc0_pka3_13) module"]
pub type LTC0_PKA3_13 = crate::Reg<u32, _LTC0_PKA3_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_13;
#[doc = "`read()` method returns [ltc0_pka3_13::R](ltc0_pka3_13::R) reader structure"]
impl crate::Readable for LTC0_PKA3_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_13::W](ltc0_pka3_13::W) writer structure"]
impl crate::Writable for LTC0_PKA3_13 {}
#[doc = "LTC PKHA A3 13 Register"]
pub mod ltc0_pka3_13;
#[doc = "LTC PKHA A 61 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_61](ltc0_pka_61) module"]
pub type LTC0_PKA_61 = crate::Reg<u32, _LTC0_PKA_61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_61;
#[doc = "`read()` method returns [ltc0_pka_61::R](ltc0_pka_61::R) reader structure"]
impl crate::Readable for LTC0_PKA_61 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_61::W](ltc0_pka_61::W) writer structure"]
impl crate::Writable for LTC0_PKA_61 {}
#[doc = "LTC PKHA A 61 Register"]
pub mod ltc0_pka_61;
#[doc = "LTC PKHA A3 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_14](ltc0_pka3_14) module"]
pub type LTC0_PKA3_14 = crate::Reg<u32, _LTC0_PKA3_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_14;
#[doc = "`read()` method returns [ltc0_pka3_14::R](ltc0_pka3_14::R) reader structure"]
impl crate::Readable for LTC0_PKA3_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_14::W](ltc0_pka3_14::W) writer structure"]
impl crate::Writable for LTC0_PKA3_14 {}
#[doc = "LTC PKHA A3 14 Register"]
pub mod ltc0_pka3_14;
#[doc = "LTC PKHA A 62 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_62](ltc0_pka_62) module"]
pub type LTC0_PKA_62 = crate::Reg<u32, _LTC0_PKA_62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_62;
#[doc = "`read()` method returns [ltc0_pka_62::R](ltc0_pka_62::R) reader structure"]
impl crate::Readable for LTC0_PKA_62 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_62::W](ltc0_pka_62::W) writer structure"]
impl crate::Writable for LTC0_PKA_62 {}
#[doc = "LTC PKHA A 62 Register"]
pub mod ltc0_pka_62;
#[doc = "LTC PKHA A3 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka3_15](ltc0_pka3_15) module"]
pub type LTC0_PKA3_15 = crate::Reg<u32, _LTC0_PKA3_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA3_15;
#[doc = "`read()` method returns [ltc0_pka3_15::R](ltc0_pka3_15::R) reader structure"]
impl crate::Readable for LTC0_PKA3_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka3_15::W](ltc0_pka3_15::W) writer structure"]
impl crate::Writable for LTC0_PKA3_15 {}
#[doc = "LTC PKHA A3 15 Register"]
pub mod ltc0_pka3_15;
#[doc = "LTC PKHA A 63 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pka_63](ltc0_pka_63) module"]
pub type LTC0_PKA_63 = crate::Reg<u32, _LTC0_PKA_63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKA_63;
#[doc = "`read()` method returns [ltc0_pka_63::R](ltc0_pka_63::R) reader structure"]
impl crate::Readable for LTC0_PKA_63 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pka_63::W](ltc0_pka_63::W) writer structure"]
impl crate::Writable for LTC0_PKA_63 {}
#[doc = "LTC PKHA A 63 Register"]
pub mod ltc0_pka_63;
#[doc = "LTC PKHA B0 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_0](ltc0_pkb0_0) module"]
pub type LTC0_PKB0_0 = crate::Reg<u32, _LTC0_PKB0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_0;
#[doc = "`read()` method returns [ltc0_pkb0_0::R](ltc0_pkb0_0::R) reader structure"]
impl crate::Readable for LTC0_PKB0_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_0::W](ltc0_pkb0_0::W) writer structure"]
impl crate::Writable for LTC0_PKB0_0 {}
#[doc = "LTC PKHA B0 0 Register"]
pub mod ltc0_pkb0_0;
#[doc = "LTC PKHA B 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_0](ltc0_pkb_0) module"]
pub type LTC0_PKB_0 = crate::Reg<u32, _LTC0_PKB_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_0;
#[doc = "`read()` method returns [ltc0_pkb_0::R](ltc0_pkb_0::R) reader structure"]
impl crate::Readable for LTC0_PKB_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_0::W](ltc0_pkb_0::W) writer structure"]
impl crate::Writable for LTC0_PKB_0 {}
#[doc = "LTC PKHA B 0 Register"]
pub mod ltc0_pkb_0;
#[doc = "LTC PKHA B0 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_1](ltc0_pkb0_1) module"]
pub type LTC0_PKB0_1 = crate::Reg<u32, _LTC0_PKB0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_1;
#[doc = "`read()` method returns [ltc0_pkb0_1::R](ltc0_pkb0_1::R) reader structure"]
impl crate::Readable for LTC0_PKB0_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_1::W](ltc0_pkb0_1::W) writer structure"]
impl crate::Writable for LTC0_PKB0_1 {}
#[doc = "LTC PKHA B0 1 Register"]
pub mod ltc0_pkb0_1;
#[doc = "LTC PKHA B 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_1](ltc0_pkb_1) module"]
pub type LTC0_PKB_1 = crate::Reg<u32, _LTC0_PKB_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_1;
#[doc = "`read()` method returns [ltc0_pkb_1::R](ltc0_pkb_1::R) reader structure"]
impl crate::Readable for LTC0_PKB_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_1::W](ltc0_pkb_1::W) writer structure"]
impl crate::Writable for LTC0_PKB_1 {}
#[doc = "LTC PKHA B 1 Register"]
pub mod ltc0_pkb_1;
#[doc = "LTC PKHA B0 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_2](ltc0_pkb0_2) module"]
pub type LTC0_PKB0_2 = crate::Reg<u32, _LTC0_PKB0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_2;
#[doc = "`read()` method returns [ltc0_pkb0_2::R](ltc0_pkb0_2::R) reader structure"]
impl crate::Readable for LTC0_PKB0_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_2::W](ltc0_pkb0_2::W) writer structure"]
impl crate::Writable for LTC0_PKB0_2 {}
#[doc = "LTC PKHA B0 2 Register"]
pub mod ltc0_pkb0_2;
#[doc = "LTC PKHA B 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_2](ltc0_pkb_2) module"]
pub type LTC0_PKB_2 = crate::Reg<u32, _LTC0_PKB_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_2;
#[doc = "`read()` method returns [ltc0_pkb_2::R](ltc0_pkb_2::R) reader structure"]
impl crate::Readable for LTC0_PKB_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_2::W](ltc0_pkb_2::W) writer structure"]
impl crate::Writable for LTC0_PKB_2 {}
#[doc = "LTC PKHA B 2 Register"]
pub mod ltc0_pkb_2;
#[doc = "LTC PKHA B0 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_3](ltc0_pkb0_3) module"]
pub type LTC0_PKB0_3 = crate::Reg<u32, _LTC0_PKB0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_3;
#[doc = "`read()` method returns [ltc0_pkb0_3::R](ltc0_pkb0_3::R) reader structure"]
impl crate::Readable for LTC0_PKB0_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_3::W](ltc0_pkb0_3::W) writer structure"]
impl crate::Writable for LTC0_PKB0_3 {}
#[doc = "LTC PKHA B0 3 Register"]
pub mod ltc0_pkb0_3;
#[doc = "LTC PKHA B 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_3](ltc0_pkb_3) module"]
pub type LTC0_PKB_3 = crate::Reg<u32, _LTC0_PKB_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_3;
#[doc = "`read()` method returns [ltc0_pkb_3::R](ltc0_pkb_3::R) reader structure"]
impl crate::Readable for LTC0_PKB_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_3::W](ltc0_pkb_3::W) writer structure"]
impl crate::Writable for LTC0_PKB_3 {}
#[doc = "LTC PKHA B 3 Register"]
pub mod ltc0_pkb_3;
#[doc = "LTC PKHA B0 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_4](ltc0_pkb0_4) module"]
pub type LTC0_PKB0_4 = crate::Reg<u32, _LTC0_PKB0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_4;
#[doc = "`read()` method returns [ltc0_pkb0_4::R](ltc0_pkb0_4::R) reader structure"]
impl crate::Readable for LTC0_PKB0_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_4::W](ltc0_pkb0_4::W) writer structure"]
impl crate::Writable for LTC0_PKB0_4 {}
#[doc = "LTC PKHA B0 4 Register"]
pub mod ltc0_pkb0_4;
#[doc = "LTC PKHA B 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_4](ltc0_pkb_4) module"]
pub type LTC0_PKB_4 = crate::Reg<u32, _LTC0_PKB_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_4;
#[doc = "`read()` method returns [ltc0_pkb_4::R](ltc0_pkb_4::R) reader structure"]
impl crate::Readable for LTC0_PKB_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_4::W](ltc0_pkb_4::W) writer structure"]
impl crate::Writable for LTC0_PKB_4 {}
#[doc = "LTC PKHA B 4 Register"]
pub mod ltc0_pkb_4;
#[doc = "LTC PKHA B0 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_5](ltc0_pkb0_5) module"]
pub type LTC0_PKB0_5 = crate::Reg<u32, _LTC0_PKB0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_5;
#[doc = "`read()` method returns [ltc0_pkb0_5::R](ltc0_pkb0_5::R) reader structure"]
impl crate::Readable for LTC0_PKB0_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_5::W](ltc0_pkb0_5::W) writer structure"]
impl crate::Writable for LTC0_PKB0_5 {}
#[doc = "LTC PKHA B0 5 Register"]
pub mod ltc0_pkb0_5;
#[doc = "LTC PKHA B 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_5](ltc0_pkb_5) module"]
pub type LTC0_PKB_5 = crate::Reg<u32, _LTC0_PKB_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_5;
#[doc = "`read()` method returns [ltc0_pkb_5::R](ltc0_pkb_5::R) reader structure"]
impl crate::Readable for LTC0_PKB_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_5::W](ltc0_pkb_5::W) writer structure"]
impl crate::Writable for LTC0_PKB_5 {}
#[doc = "LTC PKHA B 5 Register"]
pub mod ltc0_pkb_5;
#[doc = "LTC PKHA B0 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_6](ltc0_pkb0_6) module"]
pub type LTC0_PKB0_6 = crate::Reg<u32, _LTC0_PKB0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_6;
#[doc = "`read()` method returns [ltc0_pkb0_6::R](ltc0_pkb0_6::R) reader structure"]
impl crate::Readable for LTC0_PKB0_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_6::W](ltc0_pkb0_6::W) writer structure"]
impl crate::Writable for LTC0_PKB0_6 {}
#[doc = "LTC PKHA B0 6 Register"]
pub mod ltc0_pkb0_6;
#[doc = "LTC PKHA B 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_6](ltc0_pkb_6) module"]
pub type LTC0_PKB_6 = crate::Reg<u32, _LTC0_PKB_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_6;
#[doc = "`read()` method returns [ltc0_pkb_6::R](ltc0_pkb_6::R) reader structure"]
impl crate::Readable for LTC0_PKB_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_6::W](ltc0_pkb_6::W) writer structure"]
impl crate::Writable for LTC0_PKB_6 {}
#[doc = "LTC PKHA B 6 Register"]
pub mod ltc0_pkb_6;
#[doc = "LTC PKHA B0 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_7](ltc0_pkb0_7) module"]
pub type LTC0_PKB0_7 = crate::Reg<u32, _LTC0_PKB0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_7;
#[doc = "`read()` method returns [ltc0_pkb0_7::R](ltc0_pkb0_7::R) reader structure"]
impl crate::Readable for LTC0_PKB0_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_7::W](ltc0_pkb0_7::W) writer structure"]
impl crate::Writable for LTC0_PKB0_7 {}
#[doc = "LTC PKHA B0 7 Register"]
pub mod ltc0_pkb0_7;
#[doc = "LTC PKHA B 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_7](ltc0_pkb_7) module"]
pub type LTC0_PKB_7 = crate::Reg<u32, _LTC0_PKB_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_7;
#[doc = "`read()` method returns [ltc0_pkb_7::R](ltc0_pkb_7::R) reader structure"]
impl crate::Readable for LTC0_PKB_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_7::W](ltc0_pkb_7::W) writer structure"]
impl crate::Writable for LTC0_PKB_7 {}
#[doc = "LTC PKHA B 7 Register"]
pub mod ltc0_pkb_7;
#[doc = "LTC PKHA B0 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_8](ltc0_pkb0_8) module"]
pub type LTC0_PKB0_8 = crate::Reg<u32, _LTC0_PKB0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_8;
#[doc = "`read()` method returns [ltc0_pkb0_8::R](ltc0_pkb0_8::R) reader structure"]
impl crate::Readable for LTC0_PKB0_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_8::W](ltc0_pkb0_8::W) writer structure"]
impl crate::Writable for LTC0_PKB0_8 {}
#[doc = "LTC PKHA B0 8 Register"]
pub mod ltc0_pkb0_8;
#[doc = "LTC PKHA B 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_8](ltc0_pkb_8) module"]
pub type LTC0_PKB_8 = crate::Reg<u32, _LTC0_PKB_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_8;
#[doc = "`read()` method returns [ltc0_pkb_8::R](ltc0_pkb_8::R) reader structure"]
impl crate::Readable for LTC0_PKB_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_8::W](ltc0_pkb_8::W) writer structure"]
impl crate::Writable for LTC0_PKB_8 {}
#[doc = "LTC PKHA B 8 Register"]
pub mod ltc0_pkb_8;
#[doc = "LTC PKHA B0 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_9](ltc0_pkb0_9) module"]
pub type LTC0_PKB0_9 = crate::Reg<u32, _LTC0_PKB0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_9;
#[doc = "`read()` method returns [ltc0_pkb0_9::R](ltc0_pkb0_9::R) reader structure"]
impl crate::Readable for LTC0_PKB0_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_9::W](ltc0_pkb0_9::W) writer structure"]
impl crate::Writable for LTC0_PKB0_9 {}
#[doc = "LTC PKHA B0 9 Register"]
pub mod ltc0_pkb0_9;
#[doc = "LTC PKHA B 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_9](ltc0_pkb_9) module"]
pub type LTC0_PKB_9 = crate::Reg<u32, _LTC0_PKB_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_9;
#[doc = "`read()` method returns [ltc0_pkb_9::R](ltc0_pkb_9::R) reader structure"]
impl crate::Readable for LTC0_PKB_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_9::W](ltc0_pkb_9::W) writer structure"]
impl crate::Writable for LTC0_PKB_9 {}
#[doc = "LTC PKHA B 9 Register"]
pub mod ltc0_pkb_9;
#[doc = "LTC PKHA B0 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_10](ltc0_pkb0_10) module"]
pub type LTC0_PKB0_10 = crate::Reg<u32, _LTC0_PKB0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_10;
#[doc = "`read()` method returns [ltc0_pkb0_10::R](ltc0_pkb0_10::R) reader structure"]
impl crate::Readable for LTC0_PKB0_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_10::W](ltc0_pkb0_10::W) writer structure"]
impl crate::Writable for LTC0_PKB0_10 {}
#[doc = "LTC PKHA B0 10 Register"]
pub mod ltc0_pkb0_10;
#[doc = "LTC PKHA B 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_10](ltc0_pkb_10) module"]
pub type LTC0_PKB_10 = crate::Reg<u32, _LTC0_PKB_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_10;
#[doc = "`read()` method returns [ltc0_pkb_10::R](ltc0_pkb_10::R) reader structure"]
impl crate::Readable for LTC0_PKB_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_10::W](ltc0_pkb_10::W) writer structure"]
impl crate::Writable for LTC0_PKB_10 {}
#[doc = "LTC PKHA B 10 Register"]
pub mod ltc0_pkb_10;
#[doc = "LTC PKHA B0 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_11](ltc0_pkb0_11) module"]
pub type LTC0_PKB0_11 = crate::Reg<u32, _LTC0_PKB0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_11;
#[doc = "`read()` method returns [ltc0_pkb0_11::R](ltc0_pkb0_11::R) reader structure"]
impl crate::Readable for LTC0_PKB0_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_11::W](ltc0_pkb0_11::W) writer structure"]
impl crate::Writable for LTC0_PKB0_11 {}
#[doc = "LTC PKHA B0 11 Register"]
pub mod ltc0_pkb0_11;
#[doc = "LTC PKHA B 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_11](ltc0_pkb_11) module"]
pub type LTC0_PKB_11 = crate::Reg<u32, _LTC0_PKB_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_11;
#[doc = "`read()` method returns [ltc0_pkb_11::R](ltc0_pkb_11::R) reader structure"]
impl crate::Readable for LTC0_PKB_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_11::W](ltc0_pkb_11::W) writer structure"]
impl crate::Writable for LTC0_PKB_11 {}
#[doc = "LTC PKHA B 11 Register"]
pub mod ltc0_pkb_11;
#[doc = "LTC PKHA B0 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_12](ltc0_pkb0_12) module"]
pub type LTC0_PKB0_12 = crate::Reg<u32, _LTC0_PKB0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_12;
#[doc = "`read()` method returns [ltc0_pkb0_12::R](ltc0_pkb0_12::R) reader structure"]
impl crate::Readable for LTC0_PKB0_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_12::W](ltc0_pkb0_12::W) writer structure"]
impl crate::Writable for LTC0_PKB0_12 {}
#[doc = "LTC PKHA B0 12 Register"]
pub mod ltc0_pkb0_12;
#[doc = "LTC PKHA B 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_12](ltc0_pkb_12) module"]
pub type LTC0_PKB_12 = crate::Reg<u32, _LTC0_PKB_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_12;
#[doc = "`read()` method returns [ltc0_pkb_12::R](ltc0_pkb_12::R) reader structure"]
impl crate::Readable for LTC0_PKB_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_12::W](ltc0_pkb_12::W) writer structure"]
impl crate::Writable for LTC0_PKB_12 {}
#[doc = "LTC PKHA B 12 Register"]
pub mod ltc0_pkb_12;
#[doc = "LTC PKHA B0 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_13](ltc0_pkb0_13) module"]
pub type LTC0_PKB0_13 = crate::Reg<u32, _LTC0_PKB0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_13;
#[doc = "`read()` method returns [ltc0_pkb0_13::R](ltc0_pkb0_13::R) reader structure"]
impl crate::Readable for LTC0_PKB0_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_13::W](ltc0_pkb0_13::W) writer structure"]
impl crate::Writable for LTC0_PKB0_13 {}
#[doc = "LTC PKHA B0 13 Register"]
pub mod ltc0_pkb0_13;
#[doc = "LTC PKHA B 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_13](ltc0_pkb_13) module"]
pub type LTC0_PKB_13 = crate::Reg<u32, _LTC0_PKB_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_13;
#[doc = "`read()` method returns [ltc0_pkb_13::R](ltc0_pkb_13::R) reader structure"]
impl crate::Readable for LTC0_PKB_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_13::W](ltc0_pkb_13::W) writer structure"]
impl crate::Writable for LTC0_PKB_13 {}
#[doc = "LTC PKHA B 13 Register"]
pub mod ltc0_pkb_13;
#[doc = "LTC PKHA B0 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_14](ltc0_pkb0_14) module"]
pub type LTC0_PKB0_14 = crate::Reg<u32, _LTC0_PKB0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_14;
#[doc = "`read()` method returns [ltc0_pkb0_14::R](ltc0_pkb0_14::R) reader structure"]
impl crate::Readable for LTC0_PKB0_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_14::W](ltc0_pkb0_14::W) writer structure"]
impl crate::Writable for LTC0_PKB0_14 {}
#[doc = "LTC PKHA B0 14 Register"]
pub mod ltc0_pkb0_14;
#[doc = "LTC PKHA B 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_14](ltc0_pkb_14) module"]
pub type LTC0_PKB_14 = crate::Reg<u32, _LTC0_PKB_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_14;
#[doc = "`read()` method returns [ltc0_pkb_14::R](ltc0_pkb_14::R) reader structure"]
impl crate::Readable for LTC0_PKB_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_14::W](ltc0_pkb_14::W) writer structure"]
impl crate::Writable for LTC0_PKB_14 {}
#[doc = "LTC PKHA B 14 Register"]
pub mod ltc0_pkb_14;
#[doc = "LTC PKHA B0 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb0_15](ltc0_pkb0_15) module"]
pub type LTC0_PKB0_15 = crate::Reg<u32, _LTC0_PKB0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB0_15;
#[doc = "`read()` method returns [ltc0_pkb0_15::R](ltc0_pkb0_15::R) reader structure"]
impl crate::Readable for LTC0_PKB0_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb0_15::W](ltc0_pkb0_15::W) writer structure"]
impl crate::Writable for LTC0_PKB0_15 {}
#[doc = "LTC PKHA B0 15 Register"]
pub mod ltc0_pkb0_15;
#[doc = "LTC PKHA B 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_15](ltc0_pkb_15) module"]
pub type LTC0_PKB_15 = crate::Reg<u32, _LTC0_PKB_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_15;
#[doc = "`read()` method returns [ltc0_pkb_15::R](ltc0_pkb_15::R) reader structure"]
impl crate::Readable for LTC0_PKB_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_15::W](ltc0_pkb_15::W) writer structure"]
impl crate::Writable for LTC0_PKB_15 {}
#[doc = "LTC PKHA B 15 Register"]
pub mod ltc0_pkb_15;
#[doc = "LTC PKHA B1 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_0](ltc0_pkb1_0) module"]
pub type LTC0_PKB1_0 = crate::Reg<u32, _LTC0_PKB1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_0;
#[doc = "`read()` method returns [ltc0_pkb1_0::R](ltc0_pkb1_0::R) reader structure"]
impl crate::Readable for LTC0_PKB1_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_0::W](ltc0_pkb1_0::W) writer structure"]
impl crate::Writable for LTC0_PKB1_0 {}
#[doc = "LTC PKHA B1 0 Register"]
pub mod ltc0_pkb1_0;
#[doc = "LTC PKHA B 16 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_16](ltc0_pkb_16) module"]
pub type LTC0_PKB_16 = crate::Reg<u32, _LTC0_PKB_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_16;
#[doc = "`read()` method returns [ltc0_pkb_16::R](ltc0_pkb_16::R) reader structure"]
impl crate::Readable for LTC0_PKB_16 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_16::W](ltc0_pkb_16::W) writer structure"]
impl crate::Writable for LTC0_PKB_16 {}
#[doc = "LTC PKHA B 16 Register"]
pub mod ltc0_pkb_16;
#[doc = "LTC PKHA B1 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_1](ltc0_pkb1_1) module"]
pub type LTC0_PKB1_1 = crate::Reg<u32, _LTC0_PKB1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_1;
#[doc = "`read()` method returns [ltc0_pkb1_1::R](ltc0_pkb1_1::R) reader structure"]
impl crate::Readable for LTC0_PKB1_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_1::W](ltc0_pkb1_1::W) writer structure"]
impl crate::Writable for LTC0_PKB1_1 {}
#[doc = "LTC PKHA B1 1 Register"]
pub mod ltc0_pkb1_1;
#[doc = "LTC PKHA B 17 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_17](ltc0_pkb_17) module"]
pub type LTC0_PKB_17 = crate::Reg<u32, _LTC0_PKB_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_17;
#[doc = "`read()` method returns [ltc0_pkb_17::R](ltc0_pkb_17::R) reader structure"]
impl crate::Readable for LTC0_PKB_17 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_17::W](ltc0_pkb_17::W) writer structure"]
impl crate::Writable for LTC0_PKB_17 {}
#[doc = "LTC PKHA B 17 Register"]
pub mod ltc0_pkb_17;
#[doc = "LTC PKHA B1 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_2](ltc0_pkb1_2) module"]
pub type LTC0_PKB1_2 = crate::Reg<u32, _LTC0_PKB1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_2;
#[doc = "`read()` method returns [ltc0_pkb1_2::R](ltc0_pkb1_2::R) reader structure"]
impl crate::Readable for LTC0_PKB1_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_2::W](ltc0_pkb1_2::W) writer structure"]
impl crate::Writable for LTC0_PKB1_2 {}
#[doc = "LTC PKHA B1 2 Register"]
pub mod ltc0_pkb1_2;
#[doc = "LTC PKHA B 18 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_18](ltc0_pkb_18) module"]
pub type LTC0_PKB_18 = crate::Reg<u32, _LTC0_PKB_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_18;
#[doc = "`read()` method returns [ltc0_pkb_18::R](ltc0_pkb_18::R) reader structure"]
impl crate::Readable for LTC0_PKB_18 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_18::W](ltc0_pkb_18::W) writer structure"]
impl crate::Writable for LTC0_PKB_18 {}
#[doc = "LTC PKHA B 18 Register"]
pub mod ltc0_pkb_18;
#[doc = "LTC PKHA B1 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_3](ltc0_pkb1_3) module"]
pub type LTC0_PKB1_3 = crate::Reg<u32, _LTC0_PKB1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_3;
#[doc = "`read()` method returns [ltc0_pkb1_3::R](ltc0_pkb1_3::R) reader structure"]
impl crate::Readable for LTC0_PKB1_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_3::W](ltc0_pkb1_3::W) writer structure"]
impl crate::Writable for LTC0_PKB1_3 {}
#[doc = "LTC PKHA B1 3 Register"]
pub mod ltc0_pkb1_3;
#[doc = "LTC PKHA B 19 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_19](ltc0_pkb_19) module"]
pub type LTC0_PKB_19 = crate::Reg<u32, _LTC0_PKB_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_19;
#[doc = "`read()` method returns [ltc0_pkb_19::R](ltc0_pkb_19::R) reader structure"]
impl crate::Readable for LTC0_PKB_19 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_19::W](ltc0_pkb_19::W) writer structure"]
impl crate::Writable for LTC0_PKB_19 {}
#[doc = "LTC PKHA B 19 Register"]
pub mod ltc0_pkb_19;
#[doc = "LTC PKHA B1 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_4](ltc0_pkb1_4) module"]
pub type LTC0_PKB1_4 = crate::Reg<u32, _LTC0_PKB1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_4;
#[doc = "`read()` method returns [ltc0_pkb1_4::R](ltc0_pkb1_4::R) reader structure"]
impl crate::Readable for LTC0_PKB1_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_4::W](ltc0_pkb1_4::W) writer structure"]
impl crate::Writable for LTC0_PKB1_4 {}
#[doc = "LTC PKHA B1 4 Register"]
pub mod ltc0_pkb1_4;
#[doc = "LTC PKHA B 20 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_20](ltc0_pkb_20) module"]
pub type LTC0_PKB_20 = crate::Reg<u32, _LTC0_PKB_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_20;
#[doc = "`read()` method returns [ltc0_pkb_20::R](ltc0_pkb_20::R) reader structure"]
impl crate::Readable for LTC0_PKB_20 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_20::W](ltc0_pkb_20::W) writer structure"]
impl crate::Writable for LTC0_PKB_20 {}
#[doc = "LTC PKHA B 20 Register"]
pub mod ltc0_pkb_20;
#[doc = "LTC PKHA B1 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_5](ltc0_pkb1_5) module"]
pub type LTC0_PKB1_5 = crate::Reg<u32, _LTC0_PKB1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_5;
#[doc = "`read()` method returns [ltc0_pkb1_5::R](ltc0_pkb1_5::R) reader structure"]
impl crate::Readable for LTC0_PKB1_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_5::W](ltc0_pkb1_5::W) writer structure"]
impl crate::Writable for LTC0_PKB1_5 {}
#[doc = "LTC PKHA B1 5 Register"]
pub mod ltc0_pkb1_5;
#[doc = "LTC PKHA B 21 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_21](ltc0_pkb_21) module"]
pub type LTC0_PKB_21 = crate::Reg<u32, _LTC0_PKB_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_21;
#[doc = "`read()` method returns [ltc0_pkb_21::R](ltc0_pkb_21::R) reader structure"]
impl crate::Readable for LTC0_PKB_21 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_21::W](ltc0_pkb_21::W) writer structure"]
impl crate::Writable for LTC0_PKB_21 {}
#[doc = "LTC PKHA B 21 Register"]
pub mod ltc0_pkb_21;
#[doc = "LTC PKHA B1 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_6](ltc0_pkb1_6) module"]
pub type LTC0_PKB1_6 = crate::Reg<u32, _LTC0_PKB1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_6;
#[doc = "`read()` method returns [ltc0_pkb1_6::R](ltc0_pkb1_6::R) reader structure"]
impl crate::Readable for LTC0_PKB1_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_6::W](ltc0_pkb1_6::W) writer structure"]
impl crate::Writable for LTC0_PKB1_6 {}
#[doc = "LTC PKHA B1 6 Register"]
pub mod ltc0_pkb1_6;
#[doc = "LTC PKHA B 22 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_22](ltc0_pkb_22) module"]
pub type LTC0_PKB_22 = crate::Reg<u32, _LTC0_PKB_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_22;
#[doc = "`read()` method returns [ltc0_pkb_22::R](ltc0_pkb_22::R) reader structure"]
impl crate::Readable for LTC0_PKB_22 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_22::W](ltc0_pkb_22::W) writer structure"]
impl crate::Writable for LTC0_PKB_22 {}
#[doc = "LTC PKHA B 22 Register"]
pub mod ltc0_pkb_22;
#[doc = "LTC PKHA B1 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_7](ltc0_pkb1_7) module"]
pub type LTC0_PKB1_7 = crate::Reg<u32, _LTC0_PKB1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_7;
#[doc = "`read()` method returns [ltc0_pkb1_7::R](ltc0_pkb1_7::R) reader structure"]
impl crate::Readable for LTC0_PKB1_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_7::W](ltc0_pkb1_7::W) writer structure"]
impl crate::Writable for LTC0_PKB1_7 {}
#[doc = "LTC PKHA B1 7 Register"]
pub mod ltc0_pkb1_7;
#[doc = "LTC PKHA B 23 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_23](ltc0_pkb_23) module"]
pub type LTC0_PKB_23 = crate::Reg<u32, _LTC0_PKB_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_23;
#[doc = "`read()` method returns [ltc0_pkb_23::R](ltc0_pkb_23::R) reader structure"]
impl crate::Readable for LTC0_PKB_23 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_23::W](ltc0_pkb_23::W) writer structure"]
impl crate::Writable for LTC0_PKB_23 {}
#[doc = "LTC PKHA B 23 Register"]
pub mod ltc0_pkb_23;
#[doc = "LTC PKHA B1 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_8](ltc0_pkb1_8) module"]
pub type LTC0_PKB1_8 = crate::Reg<u32, _LTC0_PKB1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_8;
#[doc = "`read()` method returns [ltc0_pkb1_8::R](ltc0_pkb1_8::R) reader structure"]
impl crate::Readable for LTC0_PKB1_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_8::W](ltc0_pkb1_8::W) writer structure"]
impl crate::Writable for LTC0_PKB1_8 {}
#[doc = "LTC PKHA B1 8 Register"]
pub mod ltc0_pkb1_8;
#[doc = "LTC PKHA B 24 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_24](ltc0_pkb_24) module"]
pub type LTC0_PKB_24 = crate::Reg<u32, _LTC0_PKB_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_24;
#[doc = "`read()` method returns [ltc0_pkb_24::R](ltc0_pkb_24::R) reader structure"]
impl crate::Readable for LTC0_PKB_24 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_24::W](ltc0_pkb_24::W) writer structure"]
impl crate::Writable for LTC0_PKB_24 {}
#[doc = "LTC PKHA B 24 Register"]
pub mod ltc0_pkb_24;
#[doc = "LTC PKHA B1 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_9](ltc0_pkb1_9) module"]
pub type LTC0_PKB1_9 = crate::Reg<u32, _LTC0_PKB1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_9;
#[doc = "`read()` method returns [ltc0_pkb1_9::R](ltc0_pkb1_9::R) reader structure"]
impl crate::Readable for LTC0_PKB1_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_9::W](ltc0_pkb1_9::W) writer structure"]
impl crate::Writable for LTC0_PKB1_9 {}
#[doc = "LTC PKHA B1 9 Register"]
pub mod ltc0_pkb1_9;
#[doc = "LTC PKHA B 25 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_25](ltc0_pkb_25) module"]
pub type LTC0_PKB_25 = crate::Reg<u32, _LTC0_PKB_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_25;
#[doc = "`read()` method returns [ltc0_pkb_25::R](ltc0_pkb_25::R) reader structure"]
impl crate::Readable for LTC0_PKB_25 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_25::W](ltc0_pkb_25::W) writer structure"]
impl crate::Writable for LTC0_PKB_25 {}
#[doc = "LTC PKHA B 25 Register"]
pub mod ltc0_pkb_25;
#[doc = "LTC PKHA B1 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_10](ltc0_pkb1_10) module"]
pub type LTC0_PKB1_10 = crate::Reg<u32, _LTC0_PKB1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_10;
#[doc = "`read()` method returns [ltc0_pkb1_10::R](ltc0_pkb1_10::R) reader structure"]
impl crate::Readable for LTC0_PKB1_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_10::W](ltc0_pkb1_10::W) writer structure"]
impl crate::Writable for LTC0_PKB1_10 {}
#[doc = "LTC PKHA B1 10 Register"]
pub mod ltc0_pkb1_10;
#[doc = "LTC PKHA B 26 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_26](ltc0_pkb_26) module"]
pub type LTC0_PKB_26 = crate::Reg<u32, _LTC0_PKB_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_26;
#[doc = "`read()` method returns [ltc0_pkb_26::R](ltc0_pkb_26::R) reader structure"]
impl crate::Readable for LTC0_PKB_26 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_26::W](ltc0_pkb_26::W) writer structure"]
impl crate::Writable for LTC0_PKB_26 {}
#[doc = "LTC PKHA B 26 Register"]
pub mod ltc0_pkb_26;
#[doc = "LTC PKHA B1 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_11](ltc0_pkb1_11) module"]
pub type LTC0_PKB1_11 = crate::Reg<u32, _LTC0_PKB1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_11;
#[doc = "`read()` method returns [ltc0_pkb1_11::R](ltc0_pkb1_11::R) reader structure"]
impl crate::Readable for LTC0_PKB1_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_11::W](ltc0_pkb1_11::W) writer structure"]
impl crate::Writable for LTC0_PKB1_11 {}
#[doc = "LTC PKHA B1 11 Register"]
pub mod ltc0_pkb1_11;
#[doc = "LTC PKHA B 27 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_27](ltc0_pkb_27) module"]
pub type LTC0_PKB_27 = crate::Reg<u32, _LTC0_PKB_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_27;
#[doc = "`read()` method returns [ltc0_pkb_27::R](ltc0_pkb_27::R) reader structure"]
impl crate::Readable for LTC0_PKB_27 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_27::W](ltc0_pkb_27::W) writer structure"]
impl crate::Writable for LTC0_PKB_27 {}
#[doc = "LTC PKHA B 27 Register"]
pub mod ltc0_pkb_27;
#[doc = "LTC PKHA B1 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_12](ltc0_pkb1_12) module"]
pub type LTC0_PKB1_12 = crate::Reg<u32, _LTC0_PKB1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_12;
#[doc = "`read()` method returns [ltc0_pkb1_12::R](ltc0_pkb1_12::R) reader structure"]
impl crate::Readable for LTC0_PKB1_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_12::W](ltc0_pkb1_12::W) writer structure"]
impl crate::Writable for LTC0_PKB1_12 {}
#[doc = "LTC PKHA B1 12 Register"]
pub mod ltc0_pkb1_12;
#[doc = "LTC PKHA B 28 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_28](ltc0_pkb_28) module"]
pub type LTC0_PKB_28 = crate::Reg<u32, _LTC0_PKB_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_28;
#[doc = "`read()` method returns [ltc0_pkb_28::R](ltc0_pkb_28::R) reader structure"]
impl crate::Readable for LTC0_PKB_28 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_28::W](ltc0_pkb_28::W) writer structure"]
impl crate::Writable for LTC0_PKB_28 {}
#[doc = "LTC PKHA B 28 Register"]
pub mod ltc0_pkb_28;
#[doc = "LTC PKHA B1 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_13](ltc0_pkb1_13) module"]
pub type LTC0_PKB1_13 = crate::Reg<u32, _LTC0_PKB1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_13;
#[doc = "`read()` method returns [ltc0_pkb1_13::R](ltc0_pkb1_13::R) reader structure"]
impl crate::Readable for LTC0_PKB1_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_13::W](ltc0_pkb1_13::W) writer structure"]
impl crate::Writable for LTC0_PKB1_13 {}
#[doc = "LTC PKHA B1 13 Register"]
pub mod ltc0_pkb1_13;
#[doc = "LTC PKHA B 29 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_29](ltc0_pkb_29) module"]
pub type LTC0_PKB_29 = crate::Reg<u32, _LTC0_PKB_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_29;
#[doc = "`read()` method returns [ltc0_pkb_29::R](ltc0_pkb_29::R) reader structure"]
impl crate::Readable for LTC0_PKB_29 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_29::W](ltc0_pkb_29::W) writer structure"]
impl crate::Writable for LTC0_PKB_29 {}
#[doc = "LTC PKHA B 29 Register"]
pub mod ltc0_pkb_29;
#[doc = "LTC PKHA B1 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_14](ltc0_pkb1_14) module"]
pub type LTC0_PKB1_14 = crate::Reg<u32, _LTC0_PKB1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_14;
#[doc = "`read()` method returns [ltc0_pkb1_14::R](ltc0_pkb1_14::R) reader structure"]
impl crate::Readable for LTC0_PKB1_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_14::W](ltc0_pkb1_14::W) writer structure"]
impl crate::Writable for LTC0_PKB1_14 {}
#[doc = "LTC PKHA B1 14 Register"]
pub mod ltc0_pkb1_14;
#[doc = "LTC PKHA B 30 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_30](ltc0_pkb_30) module"]
pub type LTC0_PKB_30 = crate::Reg<u32, _LTC0_PKB_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_30;
#[doc = "`read()` method returns [ltc0_pkb_30::R](ltc0_pkb_30::R) reader structure"]
impl crate::Readable for LTC0_PKB_30 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_30::W](ltc0_pkb_30::W) writer structure"]
impl crate::Writable for LTC0_PKB_30 {}
#[doc = "LTC PKHA B 30 Register"]
pub mod ltc0_pkb_30;
#[doc = "LTC PKHA B1 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb1_15](ltc0_pkb1_15) module"]
pub type LTC0_PKB1_15 = crate::Reg<u32, _LTC0_PKB1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB1_15;
#[doc = "`read()` method returns [ltc0_pkb1_15::R](ltc0_pkb1_15::R) reader structure"]
impl crate::Readable for LTC0_PKB1_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb1_15::W](ltc0_pkb1_15::W) writer structure"]
impl crate::Writable for LTC0_PKB1_15 {}
#[doc = "LTC PKHA B1 15 Register"]
pub mod ltc0_pkb1_15;
#[doc = "LTC PKHA B 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_31](ltc0_pkb_31) module"]
pub type LTC0_PKB_31 = crate::Reg<u32, _LTC0_PKB_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_31;
#[doc = "`read()` method returns [ltc0_pkb_31::R](ltc0_pkb_31::R) reader structure"]
impl crate::Readable for LTC0_PKB_31 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_31::W](ltc0_pkb_31::W) writer structure"]
impl crate::Writable for LTC0_PKB_31 {}
#[doc = "LTC PKHA B 31 Register"]
pub mod ltc0_pkb_31;
#[doc = "LTC PKHA B2 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_0](ltc0_pkb2_0) module"]
pub type LTC0_PKB2_0 = crate::Reg<u32, _LTC0_PKB2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_0;
#[doc = "`read()` method returns [ltc0_pkb2_0::R](ltc0_pkb2_0::R) reader structure"]
impl crate::Readable for LTC0_PKB2_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_0::W](ltc0_pkb2_0::W) writer structure"]
impl crate::Writable for LTC0_PKB2_0 {}
#[doc = "LTC PKHA B2 0 Register"]
pub mod ltc0_pkb2_0;
#[doc = "LTC PKHA B 32 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_32](ltc0_pkb_32) module"]
pub type LTC0_PKB_32 = crate::Reg<u32, _LTC0_PKB_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_32;
#[doc = "`read()` method returns [ltc0_pkb_32::R](ltc0_pkb_32::R) reader structure"]
impl crate::Readable for LTC0_PKB_32 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_32::W](ltc0_pkb_32::W) writer structure"]
impl crate::Writable for LTC0_PKB_32 {}
#[doc = "LTC PKHA B 32 Register"]
pub mod ltc0_pkb_32;
#[doc = "LTC PKHA B2 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_1](ltc0_pkb2_1) module"]
pub type LTC0_PKB2_1 = crate::Reg<u32, _LTC0_PKB2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_1;
#[doc = "`read()` method returns [ltc0_pkb2_1::R](ltc0_pkb2_1::R) reader structure"]
impl crate::Readable for LTC0_PKB2_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_1::W](ltc0_pkb2_1::W) writer structure"]
impl crate::Writable for LTC0_PKB2_1 {}
#[doc = "LTC PKHA B2 1 Register"]
pub mod ltc0_pkb2_1;
#[doc = "LTC PKHA B 33 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_33](ltc0_pkb_33) module"]
pub type LTC0_PKB_33 = crate::Reg<u32, _LTC0_PKB_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_33;
#[doc = "`read()` method returns [ltc0_pkb_33::R](ltc0_pkb_33::R) reader structure"]
impl crate::Readable for LTC0_PKB_33 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_33::W](ltc0_pkb_33::W) writer structure"]
impl crate::Writable for LTC0_PKB_33 {}
#[doc = "LTC PKHA B 33 Register"]
pub mod ltc0_pkb_33;
#[doc = "LTC PKHA B2 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_2](ltc0_pkb2_2) module"]
pub type LTC0_PKB2_2 = crate::Reg<u32, _LTC0_PKB2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_2;
#[doc = "`read()` method returns [ltc0_pkb2_2::R](ltc0_pkb2_2::R) reader structure"]
impl crate::Readable for LTC0_PKB2_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_2::W](ltc0_pkb2_2::W) writer structure"]
impl crate::Writable for LTC0_PKB2_2 {}
#[doc = "LTC PKHA B2 2 Register"]
pub mod ltc0_pkb2_2;
#[doc = "LTC PKHA B 34 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_34](ltc0_pkb_34) module"]
pub type LTC0_PKB_34 = crate::Reg<u32, _LTC0_PKB_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_34;
#[doc = "`read()` method returns [ltc0_pkb_34::R](ltc0_pkb_34::R) reader structure"]
impl crate::Readable for LTC0_PKB_34 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_34::W](ltc0_pkb_34::W) writer structure"]
impl crate::Writable for LTC0_PKB_34 {}
#[doc = "LTC PKHA B 34 Register"]
pub mod ltc0_pkb_34;
#[doc = "LTC PKHA B2 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_3](ltc0_pkb2_3) module"]
pub type LTC0_PKB2_3 = crate::Reg<u32, _LTC0_PKB2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_3;
#[doc = "`read()` method returns [ltc0_pkb2_3::R](ltc0_pkb2_3::R) reader structure"]
impl crate::Readable for LTC0_PKB2_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_3::W](ltc0_pkb2_3::W) writer structure"]
impl crate::Writable for LTC0_PKB2_3 {}
#[doc = "LTC PKHA B2 3 Register"]
pub mod ltc0_pkb2_3;
#[doc = "LTC PKHA B 35 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_35](ltc0_pkb_35) module"]
pub type LTC0_PKB_35 = crate::Reg<u32, _LTC0_PKB_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_35;
#[doc = "`read()` method returns [ltc0_pkb_35::R](ltc0_pkb_35::R) reader structure"]
impl crate::Readable for LTC0_PKB_35 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_35::W](ltc0_pkb_35::W) writer structure"]
impl crate::Writable for LTC0_PKB_35 {}
#[doc = "LTC PKHA B 35 Register"]
pub mod ltc0_pkb_35;
#[doc = "LTC PKHA B2 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_4](ltc0_pkb2_4) module"]
pub type LTC0_PKB2_4 = crate::Reg<u32, _LTC0_PKB2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_4;
#[doc = "`read()` method returns [ltc0_pkb2_4::R](ltc0_pkb2_4::R) reader structure"]
impl crate::Readable for LTC0_PKB2_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_4::W](ltc0_pkb2_4::W) writer structure"]
impl crate::Writable for LTC0_PKB2_4 {}
#[doc = "LTC PKHA B2 4 Register"]
pub mod ltc0_pkb2_4;
#[doc = "LTC PKHA B 36 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_36](ltc0_pkb_36) module"]
pub type LTC0_PKB_36 = crate::Reg<u32, _LTC0_PKB_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_36;
#[doc = "`read()` method returns [ltc0_pkb_36::R](ltc0_pkb_36::R) reader structure"]
impl crate::Readable for LTC0_PKB_36 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_36::W](ltc0_pkb_36::W) writer structure"]
impl crate::Writable for LTC0_PKB_36 {}
#[doc = "LTC PKHA B 36 Register"]
pub mod ltc0_pkb_36;
#[doc = "LTC PKHA B2 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_5](ltc0_pkb2_5) module"]
pub type LTC0_PKB2_5 = crate::Reg<u32, _LTC0_PKB2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_5;
#[doc = "`read()` method returns [ltc0_pkb2_5::R](ltc0_pkb2_5::R) reader structure"]
impl crate::Readable for LTC0_PKB2_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_5::W](ltc0_pkb2_5::W) writer structure"]
impl crate::Writable for LTC0_PKB2_5 {}
#[doc = "LTC PKHA B2 5 Register"]
pub mod ltc0_pkb2_5;
#[doc = "LTC PKHA B 37 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_37](ltc0_pkb_37) module"]
pub type LTC0_PKB_37 = crate::Reg<u32, _LTC0_PKB_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_37;
#[doc = "`read()` method returns [ltc0_pkb_37::R](ltc0_pkb_37::R) reader structure"]
impl crate::Readable for LTC0_PKB_37 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_37::W](ltc0_pkb_37::W) writer structure"]
impl crate::Writable for LTC0_PKB_37 {}
#[doc = "LTC PKHA B 37 Register"]
pub mod ltc0_pkb_37;
#[doc = "LTC PKHA B2 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_6](ltc0_pkb2_6) module"]
pub type LTC0_PKB2_6 = crate::Reg<u32, _LTC0_PKB2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_6;
#[doc = "`read()` method returns [ltc0_pkb2_6::R](ltc0_pkb2_6::R) reader structure"]
impl crate::Readable for LTC0_PKB2_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_6::W](ltc0_pkb2_6::W) writer structure"]
impl crate::Writable for LTC0_PKB2_6 {}
#[doc = "LTC PKHA B2 6 Register"]
pub mod ltc0_pkb2_6;
#[doc = "LTC PKHA B 38 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_38](ltc0_pkb_38) module"]
pub type LTC0_PKB_38 = crate::Reg<u32, _LTC0_PKB_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_38;
#[doc = "`read()` method returns [ltc0_pkb_38::R](ltc0_pkb_38::R) reader structure"]
impl crate::Readable for LTC0_PKB_38 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_38::W](ltc0_pkb_38::W) writer structure"]
impl crate::Writable for LTC0_PKB_38 {}
#[doc = "LTC PKHA B 38 Register"]
pub mod ltc0_pkb_38;
#[doc = "LTC PKHA B2 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_7](ltc0_pkb2_7) module"]
pub type LTC0_PKB2_7 = crate::Reg<u32, _LTC0_PKB2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_7;
#[doc = "`read()` method returns [ltc0_pkb2_7::R](ltc0_pkb2_7::R) reader structure"]
impl crate::Readable for LTC0_PKB2_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_7::W](ltc0_pkb2_7::W) writer structure"]
impl crate::Writable for LTC0_PKB2_7 {}
#[doc = "LTC PKHA B2 7 Register"]
pub mod ltc0_pkb2_7;
#[doc = "LTC PKHA B 39 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_39](ltc0_pkb_39) module"]
pub type LTC0_PKB_39 = crate::Reg<u32, _LTC0_PKB_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_39;
#[doc = "`read()` method returns [ltc0_pkb_39::R](ltc0_pkb_39::R) reader structure"]
impl crate::Readable for LTC0_PKB_39 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_39::W](ltc0_pkb_39::W) writer structure"]
impl crate::Writable for LTC0_PKB_39 {}
#[doc = "LTC PKHA B 39 Register"]
pub mod ltc0_pkb_39;
#[doc = "LTC PKHA B2 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_8](ltc0_pkb2_8) module"]
pub type LTC0_PKB2_8 = crate::Reg<u32, _LTC0_PKB2_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_8;
#[doc = "`read()` method returns [ltc0_pkb2_8::R](ltc0_pkb2_8::R) reader structure"]
impl crate::Readable for LTC0_PKB2_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_8::W](ltc0_pkb2_8::W) writer structure"]
impl crate::Writable for LTC0_PKB2_8 {}
#[doc = "LTC PKHA B2 8 Register"]
pub mod ltc0_pkb2_8;
#[doc = "LTC PKHA B 40 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_40](ltc0_pkb_40) module"]
pub type LTC0_PKB_40 = crate::Reg<u32, _LTC0_PKB_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_40;
#[doc = "`read()` method returns [ltc0_pkb_40::R](ltc0_pkb_40::R) reader structure"]
impl crate::Readable for LTC0_PKB_40 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_40::W](ltc0_pkb_40::W) writer structure"]
impl crate::Writable for LTC0_PKB_40 {}
#[doc = "LTC PKHA B 40 Register"]
pub mod ltc0_pkb_40;
#[doc = "LTC PKHA B2 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_9](ltc0_pkb2_9) module"]
pub type LTC0_PKB2_9 = crate::Reg<u32, _LTC0_PKB2_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_9;
#[doc = "`read()` method returns [ltc0_pkb2_9::R](ltc0_pkb2_9::R) reader structure"]
impl crate::Readable for LTC0_PKB2_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_9::W](ltc0_pkb2_9::W) writer structure"]
impl crate::Writable for LTC0_PKB2_9 {}
#[doc = "LTC PKHA B2 9 Register"]
pub mod ltc0_pkb2_9;
#[doc = "LTC PKHA B 41 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_41](ltc0_pkb_41) module"]
pub type LTC0_PKB_41 = crate::Reg<u32, _LTC0_PKB_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_41;
#[doc = "`read()` method returns [ltc0_pkb_41::R](ltc0_pkb_41::R) reader structure"]
impl crate::Readable for LTC0_PKB_41 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_41::W](ltc0_pkb_41::W) writer structure"]
impl crate::Writable for LTC0_PKB_41 {}
#[doc = "LTC PKHA B 41 Register"]
pub mod ltc0_pkb_41;
#[doc = "LTC PKHA B2 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_10](ltc0_pkb2_10) module"]
pub type LTC0_PKB2_10 = crate::Reg<u32, _LTC0_PKB2_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_10;
#[doc = "`read()` method returns [ltc0_pkb2_10::R](ltc0_pkb2_10::R) reader structure"]
impl crate::Readable for LTC0_PKB2_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_10::W](ltc0_pkb2_10::W) writer structure"]
impl crate::Writable for LTC0_PKB2_10 {}
#[doc = "LTC PKHA B2 10 Register"]
pub mod ltc0_pkb2_10;
#[doc = "LTC PKHA B 42 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_42](ltc0_pkb_42) module"]
pub type LTC0_PKB_42 = crate::Reg<u32, _LTC0_PKB_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_42;
#[doc = "`read()` method returns [ltc0_pkb_42::R](ltc0_pkb_42::R) reader structure"]
impl crate::Readable for LTC0_PKB_42 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_42::W](ltc0_pkb_42::W) writer structure"]
impl crate::Writable for LTC0_PKB_42 {}
#[doc = "LTC PKHA B 42 Register"]
pub mod ltc0_pkb_42;
#[doc = "LTC PKHA B2 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_11](ltc0_pkb2_11) module"]
pub type LTC0_PKB2_11 = crate::Reg<u32, _LTC0_PKB2_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_11;
#[doc = "`read()` method returns [ltc0_pkb2_11::R](ltc0_pkb2_11::R) reader structure"]
impl crate::Readable for LTC0_PKB2_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_11::W](ltc0_pkb2_11::W) writer structure"]
impl crate::Writable for LTC0_PKB2_11 {}
#[doc = "LTC PKHA B2 11 Register"]
pub mod ltc0_pkb2_11;
#[doc = "LTC PKHA B 43 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_43](ltc0_pkb_43) module"]
pub type LTC0_PKB_43 = crate::Reg<u32, _LTC0_PKB_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_43;
#[doc = "`read()` method returns [ltc0_pkb_43::R](ltc0_pkb_43::R) reader structure"]
impl crate::Readable for LTC0_PKB_43 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_43::W](ltc0_pkb_43::W) writer structure"]
impl crate::Writable for LTC0_PKB_43 {}
#[doc = "LTC PKHA B 43 Register"]
pub mod ltc0_pkb_43;
#[doc = "LTC PKHA B2 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_12](ltc0_pkb2_12) module"]
pub type LTC0_PKB2_12 = crate::Reg<u32, _LTC0_PKB2_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_12;
#[doc = "`read()` method returns [ltc0_pkb2_12::R](ltc0_pkb2_12::R) reader structure"]
impl crate::Readable for LTC0_PKB2_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_12::W](ltc0_pkb2_12::W) writer structure"]
impl crate::Writable for LTC0_PKB2_12 {}
#[doc = "LTC PKHA B2 12 Register"]
pub mod ltc0_pkb2_12;
#[doc = "LTC PKHA B 44 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_44](ltc0_pkb_44) module"]
pub type LTC0_PKB_44 = crate::Reg<u32, _LTC0_PKB_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_44;
#[doc = "`read()` method returns [ltc0_pkb_44::R](ltc0_pkb_44::R) reader structure"]
impl crate::Readable for LTC0_PKB_44 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_44::W](ltc0_pkb_44::W) writer structure"]
impl crate::Writable for LTC0_PKB_44 {}
#[doc = "LTC PKHA B 44 Register"]
pub mod ltc0_pkb_44;
#[doc = "LTC PKHA B2 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_13](ltc0_pkb2_13) module"]
pub type LTC0_PKB2_13 = crate::Reg<u32, _LTC0_PKB2_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_13;
#[doc = "`read()` method returns [ltc0_pkb2_13::R](ltc0_pkb2_13::R) reader structure"]
impl crate::Readable for LTC0_PKB2_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_13::W](ltc0_pkb2_13::W) writer structure"]
impl crate::Writable for LTC0_PKB2_13 {}
#[doc = "LTC PKHA B2 13 Register"]
pub mod ltc0_pkb2_13;
#[doc = "LTC PKHA B 45 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_45](ltc0_pkb_45) module"]
pub type LTC0_PKB_45 = crate::Reg<u32, _LTC0_PKB_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_45;
#[doc = "`read()` method returns [ltc0_pkb_45::R](ltc0_pkb_45::R) reader structure"]
impl crate::Readable for LTC0_PKB_45 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_45::W](ltc0_pkb_45::W) writer structure"]
impl crate::Writable for LTC0_PKB_45 {}
#[doc = "LTC PKHA B 45 Register"]
pub mod ltc0_pkb_45;
#[doc = "LTC PKHA B2 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_14](ltc0_pkb2_14) module"]
pub type LTC0_PKB2_14 = crate::Reg<u32, _LTC0_PKB2_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_14;
#[doc = "`read()` method returns [ltc0_pkb2_14::R](ltc0_pkb2_14::R) reader structure"]
impl crate::Readable for LTC0_PKB2_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_14::W](ltc0_pkb2_14::W) writer structure"]
impl crate::Writable for LTC0_PKB2_14 {}
#[doc = "LTC PKHA B2 14 Register"]
pub mod ltc0_pkb2_14;
#[doc = "LTC PKHA B 46 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_46](ltc0_pkb_46) module"]
pub type LTC0_PKB_46 = crate::Reg<u32, _LTC0_PKB_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_46;
#[doc = "`read()` method returns [ltc0_pkb_46::R](ltc0_pkb_46::R) reader structure"]
impl crate::Readable for LTC0_PKB_46 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_46::W](ltc0_pkb_46::W) writer structure"]
impl crate::Writable for LTC0_PKB_46 {}
#[doc = "LTC PKHA B 46 Register"]
pub mod ltc0_pkb_46;
#[doc = "LTC PKHA B2 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb2_15](ltc0_pkb2_15) module"]
pub type LTC0_PKB2_15 = crate::Reg<u32, _LTC0_PKB2_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB2_15;
#[doc = "`read()` method returns [ltc0_pkb2_15::R](ltc0_pkb2_15::R) reader structure"]
impl crate::Readable for LTC0_PKB2_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb2_15::W](ltc0_pkb2_15::W) writer structure"]
impl crate::Writable for LTC0_PKB2_15 {}
#[doc = "LTC PKHA B2 15 Register"]
pub mod ltc0_pkb2_15;
#[doc = "LTC PKHA B 47 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_47](ltc0_pkb_47) module"]
pub type LTC0_PKB_47 = crate::Reg<u32, _LTC0_PKB_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_47;
#[doc = "`read()` method returns [ltc0_pkb_47::R](ltc0_pkb_47::R) reader structure"]
impl crate::Readable for LTC0_PKB_47 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_47::W](ltc0_pkb_47::W) writer structure"]
impl crate::Writable for LTC0_PKB_47 {}
#[doc = "LTC PKHA B 47 Register"]
pub mod ltc0_pkb_47;
#[doc = "LTC PKHA B3 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_0](ltc0_pkb3_0) module"]
pub type LTC0_PKB3_0 = crate::Reg<u32, _LTC0_PKB3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_0;
#[doc = "`read()` method returns [ltc0_pkb3_0::R](ltc0_pkb3_0::R) reader structure"]
impl crate::Readable for LTC0_PKB3_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_0::W](ltc0_pkb3_0::W) writer structure"]
impl crate::Writable for LTC0_PKB3_0 {}
#[doc = "LTC PKHA B3 0 Register"]
pub mod ltc0_pkb3_0;
#[doc = "LTC PKHA B 48 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_48](ltc0_pkb_48) module"]
pub type LTC0_PKB_48 = crate::Reg<u32, _LTC0_PKB_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_48;
#[doc = "`read()` method returns [ltc0_pkb_48::R](ltc0_pkb_48::R) reader structure"]
impl crate::Readable for LTC0_PKB_48 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_48::W](ltc0_pkb_48::W) writer structure"]
impl crate::Writable for LTC0_PKB_48 {}
#[doc = "LTC PKHA B 48 Register"]
pub mod ltc0_pkb_48;
#[doc = "LTC PKHA B3 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_1](ltc0_pkb3_1) module"]
pub type LTC0_PKB3_1 = crate::Reg<u32, _LTC0_PKB3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_1;
#[doc = "`read()` method returns [ltc0_pkb3_1::R](ltc0_pkb3_1::R) reader structure"]
impl crate::Readable for LTC0_PKB3_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_1::W](ltc0_pkb3_1::W) writer structure"]
impl crate::Writable for LTC0_PKB3_1 {}
#[doc = "LTC PKHA B3 1 Register"]
pub mod ltc0_pkb3_1;
#[doc = "LTC PKHA B 49 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_49](ltc0_pkb_49) module"]
pub type LTC0_PKB_49 = crate::Reg<u32, _LTC0_PKB_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_49;
#[doc = "`read()` method returns [ltc0_pkb_49::R](ltc0_pkb_49::R) reader structure"]
impl crate::Readable for LTC0_PKB_49 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_49::W](ltc0_pkb_49::W) writer structure"]
impl crate::Writable for LTC0_PKB_49 {}
#[doc = "LTC PKHA B 49 Register"]
pub mod ltc0_pkb_49;
#[doc = "LTC PKHA B3 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_2](ltc0_pkb3_2) module"]
pub type LTC0_PKB3_2 = crate::Reg<u32, _LTC0_PKB3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_2;
#[doc = "`read()` method returns [ltc0_pkb3_2::R](ltc0_pkb3_2::R) reader structure"]
impl crate::Readable for LTC0_PKB3_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_2::W](ltc0_pkb3_2::W) writer structure"]
impl crate::Writable for LTC0_PKB3_2 {}
#[doc = "LTC PKHA B3 2 Register"]
pub mod ltc0_pkb3_2;
#[doc = "LTC PKHA B 50 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_50](ltc0_pkb_50) module"]
pub type LTC0_PKB_50 = crate::Reg<u32, _LTC0_PKB_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_50;
#[doc = "`read()` method returns [ltc0_pkb_50::R](ltc0_pkb_50::R) reader structure"]
impl crate::Readable for LTC0_PKB_50 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_50::W](ltc0_pkb_50::W) writer structure"]
impl crate::Writable for LTC0_PKB_50 {}
#[doc = "LTC PKHA B 50 Register"]
pub mod ltc0_pkb_50;
#[doc = "LTC PKHA B3 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_3](ltc0_pkb3_3) module"]
pub type LTC0_PKB3_3 = crate::Reg<u32, _LTC0_PKB3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_3;
#[doc = "`read()` method returns [ltc0_pkb3_3::R](ltc0_pkb3_3::R) reader structure"]
impl crate::Readable for LTC0_PKB3_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_3::W](ltc0_pkb3_3::W) writer structure"]
impl crate::Writable for LTC0_PKB3_3 {}
#[doc = "LTC PKHA B3 3 Register"]
pub mod ltc0_pkb3_3;
#[doc = "LTC PKHA B 51 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_51](ltc0_pkb_51) module"]
pub type LTC0_PKB_51 = crate::Reg<u32, _LTC0_PKB_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_51;
#[doc = "`read()` method returns [ltc0_pkb_51::R](ltc0_pkb_51::R) reader structure"]
impl crate::Readable for LTC0_PKB_51 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_51::W](ltc0_pkb_51::W) writer structure"]
impl crate::Writable for LTC0_PKB_51 {}
#[doc = "LTC PKHA B 51 Register"]
pub mod ltc0_pkb_51;
#[doc = "LTC PKHA B3 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_4](ltc0_pkb3_4) module"]
pub type LTC0_PKB3_4 = crate::Reg<u32, _LTC0_PKB3_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_4;
#[doc = "`read()` method returns [ltc0_pkb3_4::R](ltc0_pkb3_4::R) reader structure"]
impl crate::Readable for LTC0_PKB3_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_4::W](ltc0_pkb3_4::W) writer structure"]
impl crate::Writable for LTC0_PKB3_4 {}
#[doc = "LTC PKHA B3 4 Register"]
pub mod ltc0_pkb3_4;
#[doc = "LTC PKHA B 52 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_52](ltc0_pkb_52) module"]
pub type LTC0_PKB_52 = crate::Reg<u32, _LTC0_PKB_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_52;
#[doc = "`read()` method returns [ltc0_pkb_52::R](ltc0_pkb_52::R) reader structure"]
impl crate::Readable for LTC0_PKB_52 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_52::W](ltc0_pkb_52::W) writer structure"]
impl crate::Writable for LTC0_PKB_52 {}
#[doc = "LTC PKHA B 52 Register"]
pub mod ltc0_pkb_52;
#[doc = "LTC PKHA B3 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_5](ltc0_pkb3_5) module"]
pub type LTC0_PKB3_5 = crate::Reg<u32, _LTC0_PKB3_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_5;
#[doc = "`read()` method returns [ltc0_pkb3_5::R](ltc0_pkb3_5::R) reader structure"]
impl crate::Readable for LTC0_PKB3_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_5::W](ltc0_pkb3_5::W) writer structure"]
impl crate::Writable for LTC0_PKB3_5 {}
#[doc = "LTC PKHA B3 5 Register"]
pub mod ltc0_pkb3_5;
#[doc = "LTC PKHA B 53 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_53](ltc0_pkb_53) module"]
pub type LTC0_PKB_53 = crate::Reg<u32, _LTC0_PKB_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_53;
#[doc = "`read()` method returns [ltc0_pkb_53::R](ltc0_pkb_53::R) reader structure"]
impl crate::Readable for LTC0_PKB_53 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_53::W](ltc0_pkb_53::W) writer structure"]
impl crate::Writable for LTC0_PKB_53 {}
#[doc = "LTC PKHA B 53 Register"]
pub mod ltc0_pkb_53;
#[doc = "LTC PKHA B3 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_6](ltc0_pkb3_6) module"]
pub type LTC0_PKB3_6 = crate::Reg<u32, _LTC0_PKB3_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_6;
#[doc = "`read()` method returns [ltc0_pkb3_6::R](ltc0_pkb3_6::R) reader structure"]
impl crate::Readable for LTC0_PKB3_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_6::W](ltc0_pkb3_6::W) writer structure"]
impl crate::Writable for LTC0_PKB3_6 {}
#[doc = "LTC PKHA B3 6 Register"]
pub mod ltc0_pkb3_6;
#[doc = "LTC PKHA B 54 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_54](ltc0_pkb_54) module"]
pub type LTC0_PKB_54 = crate::Reg<u32, _LTC0_PKB_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_54;
#[doc = "`read()` method returns [ltc0_pkb_54::R](ltc0_pkb_54::R) reader structure"]
impl crate::Readable for LTC0_PKB_54 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_54::W](ltc0_pkb_54::W) writer structure"]
impl crate::Writable for LTC0_PKB_54 {}
#[doc = "LTC PKHA B 54 Register"]
pub mod ltc0_pkb_54;
#[doc = "LTC PKHA B3 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_7](ltc0_pkb3_7) module"]
pub type LTC0_PKB3_7 = crate::Reg<u32, _LTC0_PKB3_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_7;
#[doc = "`read()` method returns [ltc0_pkb3_7::R](ltc0_pkb3_7::R) reader structure"]
impl crate::Readable for LTC0_PKB3_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_7::W](ltc0_pkb3_7::W) writer structure"]
impl crate::Writable for LTC0_PKB3_7 {}
#[doc = "LTC PKHA B3 7 Register"]
pub mod ltc0_pkb3_7;
#[doc = "LTC PKHA B 55 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_55](ltc0_pkb_55) module"]
pub type LTC0_PKB_55 = crate::Reg<u32, _LTC0_PKB_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_55;
#[doc = "`read()` method returns [ltc0_pkb_55::R](ltc0_pkb_55::R) reader structure"]
impl crate::Readable for LTC0_PKB_55 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_55::W](ltc0_pkb_55::W) writer structure"]
impl crate::Writable for LTC0_PKB_55 {}
#[doc = "LTC PKHA B 55 Register"]
pub mod ltc0_pkb_55;
#[doc = "LTC PKHA B3 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_8](ltc0_pkb3_8) module"]
pub type LTC0_PKB3_8 = crate::Reg<u32, _LTC0_PKB3_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_8;
#[doc = "`read()` method returns [ltc0_pkb3_8::R](ltc0_pkb3_8::R) reader structure"]
impl crate::Readable for LTC0_PKB3_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_8::W](ltc0_pkb3_8::W) writer structure"]
impl crate::Writable for LTC0_PKB3_8 {}
#[doc = "LTC PKHA B3 8 Register"]
pub mod ltc0_pkb3_8;
#[doc = "LTC PKHA B 56 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_56](ltc0_pkb_56) module"]
pub type LTC0_PKB_56 = crate::Reg<u32, _LTC0_PKB_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_56;
#[doc = "`read()` method returns [ltc0_pkb_56::R](ltc0_pkb_56::R) reader structure"]
impl crate::Readable for LTC0_PKB_56 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_56::W](ltc0_pkb_56::W) writer structure"]
impl crate::Writable for LTC0_PKB_56 {}
#[doc = "LTC PKHA B 56 Register"]
pub mod ltc0_pkb_56;
#[doc = "LTC PKHA B3 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_9](ltc0_pkb3_9) module"]
pub type LTC0_PKB3_9 = crate::Reg<u32, _LTC0_PKB3_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_9;
#[doc = "`read()` method returns [ltc0_pkb3_9::R](ltc0_pkb3_9::R) reader structure"]
impl crate::Readable for LTC0_PKB3_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_9::W](ltc0_pkb3_9::W) writer structure"]
impl crate::Writable for LTC0_PKB3_9 {}
#[doc = "LTC PKHA B3 9 Register"]
pub mod ltc0_pkb3_9;
#[doc = "LTC PKHA B 57 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_57](ltc0_pkb_57) module"]
pub type LTC0_PKB_57 = crate::Reg<u32, _LTC0_PKB_57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_57;
#[doc = "`read()` method returns [ltc0_pkb_57::R](ltc0_pkb_57::R) reader structure"]
impl crate::Readable for LTC0_PKB_57 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_57::W](ltc0_pkb_57::W) writer structure"]
impl crate::Writable for LTC0_PKB_57 {}
#[doc = "LTC PKHA B 57 Register"]
pub mod ltc0_pkb_57;
#[doc = "LTC PKHA B3 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_10](ltc0_pkb3_10) module"]
pub type LTC0_PKB3_10 = crate::Reg<u32, _LTC0_PKB3_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_10;
#[doc = "`read()` method returns [ltc0_pkb3_10::R](ltc0_pkb3_10::R) reader structure"]
impl crate::Readable for LTC0_PKB3_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_10::W](ltc0_pkb3_10::W) writer structure"]
impl crate::Writable for LTC0_PKB3_10 {}
#[doc = "LTC PKHA B3 10 Register"]
pub mod ltc0_pkb3_10;
#[doc = "LTC PKHA B 58 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_58](ltc0_pkb_58) module"]
pub type LTC0_PKB_58 = crate::Reg<u32, _LTC0_PKB_58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_58;
#[doc = "`read()` method returns [ltc0_pkb_58::R](ltc0_pkb_58::R) reader structure"]
impl crate::Readable for LTC0_PKB_58 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_58::W](ltc0_pkb_58::W) writer structure"]
impl crate::Writable for LTC0_PKB_58 {}
#[doc = "LTC PKHA B 58 Register"]
pub mod ltc0_pkb_58;
#[doc = "LTC PKHA B3 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_11](ltc0_pkb3_11) module"]
pub type LTC0_PKB3_11 = crate::Reg<u32, _LTC0_PKB3_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_11;
#[doc = "`read()` method returns [ltc0_pkb3_11::R](ltc0_pkb3_11::R) reader structure"]
impl crate::Readable for LTC0_PKB3_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_11::W](ltc0_pkb3_11::W) writer structure"]
impl crate::Writable for LTC0_PKB3_11 {}
#[doc = "LTC PKHA B3 11 Register"]
pub mod ltc0_pkb3_11;
#[doc = "LTC PKHA B 59 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_59](ltc0_pkb_59) module"]
pub type LTC0_PKB_59 = crate::Reg<u32, _LTC0_PKB_59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_59;
#[doc = "`read()` method returns [ltc0_pkb_59::R](ltc0_pkb_59::R) reader structure"]
impl crate::Readable for LTC0_PKB_59 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_59::W](ltc0_pkb_59::W) writer structure"]
impl crate::Writable for LTC0_PKB_59 {}
#[doc = "LTC PKHA B 59 Register"]
pub mod ltc0_pkb_59;
#[doc = "LTC PKHA B3 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_12](ltc0_pkb3_12) module"]
pub type LTC0_PKB3_12 = crate::Reg<u32, _LTC0_PKB3_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_12;
#[doc = "`read()` method returns [ltc0_pkb3_12::R](ltc0_pkb3_12::R) reader structure"]
impl crate::Readable for LTC0_PKB3_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_12::W](ltc0_pkb3_12::W) writer structure"]
impl crate::Writable for LTC0_PKB3_12 {}
#[doc = "LTC PKHA B3 12 Register"]
pub mod ltc0_pkb3_12;
#[doc = "LTC PKHA B 60 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_60](ltc0_pkb_60) module"]
pub type LTC0_PKB_60 = crate::Reg<u32, _LTC0_PKB_60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_60;
#[doc = "`read()` method returns [ltc0_pkb_60::R](ltc0_pkb_60::R) reader structure"]
impl crate::Readable for LTC0_PKB_60 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_60::W](ltc0_pkb_60::W) writer structure"]
impl crate::Writable for LTC0_PKB_60 {}
#[doc = "LTC PKHA B 60 Register"]
pub mod ltc0_pkb_60;
#[doc = "LTC PKHA B3 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_13](ltc0_pkb3_13) module"]
pub type LTC0_PKB3_13 = crate::Reg<u32, _LTC0_PKB3_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_13;
#[doc = "`read()` method returns [ltc0_pkb3_13::R](ltc0_pkb3_13::R) reader structure"]
impl crate::Readable for LTC0_PKB3_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_13::W](ltc0_pkb3_13::W) writer structure"]
impl crate::Writable for LTC0_PKB3_13 {}
#[doc = "LTC PKHA B3 13 Register"]
pub mod ltc0_pkb3_13;
#[doc = "LTC PKHA B 61 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_61](ltc0_pkb_61) module"]
pub type LTC0_PKB_61 = crate::Reg<u32, _LTC0_PKB_61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_61;
#[doc = "`read()` method returns [ltc0_pkb_61::R](ltc0_pkb_61::R) reader structure"]
impl crate::Readable for LTC0_PKB_61 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_61::W](ltc0_pkb_61::W) writer structure"]
impl crate::Writable for LTC0_PKB_61 {}
#[doc = "LTC PKHA B 61 Register"]
pub mod ltc0_pkb_61;
#[doc = "LTC PKHA B3 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_14](ltc0_pkb3_14) module"]
pub type LTC0_PKB3_14 = crate::Reg<u32, _LTC0_PKB3_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_14;
#[doc = "`read()` method returns [ltc0_pkb3_14::R](ltc0_pkb3_14::R) reader structure"]
impl crate::Readable for LTC0_PKB3_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_14::W](ltc0_pkb3_14::W) writer structure"]
impl crate::Writable for LTC0_PKB3_14 {}
#[doc = "LTC PKHA B3 14 Register"]
pub mod ltc0_pkb3_14;
#[doc = "LTC PKHA B 62 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_62](ltc0_pkb_62) module"]
pub type LTC0_PKB_62 = crate::Reg<u32, _LTC0_PKB_62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_62;
#[doc = "`read()` method returns [ltc0_pkb_62::R](ltc0_pkb_62::R) reader structure"]
impl crate::Readable for LTC0_PKB_62 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_62::W](ltc0_pkb_62::W) writer structure"]
impl crate::Writable for LTC0_PKB_62 {}
#[doc = "LTC PKHA B 62 Register"]
pub mod ltc0_pkb_62;
#[doc = "LTC PKHA B3 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb3_15](ltc0_pkb3_15) module"]
pub type LTC0_PKB3_15 = crate::Reg<u32, _LTC0_PKB3_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB3_15;
#[doc = "`read()` method returns [ltc0_pkb3_15::R](ltc0_pkb3_15::R) reader structure"]
impl crate::Readable for LTC0_PKB3_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb3_15::W](ltc0_pkb3_15::W) writer structure"]
impl crate::Writable for LTC0_PKB3_15 {}
#[doc = "LTC PKHA B3 15 Register"]
pub mod ltc0_pkb3_15;
#[doc = "LTC PKHA B 63 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkb_63](ltc0_pkb_63) module"]
pub type LTC0_PKB_63 = crate::Reg<u32, _LTC0_PKB_63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKB_63;
#[doc = "`read()` method returns [ltc0_pkb_63::R](ltc0_pkb_63::R) reader structure"]
impl crate::Readable for LTC0_PKB_63 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkb_63::W](ltc0_pkb_63::W) writer structure"]
impl crate::Writable for LTC0_PKB_63 {}
#[doc = "LTC PKHA B 63 Register"]
pub mod ltc0_pkb_63;
#[doc = "LTC PKHA N0 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_0](ltc0_pkn0_0) module"]
pub type LTC0_PKN0_0 = crate::Reg<u32, _LTC0_PKN0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_0;
#[doc = "`read()` method returns [ltc0_pkn0_0::R](ltc0_pkn0_0::R) reader structure"]
impl crate::Readable for LTC0_PKN0_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_0::W](ltc0_pkn0_0::W) writer structure"]
impl crate::Writable for LTC0_PKN0_0 {}
#[doc = "LTC PKHA N0 0 Register"]
pub mod ltc0_pkn0_0;
#[doc = "LTC PKHA N 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_0](ltc0_pkn_0) module"]
pub type LTC0_PKN_0 = crate::Reg<u32, _LTC0_PKN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_0;
#[doc = "`read()` method returns [ltc0_pkn_0::R](ltc0_pkn_0::R) reader structure"]
impl crate::Readable for LTC0_PKN_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_0::W](ltc0_pkn_0::W) writer structure"]
impl crate::Writable for LTC0_PKN_0 {}
#[doc = "LTC PKHA N 0 Register"]
pub mod ltc0_pkn_0;
#[doc = "LTC PKHA N0 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_1](ltc0_pkn0_1) module"]
pub type LTC0_PKN0_1 = crate::Reg<u32, _LTC0_PKN0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_1;
#[doc = "`read()` method returns [ltc0_pkn0_1::R](ltc0_pkn0_1::R) reader structure"]
impl crate::Readable for LTC0_PKN0_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_1::W](ltc0_pkn0_1::W) writer structure"]
impl crate::Writable for LTC0_PKN0_1 {}
#[doc = "LTC PKHA N0 1 Register"]
pub mod ltc0_pkn0_1;
#[doc = "LTC PKHA N 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_1](ltc0_pkn_1) module"]
pub type LTC0_PKN_1 = crate::Reg<u32, _LTC0_PKN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_1;
#[doc = "`read()` method returns [ltc0_pkn_1::R](ltc0_pkn_1::R) reader structure"]
impl crate::Readable for LTC0_PKN_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_1::W](ltc0_pkn_1::W) writer structure"]
impl crate::Writable for LTC0_PKN_1 {}
#[doc = "LTC PKHA N 1 Register"]
pub mod ltc0_pkn_1;
#[doc = "LTC PKHA N0 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_2](ltc0_pkn0_2) module"]
pub type LTC0_PKN0_2 = crate::Reg<u32, _LTC0_PKN0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_2;
#[doc = "`read()` method returns [ltc0_pkn0_2::R](ltc0_pkn0_2::R) reader structure"]
impl crate::Readable for LTC0_PKN0_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_2::W](ltc0_pkn0_2::W) writer structure"]
impl crate::Writable for LTC0_PKN0_2 {}
#[doc = "LTC PKHA N0 2 Register"]
pub mod ltc0_pkn0_2;
#[doc = "LTC PKHA N 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_2](ltc0_pkn_2) module"]
pub type LTC0_PKN_2 = crate::Reg<u32, _LTC0_PKN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_2;
#[doc = "`read()` method returns [ltc0_pkn_2::R](ltc0_pkn_2::R) reader structure"]
impl crate::Readable for LTC0_PKN_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_2::W](ltc0_pkn_2::W) writer structure"]
impl crate::Writable for LTC0_PKN_2 {}
#[doc = "LTC PKHA N 2 Register"]
pub mod ltc0_pkn_2;
#[doc = "LTC PKHA N0 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_3](ltc0_pkn0_3) module"]
pub type LTC0_PKN0_3 = crate::Reg<u32, _LTC0_PKN0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_3;
#[doc = "`read()` method returns [ltc0_pkn0_3::R](ltc0_pkn0_3::R) reader structure"]
impl crate::Readable for LTC0_PKN0_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_3::W](ltc0_pkn0_3::W) writer structure"]
impl crate::Writable for LTC0_PKN0_3 {}
#[doc = "LTC PKHA N0 3 Register"]
pub mod ltc0_pkn0_3;
#[doc = "LTC PKHA N 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_3](ltc0_pkn_3) module"]
pub type LTC0_PKN_3 = crate::Reg<u32, _LTC0_PKN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_3;
#[doc = "`read()` method returns [ltc0_pkn_3::R](ltc0_pkn_3::R) reader structure"]
impl crate::Readable for LTC0_PKN_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_3::W](ltc0_pkn_3::W) writer structure"]
impl crate::Writable for LTC0_PKN_3 {}
#[doc = "LTC PKHA N 3 Register"]
pub mod ltc0_pkn_3;
#[doc = "LTC PKHA N0 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_4](ltc0_pkn0_4) module"]
pub type LTC0_PKN0_4 = crate::Reg<u32, _LTC0_PKN0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_4;
#[doc = "`read()` method returns [ltc0_pkn0_4::R](ltc0_pkn0_4::R) reader structure"]
impl crate::Readable for LTC0_PKN0_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_4::W](ltc0_pkn0_4::W) writer structure"]
impl crate::Writable for LTC0_PKN0_4 {}
#[doc = "LTC PKHA N0 4 Register"]
pub mod ltc0_pkn0_4;
#[doc = "LTC PKHA N 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_4](ltc0_pkn_4) module"]
pub type LTC0_PKN_4 = crate::Reg<u32, _LTC0_PKN_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_4;
#[doc = "`read()` method returns [ltc0_pkn_4::R](ltc0_pkn_4::R) reader structure"]
impl crate::Readable for LTC0_PKN_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_4::W](ltc0_pkn_4::W) writer structure"]
impl crate::Writable for LTC0_PKN_4 {}
#[doc = "LTC PKHA N 4 Register"]
pub mod ltc0_pkn_4;
#[doc = "LTC PKHA N0 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_5](ltc0_pkn0_5) module"]
pub type LTC0_PKN0_5 = crate::Reg<u32, _LTC0_PKN0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_5;
#[doc = "`read()` method returns [ltc0_pkn0_5::R](ltc0_pkn0_5::R) reader structure"]
impl crate::Readable for LTC0_PKN0_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_5::W](ltc0_pkn0_5::W) writer structure"]
impl crate::Writable for LTC0_PKN0_5 {}
#[doc = "LTC PKHA N0 5 Register"]
pub mod ltc0_pkn0_5;
#[doc = "LTC PKHA N 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_5](ltc0_pkn_5) module"]
pub type LTC0_PKN_5 = crate::Reg<u32, _LTC0_PKN_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_5;
#[doc = "`read()` method returns [ltc0_pkn_5::R](ltc0_pkn_5::R) reader structure"]
impl crate::Readable for LTC0_PKN_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_5::W](ltc0_pkn_5::W) writer structure"]
impl crate::Writable for LTC0_PKN_5 {}
#[doc = "LTC PKHA N 5 Register"]
pub mod ltc0_pkn_5;
#[doc = "LTC PKHA N0 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_6](ltc0_pkn0_6) module"]
pub type LTC0_PKN0_6 = crate::Reg<u32, _LTC0_PKN0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_6;
#[doc = "`read()` method returns [ltc0_pkn0_6::R](ltc0_pkn0_6::R) reader structure"]
impl crate::Readable for LTC0_PKN0_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_6::W](ltc0_pkn0_6::W) writer structure"]
impl crate::Writable for LTC0_PKN0_6 {}
#[doc = "LTC PKHA N0 6 Register"]
pub mod ltc0_pkn0_6;
#[doc = "LTC PKHA N 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_6](ltc0_pkn_6) module"]
pub type LTC0_PKN_6 = crate::Reg<u32, _LTC0_PKN_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_6;
#[doc = "`read()` method returns [ltc0_pkn_6::R](ltc0_pkn_6::R) reader structure"]
impl crate::Readable for LTC0_PKN_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_6::W](ltc0_pkn_6::W) writer structure"]
impl crate::Writable for LTC0_PKN_6 {}
#[doc = "LTC PKHA N 6 Register"]
pub mod ltc0_pkn_6;
#[doc = "LTC PKHA N0 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_7](ltc0_pkn0_7) module"]
pub type LTC0_PKN0_7 = crate::Reg<u32, _LTC0_PKN0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_7;
#[doc = "`read()` method returns [ltc0_pkn0_7::R](ltc0_pkn0_7::R) reader structure"]
impl crate::Readable for LTC0_PKN0_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_7::W](ltc0_pkn0_7::W) writer structure"]
impl crate::Writable for LTC0_PKN0_7 {}
#[doc = "LTC PKHA N0 7 Register"]
pub mod ltc0_pkn0_7;
#[doc = "LTC PKHA N 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_7](ltc0_pkn_7) module"]
pub type LTC0_PKN_7 = crate::Reg<u32, _LTC0_PKN_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_7;
#[doc = "`read()` method returns [ltc0_pkn_7::R](ltc0_pkn_7::R) reader structure"]
impl crate::Readable for LTC0_PKN_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_7::W](ltc0_pkn_7::W) writer structure"]
impl crate::Writable for LTC0_PKN_7 {}
#[doc = "LTC PKHA N 7 Register"]
pub mod ltc0_pkn_7;
#[doc = "LTC PKHA N0 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_8](ltc0_pkn0_8) module"]
pub type LTC0_PKN0_8 = crate::Reg<u32, _LTC0_PKN0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_8;
#[doc = "`read()` method returns [ltc0_pkn0_8::R](ltc0_pkn0_8::R) reader structure"]
impl crate::Readable for LTC0_PKN0_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_8::W](ltc0_pkn0_8::W) writer structure"]
impl crate::Writable for LTC0_PKN0_8 {}
#[doc = "LTC PKHA N0 8 Register"]
pub mod ltc0_pkn0_8;
#[doc = "LTC PKHA N 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_8](ltc0_pkn_8) module"]
pub type LTC0_PKN_8 = crate::Reg<u32, _LTC0_PKN_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_8;
#[doc = "`read()` method returns [ltc0_pkn_8::R](ltc0_pkn_8::R) reader structure"]
impl crate::Readable for LTC0_PKN_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_8::W](ltc0_pkn_8::W) writer structure"]
impl crate::Writable for LTC0_PKN_8 {}
#[doc = "LTC PKHA N 8 Register"]
pub mod ltc0_pkn_8;
#[doc = "LTC PKHA N0 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_9](ltc0_pkn0_9) module"]
pub type LTC0_PKN0_9 = crate::Reg<u32, _LTC0_PKN0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_9;
#[doc = "`read()` method returns [ltc0_pkn0_9::R](ltc0_pkn0_9::R) reader structure"]
impl crate::Readable for LTC0_PKN0_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_9::W](ltc0_pkn0_9::W) writer structure"]
impl crate::Writable for LTC0_PKN0_9 {}
#[doc = "LTC PKHA N0 9 Register"]
pub mod ltc0_pkn0_9;
#[doc = "LTC PKHA N 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_9](ltc0_pkn_9) module"]
pub type LTC0_PKN_9 = crate::Reg<u32, _LTC0_PKN_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_9;
#[doc = "`read()` method returns [ltc0_pkn_9::R](ltc0_pkn_9::R) reader structure"]
impl crate::Readable for LTC0_PKN_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_9::W](ltc0_pkn_9::W) writer structure"]
impl crate::Writable for LTC0_PKN_9 {}
#[doc = "LTC PKHA N 9 Register"]
pub mod ltc0_pkn_9;
#[doc = "LTC PKHA N0 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_10](ltc0_pkn0_10) module"]
pub type LTC0_PKN0_10 = crate::Reg<u32, _LTC0_PKN0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_10;
#[doc = "`read()` method returns [ltc0_pkn0_10::R](ltc0_pkn0_10::R) reader structure"]
impl crate::Readable for LTC0_PKN0_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_10::W](ltc0_pkn0_10::W) writer structure"]
impl crate::Writable for LTC0_PKN0_10 {}
#[doc = "LTC PKHA N0 10 Register"]
pub mod ltc0_pkn0_10;
#[doc = "LTC PKHA N 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_10](ltc0_pkn_10) module"]
pub type LTC0_PKN_10 = crate::Reg<u32, _LTC0_PKN_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_10;
#[doc = "`read()` method returns [ltc0_pkn_10::R](ltc0_pkn_10::R) reader structure"]
impl crate::Readable for LTC0_PKN_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_10::W](ltc0_pkn_10::W) writer structure"]
impl crate::Writable for LTC0_PKN_10 {}
#[doc = "LTC PKHA N 10 Register"]
pub mod ltc0_pkn_10;
#[doc = "LTC PKHA N0 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_11](ltc0_pkn0_11) module"]
pub type LTC0_PKN0_11 = crate::Reg<u32, _LTC0_PKN0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_11;
#[doc = "`read()` method returns [ltc0_pkn0_11::R](ltc0_pkn0_11::R) reader structure"]
impl crate::Readable for LTC0_PKN0_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_11::W](ltc0_pkn0_11::W) writer structure"]
impl crate::Writable for LTC0_PKN0_11 {}
#[doc = "LTC PKHA N0 11 Register"]
pub mod ltc0_pkn0_11;
#[doc = "LTC PKHA N 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_11](ltc0_pkn_11) module"]
pub type LTC0_PKN_11 = crate::Reg<u32, _LTC0_PKN_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_11;
#[doc = "`read()` method returns [ltc0_pkn_11::R](ltc0_pkn_11::R) reader structure"]
impl crate::Readable for LTC0_PKN_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_11::W](ltc0_pkn_11::W) writer structure"]
impl crate::Writable for LTC0_PKN_11 {}
#[doc = "LTC PKHA N 11 Register"]
pub mod ltc0_pkn_11;
#[doc = "LTC PKHA N0 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_12](ltc0_pkn0_12) module"]
pub type LTC0_PKN0_12 = crate::Reg<u32, _LTC0_PKN0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_12;
#[doc = "`read()` method returns [ltc0_pkn0_12::R](ltc0_pkn0_12::R) reader structure"]
impl crate::Readable for LTC0_PKN0_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_12::W](ltc0_pkn0_12::W) writer structure"]
impl crate::Writable for LTC0_PKN0_12 {}
#[doc = "LTC PKHA N0 12 Register"]
pub mod ltc0_pkn0_12;
#[doc = "LTC PKHA N 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_12](ltc0_pkn_12) module"]
pub type LTC0_PKN_12 = crate::Reg<u32, _LTC0_PKN_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_12;
#[doc = "`read()` method returns [ltc0_pkn_12::R](ltc0_pkn_12::R) reader structure"]
impl crate::Readable for LTC0_PKN_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_12::W](ltc0_pkn_12::W) writer structure"]
impl crate::Writable for LTC0_PKN_12 {}
#[doc = "LTC PKHA N 12 Register"]
pub mod ltc0_pkn_12;
#[doc = "LTC PKHA N0 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_13](ltc0_pkn0_13) module"]
pub type LTC0_PKN0_13 = crate::Reg<u32, _LTC0_PKN0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_13;
#[doc = "`read()` method returns [ltc0_pkn0_13::R](ltc0_pkn0_13::R) reader structure"]
impl crate::Readable for LTC0_PKN0_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_13::W](ltc0_pkn0_13::W) writer structure"]
impl crate::Writable for LTC0_PKN0_13 {}
#[doc = "LTC PKHA N0 13 Register"]
pub mod ltc0_pkn0_13;
#[doc = "LTC PKHA N 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_13](ltc0_pkn_13) module"]
pub type LTC0_PKN_13 = crate::Reg<u32, _LTC0_PKN_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_13;
#[doc = "`read()` method returns [ltc0_pkn_13::R](ltc0_pkn_13::R) reader structure"]
impl crate::Readable for LTC0_PKN_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_13::W](ltc0_pkn_13::W) writer structure"]
impl crate::Writable for LTC0_PKN_13 {}
#[doc = "LTC PKHA N 13 Register"]
pub mod ltc0_pkn_13;
#[doc = "LTC PKHA N0 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_14](ltc0_pkn0_14) module"]
pub type LTC0_PKN0_14 = crate::Reg<u32, _LTC0_PKN0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_14;
#[doc = "`read()` method returns [ltc0_pkn0_14::R](ltc0_pkn0_14::R) reader structure"]
impl crate::Readable for LTC0_PKN0_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_14::W](ltc0_pkn0_14::W) writer structure"]
impl crate::Writable for LTC0_PKN0_14 {}
#[doc = "LTC PKHA N0 14 Register"]
pub mod ltc0_pkn0_14;
#[doc = "LTC PKHA N 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_14](ltc0_pkn_14) module"]
pub type LTC0_PKN_14 = crate::Reg<u32, _LTC0_PKN_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_14;
#[doc = "`read()` method returns [ltc0_pkn_14::R](ltc0_pkn_14::R) reader structure"]
impl crate::Readable for LTC0_PKN_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_14::W](ltc0_pkn_14::W) writer structure"]
impl crate::Writable for LTC0_PKN_14 {}
#[doc = "LTC PKHA N 14 Register"]
pub mod ltc0_pkn_14;
#[doc = "LTC PKHA N0 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn0_15](ltc0_pkn0_15) module"]
pub type LTC0_PKN0_15 = crate::Reg<u32, _LTC0_PKN0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN0_15;
#[doc = "`read()` method returns [ltc0_pkn0_15::R](ltc0_pkn0_15::R) reader structure"]
impl crate::Readable for LTC0_PKN0_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn0_15::W](ltc0_pkn0_15::W) writer structure"]
impl crate::Writable for LTC0_PKN0_15 {}
#[doc = "LTC PKHA N0 15 Register"]
pub mod ltc0_pkn0_15;
#[doc = "LTC PKHA N 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_15](ltc0_pkn_15) module"]
pub type LTC0_PKN_15 = crate::Reg<u32, _LTC0_PKN_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_15;
#[doc = "`read()` method returns [ltc0_pkn_15::R](ltc0_pkn_15::R) reader structure"]
impl crate::Readable for LTC0_PKN_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_15::W](ltc0_pkn_15::W) writer structure"]
impl crate::Writable for LTC0_PKN_15 {}
#[doc = "LTC PKHA N 15 Register"]
pub mod ltc0_pkn_15;
#[doc = "LTC PKHA N1 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_0](ltc0_pkn1_0) module"]
pub type LTC0_PKN1_0 = crate::Reg<u32, _LTC0_PKN1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_0;
#[doc = "`read()` method returns [ltc0_pkn1_0::R](ltc0_pkn1_0::R) reader structure"]
impl crate::Readable for LTC0_PKN1_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_0::W](ltc0_pkn1_0::W) writer structure"]
impl crate::Writable for LTC0_PKN1_0 {}
#[doc = "LTC PKHA N1 0 Register"]
pub mod ltc0_pkn1_0;
#[doc = "LTC PKHA N 16 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_16](ltc0_pkn_16) module"]
pub type LTC0_PKN_16 = crate::Reg<u32, _LTC0_PKN_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_16;
#[doc = "`read()` method returns [ltc0_pkn_16::R](ltc0_pkn_16::R) reader structure"]
impl crate::Readable for LTC0_PKN_16 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_16::W](ltc0_pkn_16::W) writer structure"]
impl crate::Writable for LTC0_PKN_16 {}
#[doc = "LTC PKHA N 16 Register"]
pub mod ltc0_pkn_16;
#[doc = "LTC PKHA N1 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_1](ltc0_pkn1_1) module"]
pub type LTC0_PKN1_1 = crate::Reg<u32, _LTC0_PKN1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_1;
#[doc = "`read()` method returns [ltc0_pkn1_1::R](ltc0_pkn1_1::R) reader structure"]
impl crate::Readable for LTC0_PKN1_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_1::W](ltc0_pkn1_1::W) writer structure"]
impl crate::Writable for LTC0_PKN1_1 {}
#[doc = "LTC PKHA N1 1 Register"]
pub mod ltc0_pkn1_1;
#[doc = "LTC PKHA N 17 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_17](ltc0_pkn_17) module"]
pub type LTC0_PKN_17 = crate::Reg<u32, _LTC0_PKN_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_17;
#[doc = "`read()` method returns [ltc0_pkn_17::R](ltc0_pkn_17::R) reader structure"]
impl crate::Readable for LTC0_PKN_17 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_17::W](ltc0_pkn_17::W) writer structure"]
impl crate::Writable for LTC0_PKN_17 {}
#[doc = "LTC PKHA N 17 Register"]
pub mod ltc0_pkn_17;
#[doc = "LTC PKHA N1 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_2](ltc0_pkn1_2) module"]
pub type LTC0_PKN1_2 = crate::Reg<u32, _LTC0_PKN1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_2;
#[doc = "`read()` method returns [ltc0_pkn1_2::R](ltc0_pkn1_2::R) reader structure"]
impl crate::Readable for LTC0_PKN1_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_2::W](ltc0_pkn1_2::W) writer structure"]
impl crate::Writable for LTC0_PKN1_2 {}
#[doc = "LTC PKHA N1 2 Register"]
pub mod ltc0_pkn1_2;
#[doc = "LTC PKHA N 18 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_18](ltc0_pkn_18) module"]
pub type LTC0_PKN_18 = crate::Reg<u32, _LTC0_PKN_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_18;
#[doc = "`read()` method returns [ltc0_pkn_18::R](ltc0_pkn_18::R) reader structure"]
impl crate::Readable for LTC0_PKN_18 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_18::W](ltc0_pkn_18::W) writer structure"]
impl crate::Writable for LTC0_PKN_18 {}
#[doc = "LTC PKHA N 18 Register"]
pub mod ltc0_pkn_18;
#[doc = "LTC PKHA N1 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_3](ltc0_pkn1_3) module"]
pub type LTC0_PKN1_3 = crate::Reg<u32, _LTC0_PKN1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_3;
#[doc = "`read()` method returns [ltc0_pkn1_3::R](ltc0_pkn1_3::R) reader structure"]
impl crate::Readable for LTC0_PKN1_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_3::W](ltc0_pkn1_3::W) writer structure"]
impl crate::Writable for LTC0_PKN1_3 {}
#[doc = "LTC PKHA N1 3 Register"]
pub mod ltc0_pkn1_3;
#[doc = "LTC PKHA N 19 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_19](ltc0_pkn_19) module"]
pub type LTC0_PKN_19 = crate::Reg<u32, _LTC0_PKN_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_19;
#[doc = "`read()` method returns [ltc0_pkn_19::R](ltc0_pkn_19::R) reader structure"]
impl crate::Readable for LTC0_PKN_19 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_19::W](ltc0_pkn_19::W) writer structure"]
impl crate::Writable for LTC0_PKN_19 {}
#[doc = "LTC PKHA N 19 Register"]
pub mod ltc0_pkn_19;
#[doc = "LTC PKHA N1 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_4](ltc0_pkn1_4) module"]
pub type LTC0_PKN1_4 = crate::Reg<u32, _LTC0_PKN1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_4;
#[doc = "`read()` method returns [ltc0_pkn1_4::R](ltc0_pkn1_4::R) reader structure"]
impl crate::Readable for LTC0_PKN1_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_4::W](ltc0_pkn1_4::W) writer structure"]
impl crate::Writable for LTC0_PKN1_4 {}
#[doc = "LTC PKHA N1 4 Register"]
pub mod ltc0_pkn1_4;
#[doc = "LTC PKHA N 20 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_20](ltc0_pkn_20) module"]
pub type LTC0_PKN_20 = crate::Reg<u32, _LTC0_PKN_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_20;
#[doc = "`read()` method returns [ltc0_pkn_20::R](ltc0_pkn_20::R) reader structure"]
impl crate::Readable for LTC0_PKN_20 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_20::W](ltc0_pkn_20::W) writer structure"]
impl crate::Writable for LTC0_PKN_20 {}
#[doc = "LTC PKHA N 20 Register"]
pub mod ltc0_pkn_20;
#[doc = "LTC PKHA N1 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_5](ltc0_pkn1_5) module"]
pub type LTC0_PKN1_5 = crate::Reg<u32, _LTC0_PKN1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_5;
#[doc = "`read()` method returns [ltc0_pkn1_5::R](ltc0_pkn1_5::R) reader structure"]
impl crate::Readable for LTC0_PKN1_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_5::W](ltc0_pkn1_5::W) writer structure"]
impl crate::Writable for LTC0_PKN1_5 {}
#[doc = "LTC PKHA N1 5 Register"]
pub mod ltc0_pkn1_5;
#[doc = "LTC PKHA N 21 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_21](ltc0_pkn_21) module"]
pub type LTC0_PKN_21 = crate::Reg<u32, _LTC0_PKN_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_21;
#[doc = "`read()` method returns [ltc0_pkn_21::R](ltc0_pkn_21::R) reader structure"]
impl crate::Readable for LTC0_PKN_21 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_21::W](ltc0_pkn_21::W) writer structure"]
impl crate::Writable for LTC0_PKN_21 {}
#[doc = "LTC PKHA N 21 Register"]
pub mod ltc0_pkn_21;
#[doc = "LTC PKHA N1 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_6](ltc0_pkn1_6) module"]
pub type LTC0_PKN1_6 = crate::Reg<u32, _LTC0_PKN1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_6;
#[doc = "`read()` method returns [ltc0_pkn1_6::R](ltc0_pkn1_6::R) reader structure"]
impl crate::Readable for LTC0_PKN1_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_6::W](ltc0_pkn1_6::W) writer structure"]
impl crate::Writable for LTC0_PKN1_6 {}
#[doc = "LTC PKHA N1 6 Register"]
pub mod ltc0_pkn1_6;
#[doc = "LTC PKHA N 22 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_22](ltc0_pkn_22) module"]
pub type LTC0_PKN_22 = crate::Reg<u32, _LTC0_PKN_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_22;
#[doc = "`read()` method returns [ltc0_pkn_22::R](ltc0_pkn_22::R) reader structure"]
impl crate::Readable for LTC0_PKN_22 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_22::W](ltc0_pkn_22::W) writer structure"]
impl crate::Writable for LTC0_PKN_22 {}
#[doc = "LTC PKHA N 22 Register"]
pub mod ltc0_pkn_22;
#[doc = "LTC PKHA N1 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_7](ltc0_pkn1_7) module"]
pub type LTC0_PKN1_7 = crate::Reg<u32, _LTC0_PKN1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_7;
#[doc = "`read()` method returns [ltc0_pkn1_7::R](ltc0_pkn1_7::R) reader structure"]
impl crate::Readable for LTC0_PKN1_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_7::W](ltc0_pkn1_7::W) writer structure"]
impl crate::Writable for LTC0_PKN1_7 {}
#[doc = "LTC PKHA N1 7 Register"]
pub mod ltc0_pkn1_7;
#[doc = "LTC PKHA N 23 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_23](ltc0_pkn_23) module"]
pub type LTC0_PKN_23 = crate::Reg<u32, _LTC0_PKN_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_23;
#[doc = "`read()` method returns [ltc0_pkn_23::R](ltc0_pkn_23::R) reader structure"]
impl crate::Readable for LTC0_PKN_23 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_23::W](ltc0_pkn_23::W) writer structure"]
impl crate::Writable for LTC0_PKN_23 {}
#[doc = "LTC PKHA N 23 Register"]
pub mod ltc0_pkn_23;
#[doc = "LTC PKHA N1 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_8](ltc0_pkn1_8) module"]
pub type LTC0_PKN1_8 = crate::Reg<u32, _LTC0_PKN1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_8;
#[doc = "`read()` method returns [ltc0_pkn1_8::R](ltc0_pkn1_8::R) reader structure"]
impl crate::Readable for LTC0_PKN1_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_8::W](ltc0_pkn1_8::W) writer structure"]
impl crate::Writable for LTC0_PKN1_8 {}
#[doc = "LTC PKHA N1 8 Register"]
pub mod ltc0_pkn1_8;
#[doc = "LTC PKHA N 24 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_24](ltc0_pkn_24) module"]
pub type LTC0_PKN_24 = crate::Reg<u32, _LTC0_PKN_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_24;
#[doc = "`read()` method returns [ltc0_pkn_24::R](ltc0_pkn_24::R) reader structure"]
impl crate::Readable for LTC0_PKN_24 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_24::W](ltc0_pkn_24::W) writer structure"]
impl crate::Writable for LTC0_PKN_24 {}
#[doc = "LTC PKHA N 24 Register"]
pub mod ltc0_pkn_24;
#[doc = "LTC PKHA N1 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_9](ltc0_pkn1_9) module"]
pub type LTC0_PKN1_9 = crate::Reg<u32, _LTC0_PKN1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_9;
#[doc = "`read()` method returns [ltc0_pkn1_9::R](ltc0_pkn1_9::R) reader structure"]
impl crate::Readable for LTC0_PKN1_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_9::W](ltc0_pkn1_9::W) writer structure"]
impl crate::Writable for LTC0_PKN1_9 {}
#[doc = "LTC PKHA N1 9 Register"]
pub mod ltc0_pkn1_9;
#[doc = "LTC PKHA N 25 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_25](ltc0_pkn_25) module"]
pub type LTC0_PKN_25 = crate::Reg<u32, _LTC0_PKN_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_25;
#[doc = "`read()` method returns [ltc0_pkn_25::R](ltc0_pkn_25::R) reader structure"]
impl crate::Readable for LTC0_PKN_25 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_25::W](ltc0_pkn_25::W) writer structure"]
impl crate::Writable for LTC0_PKN_25 {}
#[doc = "LTC PKHA N 25 Register"]
pub mod ltc0_pkn_25;
#[doc = "LTC PKHA N1 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_10](ltc0_pkn1_10) module"]
pub type LTC0_PKN1_10 = crate::Reg<u32, _LTC0_PKN1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_10;
#[doc = "`read()` method returns [ltc0_pkn1_10::R](ltc0_pkn1_10::R) reader structure"]
impl crate::Readable for LTC0_PKN1_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_10::W](ltc0_pkn1_10::W) writer structure"]
impl crate::Writable for LTC0_PKN1_10 {}
#[doc = "LTC PKHA N1 10 Register"]
pub mod ltc0_pkn1_10;
#[doc = "LTC PKHA N 26 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_26](ltc0_pkn_26) module"]
pub type LTC0_PKN_26 = crate::Reg<u32, _LTC0_PKN_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_26;
#[doc = "`read()` method returns [ltc0_pkn_26::R](ltc0_pkn_26::R) reader structure"]
impl crate::Readable for LTC0_PKN_26 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_26::W](ltc0_pkn_26::W) writer structure"]
impl crate::Writable for LTC0_PKN_26 {}
#[doc = "LTC PKHA N 26 Register"]
pub mod ltc0_pkn_26;
#[doc = "LTC PKHA N1 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_11](ltc0_pkn1_11) module"]
pub type LTC0_PKN1_11 = crate::Reg<u32, _LTC0_PKN1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_11;
#[doc = "`read()` method returns [ltc0_pkn1_11::R](ltc0_pkn1_11::R) reader structure"]
impl crate::Readable for LTC0_PKN1_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_11::W](ltc0_pkn1_11::W) writer structure"]
impl crate::Writable for LTC0_PKN1_11 {}
#[doc = "LTC PKHA N1 11 Register"]
pub mod ltc0_pkn1_11;
#[doc = "LTC PKHA N 27 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_27](ltc0_pkn_27) module"]
pub type LTC0_PKN_27 = crate::Reg<u32, _LTC0_PKN_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_27;
#[doc = "`read()` method returns [ltc0_pkn_27::R](ltc0_pkn_27::R) reader structure"]
impl crate::Readable for LTC0_PKN_27 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_27::W](ltc0_pkn_27::W) writer structure"]
impl crate::Writable for LTC0_PKN_27 {}
#[doc = "LTC PKHA N 27 Register"]
pub mod ltc0_pkn_27;
#[doc = "LTC PKHA N1 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_12](ltc0_pkn1_12) module"]
pub type LTC0_PKN1_12 = crate::Reg<u32, _LTC0_PKN1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_12;
#[doc = "`read()` method returns [ltc0_pkn1_12::R](ltc0_pkn1_12::R) reader structure"]
impl crate::Readable for LTC0_PKN1_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_12::W](ltc0_pkn1_12::W) writer structure"]
impl crate::Writable for LTC0_PKN1_12 {}
#[doc = "LTC PKHA N1 12 Register"]
pub mod ltc0_pkn1_12;
#[doc = "LTC PKHA N 28 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_28](ltc0_pkn_28) module"]
pub type LTC0_PKN_28 = crate::Reg<u32, _LTC0_PKN_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_28;
#[doc = "`read()` method returns [ltc0_pkn_28::R](ltc0_pkn_28::R) reader structure"]
impl crate::Readable for LTC0_PKN_28 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_28::W](ltc0_pkn_28::W) writer structure"]
impl crate::Writable for LTC0_PKN_28 {}
#[doc = "LTC PKHA N 28 Register"]
pub mod ltc0_pkn_28;
#[doc = "LTC PKHA N1 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_13](ltc0_pkn1_13) module"]
pub type LTC0_PKN1_13 = crate::Reg<u32, _LTC0_PKN1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_13;
#[doc = "`read()` method returns [ltc0_pkn1_13::R](ltc0_pkn1_13::R) reader structure"]
impl crate::Readable for LTC0_PKN1_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_13::W](ltc0_pkn1_13::W) writer structure"]
impl crate::Writable for LTC0_PKN1_13 {}
#[doc = "LTC PKHA N1 13 Register"]
pub mod ltc0_pkn1_13;
#[doc = "LTC PKHA N 29 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_29](ltc0_pkn_29) module"]
pub type LTC0_PKN_29 = crate::Reg<u32, _LTC0_PKN_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_29;
#[doc = "`read()` method returns [ltc0_pkn_29::R](ltc0_pkn_29::R) reader structure"]
impl crate::Readable for LTC0_PKN_29 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_29::W](ltc0_pkn_29::W) writer structure"]
impl crate::Writable for LTC0_PKN_29 {}
#[doc = "LTC PKHA N 29 Register"]
pub mod ltc0_pkn_29;
#[doc = "LTC PKHA N1 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_14](ltc0_pkn1_14) module"]
pub type LTC0_PKN1_14 = crate::Reg<u32, _LTC0_PKN1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_14;
#[doc = "`read()` method returns [ltc0_pkn1_14::R](ltc0_pkn1_14::R) reader structure"]
impl crate::Readable for LTC0_PKN1_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_14::W](ltc0_pkn1_14::W) writer structure"]
impl crate::Writable for LTC0_PKN1_14 {}
#[doc = "LTC PKHA N1 14 Register"]
pub mod ltc0_pkn1_14;
#[doc = "LTC PKHA N 30 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_30](ltc0_pkn_30) module"]
pub type LTC0_PKN_30 = crate::Reg<u32, _LTC0_PKN_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_30;
#[doc = "`read()` method returns [ltc0_pkn_30::R](ltc0_pkn_30::R) reader structure"]
impl crate::Readable for LTC0_PKN_30 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_30::W](ltc0_pkn_30::W) writer structure"]
impl crate::Writable for LTC0_PKN_30 {}
#[doc = "LTC PKHA N 30 Register"]
pub mod ltc0_pkn_30;
#[doc = "LTC PKHA N1 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn1_15](ltc0_pkn1_15) module"]
pub type LTC0_PKN1_15 = crate::Reg<u32, _LTC0_PKN1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN1_15;
#[doc = "`read()` method returns [ltc0_pkn1_15::R](ltc0_pkn1_15::R) reader structure"]
impl crate::Readable for LTC0_PKN1_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn1_15::W](ltc0_pkn1_15::W) writer structure"]
impl crate::Writable for LTC0_PKN1_15 {}
#[doc = "LTC PKHA N1 15 Register"]
pub mod ltc0_pkn1_15;
#[doc = "LTC PKHA N 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_31](ltc0_pkn_31) module"]
pub type LTC0_PKN_31 = crate::Reg<u32, _LTC0_PKN_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_31;
#[doc = "`read()` method returns [ltc0_pkn_31::R](ltc0_pkn_31::R) reader structure"]
impl crate::Readable for LTC0_PKN_31 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_31::W](ltc0_pkn_31::W) writer structure"]
impl crate::Writable for LTC0_PKN_31 {}
#[doc = "LTC PKHA N 31 Register"]
pub mod ltc0_pkn_31;
#[doc = "LTC PKHA N2 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_0](ltc0_pkn2_0) module"]
pub type LTC0_PKN2_0 = crate::Reg<u32, _LTC0_PKN2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_0;
#[doc = "`read()` method returns [ltc0_pkn2_0::R](ltc0_pkn2_0::R) reader structure"]
impl crate::Readable for LTC0_PKN2_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_0::W](ltc0_pkn2_0::W) writer structure"]
impl crate::Writable for LTC0_PKN2_0 {}
#[doc = "LTC PKHA N2 0 Register"]
pub mod ltc0_pkn2_0;
#[doc = "LTC PKHA N 32 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_32](ltc0_pkn_32) module"]
pub type LTC0_PKN_32 = crate::Reg<u32, _LTC0_PKN_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_32;
#[doc = "`read()` method returns [ltc0_pkn_32::R](ltc0_pkn_32::R) reader structure"]
impl crate::Readable for LTC0_PKN_32 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_32::W](ltc0_pkn_32::W) writer structure"]
impl crate::Writable for LTC0_PKN_32 {}
#[doc = "LTC PKHA N 32 Register"]
pub mod ltc0_pkn_32;
#[doc = "LTC PKHA N2 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_1](ltc0_pkn2_1) module"]
pub type LTC0_PKN2_1 = crate::Reg<u32, _LTC0_PKN2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_1;
#[doc = "`read()` method returns [ltc0_pkn2_1::R](ltc0_pkn2_1::R) reader structure"]
impl crate::Readable for LTC0_PKN2_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_1::W](ltc0_pkn2_1::W) writer structure"]
impl crate::Writable for LTC0_PKN2_1 {}
#[doc = "LTC PKHA N2 1 Register"]
pub mod ltc0_pkn2_1;
#[doc = "LTC PKHA N 33 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_33](ltc0_pkn_33) module"]
pub type LTC0_PKN_33 = crate::Reg<u32, _LTC0_PKN_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_33;
#[doc = "`read()` method returns [ltc0_pkn_33::R](ltc0_pkn_33::R) reader structure"]
impl crate::Readable for LTC0_PKN_33 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_33::W](ltc0_pkn_33::W) writer structure"]
impl crate::Writable for LTC0_PKN_33 {}
#[doc = "LTC PKHA N 33 Register"]
pub mod ltc0_pkn_33;
#[doc = "LTC PKHA N2 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_2](ltc0_pkn2_2) module"]
pub type LTC0_PKN2_2 = crate::Reg<u32, _LTC0_PKN2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_2;
#[doc = "`read()` method returns [ltc0_pkn2_2::R](ltc0_pkn2_2::R) reader structure"]
impl crate::Readable for LTC0_PKN2_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_2::W](ltc0_pkn2_2::W) writer structure"]
impl crate::Writable for LTC0_PKN2_2 {}
#[doc = "LTC PKHA N2 2 Register"]
pub mod ltc0_pkn2_2;
#[doc = "LTC PKHA N 34 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_34](ltc0_pkn_34) module"]
pub type LTC0_PKN_34 = crate::Reg<u32, _LTC0_PKN_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_34;
#[doc = "`read()` method returns [ltc0_pkn_34::R](ltc0_pkn_34::R) reader structure"]
impl crate::Readable for LTC0_PKN_34 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_34::W](ltc0_pkn_34::W) writer structure"]
impl crate::Writable for LTC0_PKN_34 {}
#[doc = "LTC PKHA N 34 Register"]
pub mod ltc0_pkn_34;
#[doc = "LTC PKHA N2 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_3](ltc0_pkn2_3) module"]
pub type LTC0_PKN2_3 = crate::Reg<u32, _LTC0_PKN2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_3;
#[doc = "`read()` method returns [ltc0_pkn2_3::R](ltc0_pkn2_3::R) reader structure"]
impl crate::Readable for LTC0_PKN2_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_3::W](ltc0_pkn2_3::W) writer structure"]
impl crate::Writable for LTC0_PKN2_3 {}
#[doc = "LTC PKHA N2 3 Register"]
pub mod ltc0_pkn2_3;
#[doc = "LTC PKHA N 35 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_35](ltc0_pkn_35) module"]
pub type LTC0_PKN_35 = crate::Reg<u32, _LTC0_PKN_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_35;
#[doc = "`read()` method returns [ltc0_pkn_35::R](ltc0_pkn_35::R) reader structure"]
impl crate::Readable for LTC0_PKN_35 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_35::W](ltc0_pkn_35::W) writer structure"]
impl crate::Writable for LTC0_PKN_35 {}
#[doc = "LTC PKHA N 35 Register"]
pub mod ltc0_pkn_35;
#[doc = "LTC PKHA N2 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_4](ltc0_pkn2_4) module"]
pub type LTC0_PKN2_4 = crate::Reg<u32, _LTC0_PKN2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_4;
#[doc = "`read()` method returns [ltc0_pkn2_4::R](ltc0_pkn2_4::R) reader structure"]
impl crate::Readable for LTC0_PKN2_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_4::W](ltc0_pkn2_4::W) writer structure"]
impl crate::Writable for LTC0_PKN2_4 {}
#[doc = "LTC PKHA N2 4 Register"]
pub mod ltc0_pkn2_4;
#[doc = "LTC PKHA N 36 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_36](ltc0_pkn_36) module"]
pub type LTC0_PKN_36 = crate::Reg<u32, _LTC0_PKN_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_36;
#[doc = "`read()` method returns [ltc0_pkn_36::R](ltc0_pkn_36::R) reader structure"]
impl crate::Readable for LTC0_PKN_36 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_36::W](ltc0_pkn_36::W) writer structure"]
impl crate::Writable for LTC0_PKN_36 {}
#[doc = "LTC PKHA N 36 Register"]
pub mod ltc0_pkn_36;
#[doc = "LTC PKHA N2 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_5](ltc0_pkn2_5) module"]
pub type LTC0_PKN2_5 = crate::Reg<u32, _LTC0_PKN2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_5;
#[doc = "`read()` method returns [ltc0_pkn2_5::R](ltc0_pkn2_5::R) reader structure"]
impl crate::Readable for LTC0_PKN2_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_5::W](ltc0_pkn2_5::W) writer structure"]
impl crate::Writable for LTC0_PKN2_5 {}
#[doc = "LTC PKHA N2 5 Register"]
pub mod ltc0_pkn2_5;
#[doc = "LTC PKHA N 37 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_37](ltc0_pkn_37) module"]
pub type LTC0_PKN_37 = crate::Reg<u32, _LTC0_PKN_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_37;
#[doc = "`read()` method returns [ltc0_pkn_37::R](ltc0_pkn_37::R) reader structure"]
impl crate::Readable for LTC0_PKN_37 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_37::W](ltc0_pkn_37::W) writer structure"]
impl crate::Writable for LTC0_PKN_37 {}
#[doc = "LTC PKHA N 37 Register"]
pub mod ltc0_pkn_37;
#[doc = "LTC PKHA N2 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_6](ltc0_pkn2_6) module"]
pub type LTC0_PKN2_6 = crate::Reg<u32, _LTC0_PKN2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_6;
#[doc = "`read()` method returns [ltc0_pkn2_6::R](ltc0_pkn2_6::R) reader structure"]
impl crate::Readable for LTC0_PKN2_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_6::W](ltc0_pkn2_6::W) writer structure"]
impl crate::Writable for LTC0_PKN2_6 {}
#[doc = "LTC PKHA N2 6 Register"]
pub mod ltc0_pkn2_6;
#[doc = "LTC PKHA N 38 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_38](ltc0_pkn_38) module"]
pub type LTC0_PKN_38 = crate::Reg<u32, _LTC0_PKN_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_38;
#[doc = "`read()` method returns [ltc0_pkn_38::R](ltc0_pkn_38::R) reader structure"]
impl crate::Readable for LTC0_PKN_38 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_38::W](ltc0_pkn_38::W) writer structure"]
impl crate::Writable for LTC0_PKN_38 {}
#[doc = "LTC PKHA N 38 Register"]
pub mod ltc0_pkn_38;
#[doc = "LTC PKHA N2 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_7](ltc0_pkn2_7) module"]
pub type LTC0_PKN2_7 = crate::Reg<u32, _LTC0_PKN2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_7;
#[doc = "`read()` method returns [ltc0_pkn2_7::R](ltc0_pkn2_7::R) reader structure"]
impl crate::Readable for LTC0_PKN2_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_7::W](ltc0_pkn2_7::W) writer structure"]
impl crate::Writable for LTC0_PKN2_7 {}
#[doc = "LTC PKHA N2 7 Register"]
pub mod ltc0_pkn2_7;
#[doc = "LTC PKHA N 39 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_39](ltc0_pkn_39) module"]
pub type LTC0_PKN_39 = crate::Reg<u32, _LTC0_PKN_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_39;
#[doc = "`read()` method returns [ltc0_pkn_39::R](ltc0_pkn_39::R) reader structure"]
impl crate::Readable for LTC0_PKN_39 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_39::W](ltc0_pkn_39::W) writer structure"]
impl crate::Writable for LTC0_PKN_39 {}
#[doc = "LTC PKHA N 39 Register"]
pub mod ltc0_pkn_39;
#[doc = "LTC PKHA N2 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_8](ltc0_pkn2_8) module"]
pub type LTC0_PKN2_8 = crate::Reg<u32, _LTC0_PKN2_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_8;
#[doc = "`read()` method returns [ltc0_pkn2_8::R](ltc0_pkn2_8::R) reader structure"]
impl crate::Readable for LTC0_PKN2_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_8::W](ltc0_pkn2_8::W) writer structure"]
impl crate::Writable for LTC0_PKN2_8 {}
#[doc = "LTC PKHA N2 8 Register"]
pub mod ltc0_pkn2_8;
#[doc = "LTC PKHA N 40 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_40](ltc0_pkn_40) module"]
pub type LTC0_PKN_40 = crate::Reg<u32, _LTC0_PKN_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_40;
#[doc = "`read()` method returns [ltc0_pkn_40::R](ltc0_pkn_40::R) reader structure"]
impl crate::Readable for LTC0_PKN_40 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_40::W](ltc0_pkn_40::W) writer structure"]
impl crate::Writable for LTC0_PKN_40 {}
#[doc = "LTC PKHA N 40 Register"]
pub mod ltc0_pkn_40;
#[doc = "LTC PKHA N2 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_9](ltc0_pkn2_9) module"]
pub type LTC0_PKN2_9 = crate::Reg<u32, _LTC0_PKN2_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_9;
#[doc = "`read()` method returns [ltc0_pkn2_9::R](ltc0_pkn2_9::R) reader structure"]
impl crate::Readable for LTC0_PKN2_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_9::W](ltc0_pkn2_9::W) writer structure"]
impl crate::Writable for LTC0_PKN2_9 {}
#[doc = "LTC PKHA N2 9 Register"]
pub mod ltc0_pkn2_9;
#[doc = "LTC PKHA N 41 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_41](ltc0_pkn_41) module"]
pub type LTC0_PKN_41 = crate::Reg<u32, _LTC0_PKN_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_41;
#[doc = "`read()` method returns [ltc0_pkn_41::R](ltc0_pkn_41::R) reader structure"]
impl crate::Readable for LTC0_PKN_41 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_41::W](ltc0_pkn_41::W) writer structure"]
impl crate::Writable for LTC0_PKN_41 {}
#[doc = "LTC PKHA N 41 Register"]
pub mod ltc0_pkn_41;
#[doc = "LTC PKHA N2 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_10](ltc0_pkn2_10) module"]
pub type LTC0_PKN2_10 = crate::Reg<u32, _LTC0_PKN2_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_10;
#[doc = "`read()` method returns [ltc0_pkn2_10::R](ltc0_pkn2_10::R) reader structure"]
impl crate::Readable for LTC0_PKN2_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_10::W](ltc0_pkn2_10::W) writer structure"]
impl crate::Writable for LTC0_PKN2_10 {}
#[doc = "LTC PKHA N2 10 Register"]
pub mod ltc0_pkn2_10;
#[doc = "LTC PKHA N 42 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_42](ltc0_pkn_42) module"]
pub type LTC0_PKN_42 = crate::Reg<u32, _LTC0_PKN_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_42;
#[doc = "`read()` method returns [ltc0_pkn_42::R](ltc0_pkn_42::R) reader structure"]
impl crate::Readable for LTC0_PKN_42 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_42::W](ltc0_pkn_42::W) writer structure"]
impl crate::Writable for LTC0_PKN_42 {}
#[doc = "LTC PKHA N 42 Register"]
pub mod ltc0_pkn_42;
#[doc = "LTC PKHA N2 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_11](ltc0_pkn2_11) module"]
pub type LTC0_PKN2_11 = crate::Reg<u32, _LTC0_PKN2_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_11;
#[doc = "`read()` method returns [ltc0_pkn2_11::R](ltc0_pkn2_11::R) reader structure"]
impl crate::Readable for LTC0_PKN2_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_11::W](ltc0_pkn2_11::W) writer structure"]
impl crate::Writable for LTC0_PKN2_11 {}
#[doc = "LTC PKHA N2 11 Register"]
pub mod ltc0_pkn2_11;
#[doc = "LTC PKHA N 43 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_43](ltc0_pkn_43) module"]
pub type LTC0_PKN_43 = crate::Reg<u32, _LTC0_PKN_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_43;
#[doc = "`read()` method returns [ltc0_pkn_43::R](ltc0_pkn_43::R) reader structure"]
impl crate::Readable for LTC0_PKN_43 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_43::W](ltc0_pkn_43::W) writer structure"]
impl crate::Writable for LTC0_PKN_43 {}
#[doc = "LTC PKHA N 43 Register"]
pub mod ltc0_pkn_43;
#[doc = "LTC PKHA N2 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_12](ltc0_pkn2_12) module"]
pub type LTC0_PKN2_12 = crate::Reg<u32, _LTC0_PKN2_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_12;
#[doc = "`read()` method returns [ltc0_pkn2_12::R](ltc0_pkn2_12::R) reader structure"]
impl crate::Readable for LTC0_PKN2_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_12::W](ltc0_pkn2_12::W) writer structure"]
impl crate::Writable for LTC0_PKN2_12 {}
#[doc = "LTC PKHA N2 12 Register"]
pub mod ltc0_pkn2_12;
#[doc = "LTC PKHA N 44 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_44](ltc0_pkn_44) module"]
pub type LTC0_PKN_44 = crate::Reg<u32, _LTC0_PKN_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_44;
#[doc = "`read()` method returns [ltc0_pkn_44::R](ltc0_pkn_44::R) reader structure"]
impl crate::Readable for LTC0_PKN_44 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_44::W](ltc0_pkn_44::W) writer structure"]
impl crate::Writable for LTC0_PKN_44 {}
#[doc = "LTC PKHA N 44 Register"]
pub mod ltc0_pkn_44;
#[doc = "LTC PKHA N2 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_13](ltc0_pkn2_13) module"]
pub type LTC0_PKN2_13 = crate::Reg<u32, _LTC0_PKN2_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_13;
#[doc = "`read()` method returns [ltc0_pkn2_13::R](ltc0_pkn2_13::R) reader structure"]
impl crate::Readable for LTC0_PKN2_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_13::W](ltc0_pkn2_13::W) writer structure"]
impl crate::Writable for LTC0_PKN2_13 {}
#[doc = "LTC PKHA N2 13 Register"]
pub mod ltc0_pkn2_13;
#[doc = "LTC PKHA N 45 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_45](ltc0_pkn_45) module"]
pub type LTC0_PKN_45 = crate::Reg<u32, _LTC0_PKN_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_45;
#[doc = "`read()` method returns [ltc0_pkn_45::R](ltc0_pkn_45::R) reader structure"]
impl crate::Readable for LTC0_PKN_45 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_45::W](ltc0_pkn_45::W) writer structure"]
impl crate::Writable for LTC0_PKN_45 {}
#[doc = "LTC PKHA N 45 Register"]
pub mod ltc0_pkn_45;
#[doc = "LTC PKHA N2 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_14](ltc0_pkn2_14) module"]
pub type LTC0_PKN2_14 = crate::Reg<u32, _LTC0_PKN2_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_14;
#[doc = "`read()` method returns [ltc0_pkn2_14::R](ltc0_pkn2_14::R) reader structure"]
impl crate::Readable for LTC0_PKN2_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_14::W](ltc0_pkn2_14::W) writer structure"]
impl crate::Writable for LTC0_PKN2_14 {}
#[doc = "LTC PKHA N2 14 Register"]
pub mod ltc0_pkn2_14;
#[doc = "LTC PKHA N 46 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_46](ltc0_pkn_46) module"]
pub type LTC0_PKN_46 = crate::Reg<u32, _LTC0_PKN_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_46;
#[doc = "`read()` method returns [ltc0_pkn_46::R](ltc0_pkn_46::R) reader structure"]
impl crate::Readable for LTC0_PKN_46 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_46::W](ltc0_pkn_46::W) writer structure"]
impl crate::Writable for LTC0_PKN_46 {}
#[doc = "LTC PKHA N 46 Register"]
pub mod ltc0_pkn_46;
#[doc = "LTC PKHA N2 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn2_15](ltc0_pkn2_15) module"]
pub type LTC0_PKN2_15 = crate::Reg<u32, _LTC0_PKN2_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN2_15;
#[doc = "`read()` method returns [ltc0_pkn2_15::R](ltc0_pkn2_15::R) reader structure"]
impl crate::Readable for LTC0_PKN2_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn2_15::W](ltc0_pkn2_15::W) writer structure"]
impl crate::Writable for LTC0_PKN2_15 {}
#[doc = "LTC PKHA N2 15 Register"]
pub mod ltc0_pkn2_15;
#[doc = "LTC PKHA N 47 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_47](ltc0_pkn_47) module"]
pub type LTC0_PKN_47 = crate::Reg<u32, _LTC0_PKN_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_47;
#[doc = "`read()` method returns [ltc0_pkn_47::R](ltc0_pkn_47::R) reader structure"]
impl crate::Readable for LTC0_PKN_47 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_47::W](ltc0_pkn_47::W) writer structure"]
impl crate::Writable for LTC0_PKN_47 {}
#[doc = "LTC PKHA N 47 Register"]
pub mod ltc0_pkn_47;
#[doc = "LTC PKHA N3 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_0](ltc0_pkn3_0) module"]
pub type LTC0_PKN3_0 = crate::Reg<u32, _LTC0_PKN3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_0;
#[doc = "`read()` method returns [ltc0_pkn3_0::R](ltc0_pkn3_0::R) reader structure"]
impl crate::Readable for LTC0_PKN3_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_0::W](ltc0_pkn3_0::W) writer structure"]
impl crate::Writable for LTC0_PKN3_0 {}
#[doc = "LTC PKHA N3 0 Register"]
pub mod ltc0_pkn3_0;
#[doc = "LTC PKHA N 48 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_48](ltc0_pkn_48) module"]
pub type LTC0_PKN_48 = crate::Reg<u32, _LTC0_PKN_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_48;
#[doc = "`read()` method returns [ltc0_pkn_48::R](ltc0_pkn_48::R) reader structure"]
impl crate::Readable for LTC0_PKN_48 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_48::W](ltc0_pkn_48::W) writer structure"]
impl crate::Writable for LTC0_PKN_48 {}
#[doc = "LTC PKHA N 48 Register"]
pub mod ltc0_pkn_48;
#[doc = "LTC PKHA N3 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_1](ltc0_pkn3_1) module"]
pub type LTC0_PKN3_1 = crate::Reg<u32, _LTC0_PKN3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_1;
#[doc = "`read()` method returns [ltc0_pkn3_1::R](ltc0_pkn3_1::R) reader structure"]
impl crate::Readable for LTC0_PKN3_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_1::W](ltc0_pkn3_1::W) writer structure"]
impl crate::Writable for LTC0_PKN3_1 {}
#[doc = "LTC PKHA N3 1 Register"]
pub mod ltc0_pkn3_1;
#[doc = "LTC PKHA N 49 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_49](ltc0_pkn_49) module"]
pub type LTC0_PKN_49 = crate::Reg<u32, _LTC0_PKN_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_49;
#[doc = "`read()` method returns [ltc0_pkn_49::R](ltc0_pkn_49::R) reader structure"]
impl crate::Readable for LTC0_PKN_49 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_49::W](ltc0_pkn_49::W) writer structure"]
impl crate::Writable for LTC0_PKN_49 {}
#[doc = "LTC PKHA N 49 Register"]
pub mod ltc0_pkn_49;
#[doc = "LTC PKHA N3 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_2](ltc0_pkn3_2) module"]
pub type LTC0_PKN3_2 = crate::Reg<u32, _LTC0_PKN3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_2;
#[doc = "`read()` method returns [ltc0_pkn3_2::R](ltc0_pkn3_2::R) reader structure"]
impl crate::Readable for LTC0_PKN3_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_2::W](ltc0_pkn3_2::W) writer structure"]
impl crate::Writable for LTC0_PKN3_2 {}
#[doc = "LTC PKHA N3 2 Register"]
pub mod ltc0_pkn3_2;
#[doc = "LTC PKHA N 50 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_50](ltc0_pkn_50) module"]
pub type LTC0_PKN_50 = crate::Reg<u32, _LTC0_PKN_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_50;
#[doc = "`read()` method returns [ltc0_pkn_50::R](ltc0_pkn_50::R) reader structure"]
impl crate::Readable for LTC0_PKN_50 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_50::W](ltc0_pkn_50::W) writer structure"]
impl crate::Writable for LTC0_PKN_50 {}
#[doc = "LTC PKHA N 50 Register"]
pub mod ltc0_pkn_50;
#[doc = "LTC PKHA N3 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_3](ltc0_pkn3_3) module"]
pub type LTC0_PKN3_3 = crate::Reg<u32, _LTC0_PKN3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_3;
#[doc = "`read()` method returns [ltc0_pkn3_3::R](ltc0_pkn3_3::R) reader structure"]
impl crate::Readable for LTC0_PKN3_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_3::W](ltc0_pkn3_3::W) writer structure"]
impl crate::Writable for LTC0_PKN3_3 {}
#[doc = "LTC PKHA N3 3 Register"]
pub mod ltc0_pkn3_3;
#[doc = "LTC PKHA N 51 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_51](ltc0_pkn_51) module"]
pub type LTC0_PKN_51 = crate::Reg<u32, _LTC0_PKN_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_51;
#[doc = "`read()` method returns [ltc0_pkn_51::R](ltc0_pkn_51::R) reader structure"]
impl crate::Readable for LTC0_PKN_51 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_51::W](ltc0_pkn_51::W) writer structure"]
impl crate::Writable for LTC0_PKN_51 {}
#[doc = "LTC PKHA N 51 Register"]
pub mod ltc0_pkn_51;
#[doc = "LTC PKHA N3 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_4](ltc0_pkn3_4) module"]
pub type LTC0_PKN3_4 = crate::Reg<u32, _LTC0_PKN3_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_4;
#[doc = "`read()` method returns [ltc0_pkn3_4::R](ltc0_pkn3_4::R) reader structure"]
impl crate::Readable for LTC0_PKN3_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_4::W](ltc0_pkn3_4::W) writer structure"]
impl crate::Writable for LTC0_PKN3_4 {}
#[doc = "LTC PKHA N3 4 Register"]
pub mod ltc0_pkn3_4;
#[doc = "LTC PKHA N 52 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_52](ltc0_pkn_52) module"]
pub type LTC0_PKN_52 = crate::Reg<u32, _LTC0_PKN_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_52;
#[doc = "`read()` method returns [ltc0_pkn_52::R](ltc0_pkn_52::R) reader structure"]
impl crate::Readable for LTC0_PKN_52 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_52::W](ltc0_pkn_52::W) writer structure"]
impl crate::Writable for LTC0_PKN_52 {}
#[doc = "LTC PKHA N 52 Register"]
pub mod ltc0_pkn_52;
#[doc = "LTC PKHA N3 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_5](ltc0_pkn3_5) module"]
pub type LTC0_PKN3_5 = crate::Reg<u32, _LTC0_PKN3_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_5;
#[doc = "`read()` method returns [ltc0_pkn3_5::R](ltc0_pkn3_5::R) reader structure"]
impl crate::Readable for LTC0_PKN3_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_5::W](ltc0_pkn3_5::W) writer structure"]
impl crate::Writable for LTC0_PKN3_5 {}
#[doc = "LTC PKHA N3 5 Register"]
pub mod ltc0_pkn3_5;
#[doc = "LTC PKHA N 53 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_53](ltc0_pkn_53) module"]
pub type LTC0_PKN_53 = crate::Reg<u32, _LTC0_PKN_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_53;
#[doc = "`read()` method returns [ltc0_pkn_53::R](ltc0_pkn_53::R) reader structure"]
impl crate::Readable for LTC0_PKN_53 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_53::W](ltc0_pkn_53::W) writer structure"]
impl crate::Writable for LTC0_PKN_53 {}
#[doc = "LTC PKHA N 53 Register"]
pub mod ltc0_pkn_53;
#[doc = "LTC PKHA N3 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_6](ltc0_pkn3_6) module"]
pub type LTC0_PKN3_6 = crate::Reg<u32, _LTC0_PKN3_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_6;
#[doc = "`read()` method returns [ltc0_pkn3_6::R](ltc0_pkn3_6::R) reader structure"]
impl crate::Readable for LTC0_PKN3_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_6::W](ltc0_pkn3_6::W) writer structure"]
impl crate::Writable for LTC0_PKN3_6 {}
#[doc = "LTC PKHA N3 6 Register"]
pub mod ltc0_pkn3_6;
#[doc = "LTC PKHA N 54 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_54](ltc0_pkn_54) module"]
pub type LTC0_PKN_54 = crate::Reg<u32, _LTC0_PKN_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_54;
#[doc = "`read()` method returns [ltc0_pkn_54::R](ltc0_pkn_54::R) reader structure"]
impl crate::Readable for LTC0_PKN_54 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_54::W](ltc0_pkn_54::W) writer structure"]
impl crate::Writable for LTC0_PKN_54 {}
#[doc = "LTC PKHA N 54 Register"]
pub mod ltc0_pkn_54;
#[doc = "LTC PKHA N3 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_7](ltc0_pkn3_7) module"]
pub type LTC0_PKN3_7 = crate::Reg<u32, _LTC0_PKN3_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_7;
#[doc = "`read()` method returns [ltc0_pkn3_7::R](ltc0_pkn3_7::R) reader structure"]
impl crate::Readable for LTC0_PKN3_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_7::W](ltc0_pkn3_7::W) writer structure"]
impl crate::Writable for LTC0_PKN3_7 {}
#[doc = "LTC PKHA N3 7 Register"]
pub mod ltc0_pkn3_7;
#[doc = "LTC PKHA N 55 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_55](ltc0_pkn_55) module"]
pub type LTC0_PKN_55 = crate::Reg<u32, _LTC0_PKN_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_55;
#[doc = "`read()` method returns [ltc0_pkn_55::R](ltc0_pkn_55::R) reader structure"]
impl crate::Readable for LTC0_PKN_55 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_55::W](ltc0_pkn_55::W) writer structure"]
impl crate::Writable for LTC0_PKN_55 {}
#[doc = "LTC PKHA N 55 Register"]
pub mod ltc0_pkn_55;
#[doc = "LTC PKHA N3 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_8](ltc0_pkn3_8) module"]
pub type LTC0_PKN3_8 = crate::Reg<u32, _LTC0_PKN3_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_8;
#[doc = "`read()` method returns [ltc0_pkn3_8::R](ltc0_pkn3_8::R) reader structure"]
impl crate::Readable for LTC0_PKN3_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_8::W](ltc0_pkn3_8::W) writer structure"]
impl crate::Writable for LTC0_PKN3_8 {}
#[doc = "LTC PKHA N3 8 Register"]
pub mod ltc0_pkn3_8;
#[doc = "LTC PKHA N 56 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_56](ltc0_pkn_56) module"]
pub type LTC0_PKN_56 = crate::Reg<u32, _LTC0_PKN_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_56;
#[doc = "`read()` method returns [ltc0_pkn_56::R](ltc0_pkn_56::R) reader structure"]
impl crate::Readable for LTC0_PKN_56 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_56::W](ltc0_pkn_56::W) writer structure"]
impl crate::Writable for LTC0_PKN_56 {}
#[doc = "LTC PKHA N 56 Register"]
pub mod ltc0_pkn_56;
#[doc = "LTC PKHA N3 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_9](ltc0_pkn3_9) module"]
pub type LTC0_PKN3_9 = crate::Reg<u32, _LTC0_PKN3_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_9;
#[doc = "`read()` method returns [ltc0_pkn3_9::R](ltc0_pkn3_9::R) reader structure"]
impl crate::Readable for LTC0_PKN3_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_9::W](ltc0_pkn3_9::W) writer structure"]
impl crate::Writable for LTC0_PKN3_9 {}
#[doc = "LTC PKHA N3 9 Register"]
pub mod ltc0_pkn3_9;
#[doc = "LTC PKHA N 57 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_57](ltc0_pkn_57) module"]
pub type LTC0_PKN_57 = crate::Reg<u32, _LTC0_PKN_57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_57;
#[doc = "`read()` method returns [ltc0_pkn_57::R](ltc0_pkn_57::R) reader structure"]
impl crate::Readable for LTC0_PKN_57 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_57::W](ltc0_pkn_57::W) writer structure"]
impl crate::Writable for LTC0_PKN_57 {}
#[doc = "LTC PKHA N 57 Register"]
pub mod ltc0_pkn_57;
#[doc = "LTC PKHA N3 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_10](ltc0_pkn3_10) module"]
pub type LTC0_PKN3_10 = crate::Reg<u32, _LTC0_PKN3_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_10;
#[doc = "`read()` method returns [ltc0_pkn3_10::R](ltc0_pkn3_10::R) reader structure"]
impl crate::Readable for LTC0_PKN3_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_10::W](ltc0_pkn3_10::W) writer structure"]
impl crate::Writable for LTC0_PKN3_10 {}
#[doc = "LTC PKHA N3 10 Register"]
pub mod ltc0_pkn3_10;
#[doc = "LTC PKHA N 58 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_58](ltc0_pkn_58) module"]
pub type LTC0_PKN_58 = crate::Reg<u32, _LTC0_PKN_58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_58;
#[doc = "`read()` method returns [ltc0_pkn_58::R](ltc0_pkn_58::R) reader structure"]
impl crate::Readable for LTC0_PKN_58 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_58::W](ltc0_pkn_58::W) writer structure"]
impl crate::Writable for LTC0_PKN_58 {}
#[doc = "LTC PKHA N 58 Register"]
pub mod ltc0_pkn_58;
#[doc = "LTC PKHA N3 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_11](ltc0_pkn3_11) module"]
pub type LTC0_PKN3_11 = crate::Reg<u32, _LTC0_PKN3_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_11;
#[doc = "`read()` method returns [ltc0_pkn3_11::R](ltc0_pkn3_11::R) reader structure"]
impl crate::Readable for LTC0_PKN3_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_11::W](ltc0_pkn3_11::W) writer structure"]
impl crate::Writable for LTC0_PKN3_11 {}
#[doc = "LTC PKHA N3 11 Register"]
pub mod ltc0_pkn3_11;
#[doc = "LTC PKHA N 59 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_59](ltc0_pkn_59) module"]
pub type LTC0_PKN_59 = crate::Reg<u32, _LTC0_PKN_59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_59;
#[doc = "`read()` method returns [ltc0_pkn_59::R](ltc0_pkn_59::R) reader structure"]
impl crate::Readable for LTC0_PKN_59 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_59::W](ltc0_pkn_59::W) writer structure"]
impl crate::Writable for LTC0_PKN_59 {}
#[doc = "LTC PKHA N 59 Register"]
pub mod ltc0_pkn_59;
#[doc = "LTC PKHA N3 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_12](ltc0_pkn3_12) module"]
pub type LTC0_PKN3_12 = crate::Reg<u32, _LTC0_PKN3_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_12;
#[doc = "`read()` method returns [ltc0_pkn3_12::R](ltc0_pkn3_12::R) reader structure"]
impl crate::Readable for LTC0_PKN3_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_12::W](ltc0_pkn3_12::W) writer structure"]
impl crate::Writable for LTC0_PKN3_12 {}
#[doc = "LTC PKHA N3 12 Register"]
pub mod ltc0_pkn3_12;
#[doc = "LTC PKHA N 60 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_60](ltc0_pkn_60) module"]
pub type LTC0_PKN_60 = crate::Reg<u32, _LTC0_PKN_60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_60;
#[doc = "`read()` method returns [ltc0_pkn_60::R](ltc0_pkn_60::R) reader structure"]
impl crate::Readable for LTC0_PKN_60 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_60::W](ltc0_pkn_60::W) writer structure"]
impl crate::Writable for LTC0_PKN_60 {}
#[doc = "LTC PKHA N 60 Register"]
pub mod ltc0_pkn_60;
#[doc = "LTC PKHA N3 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_13](ltc0_pkn3_13) module"]
pub type LTC0_PKN3_13 = crate::Reg<u32, _LTC0_PKN3_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_13;
#[doc = "`read()` method returns [ltc0_pkn3_13::R](ltc0_pkn3_13::R) reader structure"]
impl crate::Readable for LTC0_PKN3_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_13::W](ltc0_pkn3_13::W) writer structure"]
impl crate::Writable for LTC0_PKN3_13 {}
#[doc = "LTC PKHA N3 13 Register"]
pub mod ltc0_pkn3_13;
#[doc = "LTC PKHA N 61 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_61](ltc0_pkn_61) module"]
pub type LTC0_PKN_61 = crate::Reg<u32, _LTC0_PKN_61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_61;
#[doc = "`read()` method returns [ltc0_pkn_61::R](ltc0_pkn_61::R) reader structure"]
impl crate::Readable for LTC0_PKN_61 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_61::W](ltc0_pkn_61::W) writer structure"]
impl crate::Writable for LTC0_PKN_61 {}
#[doc = "LTC PKHA N 61 Register"]
pub mod ltc0_pkn_61;
#[doc = "LTC PKHA N3 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_14](ltc0_pkn3_14) module"]
pub type LTC0_PKN3_14 = crate::Reg<u32, _LTC0_PKN3_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_14;
#[doc = "`read()` method returns [ltc0_pkn3_14::R](ltc0_pkn3_14::R) reader structure"]
impl crate::Readable for LTC0_PKN3_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_14::W](ltc0_pkn3_14::W) writer structure"]
impl crate::Writable for LTC0_PKN3_14 {}
#[doc = "LTC PKHA N3 14 Register"]
pub mod ltc0_pkn3_14;
#[doc = "LTC PKHA N 62 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_62](ltc0_pkn_62) module"]
pub type LTC0_PKN_62 = crate::Reg<u32, _LTC0_PKN_62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_62;
#[doc = "`read()` method returns [ltc0_pkn_62::R](ltc0_pkn_62::R) reader structure"]
impl crate::Readable for LTC0_PKN_62 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_62::W](ltc0_pkn_62::W) writer structure"]
impl crate::Writable for LTC0_PKN_62 {}
#[doc = "LTC PKHA N 62 Register"]
pub mod ltc0_pkn_62;
#[doc = "LTC PKHA N3 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn3_15](ltc0_pkn3_15) module"]
pub type LTC0_PKN3_15 = crate::Reg<u32, _LTC0_PKN3_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN3_15;
#[doc = "`read()` method returns [ltc0_pkn3_15::R](ltc0_pkn3_15::R) reader structure"]
impl crate::Readable for LTC0_PKN3_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn3_15::W](ltc0_pkn3_15::W) writer structure"]
impl crate::Writable for LTC0_PKN3_15 {}
#[doc = "LTC PKHA N3 15 Register"]
pub mod ltc0_pkn3_15;
#[doc = "LTC PKHA N 63 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pkn_63](ltc0_pkn_63) module"]
pub type LTC0_PKN_63 = crate::Reg<u32, _LTC0_PKN_63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKN_63;
#[doc = "`read()` method returns [ltc0_pkn_63::R](ltc0_pkn_63::R) reader structure"]
impl crate::Readable for LTC0_PKN_63 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pkn_63::W](ltc0_pkn_63::W) writer structure"]
impl crate::Writable for LTC0_PKN_63 {}
#[doc = "LTC PKHA N 63 Register"]
pub mod ltc0_pkn_63;
#[doc = "LTC PKHA E0 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_0](ltc0_pke0_0) module"]
pub type LTC0_PKE0_0 = crate::Reg<u32, _LTC0_PKE0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_0;
#[doc = "`read()` method returns [ltc0_pke0_0::R](ltc0_pke0_0::R) reader structure"]
impl crate::Readable for LTC0_PKE0_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_0::W](ltc0_pke0_0::W) writer structure"]
impl crate::Writable for LTC0_PKE0_0 {}
#[doc = "LTC PKHA E0 0 Register"]
pub mod ltc0_pke0_0;
#[doc = "LTC PKHA E 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_0](ltc0_pke_0) module"]
pub type LTC0_PKE_0 = crate::Reg<u32, _LTC0_PKE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_0;
#[doc = "`read()` method returns [ltc0_pke_0::R](ltc0_pke_0::R) reader structure"]
impl crate::Readable for LTC0_PKE_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_0::W](ltc0_pke_0::W) writer structure"]
impl crate::Writable for LTC0_PKE_0 {}
#[doc = "LTC PKHA E 0 Register"]
pub mod ltc0_pke_0;
#[doc = "LTC PKHA E0 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_1](ltc0_pke0_1) module"]
pub type LTC0_PKE0_1 = crate::Reg<u32, _LTC0_PKE0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_1;
#[doc = "`read()` method returns [ltc0_pke0_1::R](ltc0_pke0_1::R) reader structure"]
impl crate::Readable for LTC0_PKE0_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_1::W](ltc0_pke0_1::W) writer structure"]
impl crate::Writable for LTC0_PKE0_1 {}
#[doc = "LTC PKHA E0 1 Register"]
pub mod ltc0_pke0_1;
#[doc = "LTC PKHA E 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_1](ltc0_pke_1) module"]
pub type LTC0_PKE_1 = crate::Reg<u32, _LTC0_PKE_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_1;
#[doc = "`read()` method returns [ltc0_pke_1::R](ltc0_pke_1::R) reader structure"]
impl crate::Readable for LTC0_PKE_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_1::W](ltc0_pke_1::W) writer structure"]
impl crate::Writable for LTC0_PKE_1 {}
#[doc = "LTC PKHA E 1 Register"]
pub mod ltc0_pke_1;
#[doc = "LTC PKHA E0 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_2](ltc0_pke0_2) module"]
pub type LTC0_PKE0_2 = crate::Reg<u32, _LTC0_PKE0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_2;
#[doc = "`read()` method returns [ltc0_pke0_2::R](ltc0_pke0_2::R) reader structure"]
impl crate::Readable for LTC0_PKE0_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_2::W](ltc0_pke0_2::W) writer structure"]
impl crate::Writable for LTC0_PKE0_2 {}
#[doc = "LTC PKHA E0 2 Register"]
pub mod ltc0_pke0_2;
#[doc = "LTC PKHA E 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_2](ltc0_pke_2) module"]
pub type LTC0_PKE_2 = crate::Reg<u32, _LTC0_PKE_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_2;
#[doc = "`read()` method returns [ltc0_pke_2::R](ltc0_pke_2::R) reader structure"]
impl crate::Readable for LTC0_PKE_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_2::W](ltc0_pke_2::W) writer structure"]
impl crate::Writable for LTC0_PKE_2 {}
#[doc = "LTC PKHA E 2 Register"]
pub mod ltc0_pke_2;
#[doc = "LTC PKHA E0 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_3](ltc0_pke0_3) module"]
pub type LTC0_PKE0_3 = crate::Reg<u32, _LTC0_PKE0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_3;
#[doc = "`read()` method returns [ltc0_pke0_3::R](ltc0_pke0_3::R) reader structure"]
impl crate::Readable for LTC0_PKE0_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_3::W](ltc0_pke0_3::W) writer structure"]
impl crate::Writable for LTC0_PKE0_3 {}
#[doc = "LTC PKHA E0 3 Register"]
pub mod ltc0_pke0_3;
#[doc = "LTC PKHA E 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_3](ltc0_pke_3) module"]
pub type LTC0_PKE_3 = crate::Reg<u32, _LTC0_PKE_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_3;
#[doc = "`read()` method returns [ltc0_pke_3::R](ltc0_pke_3::R) reader structure"]
impl crate::Readable for LTC0_PKE_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_3::W](ltc0_pke_3::W) writer structure"]
impl crate::Writable for LTC0_PKE_3 {}
#[doc = "LTC PKHA E 3 Register"]
pub mod ltc0_pke_3;
#[doc = "LTC PKHA E0 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_4](ltc0_pke0_4) module"]
pub type LTC0_PKE0_4 = crate::Reg<u32, _LTC0_PKE0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_4;
#[doc = "`read()` method returns [ltc0_pke0_4::R](ltc0_pke0_4::R) reader structure"]
impl crate::Readable for LTC0_PKE0_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_4::W](ltc0_pke0_4::W) writer structure"]
impl crate::Writable for LTC0_PKE0_4 {}
#[doc = "LTC PKHA E0 4 Register"]
pub mod ltc0_pke0_4;
#[doc = "LTC PKHA E 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_4](ltc0_pke_4) module"]
pub type LTC0_PKE_4 = crate::Reg<u32, _LTC0_PKE_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_4;
#[doc = "`read()` method returns [ltc0_pke_4::R](ltc0_pke_4::R) reader structure"]
impl crate::Readable for LTC0_PKE_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_4::W](ltc0_pke_4::W) writer structure"]
impl crate::Writable for LTC0_PKE_4 {}
#[doc = "LTC PKHA E 4 Register"]
pub mod ltc0_pke_4;
#[doc = "LTC PKHA E0 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_5](ltc0_pke0_5) module"]
pub type LTC0_PKE0_5 = crate::Reg<u32, _LTC0_PKE0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_5;
#[doc = "`read()` method returns [ltc0_pke0_5::R](ltc0_pke0_5::R) reader structure"]
impl crate::Readable for LTC0_PKE0_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_5::W](ltc0_pke0_5::W) writer structure"]
impl crate::Writable for LTC0_PKE0_5 {}
#[doc = "LTC PKHA E0 5 Register"]
pub mod ltc0_pke0_5;
#[doc = "LTC PKHA E 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_5](ltc0_pke_5) module"]
pub type LTC0_PKE_5 = crate::Reg<u32, _LTC0_PKE_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_5;
#[doc = "`read()` method returns [ltc0_pke_5::R](ltc0_pke_5::R) reader structure"]
impl crate::Readable for LTC0_PKE_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_5::W](ltc0_pke_5::W) writer structure"]
impl crate::Writable for LTC0_PKE_5 {}
#[doc = "LTC PKHA E 5 Register"]
pub mod ltc0_pke_5;
#[doc = "LTC PKHA E0 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_6](ltc0_pke0_6) module"]
pub type LTC0_PKE0_6 = crate::Reg<u32, _LTC0_PKE0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_6;
#[doc = "`read()` method returns [ltc0_pke0_6::R](ltc0_pke0_6::R) reader structure"]
impl crate::Readable for LTC0_PKE0_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_6::W](ltc0_pke0_6::W) writer structure"]
impl crate::Writable for LTC0_PKE0_6 {}
#[doc = "LTC PKHA E0 6 Register"]
pub mod ltc0_pke0_6;
#[doc = "LTC PKHA E 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_6](ltc0_pke_6) module"]
pub type LTC0_PKE_6 = crate::Reg<u32, _LTC0_PKE_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_6;
#[doc = "`read()` method returns [ltc0_pke_6::R](ltc0_pke_6::R) reader structure"]
impl crate::Readable for LTC0_PKE_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_6::W](ltc0_pke_6::W) writer structure"]
impl crate::Writable for LTC0_PKE_6 {}
#[doc = "LTC PKHA E 6 Register"]
pub mod ltc0_pke_6;
#[doc = "LTC PKHA E0 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_7](ltc0_pke0_7) module"]
pub type LTC0_PKE0_7 = crate::Reg<u32, _LTC0_PKE0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_7;
#[doc = "`read()` method returns [ltc0_pke0_7::R](ltc0_pke0_7::R) reader structure"]
impl crate::Readable for LTC0_PKE0_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_7::W](ltc0_pke0_7::W) writer structure"]
impl crate::Writable for LTC0_PKE0_7 {}
#[doc = "LTC PKHA E0 7 Register"]
pub mod ltc0_pke0_7;
#[doc = "LTC PKHA E 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_7](ltc0_pke_7) module"]
pub type LTC0_PKE_7 = crate::Reg<u32, _LTC0_PKE_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_7;
#[doc = "`read()` method returns [ltc0_pke_7::R](ltc0_pke_7::R) reader structure"]
impl crate::Readable for LTC0_PKE_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_7::W](ltc0_pke_7::W) writer structure"]
impl crate::Writable for LTC0_PKE_7 {}
#[doc = "LTC PKHA E 7 Register"]
pub mod ltc0_pke_7;
#[doc = "LTC PKHA E0 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_8](ltc0_pke0_8) module"]
pub type LTC0_PKE0_8 = crate::Reg<u32, _LTC0_PKE0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_8;
#[doc = "`read()` method returns [ltc0_pke0_8::R](ltc0_pke0_8::R) reader structure"]
impl crate::Readable for LTC0_PKE0_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_8::W](ltc0_pke0_8::W) writer structure"]
impl crate::Writable for LTC0_PKE0_8 {}
#[doc = "LTC PKHA E0 8 Register"]
pub mod ltc0_pke0_8;
#[doc = "LTC PKHA E 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_8](ltc0_pke_8) module"]
pub type LTC0_PKE_8 = crate::Reg<u32, _LTC0_PKE_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_8;
#[doc = "`read()` method returns [ltc0_pke_8::R](ltc0_pke_8::R) reader structure"]
impl crate::Readable for LTC0_PKE_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_8::W](ltc0_pke_8::W) writer structure"]
impl crate::Writable for LTC0_PKE_8 {}
#[doc = "LTC PKHA E 8 Register"]
pub mod ltc0_pke_8;
#[doc = "LTC PKHA E0 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_9](ltc0_pke0_9) module"]
pub type LTC0_PKE0_9 = crate::Reg<u32, _LTC0_PKE0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_9;
#[doc = "`read()` method returns [ltc0_pke0_9::R](ltc0_pke0_9::R) reader structure"]
impl crate::Readable for LTC0_PKE0_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_9::W](ltc0_pke0_9::W) writer structure"]
impl crate::Writable for LTC0_PKE0_9 {}
#[doc = "LTC PKHA E0 9 Register"]
pub mod ltc0_pke0_9;
#[doc = "LTC PKHA E 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_9](ltc0_pke_9) module"]
pub type LTC0_PKE_9 = crate::Reg<u32, _LTC0_PKE_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_9;
#[doc = "`read()` method returns [ltc0_pke_9::R](ltc0_pke_9::R) reader structure"]
impl crate::Readable for LTC0_PKE_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_9::W](ltc0_pke_9::W) writer structure"]
impl crate::Writable for LTC0_PKE_9 {}
#[doc = "LTC PKHA E 9 Register"]
pub mod ltc0_pke_9;
#[doc = "LTC PKHA E0 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_10](ltc0_pke0_10) module"]
pub type LTC0_PKE0_10 = crate::Reg<u32, _LTC0_PKE0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_10;
#[doc = "`read()` method returns [ltc0_pke0_10::R](ltc0_pke0_10::R) reader structure"]
impl crate::Readable for LTC0_PKE0_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_10::W](ltc0_pke0_10::W) writer structure"]
impl crate::Writable for LTC0_PKE0_10 {}
#[doc = "LTC PKHA E0 10 Register"]
pub mod ltc0_pke0_10;
#[doc = "LTC PKHA E 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_10](ltc0_pke_10) module"]
pub type LTC0_PKE_10 = crate::Reg<u32, _LTC0_PKE_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_10;
#[doc = "`read()` method returns [ltc0_pke_10::R](ltc0_pke_10::R) reader structure"]
impl crate::Readable for LTC0_PKE_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_10::W](ltc0_pke_10::W) writer structure"]
impl crate::Writable for LTC0_PKE_10 {}
#[doc = "LTC PKHA E 10 Register"]
pub mod ltc0_pke_10;
#[doc = "LTC PKHA E0 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_11](ltc0_pke0_11) module"]
pub type LTC0_PKE0_11 = crate::Reg<u32, _LTC0_PKE0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_11;
#[doc = "`read()` method returns [ltc0_pke0_11::R](ltc0_pke0_11::R) reader structure"]
impl crate::Readable for LTC0_PKE0_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_11::W](ltc0_pke0_11::W) writer structure"]
impl crate::Writable for LTC0_PKE0_11 {}
#[doc = "LTC PKHA E0 11 Register"]
pub mod ltc0_pke0_11;
#[doc = "LTC PKHA E 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_11](ltc0_pke_11) module"]
pub type LTC0_PKE_11 = crate::Reg<u32, _LTC0_PKE_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_11;
#[doc = "`read()` method returns [ltc0_pke_11::R](ltc0_pke_11::R) reader structure"]
impl crate::Readable for LTC0_PKE_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_11::W](ltc0_pke_11::W) writer structure"]
impl crate::Writable for LTC0_PKE_11 {}
#[doc = "LTC PKHA E 11 Register"]
pub mod ltc0_pke_11;
#[doc = "LTC PKHA E0 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_12](ltc0_pke0_12) module"]
pub type LTC0_PKE0_12 = crate::Reg<u32, _LTC0_PKE0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_12;
#[doc = "`read()` method returns [ltc0_pke0_12::R](ltc0_pke0_12::R) reader structure"]
impl crate::Readable for LTC0_PKE0_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_12::W](ltc0_pke0_12::W) writer structure"]
impl crate::Writable for LTC0_PKE0_12 {}
#[doc = "LTC PKHA E0 12 Register"]
pub mod ltc0_pke0_12;
#[doc = "LTC PKHA E 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_12](ltc0_pke_12) module"]
pub type LTC0_PKE_12 = crate::Reg<u32, _LTC0_PKE_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_12;
#[doc = "`read()` method returns [ltc0_pke_12::R](ltc0_pke_12::R) reader structure"]
impl crate::Readable for LTC0_PKE_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_12::W](ltc0_pke_12::W) writer structure"]
impl crate::Writable for LTC0_PKE_12 {}
#[doc = "LTC PKHA E 12 Register"]
pub mod ltc0_pke_12;
#[doc = "LTC PKHA E0 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_13](ltc0_pke0_13) module"]
pub type LTC0_PKE0_13 = crate::Reg<u32, _LTC0_PKE0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_13;
#[doc = "`read()` method returns [ltc0_pke0_13::R](ltc0_pke0_13::R) reader structure"]
impl crate::Readable for LTC0_PKE0_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_13::W](ltc0_pke0_13::W) writer structure"]
impl crate::Writable for LTC0_PKE0_13 {}
#[doc = "LTC PKHA E0 13 Register"]
pub mod ltc0_pke0_13;
#[doc = "LTC PKHA E 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_13](ltc0_pke_13) module"]
pub type LTC0_PKE_13 = crate::Reg<u32, _LTC0_PKE_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_13;
#[doc = "`read()` method returns [ltc0_pke_13::R](ltc0_pke_13::R) reader structure"]
impl crate::Readable for LTC0_PKE_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_13::W](ltc0_pke_13::W) writer structure"]
impl crate::Writable for LTC0_PKE_13 {}
#[doc = "LTC PKHA E 13 Register"]
pub mod ltc0_pke_13;
#[doc = "LTC PKHA E0 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_14](ltc0_pke0_14) module"]
pub type LTC0_PKE0_14 = crate::Reg<u32, _LTC0_PKE0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_14;
#[doc = "`read()` method returns [ltc0_pke0_14::R](ltc0_pke0_14::R) reader structure"]
impl crate::Readable for LTC0_PKE0_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_14::W](ltc0_pke0_14::W) writer structure"]
impl crate::Writable for LTC0_PKE0_14 {}
#[doc = "LTC PKHA E0 14 Register"]
pub mod ltc0_pke0_14;
#[doc = "LTC PKHA E 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_14](ltc0_pke_14) module"]
pub type LTC0_PKE_14 = crate::Reg<u32, _LTC0_PKE_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_14;
#[doc = "`read()` method returns [ltc0_pke_14::R](ltc0_pke_14::R) reader structure"]
impl crate::Readable for LTC0_PKE_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_14::W](ltc0_pke_14::W) writer structure"]
impl crate::Writable for LTC0_PKE_14 {}
#[doc = "LTC PKHA E 14 Register"]
pub mod ltc0_pke_14;
#[doc = "LTC PKHA E0 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke0_15](ltc0_pke0_15) module"]
pub type LTC0_PKE0_15 = crate::Reg<u32, _LTC0_PKE0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE0_15;
#[doc = "`read()` method returns [ltc0_pke0_15::R](ltc0_pke0_15::R) reader structure"]
impl crate::Readable for LTC0_PKE0_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke0_15::W](ltc0_pke0_15::W) writer structure"]
impl crate::Writable for LTC0_PKE0_15 {}
#[doc = "LTC PKHA E0 15 Register"]
pub mod ltc0_pke0_15;
#[doc = "LTC PKHA E 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_15](ltc0_pke_15) module"]
pub type LTC0_PKE_15 = crate::Reg<u32, _LTC0_PKE_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_15;
#[doc = "`read()` method returns [ltc0_pke_15::R](ltc0_pke_15::R) reader structure"]
impl crate::Readable for LTC0_PKE_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_15::W](ltc0_pke_15::W) writer structure"]
impl crate::Writable for LTC0_PKE_15 {}
#[doc = "LTC PKHA E 15 Register"]
pub mod ltc0_pke_15;
#[doc = "LTC PKHA E1 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_0](ltc0_pke1_0) module"]
pub type LTC0_PKE1_0 = crate::Reg<u32, _LTC0_PKE1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_0;
#[doc = "`read()` method returns [ltc0_pke1_0::R](ltc0_pke1_0::R) reader structure"]
impl crate::Readable for LTC0_PKE1_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_0::W](ltc0_pke1_0::W) writer structure"]
impl crate::Writable for LTC0_PKE1_0 {}
#[doc = "LTC PKHA E1 0 Register"]
pub mod ltc0_pke1_0;
#[doc = "LTC PKHA E 16 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_16](ltc0_pke_16) module"]
pub type LTC0_PKE_16 = crate::Reg<u32, _LTC0_PKE_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_16;
#[doc = "`read()` method returns [ltc0_pke_16::R](ltc0_pke_16::R) reader structure"]
impl crate::Readable for LTC0_PKE_16 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_16::W](ltc0_pke_16::W) writer structure"]
impl crate::Writable for LTC0_PKE_16 {}
#[doc = "LTC PKHA E 16 Register"]
pub mod ltc0_pke_16;
#[doc = "LTC PKHA E1 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_1](ltc0_pke1_1) module"]
pub type LTC0_PKE1_1 = crate::Reg<u32, _LTC0_PKE1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_1;
#[doc = "`read()` method returns [ltc0_pke1_1::R](ltc0_pke1_1::R) reader structure"]
impl crate::Readable for LTC0_PKE1_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_1::W](ltc0_pke1_1::W) writer structure"]
impl crate::Writable for LTC0_PKE1_1 {}
#[doc = "LTC PKHA E1 1 Register"]
pub mod ltc0_pke1_1;
#[doc = "LTC PKHA E 17 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_17](ltc0_pke_17) module"]
pub type LTC0_PKE_17 = crate::Reg<u32, _LTC0_PKE_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_17;
#[doc = "`read()` method returns [ltc0_pke_17::R](ltc0_pke_17::R) reader structure"]
impl crate::Readable for LTC0_PKE_17 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_17::W](ltc0_pke_17::W) writer structure"]
impl crate::Writable for LTC0_PKE_17 {}
#[doc = "LTC PKHA E 17 Register"]
pub mod ltc0_pke_17;
#[doc = "LTC PKHA E1 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_2](ltc0_pke1_2) module"]
pub type LTC0_PKE1_2 = crate::Reg<u32, _LTC0_PKE1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_2;
#[doc = "`read()` method returns [ltc0_pke1_2::R](ltc0_pke1_2::R) reader structure"]
impl crate::Readable for LTC0_PKE1_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_2::W](ltc0_pke1_2::W) writer structure"]
impl crate::Writable for LTC0_PKE1_2 {}
#[doc = "LTC PKHA E1 2 Register"]
pub mod ltc0_pke1_2;
#[doc = "LTC PKHA E 18 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_18](ltc0_pke_18) module"]
pub type LTC0_PKE_18 = crate::Reg<u32, _LTC0_PKE_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_18;
#[doc = "`read()` method returns [ltc0_pke_18::R](ltc0_pke_18::R) reader structure"]
impl crate::Readable for LTC0_PKE_18 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_18::W](ltc0_pke_18::W) writer structure"]
impl crate::Writable for LTC0_PKE_18 {}
#[doc = "LTC PKHA E 18 Register"]
pub mod ltc0_pke_18;
#[doc = "LTC PKHA E1 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_3](ltc0_pke1_3) module"]
pub type LTC0_PKE1_3 = crate::Reg<u32, _LTC0_PKE1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_3;
#[doc = "`read()` method returns [ltc0_pke1_3::R](ltc0_pke1_3::R) reader structure"]
impl crate::Readable for LTC0_PKE1_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_3::W](ltc0_pke1_3::W) writer structure"]
impl crate::Writable for LTC0_PKE1_3 {}
#[doc = "LTC PKHA E1 3 Register"]
pub mod ltc0_pke1_3;
#[doc = "LTC PKHA E 19 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_19](ltc0_pke_19) module"]
pub type LTC0_PKE_19 = crate::Reg<u32, _LTC0_PKE_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_19;
#[doc = "`read()` method returns [ltc0_pke_19::R](ltc0_pke_19::R) reader structure"]
impl crate::Readable for LTC0_PKE_19 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_19::W](ltc0_pke_19::W) writer structure"]
impl crate::Writable for LTC0_PKE_19 {}
#[doc = "LTC PKHA E 19 Register"]
pub mod ltc0_pke_19;
#[doc = "LTC PKHA E1 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_4](ltc0_pke1_4) module"]
pub type LTC0_PKE1_4 = crate::Reg<u32, _LTC0_PKE1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_4;
#[doc = "`read()` method returns [ltc0_pke1_4::R](ltc0_pke1_4::R) reader structure"]
impl crate::Readable for LTC0_PKE1_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_4::W](ltc0_pke1_4::W) writer structure"]
impl crate::Writable for LTC0_PKE1_4 {}
#[doc = "LTC PKHA E1 4 Register"]
pub mod ltc0_pke1_4;
#[doc = "LTC PKHA E 20 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_20](ltc0_pke_20) module"]
pub type LTC0_PKE_20 = crate::Reg<u32, _LTC0_PKE_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_20;
#[doc = "`read()` method returns [ltc0_pke_20::R](ltc0_pke_20::R) reader structure"]
impl crate::Readable for LTC0_PKE_20 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_20::W](ltc0_pke_20::W) writer structure"]
impl crate::Writable for LTC0_PKE_20 {}
#[doc = "LTC PKHA E 20 Register"]
pub mod ltc0_pke_20;
#[doc = "LTC PKHA E1 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_5](ltc0_pke1_5) module"]
pub type LTC0_PKE1_5 = crate::Reg<u32, _LTC0_PKE1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_5;
#[doc = "`read()` method returns [ltc0_pke1_5::R](ltc0_pke1_5::R) reader structure"]
impl crate::Readable for LTC0_PKE1_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_5::W](ltc0_pke1_5::W) writer structure"]
impl crate::Writable for LTC0_PKE1_5 {}
#[doc = "LTC PKHA E1 5 Register"]
pub mod ltc0_pke1_5;
#[doc = "LTC PKHA E 21 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_21](ltc0_pke_21) module"]
pub type LTC0_PKE_21 = crate::Reg<u32, _LTC0_PKE_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_21;
#[doc = "`read()` method returns [ltc0_pke_21::R](ltc0_pke_21::R) reader structure"]
impl crate::Readable for LTC0_PKE_21 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_21::W](ltc0_pke_21::W) writer structure"]
impl crate::Writable for LTC0_PKE_21 {}
#[doc = "LTC PKHA E 21 Register"]
pub mod ltc0_pke_21;
#[doc = "LTC PKHA E1 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_6](ltc0_pke1_6) module"]
pub type LTC0_PKE1_6 = crate::Reg<u32, _LTC0_PKE1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_6;
#[doc = "`read()` method returns [ltc0_pke1_6::R](ltc0_pke1_6::R) reader structure"]
impl crate::Readable for LTC0_PKE1_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_6::W](ltc0_pke1_6::W) writer structure"]
impl crate::Writable for LTC0_PKE1_6 {}
#[doc = "LTC PKHA E1 6 Register"]
pub mod ltc0_pke1_6;
#[doc = "LTC PKHA E 22 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_22](ltc0_pke_22) module"]
pub type LTC0_PKE_22 = crate::Reg<u32, _LTC0_PKE_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_22;
#[doc = "`read()` method returns [ltc0_pke_22::R](ltc0_pke_22::R) reader structure"]
impl crate::Readable for LTC0_PKE_22 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_22::W](ltc0_pke_22::W) writer structure"]
impl crate::Writable for LTC0_PKE_22 {}
#[doc = "LTC PKHA E 22 Register"]
pub mod ltc0_pke_22;
#[doc = "LTC PKHA E1 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_7](ltc0_pke1_7) module"]
pub type LTC0_PKE1_7 = crate::Reg<u32, _LTC0_PKE1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_7;
#[doc = "`read()` method returns [ltc0_pke1_7::R](ltc0_pke1_7::R) reader structure"]
impl crate::Readable for LTC0_PKE1_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_7::W](ltc0_pke1_7::W) writer structure"]
impl crate::Writable for LTC0_PKE1_7 {}
#[doc = "LTC PKHA E1 7 Register"]
pub mod ltc0_pke1_7;
#[doc = "LTC PKHA E 23 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_23](ltc0_pke_23) module"]
pub type LTC0_PKE_23 = crate::Reg<u32, _LTC0_PKE_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_23;
#[doc = "`read()` method returns [ltc0_pke_23::R](ltc0_pke_23::R) reader structure"]
impl crate::Readable for LTC0_PKE_23 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_23::W](ltc0_pke_23::W) writer structure"]
impl crate::Writable for LTC0_PKE_23 {}
#[doc = "LTC PKHA E 23 Register"]
pub mod ltc0_pke_23;
#[doc = "LTC PKHA E1 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_8](ltc0_pke1_8) module"]
pub type LTC0_PKE1_8 = crate::Reg<u32, _LTC0_PKE1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_8;
#[doc = "`read()` method returns [ltc0_pke1_8::R](ltc0_pke1_8::R) reader structure"]
impl crate::Readable for LTC0_PKE1_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_8::W](ltc0_pke1_8::W) writer structure"]
impl crate::Writable for LTC0_PKE1_8 {}
#[doc = "LTC PKHA E1 8 Register"]
pub mod ltc0_pke1_8;
#[doc = "LTC PKHA E 24 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_24](ltc0_pke_24) module"]
pub type LTC0_PKE_24 = crate::Reg<u32, _LTC0_PKE_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_24;
#[doc = "`read()` method returns [ltc0_pke_24::R](ltc0_pke_24::R) reader structure"]
impl crate::Readable for LTC0_PKE_24 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_24::W](ltc0_pke_24::W) writer structure"]
impl crate::Writable for LTC0_PKE_24 {}
#[doc = "LTC PKHA E 24 Register"]
pub mod ltc0_pke_24;
#[doc = "LTC PKHA E1 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_9](ltc0_pke1_9) module"]
pub type LTC0_PKE1_9 = crate::Reg<u32, _LTC0_PKE1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_9;
#[doc = "`read()` method returns [ltc0_pke1_9::R](ltc0_pke1_9::R) reader structure"]
impl crate::Readable for LTC0_PKE1_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_9::W](ltc0_pke1_9::W) writer structure"]
impl crate::Writable for LTC0_PKE1_9 {}
#[doc = "LTC PKHA E1 9 Register"]
pub mod ltc0_pke1_9;
#[doc = "LTC PKHA E 25 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_25](ltc0_pke_25) module"]
pub type LTC0_PKE_25 = crate::Reg<u32, _LTC0_PKE_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_25;
#[doc = "`read()` method returns [ltc0_pke_25::R](ltc0_pke_25::R) reader structure"]
impl crate::Readable for LTC0_PKE_25 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_25::W](ltc0_pke_25::W) writer structure"]
impl crate::Writable for LTC0_PKE_25 {}
#[doc = "LTC PKHA E 25 Register"]
pub mod ltc0_pke_25;
#[doc = "LTC PKHA E1 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_10](ltc0_pke1_10) module"]
pub type LTC0_PKE1_10 = crate::Reg<u32, _LTC0_PKE1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_10;
#[doc = "`read()` method returns [ltc0_pke1_10::R](ltc0_pke1_10::R) reader structure"]
impl crate::Readable for LTC0_PKE1_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_10::W](ltc0_pke1_10::W) writer structure"]
impl crate::Writable for LTC0_PKE1_10 {}
#[doc = "LTC PKHA E1 10 Register"]
pub mod ltc0_pke1_10;
#[doc = "LTC PKHA E 26 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_26](ltc0_pke_26) module"]
pub type LTC0_PKE_26 = crate::Reg<u32, _LTC0_PKE_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_26;
#[doc = "`read()` method returns [ltc0_pke_26::R](ltc0_pke_26::R) reader structure"]
impl crate::Readable for LTC0_PKE_26 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_26::W](ltc0_pke_26::W) writer structure"]
impl crate::Writable for LTC0_PKE_26 {}
#[doc = "LTC PKHA E 26 Register"]
pub mod ltc0_pke_26;
#[doc = "LTC PKHA E1 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_11](ltc0_pke1_11) module"]
pub type LTC0_PKE1_11 = crate::Reg<u32, _LTC0_PKE1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_11;
#[doc = "`read()` method returns [ltc0_pke1_11::R](ltc0_pke1_11::R) reader structure"]
impl crate::Readable for LTC0_PKE1_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_11::W](ltc0_pke1_11::W) writer structure"]
impl crate::Writable for LTC0_PKE1_11 {}
#[doc = "LTC PKHA E1 11 Register"]
pub mod ltc0_pke1_11;
#[doc = "LTC PKHA E 27 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_27](ltc0_pke_27) module"]
pub type LTC0_PKE_27 = crate::Reg<u32, _LTC0_PKE_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_27;
#[doc = "`read()` method returns [ltc0_pke_27::R](ltc0_pke_27::R) reader structure"]
impl crate::Readable for LTC0_PKE_27 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_27::W](ltc0_pke_27::W) writer structure"]
impl crate::Writable for LTC0_PKE_27 {}
#[doc = "LTC PKHA E 27 Register"]
pub mod ltc0_pke_27;
#[doc = "LTC PKHA E1 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_12](ltc0_pke1_12) module"]
pub type LTC0_PKE1_12 = crate::Reg<u32, _LTC0_PKE1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_12;
#[doc = "`read()` method returns [ltc0_pke1_12::R](ltc0_pke1_12::R) reader structure"]
impl crate::Readable for LTC0_PKE1_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_12::W](ltc0_pke1_12::W) writer structure"]
impl crate::Writable for LTC0_PKE1_12 {}
#[doc = "LTC PKHA E1 12 Register"]
pub mod ltc0_pke1_12;
#[doc = "LTC PKHA E 28 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_28](ltc0_pke_28) module"]
pub type LTC0_PKE_28 = crate::Reg<u32, _LTC0_PKE_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_28;
#[doc = "`read()` method returns [ltc0_pke_28::R](ltc0_pke_28::R) reader structure"]
impl crate::Readable for LTC0_PKE_28 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_28::W](ltc0_pke_28::W) writer structure"]
impl crate::Writable for LTC0_PKE_28 {}
#[doc = "LTC PKHA E 28 Register"]
pub mod ltc0_pke_28;
#[doc = "LTC PKHA E1 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_13](ltc0_pke1_13) module"]
pub type LTC0_PKE1_13 = crate::Reg<u32, _LTC0_PKE1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_13;
#[doc = "`read()` method returns [ltc0_pke1_13::R](ltc0_pke1_13::R) reader structure"]
impl crate::Readable for LTC0_PKE1_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_13::W](ltc0_pke1_13::W) writer structure"]
impl crate::Writable for LTC0_PKE1_13 {}
#[doc = "LTC PKHA E1 13 Register"]
pub mod ltc0_pke1_13;
#[doc = "LTC PKHA E 29 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_29](ltc0_pke_29) module"]
pub type LTC0_PKE_29 = crate::Reg<u32, _LTC0_PKE_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_29;
#[doc = "`read()` method returns [ltc0_pke_29::R](ltc0_pke_29::R) reader structure"]
impl crate::Readable for LTC0_PKE_29 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_29::W](ltc0_pke_29::W) writer structure"]
impl crate::Writable for LTC0_PKE_29 {}
#[doc = "LTC PKHA E 29 Register"]
pub mod ltc0_pke_29;
#[doc = "LTC PKHA E1 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_14](ltc0_pke1_14) module"]
pub type LTC0_PKE1_14 = crate::Reg<u32, _LTC0_PKE1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_14;
#[doc = "`read()` method returns [ltc0_pke1_14::R](ltc0_pke1_14::R) reader structure"]
impl crate::Readable for LTC0_PKE1_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_14::W](ltc0_pke1_14::W) writer structure"]
impl crate::Writable for LTC0_PKE1_14 {}
#[doc = "LTC PKHA E1 14 Register"]
pub mod ltc0_pke1_14;
#[doc = "LTC PKHA E 30 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_30](ltc0_pke_30) module"]
pub type LTC0_PKE_30 = crate::Reg<u32, _LTC0_PKE_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_30;
#[doc = "`read()` method returns [ltc0_pke_30::R](ltc0_pke_30::R) reader structure"]
impl crate::Readable for LTC0_PKE_30 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_30::W](ltc0_pke_30::W) writer structure"]
impl crate::Writable for LTC0_PKE_30 {}
#[doc = "LTC PKHA E 30 Register"]
pub mod ltc0_pke_30;
#[doc = "LTC PKHA E1 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke1_15](ltc0_pke1_15) module"]
pub type LTC0_PKE1_15 = crate::Reg<u32, _LTC0_PKE1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE1_15;
#[doc = "`read()` method returns [ltc0_pke1_15::R](ltc0_pke1_15::R) reader structure"]
impl crate::Readable for LTC0_PKE1_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke1_15::W](ltc0_pke1_15::W) writer structure"]
impl crate::Writable for LTC0_PKE1_15 {}
#[doc = "LTC PKHA E1 15 Register"]
pub mod ltc0_pke1_15;
#[doc = "LTC PKHA E 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_31](ltc0_pke_31) module"]
pub type LTC0_PKE_31 = crate::Reg<u32, _LTC0_PKE_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_31;
#[doc = "`read()` method returns [ltc0_pke_31::R](ltc0_pke_31::R) reader structure"]
impl crate::Readable for LTC0_PKE_31 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_31::W](ltc0_pke_31::W) writer structure"]
impl crate::Writable for LTC0_PKE_31 {}
#[doc = "LTC PKHA E 31 Register"]
pub mod ltc0_pke_31;
#[doc = "LTC PKHA E2 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_0](ltc0_pke2_0) module"]
pub type LTC0_PKE2_0 = crate::Reg<u32, _LTC0_PKE2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_0;
#[doc = "`read()` method returns [ltc0_pke2_0::R](ltc0_pke2_0::R) reader structure"]
impl crate::Readable for LTC0_PKE2_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_0::W](ltc0_pke2_0::W) writer structure"]
impl crate::Writable for LTC0_PKE2_0 {}
#[doc = "LTC PKHA E2 0 Register"]
pub mod ltc0_pke2_0;
#[doc = "LTC PKHA E 32 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_32](ltc0_pke_32) module"]
pub type LTC0_PKE_32 = crate::Reg<u32, _LTC0_PKE_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_32;
#[doc = "`read()` method returns [ltc0_pke_32::R](ltc0_pke_32::R) reader structure"]
impl crate::Readable for LTC0_PKE_32 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_32::W](ltc0_pke_32::W) writer structure"]
impl crate::Writable for LTC0_PKE_32 {}
#[doc = "LTC PKHA E 32 Register"]
pub mod ltc0_pke_32;
#[doc = "LTC PKHA E2 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_1](ltc0_pke2_1) module"]
pub type LTC0_PKE2_1 = crate::Reg<u32, _LTC0_PKE2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_1;
#[doc = "`read()` method returns [ltc0_pke2_1::R](ltc0_pke2_1::R) reader structure"]
impl crate::Readable for LTC0_PKE2_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_1::W](ltc0_pke2_1::W) writer structure"]
impl crate::Writable for LTC0_PKE2_1 {}
#[doc = "LTC PKHA E2 1 Register"]
pub mod ltc0_pke2_1;
#[doc = "LTC PKHA E 33 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_33](ltc0_pke_33) module"]
pub type LTC0_PKE_33 = crate::Reg<u32, _LTC0_PKE_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_33;
#[doc = "`read()` method returns [ltc0_pke_33::R](ltc0_pke_33::R) reader structure"]
impl crate::Readable for LTC0_PKE_33 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_33::W](ltc0_pke_33::W) writer structure"]
impl crate::Writable for LTC0_PKE_33 {}
#[doc = "LTC PKHA E 33 Register"]
pub mod ltc0_pke_33;
#[doc = "LTC PKHA E2 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_2](ltc0_pke2_2) module"]
pub type LTC0_PKE2_2 = crate::Reg<u32, _LTC0_PKE2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_2;
#[doc = "`read()` method returns [ltc0_pke2_2::R](ltc0_pke2_2::R) reader structure"]
impl crate::Readable for LTC0_PKE2_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_2::W](ltc0_pke2_2::W) writer structure"]
impl crate::Writable for LTC0_PKE2_2 {}
#[doc = "LTC PKHA E2 2 Register"]
pub mod ltc0_pke2_2;
#[doc = "LTC PKHA E 34 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_34](ltc0_pke_34) module"]
pub type LTC0_PKE_34 = crate::Reg<u32, _LTC0_PKE_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_34;
#[doc = "`read()` method returns [ltc0_pke_34::R](ltc0_pke_34::R) reader structure"]
impl crate::Readable for LTC0_PKE_34 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_34::W](ltc0_pke_34::W) writer structure"]
impl crate::Writable for LTC0_PKE_34 {}
#[doc = "LTC PKHA E 34 Register"]
pub mod ltc0_pke_34;
#[doc = "LTC PKHA E2 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_3](ltc0_pke2_3) module"]
pub type LTC0_PKE2_3 = crate::Reg<u32, _LTC0_PKE2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_3;
#[doc = "`read()` method returns [ltc0_pke2_3::R](ltc0_pke2_3::R) reader structure"]
impl crate::Readable for LTC0_PKE2_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_3::W](ltc0_pke2_3::W) writer structure"]
impl crate::Writable for LTC0_PKE2_3 {}
#[doc = "LTC PKHA E2 3 Register"]
pub mod ltc0_pke2_3;
#[doc = "LTC PKHA E 35 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_35](ltc0_pke_35) module"]
pub type LTC0_PKE_35 = crate::Reg<u32, _LTC0_PKE_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_35;
#[doc = "`read()` method returns [ltc0_pke_35::R](ltc0_pke_35::R) reader structure"]
impl crate::Readable for LTC0_PKE_35 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_35::W](ltc0_pke_35::W) writer structure"]
impl crate::Writable for LTC0_PKE_35 {}
#[doc = "LTC PKHA E 35 Register"]
pub mod ltc0_pke_35;
#[doc = "LTC PKHA E2 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_4](ltc0_pke2_4) module"]
pub type LTC0_PKE2_4 = crate::Reg<u32, _LTC0_PKE2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_4;
#[doc = "`read()` method returns [ltc0_pke2_4::R](ltc0_pke2_4::R) reader structure"]
impl crate::Readable for LTC0_PKE2_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_4::W](ltc0_pke2_4::W) writer structure"]
impl crate::Writable for LTC0_PKE2_4 {}
#[doc = "LTC PKHA E2 4 Register"]
pub mod ltc0_pke2_4;
#[doc = "LTC PKHA E 36 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_36](ltc0_pke_36) module"]
pub type LTC0_PKE_36 = crate::Reg<u32, _LTC0_PKE_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_36;
#[doc = "`read()` method returns [ltc0_pke_36::R](ltc0_pke_36::R) reader structure"]
impl crate::Readable for LTC0_PKE_36 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_36::W](ltc0_pke_36::W) writer structure"]
impl crate::Writable for LTC0_PKE_36 {}
#[doc = "LTC PKHA E 36 Register"]
pub mod ltc0_pke_36;
#[doc = "LTC PKHA E2 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_5](ltc0_pke2_5) module"]
pub type LTC0_PKE2_5 = crate::Reg<u32, _LTC0_PKE2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_5;
#[doc = "`read()` method returns [ltc0_pke2_5::R](ltc0_pke2_5::R) reader structure"]
impl crate::Readable for LTC0_PKE2_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_5::W](ltc0_pke2_5::W) writer structure"]
impl crate::Writable for LTC0_PKE2_5 {}
#[doc = "LTC PKHA E2 5 Register"]
pub mod ltc0_pke2_5;
#[doc = "LTC PKHA E 37 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_37](ltc0_pke_37) module"]
pub type LTC0_PKE_37 = crate::Reg<u32, _LTC0_PKE_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_37;
#[doc = "`read()` method returns [ltc0_pke_37::R](ltc0_pke_37::R) reader structure"]
impl crate::Readable for LTC0_PKE_37 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_37::W](ltc0_pke_37::W) writer structure"]
impl crate::Writable for LTC0_PKE_37 {}
#[doc = "LTC PKHA E 37 Register"]
pub mod ltc0_pke_37;
#[doc = "LTC PKHA E2 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_6](ltc0_pke2_6) module"]
pub type LTC0_PKE2_6 = crate::Reg<u32, _LTC0_PKE2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_6;
#[doc = "`read()` method returns [ltc0_pke2_6::R](ltc0_pke2_6::R) reader structure"]
impl crate::Readable for LTC0_PKE2_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_6::W](ltc0_pke2_6::W) writer structure"]
impl crate::Writable for LTC0_PKE2_6 {}
#[doc = "LTC PKHA E2 6 Register"]
pub mod ltc0_pke2_6;
#[doc = "LTC PKHA E 38 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_38](ltc0_pke_38) module"]
pub type LTC0_PKE_38 = crate::Reg<u32, _LTC0_PKE_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_38;
#[doc = "`read()` method returns [ltc0_pke_38::R](ltc0_pke_38::R) reader structure"]
impl crate::Readable for LTC0_PKE_38 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_38::W](ltc0_pke_38::W) writer structure"]
impl crate::Writable for LTC0_PKE_38 {}
#[doc = "LTC PKHA E 38 Register"]
pub mod ltc0_pke_38;
#[doc = "LTC PKHA E2 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_7](ltc0_pke2_7) module"]
pub type LTC0_PKE2_7 = crate::Reg<u32, _LTC0_PKE2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_7;
#[doc = "`read()` method returns [ltc0_pke2_7::R](ltc0_pke2_7::R) reader structure"]
impl crate::Readable for LTC0_PKE2_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_7::W](ltc0_pke2_7::W) writer structure"]
impl crate::Writable for LTC0_PKE2_7 {}
#[doc = "LTC PKHA E2 7 Register"]
pub mod ltc0_pke2_7;
#[doc = "LTC PKHA E 39 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_39](ltc0_pke_39) module"]
pub type LTC0_PKE_39 = crate::Reg<u32, _LTC0_PKE_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_39;
#[doc = "`read()` method returns [ltc0_pke_39::R](ltc0_pke_39::R) reader structure"]
impl crate::Readable for LTC0_PKE_39 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_39::W](ltc0_pke_39::W) writer structure"]
impl crate::Writable for LTC0_PKE_39 {}
#[doc = "LTC PKHA E 39 Register"]
pub mod ltc0_pke_39;
#[doc = "LTC PKHA E2 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_8](ltc0_pke2_8) module"]
pub type LTC0_PKE2_8 = crate::Reg<u32, _LTC0_PKE2_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_8;
#[doc = "`read()` method returns [ltc0_pke2_8::R](ltc0_pke2_8::R) reader structure"]
impl crate::Readable for LTC0_PKE2_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_8::W](ltc0_pke2_8::W) writer structure"]
impl crate::Writable for LTC0_PKE2_8 {}
#[doc = "LTC PKHA E2 8 Register"]
pub mod ltc0_pke2_8;
#[doc = "LTC PKHA E 40 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_40](ltc0_pke_40) module"]
pub type LTC0_PKE_40 = crate::Reg<u32, _LTC0_PKE_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_40;
#[doc = "`read()` method returns [ltc0_pke_40::R](ltc0_pke_40::R) reader structure"]
impl crate::Readable for LTC0_PKE_40 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_40::W](ltc0_pke_40::W) writer structure"]
impl crate::Writable for LTC0_PKE_40 {}
#[doc = "LTC PKHA E 40 Register"]
pub mod ltc0_pke_40;
#[doc = "LTC PKHA E2 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_9](ltc0_pke2_9) module"]
pub type LTC0_PKE2_9 = crate::Reg<u32, _LTC0_PKE2_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_9;
#[doc = "`read()` method returns [ltc0_pke2_9::R](ltc0_pke2_9::R) reader structure"]
impl crate::Readable for LTC0_PKE2_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_9::W](ltc0_pke2_9::W) writer structure"]
impl crate::Writable for LTC0_PKE2_9 {}
#[doc = "LTC PKHA E2 9 Register"]
pub mod ltc0_pke2_9;
#[doc = "LTC PKHA E 41 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_41](ltc0_pke_41) module"]
pub type LTC0_PKE_41 = crate::Reg<u32, _LTC0_PKE_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_41;
#[doc = "`read()` method returns [ltc0_pke_41::R](ltc0_pke_41::R) reader structure"]
impl crate::Readable for LTC0_PKE_41 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_41::W](ltc0_pke_41::W) writer structure"]
impl crate::Writable for LTC0_PKE_41 {}
#[doc = "LTC PKHA E 41 Register"]
pub mod ltc0_pke_41;
#[doc = "LTC PKHA E2 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_10](ltc0_pke2_10) module"]
pub type LTC0_PKE2_10 = crate::Reg<u32, _LTC0_PKE2_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_10;
#[doc = "`read()` method returns [ltc0_pke2_10::R](ltc0_pke2_10::R) reader structure"]
impl crate::Readable for LTC0_PKE2_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_10::W](ltc0_pke2_10::W) writer structure"]
impl crate::Writable for LTC0_PKE2_10 {}
#[doc = "LTC PKHA E2 10 Register"]
pub mod ltc0_pke2_10;
#[doc = "LTC PKHA E 42 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_42](ltc0_pke_42) module"]
pub type LTC0_PKE_42 = crate::Reg<u32, _LTC0_PKE_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_42;
#[doc = "`read()` method returns [ltc0_pke_42::R](ltc0_pke_42::R) reader structure"]
impl crate::Readable for LTC0_PKE_42 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_42::W](ltc0_pke_42::W) writer structure"]
impl crate::Writable for LTC0_PKE_42 {}
#[doc = "LTC PKHA E 42 Register"]
pub mod ltc0_pke_42;
#[doc = "LTC PKHA E2 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_11](ltc0_pke2_11) module"]
pub type LTC0_PKE2_11 = crate::Reg<u32, _LTC0_PKE2_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_11;
#[doc = "`read()` method returns [ltc0_pke2_11::R](ltc0_pke2_11::R) reader structure"]
impl crate::Readable for LTC0_PKE2_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_11::W](ltc0_pke2_11::W) writer structure"]
impl crate::Writable for LTC0_PKE2_11 {}
#[doc = "LTC PKHA E2 11 Register"]
pub mod ltc0_pke2_11;
#[doc = "LTC PKHA E 43 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_43](ltc0_pke_43) module"]
pub type LTC0_PKE_43 = crate::Reg<u32, _LTC0_PKE_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_43;
#[doc = "`read()` method returns [ltc0_pke_43::R](ltc0_pke_43::R) reader structure"]
impl crate::Readable for LTC0_PKE_43 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_43::W](ltc0_pke_43::W) writer structure"]
impl crate::Writable for LTC0_PKE_43 {}
#[doc = "LTC PKHA E 43 Register"]
pub mod ltc0_pke_43;
#[doc = "LTC PKHA E2 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_12](ltc0_pke2_12) module"]
pub type LTC0_PKE2_12 = crate::Reg<u32, _LTC0_PKE2_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_12;
#[doc = "`read()` method returns [ltc0_pke2_12::R](ltc0_pke2_12::R) reader structure"]
impl crate::Readable for LTC0_PKE2_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_12::W](ltc0_pke2_12::W) writer structure"]
impl crate::Writable for LTC0_PKE2_12 {}
#[doc = "LTC PKHA E2 12 Register"]
pub mod ltc0_pke2_12;
#[doc = "LTC PKHA E 44 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_44](ltc0_pke_44) module"]
pub type LTC0_PKE_44 = crate::Reg<u32, _LTC0_PKE_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_44;
#[doc = "`read()` method returns [ltc0_pke_44::R](ltc0_pke_44::R) reader structure"]
impl crate::Readable for LTC0_PKE_44 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_44::W](ltc0_pke_44::W) writer structure"]
impl crate::Writable for LTC0_PKE_44 {}
#[doc = "LTC PKHA E 44 Register"]
pub mod ltc0_pke_44;
#[doc = "LTC PKHA E2 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_13](ltc0_pke2_13) module"]
pub type LTC0_PKE2_13 = crate::Reg<u32, _LTC0_PKE2_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_13;
#[doc = "`read()` method returns [ltc0_pke2_13::R](ltc0_pke2_13::R) reader structure"]
impl crate::Readable for LTC0_PKE2_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_13::W](ltc0_pke2_13::W) writer structure"]
impl crate::Writable for LTC0_PKE2_13 {}
#[doc = "LTC PKHA E2 13 Register"]
pub mod ltc0_pke2_13;
#[doc = "LTC PKHA E 45 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_45](ltc0_pke_45) module"]
pub type LTC0_PKE_45 = crate::Reg<u32, _LTC0_PKE_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_45;
#[doc = "`read()` method returns [ltc0_pke_45::R](ltc0_pke_45::R) reader structure"]
impl crate::Readable for LTC0_PKE_45 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_45::W](ltc0_pke_45::W) writer structure"]
impl crate::Writable for LTC0_PKE_45 {}
#[doc = "LTC PKHA E 45 Register"]
pub mod ltc0_pke_45;
#[doc = "LTC PKHA E2 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_14](ltc0_pke2_14) module"]
pub type LTC0_PKE2_14 = crate::Reg<u32, _LTC0_PKE2_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_14;
#[doc = "`read()` method returns [ltc0_pke2_14::R](ltc0_pke2_14::R) reader structure"]
impl crate::Readable for LTC0_PKE2_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_14::W](ltc0_pke2_14::W) writer structure"]
impl crate::Writable for LTC0_PKE2_14 {}
#[doc = "LTC PKHA E2 14 Register"]
pub mod ltc0_pke2_14;
#[doc = "LTC PKHA E 46 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_46](ltc0_pke_46) module"]
pub type LTC0_PKE_46 = crate::Reg<u32, _LTC0_PKE_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_46;
#[doc = "`read()` method returns [ltc0_pke_46::R](ltc0_pke_46::R) reader structure"]
impl crate::Readable for LTC0_PKE_46 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_46::W](ltc0_pke_46::W) writer structure"]
impl crate::Writable for LTC0_PKE_46 {}
#[doc = "LTC PKHA E 46 Register"]
pub mod ltc0_pke_46;
#[doc = "LTC PKHA E2 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke2_15](ltc0_pke2_15) module"]
pub type LTC0_PKE2_15 = crate::Reg<u32, _LTC0_PKE2_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE2_15;
#[doc = "`read()` method returns [ltc0_pke2_15::R](ltc0_pke2_15::R) reader structure"]
impl crate::Readable for LTC0_PKE2_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke2_15::W](ltc0_pke2_15::W) writer structure"]
impl crate::Writable for LTC0_PKE2_15 {}
#[doc = "LTC PKHA E2 15 Register"]
pub mod ltc0_pke2_15;
#[doc = "LTC PKHA E 47 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_47](ltc0_pke_47) module"]
pub type LTC0_PKE_47 = crate::Reg<u32, _LTC0_PKE_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_47;
#[doc = "`read()` method returns [ltc0_pke_47::R](ltc0_pke_47::R) reader structure"]
impl crate::Readable for LTC0_PKE_47 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_47::W](ltc0_pke_47::W) writer structure"]
impl crate::Writable for LTC0_PKE_47 {}
#[doc = "LTC PKHA E 47 Register"]
pub mod ltc0_pke_47;
#[doc = "LTC PKHA E3 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_0](ltc0_pke3_0) module"]
pub type LTC0_PKE3_0 = crate::Reg<u32, _LTC0_PKE3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_0;
#[doc = "`read()` method returns [ltc0_pke3_0::R](ltc0_pke3_0::R) reader structure"]
impl crate::Readable for LTC0_PKE3_0 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_0::W](ltc0_pke3_0::W) writer structure"]
impl crate::Writable for LTC0_PKE3_0 {}
#[doc = "LTC PKHA E3 0 Register"]
pub mod ltc0_pke3_0;
#[doc = "LTC PKHA E 48 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_48](ltc0_pke_48) module"]
pub type LTC0_PKE_48 = crate::Reg<u32, _LTC0_PKE_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_48;
#[doc = "`read()` method returns [ltc0_pke_48::R](ltc0_pke_48::R) reader structure"]
impl crate::Readable for LTC0_PKE_48 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_48::W](ltc0_pke_48::W) writer structure"]
impl crate::Writable for LTC0_PKE_48 {}
#[doc = "LTC PKHA E 48 Register"]
pub mod ltc0_pke_48;
#[doc = "LTC PKHA E3 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_1](ltc0_pke3_1) module"]
pub type LTC0_PKE3_1 = crate::Reg<u32, _LTC0_PKE3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_1;
#[doc = "`read()` method returns [ltc0_pke3_1::R](ltc0_pke3_1::R) reader structure"]
impl crate::Readable for LTC0_PKE3_1 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_1::W](ltc0_pke3_1::W) writer structure"]
impl crate::Writable for LTC0_PKE3_1 {}
#[doc = "LTC PKHA E3 1 Register"]
pub mod ltc0_pke3_1;
#[doc = "LTC PKHA E 49 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_49](ltc0_pke_49) module"]
pub type LTC0_PKE_49 = crate::Reg<u32, _LTC0_PKE_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_49;
#[doc = "`read()` method returns [ltc0_pke_49::R](ltc0_pke_49::R) reader structure"]
impl crate::Readable for LTC0_PKE_49 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_49::W](ltc0_pke_49::W) writer structure"]
impl crate::Writable for LTC0_PKE_49 {}
#[doc = "LTC PKHA E 49 Register"]
pub mod ltc0_pke_49;
#[doc = "LTC PKHA E3 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_2](ltc0_pke3_2) module"]
pub type LTC0_PKE3_2 = crate::Reg<u32, _LTC0_PKE3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_2;
#[doc = "`read()` method returns [ltc0_pke3_2::R](ltc0_pke3_2::R) reader structure"]
impl crate::Readable for LTC0_PKE3_2 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_2::W](ltc0_pke3_2::W) writer structure"]
impl crate::Writable for LTC0_PKE3_2 {}
#[doc = "LTC PKHA E3 2 Register"]
pub mod ltc0_pke3_2;
#[doc = "LTC PKHA E 50 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_50](ltc0_pke_50) module"]
pub type LTC0_PKE_50 = crate::Reg<u32, _LTC0_PKE_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_50;
#[doc = "`read()` method returns [ltc0_pke_50::R](ltc0_pke_50::R) reader structure"]
impl crate::Readable for LTC0_PKE_50 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_50::W](ltc0_pke_50::W) writer structure"]
impl crate::Writable for LTC0_PKE_50 {}
#[doc = "LTC PKHA E 50 Register"]
pub mod ltc0_pke_50;
#[doc = "LTC PKHA E3 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_3](ltc0_pke3_3) module"]
pub type LTC0_PKE3_3 = crate::Reg<u32, _LTC0_PKE3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_3;
#[doc = "`read()` method returns [ltc0_pke3_3::R](ltc0_pke3_3::R) reader structure"]
impl crate::Readable for LTC0_PKE3_3 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_3::W](ltc0_pke3_3::W) writer structure"]
impl crate::Writable for LTC0_PKE3_3 {}
#[doc = "LTC PKHA E3 3 Register"]
pub mod ltc0_pke3_3;
#[doc = "LTC PKHA E 51 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_51](ltc0_pke_51) module"]
pub type LTC0_PKE_51 = crate::Reg<u32, _LTC0_PKE_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_51;
#[doc = "`read()` method returns [ltc0_pke_51::R](ltc0_pke_51::R) reader structure"]
impl crate::Readable for LTC0_PKE_51 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_51::W](ltc0_pke_51::W) writer structure"]
impl crate::Writable for LTC0_PKE_51 {}
#[doc = "LTC PKHA E 51 Register"]
pub mod ltc0_pke_51;
#[doc = "LTC PKHA E3 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_4](ltc0_pke3_4) module"]
pub type LTC0_PKE3_4 = crate::Reg<u32, _LTC0_PKE3_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_4;
#[doc = "`read()` method returns [ltc0_pke3_4::R](ltc0_pke3_4::R) reader structure"]
impl crate::Readable for LTC0_PKE3_4 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_4::W](ltc0_pke3_4::W) writer structure"]
impl crate::Writable for LTC0_PKE3_4 {}
#[doc = "LTC PKHA E3 4 Register"]
pub mod ltc0_pke3_4;
#[doc = "LTC PKHA E 52 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_52](ltc0_pke_52) module"]
pub type LTC0_PKE_52 = crate::Reg<u32, _LTC0_PKE_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_52;
#[doc = "`read()` method returns [ltc0_pke_52::R](ltc0_pke_52::R) reader structure"]
impl crate::Readable for LTC0_PKE_52 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_52::W](ltc0_pke_52::W) writer structure"]
impl crate::Writable for LTC0_PKE_52 {}
#[doc = "LTC PKHA E 52 Register"]
pub mod ltc0_pke_52;
#[doc = "LTC PKHA E3 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_5](ltc0_pke3_5) module"]
pub type LTC0_PKE3_5 = crate::Reg<u32, _LTC0_PKE3_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_5;
#[doc = "`read()` method returns [ltc0_pke3_5::R](ltc0_pke3_5::R) reader structure"]
impl crate::Readable for LTC0_PKE3_5 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_5::W](ltc0_pke3_5::W) writer structure"]
impl crate::Writable for LTC0_PKE3_5 {}
#[doc = "LTC PKHA E3 5 Register"]
pub mod ltc0_pke3_5;
#[doc = "LTC PKHA E 53 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_53](ltc0_pke_53) module"]
pub type LTC0_PKE_53 = crate::Reg<u32, _LTC0_PKE_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_53;
#[doc = "`read()` method returns [ltc0_pke_53::R](ltc0_pke_53::R) reader structure"]
impl crate::Readable for LTC0_PKE_53 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_53::W](ltc0_pke_53::W) writer structure"]
impl crate::Writable for LTC0_PKE_53 {}
#[doc = "LTC PKHA E 53 Register"]
pub mod ltc0_pke_53;
#[doc = "LTC PKHA E3 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_6](ltc0_pke3_6) module"]
pub type LTC0_PKE3_6 = crate::Reg<u32, _LTC0_PKE3_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_6;
#[doc = "`read()` method returns [ltc0_pke3_6::R](ltc0_pke3_6::R) reader structure"]
impl crate::Readable for LTC0_PKE3_6 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_6::W](ltc0_pke3_6::W) writer structure"]
impl crate::Writable for LTC0_PKE3_6 {}
#[doc = "LTC PKHA E3 6 Register"]
pub mod ltc0_pke3_6;
#[doc = "LTC PKHA E 54 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_54](ltc0_pke_54) module"]
pub type LTC0_PKE_54 = crate::Reg<u32, _LTC0_PKE_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_54;
#[doc = "`read()` method returns [ltc0_pke_54::R](ltc0_pke_54::R) reader structure"]
impl crate::Readable for LTC0_PKE_54 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_54::W](ltc0_pke_54::W) writer structure"]
impl crate::Writable for LTC0_PKE_54 {}
#[doc = "LTC PKHA E 54 Register"]
pub mod ltc0_pke_54;
#[doc = "LTC PKHA E3 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_7](ltc0_pke3_7) module"]
pub type LTC0_PKE3_7 = crate::Reg<u32, _LTC0_PKE3_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_7;
#[doc = "`read()` method returns [ltc0_pke3_7::R](ltc0_pke3_7::R) reader structure"]
impl crate::Readable for LTC0_PKE3_7 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_7::W](ltc0_pke3_7::W) writer structure"]
impl crate::Writable for LTC0_PKE3_7 {}
#[doc = "LTC PKHA E3 7 Register"]
pub mod ltc0_pke3_7;
#[doc = "LTC PKHA E 55 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_55](ltc0_pke_55) module"]
pub type LTC0_PKE_55 = crate::Reg<u32, _LTC0_PKE_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_55;
#[doc = "`read()` method returns [ltc0_pke_55::R](ltc0_pke_55::R) reader structure"]
impl crate::Readable for LTC0_PKE_55 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_55::W](ltc0_pke_55::W) writer structure"]
impl crate::Writable for LTC0_PKE_55 {}
#[doc = "LTC PKHA E 55 Register"]
pub mod ltc0_pke_55;
#[doc = "LTC PKHA E3 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_8](ltc0_pke3_8) module"]
pub type LTC0_PKE3_8 = crate::Reg<u32, _LTC0_PKE3_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_8;
#[doc = "`read()` method returns [ltc0_pke3_8::R](ltc0_pke3_8::R) reader structure"]
impl crate::Readable for LTC0_PKE3_8 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_8::W](ltc0_pke3_8::W) writer structure"]
impl crate::Writable for LTC0_PKE3_8 {}
#[doc = "LTC PKHA E3 8 Register"]
pub mod ltc0_pke3_8;
#[doc = "LTC PKHA E 56 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_56](ltc0_pke_56) module"]
pub type LTC0_PKE_56 = crate::Reg<u32, _LTC0_PKE_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_56;
#[doc = "`read()` method returns [ltc0_pke_56::R](ltc0_pke_56::R) reader structure"]
impl crate::Readable for LTC0_PKE_56 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_56::W](ltc0_pke_56::W) writer structure"]
impl crate::Writable for LTC0_PKE_56 {}
#[doc = "LTC PKHA E 56 Register"]
pub mod ltc0_pke_56;
#[doc = "LTC PKHA E3 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_9](ltc0_pke3_9) module"]
pub type LTC0_PKE3_9 = crate::Reg<u32, _LTC0_PKE3_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_9;
#[doc = "`read()` method returns [ltc0_pke3_9::R](ltc0_pke3_9::R) reader structure"]
impl crate::Readable for LTC0_PKE3_9 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_9::W](ltc0_pke3_9::W) writer structure"]
impl crate::Writable for LTC0_PKE3_9 {}
#[doc = "LTC PKHA E3 9 Register"]
pub mod ltc0_pke3_9;
#[doc = "LTC PKHA E 57 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_57](ltc0_pke_57) module"]
pub type LTC0_PKE_57 = crate::Reg<u32, _LTC0_PKE_57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_57;
#[doc = "`read()` method returns [ltc0_pke_57::R](ltc0_pke_57::R) reader structure"]
impl crate::Readable for LTC0_PKE_57 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_57::W](ltc0_pke_57::W) writer structure"]
impl crate::Writable for LTC0_PKE_57 {}
#[doc = "LTC PKHA E 57 Register"]
pub mod ltc0_pke_57;
#[doc = "LTC PKHA E3 10 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_10](ltc0_pke3_10) module"]
pub type LTC0_PKE3_10 = crate::Reg<u32, _LTC0_PKE3_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_10;
#[doc = "`read()` method returns [ltc0_pke3_10::R](ltc0_pke3_10::R) reader structure"]
impl crate::Readable for LTC0_PKE3_10 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_10::W](ltc0_pke3_10::W) writer structure"]
impl crate::Writable for LTC0_PKE3_10 {}
#[doc = "LTC PKHA E3 10 Register"]
pub mod ltc0_pke3_10;
#[doc = "LTC PKHA E 58 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_58](ltc0_pke_58) module"]
pub type LTC0_PKE_58 = crate::Reg<u32, _LTC0_PKE_58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_58;
#[doc = "`read()` method returns [ltc0_pke_58::R](ltc0_pke_58::R) reader structure"]
impl crate::Readable for LTC0_PKE_58 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_58::W](ltc0_pke_58::W) writer structure"]
impl crate::Writable for LTC0_PKE_58 {}
#[doc = "LTC PKHA E 58 Register"]
pub mod ltc0_pke_58;
#[doc = "LTC PKHA E3 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_11](ltc0_pke3_11) module"]
pub type LTC0_PKE3_11 = crate::Reg<u32, _LTC0_PKE3_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_11;
#[doc = "`read()` method returns [ltc0_pke3_11::R](ltc0_pke3_11::R) reader structure"]
impl crate::Readable for LTC0_PKE3_11 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_11::W](ltc0_pke3_11::W) writer structure"]
impl crate::Writable for LTC0_PKE3_11 {}
#[doc = "LTC PKHA E3 11 Register"]
pub mod ltc0_pke3_11;
#[doc = "LTC PKHA E 59 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_59](ltc0_pke_59) module"]
pub type LTC0_PKE_59 = crate::Reg<u32, _LTC0_PKE_59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_59;
#[doc = "`read()` method returns [ltc0_pke_59::R](ltc0_pke_59::R) reader structure"]
impl crate::Readable for LTC0_PKE_59 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_59::W](ltc0_pke_59::W) writer structure"]
impl crate::Writable for LTC0_PKE_59 {}
#[doc = "LTC PKHA E 59 Register"]
pub mod ltc0_pke_59;
#[doc = "LTC PKHA E3 12 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_12](ltc0_pke3_12) module"]
pub type LTC0_PKE3_12 = crate::Reg<u32, _LTC0_PKE3_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_12;
#[doc = "`read()` method returns [ltc0_pke3_12::R](ltc0_pke3_12::R) reader structure"]
impl crate::Readable for LTC0_PKE3_12 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_12::W](ltc0_pke3_12::W) writer structure"]
impl crate::Writable for LTC0_PKE3_12 {}
#[doc = "LTC PKHA E3 12 Register"]
pub mod ltc0_pke3_12;
#[doc = "LTC PKHA E 60 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_60](ltc0_pke_60) module"]
pub type LTC0_PKE_60 = crate::Reg<u32, _LTC0_PKE_60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_60;
#[doc = "`read()` method returns [ltc0_pke_60::R](ltc0_pke_60::R) reader structure"]
impl crate::Readable for LTC0_PKE_60 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_60::W](ltc0_pke_60::W) writer structure"]
impl crate::Writable for LTC0_PKE_60 {}
#[doc = "LTC PKHA E 60 Register"]
pub mod ltc0_pke_60;
#[doc = "LTC PKHA E3 13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_13](ltc0_pke3_13) module"]
pub type LTC0_PKE3_13 = crate::Reg<u32, _LTC0_PKE3_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_13;
#[doc = "`read()` method returns [ltc0_pke3_13::R](ltc0_pke3_13::R) reader structure"]
impl crate::Readable for LTC0_PKE3_13 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_13::W](ltc0_pke3_13::W) writer structure"]
impl crate::Writable for LTC0_PKE3_13 {}
#[doc = "LTC PKHA E3 13 Register"]
pub mod ltc0_pke3_13;
#[doc = "LTC PKHA E 61 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_61](ltc0_pke_61) module"]
pub type LTC0_PKE_61 = crate::Reg<u32, _LTC0_PKE_61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_61;
#[doc = "`read()` method returns [ltc0_pke_61::R](ltc0_pke_61::R) reader structure"]
impl crate::Readable for LTC0_PKE_61 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_61::W](ltc0_pke_61::W) writer structure"]
impl crate::Writable for LTC0_PKE_61 {}
#[doc = "LTC PKHA E 61 Register"]
pub mod ltc0_pke_61;
#[doc = "LTC PKHA E3 14 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_14](ltc0_pke3_14) module"]
pub type LTC0_PKE3_14 = crate::Reg<u32, _LTC0_PKE3_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_14;
#[doc = "`read()` method returns [ltc0_pke3_14::R](ltc0_pke3_14::R) reader structure"]
impl crate::Readable for LTC0_PKE3_14 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_14::W](ltc0_pke3_14::W) writer structure"]
impl crate::Writable for LTC0_PKE3_14 {}
#[doc = "LTC PKHA E3 14 Register"]
pub mod ltc0_pke3_14;
#[doc = "LTC PKHA E 62 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_62](ltc0_pke_62) module"]
pub type LTC0_PKE_62 = crate::Reg<u32, _LTC0_PKE_62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_62;
#[doc = "`read()` method returns [ltc0_pke_62::R](ltc0_pke_62::R) reader structure"]
impl crate::Readable for LTC0_PKE_62 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_62::W](ltc0_pke_62::W) writer structure"]
impl crate::Writable for LTC0_PKE_62 {}
#[doc = "LTC PKHA E 62 Register"]
pub mod ltc0_pke_62;
#[doc = "LTC PKHA E3 15 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke3_15](ltc0_pke3_15) module"]
pub type LTC0_PKE3_15 = crate::Reg<u32, _LTC0_PKE3_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE3_15;
#[doc = "`read()` method returns [ltc0_pke3_15::R](ltc0_pke3_15::R) reader structure"]
impl crate::Readable for LTC0_PKE3_15 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke3_15::W](ltc0_pke3_15::W) writer structure"]
impl crate::Writable for LTC0_PKE3_15 {}
#[doc = "LTC PKHA E3 15 Register"]
pub mod ltc0_pke3_15;
#[doc = "LTC PKHA E 63 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltc0_pke_63](ltc0_pke_63) module"]
pub type LTC0_PKE_63 = crate::Reg<u32, _LTC0_PKE_63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTC0_PKE_63;
#[doc = "`read()` method returns [ltc0_pke_63::R](ltc0_pke_63::R) reader structure"]
impl crate::Readable for LTC0_PKE_63 {}
#[doc = "`write(|w| ..)` method takes [ltc0_pke_63::W](ltc0_pke_63::W) writer structure"]
impl crate::Writable for LTC0_PKE_63 {}
#[doc = "LTC PKHA E 63 Register"]
pub mod ltc0_pke_63;
