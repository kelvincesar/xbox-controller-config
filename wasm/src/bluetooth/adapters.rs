//! Bluetooth HID reports <-> model.
//!
//! WebHID passes the report ID separately and strips it from the payload, so
//! byte offsets here are one less than in SDL, which reads raw hidapi buffers.

use super::model::{Battery, BatteryLevel, Rumble};

pub const RUMBLE_REPORT_ID: u8 = 0x03;
const BATTERY_REPORT_ID: u8 = 0x04;

/// Payload of output report 0x03:
/// `[enable_mask, left_trigger, right_trigger, low_freq, high_freq, duration, delay, loops]`.
/// `0xFF, 0x00, 0xEB` are SDL's values: keep going until the next report.
pub fn rumble_report(rumble: Rumble) -> Vec<u8> {
    vec![
        0x0F,
        rumble.left_trigger.value(),
        rumble.right_trigger.value(),
        rumble.low_frequency.value(),
        rumble.high_frequency.value(),
        0xFF,
        0x00,
        0xEB,
    ]
}

/// Battery state from input report 0x04; `None` for any other report.
pub fn battery(report_id: u8, payload: &[u8]) -> Option<Battery> {
    let (BATTERY_REPORT_ID, [flags, ..]) = (report_id, payload) else {
        return None;
    };
    let level = match flags & 0x03 {
        0 => BatteryLevel::Critical,
        1 => BatteryLevel::Low,
        2 => BatteryLevel::Medium,
        _ => BatteryLevel::Full,
    };
    Some(Battery {
        level,
        charging: (flags & 0x0C) >> 2 == 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bluetooth::model::Strength;

    #[test]
    fn when_encoding_rumble_it_should_match_sdl_without_report_id() {
        // SDL: { 0x03, 0x0F, lt, rt, low, high, 0xFF, 0x00, 0xEB }
        let rumble = Rumble {
            left_trigger: Strength::saturating(1),
            right_trigger: Strength::saturating(2),
            low_frequency: Strength::saturating(3),
            high_frequency: Strength::saturating(4),
        };
        assert_eq!(
            rumble_report(rumble),
            vec![0x0F, 1, 2, 3, 4, 0xFF, 0x00, 0xEB]
        );
    }

    #[test]
    fn when_rumble_is_still_it_should_send_zero_strengths() {
        assert_eq!(rumble_report(Rumble::STILL)[1..5], [0, 0, 0, 0]);
    }

    #[test]
    fn when_report_is_battery_it_should_read_the_level() {
        assert_eq!(
            battery(0x04, &[0b0100]).unwrap().level,
            BatteryLevel::Critical
        );
        assert_eq!(battery(0x04, &[0b0101]).unwrap().level, BatteryLevel::Low);
        assert_eq!(
            battery(0x04, &[0b0110]).unwrap().level,
            BatteryLevel::Medium
        );
        assert_eq!(battery(0x04, &[0b0111]).unwrap().level, BatteryLevel::Full);
    }

    #[test]
    fn when_power_bits_are_zero_it_should_be_charging() {
        assert!(battery(0x04, &[0b0011]).unwrap().charging);
        assert!(!battery(0x04, &[0b0111]).unwrap().charging);
    }

    #[test]
    fn when_report_is_not_battery_it_should_return_none() {
        assert_eq!(battery(0x01, &[0x06]), None);
    }

    #[test]
    fn when_battery_payload_is_empty_it_should_return_none() {
        assert_eq!(battery(0x04, &[]), None);
    }
}
