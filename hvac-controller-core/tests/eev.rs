#![feature(generic_associated_types)]
extern crate linux_embedded_hal as platform_hal;

use platform_hal::SysTimer;
use embedded_hal::timer::CountDown;

use hvac_controller_core::eev::Eev;
use hvac_controller_core::block_async;
use hvac_controller_core::hal::{Stepper, MovementDirection, ActuationError};

use core::time::Duration;
use std::future::Future;
use embedded_hal_async::delay::DelayUs;

use mockall::*;
use mockall::predicate::*;
use tokio::task::futures::TaskLocalFuture;
use tokio::time::{sleep, Sleep};

use futures_core::{future};

struct DelayUsPosix {}

impl DelayUsPosix {
    async fn async_delay(duration: Duration) -> Result<(), ()> {
        sleep(duration).await;
       return Ok(());
    }
}

impl DelayUs for DelayUsPosix{
    type Error = ();
    type DelayUsFuture<'a> where Self: 'a = Sleep;

    fn delay_us(&mut self, us: u32)  {
        let delay = DelayUsPosix::async_delay(Duration::from_micros(us as u64));
    }

    type DelayMsFuture<'a> where Self: 'a = Sleep;

    fn delay_ms(&mut self, ms: u32) -> Sleep {
        sleep(Duration::from_millis(ms as u64))
    }
}

#[tokio::test]
async fn eev_initialization() {

    let mut timer : SysTimer = SysTimer::new();

    mock!{
            Stepper{}
            impl Stepper for Stepper {
                fn actuate(&self, dir: &MovementDirection) -> Result<(), ActuationError>;
                fn release(&self);
            }
        }

    let stepper_motor = MockStepper::new();
    let stepper_driver = Box::new(stepper_motor);

    let range : u16 = 100;
    let overdrive : u16 = 10;
    let max_steps : u16 = range + overdrive;
    let mut eev = Eev::<DelayUsPosix>::new(stepper_driver, range, overdrive, [Duration::from_millis(3), Duration::from_millis(2), Duration::from_millis(1)]);

    assert!(eev.current_position() == None);
    eev.initialize().await;
    assert_eq!(eev.current_position(), Some(max_steps));
    let mut step = 0;
    while step <= max_steps {
        eev.run().await;
        //time.sleep(0.005);
        timer.start(Duration::from_millis(100));
        match block_async!(timer.wait()) {
            Ok(_r) => {},
            Err(_) => panic!()
        };
        step += 1;
    }
    assert_eq!(eev.current_position(), Some(0));
    assert!(eev.is_closed());
}