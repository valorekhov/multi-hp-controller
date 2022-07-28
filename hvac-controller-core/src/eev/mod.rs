use crate::hal::{Stepper, MovementDirection};
use async_mutex::Mutex;

use core::ops::Add;
use core::time::Duration;
use alloc::boxed::Box;

use percentage::PercentageInteger;

pub struct Eev {
    _max_pulses: u16,
    _overdrive: u16,
    _current_position: Option<u16>,
    _current_direction: Option<MovementDirection>,
    _target_speeds: [Duration; 3],
    _stepper_mutex: Mutex<Box<dyn Stepper>>
}

impl Eev {
    pub fn new(
        stepper_driver: Box<dyn Stepper>,
        max_pulses: u16,
        overdrive: u16,
        target_speeds: [Duration; 3],
    ) -> Self {
        Eev {
            _max_pulses: max_pulses,
            _overdrive: overdrive,
            _current_position: None,
            _current_direction: None,
            _target_speeds: target_speeds,
            _stepper_mutex: Mutex::new(stepper_driver)
        }
    }

    // Initialize the EEV to its home position.
    // Sets the stepper to drive in the closing direction for the count of
    // max_pulses plus overdrive.     
    pub async fn initialize(&mut self) {
        self._current_position = Some(self._max_pulses + self._overdrive);
        //self._set_next_tick_millis(0);
    }

    pub fn current_position(&self) -> Option<PercentageInteger> { Some(PercentageInteger::from( self._max_pulses / self._current_position ))  }

    pub fn is_closed(&self) -> bool { self._current_position == Some(0) }

    // Operate the EEV stepper motor.
    // Checks if the current millis is greater than the specified next millis.
    // If so, then the stepper is driven in the target direction.
    // If the target position is reached, then the stepper is stopped.
    pub async fn run(&mut self, target_position: PercentageInteger,  speed: usize) {
        if self._current_position.is_none() { panic!("Current position unknown. Call `::initialize()` first!"); }

        let target_pulsed_position = target_position.apply_to(self._max_pulses);
        let current_pulsed_position = self._current_position.unwrap();
        if target_pulsed_position == current_pulsed_position {
            return;
        }
        // TODO: Calc direction from current and target positions;
        let direction = if target_pulsed_position < current_pulsed_position {MovementDirection::Closing} else {MovementDirection::Opening};

        // Lock on the stepper to ensure only a single operation works at a time
        let stepper = self._stepper_mutex.lock().await;
        stepper.as_ref().one_step(direction);


        let p: i32 = self._current_position.unwrap_or(0) as i32;
        let pos: i32 = p.add(match direction {
            MovementDirection::Closing => {-1}
            MovementDirection::Opening => {1} 
            _ => {0}
        });

        self._current_position = Some(if pos < 0 {0} else if (pos as u16) > self._max_pulses { self._max_pulses } else { pos as u16 } );

        let delay = self._target_speeds[speed];
        // TODO: Sleep according to specified speed

        drop(stepper);
        //self._set_next_tick_millis(self._target_speed);        
    }
}


#[cfg(test)]
mod tests {

}
