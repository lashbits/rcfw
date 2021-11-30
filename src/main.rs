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

#[rtic::app(device = crate::hal::pac)]
const APP: () = {
    struct Resources {
        #[init(false)]
        bluetooth_initialized: bool,
    }

    #[init]
    fn init(cx: init::Context) {
        defmt::info!("hello world!");
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        // sd_softdevice_enable must not be called in RTIC init because RTIC runs init with interrupts disabled.
        // The softdevice crashes if it is enabled while interrupts are disabled.
        // On the other hand, RTIC idle runs with interrupts enabled.
        bluetooth::init();
        //bluetooth::connect([0x44, 0x16, 0x22, 0xc1, 0x19, 0x46]);
        defmt::info!("going into the infinite loop");
        loop {
            //cortex_m::asm::wfi();
        }
    }
};

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    defmt::panic!("memory allocation error");
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::bkpt();
    }
}
