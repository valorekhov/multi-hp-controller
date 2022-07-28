// #![feature(generators, generator_trait)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "alloc")]
extern crate alloc;

// extern crate embedded_hal as hal;


mod eev;
mod util;
mod hal;

pub use crate::hal::*;

#[macro_export]
macro_rules! block_async {
    ($e:expr) => {
        loop {
            #[allow(unreachable_patterns)]
            match $e {
                Err(nb::Error::Other(e)) => {
                    #[allow(unreachable_code)]
                    break Err(e)
                }
                Err(nb::Error::WouldBlock) => {
                    crate::util::yield_now().await;
                }
                Ok(x) => break Ok(x),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
