#[repr(i8)]
pub enum MovementDirection{
    Closing = -1,
    Holding = 0,
    Opening = 1
}

pub trait Stepper {
    fn one_step(&self, direction: MovementDirection);
}
