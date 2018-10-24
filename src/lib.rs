//! A minimal device crate for the LM3S6965
//!
//! This device crate only contains interrupt bindings and it's main use case is running [Real Time
//! For the Masses][rtfm] (RTFM) programs on QEMU.
//!
//! [rtfm]: https://github.com/japaric/cortex-m-rtfm
//!
//! **NOTE**: This crate targets Rust 1.31 so you'll need nightly or beta until that
//! version is released.
//!
//! # References
//!
//! - [LM3S6965 data sheet](https://www.ti.com/lit/ds/symlink/lm3s6965.pdf)

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub use self::Interrupt as interrupt;
use bare_metal::Nr;
pub use cortex_m_rt::interrupt;

/// Number of bits available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 3;

/// Enumeration of all interrupts
// NOTE: See Table 2-9 Interrupts. Section 2.5.2 "Exception types"
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// GPIO Port A
    GPIOA,
    /// GPIO Port B
    GPIOB,
    /// GPIO Port C
    GPIOC,
    /// GPIO Port D
    GPIOD,
    /// GPIO Port E
    GPIOE,
    /// UART0
    UART0,
    /// UART1
    UART1,
    /// SSI0
    SSI0,
    /// I2C0
    I2C0,
    /// PWM fault
    PWM_FAULT,
    /// PWM Generator 0
    PWM_GENERATOR_0,
    /// PWM Generator 1
    PWM_GENERATOR_1,
    /// PWM Generator 2
    PWM_GENERATOR_2,
    /// QEI0
    QEI0,
    /// ADC0 Sequence 0
    ADC0_SEQUENCE_0,
    /// ADC0 Sequence 1
    ADC0_SEQUENCE_1,
    /// ADC0 Sequence 2
    ADC0_SEQUENCE_2,
    /// ADC0 Sequence 3
    ADC0_SEQUENCE_3,
    /// Watchdog Timer 0
    WATCHDOG_TIMER_0,
    /// Timer 0A
    TIMER_0A,
    /// Timer 0B
    TIMER_0B,
    /// Timer 1A
    TIMER_1A,
    /// Timer 1B
    TIMER_1B,
    /// Timer 2A
    TIMER_2A,
    /// Timer 2B
    TIMER_2B,
    /// Analog Comparator 0
    ANALOG_COMPARATOR_0,
    /// Analog Comparator 1
    ANALOG_COMPARATOR_1,
    /// System Control
    SYSTEM_CONTROL,
    /// Flash Memory Control
    FLASH_MEMORY_CONTROL,
    /// GPIO Port F
    GPIOF,
    /// GPIO Port G
    GPIOG,
    /// UART2
    UART2,
    /// Timer 3A
    TIMER_3A,
    /// Timer 3B
    TIMER_3B,
    /// I2C1
    I2C1,
    /// QEI1
    QEI1,
    /// Ethernet Controller
    ETHERNET,
    /// Hibernation Module
    HIBERNATION,
}

unsafe impl Nr for Interrupt {
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::GPIOA => 0,
            Interrupt::GPIOB => 1,
            Interrupt::GPIOC => 2,
            Interrupt::GPIOD => 3,
            Interrupt::GPIOE => 4,
            Interrupt::UART0 => 5,
            Interrupt::UART1 => 6,
            Interrupt::SSI0 => 7,
            Interrupt::I2C0 => 8,
            Interrupt::PWM_FAULT => 9,
            Interrupt::PWM_GENERATOR_0 => 10,
            Interrupt::PWM_GENERATOR_1 => 11,
            Interrupt::PWM_GENERATOR_2 => 12,
            Interrupt::QEI0 => 13,
            Interrupt::ADC0_SEQUENCE_0 => 14,
            Interrupt::ADC0_SEQUENCE_1 => 15,
            Interrupt::ADC0_SEQUENCE_2 => 16,
            Interrupt::ADC0_SEQUENCE_3 => 17,
            Interrupt::WATCHDOG_TIMER_0 => 18,
            Interrupt::TIMER_0A => 19,
            Interrupt::TIMER_0B => 20,
            Interrupt::TIMER_1A => 21,
            Interrupt::TIMER_1B => 22,
            Interrupt::TIMER_2A => 23,
            Interrupt::TIMER_2B => 24,
            Interrupt::ANALOG_COMPARATOR_0 => 25,
            Interrupt::ANALOG_COMPARATOR_1 => 26,
            // Interrupt::RESERVED => 27,
            Interrupt::SYSTEM_CONTROL => 28,
            Interrupt::FLASH_MEMORY_CONTROL => 29,
            Interrupt::GPIOF => 30,
            Interrupt::GPIOG => 31,
            // Interrupt::RESERVED => 32,
            Interrupt::UART2 => 33,
            // Interrupt::RESERVED => 34,
            Interrupt::TIMER_3A => 35,
            Interrupt::TIMER_3B => 36,
            Interrupt::I2C1 => 37,
            Interrupt::QEI1 => 38,
            // Interrupt::RESERVED => 39...41,
            Interrupt::ETHERNET => 42,
            Interrupt::HIBERNATION => 43,
        }
    }
}

