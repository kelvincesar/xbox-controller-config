//! wasm-bindgen boundary for the wired controller. Each export adapts its
//! JS arguments into model values, asks logic, and adapts the answer back.

use wasm_bindgen::prelude::*;

use super::model::{Brightness, LedMode, Sequence};
use super::{adapters, logic};

/// Keeps the packet sequence counter for one USB connection.
#[wasm_bindgen]
pub struct GipSession {
    sequence: Sequence,
}

#[wasm_bindgen]
impl GipSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GipSession {
        GipSession {
            sequence: Sequence::NONE,
        }
    }

    fn advance(&mut self) -> Sequence {
        self.sequence = logic::next_sequence(self.sequence);
        self.sequence
    }

    #[wasm_bindgen(js_name = powerOnPacket)]
    pub fn power_on_packet(&mut self) -> Vec<u8> {
        let sequence = self.advance();
        adapters::power_on_packet(sequence)
    }

    /// LED packet for a mode code from `ledModes()` and a brightness; values
    /// above `brightnessMax()` saturate.
    #[wasm_bindgen(js_name = ledPacket)]
    pub fn led_packet(&mut self, mode: u8, brightness: u8) -> Result<Vec<u8>, JsError> {
        let mode = LedMode::try_from(mode)
            .map_err(|code| JsError::new(&format!("unknown LED mode 0x{code:02X}")))?;
        let setting = logic::led_setting(mode, Brightness::saturating(brightness));
        let sequence = self.advance();
        Ok(adapters::led_packet(sequence, setting))
    }
}

impl Default for GipSession {
    fn default() -> Self {
        GipSession::new()
    }
}

#[wasm_bindgen(js_name = ledModes)]
pub fn led_modes() -> Vec<u8> {
    LedMode::ALL.into_iter().map(adapters::mode_code).collect()
}

#[wasm_bindgen(js_name = ledModeLabel)]
pub fn led_mode_label(mode: u8) -> String {
    LedMode::try_from(mode)
        .map(adapters::mode_label)
        .unwrap_or("Unknown")
        .to_string()
}

#[wasm_bindgen(js_name = ledModeKey)]
pub fn led_mode_key(mode: u8) -> String {
    LedMode::try_from(mode)
        .map(adapters::mode_key)
        .unwrap_or("unknown")
        .to_string()
}

#[wasm_bindgen(js_name = ledModeOff)]
pub fn led_mode_off() -> u8 {
    adapters::mode_code(LedMode::Off)
}

#[wasm_bindgen(js_name = ledModeDefault)]
pub fn led_mode_default() -> u8 {
    adapters::mode_code(logic::default_mode())
}

#[wasm_bindgen(js_name = brightnessMax)]
pub fn brightness_max() -> u8 {
    Brightness::MAX.value()
}

#[wasm_bindgen(js_name = brightnessDefault)]
pub fn brightness_default() -> u8 {
    Brightness::DEFAULT.value()
}

#[wasm_bindgen(js_name = packetSummary)]
pub fn packet_summary(bytes: &[u8]) -> String {
    adapters::summary(bytes)
}

/// Whether a received packet is routine chatter the log should skip.
#[wasm_bindgen(js_name = noisyPacket)]
pub fn noisy_packet(bytes: &[u8]) -> bool {
    adapters::packet(bytes).is_some_and(|packet| logic::noisy(packet.command))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_session_is_new_it_should_number_packets_from_one() {
        let mut session = GipSession::new();
        assert_eq!(session.power_on_packet()[2], 1);
        assert_eq!(session.led_packet(0x00, 0).unwrap()[2], 2);
    }

    #[test]
    fn when_brightness_is_out_of_range_it_should_saturate() {
        let mut session = GipSession::new();
        assert_eq!(session.led_packet(0x01, 255).unwrap()[6], 50);
    }

    #[test]
    fn when_mode_is_off_it_should_send_zero_brightness() {
        let mut session = GipSession::new();
        assert_eq!(session.led_packet(0x00, 40).unwrap()[6], 0);
    }

    #[test]
    fn when_listing_modes_it_should_return_every_wire_code() {
        assert_eq!(led_modes(), vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x08, 0x09]);
    }

    #[test]
    fn when_mode_code_is_unknown_it_should_label_it_unknown() {
        assert_eq!(led_mode_label(0x00), "Off");
        assert_eq!(led_mode_label(0x42), "Unknown");
    }

    #[test]
    fn when_packet_is_input_it_should_be_noisy() {
        assert!(noisy_packet(&[0x20, 0x00, 0x01, 0x0E]));
        assert!(!noisy_packet(&[0x02, 0x20, 0x01, 0x1C]));
        assert!(!noisy_packet(&[0x20]));
    }
}
