//! wasm-bindgen boundary for the Bluetooth controller.

use wasm_bindgen::prelude::*;

use super::model::Rumble;
use super::{adapters, logic};

/// Battery reading handed to JS.
#[wasm_bindgen]
pub struct BatteryStatus {
    pub percent: u8,
    pub charging: bool,
}

#[wasm_bindgen(js_name = rumbleReportId)]
pub fn rumble_report_id() -> u8 {
    adapters::RUMBLE_REPORT_ID
}

#[wasm_bindgen(js_name = testRumbleReport)]
pub fn test_rumble_report() -> Vec<u8> {
    adapters::rumble_report(logic::test_rumble())
}

#[wasm_bindgen(js_name = stopRumbleReport)]
pub fn stop_rumble_report() -> Vec<u8> {
    adapters::rumble_report(Rumble::STILL)
}

/// Battery status for input report 0x04, `undefined` for every other report.
#[wasm_bindgen(js_name = batteryStatus)]
pub fn battery_status(report_id: u8, payload: &[u8]) -> Option<BatteryStatus> {
    adapters::battery(report_id, payload).map(|battery| BatteryStatus {
        percent: logic::battery_percent(battery.level),
        charging: battery.charging,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_report_is_battery_it_should_return_percent_and_charging() {
        let status = battery_status(0x04, &[0b0110]).unwrap();
        assert_eq!((status.percent, status.charging), (70, false));
    }

    #[test]
    fn when_report_is_input_it_should_return_no_battery() {
        assert!(battery_status(0x01, &[0b0110]).is_none());
    }

    #[test]
    fn when_stopping_rumble_it_should_zero_every_motor() {
        assert_eq!(stop_rumble_report()[1..5], [0, 0, 0, 0]);
    }
}
