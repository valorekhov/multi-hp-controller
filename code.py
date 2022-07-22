import time
import board
import digitalio
from stepper import StepperMotor, BACKWARD, DOUBLE, INTERLEAVE

coils = (
    digitalio.DigitalInOut(board.IO14),  
    digitalio.DigitalInOut(board.IO13),  
    digitalio.DigitalInOut(board.IO12),  
    digitalio.DigitalInOut(board.IO11),  
)

for coil in coils:
    coil.direction = digitalio.Direction.OUTPUT

firing_sequence = bytes(
    [0b1001, 0b1100, 0b0110, 0b0011]
)

ets_6_firing_sequence = bytes(
    [0b1000, 0b1010, 0b0010, 0b0110, 0b0100, 0b0101, 0b0001, 0b1001]
)

motor = StepperMotor(coils[0], coils[1], coils[2], coils[3], single_firing_sequence=firing_sequence)


#Define steps per rotation
DELAY = 0.01
STEPS = 2048

for step in range(STEPS):
    motor.onestep()
    time.sleep(DELAY)

for step in range(STEPS):
    motor.onestep(direction=BACKWARD)
    time.sleep(DELAY)

# for step in range(STEPS):
#     motor.onestep(style=DOUBLE)
#     time.sleep(DELAY)

# for step in range(STEPS):
#     motor.onestep(direction=BACKWARD, style=DOUBLE)
#     time.sleep(DELAY)

# for step in range(STEPS):
#     motor.onestep(style=INTERLEAVE)
#     time.sleep(DELAY)

# for step in range(STEPS):
#     motor.onestep(direction=BACKWARD, style=INTERLEAVE)
#     time.sleep(DELAY)

motor.release()

time.sleep(30)

