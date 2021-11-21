#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use(defer)]
extern crate scopeguard;

mod bluetooth;
mod softdevice;

#[cfg(feature = "52840")]
use nrf52840_hal as hal;

use defmt_rtt as _;

use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    bluetooth::init();
    bluetooth::connect([0x44, 0x16, 0x22, 0xc1, 0x19, 0x46]);

    defmt::info!("going into the infinite loop");
    loop {
        //cortex_m::asm::wfi();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    defmt::panic!("memory allocation error");
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
