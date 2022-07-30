#![feature(generic_associated_types)]
extern crate linux_embedded_hal as platform_hal;

use hvac_controller_core::eev::Eev;
use hvac_controller_core::hal::{Stepper, MovementDirection, ActuationError};

use core::time::Duration;
use core::future::{Future};
use core::pin::Pin;
use core::task::{Context, Poll};
use embedded_hal_async::delay::DelayUs;

use mockall::*;
use mockall::predicate::*;
use tokio::time::{sleep, Sleep};

struct SleepWrapper {
     pub sleep: Pin<Box<Sleep>>,
}

impl Future for SleepWrapper {
     type Output = Result<(), ()>;
    
     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match self.sleep.as_mut().poll(cx) {
                Poll::Ready(()) => Poll::Ready(Ok(())),
                Poll::Pending => Poll::Pending,
            }
    }
}

struct DelayUsPosix {}

impl DelayUs for DelayUsPosix{
    type Error = ();
    type DelayUsFuture<'a> = SleepWrapper where Self: 'a;

    fn delay_us(&mut self, us: u32) -> SleepWrapper {
        SleepWrapper{sleep: Box::pin(sleep(Duration::from_micros(us as u64)))}
    }

    type DelayMsFuture<'a> = SleepWrapper where Self: 'a;

    fn delay_ms(&mut self, ms: u32) -> SleepWrapper {
        SleepWrapper{sleep: Box::pin(sleep(Duration::from_millis(ms as u64)))}
    }
}

#[tokio::test]
async fn eev_initialization() {

    let range : u16 = 100;
    let overdrive : u16 = 10;
    let max_steps : u16 = range + overdrive;

    mock!{
            Stepper{}
            impl Stepper for Stepper {
                fn actuate(&self, dir: &MovementDirection) -> Result<(), ActuationError>;
                fn release(&self);
            }
        }

    let mut stepper_motor = MockStepper::new();
    stepper_motor.expect_actuate()
        .times(max_steps as usize)
        .returning(|_| Ok(()));

    let delay_generator = DelayUsPosix{};

    let mut eev = Eev::new( stepper_motor, range, overdrive, 
            [Duration::from_millis(3), Duration::from_millis(2), Duration::from_millis(1)],
            delay_generator);

    assert!(eev.current_position().is_none());
    assert!(eev.current_direction().is_none());
    match eev.initialize().await {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };

    assert_eq!(eev.current_direction(), Some(MovementDirection::Hold));
    assert!(eev.current_position().is_some());
    assert_eq!(eev.current_position().unwrap().value(), 0);

    stepper_motor.checkpoint();

    // while step <= max_steps {
    //     eev.actuate(Percentage::).await;
    //     //time.sleep(0.005);
    //     timer.start(Duration::from_millis(100));
    //     match block_async!(timer.wait()) {
    //         Ok(_r) => {},
    //         Err(_) => panic!()
    //     };
    //     step += 1;
    // }
    assert!(eev.current_position().is_some());
    assert_eq!(eev.current_position().unwrap().value(), 100);
    assert!(eev.is_fully_closed());
}