from stepper import StepperMotor, FORWARD, BACKWARD
import time

CLOSING = -1
OPENING = 1
"""Step forward"""

class Eev:
    def __init__(self, stepper_driver: StepperMotor, 
        max_pulses: int, overdrive: int, 
        ### Target speeds expressed in millis per pulse
        ### Coversion is 1 / pps
        target_speeds = [ 1 / 30, 1 / 80 ]) -> None:
        self._stepper_driver = stepper_driver
        self._max_pulses = max_pulses
        self._overdrive = overdrive
        self._current_position = None
        self._target_position = None
        self._target_direction = None
        self._target_speeds = target_speeds
        pass

    def initialize(self):
        """Initialize the EEV to its home position.
            Sets the stepper to drive in the closing direction for the count of
            max_pulses plus overdrive.
        """
        self._target_position = 0
        self._target_direction = CLOSING
        self._target_speed = 0
        self._current_position = self._max_pulses + self._overdrive
        self._set_next_tick_millis(0)

    def _set_next_tick_millis(self, speed = 0):
        """Sets and returns the next tick millis.
            This is the next time the stepper will be driven.
            Calculated as current monotonic time plus the specified target speed millis.
        """
        self._next_millis = time.monotonic() + self._target_speeds[speed]
        return self._next_millis

    def current_position(self):
        return self._current_position

    def is_closed(self):
        return self._current_position == 0

    def run(self):
        """Operate the EEV stepper motor.
            Checks if the current millis is greater than the specified next millis.
            If so, then the stepper is driven in the target direction.
            If the target position is reached, then the stepper is stopped.
        """
        if self._target_position == self._current_position:
            return
        if time.monotonic() > self._next_millis:
            self._stepper_driver.onestep(BACKWARD if self._target_direction == CLOSING else FORWARD)
            self._current_position += 1 * self._target_direction
            self._set_next_tick_millis(self._target_speed)
        pass
