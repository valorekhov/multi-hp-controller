import os
import sys
import time

current = os.path.dirname(os.path.realpath(__file__))
parent = os.path.dirname(current)
sys.path.append(parent)
from eev import Eev

def test_eev_initialization(mocker):
    # Mock the stepper driver
    mock_stepper_driver = mocker.Mock()

    range = 100
    overdrive = 10

    max_steps = range+overdrive

    eev = Eev(mock_stepper_driver, range, overdrive, [.001, .002])
    assert eev.current_position() is None, "EEV position unknown"
    eev.initialize()
    assert eev.current_position() is max_steps, "EEV current position is max_steps"
    
    step = 0
    while step <= max_steps:
        # print(time.monotonic(), eev.current_position())
        eev.run()
        time.sleep(.005)
        step += 1

    # assert step == max_steps + 1, "EEV initialized in under maximum steps"
    assert eev.current_position() == 0, "EEV position is zero"
    assert eev.is_closed() is True, "EEV is closed"