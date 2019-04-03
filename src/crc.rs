#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CRC_CRCLL register. CRC_CRCL register. CRC Data register CRC_CRCLU register. CRC_CRCHL register. CRC_CRCH register. CRC_CRCHU register."]
    pub crch: CRCH_UNION,
    #[doc = "CRC_GPOLYLL register. CRC_GPOLYL register. CRC Polynomial register CRC_GPOLYLU register. CRC_GPOLYHL register. CRC_GPOLYH register. CRC_GPOLYHU register."]
    pub gpoly: GPOLY_UNION,
    #[doc = "CRC Control register CRC_CTRLHU register."]
    pub ctrl: CTRL_UNION,
}
#[doc = "CRC_CRCLL register. CRC_CRCL register. CRC Data register CRC_CRCLU register. CRC_CRCHL register. CRC_CRCH register. CRC_CRCHU register."]
#[repr(C)]
pub union CRCH_UNION {
    #[doc = "0x00 - CRC_CRCLL register."]
    pub crcll: CRCLL,
    #[doc = "0x00 - CRC_CRCL register."]
    pub crcl: CRCL,
    #[doc = "0x00 - CRC Data register"]
    pub data: DATA,
    #[doc = "0x01 - CRC_CRCLU register."]
    pub crclu: CRCLU,
    #[doc = "0x02 - CRC_CRCHL register."]
    pub crchl: CRCHL,
    #[doc = "0x02 - CRC_CRCH register."]
    pub crch: CRCH,
    #[doc = "0x03 - CRC_CRCHU register."]
    pub crchu: CRCHU,
}
#[doc = "CRC_GPOLYLL register. CRC_GPOLYL register. CRC Polynomial register CRC_GPOLYLU register. CRC_GPOLYHL register. CRC_GPOLYH register. CRC_GPOLYHU register."]
#[repr(C)]
pub union GPOLY_UNION {
    #[doc = "0x04 - CRC_GPOLYLL register."]
    pub gpolyll: GPOLYLL,
    #[doc = "0x04 - CRC_GPOLYL register."]
    pub gpolyl: GPOLYL,
    #[doc = "0x04 - CRC Polynomial register"]
    pub gpoly: GPOLY,
    #[doc = "0x05 - CRC_GPOLYLU register."]
    pub gpolylu: GPOLYLU,
    #[doc = "0x06 - CRC_GPOLYHL register."]
    pub gpolyhl: GPOLYHL,
    #[doc = "0x06 - CRC_GPOLYH register."]
    pub gpolyh: GPOLYH,
    #[doc = "0x07 - CRC_GPOLYHU register."]
    pub gpolyhu: GPOLYHU,
}
#[doc = "CRC Control register CRC_CTRLHU register."]
#[repr(C)]
pub union CTRL_UNION {
    #[doc = "0x08 - CRC Control register"]
    pub ctrl: CTRL,
    #[doc = "0x0b - CRC_CTRLHU register."]
    pub ctrlhu: CTRLHU,
}
#[doc = "CRC Data register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Data register"]
pub mod data;
#[doc = "CRC_CRCL register."]
pub struct CRCL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC_CRCL register."]
pub mod crcl;
#[doc = "CRC_CRCLL register."]
pub struct CRCLL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CRCLL register."]
pub mod crcll;
#[doc = "CRC_CRCLU register."]
pub struct CRCLU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CRCLU register."]
pub mod crclu;
#[doc = "CRC_CRCH register."]
pub struct CRCH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC_CRCH register."]
pub mod crch;
#[doc = "CRC_CRCHL register."]
pub struct CRCHL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CRCHL register."]
pub mod crchl;
#[doc = "CRC_CRCHU register."]
pub struct CRCHU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CRCHU register."]
pub mod crchu;
#[doc = "CRC Polynomial register"]
pub struct GPOLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Polynomial register"]
pub mod gpoly;
#[doc = "CRC_GPOLYL register."]
pub struct GPOLYL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC_GPOLYL register."]
pub mod gpolyl;
#[doc = "CRC_GPOLYLL register."]
pub struct GPOLYLL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_GPOLYLL register."]
pub mod gpolyll;
#[doc = "CRC_GPOLYLU register."]
pub struct GPOLYLU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_GPOLYLU register."]
pub mod gpolylu;
#[doc = "CRC_GPOLYH register."]
pub struct GPOLYH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC_GPOLYH register."]
pub mod gpolyh;
#[doc = "CRC_GPOLYHL register."]
pub struct GPOLYHL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_GPOLYHL register."]
pub mod gpolyhl;
#[doc = "CRC_GPOLYHU register."]
pub struct GPOLYHU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_GPOLYHU register."]
pub mod gpolyhu;
#[doc = "CRC Control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Control register"]
pub mod ctrl;
#[doc = "CRC_CTRLHU register."]
pub struct CTRLHU {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CRC_CTRLHU register."]
pub mod ctrlhu;
