#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

//! A basic blinky application. The purpose of this application is to serve as an easy starting point
//! for embedded rust projects. Be sure to read the `README.md`. It gives usefull information for how
//! to quickly get your environment up and running within vscode and illustrates some useful tools
//! such as a debugger and a way to flash your microcontroller. All of these tools ship with docker
//! container and have been .

cfg_if::cfg_if! {
    if #[cfg(not(test))] {
        use cortex_m_rt::entry;
        use stm32f4xx_hal as hal;
        use hal::{
            prelude::*,
            stm32,
        };

        #[allow(unused_imports)]
        use panic_halt;
    }
}

// Program will have an odd linking error if you don't bring
// stm32f4xx_hal into scope using use keyword

// NOTE(allow) bug rust-lang/rust#53964
#[cfg_attr(not(test), entry)]
#[cfg(not(test))]
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

fn add_two(x: u32) -> u32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_good_value() {
        assert_eq!(2, add_two(0));
    }
}
