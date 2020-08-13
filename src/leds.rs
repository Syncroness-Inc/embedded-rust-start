use core::ops::Deref;
trait GPIO {
    
}

cfg_if::cfg_if!{
    if #[cfg(feature = "stm32f4xx_hal")] {

        use stm32f4xx_hal;
        struct Led2<T> {
            inner: stm32f4xx_hal::gpio::gpiog::PG15<T>
        }
        
        impl<T> Deref for Led2<T> {
            type Target = T;
        
            fn deref(&self) -> Self::Target {
                &self.inner
            }
        }

        impl<U, T> Led2<T> {
            fn toggle(&mut self) -> Result<(), U> {
                self.
            }
        }
    } else if #[cfg(test)] {

    }
}