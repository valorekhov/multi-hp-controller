#[repr(i8)]
pub enum MovementDirection{
    Closing = -1,
    Holding = 0,
    Opening = 1
}

pub trait StepperMotor : 'static {
    fn next(&self);
}
