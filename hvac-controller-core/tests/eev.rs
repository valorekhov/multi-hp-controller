#![feature(generic_associated_types)]

use hvac_controller_core::eev::Eev;
use hvac_controller_core::hal::{Stepper, MovementDirection, ActuationError};
use percentage::Percentage;

use core::time::Duration;
mod util;
use util::DelayUsPosix;

use mockall::*;
use mockall::predicate::*;

const RANGE : u16 = 200;
const OVERDRIVE : u16 = 15;
const MAX_STEPS : u16 = RANGE + OVERDRIVE;
const SPEEDS : [Duration; 3] = [Duration::from_millis(3), Duration::from_millis(2), Duration::from_millis(1)];

const DELAY_GENERATOR : DelayUsPosix = DelayUsPosix{};

mock!{
    Stepper{}
    impl Stepper for Stepper {
        fn actuate(&self, dir: &MovementDirection) -> Result<(), ActuationError>;
        fn release(&self) -> Result<(), ActuationError>;
    }
}   

#[tokio::test]
async fn eev_initialization() {
     
    let mut stepper_motor = MockStepper::new();
    stepper_motor.expect_actuate()
        .times(MAX_STEPS as usize)
        .returning(|_| Ok(()));
    stepper_motor.expect_release()
        .times(1)
        .returning(|| Ok(()));

    let mut eev = Eev::new( stepper_motor, 
        RANGE, OVERDRIVE, DELAY_GENERATOR);

    assert!(eev.current_position().is_none());
    assert!(eev.current_direction().is_none());
    match eev.initialize(SPEEDS[1]).await {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };

    assert_eq!(eev.current_direction(), Some(MovementDirection::Hold));
    assert_eq!(eev.current_position().map(|p| p.value()), Some(0));
}

#[tokio::test]
async fn eev_fully_open() {
    let mut stepper_motor = MockStepper::new();
    stepper_motor.expect_actuate()
        .times((MAX_STEPS + RANGE) as usize)
        .returning(|_| Ok(()));
    stepper_motor.expect_release()
        .times(2)
        .returning(|| Ok(()));

    let mut eev = Eev::new( stepper_motor, 
        RANGE, OVERDRIVE, DELAY_GENERATOR);

    match eev.initialize(SPEEDS[1]).await {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };
    assert_eq!(eev.current_direction(), Some(MovementDirection::Hold));
    assert_eq!(eev.current_position().map(|p| p.value()), Some(0));

    match eev.actuate(Percentage::from(100), SPEEDS[0]).await {
        Ok(pct) => assert_eq!(pct.value(), 100),
        Err(_) => assert!(false),
    };
    assert_eq!(eev.current_direction(), Some(MovementDirection::Hold));
    assert_eq!(eev.current_position().map(|p| p.value()), Some(100));
}

#[tokio::test]
async fn eev_actuate() {
    let mut stepper_motor = MockStepper::new();
    stepper_motor.expect_actuate()
        .times((MAX_STEPS + RANGE + core::ops::Div::div(RANGE, 2)) as usize)
        .returning(|_| Ok(()));
    stepper_motor.expect_release()
        .times(3)
        .returning(|| Ok(()));

    let mut eev = Eev::new( stepper_motor, 
        RANGE, OVERDRIVE, DELAY_GENERATOR);

    match eev.initialize(SPEEDS[1]).await {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };
    assert_eq!(eev.current_position().map(|p| p.value()), Some(0));

    match eev.actuate(Percentage::from(100), SPEEDS[0]).await {
        Ok(pct) => assert_eq!(pct.value(), 100),
        Err(_) => assert!(false),
    };
    assert_eq!(eev.current_position().map(|p| p.value()), Some(100));

    match eev.actuate(Percentage::from(50), SPEEDS[0]).await {
        Ok(pct) => assert_eq!(pct.value(), 50),
        Err(_) => assert!(false),
    };
    assert_eq!(eev.current_position().map(|p| p.value()), Some(50));
}

#[test]
#[should_panic(expected = "Percentage value must be between 0 and 100")]
fn percentage_goes_to_hundred_max() {
    Percentage::from(101);
}