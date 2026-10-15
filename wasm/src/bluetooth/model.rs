/// Rumble motor strength as a percentage. Always within `0..=Strength::MAX`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Strength(u8);

impl Strength {
    pub const NONE: Strength = Strength(0);
    pub const MAX: Strength = Strength(100);

    /// Out-of-range values saturate at `MAX`.
    pub fn saturating(value: u8) -> Strength {
        Strength(value.min(Strength::MAX.0))
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rumble {
    pub left_trigger: Strength,
    pub right_trigger: Strength,
    pub low_frequency: Strength,
    pub high_frequency: Strength,
}

impl Rumble {
    pub const STILL: Rumble = Rumble {
        left_trigger: Strength::NONE,
        right_trigger: Strength::NONE,
        low_frequency: Strength::NONE,
        high_frequency: Strength::NONE,
    };
}

/// The controller reports battery charge as one of four coarse levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatteryLevel {
    Critical,
    Low,
    Medium,
    Full,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Battery {
    pub level: BatteryLevel,
    /// Powered over USB.
    pub charging: bool,
}