extern "C" {
    fn GPIOA();
    fn GPIOB();
    fn GPIOC();
    fn GPIOD();
    fn GPIOE();
    fn UART0();
    fn UART1();
    fn SSI0();
    fn I2C0();
    fn PWM_FAULT();
    fn PWM_GENERATOR_0();
    fn PWM_GENERATOR_1();
    fn PWM_GENERATOR_2();
    fn QEI0();
    fn ADC0_SEQUENCE_0();
    fn ADC0_SEQUENCE_1();
    fn ADC0_SEQUENCE_2();
    fn ADC0_SEQUENCE_3();
    fn WATCHDOG_TIMER_0();
    fn TIMER_0A();
    fn TIMER_0B();
    fn TIMER_1A();
    fn TIMER_1B();
    fn TIMER_2A();
    fn TIMER_2B();
    fn ANALOG_COMPARATOR_0();
    fn ANALOG_COMPARATOR_1();
    fn SYSTEM_CONTROL();
    fn FLASH_MEMORY_CONTROL();
    fn GPIOF();
    fn GPIOG();
    fn UART2();
    fn TIMER_3A();
    fn TIMER_3B();
    fn I2C1();
    fn QEI1();
    fn ETHERNET();
    fn HIBERNATION();
}

union Vector {
    handler: unsafe extern "C" fn(),
    reserved: u32,
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
static __INTERRUPTS: [Vector; 44] = [
    Vector { handler: GPIOA },
    Vector { handler: GPIOB },
    Vector { handler: GPIOC },
    Vector { handler: GPIOD },
    Vector { handler: GPIOE },
    Vector { handler: UART0 },
    Vector { handler: UART1 },
    Vector { handler: SSI0 },
    Vector { handler: I2C0 },
    Vector { handler: PWM_FAULT },
    Vector {
        handler: PWM_GENERATOR_0,
    },
    Vector {
        handler: PWM_GENERATOR_1,
    },
    Vector {
        handler: PWM_GENERATOR_2,
    },
    Vector { handler: QEI0 },
    Vector {
        handler: ADC0_SEQUENCE_0,
    },
    Vector {
        handler: ADC0_SEQUENCE_1,
    },
    Vector {
        handler: ADC0_SEQUENCE_2,
    },
    Vector {
        handler: ADC0_SEQUENCE_3,
    },
    Vector {
        handler: WATCHDOG_TIMER_0,
    },
    Vector { handler: TIMER_0A },
    Vector { handler: TIMER_0B },
    Vector { handler: TIMER_1A },
    Vector { handler: TIMER_1B },
    Vector { handler: TIMER_2A },
    Vector { handler: TIMER_2B },
    Vector {
        handler: ANALOG_COMPARATOR_0,
    },
    Vector {
        handler: ANALOG_COMPARATOR_1,
    },
    Vector { reserved: 0 },
    Vector {
        handler: SYSTEM_CONTROL,
    },
    Vector {
        handler: FLASH_MEMORY_CONTROL,
    },
    Vector { handler: GPIOF },
    Vector { handler: GPIOG },
    Vector { reserved: 0 },
    Vector { handler: UART2 },
    Vector { reserved: 0 },
    Vector { handler: TIMER_3A },
    Vector { handler: TIMER_3B },
    Vector { handler: I2C1 },
    Vector { handler: QEI1 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: ETHERNET },
    Vector {
        handler: HIBERNATION,
    },
];

/// All the peripherals
///
/// Yep, this is currently empty!
pub struct Peripherals {
    _0: (),
}

impl Peripherals {
    /// Kind of useless as there's no register API but this is required by RTFM
    pub unsafe fn steal() -> Self {
        Peripherals { _0: () }
    }
}
