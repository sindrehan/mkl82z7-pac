#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Error Status Register"]
    pub es: crate::Reg<es::ES_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: crate::Reg<erq::ERQ_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: crate::Reg<eei::EEI_SPEC>,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: crate::Reg<ceei::CEEI_SPEC>,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: crate::Reg<seei::SEEI_SPEC>,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: crate::Reg<cerq::CERQ_SPEC>,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: crate::Reg<serq::SERQ_SPEC>,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: crate::Reg<cdne::CDNE_SPEC>,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: crate::Reg<ssrt::SSRT_SPEC>,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: crate::Reg<cerr::CERR_SPEC>,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: crate::Reg<cint::CINT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: crate::Reg<int::INT_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x2c - Error Register"]
    pub err: crate::Reg<err::ERR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: crate::Reg<hrs::HRS_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: crate::Reg<ears::EARS_SPEC>,
    _reserved16: [u8; 0xb8],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: crate::Reg<dchpri::DCHPRI_SPEC>,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: crate::Reg<dchpri::DCHPRI_SPEC>,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: crate::Reg<dchpri::DCHPRI_SPEC>,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: crate::Reg<dchpri::DCHPRI_SPEC>,
    #[doc = "0x104 - Channel n Priority Register"]
    pub dchpri7: crate::Reg<dchpri::DCHPRI_SPEC>,
    #[doc = "0x105 - Channel n Priority Register"]
    pub dchpri6: crate::Reg<dchpri::DCHPRI_SPEC>,
    #[doc = "0x106 - Channel n Priority Register"]
    pub dchpri5: crate::Reg<dchpri::DCHPRI_SPEC>,
    #[doc = "0x107 - Channel n Priority Register"]
    pub dchpri4: crate::Reg<dchpri::DCHPRI_SPEC>,
    _reserved24: [u8; 0x0ef8],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: crate::Reg<tcd_saddr::TCD_SADDR_SPEC>,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: crate::Reg<tcd_soff::TCD_SOFF_SPEC>,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: crate::Reg<tcd_attr::TCD_ATTR_SPEC>,
    _reserved_27_dma_tcd0_nbytes: [u8; 0x04],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: crate::Reg<tcd_slast::TCD_SLAST_SPEC>,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: crate::Reg<tcd_daddr::TCD_DADDR_SPEC>,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: crate::Reg<tcd_doff::TCD_DOFF_SPEC>,
    _reserved_31_dma_tcd0_citer: [u8; 0x02],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: crate::Reg<tcd_csr::TCD_CSR_SPEC>,
    _reserved_34_dma_tcd0_biter: [u8; 0x02],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: crate::Reg<tcd_saddr::TCD_SADDR_SPEC>,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: crate::Reg<tcd_soff::TCD_SOFF_SPEC>,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: crate::Reg<tcd_attr::TCD_ATTR_SPEC>,
    _reserved_38_dma_tcd1_nbytes: [u8; 0x04],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: crate::Reg<tcd_slast::TCD_SLAST_SPEC>,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: crate::Reg<tcd_daddr::TCD_DADDR_SPEC>,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: crate::Reg<tcd_doff::TCD_DOFF_SPEC>,
    _reserved_42_dma_tcd1_citer: [u8; 0x02],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: crate::Reg<tcd_csr::TCD_CSR_SPEC>,
    _reserved_45_dma_tcd1_biter: [u8; 0x02],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: crate::Reg<tcd_saddr::TCD_SADDR_SPEC>,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: crate::Reg<tcd_soff::TCD_SOFF_SPEC>,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: crate::Reg<tcd_attr::TCD_ATTR_SPEC>,
    _reserved_49_dma_tcd2_nbytes: [u8; 0x04],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: crate::Reg<tcd_slast::TCD_SLAST_SPEC>,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: crate::Reg<tcd_daddr::TCD_DADDR_SPEC>,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: crate::Reg<tcd_doff::TCD_DOFF_SPEC>,
    _reserved_53_dma_tcd2_citer: [u8; 0x02],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: crate::Reg<tcd_csr::TCD_CSR_SPEC>,
    _reserved_56_dma_tcd2_biter: [u8; 0x02],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: crate::Reg<tcd_saddr::TCD_SADDR_SPEC>,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: crate::Reg<tcd_soff::TCD_SOFF_SPEC>,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: crate::Reg<tcd_attr::TCD_ATTR_SPEC>,
    _reserved_60_dma_tcd3_nbytes: [u8; 0x04],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: crate::Reg<tcd_slast::TCD_SLAST_SPEC>,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: crate::Reg<tcd_daddr::TCD_DADDR_SPEC>,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: crate::Reg<tcd_doff::TCD_DOFF_SPEC>,
    _reserved_64_dma_tcd3_citer: [u8; 0x02],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: crate::Reg<tcd_csr::TCD_CSR_SPEC>,
    _reserved_67_dma_tcd3_biter: [u8; 0x02],
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: crate::Reg<tcd_saddr::TCD_SADDR_SPEC>,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: crate::Reg<tcd_soff::TCD_SOFF_SPEC>,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: crate::Reg<tcd_attr::TCD_ATTR_SPEC>,
    _reserved_71_dma_tcd4_nbytes: [u8; 0x04],
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: crate::Reg<tcd_slast::TCD_SLAST_SPEC>,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: crate::Reg<tcd_daddr::TCD_DADDR_SPEC>,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: crate::Reg<tcd_doff::TCD_DOFF_SPEC>,
    _reserved_75_dma_tcd4_citer: [u8; 0x02],
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: crate::Reg<tcd_csr::TCD_CSR_SPEC>,
    _reserved_78_dma_tcd4_biter: [u8; 0x02],
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: crate::Reg<tcd_saddr::TCD_SADDR_SPEC>,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: crate::Reg<tcd_soff::TCD_SOFF_SPEC>,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: crate::Reg<tcd_attr::TCD_ATTR_SPEC>,
    _reserved_82_dma_tcd5_nbytes: [u8; 0x04],
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: crate::Reg<tcd_slast::TCD_SLAST_SPEC>,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: crate::Reg<tcd_daddr::TCD_DADDR_SPEC>,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: crate::Reg<tcd_doff::TCD_DOFF_SPEC>,
    _reserved_86_dma_tcd5_citer: [u8; 0x02],
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: crate::Reg<tcd_csr::TCD_CSR_SPEC>,
    _reserved_89_dma_tcd5_biter: [u8; 0x02],
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: crate::Reg<tcd_saddr::TCD_SADDR_SPEC>,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: crate::Reg<tcd_soff::TCD_SOFF_SPEC>,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: crate::Reg<tcd_attr::TCD_ATTR_SPEC>,
    _reserved_93_dma_tcd6_nbytes: [u8; 0x04],
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: crate::Reg<tcd_slast::TCD_SLAST_SPEC>,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: crate::Reg<tcd_daddr::TCD_DADDR_SPEC>,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: crate::Reg<tcd_doff::TCD_DOFF_SPEC>,
    _reserved_97_dma_tcd6_citer: [u8; 0x02],
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: crate::Reg<tcd_csr::TCD_CSR_SPEC>,
    _reserved_100_dma_tcd6_biter: [u8; 0x02],
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: crate::Reg<tcd_saddr::TCD_SADDR_SPEC>,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: crate::Reg<tcd_soff::TCD_SOFF_SPEC>,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: crate::Reg<tcd_attr::TCD_ATTR_SPEC>,
    _reserved_104_dma_tcd7_nbytes: [u8; 0x04],
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: crate::Reg<tcd_slast::TCD_SLAST_SPEC>,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: crate::Reg<tcd_daddr::TCD_DADDR_SPEC>,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: crate::Reg<tcd_doff::TCD_DOFF_SPEC>,
    _reserved_108_dma_tcd7_citer: [u8; 0x02],
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: crate::Reg<tcd_csr::TCD_CSR_SPEC>,
    _reserved_111_dma_tcd7_biter: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd0_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd0_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd0_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize)
                as *const crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd0_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4118usize)
                as *const crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd0_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4118usize)
                as *const crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd0_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4126usize)
                as *const crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd0_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4126usize)
                as *const crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd1_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd1_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd1_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize)
                as *const crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd1_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4150usize)
                as *const crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd1_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4150usize)
                as *const crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd1_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4158usize)
                as *const crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd1_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4158usize)
                as *const crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd2_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd2_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd2_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize)
                as *const crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd2_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4182usize)
                as *const crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd2_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4182usize)
                as *const crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd2_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4190usize)
                as *const crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd2_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4190usize)
                as *const crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd3_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd3_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd3_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize)
                as *const crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd3_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4214usize)
                as *const crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd3_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4214usize)
                as *const crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd3_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4222usize)
                as *const crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd3_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4222usize)
                as *const crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd4_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd4_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd4_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize)
                as *const crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd4_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4246usize)
                as *const crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd4_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4246usize)
                as *const crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd4_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4254usize)
                as *const crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd4_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4254usize)
                as *const crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd5_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd5_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd5_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize)
                as *const crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd5_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4278usize)
                as *const crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd5_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4278usize)
                as *const crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd5_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4286usize)
                as *const crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd5_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4286usize)
                as *const crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd6_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd6_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd6_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize)
                as *const crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd6_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4310usize)
                as *const crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd6_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4310usize)
                as *const crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd6_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4318usize)
                as *const crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd6_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4318usize)
                as *const crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd7_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd7_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize)
                as *const crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd7_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize)
                as *const crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd7_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4342usize)
                as *const crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd7_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4342usize)
                as *const crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd7_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4350usize)
                as *const crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd7_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4350usize)
                as *const crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>)
        }
    }
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "ES register accessor: an alias for `Reg<ES_SPEC>`"]
pub type ES = crate::Reg<es::ES_SPEC>;
#[doc = "Error Status Register"]
pub mod es;
#[doc = "ERQ register accessor: an alias for `Reg<ERQ_SPEC>`"]
pub type ERQ = crate::Reg<erq::ERQ_SPEC>;
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "EEI register accessor: an alias for `Reg<EEI_SPEC>`"]
pub type EEI = crate::Reg<eei::EEI_SPEC>;
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "CEEI register accessor: an alias for `Reg<CEEI_SPEC>`"]
pub type CEEI = crate::Reg<ceei::CEEI_SPEC>;
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "SEEI register accessor: an alias for `Reg<SEEI_SPEC>`"]
pub type SEEI = crate::Reg<seei::SEEI_SPEC>;
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "CERQ register accessor: an alias for `Reg<CERQ_SPEC>`"]
pub type CERQ = crate::Reg<cerq::CERQ_SPEC>;
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "SERQ register accessor: an alias for `Reg<SERQ_SPEC>`"]
pub type SERQ = crate::Reg<serq::SERQ_SPEC>;
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "CDNE register accessor: an alias for `Reg<CDNE_SPEC>`"]
pub type CDNE = crate::Reg<cdne::CDNE_SPEC>;
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "SSRT register accessor: an alias for `Reg<SSRT_SPEC>`"]
pub type SSRT = crate::Reg<ssrt::SSRT_SPEC>;
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "CERR register accessor: an alias for `Reg<CERR_SPEC>`"]
pub type CERR = crate::Reg<cerr::CERR_SPEC>;
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "CINT register accessor: an alias for `Reg<CINT_SPEC>`"]
pub type CINT = crate::Reg<cint::CINT_SPEC>;
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "INT register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "ERR register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error Register"]
pub mod err;
#[doc = "HRS register accessor: an alias for `Reg<HRS_SPEC>`"]
pub type HRS = crate::Reg<hrs::HRS_SPEC>;
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "EARS register accessor: an alias for `Reg<EARS_SPEC>`"]
pub type EARS = crate::Reg<ears::EARS_SPEC>;
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "DCHPRI register accessor: an alias for `Reg<DCHPRI_SPEC>`"]
pub type DCHPRI = crate::Reg<dchpri::DCHPRI_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD_SADDR register accessor: an alias for `Reg<TCD_SADDR_SPEC>`"]
pub type TCD_SADDR = crate::Reg<tcd_saddr::TCD_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD_SOFF register accessor: an alias for `Reg<TCD_SOFF_SPEC>`"]
pub type TCD_SOFF = crate::Reg<tcd_soff::TCD_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD_ATTR register accessor: an alias for `Reg<TCD_ATTR_SPEC>`"]
pub type TCD_ATTR = crate::Reg<tcd_attr::TCD_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "DMA_TCD_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD_NBYTES_MLNO = crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd_nbytes_mlno;
#[doc = "DMA_TCD_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD_NBYTES_MLOFFNO = crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd_nbytes_mloffno;
#[doc = "DMA_TCD_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd_nbytes_mloffyes;
#[doc = "TCD_SLAST register accessor: an alias for `Reg<TCD_SLAST_SPEC>`"]
pub type TCD_SLAST = crate::Reg<tcd_slast::TCD_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD_DADDR register accessor: an alias for `Reg<TCD_DADDR_SPEC>`"]
pub type TCD_DADDR = crate::Reg<tcd_daddr::TCD_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD_DOFF register accessor: an alias for `Reg<TCD_DOFF_SPEC>`"]
pub type TCD_DOFF = crate::Reg<tcd_doff::TCD_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "DMA_TCD_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD_CITER_ELINKNO = crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd_citer_elinkno;
#[doc = "DMA_TCD_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD_CITER_ELINKYES = crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd_citer_elinkyes;
#[doc = "TCD_DLASTSGA register accessor: an alias for `Reg<TCD_DLASTSGA_SPEC>`"]
pub type TCD_DLASTSGA = crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD_CSR register accessor: an alias for `Reg<TCD_CSR_SPEC>`"]
pub type TCD_CSR = crate::Reg<tcd_csr::TCD_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "DMA_TCD_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD_BITER_ELINKNO = crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd_biter_elinkno;
#[doc = "DMA_TCD_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD_BITER_ELINKYES = crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd_biter_elinkyes;
