#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate embedded_hal as hal;

mod eev;

#[repr(i8)]
pub enum MovementDirection{
    Closing = -1,
    Holding = 0,
    Opening = 1
}

pub trait StepperMotor : Sized + 'static {
    fn next(&self) -> Self;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
