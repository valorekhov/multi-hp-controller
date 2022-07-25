#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::*;

    const current: _ = os.path.dirname(os.path.realpath(__file__));
    const parent: _ = os.path.dirname(current);

    use eev::Eev;

    fn test_eev_initialization<T0>(mocker: T0) {
        let mock_stepper_driver = mocker.Mock();
        let range = 100;
        let overdrive = 10;
        let max_steps = (range + overdrive);
        let eev = Eev(mock_stepper_driver, range, overdrive, vec![0.001, 0.002]);
        assert!(eev.current_position() == None);
        eev.initialize();
        assert!(eev.current_position() == max_steps);
        let mut step = 0;
        while step <= max_steps {
            eev.run();
            time.sleep(0.005);
            step += 1;
        }
        assert!(eev.current_position() == 0);
        assert!(eev.is_closed() == true);
    }
}
