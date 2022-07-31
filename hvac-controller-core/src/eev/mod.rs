use crate::hal::{Stepper, MovementDirection};
use async_mutex::Mutex;

use core::time::Duration;
use core::ops::{Div, Mul};
use embedded_hal_async::delay::DelayUs;

use percentage::{Percentage, PercentageInteger};
use crate::ActuationError;


/// EEV is a controller for a stepper-driver actuated Electronic Expansion Valve. 
/// The EEV is configured with a movement range expressed in stepper actuation pulses.
/// To support initialization, the EEV also accepts an "overdrive" stepper pulse count
/// for valves which allow exceeding the movement range in the closing position to support reliable
/// initialization.
/// Upon startup the valve moves into the initially-closed position by issing the maximum range 
/// plust "overdrive" number of pulses in the closing direction.
/// Once initialized, acutation range may be set using `PercentageInteger` values 0..100.
/// Movement speed for initialization and actuation operations is specified as a `Duration` interval between
/// individual stepper pulses.
/// To support delay-based actuation, the EEV also accepts a `DelayUs` implementation which is used to generate
/// delay futures.
pub struct Eev<TStepper, TDelay> where TStepper: Stepper, TDelay : DelayUs {
    _max_pulses: u16,
    _overdrive: u16,
    _current_position: Option<u16>,
    _current_direction: Option<MovementDirection>,
    _stepper_mutex: Mutex<TStepper>,
    _delay_generator: TDelay
}

impl<TStepper, TDelay> Eev<TStepper, TDelay> where TStepper: Stepper, TDelay : DelayUs {
    pub fn new(
        stepper_driver: TStepper,
        max_pulses: u16,
        overdrive: u16,
        delay_generator: TDelay,
    ) -> Self {
        Eev {
            _max_pulses: max_pulses,
            _overdrive: overdrive,
            _current_position: None,
            _current_direction: None,
            _stepper_mutex: Mutex::new(stepper_driver),
            _delay_generator: delay_generator
        }
    }

    pub fn current_position(&self) -> Option<PercentageInteger> {
        self._current_position.map(|pos| Percentage::from(  pos.mul(100).div(self._max_pulses)))
    }

    pub fn current_direction(&self) -> Option<MovementDirection> {
        self._current_direction
    }

    pub fn is_fully_closed(&self) -> bool { self._current_position == Some(0) }

    pub fn is_fully_opened(&self) -> bool { self._current_position == Some(self._max_pulses) }

    // Initialize the EEV to its home position.
    // Sets the stepper to drive in the closing direction for the count of
    // max_pulses plus overdrive.
    pub async fn initialize(&mut self, step_interval: Duration) -> Result<PercentageInteger, ActuationError> {
        self._current_position = Some(self._max_pulses + self._overdrive);
        self.actuate(Percentage::from(0), step_interval).await
    }

    // Operate the EEV stepper motor.
    // Checks if the current millis is greater than the specified next millis.
    // If so, then the stepper is driven in the target direction.
    // If the target position is reached, then the stepper is stopped.
    pub async fn actuate(&mut self, target_position: PercentageInteger, step_interval: Duration) -> Result<PercentageInteger, ActuationError> {
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

            let opening = direction == MovementDirection::Open;
            let closing = direction == MovementDirection::Close;

            while (closing && current_pulsed_position > target_pulsed_position)
                || (opening && current_pulsed_position < target_pulsed_position) {
                stepper.actuate(&direction)?;

                current_pulsed_position =
                    current_pulsed_position.saturating_add_signed(match direction {
                        MovementDirection::Close => { -1 }
                        MovementDirection::Open => { 1 }
                        _ => { 0 }
                    });

                self._current_position = Some(current_pulsed_position);

                // Explicit termination conditions at extreme edges of motion range
                if (closing && current_pulsed_position == 0) 
                    || (opening && current_pulsed_position >= self._max_pulses) {
                    break;
                }

                let _ = self._delay_generator.delay_ms(step_interval.as_millis() as u32).await;
            }

            stepper.release().expect("Failed to release stepper");
            self._current_direction = Some(MovementDirection::Hold);

            Ok(Percentage::from((current_pulsed_position as u32).mul(100).div(self._max_pulses as u32)))
        }
    }
}


#[cfg(test)]
mod tests {

}
