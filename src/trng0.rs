#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRNG0 Miscellaneous Control Register"]
    pub trng0_mctl: TRNG0_MCTL,
    #[doc = "0x04 - TRNG0 Statistical Check Miscellaneous Register"]
    pub trng0_scmisc: TRNG0_SCMISC,
    #[doc = "0x08 - TRNG0 Poker Range Register"]
    pub trng0_pkrrng: TRNG0_PKRRNG,
    #[doc = "TRNG0 Poker Square Calculation Result Register TRNG0 Poker Maximum Limit Register"]
    pub trng0_pkrsq: TRNG0_PKRSQ_UNION,
    #[doc = "0x10 - TRNG0 Seed Control Register"]
    pub trng0_sdctl: TRNG0_SDCTL,
    #[doc = "TRNG0 Total Samples Register TRNG0 Sparse Bit Limit Register"]
    pub trng0_sblim: TRNG0_SBLIM_UNION,
    #[doc = "0x18 - TRNG0 Frequency Count Minimum Limit Register"]
    pub trng0_frqmin: TRNG0_FRQMIN,
    #[doc = "TRNG0 Frequency Count Maximum Limit Register TRNG0 Frequency Count Register"]
    pub trng0_frqcnt: TRNG0_FRQCNT_UNION,
    #[doc = "TRNG0 Statistical Check Monobit Limit Register TRNG0 Statistical Check Monobit Count Register"]
    pub trng0_scmc: TRNG0_SCMC_UNION,
    #[doc = "TRNG0 Statistical Check Run Length 1 Limit Register TRNG0 Statistical Check Run Length 1 Count Register"]
    pub trng0_scr1c: TRNG0_SCR1C_UNION,
    #[doc = "TRNG0 Statistical Check Run Length 2 Limit Register TRNG0 Statistical Check Run Length 2 Count Register"]
    pub trng0_scr2c: TRNG0_SCR2C_UNION,
    #[doc = "TRNG0 Statistical Check Run Length 3 Limit Register TRNG0 Statistical Check Run Length 3 Count Register"]
    pub trng0_scr3c: TRNG0_SCR3C_UNION,
    #[doc = "TRNG0 Statistical Check Run Length 4 Limit Register TRNG0 Statistical Check Run Length 4 Count Register"]
    pub trng0_scr4c: TRNG0_SCR4C_UNION,
    #[doc = "TRNG0 Statistical Check Run Length 5 Limit Register TRNG0 Statistical Check Run Length 5 Count Register"]
    pub trng0_scr5c: TRNG0_SCR5C_UNION,
    #[doc = "TRNG0 Statistical Check Run Length 6+ Limit Register TRNG0 Statistical Check Run Length 6+ Count Register"]
    pub trng0_scr6pc: TRNG0_SCR6PC_UNION,
    #[doc = "0x3c - TRNG0 Status Register"]
    pub trng0_status: TRNG0_STATUS,
    #[doc = "0x40 - RNG TRNG Entropy Read Register"]
    pub trng0_ent0: TRNG0_ENT0,
    #[doc = "0x44 - RNG TRNG Entropy Read Register"]
    pub trng0_ent1: TRNG0_ENT1,
    #[doc = "0x48 - RNG TRNG Entropy Read Register"]
    pub trng0_ent2: TRNG0_ENT2,
    #[doc = "0x4c - RNG TRNG Entropy Read Register"]
    pub trng0_ent3: TRNG0_ENT3,
    #[doc = "0x50 - RNG TRNG Entropy Read Register"]
    pub trng0_ent4: TRNG0_ENT4,
    #[doc = "0x54 - RNG TRNG Entropy Read Register"]
    pub trng0_ent5: TRNG0_ENT5,
    #[doc = "0x58 - RNG TRNG Entropy Read Register"]
    pub trng0_ent6: TRNG0_ENT6,
    #[doc = "0x5c - RNG TRNG Entropy Read Register"]
    pub trng0_ent7: TRNG0_ENT7,
    #[doc = "0x60 - RNG TRNG Entropy Read Register"]
    pub trng0_ent8: TRNG0_ENT8,
    #[doc = "0x64 - RNG TRNG Entropy Read Register"]
    pub trng0_ent9: TRNG0_ENT9,
    #[doc = "0x68 - RNG TRNG Entropy Read Register"]
    pub trng0_ent10: TRNG0_ENT10,
    #[doc = "0x6c - RNG TRNG Entropy Read Register"]
    pub trng0_ent11: TRNG0_ENT11,
    #[doc = "0x70 - RNG TRNG Entropy Read Register"]
    pub trng0_ent12: TRNG0_ENT12,
    #[doc = "0x74 - RNG TRNG Entropy Read Register"]
    pub trng0_ent13: TRNG0_ENT13,
    #[doc = "0x78 - RNG TRNG Entropy Read Register"]
    pub trng0_ent14: TRNG0_ENT14,
    #[doc = "0x7c - RNG TRNG Entropy Read Register"]
    pub trng0_ent15: TRNG0_ENT15,
    #[doc = "0x80 - TRNG0 Statistical Check Poker Count 1 and 0 Register"]
    pub trng0_pkrcnt10: TRNG0_PKRCNT10,
    #[doc = "0x84 - TRNG0 Statistical Check Poker Count 3 and 2 Register"]
    pub trng0_pkrcnt32: TRNG0_PKRCNT32,
    #[doc = "0x88 - TRNG0 Statistical Check Poker Count 5 and 4 Register"]
    pub trng0_pkrcnt54: TRNG0_PKRCNT54,
    #[doc = "0x8c - TRNG0 Statistical Check Poker Count 7 and 6 Register"]
    pub trng0_pkrcnt76: TRNG0_PKRCNT76,
    #[doc = "0x90 - TRNG0 Statistical Check Poker Count 9 and 8 Register"]
    pub trng0_pkrcnt98: TRNG0_PKRCNT98,
    #[doc = "0x94 - TRNG0 Statistical Check Poker Count B and A Register"]
    pub trng0_pkrcntba: TRNG0_PKRCNTBA,
    #[doc = "0x98 - TRNG0 Statistical Check Poker Count D and C Register"]
    pub trng0_pkrcntdc: TRNG0_PKRCNTDC,
    #[doc = "0x9c - TRNG0 Statistical Check Poker Count F and E Register"]
    pub trng0_pkrcntfe: TRNG0_PKRCNTFE,
    #[doc = "0xa0 - TRNG0 Security Configuration Register"]
    pub trng0_sec_cfg: TRNG0_SEC_CFG,
    #[doc = "0xa4 - TRNG0 Interrupt Control Register"]
    pub trng0_int_ctrl: TRNG0_INT_CTRL,
    #[doc = "0xa8 - TRNG0 Mask Register"]
    pub trng0_int_mask: TRNG0_INT_MASK,
    #[doc = "0xac - TRNG0 Interrupt Status Register"]
    pub trng0_int_status: TRNG0_INT_STATUS,
    _reserved44: [u8; 64usize],
    #[doc = "0xf0 - TRNG0 Version ID Register (MS)"]
    pub trng0_vid1: TRNG0_VID1,
    #[doc = "0xf4 - TRNG0 Version ID Register (LS)"]
    pub trng0_vid2: TRNG0_VID2,
}
#[doc = "TRNG0 Poker Square Calculation Result Register TRNG0 Poker Maximum Limit Register"]
#[repr(C)]
pub union TRNG0_PKRSQ_UNION {
    #[doc = "0x0c - TRNG0 Poker Square Calculation Result Register"]
    pub trng0_pkrsq: TRNG0_PKRSQ,
    #[doc = "0x0c - TRNG0 Poker Maximum Limit Register"]
    pub trng0_pkrmax: TRNG0_PKRMAX,
}
#[doc = "TRNG0 Total Samples Register TRNG0 Sparse Bit Limit Register"]
#[repr(C)]
pub union TRNG0_SBLIM_UNION {
    #[doc = "0x14 - TRNG0 Total Samples Register"]
    pub trng0_totsam: TRNG0_TOTSAM,
    #[doc = "0x14 - TRNG0 Sparse Bit Limit Register"]
    pub trng0_sblim: TRNG0_SBLIM,
}
#[doc = "TRNG0 Frequency Count Maximum Limit Register TRNG0 Frequency Count Register"]
#[repr(C)]
pub union TRNG0_FRQCNT_UNION {
    #[doc = "0x1c - TRNG0 Frequency Count Maximum Limit Register"]
    pub trng0_frqmax: TRNG0_FRQMAX,
    #[doc = "0x1c - TRNG0 Frequency Count Register"]
    pub trng0_frqcnt: TRNG0_FRQCNT,
}
#[doc = "TRNG0 Statistical Check Monobit Limit Register TRNG0 Statistical Check Monobit Count Register"]
#[repr(C)]
pub union TRNG0_SCMC_UNION {
    #[doc = "0x20 - TRNG0 Statistical Check Monobit Limit Register"]
    pub trng0_scml: TRNG0_SCML,
    #[doc = "0x20 - TRNG0 Statistical Check Monobit Count Register"]
    pub trng0_scmc: TRNG0_SCMC,
}
#[doc = "TRNG0 Statistical Check Run Length 1 Limit Register TRNG0 Statistical Check Run Length 1 Count Register"]
#[repr(C)]
pub union TRNG0_SCR1C_UNION {
    #[doc = "0x24 - TRNG0 Statistical Check Run Length 1 Limit Register"]
    pub trng0_scr1l: TRNG0_SCR1L,
    #[doc = "0x24 - TRNG0 Statistical Check Run Length 1 Count Register"]
    pub trng0_scr1c: TRNG0_SCR1C,
}
#[doc = "TRNG0 Statistical Check Run Length 2 Limit Register TRNG0 Statistical Check Run Length 2 Count Register"]
#[repr(C)]
pub union TRNG0_SCR2C_UNION {
    #[doc = "0x28 - TRNG0 Statistical Check Run Length 2 Limit Register"]
    pub trng0_scr2l: TRNG0_SCR2L,
    #[doc = "0x28 - TRNG0 Statistical Check Run Length 2 Count Register"]
    pub trng0_scr2c: TRNG0_SCR2C,
}
#[doc = "TRNG0 Statistical Check Run Length 3 Limit Register TRNG0 Statistical Check Run Length 3 Count Register"]
#[repr(C)]
pub union TRNG0_SCR3C_UNION {
    #[doc = "0x2c - TRNG0 Statistical Check Run Length 3 Limit Register"]
    pub trng0_scr3l: TRNG0_SCR3L,
    #[doc = "0x2c - TRNG0 Statistical Check Run Length 3 Count Register"]
    pub trng0_scr3c: TRNG0_SCR3C,
}
#[doc = "TRNG0 Statistical Check Run Length 4 Limit Register TRNG0 Statistical Check Run Length 4 Count Register"]
#[repr(C)]
pub union TRNG0_SCR4C_UNION {
    #[doc = "0x30 - TRNG0 Statistical Check Run Length 4 Limit Register"]
    pub trng0_scr4l: TRNG0_SCR4L,
    #[doc = "0x30 - TRNG0 Statistical Check Run Length 4 Count Register"]
    pub trng0_scr4c: TRNG0_SCR4C,
}
#[doc = "TRNG0 Statistical Check Run Length 5 Limit Register TRNG0 Statistical Check Run Length 5 Count Register"]
#[repr(C)]
pub union TRNG0_SCR5C_UNION {
    #[doc = "0x34 - TRNG0 Statistical Check Run Length 5 Limit Register"]
    pub trng0_scr5l: TRNG0_SCR5L,
    #[doc = "0x34 - TRNG0 Statistical Check Run Length 5 Count Register"]
    pub trng0_scr5c: TRNG0_SCR5C,
}
#[doc = "TRNG0 Statistical Check Run Length 6+ Limit Register TRNG0 Statistical Check Run Length 6+ Count Register"]
#[repr(C)]
pub union TRNG0_SCR6PC_UNION {
    #[doc = "0x38 - TRNG0 Statistical Check Run Length 6+ Limit Register"]
    pub trng0_scr6pl: TRNG0_SCR6PL,
    #[doc = "0x38 - TRNG0 Statistical Check Run Length 6+ Count Register"]
    pub trng0_scr6pc: TRNG0_SCR6PC,
}
#[doc = "TRNG0 Miscellaneous Control Register"]
pub struct TRNG0_MCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Miscellaneous Control Register"]
pub mod trng0_mctl;
#[doc = "TRNG0 Statistical Check Miscellaneous Register"]
pub struct TRNG0_SCMISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Miscellaneous Register"]
pub mod trng0_scmisc;
#[doc = "TRNG0 Poker Range Register"]
pub struct TRNG0_PKRRNG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Poker Range Register"]
pub mod trng0_pkrrng;
#[doc = "TRNG0 Poker Maximum Limit Register"]
pub struct TRNG0_PKRMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Poker Maximum Limit Register"]
pub mod trng0_pkrmax;
#[doc = "TRNG0 Poker Square Calculation Result Register"]
pub struct TRNG0_PKRSQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Poker Square Calculation Result Register"]
pub mod trng0_pkrsq;
#[doc = "TRNG0 Seed Control Register"]
pub struct TRNG0_SDCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Seed Control Register"]
pub mod trng0_sdctl;
#[doc = "TRNG0 Sparse Bit Limit Register"]
pub struct TRNG0_SBLIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Sparse Bit Limit Register"]
pub mod trng0_sblim;
#[doc = "TRNG0 Total Samples Register"]
pub struct TRNG0_TOTSAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Total Samples Register"]
pub mod trng0_totsam;
#[doc = "TRNG0 Frequency Count Minimum Limit Register"]
pub struct TRNG0_FRQMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Frequency Count Minimum Limit Register"]
pub mod trng0_frqmin;
#[doc = "TRNG0 Frequency Count Register"]
pub struct TRNG0_FRQCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Frequency Count Register"]
pub mod trng0_frqcnt;
#[doc = "TRNG0 Frequency Count Maximum Limit Register"]
pub struct TRNG0_FRQMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Frequency Count Maximum Limit Register"]
pub mod trng0_frqmax;
#[doc = "TRNG0 Statistical Check Monobit Count Register"]
pub struct TRNG0_SCMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Monobit Count Register"]
pub mod trng0_scmc;
#[doc = "TRNG0 Statistical Check Monobit Limit Register"]
pub struct TRNG0_SCML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Monobit Limit Register"]
pub mod trng0_scml;
#[doc = "TRNG0 Statistical Check Run Length 1 Count Register"]
pub struct TRNG0_SCR1C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 1 Count Register"]
pub mod trng0_scr1c;
#[doc = "TRNG0 Statistical Check Run Length 1 Limit Register"]
pub struct TRNG0_SCR1L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 1 Limit Register"]
pub mod trng0_scr1l;
#[doc = "TRNG0 Statistical Check Run Length 2 Count Register"]
pub struct TRNG0_SCR2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 2 Count Register"]
pub mod trng0_scr2c;
#[doc = "TRNG0 Statistical Check Run Length 2 Limit Register"]
pub struct TRNG0_SCR2L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 2 Limit Register"]
pub mod trng0_scr2l;
#[doc = "TRNG0 Statistical Check Run Length 3 Count Register"]
pub struct TRNG0_SCR3C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 3 Count Register"]
pub mod trng0_scr3c;
#[doc = "TRNG0 Statistical Check Run Length 3 Limit Register"]
pub struct TRNG0_SCR3L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 3 Limit Register"]
pub mod trng0_scr3l;
#[doc = "TRNG0 Statistical Check Run Length 4 Count Register"]
pub struct TRNG0_SCR4C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 4 Count Register"]
pub mod trng0_scr4c;
#[doc = "TRNG0 Statistical Check Run Length 4 Limit Register"]
pub struct TRNG0_SCR4L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 4 Limit Register"]
pub mod trng0_scr4l;
#[doc = "TRNG0 Statistical Check Run Length 5 Count Register"]
pub struct TRNG0_SCR5C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 5 Count Register"]
pub mod trng0_scr5c;
#[doc = "TRNG0 Statistical Check Run Length 5 Limit Register"]
pub struct TRNG0_SCR5L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 5 Limit Register"]
pub mod trng0_scr5l;
#[doc = "TRNG0 Statistical Check Run Length 6+ Count Register"]
pub struct TRNG0_SCR6PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 6+ Count Register"]
pub mod trng0_scr6pc;
#[doc = "TRNG0 Statistical Check Run Length 6+ Limit Register"]
pub struct TRNG0_SCR6PL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Run Length 6+ Limit Register"]
pub mod trng0_scr6pl;
#[doc = "TRNG0 Status Register"]
pub struct TRNG0_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Status Register"]
pub mod trng0_status;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent0;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent1;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent2;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent3;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent4;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent5;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent6;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent7;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent8;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent9;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent10;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent11;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent12;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent13;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent14;
#[doc = "RNG TRNG Entropy Read Register"]
pub struct TRNG0_ENT15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RNG TRNG Entropy Read Register"]
pub mod trng0_ent15;
#[doc = "TRNG0 Statistical Check Poker Count 1 and 0 Register"]
pub struct TRNG0_PKRCNT10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Poker Count 1 and 0 Register"]
pub mod trng0_pkrcnt10;
#[doc = "TRNG0 Statistical Check Poker Count 3 and 2 Register"]
pub struct TRNG0_PKRCNT32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Poker Count 3 and 2 Register"]
pub mod trng0_pkrcnt32;
#[doc = "TRNG0 Statistical Check Poker Count 5 and 4 Register"]
pub struct TRNG0_PKRCNT54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Poker Count 5 and 4 Register"]
pub mod trng0_pkrcnt54;
#[doc = "TRNG0 Statistical Check Poker Count 7 and 6 Register"]
pub struct TRNG0_PKRCNT76 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Poker Count 7 and 6 Register"]
pub mod trng0_pkrcnt76;
#[doc = "TRNG0 Statistical Check Poker Count 9 and 8 Register"]
pub struct TRNG0_PKRCNT98 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Poker Count 9 and 8 Register"]
pub mod trng0_pkrcnt98;
#[doc = "TRNG0 Statistical Check Poker Count B and A Register"]
pub struct TRNG0_PKRCNTBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Poker Count B and A Register"]
pub mod trng0_pkrcntba;
#[doc = "TRNG0 Statistical Check Poker Count D and C Register"]
pub struct TRNG0_PKRCNTDC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Poker Count D and C Register"]
pub mod trng0_pkrcntdc;
#[doc = "TRNG0 Statistical Check Poker Count F and E Register"]
pub struct TRNG0_PKRCNTFE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Statistical Check Poker Count F and E Register"]
pub mod trng0_pkrcntfe;
#[doc = "TRNG0 Security Configuration Register"]
pub struct TRNG0_SEC_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Security Configuration Register"]
pub mod trng0_sec_cfg;
#[doc = "TRNG0 Interrupt Control Register"]
pub struct TRNG0_INT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Interrupt Control Register"]
pub mod trng0_int_ctrl;
#[doc = "TRNG0 Mask Register"]
pub struct TRNG0_INT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Mask Register"]
pub mod trng0_int_mask;
#[doc = "TRNG0 Interrupt Status Register"]
pub struct TRNG0_INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Interrupt Status Register"]
pub mod trng0_int_status;
#[doc = "TRNG0 Version ID Register (MS)"]
pub struct TRNG0_VID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Version ID Register (MS)"]
pub mod trng0_vid1;
#[doc = "TRNG0 Version ID Register (LS)"]
pub struct TRNG0_VID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG0 Version ID Register (LS)"]
pub mod trng0_vid2;
