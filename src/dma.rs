#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved3: [u8; 4usize],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved12: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved13: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved14: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved15: [u8; 12usize],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved16: [u8; 184usize],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: DCHPRI,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: DCHPRI,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: DCHPRI,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: DCHPRI,
    #[doc = "0x104 - Channel n Priority Register"]
    pub dchpri7: DCHPRI,
    #[doc = "0x105 - Channel n Priority Register"]
    pub dchpri6: DCHPRI,
    #[doc = "0x106 - Channel n Priority Register"]
    pub dchpri5: DCHPRI,
    #[doc = "0x107 - Channel n Priority Register"]
    pub dchpri4: DCHPRI,
    _reserved24: [u8; 3832usize],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd0_nbytes: TCD0_NBYTES_UNION,
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_citer: TCD0_CITER_UNION,
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD_CSR,
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_biter: TCD0_BITER_UNION,
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd1_nbytes: TCD1_NBYTES_UNION,
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_citer: TCD1_CITER_UNION,
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD_CSR,
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_biter: TCD1_BITER_UNION,
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd2_nbytes: TCD2_NBYTES_UNION,
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_citer: TCD2_CITER_UNION,
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD_CSR,
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_biter: TCD2_BITER_UNION,
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd3_nbytes: TCD3_NBYTES_UNION,
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_citer: TCD3_CITER_UNION,
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD_CSR,
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_biter: TCD3_BITER_UNION,
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD_ATTR,
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd4_nbytes: TCD4_NBYTES_UNION,
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD_DOFF,
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd4_citer: TCD4_CITER_UNION,
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD_CSR,
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd4_biter: TCD4_BITER_UNION,
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD_ATTR,
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd5_nbytes: TCD5_NBYTES_UNION,
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD_DOFF,
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd5_citer: TCD5_CITER_UNION,
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD_CSR,
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd5_biter: TCD5_BITER_UNION,
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD_ATTR,
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd6_nbytes: TCD6_NBYTES_UNION,
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD_DOFF,
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd6_citer: TCD6_CITER_UNION,
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD_CSR,
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd6_biter: TCD6_BITER_UNION,
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD_ATTR,
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd7_nbytes: TCD7_NBYTES_UNION,
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD_DOFF,
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd7_citer: TCD7_CITER_UNION,
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD_CSR,
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd7_biter: TCD7_BITER_UNION,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(C)]
pub union TCD0_NBYTES_UNION {
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    pub tcd0_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    pub tcd0_nbytes_mloffno: TCD_NBYTES_MLOFFNO,
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd0_nbytes_mlno: TCD_NBYTES_MLNO,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD0_CITER_UNION {
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd0_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_citer_elinkno: TCD_CITER_ELINKNO,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD0_BITER_UNION {
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd0_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(C)]
pub union TCD1_NBYTES_UNION {
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    pub tcd1_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    pub tcd1_nbytes_mloffno: TCD_NBYTES_MLOFFNO,
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd1_nbytes_mlno: TCD_NBYTES_MLNO,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD1_CITER_UNION {
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd1_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_citer_elinkno: TCD_CITER_ELINKNO,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD1_BITER_UNION {
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd1_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(C)]
pub union TCD2_NBYTES_UNION {
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    pub tcd2_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    pub tcd2_nbytes_mloffno: TCD_NBYTES_MLOFFNO,
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd2_nbytes_mlno: TCD_NBYTES_MLNO,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD2_CITER_UNION {
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd2_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_citer_elinkno: TCD_CITER_ELINKNO,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD2_BITER_UNION {
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd2_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(C)]
pub union TCD3_NBYTES_UNION {
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    pub tcd3_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    pub tcd3_nbytes_mloffno: TCD_NBYTES_MLOFFNO,
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd3_nbytes_mlno: TCD_NBYTES_MLNO,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD3_CITER_UNION {
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd3_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_citer_elinkno: TCD_CITER_ELINKNO,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD3_BITER_UNION {
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd3_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(C)]
pub union TCD4_NBYTES_UNION {
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    pub tcd4_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    pub tcd4_nbytes_mloffno: TCD_NBYTES_MLOFFNO,
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd4_nbytes_mlno: TCD_NBYTES_MLNO,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD4_CITER_UNION {
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd4_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd4_citer_elinkno: TCD_CITER_ELINKNO,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD4_BITER_UNION {
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd4_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd4_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(C)]
pub union TCD5_NBYTES_UNION {
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    pub tcd5_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    pub tcd5_nbytes_mloffno: TCD_NBYTES_MLOFFNO,
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd5_nbytes_mlno: TCD_NBYTES_MLNO,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD5_CITER_UNION {
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd5_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd5_citer_elinkno: TCD_CITER_ELINKNO,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD5_BITER_UNION {
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd5_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd5_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(C)]
pub union TCD6_NBYTES_UNION {
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    pub tcd6_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    pub tcd6_nbytes_mloffno: TCD_NBYTES_MLOFFNO,
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd6_nbytes_mlno: TCD_NBYTES_MLNO,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD6_CITER_UNION {
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd6_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd6_citer_elinkno: TCD_CITER_ELINKNO,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD6_BITER_UNION {
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd6_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd6_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled) TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled) TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(C)]
pub union TCD7_NBYTES_UNION {
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    pub tcd7_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    pub tcd7_nbytes_mloffno: TCD_NBYTES_MLOFFNO,
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd7_nbytes_mlno: TCD_NBYTES_MLNO,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD7_CITER_UNION {
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd7_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd7_citer_elinkno: TCD_CITER_ELINKNO,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled) TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(C)]
pub union TCD7_BITER_UNION {
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd7_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd7_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register"]
pub struct ES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register"]
pub struct ERQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register"]
pub struct EEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register"]
pub struct CEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register"]
pub struct SEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register"]
pub struct CERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register"]
pub struct SERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register"]
pub struct CDNE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register"]
pub struct SSRT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register"]
pub struct CERR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register"]
pub struct CINT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register"]
pub struct ERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register"]
pub struct HRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Enable Asynchronous Request in Stop Register"]
pub struct EARS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD Source Address"]
pub struct TCD_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD Destination Address"]
pub struct TCD_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elinkyes;
