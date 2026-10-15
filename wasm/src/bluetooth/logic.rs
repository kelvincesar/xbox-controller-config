use super::model::{BatteryLevel, Rumble, Strength};

/// Percentage shown for a battery level. Values from Microsoft's GameInput
/// `GetBatteryState` mapping, as used by SDL.
pub fn battery_percent(level: BatteryLevel) -> u8 {
    match level {
        BatteryLevel::Critical => 10,
        BatteryLevel::Low => 40,
        BatteryLevel::Medium => 70,
        BatteryLevel::Full => 100,
    }
}

/// A short, gentle buzz on the grip motors to confirm the link works.
pub fn test_rumble() -> Rumble {
    Rumble {
        low_frequency: Strength::saturating(35),
        high_frequency: Strength::saturating(35),
        ..Rumble::STILL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_battery_level_is_known_it_should_map_to_sdl_percentages() {
        assert_eq!(battery_percent(BatteryLevel::Critical), 10);
        assert_eq!(battery_percent(BatteryLevel::Low), 40);
        assert_eq!(battery_percent(BatteryLevel::Medium), 70);
        assert_eq!(battery_percent(BatteryLevel::Full), 100);
    }

    #[test]
    fn when_testing_rumble_it_should_only_use_the_grip_motors() {
        let rumble = test_rumble();
        assert_eq!(rumble.left_trigger, Strength::NONE);
        assert_eq!(rumble.right_trigger, Strength::NONE);
        assert!(rumble.low_frequency.value() > 0);
        assert!(rumble.high_frequency.value() > 0);
    }
}
