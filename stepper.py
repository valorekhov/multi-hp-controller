# SPDX-FileCopyrightText: 2017 Scott Shawcroft  for Adafruit Industries
#
# SPDX-License-Identifier: MIT

"""
`adafruit_motor.stepper`
====================================================

Stepper motors feature multiple wire coils that are used to rotate the magnets connected to the
motor shaft in a precise way. Each increment of the motor is called a step. Stepper motors have a
varying number of steps per rotation so check the motor's documentation to determine exactly how
precise each step is.

* Author(s): Tony DiCola, Scott Shawcroft
"""
try:
    from micropython import const
except ImportError:
    def const(x): return x
    pass


try:
    from typing import Union, Optional
    from digitalio import DigitalInOut

except ImportError:
    class DigitalInOut:
        pass
    pass

__version__ = "3.4.2"
__repo__ = "https://github.com/adafruit/Adafruit_CircuitPython_Motor.git"

# Stepper Motor Shield/Wing Driver
# Based on Adafruit Motorshield library:
# https://github.com/adafruit/Adafruit_Motor_Shield_V2_Library

# Constants that specify the direction and style of steps.
FORWARD = const(1)
"""Step forward"""
BACKWARD = const(2)
""""Step backward"""
SINGLE = const(1)
"""Step so that each step only activates a single coil"""
DOUBLE = const(2)
"""Step so that each step only activates two coils to produce more torque."""
INTERLEAVE = const(3)
"""Step half a step to alternate between single coil and double coil steps."""

_SINGLE_STEPS = bytes([0b0010, 0b0100, 0b0001, 0b1000])

_DOUBLE_STEPS = bytes([0b1010, 0b0110, 0b0101, 0b1001])

_INTERLEAVE_STEPS = bytes(
    [0b1010, 0b0010, 0b0110, 0b0100, 0b0101, 0b0001, 0b1001, 0b1000]
)


class StepperMotor:
    """A bipolar stepper motor or four coil unipolar motor. The use of microstepping requires
    pins that can output PWM. For non-microstepping, can set microsteps to None and use
    digital out pins.

    **Digital Out**

    :param ~digitalio.DigitalInOut ain1: `digitalio.DigitalInOut`-compatible output connected to
      the driver for the first coil (unipolar) or first input to first coil (bipolar).
    :param ~digitalio.DigitalInOut ain2: `digitalio.DigitalInOut`-compatible output connected to
      the driver for the third coil (unipolar) or second input to first coil (bipolar).
    :param ~digitalio.DigitalInOut bin1: `digitalio.DigitalInOut`-compatible output connected to
      the driver for the second coil (unipolar) or first input to second coil (bipolar).
    :param ~digitalio.DigitalInOut bin2: `digitalio.DigitalInOut`-compatible output connected to
      the driver for the fourth coil (unipolar) or second input to second coil (bipolar).
    :param microsteps: set to `None`
    """

    def __init__(
        self,
        ain1: DigitalInOut,
        ain2: DigitalInOut,
        bin1: DigitalInOut,
        bin2: DigitalInOut,
        *,
        single_firing_sequence : Optional[bytes] = None,
        double_firing_sequence : Optional[bytes] = None,
        interleave_firing_sequence : Optional[bytes] = None
    ) -> None:
        #
        # Digital IO Pins
        #
        self._steps = None
        self._coil = (ain1, ain2, bin1, bin2)

        self._current_microstep = 0

        self._single_step_sequence = _SINGLE_STEPS
        self._double_step_sequence = _DOUBLE_STEPS
        self._interleaave_step_sequence = _INTERLEAVE_STEPS

        if single_firing_sequence:
            self._single_step_sequence = single_firing_sequence
        if double_firing_sequence:
            self._double_step_sequence = double_firing_sequence
        if interleave_firing_sequence:
            self._double_step_sequence = interleave_firing_sequence

        self._update_coils()

    def _update_coils(self) -> None:
        #
        # Digital IO Pins
        #
        # Get coil activation sequence
        if self._steps is None:
            steps = 0b0000
        else:
            steps = self._steps[self._current_microstep % len(self._steps)]
        # Energize coils as appropriate:
        for i, coil in enumerate(self._coil):
            coil.value = (steps >> i) & 0x01

    def release(self) -> None:
        """Releases all the coils so the motor can free spin, also won't use any power"""
        # De-energize coils:
        for coil in self._coil:
            coil.value = 0

    def onestep(  # pylint: disable=too-many-branches
        self, *, direction: int = FORWARD, style: int = SINGLE
    ) -> None:
        """Performs one step of a particular style. The actual rotation amount will vary by style.
        `SINGLE` and `DOUBLE` will normal cause a full step rotation. `INTERLEAVE` will normally
        do a half step rotation. `MICROSTEP` will perform the smallest configured step.

        When step styles are mixed, subsequent `SINGLE`, `DOUBLE` or `INTERLEAVE` steps may be
        less than normal in order to align to the desired style's pattern.

        :param int direction: Either `FORWARD` or `BACKWARD`
        :param int style: `SINGLE`, `DOUBLE`, `INTERLEAVE`"""

        step_size = 1
        if style == SINGLE:
            self._steps = self._single_step_sequence
        elif style == DOUBLE:
            self._steps = self._double_step_sequence
        elif style == INTERLEAVE:
            self._steps = self._interleaave_step_sequence
        else:
            raise ValueError("Unsupported step style.")

        if direction == FORWARD:
            self._current_microstep += step_size
        else:
            self._current_microstep -= step_size

        # Now that we know our target microstep we can determine how to energize the four coils.
        self._update_coils()

        return self._current_microstep
