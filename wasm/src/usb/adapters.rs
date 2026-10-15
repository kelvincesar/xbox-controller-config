//! GIP wire format <-> model.
//!
//! Every packet: `[command, options, sequence, payload_length, payload...]`.
//! The LED payload is `[0x00, mode, brightness]`.

use super::model::{Brightness, Command, LedMode, LedSetting, Packet, Sequence};

const HEADER_LENGTH: usize = 4;
/// `GIP_OPT_INTERNAL`: a system command addressed to client 0.
const OPT_INTERNAL: u8 = 0x20;

const CMD_ACKNOWLEDGE: u8 = 0x01;
const CMD_ANNOUNCE: u8 = 0x02;
const CMD_STATUS: u8 = 0x03;
const CMD_IDENTIFY: u8 = 0x04;
const CMD_POWER: u8 = 0x05;
const CMD_AUTHENTICATE: u8 = 0x06;
const CMD_GUIDE_BUTTON: u8 = 0x07;
const CMD_RUMBLE: u8 = 0x09;
const CMD_LED: u8 = 0x0A;
const CMD_INPUT: u8 = 0x20;

pub fn mode_code(mode: LedMode) -> u8 {
    match mode {
        LedMode::Off => 0x00,
        LedMode::Steady => 0x01,
        LedMode::BlinkFast => 0x02,
        LedMode::Blink => 0x03,
        LedMode::BlinkSlow => 0x04,
        LedMode::FadeSlow => 0x08,
        LedMode::FadeFast => 0x09,
    }
}

impl TryFrom<u8> for LedMode {
    type Error = u8;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        LedMode::ALL
            .into_iter()
            .find(|mode| mode_code(*mode) == code)
            .ok_or(code)
    }
}

pub fn mode_label(mode: LedMode) -> &'static str {
    match mode {
        LedMode::Off => "Off",
        LedMode::Steady => "Steady",
        LedMode::BlinkFast => "Fast blink",
        LedMode::Blink => "Blink",
        LedMode::BlinkSlow => "Slow blink",
        LedMode::FadeSlow => "Slow fade",
        LedMode::FadeFast => "Fast fade",
    }
}

/// Stable identifier for a mode, used by the web UI to pick a preview animation.
pub fn mode_key(mode: LedMode) -> &'static str {
    match mode {
        LedMode::Off => "off",
        LedMode::Steady => "steady",
        LedMode::BlinkFast => "blink-fast",
        LedMode::Blink => "blink",
        LedMode::BlinkSlow => "blink-slow",
        LedMode::FadeSlow => "fade-slow",
        LedMode::FadeFast => "fade-fast",
    }
}

pub fn command(code: u8) -> Command {
    match code {
        CMD_ACKNOWLEDGE => Command::Acknowledge,
        CMD_ANNOUNCE => Command::Announce,
        CMD_STATUS => Command::Status,
        CMD_IDENTIFY => Command::Identify,
        CMD_POWER => Command::Power,
        CMD_AUTHENTICATE => Command::Authenticate,
        CMD_GUIDE_BUTTON => Command::GuideButton,
        CMD_RUMBLE => Command::Rumble,
        CMD_LED => Command::Led,
        CMD_INPUT => Command::Input,
        other => Command::Unknown(other),
    }
}

pub fn command_name(command: Command) -> &'static str {
    match command {
        Command::Acknowledge => "acknowledge",
        Command::Announce => "announce",
        Command::Status => "status",
        Command::Identify => "identify",
        Command::Power => "power",
        Command::Authenticate => "authenticate",
        Command::GuideButton => "guide button",
        Command::Rumble => "rumble",
        Command::Led => "LED",
        Command::Input => "input",
        Command::Unknown(_) => "unknown",
    }
}

/// `[0x05, 0x20, seq, 0x01, 0x00]`: the host is ready (SDL `xbox_init_power_on`).
pub fn power_on_packet(sequence: Sequence) -> Vec<u8> {
    vec![CMD_POWER, OPT_INTERNAL, sequence.0, 0x01, 0x00]
}

/// `[0x0A, 0x20, seq, 0x03, 0x00, mode, brightness]` (SDL `SetHomeLED`, xone `gip_set_led_mode`).
pub fn led_packet(sequence: Sequence, setting: LedSetting) -> Vec<u8> {
    vec![
        CMD_LED,
        OPT_INTERNAL,
        sequence.0,
        0x03,
        0x00,
        mode_code(setting.mode),
        setting.brightness.value(),
    ]
}

