#[repr(i8)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MovementDirection{
    Close = -1,
    Hold = 0,
    Open = 1
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ActuationError {
    Generic,
    ExceededRange,
    TimedOut
}

pub trait Stepper {
    fn actuate(&self, direction: &MovementDirection) -> Result<(), ActuationError>;
    fn release(&self) -> Result<(), ActuationError>;
}