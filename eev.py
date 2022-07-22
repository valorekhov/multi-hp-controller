class Eev:
    def __init__(self, max_pulses: int, overdrive: int) -> None:
        self._max_pulses = max_pulses
        self._overdrive = overdrive
        self._current_position = None
        pass

    def initialize(self):
        pass

    def current_position(self):
        return self._current_position
