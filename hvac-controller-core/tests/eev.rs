extern crate linux_embedded_hal as platform_hal;

use platform_hal::SysTimer;
use embedded_hal::timer::CountDown;

use hvac_controller_core::eev::Eev;
use hvac_controller_core::block_async;
use hvac_controller_core::hal::{Stepper, MovementDirection};

use core::time::Duration;

use mockall::*;
use mockall::predicate::*;

#[tokio::test]
async fn eev_initialization() {

    let mut timer : SysTimer = SysTimer::new();

    mock!{
            Stepper{}
            impl Stepper for Stepper {
                fn one_step(&self, dir: MovementDirection);
            }
        }

    let stepper_motor = MockStepper::new();
    let stepper_driver = Box::new(stepper_motor);

    let range : u16 = 100;
    let overdrive : u16 = 10;
    let max_steps : u16 = range + overdrive;
    let mut eev = Eev::new(stepper_driver, range, overdrive, [Duration::from_millis(3), Duration::from_millis(2), Duration::from_millis(1)]);

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