#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]

#[macro_use(defer)]
extern crate scopeguard;

mod bluetooth;
mod softdevice;

#[cfg(feature = "52840")]
use nrf52840_hal as hal;

use alloc_cortex_m::CortexMHeap;
use cortex_m_rt::entry;
use defmt::unwrap;
use defmt_rtt as _;
use embassy::executor::Executor;
use embassy::util::Forever;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
static EXECUTOR: Forever<Executor> = Forever::new();

async fn fake_sleep() {
    let mut a = 0;
    for i in 1..0xfffff {
        a += 1;
    }
}

#[embassy::task]
async fn softdevice_task() {
    bluetooth::init();
    //bluetooth::connect([0x44, 0x16, 0x22, 0xc1, 0x19, 0x46]);
    defmt::info!("going into the infinite loop");

    loop {
        fake_sleep().await;
        //sd.run().await;
        defmt::info!("in the loop buoy");
    }
}

#[entry]
fn main() -> ! {
    defmt::info!("hello world!");

    // bluetooth::enable simply starts the SoftDevice firmware.
    bluetooth::enable();

    let executor = EXECUTOR.put(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(softdevice_task()));
    });
}

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
