#![doc = "Peripheral access API for MKL82Z7 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(untagged_unions)]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0_DMA4();
    fn DMA1_DMA5();
    fn DMA2_DMA6();
    fn DMA3_DMA7();
    fn DMA_ERROR();
    fn FLEXIO0();
    fn TPM0();
    fn TPM1();
    fn TPM2();
    fn PIT0();
    fn SPI0();
    fn EMVSIM0();
    fn LPUART0();
    fn LPUART1();
    fn I2C0();
    fn QSPI0();
    fn PORTA();
    fn PORTB();
    fn PORTC();
    fn PORTD();
    fn PORTE();
    fn LLWU();
    fn LTC0();
    fn USB0();
    fn ADC0();
    fn LPTMR0();
    fn RTC_SECONDS();
    fn INTMUX0_0();
    fn INTMUX0_1();
    fn INTMUX0_2();
    fn INTMUX0_3();
    fn LPTMR1();
    fn SPI1();
    fn LPUART2();
    fn EMVSIM1();
    fn I2C1();
    fn TSI0();
    fn PMC();
    fn FTFA();
    fn MCG();
    fn WDOG_EWM();
    fn DAC0();
    fn TRNG0();
    fn CMP0();
    fn RTC_ALARM();
    fn DMA4();
    fn DMA5();
    fn DMA6();
    fn DMA7();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 60] = [
    Vector {
        _handler: DMA0_DMA4,
    },
    Vector {
        _handler: DMA1_DMA5,
    },
    Vector {
        _handler: DMA2_DMA6,
    },
    Vector {
        _handler: DMA3_DMA7,
    },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector { _handler: FLEXIO0 },
    Vector { _handler: TPM0 },
    Vector { _handler: TPM1 },
    Vector { _handler: TPM2 },
    Vector { _handler: PIT0 },
    Vector { _handler: SPI0 },
    Vector { _handler: EMVSIM0 },
    Vector { _handler: LPUART0 },
    Vector { _handler: LPUART1 },
    Vector { _handler: I2C0 },
    Vector { _handler: QSPI0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORTA },
    Vector { _handler: PORTB },
    Vector { _handler: PORTC },
    Vector { _handler: PORTD },
    Vector { _handler: PORTE },
    Vector { _handler: LLWU },
    Vector { _handler: LTC0 },
    Vector { _handler: USB0 },
    Vector { _handler: ADC0 },
    Vector { _handler: LPTMR0 },
    Vector {
        _handler: RTC_SECONDS,
    },
    Vector {
        _handler: INTMUX0_0,
    },
    Vector {
        _handler: INTMUX0_1,
    },
    Vector {
        _handler: INTMUX0_2,
    },
    Vector {
        _handler: INTMUX0_3,
    },
    Vector { _handler: LPTMR1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: EMVSIM1 },
    Vector { _handler: I2C1 },
    Vector { _handler: TSI0 },
    Vector { _handler: PMC },
    Vector { _handler: FTFA },
    Vector { _handler: MCG },
    Vector { _handler: WDOG_EWM },
    Vector { _handler: DAC0 },
    Vector { _handler: TRNG0 },
    Vector { _reserved: 0 },
    Vector { _handler: CMP0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA4 },
    Vector { _handler: DMA5 },
    Vector { _handler: DMA6 },
    Vector { _handler: DMA7 },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - DMA0_DMA4"]
    DMA0_DMA4,
    #[doc = "1 - DMA1_DMA5"]
    DMA1_DMA5,
    #[doc = "2 - DMA2_DMA6"]
    DMA2_DMA6,
    #[doc = "3 - DMA3_DMA7"]
    DMA3_DMA7,
    #[doc = "4 - DMA_Error"]
    DMA_ERROR,
    #[doc = "5 - FLEXIO0"]
    FLEXIO0,
    #[doc = "6 - TPM0"]
    TPM0,
    #[doc = "7 - TPM1"]
    TPM1,
    #[doc = "8 - TPM2"]
    TPM2,
    #[doc = "9 - PIT0"]
    PIT0,
    #[doc = "10 - SPI0"]
    SPI0,
    #[doc = "11 - EMVSIM0"]
    EMVSIM0,
    #[doc = "12 - LPUART0"]
    LPUART0,
    #[doc = "13 - LPUART1"]
    LPUART1,
    #[doc = "14 - I2C0"]
    I2C0,
    #[doc = "15 - QSPI0"]
    QSPI0,
    #[doc = "17 - PORTA"]
    PORTA,
    #[doc = "18 - PORTB"]
    PORTB,
    #[doc = "19 - PORTC"]
    PORTC,
    #[doc = "20 - PORTD"]
    PORTD,
    #[doc = "21 - PORTE"]
    PORTE,
    #[doc = "22 - LLWU"]
    LLWU,
    #[doc = "23 - LTC0"]
    LTC0,
    #[doc = "24 - USB0"]
    USB0,
    #[doc = "25 - ADC0"]
    ADC0,
    #[doc = "26 - LPTMR0"]
    LPTMR0,
    #[doc = "27 - RTC_Seconds"]
    RTC_SECONDS,
    #[doc = "28 - INTMUX0_0"]
    INTMUX0_0,
    #[doc = "29 - INTMUX0_1"]
    INTMUX0_1,
    #[doc = "30 - INTMUX0_2"]
    INTMUX0_2,
    #[doc = "31 - INTMUX0_3"]
    INTMUX0_3,
    #[doc = "32 - LPTMR1"]
    LPTMR1,
    #[doc = "36 - SPI1"]
    SPI1,
    #[doc = "37 - LPUART2"]
    LPUART2,
    #[doc = "38 - EMVSIM1"]
    EMVSIM1,
    #[doc = "39 - I2C1"]
    I2C1,
    #[doc = "40 - TSI0"]
    TSI0,
    #[doc = "41 - PMC"]
    PMC,
    #[doc = "42 - FTFA"]
    FTFA,
    #[doc = "43 - MCG"]
    MCG,
    #[doc = "44 - WDOG_EWM"]
    WDOG_EWM,
    #[doc = "45 - DAC0"]
    DAC0,
    #[doc = "46 - TRNG0"]
    TRNG0,
    #[doc = "48 - CMP0"]
    CMP0,
    #[doc = "50 - RTC_Alarm"]
    RTC_ALARM,
    #[doc = "56 - DMA4"]
    DMA4,
    #[doc = "57 - DMA5"]
    DMA5,
    #[doc = "58 - DMA6"]
    DMA6,
    #[doc = "59 - DMA7"]
    DMA7,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA0_DMA4 => 0,
            Interrupt::DMA1_DMA5 => 1,
            Interrupt::DMA2_DMA6 => 2,
            Interrupt::DMA3_DMA7 => 3,
            Interrupt::DMA_ERROR => 4,
            Interrupt::FLEXIO0 => 5,
            Interrupt::TPM0 => 6,
            Interrupt::TPM1 => 7,
            Interrupt::TPM2 => 8,
            Interrupt::PIT0 => 9,
            Interrupt::SPI0 => 10,
            Interrupt::EMVSIM0 => 11,
            Interrupt::LPUART0 => 12,
            Interrupt::LPUART1 => 13,
            Interrupt::I2C0 => 14,
            Interrupt::QSPI0 => 15,
            Interrupt::PORTA => 17,
            Interrupt::PORTB => 18,
            Interrupt::PORTC => 19,
            Interrupt::PORTD => 20,
            Interrupt::PORTE => 21,
            Interrupt::LLWU => 22,
            Interrupt::LTC0 => 23,
            Interrupt::USB0 => 24,
            Interrupt::ADC0 => 25,
            Interrupt::LPTMR0 => 26,
            Interrupt::RTC_SECONDS => 27,
            Interrupt::INTMUX0_0 => 28,
            Interrupt::INTMUX0_1 => 29,
            Interrupt::INTMUX0_2 => 30,
            Interrupt::INTMUX0_3 => 31,
            Interrupt::LPTMR1 => 32,
            Interrupt::SPI1 => 36,
            Interrupt::LPUART2 => 37,
            Interrupt::EMVSIM1 => 38,
            Interrupt::I2C1 => 39,
            Interrupt::TSI0 => 40,
            Interrupt::PMC => 41,
            Interrupt::FTFA => 42,
            Interrupt::MCG => 43,
            Interrupt::WDOG_EWM => 44,
            Interrupt::DAC0 => 45,
            Interrupt::TRNG0 => 46,
            Interrupt::CMP0 => 48,
            Interrupt::RTC_ALARM => 50,
            Interrupt::DMA4 => 56,
            Interrupt::DMA5 => 57,
            Interrupt::DMA6 => 58,
            Interrupt::DMA7 => 59,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Flash configuration field"]
pub struct FTFA_FLASHCONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA_FLASHCONFIG {}
impl FTFA_FLASHCONFIG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfa_flash_config::RegisterBlock {
        1024 as *const _
    }
}
impl Deref for FTFA_FLASHCONFIG {
    type Target = ftfa_flash_config::RegisterBlock;
    fn deref(&self) -> &ftfa_flash_config::RegisterBlock {
        unsafe { &*FTFA_FLASHCONFIG::ptr() }
    }
}
#[doc = "Flash configuration field"]
pub mod ftfa_flash_config;
#[doc = "AIPS-Lite Bridge"]
pub struct AIPS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPS {}
impl AIPS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aips::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for AIPS {
    type Target = aips::RegisterBlock;
    fn deref(&self) -> &aips::RegisterBlock {
        unsafe { &*AIPS::ptr() }
    }
}
#[doc = "AIPS-Lite Bridge"]
pub mod aips;
#[doc = "Enhanced direct memory access controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "Enhanced direct memory access controller"]
pub mod dma;
#[doc = "Flash Memory Interface"]
pub struct FTFA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA {}
impl FTFA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfa::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for FTFA {
    type Target = ftfa::RegisterBlock;
    fn deref(&self) -> &ftfa::RegisterBlock {
        unsafe { &*FTFA::ptr() }
    }
}
#[doc = "Flash Memory Interface"]
pub mod ftfa;
#[doc = "DMA channel multiplexor"]
pub struct DMAMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX {}
impl DMAMUX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmamux::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    fn deref(&self) -> &dmamux::RegisterBlock {
        unsafe { &*DMAMUX::ptr() }
    }
}
#[doc = "DMA channel multiplexor"]
pub mod dmamux;
#[doc = "Interrupt Multiplexer"]
pub struct INTMUX0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INTMUX0 {}
impl INTMUX0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const intmux0::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for INTMUX0 {
    type Target = intmux0::RegisterBlock;
    fn deref(&self) -> &intmux0::RegisterBlock {
        unsafe { &*INTMUX0::ptr() }
    }
}
#[doc = "Interrupt Multiplexer"]
pub mod intmux0;
#[doc = "TRNG0"]
pub struct TRNG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG0 {}
impl TRNG0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const trng0::RegisterBlock {
        1073893376 as *const _
    }
}
impl Deref for TRNG0 {
    type Target = trng0::RegisterBlock;
    fn deref(&self) -> &trng0::RegisterBlock {
        unsafe { &*TRNG0::ptr() }
    }
}
#[doc = "TRNG0"]
pub mod trng0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073926144 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi1;
#[doc = "Cyclic Redundancy Check"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073946624 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check"]
pub mod crc;
#[doc = "Periodic Interrupt Timer"]
pub struct PIT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT0 {}
impl PIT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pit0::RegisterBlock {
        1073967104 as *const _
    }
}
impl Deref for PIT0 {
    type Target = pit0::RegisterBlock;
    fn deref(&self) -> &pit0::RegisterBlock {
        unsafe { &*PIT0::ptr() }
    }
}
#[doc = "Periodic Interrupt Timer"]
pub mod pit0;
#[doc = "Timer/PWM Module"]
pub struct TPM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM0 {}
impl TPM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm0::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for TPM0 {
    type Target = tpm0::RegisterBlock;
    fn deref(&self) -> &tpm0::RegisterBlock {
        unsafe { &*TPM0::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm0;
#[doc = "Timer/PWM Module"]
pub struct TPM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM1 {}
impl TPM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm1::RegisterBlock {
        1073975296 as *const _
    }
}
impl Deref for TPM1 {
    type Target = tpm1::RegisterBlock;
    fn deref(&self) -> &tpm1::RegisterBlock {
        unsafe { &*TPM1::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm1;
#[doc = "Timer/PWM Module"]
pub struct TPM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM2 {}
impl TPM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpm2::RegisterBlock {
        1073979392 as *const _
    }
}
impl Deref for TPM2 {
    type Target = tpm2::RegisterBlock;
    fn deref(&self) -> &tpm2::RegisterBlock {
        unsafe { &*TPM2::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm2;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1073983488 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Secure Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073991680 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Secure Real Time Clock"]
pub mod rtc;
#[doc = "VBAT register file"]
pub struct RFVBAT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFVBAT {}
impl RFVBAT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rfvbat::RegisterBlock {
        1073995776 as *const _
    }
}
impl Deref for RFVBAT {
    type Target = rfvbat::RegisterBlock;
    fn deref(&self) -> &rfvbat::RegisterBlock {
        unsafe { &*RFVBAT::ptr() }
    }
}
#[doc = "VBAT register file"]
pub mod rfvbat;
#[doc = "12-Bit Digital-to-Analog Converter"]
pub struct DAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC0 {}
impl DAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac0::RegisterBlock {
        1073999872 as *const _
    }
}
impl Deref for DAC0 {
    type Target = dac0::RegisterBlock;
    fn deref(&self) -> &dac0::RegisterBlock {
        unsafe { &*DAC0::ptr() }
    }
}
#[doc = "12-Bit Digital-to-Analog Converter"]
pub mod dac0;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptmr0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    fn deref(&self) -> &lptmr0::RegisterBlock {
        unsafe { &*LPTMR0::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "Low Power Timer"]
pub struct LPTMR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR1 {}
impl LPTMR1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptmr1::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for LPTMR1 {
    type Target = lptmr1::RegisterBlock;
    fn deref(&self) -> &lptmr1::RegisterBlock {
        unsafe { &*LPTMR1::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr1;
#[doc = "System register file"]
pub struct RFSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFSYS {}
impl RFSYS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rfsys::RegisterBlock {
        1074008064 as *const _
    }
}
impl Deref for RFSYS {
    type Target = rfsys::RegisterBlock;
    fn deref(&self) -> &rfsys::RegisterBlock {
        unsafe { &*RFSYS::ptr() }
    }
}
#[doc = "System register file"]
pub mod rfsys;
#[doc = "Touch sense input"]
pub struct TSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSI0 {}
impl TSI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsi0::RegisterBlock {
        1074024448 as *const _
    }
}
impl Deref for TSI0 {
    type Target = tsi0::RegisterBlock;
    fn deref(&self) -> &tsi0::RegisterBlock {
        unsafe { &*TSI0::ptr() }
    }
}
#[doc = "Touch sense input"]
pub mod tsi0;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sim::RegisterBlock {
        1074032640 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    fn deref(&self) -> &sim::RegisterBlock {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porta::RegisterBlock {
        1074040832 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    fn deref(&self) -> &porta::RegisterBlock {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portb::RegisterBlock {
        1074044928 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &portb::RegisterBlock {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Pin Control and Interrupts"]
pub struct PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portc::RegisterBlock {
        1074049024 as *const _
    }
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    fn deref(&self) -> &portc::RegisterBlock {
        unsafe { &*PORTC::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portc;
#[doc = "Pin Control and Interrupts"]
pub struct PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTD {}
impl PORTD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portd::RegisterBlock {
        1074053120 as *const _
    }
}
impl Deref for PORTD {
    type Target = portd::RegisterBlock;
    fn deref(&self) -> &portd::RegisterBlock {
        unsafe { &*PORTD::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portd;
#[doc = "Pin Control and Interrupts"]
pub struct PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTE {}
impl PORTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porte::RegisterBlock {
        1074057216 as *const _
    }
}
impl Deref for PORTE {
    type Target = porte::RegisterBlock;
    fn deref(&self) -> &porte::RegisterBlock {
        unsafe { &*PORTE::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porte;
#[doc = "EMVSIM"]
pub struct EMVSIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMVSIM0 {}
impl EMVSIM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emvsim0::RegisterBlock {
        1074061312 as *const _
    }
}
impl Deref for EMVSIM0 {
    type Target = emvsim0::RegisterBlock;
    fn deref(&self) -> &emvsim0::RegisterBlock {
        unsafe { &*EMVSIM0::ptr() }
    }
}
#[doc = "EMVSIM"]
pub mod emvsim0;
#[doc = "EMVSIM"]
pub struct EMVSIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMVSIM1 {}
impl EMVSIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emvsim1::RegisterBlock {
        1074065408 as *const _
    }
}
impl Deref for EMVSIM1 {
    type Target = emvsim1::RegisterBlock;
    fn deref(&self) -> &emvsim1::RegisterBlock {
        unsafe { &*EMVSIM1::ptr() }
    }
}
#[doc = "EMVSIM"]
pub mod emvsim1;
#[doc = "LTC"]
pub struct LTC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LTC0 {}
impl LTC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ltc0::RegisterBlock {
        1074073600 as *const _
    }
}
impl Deref for LTC0 {
    type Target = ltc0::RegisterBlock;
    fn deref(&self) -> &ltc0::RegisterBlock {
        unsafe { &*LTC0::ptr() }
    }
}
#[doc = "LTC"]
pub mod ltc0;
#[doc = "Generation 2008 Watchdog Timer"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog::RegisterBlock {
        1074077696 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    fn deref(&self) -> &wdog::RegisterBlock {
        unsafe { &*WDOG::ptr() }
    }
}
#[doc = "Generation 2008 Watchdog Timer"]
pub mod wdog;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART0 {}
impl LPUART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart0::RegisterBlock {
        1074085888 as *const _
    }
}
impl Deref for LPUART0 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &lpuart0::RegisterBlock {
        unsafe { &*LPUART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod lpuart0;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart1::RegisterBlock {
        1074089984 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &lpuart1::RegisterBlock {
        unsafe { &*LPUART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod lpuart1;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART2 {}
impl LPUART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart2::RegisterBlock {
        1074094080 as *const _
    }
}
impl Deref for LPUART2 {
    type Target = lpuart2::RegisterBlock;
    fn deref(&self) -> &lpuart2::RegisterBlock {
        unsafe { &*LPUART2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod lpuart2;
#[doc = "QuadSPI"]
pub struct QUADSPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QUADSPI0 {}
impl QUADSPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const quad_spi0::RegisterBlock {
        1074110464 as *const _
    }
}
impl Deref for QUADSPI0 {
    type Target = quad_spi0::RegisterBlock;
    fn deref(&self) -> &quad_spi0::RegisterBlock {
        unsafe { &*QUADSPI0::ptr() }
    }
}
#[doc = "QuadSPI"]
pub mod quad_spi0;
#[doc = "The FLEXIO Memory Map/Register Definition can be found here."]
pub struct FLEXIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXIO0 {}
impl FLEXIO0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flexio0::RegisterBlock {
        1074130944 as *const _
    }
}
impl Deref for FLEXIO0 {
    type Target = flexio0::RegisterBlock;
    fn deref(&self) -> &flexio0::RegisterBlock {
        unsafe { &*FLEXIO0::ptr() }
    }
}
#[doc = "The FLEXIO Memory Map/Register Definition can be found here."]
pub mod flexio0;
#[doc = "External Watchdog Monitor"]
pub struct EWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EWM {}
impl EWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ewm::RegisterBlock {
        1074139136 as *const _
    }
}
impl Deref for EWM {
    type Target = ewm::RegisterBlock;
    fn deref(&self) -> &ewm::RegisterBlock {
        unsafe { &*EWM::ptr() }
    }
}
#[doc = "External Watchdog Monitor"]
pub mod ewm;
#[doc = "Multipurpose Clock Generator module"]
pub struct MCG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCG {}
impl MCG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcg::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for MCG {
    type Target = mcg::RegisterBlock;
    fn deref(&self) -> &mcg::RegisterBlock {
        unsafe { &*MCG::ptr() }
    }
}
#[doc = "Multipurpose Clock Generator module"]
pub mod mcg;
#[doc = "Oscillator"]
pub struct OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC {}
impl OSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const osc::RegisterBlock {
        1074155520 as *const _
    }
}
impl Deref for OSC {
    type Target = osc::RegisterBlock;
    fn deref(&self) -> &osc::RegisterBlock {
        unsafe { &*OSC::ptr() }
    }
}
#[doc = "Oscillator"]
pub mod osc;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074159616 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1074163712 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c1;
#[doc = "Universal Serial Bus, OTG Capable Controller"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0::RegisterBlock {
        1074208768 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &usb0::RegisterBlock {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Universal Serial Bus, OTG Capable Controller"]
pub mod usb0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp0::RegisterBlock {
        1074212864 as *const _
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    fn deref(&self) -> &cmp0::RegisterBlock {
        unsafe { &*CMP0::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "Voltage Reference"]
pub struct VREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREF {}
impl VREF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vref::RegisterBlock {
        1074216960 as *const _
    }
}
impl Deref for VREF {
    type Target = vref::RegisterBlock;
    fn deref(&self) -> &vref::RegisterBlock {
        unsafe { &*VREF::ptr() }
    }
}
#[doc = "Voltage Reference"]
pub mod vref;
#[doc = "Low leakage wakeup unit"]
pub struct LLWU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LLWU {}
impl LLWU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const llwu::RegisterBlock {
        1074249728 as *const _
    }
}
impl Deref for LLWU {
    type Target = llwu::RegisterBlock;
    fn deref(&self) -> &llwu::RegisterBlock {
        unsafe { &*LLWU::ptr() }
    }
}
#[doc = "Low leakage wakeup unit"]
pub mod llwu;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmc::RegisterBlock {
        1074253824 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "System Mode Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smc::RegisterBlock {
        1074257920 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &smc::RegisterBlock {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "Reset Control Module"]
pub struct RCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCM {}
impl RCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcm::RegisterBlock {
        1074262016 as *const _
    }
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    fn deref(&self) -> &rcm::RegisterBlock {
        unsafe { &*RCM::ptr() }
    }
}
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "General Purpose Input/Output"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1074786304 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioa;
#[doc = "General Purpose Input/Output"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1074786368 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiob;
#[doc = "General Purpose Input/Output"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1074786432 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioc;
#[doc = "General Purpose Input/Output"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiod::RegisterBlock {
        1074786496 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    fn deref(&self) -> &gpiod::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiod;
#[doc = "General Purpose Input/Output"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioe::RegisterBlock {
        1074786560 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    fn deref(&self) -> &gpioe::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioe;
#[doc = "Micro Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtb::RegisterBlock {
        4026531840 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    fn deref(&self) -> &mtb::RegisterBlock {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "Micro Trace Buffer"]
pub mod mtb;
#[doc = "MTB data watchpoint and trace"]
pub struct MTBDWT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTBDWT {}
impl MTBDWT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtbdwt::RegisterBlock {
        4026535936 as *const _
    }
}
impl Deref for MTBDWT {
    type Target = mtbdwt::RegisterBlock;
    fn deref(&self) -> &mtbdwt::RegisterBlock {
        unsafe { &*MTBDWT::ptr() }
    }
}
#[doc = "MTB data watchpoint and trace"]
pub mod mtbdwt;
#[doc = "System ROM"]
pub struct ROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROM {}
impl ROM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rom::RegisterBlock {
        4026540032 as *const _
    }
}
impl Deref for ROM {
    type Target = rom::RegisterBlock;
    fn deref(&self) -> &rom::RegisterBlock {
        unsafe { &*ROM::ptr() }
    }
}
#[doc = "System ROM"]
pub mod rom;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcm::RegisterBlock {
        4026544128 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    fn deref(&self) -> &mcm::RegisterBlock {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOA {}
impl FGPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpioa::RegisterBlock {
        4160749568 as *const _
    }
}
impl Deref for FGPIOA {
    type Target = fgpioa::RegisterBlock;
    fn deref(&self) -> &fgpioa::RegisterBlock {
        unsafe { &*FGPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioa;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOB {}
impl FGPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpiob::RegisterBlock {
        4160749632 as *const _
    }
}
impl Deref for FGPIOB {
    type Target = fgpiob::RegisterBlock;
    fn deref(&self) -> &fgpiob::RegisterBlock {
        unsafe { &*FGPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiob;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOC {}
impl FGPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpioc::RegisterBlock {
        4160749696 as *const _
    }
}
impl Deref for FGPIOC {
    type Target = fgpioc::RegisterBlock;
    fn deref(&self) -> &fgpioc::RegisterBlock {
        unsafe { &*FGPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioc;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOD {}
impl FGPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpiod::RegisterBlock {
        4160749760 as *const _
    }
}
impl Deref for FGPIOD {
    type Target = fgpiod::RegisterBlock;
    fn deref(&self) -> &fgpiod::RegisterBlock {
        unsafe { &*FGPIOD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiod;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOE {}
impl FGPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fgpioe::RegisterBlock {
        4160749824 as *const _
    }
}
impl Deref for FGPIOE {
    type Target = fgpioe::RegisterBlock;
    fn deref(&self) -> &fgpioe::RegisterBlock {
        unsafe { &*FGPIOE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioe;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FTFA_FLASHCONFIG"]
    pub FTFA_FLASHCONFIG: FTFA_FLASHCONFIG,
    #[doc = "AIPS"]
    pub AIPS: AIPS,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "FTFA"]
    pub FTFA: FTFA,
    #[doc = "DMAMUX"]
    pub DMAMUX: DMAMUX,
    #[doc = "INTMUX0"]
    pub INTMUX0: INTMUX0,
    #[doc = "TRNG0"]
    pub TRNG0: TRNG0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "PIT0"]
    pub PIT0: PIT0,
    #[doc = "TPM0"]
    pub TPM0: TPM0,
    #[doc = "TPM1"]
    pub TPM1: TPM1,
    #[doc = "TPM2"]
    pub TPM2: TPM2,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "RFVBAT"]
    pub RFVBAT: RFVBAT,
    #[doc = "DAC0"]
    pub DAC0: DAC0,
    #[doc = "LPTMR0"]
    pub LPTMR0: LPTMR0,
    #[doc = "LPTMR1"]
    pub LPTMR1: LPTMR1,
    #[doc = "RFSYS"]
    pub RFSYS: RFSYS,
    #[doc = "TSI0"]
    pub TSI0: TSI0,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "PORTC"]
    pub PORTC: PORTC,
    #[doc = "PORTD"]
    pub PORTD: PORTD,
    #[doc = "PORTE"]
    pub PORTE: PORTE,
    #[doc = "EMVSIM0"]
    pub EMVSIM0: EMVSIM0,
    #[doc = "EMVSIM1"]
    pub EMVSIM1: EMVSIM1,
    #[doc = "LTC0"]
    pub LTC0: LTC0,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "LPUART0"]
    pub LPUART0: LPUART0,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "LPUART2"]
    pub LPUART2: LPUART2,
    #[doc = "QUADSPI0"]
    pub QUADSPI0: QUADSPI0,
    #[doc = "FLEXIO0"]
    pub FLEXIO0: FLEXIO0,
    #[doc = "EWM"]
    pub EWM: EWM,
    #[doc = "MCG"]
    pub MCG: MCG,
    #[doc = "OSC"]
    pub OSC: OSC,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "VREF"]
    pub VREF: VREF,
    #[doc = "LLWU"]
    pub LLWU: LLWU,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "RCM"]
    pub RCM: RCM,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "MTBDWT"]
    pub MTBDWT: MTBDWT,
    #[doc = "ROM"]
    pub ROM: ROM,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "FGPIOA"]
    pub FGPIOA: FGPIOA,
    #[doc = "FGPIOB"]
    pub FGPIOB: FGPIOB,
    #[doc = "FGPIOC"]
    pub FGPIOC: FGPIOC,
    #[doc = "FGPIOD"]
    pub FGPIOD: FGPIOD,
    #[doc = "FGPIOE"]
    pub FGPIOE: FGPIOE,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FTFA_FLASHCONFIG: FTFA_FLASHCONFIG {
                _marker: PhantomData,
            },
            AIPS: AIPS {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            FTFA: FTFA {
                _marker: PhantomData,
            },
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            INTMUX0: INTMUX0 {
                _marker: PhantomData,
            },
            TRNG0: TRNG0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            PIT0: PIT0 {
                _marker: PhantomData,
            },
            TPM0: TPM0 {
                _marker: PhantomData,
            },
            TPM1: TPM1 {
                _marker: PhantomData,
            },
            TPM2: TPM2 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            RFVBAT: RFVBAT {
                _marker: PhantomData,
            },
            DAC0: DAC0 {
                _marker: PhantomData,
            },
            LPTMR0: LPTMR0 {
                _marker: PhantomData,
            },
            LPTMR1: LPTMR1 {
                _marker: PhantomData,
            },
            RFSYS: RFSYS {
                _marker: PhantomData,
            },
            TSI0: TSI0 {
                _marker: PhantomData,
            },
            SIM: SIM {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTB: PORTB {
                _marker: PhantomData,
            },
            PORTC: PORTC {
                _marker: PhantomData,
            },
            PORTD: PORTD {
                _marker: PhantomData,
            },
            PORTE: PORTE {
                _marker: PhantomData,
            },
            EMVSIM0: EMVSIM0 {
                _marker: PhantomData,
            },
            EMVSIM1: EMVSIM1 {
                _marker: PhantomData,
            },
            LTC0: LTC0 {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
            LPUART0: LPUART0 {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            LPUART2: LPUART2 {
                _marker: PhantomData,
            },
            QUADSPI0: QUADSPI0 {
                _marker: PhantomData,
            },
            FLEXIO0: FLEXIO0 {
                _marker: PhantomData,
            },
            EWM: EWM {
                _marker: PhantomData,
            },
            MCG: MCG {
                _marker: PhantomData,
            },
            OSC: OSC {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            USB0: USB0 {
                _marker: PhantomData,
            },
            CMP0: CMP0 {
                _marker: PhantomData,
            },
            VREF: VREF {
                _marker: PhantomData,
            },
            LLWU: LLWU {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            RCM: RCM {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
            MTBDWT: MTBDWT {
                _marker: PhantomData,
            },
            ROM: ROM {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
            FGPIOA: FGPIOA {
                _marker: PhantomData,
            },
            FGPIOB: FGPIOB {
                _marker: PhantomData,
            },
            FGPIOC: FGPIOC {
                _marker: PhantomData,
            },
            FGPIOD: FGPIOD {
                _marker: PhantomData,
            },
            FGPIOE: FGPIOE {
                _marker: PhantomData,
            },
        }
    }
}
