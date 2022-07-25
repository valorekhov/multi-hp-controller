use std::collections::HashMap;
use std::*;

use stepper::{StepperMotor, BACKWARD, FORWARD};
const CLOSING: _ = -1;
const OPENING: _ = 1;

struct Eev {
    _stepper_driver: StepperMotor,
    _max_pulses: u32,
    _overdrive: u32,
    _current_position: Option<_>,
    _target_position: Option<_>,
    _target_direction: Option<_>,
    _target_speeds: ST0,
    _target_speed: ST1,
    _next_millis: ST2,
}

impl Eev {
    fn __init__<T0>(
        &self,
        stepper_driver: StepperMotor,
        max_pulses: i32,
        overdrive: i32,
        target_speeds: T0,
    ) {
        self._stepper_driver = stepper_driver;
        self._max_pulses = max_pulses;
        self._overdrive = overdrive;
        self._current_position = None;
        self._target_position = None;
        self._target_direction = None;
        self._target_speeds = target_speeds;
        /*pass*/
    }
    fn initialize(&self) {
        "Initialize the EEV to its home position.
            Sets the stepper to drive in the closing direction for the count of
            max_pulses plus overdrive.
        ";
        self._target_position = 0;
        self._target_direction = CLOSING;
        self._target_speed = 0;
        self._current_position = (self._max_pulses + self._overdrive);
        self._set_next_tick_millis(0);
    }
    fn _set_next_tick_millis<T0, RT>(&self, speed: T0) -> RT {
        "Sets and returns the next tick millis.
            This is the next time the stepper will be driven.
            Calculated as current monotonic time plus the specified target speed millis.
        ";
        self._next_millis = (time.monotonic() + self._target_speeds[speed]);
        return self._next_millis;
    }
    fn current_position<RT>(&self) -> RT {
        return self._current_position;
    }
    fn is_closed<RT>(&self) -> RT {
        return self._current_position == 0;
    }
    fn run(&self) {
        "Operate the EEV stepper motor.
            Checks if the current millis is greater than the specified next millis.
            If so, then the stepper is driven in the target direction.
            If the target position is reached, then the stepper is stopped.
        ";
        if self._target_position == self._current_position {
            return;
        }
        if time.monotonic() > self._next_millis {
            self._stepper_driver
                .onestep(if self._target_direction == CLOSING {
                    BACKWARD
                } else {
                    FORWARD
                });
            self._current_position += (1 * self._target_direction);
            self._set_next_tick_millis(self._target_speed);
        }
    }
}