/// Read a packet header. `None` when the bytes are too short to be a packet.
pub fn packet(bytes: &[u8]) -> Option<Packet> {
    let [code, _options, sequence, _length, payload @ ..] = bytes else {
        return None;
    };
    let command = command(*code);
    let led = match (command, payload) {
        (Command::Led, [_, mode, brightness, ..]) => {
            LedMode::try_from(*mode).ok().map(|mode| LedSetting {
                mode,
                brightness: Brightness::saturating(*brightness),
            })
        }
        _ => None,
    };
    Some(Packet {
        command,
        sequence: Sequence(*sequence),
        led,
    })
}

pub fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// One-line, human-readable form of raw packet bytes for the activity log.
pub fn summary(bytes: &[u8]) -> String {
    let Some(packet) = packet(bytes) else {
        return format!("short packet ({} bytes, need {HEADER_LENGTH})", bytes.len());
    };
    let detail = packet
        .led
        .map(|led| {
            format!(
                ": {}, brightness {}",
                mode_label(led.mode),
                led.brightness.value()
            )
        })
        .unwrap_or_default();
    format!(
        "{}{detail} [seq {}] {}",
        command_name(packet.command),
        packet.sequence.0,
        hex(bytes)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setting(mode: LedMode, brightness: u8) -> LedSetting {
        LedSetting {
            mode,
            brightness: Brightness::saturating(brightness),
        }
    }

    #[test]
    fn when_led_is_default_it_should_match_sdl_init_packet() {
        // SDL xbox_init_enable_led, sequence filled in as 2.
        assert_eq!(
            led_packet(Sequence(2), setting(LedMode::Steady, 20)),
            vec![0x0A, 0x20, 0x02, 0x03, 0x00, 0x01, 0x14]
        );
    }

    #[test]
    fn when_led_is_off_it_should_match_sdl_set_home_led_zero() {
        assert_eq!(
            led_packet(Sequence(7), setting(LedMode::Off, 0)),
            vec![0x0A, 0x20, 0x07, 0x03, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn when_powering_on_it_should_match_sdl() {
        assert_eq!(
            power_on_packet(Sequence(1)),
            vec![0x05, 0x20, 0x01, 0x01, 0x00]
        );
    }

    #[test]
    fn when_mode_code_is_known_it_should_round_trip() {
        for mode in LedMode::ALL {
            assert_eq!(LedMode::try_from(mode_code(mode)), Ok(mode));
        }
    }

    #[test]
    fn when_modes_are_keyed_it_should_give_each_a_distinct_key() {
        let mut keys: Vec<_> = LedMode::ALL.into_iter().map(mode_key).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), LedMode::ALL.len());
    }

    #[test]
    fn when_mode_code_is_unknown_it_should_reject_it() {
        assert_eq!(LedMode::try_from(0x05), Err(0x05));
    }

    #[test]
    fn when_command_code_is_unknown_it_should_keep_the_code() {
        assert_eq!(command(0x7F), Command::Unknown(0x7F));
        assert_eq!(command(0x0A), Command::Led);
    }

    #[test]
    fn when_bytes_are_shorter_than_a_header_it_should_not_be_a_packet() {
        assert_eq!(packet(&[0x0A, 0x20, 0x01]), None);
    }

    #[test]
    fn when_packet_is_led_it_should_read_the_setting() {
        let read = packet(&[0x0A, 0x20, 0x03, 0x03, 0x00, 0x04, 0x1E]).unwrap();
        assert_eq!(read.command, Command::Led);
        assert_eq!(read.sequence, Sequence(3));
        assert_eq!(read.led, Some(setting(LedMode::BlinkSlow, 30)));
    }

    #[test]
    fn when_packet_is_not_led_it_should_have_no_setting() {
        assert_eq!(packet(&[0x03, 0x20, 0x01, 0x00]).unwrap().led, None);
    }

    #[test]
    fn when_summarising_led_packet_it_should_name_mode_and_brightness() {
        assert_eq!(
            summary(&[0x0A, 0x20, 0x03, 0x03, 0x00, 0x04, 0x1E]),
            "LED: Slow blink, brightness 30 [seq 3] 0A 20 03 03 00 04 1E"
        );
    }

    #[test]
    fn when_summarising_short_bytes_it_should_say_so() {
        assert_eq!(summary(&[0x0A]), "short packet (1 bytes, need 4)");
    }

    #[test]
    fn when_summarising_unknown_command_it_should_still_show_bytes() {
        assert_eq!(
            summary(&[0x7F, 0x00, 0x09, 0x00]),
            "unknown [seq 9] 7F 00 09 00"
        );
    }
}
