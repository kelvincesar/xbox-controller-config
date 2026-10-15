use super::model::{Brightness, Command, LedMode, LedSetting, Sequence};

/// Sequence number after `current`. Wraps from 255 to 1, skipping 0, as SDL does.
pub fn next_sequence(current: Sequence) -> Sequence {
    match current.0.wrapping_add(1) {
        0 => Sequence(1),
        next => Sequence(next),
    }
}

/// The setting actually sent for a requested mode and brightness.
/// `Off` always goes out at brightness 0, matching SDL's `SetHomeLED`.
pub fn led_setting(mode: LedMode, brightness: Brightness) -> LedSetting {
    let brightness = match mode {
        LedMode::Off => Brightness::OFF,
        _ => brightness,
    };
    LedSetting { mode, brightness }
}

/// The pattern the controller shows after a driver initialises it.
pub fn default_mode() -> LedMode {
    LedMode::Steady
}

/// Packets the controller sends many times a second; not worth logging.
pub fn noisy(command: Command) -> bool {
    matches!(command, Command::Status | Command::Input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_sequence_is_none_it_should_start_at_one() {
        assert_eq!(next_sequence(Sequence::NONE), Sequence(1));
    }

    #[test]
    fn when_sequence_is_mid_range_it_should_increment() {
        assert_eq!(next_sequence(Sequence(1)), Sequence(2));
        assert_eq!(next_sequence(Sequence(254)), Sequence(255));
    }

    #[test]
    fn when_sequence_is_255_it_should_wrap_to_one_not_zero() {
        assert_eq!(next_sequence(Sequence(255)), Sequence(1));
    }

    #[test]
    fn when_mode_is_off_it_should_force_brightness_to_zero() {
        let setting = led_setting(LedMode::Off, Brightness::MAX);
        assert_eq!(setting.brightness, Brightness::OFF);
    }

    #[test]
    fn when_mode_is_on_it_should_keep_requested_brightness() {
        let setting = led_setting(LedMode::Steady, Brightness::saturating(33));
        assert_eq!(setting.brightness, Brightness::saturating(33));
    }

    #[test]
    fn when_asking_for_the_default_mode_it_should_be_steady() {
        assert_eq!(default_mode(), LedMode::Steady);
    }

    #[test]
    fn when_command_is_status_or_input_it_should_be_noisy() {
        assert!(noisy(Command::Status));
        assert!(noisy(Command::Input));
    }

    #[test]
    fn when_command_is_led_or_announce_it_should_not_be_noisy() {
        assert!(!noisy(Command::Led));
        assert!(!noisy(Command::Announce));
        assert!(!noisy(Command::Unknown(0x7F)));
    }
}
