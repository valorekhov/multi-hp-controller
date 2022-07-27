// use stepper::{StepperMotor, BACKWARD, FORWARD};


use core::ops::Add;

use crate::{MovementDirection, StepperMotor};

use core::time::Duration;

pub struct Eev {
    //_stepper_driver: Box<StepperMotor>,
    _max_pulses: u16,
    _overdrive: u16,
    _current_position: Option<u16>,
    _target_position: Option<u16>,
    _target_direction: MovementDirection,
    _target_speeds: [Duration; 3],
    _target_speed: Option<u8>
}

impl Eev {
    pub fn new(
        //stepper_driver: &dyn StepperMotor,
        max_pulses: u16,
        overdrive: u16,
        target_speeds: [Duration; 3],
    ) -> Self {
        Self {
            //_stepper_driver: stepper_driver,
            _max_pulses: max_pulses,
            _overdrive: overdrive,
            _current_position: None,
            _target_position: None,
            _target_direction: MovementDirection::Holding,
            _target_speeds: target_speeds,
            _target_speed: None
        }
    }

    // Initialize the EEV to its home position.
    // Sets the stepper to drive in the closing direction for the count of
    // max_pulses plus overdrive.     
    pub async fn initialize(&mut self) {
        self._target_position = Some(0);
        self._target_direction = MovementDirection::Closing;
        self._target_speed = Some(0);
        self._current_position = Some(self._max_pulses + self._overdrive);
        //self._set_next_tick_millis(0);
    }

    pub fn current_position(&self) -> Option<u16> {
        return self._current_position;
    }

    pub fn is_closed(&self) -> bool {
        return self._current_position == Some(0);
    }

    // Operate the EEV stepper motor.
    // Checks if the current millis is greater than the specified next millis.
    // If so, then the stepper is driven in the target direction.
    // If the target position is reached, then the stepper is stopped.
    pub async fn run(&mut self) {
        if self._target_position == self._current_position {
            return;
        }
        
        //self._stepper_driver.onestep(self._target_direction);
        
        let p: i32 = self._current_position.unwrap_or(0) as i32;
        let pos: i32 = p.add(match self._target_direction {
            MovementDirection::Closing => { -1}
            MovementDirection::Opening => {1} 
            _ => {0}
        });

        self._current_position = Some(if pos < 0 {0} else if (pos as u16) > self._max_pulses { self._max_pulses } else { pos as u16 } );

        //self._set_next_tick_millis(self._target_speed);        
    }
}


#[cfg(test)]
mod tests {
    use crate::eev::Eev;
    use core::time::Duration;

    extern crate linux_embedded_hal as hal;

    use crate::hal::prelude::_embedded_hal_timer_CountDown;

    use self::hal::SysTimer;
    
    #[test]
    fn eev_initialization() {

        let mut timer : SysTimer = SysTimer::new();

        //let mock_stepper_driver = mocker.Mock();
        let range : u16 = 100;
        let overdrive : u16 = 10;
        let max_steps : u16 = range + overdrive;
        let mut eev = Eev::new(range, overdrive, [Duration::from_millis(3), Duration::from_millis(2), Duration::from_millis(1)]);
        assert!(eev.current_position() == None);
        eev.initialize().await;
        assert!(eev.current_position() == Some(max_steps));
        let mut step = 0;
        while step <= max_steps {
            eev.run().await;
            //time.sleep(0.005);
            timer.start(Duration::from_millis(10));
            timer.wait();
            step += 1;
        }
        assert!(eev.current_position() == Some(0));
        assert!(eev.is_closed() == true);
    }
}
