/// Guide-button LED patterns (xone `enum gip_led_mode`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedMode {
    Off,
    Steady,
    BlinkFast,
    Blink,
    BlinkSlow,
    FadeSlow,
    FadeFast,
}

impl LedMode {
    pub const ALL: [LedMode; 7] = [
        LedMode::Off,
        LedMode::Steady,
        LedMode::BlinkFast,
        LedMode::Blink,
        LedMode::BlinkSlow,
        LedMode::FadeSlow,
        LedMode::FadeFast,
    ];
}

/// LED brightness. Always within `0..=Brightness::MAX`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Brightness(u8);

impl Brightness {
    pub const OFF: Brightness = Brightness(0);
    /// What the drivers set on connect (xone `GIP_LED_BRIGHTNESS_DEFAULT`, SDL).
    pub const DEFAULT: Brightness = Brightness(20);
    /// Highest value the drivers send (xone `GIP_LED_BRIGHTNESS_MAX`, SDL `MAX_VALUE`).
    pub const MAX: Brightness = Brightness(50);

    /// Out-of-range values saturate at `MAX`.
    pub fn saturating(value: u8) -> Brightness {
        Brightness(value.min(Brightness::MAX.0))
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LedSetting {
    pub mode: LedMode,
    pub brightness: Brightness,
}

/// Packet sequence number. `NONE` (0) is never sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sequence(pub u8);

impl Sequence {
    pub const NONE: Sequence = Sequence(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Acknowledge,
    Announce,
    Status,
    Identify,
    Power,
    Authenticate,
    GuideButton,
    Rumble,
    Led,
    Input,
    Unknown(u8),
}

/// A packet read from (or about to be written to) the controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Packet {
    pub command: Command,
    pub sequence: Sequence,
    /// Present for well-formed LED packets only.
    pub led: Option<LedSetting>,
}
