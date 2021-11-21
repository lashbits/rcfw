#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

mod bluetooth;
mod softdevice;

use defmt_rtt as _;
use nrf52840_hal as hal;

use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    bluetooth::init();

    defmt::info!("Going into the infinite loop...");
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
