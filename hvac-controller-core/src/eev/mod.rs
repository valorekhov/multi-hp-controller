use crate::hal::{Stepper, MovementDirection};
use async_mutex::Mutex;

use core::time::Duration;
use core::ops::{Div, Mul};
use alloc::boxed::Box;
use embedded_hal_async::delay::DelayUs;

use percentage::{Percentage, PercentageInteger};
use crate::ActuationError;

pub struct Eev<TDelay> where TDelay : DelayUs{
    _max_pulses: u16,
    _overdrive: u16,
    _current_position: Option<u16>,
    _current_direction: Option<MovementDirection>,
    _target_speeds: [Duration; 3],
    _stepper_mutex: Mutex<Box<dyn Stepper>>,
    _delay_generator: TDelay
}

impl<TDelay> Eev<TDelay>  where TDelay : DelayUs {
    pub fn new(
        stepper_driver: Box<dyn Stepper>,
        max_pulses: u16,
        overdrive: u16,
        target_speeds: [Duration; 3],
        delay_generator: TDelay,
    ) -> Self {
        Eev {
            _max_pulses: max_pulses,
            _overdrive: overdrive,
            _current_position: None,
            _current_direction: None,
            _target_speeds: target_speeds,
            _stepper_mutex: Mutex::new(stepper_driver),
            _delay_generator: delay_generator
        }
    }

    pub fn current_position(&self) -> Option<PercentageInteger> {
        self._current_position.map(|pos| Percentage::from(  pos.mul(100).div(self._max_pulses)))
    }

    pub fn is_fully_closed(&self) -> bool { self._current_position == Some(0) }

    pub fn is_fully_opened(&self) -> bool { self._current_position == Some(self._max_pulses) }

    // Initialize the EEV to its home position.
    // Sets the stepper to drive in the closing direction for the count of
    // max_pulses plus overdrive.
    pub async fn initialize(&mut self) -> Result<PercentageInteger, ActuationError> {
        self._current_position = Some(self._max_pulses + self._overdrive);
        self.actuate(Percentage::from(0), 1).await
    }

    // Operate the EEV stepper motor.
    // Checks if the current millis is greater than the specified next millis.
    // If so, then the stepper is driven in the target direction.
    // If the target position is reached, then the stepper is stopped.
    pub async fn actuate(&mut self, target_position: PercentageInteger, speed: usize) -> Result<PercentageInteger, ActuationError> {
        if self._current_position.is_none() { panic!("Current position unknown. Call `::initialize()` first!"); }

        let target_pulsed_position = target_position.apply_to(self._max_pulses);
        let mut current_pulsed_position = self._current_position.unwrap();
        if target_pulsed_position == current_pulsed_position {
            return Ok(target_position);
        }

        let direction = if target_pulsed_position < current_pulsed_position {MovementDirection::Close} else {MovementDirection::Open};
        self._current_direction = Some(direction);

        {
            // Lock on the stepper to ensure only a single operation works at a time
            let stepper = self._stepper_mutex.lock().await;

            while (direction == MovementDirection::Close && current_pulsed_position > target_pulsed_position)
                || (direction == MovementDirection::Open && current_pulsed_position < target_pulsed_position) {
                stepper.actuate(&direction)?;

                current_pulsed_position =
                    current_pulsed_position.saturating_add_signed(match direction {
                        MovementDirection::Close => { -1 }
                        MovementDirection::Open => { 1 }
                        _ => { 0 }
                    });

                if current_pulsed_position > self._max_pulses {
                    current_pulsed_position = self._max_pulses;
                }

                self._current_position = Some(current_pulsed_position);

                // Explicit termination conditions at extreme edges of motion range
                if current_pulsed_position == 0 && direction == MovementDirection::Close {
                    return Ok(Percentage::from(0));
                }

                if current_pulsed_position == self._max_pulses && direction == MovementDirection::Open {
                    return Ok(Percentage::from(100));
                }

                let _ = self._delay_generator.delay_ms(self._target_speeds[speed].as_millis() as u32).await;
            }

            Ok(Percentage::from((current_pulsed_position as u32).mul(100).div(self._max_pulses as u32)))
        }
    }
}


#[cfg(test)]
mod tests {

}
