#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral ID register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Peripheral ID Complement register"]
    pub idcomp: crate::Reg<idcomp::IDCOMP_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Peripheral Revision register"]
    pub rev: crate::Reg<rev::REV_SPEC>,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - Peripheral Additional Info register"]
    pub addinfo: crate::Reg<addinfo::ADDINFO_SPEC>,
    _reserved4: [u8; 0x03],
    #[doc = "0x10 - OTG Interrupt Status register"]
    pub otgistat: crate::Reg<otgistat::OTGISTAT_SPEC>,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - OTG Interrupt Control register"]
    pub otgicr: crate::Reg<otgicr::OTGICR_SPEC>,
    _reserved6: [u8; 0x03],
    #[doc = "0x18 - OTG Status register"]
    pub otgstat: crate::Reg<otgstat::OTGSTAT_SPEC>,
    _reserved7: [u8; 0x03],
    #[doc = "0x1c - OTG Control register"]
    pub otgctl: crate::Reg<otgctl::OTGCTL_SPEC>,
    _reserved8: [u8; 0x63],
    #[doc = "0x80 - Interrupt Status register"]
    pub istat: crate::Reg<istat::ISTAT_SPEC>,
    _reserved9: [u8; 0x03],
    #[doc = "0x84 - Interrupt Enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    _reserved10: [u8; 0x03],
    #[doc = "0x88 - Error Interrupt Status register"]
    pub errstat: crate::Reg<errstat::ERRSTAT_SPEC>,
    _reserved11: [u8; 0x03],
    #[doc = "0x8c - Error Interrupt Enable register"]
    pub erren: crate::Reg<erren::ERREN_SPEC>,
    _reserved12: [u8; 0x03],
    #[doc = "0x90 - Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    _reserved13: [u8; 0x03],
    #[doc = "0x94 - Control register"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    _reserved14: [u8; 0x03],
    #[doc = "0x98 - Address register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    _reserved15: [u8; 0x03],
    #[doc = "0x9c - BDT Page register 1"]
    pub bdtpage1: crate::Reg<bdtpage1::BDTPAGE1_SPEC>,
    _reserved16: [u8; 0x03],
    #[doc = "0xa0 - Frame Number register Low"]
    pub frmnuml: crate::Reg<frmnuml::FRMNUML_SPEC>,
    _reserved17: [u8; 0x03],
    #[doc = "0xa4 - Frame Number register High"]
    pub frmnumh: crate::Reg<frmnumh::FRMNUMH_SPEC>,
    _reserved18: [u8; 0x03],
    #[doc = "0xa8 - Token register"]
    pub token: crate::Reg<token::TOKEN_SPEC>,
    _reserved19: [u8; 0x03],
    #[doc = "0xac - SOF Threshold register"]
    pub softhld: crate::Reg<softhld::SOFTHLD_SPEC>,
    _reserved20: [u8; 0x03],
    #[doc = "0xb0 - BDT Page Register 2"]
    pub bdtpage2: crate::Reg<bdtpage2::BDTPAGE2_SPEC>,
    _reserved21: [u8; 0x03],
    #[doc = "0xb4 - BDT Page Register 3"]
    pub bdtpage3: crate::Reg<bdtpage3::BDTPAGE3_SPEC>,
    _reserved22: [u8; 0x0b],
    #[doc = "0xc0 - Endpoint Control register"]
    pub endpt0: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved23: [u8; 0x03],
    #[doc = "0xc4 - Endpoint Control register"]
    pub endpt1: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved24: [u8; 0x03],
    #[doc = "0xc8 - Endpoint Control register"]
    pub endpt2: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved25: [u8; 0x03],
    #[doc = "0xcc - Endpoint Control register"]
    pub endpt3: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved26: [u8; 0x03],
    #[doc = "0xd0 - Endpoint Control register"]
    pub endpt4: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved27: [u8; 0x03],
    #[doc = "0xd4 - Endpoint Control register"]
    pub endpt5: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved28: [u8; 0x03],
    #[doc = "0xd8 - Endpoint Control register"]
    pub endpt6: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved29: [u8; 0x03],
    #[doc = "0xdc - Endpoint Control register"]
    pub endpt7: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved30: [u8; 0x03],
    #[doc = "0xe0 - Endpoint Control register"]
    pub endpt8: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved31: [u8; 0x03],
    #[doc = "0xe4 - Endpoint Control register"]
    pub endpt9: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved32: [u8; 0x03],
    #[doc = "0xe8 - Endpoint Control register"]
    pub endpt10: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved33: [u8; 0x03],
    #[doc = "0xec - Endpoint Control register"]
    pub endpt11: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved34: [u8; 0x03],
    #[doc = "0xf0 - Endpoint Control register"]
    pub endpt12: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved35: [u8; 0x03],
    #[doc = "0xf4 - Endpoint Control register"]
    pub endpt13: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved36: [u8; 0x03],
    #[doc = "0xf8 - Endpoint Control register"]
    pub endpt14: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved37: [u8; 0x03],
    #[doc = "0xfc - Endpoint Control register"]
    pub endpt15: crate::Reg<endpt::ENDPT_SPEC>,
    _reserved38: [u8; 0x03],
    #[doc = "0x100 - USB Control register"]
    pub usbctrl: crate::Reg<usbctrl::USBCTRL_SPEC>,
    _reserved39: [u8; 0x03],
    #[doc = "0x104 - USB OTG Observe register"]
    pub observe: crate::Reg<observe::OBSERVE_SPEC>,
    _reserved40: [u8; 0x03],
    #[doc = "0x108 - USB OTG Control register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    _reserved41: [u8; 0x03],
    #[doc = "0x10c - USB Transceiver Control register 0"]
    pub usbtrc0: crate::Reg<usbtrc0::USBTRC0_SPEC>,
    _reserved42: [u8; 0x07],
    #[doc = "0x114 - Frame Adjust Register"]
    pub usbfrmadjust: crate::Reg<usbfrmadjust::USBFRMADJUST_SPEC>,
    _reserved43: [u8; 0x0f],
    #[doc = "0x124 - Keep Alive mode control"]
    pub keep_alive_ctrl: crate::Reg<keep_alive_ctrl::KEEP_ALIVE_CTRL_SPEC>,
    _reserved44: [u8; 0x03],
    #[doc = "0x128 - Keep Alive mode wakeup control"]
    pub keep_alive_wkctrl: crate::Reg<keep_alive_wkctrl::KEEP_ALIVE_WKCTRL_SPEC>,
    _reserved45: [u8; 0x03],
    #[doc = "0x12c - Miscellaneous Control register"]
    pub miscctrl: crate::Reg<miscctrl::MISCCTRL_SPEC>,
    _reserved46: [u8; 0x03],
    #[doc = "0x130 - Peripheral mode stall disable for endpoints 7 to 0 in IN direction"]
    pub stall_il_dis: crate::Reg<stall_il_dis::STALL_IL_DIS_SPEC>,
    _reserved47: [u8; 0x03],
    #[doc = "0x134 - Peripheral mode stall disable for endpoints 15 to 8 in IN direction"]
    pub stall_ih_dis: crate::Reg<stall_ih_dis::STALL_IH_DIS_SPEC>,
    _reserved48: [u8; 0x03],
    #[doc = "0x138 - Peripheral mode stall disable for endpoints 7 to 0 in OUT direction"]
    pub stall_ol_dis: crate::Reg<stall_ol_dis::STALL_OL_DIS_SPEC>,
    _reserved49: [u8; 0x03],
    #[doc = "0x13c - Peripheral mode stall disable for endpoints 15 to 8 in OUT direction"]
    pub stall_oh_dis: crate::Reg<stall_oh_dis::STALL_OH_DIS_SPEC>,
    _reserved50: [u8; 0x03],
    #[doc = "0x140 - USB Clock recovery control"]
    pub clk_recover_ctrl: crate::Reg<clk_recover_ctrl::CLK_RECOVER_CTRL_SPEC>,
    _reserved51: [u8; 0x03],
    #[doc = "0x144 - IRC48M oscillator enable register"]
    pub clk_recover_irc_en: crate::Reg<clk_recover_irc_en::CLK_RECOVER_IRC_EN_SPEC>,
    _reserved52: [u8; 0x0f],
    #[doc = "0x154 - Clock recovery combined interrupt enable"]
    pub clk_recover_int_en: crate::Reg<clk_recover_int_en::CLK_RECOVER_INT_EN_SPEC>,
    _reserved53: [u8; 0x07],
    #[doc = "0x15c - Clock recovery separated interrupt status"]
    pub clk_recover_int_status: crate::Reg<clk_recover_int_status::CLK_RECOVER_INT_STATUS_SPEC>,
}
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID register"]
pub mod perid;
#[doc = "IDCOMP register accessor: an alias for `Reg<IDCOMP_SPEC>`"]
pub type IDCOMP = crate::Reg<idcomp::IDCOMP_SPEC>;
#[doc = "Peripheral ID Complement register"]
pub mod idcomp;
#[doc = "REV register accessor: an alias for `Reg<REV_SPEC>`"]
pub type REV = crate::Reg<rev::REV_SPEC>;
#[doc = "Peripheral Revision register"]
pub mod rev;
#[doc = "ADDINFO register accessor: an alias for `Reg<ADDINFO_SPEC>`"]
pub type ADDINFO = crate::Reg<addinfo::ADDINFO_SPEC>;
#[doc = "Peripheral Additional Info register"]
pub mod addinfo;
#[doc = "OTGISTAT register accessor: an alias for `Reg<OTGISTAT_SPEC>`"]
pub type OTGISTAT = crate::Reg<otgistat::OTGISTAT_SPEC>;
#[doc = "OTG Interrupt Status register"]
pub mod otgistat;
#[doc = "OTGICR register accessor: an alias for `Reg<OTGICR_SPEC>`"]
pub type OTGICR = crate::Reg<otgicr::OTGICR_SPEC>;
#[doc = "OTG Interrupt Control register"]
pub mod otgicr;
#[doc = "OTGSTAT register accessor: an alias for `Reg<OTGSTAT_SPEC>`"]
pub type OTGSTAT = crate::Reg<otgstat::OTGSTAT_SPEC>;
#[doc = "OTG Status register"]
pub mod otgstat;
#[doc = "OTGCTL register accessor: an alias for `Reg<OTGCTL_SPEC>`"]
pub type OTGCTL = crate::Reg<otgctl::OTGCTL_SPEC>;
#[doc = "OTG Control register"]
pub mod otgctl;
#[doc = "ISTAT register accessor: an alias for `Reg<ISTAT_SPEC>`"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "Interrupt Status register"]
pub mod istat;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable register"]
pub mod inten;
#[doc = "ERRSTAT register accessor: an alias for `Reg<ERRSTAT_SPEC>`"]
pub type ERRSTAT = crate::Reg<errstat::ERRSTAT_SPEC>;
#[doc = "Error Interrupt Status register"]
pub mod errstat;
#[doc = "ERREN register accessor: an alias for `Reg<ERREN_SPEC>`"]
pub type ERREN = crate::Reg<erren::ERREN_SPEC>;
#[doc = "Error Interrupt Enable register"]
pub mod erren;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register"]
pub mod addr;
#[doc = "BDTPAGE1 register accessor: an alias for `Reg<BDTPAGE1_SPEC>`"]
pub type BDTPAGE1 = crate::Reg<bdtpage1::BDTPAGE1_SPEC>;
#[doc = "BDT Page register 1"]
pub mod bdtpage1;
#[doc = "FRMNUML register accessor: an alias for `Reg<FRMNUML_SPEC>`"]
pub type FRMNUML = crate::Reg<frmnuml::FRMNUML_SPEC>;
#[doc = "Frame Number register Low"]
pub mod frmnuml;
#[doc = "FRMNUMH register accessor: an alias for `Reg<FRMNUMH_SPEC>`"]
pub type FRMNUMH = crate::Reg<frmnumh::FRMNUMH_SPEC>;
#[doc = "Frame Number register High"]
pub mod frmnumh;
#[doc = "TOKEN register accessor: an alias for `Reg<TOKEN_SPEC>`"]
pub type TOKEN = crate::Reg<token::TOKEN_SPEC>;
#[doc = "Token register"]
pub mod token;
#[doc = "SOFTHLD register accessor: an alias for `Reg<SOFTHLD_SPEC>`"]
pub type SOFTHLD = crate::Reg<softhld::SOFTHLD_SPEC>;
#[doc = "SOF Threshold register"]
pub mod softhld;
#[doc = "BDTPAGE2 register accessor: an alias for `Reg<BDTPAGE2_SPEC>`"]
pub type BDTPAGE2 = crate::Reg<bdtpage2::BDTPAGE2_SPEC>;
#[doc = "BDT Page Register 2"]
pub mod bdtpage2;
#[doc = "BDTPAGE3 register accessor: an alias for `Reg<BDTPAGE3_SPEC>`"]
pub type BDTPAGE3 = crate::Reg<bdtpage3::BDTPAGE3_SPEC>;
#[doc = "BDT Page Register 3"]
pub mod bdtpage3;
#[doc = "ENDPT register accessor: an alias for `Reg<ENDPT_SPEC>`"]
pub type ENDPT = crate::Reg<endpt::ENDPT_SPEC>;
#[doc = "Endpoint Control register"]
pub mod endpt;
#[doc = "USBCTRL register accessor: an alias for `Reg<USBCTRL_SPEC>`"]
pub type USBCTRL = crate::Reg<usbctrl::USBCTRL_SPEC>;
#[doc = "USB Control register"]
pub mod usbctrl;
#[doc = "OBSERVE register accessor: an alias for `Reg<OBSERVE_SPEC>`"]
pub type OBSERVE = crate::Reg<observe::OBSERVE_SPEC>;
#[doc = "USB OTG Observe register"]
pub mod observe;
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "USB OTG Control register"]
pub mod control;
#[doc = "USBTRC0 register accessor: an alias for `Reg<USBTRC0_SPEC>`"]
pub type USBTRC0 = crate::Reg<usbtrc0::USBTRC0_SPEC>;
#[doc = "USB Transceiver Control register 0"]
pub mod usbtrc0;
#[doc = "USBFRMADJUST register accessor: an alias for `Reg<USBFRMADJUST_SPEC>`"]
pub type USBFRMADJUST = crate::Reg<usbfrmadjust::USBFRMADJUST_SPEC>;
#[doc = "Frame Adjust Register"]
pub mod usbfrmadjust;
#[doc = "KEEP_ALIVE_CTRL register accessor: an alias for `Reg<KEEP_ALIVE_CTRL_SPEC>`"]
pub type KEEP_ALIVE_CTRL = crate::Reg<keep_alive_ctrl::KEEP_ALIVE_CTRL_SPEC>;
#[doc = "Keep Alive mode control"]
pub mod keep_alive_ctrl;
#[doc = "KEEP_ALIVE_WKCTRL register accessor: an alias for `Reg<KEEP_ALIVE_WKCTRL_SPEC>`"]
pub type KEEP_ALIVE_WKCTRL = crate::Reg<keep_alive_wkctrl::KEEP_ALIVE_WKCTRL_SPEC>;
#[doc = "Keep Alive mode wakeup control"]
pub mod keep_alive_wkctrl;
#[doc = "MISCCTRL register accessor: an alias for `Reg<MISCCTRL_SPEC>`"]
pub type MISCCTRL = crate::Reg<miscctrl::MISCCTRL_SPEC>;
#[doc = "Miscellaneous Control register"]
pub mod miscctrl;
#[doc = "STALL_IL_DIS register accessor: an alias for `Reg<STALL_IL_DIS_SPEC>`"]
pub type STALL_IL_DIS = crate::Reg<stall_il_dis::STALL_IL_DIS_SPEC>;
#[doc = "Peripheral mode stall disable for endpoints 7 to 0 in IN direction"]
pub mod stall_il_dis;
#[doc = "STALL_IH_DIS register accessor: an alias for `Reg<STALL_IH_DIS_SPEC>`"]
pub type STALL_IH_DIS = crate::Reg<stall_ih_dis::STALL_IH_DIS_SPEC>;
#[doc = "Peripheral mode stall disable for endpoints 15 to 8 in IN direction"]
pub mod stall_ih_dis;
#[doc = "STALL_OL_DIS register accessor: an alias for `Reg<STALL_OL_DIS_SPEC>`"]
pub type STALL_OL_DIS = crate::Reg<stall_ol_dis::STALL_OL_DIS_SPEC>;
#[doc = "Peripheral mode stall disable for endpoints 7 to 0 in OUT direction"]
pub mod stall_ol_dis;
#[doc = "STALL_OH_DIS register accessor: an alias for `Reg<STALL_OH_DIS_SPEC>`"]
pub type STALL_OH_DIS = crate::Reg<stall_oh_dis::STALL_OH_DIS_SPEC>;
#[doc = "Peripheral mode stall disable for endpoints 15 to 8 in OUT direction"]
pub mod stall_oh_dis;
#[doc = "CLK_RECOVER_CTRL register accessor: an alias for `Reg<CLK_RECOVER_CTRL_SPEC>`"]
pub type CLK_RECOVER_CTRL = crate::Reg<clk_recover_ctrl::CLK_RECOVER_CTRL_SPEC>;
#[doc = "USB Clock recovery control"]
pub mod clk_recover_ctrl;
#[doc = "CLK_RECOVER_IRC_EN register accessor: an alias for `Reg<CLK_RECOVER_IRC_EN_SPEC>`"]
pub type CLK_RECOVER_IRC_EN = crate::Reg<clk_recover_irc_en::CLK_RECOVER_IRC_EN_SPEC>;
#[doc = "IRC48M oscillator enable register"]
pub mod clk_recover_irc_en;
#[doc = "CLK_RECOVER_INT_EN register accessor: an alias for `Reg<CLK_RECOVER_INT_EN_SPEC>`"]
pub type CLK_RECOVER_INT_EN = crate::Reg<clk_recover_int_en::CLK_RECOVER_INT_EN_SPEC>;
#[doc = "Clock recovery combined interrupt enable"]
pub mod clk_recover_int_en;
#[doc = "CLK_RECOVER_INT_STATUS register accessor: an alias for `Reg<CLK_RECOVER_INT_STATUS_SPEC>`"]
pub type CLK_RECOVER_INT_STATUS = crate::Reg<clk_recover_int_status::CLK_RECOVER_INT_STATUS_SPEC>;
#[doc = "Clock recovery separated interrupt status"]
pub mod clk_recover_int_status;
