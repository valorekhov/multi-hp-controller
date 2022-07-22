import os
import sys
import pytest

current = os.path.dirname(os.path.realpath(__file__))
parent = os.path.dirname(current)
sys.path.append(parent)
from eev import Eev

def test_eev_init():
    eev = Eev(100, 10)
    assert eev is not None, "EEV class initialized"
    assert eev.current_position() is None, "EEV position unknown"
    eev.initialize()
    assert eev.current_position() == 0, "EEV not homed in"