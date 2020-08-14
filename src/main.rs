//! A basic blinky application. The purpose of this application is to serve as an easy starting point
//! for embedded rust projects. Be sure to read the `README.md`. It gives useful information for how
//! to quickly get your environment up and running within vscode and illustrates some useful tools
//! such as a debugger and a way to flash your microcontroller. All of these tools ship with docker
//! container and have been .
#![cfg_attr(feature = "embedded_platform", no_std)]
#![cfg_attr(feature = "embedded_platform", no_main)]

#[cfg(feature = "embedded_platform")] 
use cortex_m_rt::entry;

#[cfg(feature = "stm32f407")]
use stm32f4xx_hal as hal;

#[cfg(feature = "stm32f407")]
use hal::{
    prelude::*,
    stm32 as board,
};

#[cfg_attr(feature = "embedded_platform", allow(unused_imports))]
#[cfg(feature = "embedded_platform")]
use panic_halt;

// NOTE(allow) bug rust-lang/rust#53964
#[cfg_attr(feature = "embedded_platform", entry)]
#[cfg(feature = "embedded_platform")]
fn main() -> ! {
    let board_peripherals = board::Peripherals::take().unwrap();
    let processor_peripherals = cortex_m::Peripherals::take().unwrap();
    
    // Setting system clock speed
    let clock_controller = board_peripherals.RCC.constrain();
    let system_clock = clock_controller.cfgr.sysclk(48.mhz()).freeze();

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

/// A trivial function that demonstrates unit testing on our host platform.
fn add_two(x: u32) -> u32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A trivial test to demonstrate we can run 
    /// unit tests on our host platforms
    #[test]
    fn add_two_good_value() {
        assert_eq!(2, add_two(0));
    }
}
