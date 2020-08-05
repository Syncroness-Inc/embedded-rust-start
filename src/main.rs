#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
// Program will have an odd linking error if you don't bring 
// stm32f4xx_hal into scope using use keyword
use hal::{
    prelude::*,
    stm32,
};

// NOTE(allow) bug rust-lang/rust#53964
#[allow(unused_imports)] 
use panic_halt;

#[entry]
fn main() -> ! {
    let board_peripherals = stm32::Peripherals::take().unwrap();
    let processor_peripherals = cortex_m::Peripherals::take().unwrap();

    // Setting system clock speed
    let clock_controler = board_peripherals.RCC.constrain();
    let system_clock = clock_controler.cfgr.sysclk(48.mhz()).freeze();

    let mut delay = hal::delay::Delay::new(processor_peripherals.SYST, system_clock);
    
    let mut led2 = board_peripherals.GPIOG.split().pg13.into_push_pull_output();
    
    loop {
        // On for 1s, off for 1s
        led2.set_high().unwrap();
        delay.delay_ms(1000_u32);
        led2.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }
}
