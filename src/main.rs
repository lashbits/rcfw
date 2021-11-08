#![no_std]
#![no_main]

use core::fmt::Write;
use hal::{gpio, uarte, uarte::Uarte};
use nrf52840_hal as hal;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    let cdc_pins = uarte::Pins {
        txd: port0
            .p0_15
            .into_push_pull_output(gpio::Level::High)
            .degrade(),
        rxd: port0.p0_20.into_floating_input().degrade(),
        cts: None,
        rts: None,
    };

    let mut uarte = Uarte::new(
        p.UARTE0,
        cdc_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    write!(uarte, "Hello, World!\r\n").unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
