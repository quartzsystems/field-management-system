// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

// The possible operating modes that the FMS can send to the DS.
pub enum Mode {
    TeleOp = 0x00,
    Test = 0x01,
    Autonomous = 0x02,
}

// The possible alliance colors.
pub enum AllianceColor {
    Blue,
    Red,
}

// The possible tournament levels of the competition.
pub enum TournamentLevel {
    Test = 0x00,
    Practice = 0x01,
    Qualification = 0x02,
    Playoff = 0x03,
}

// The possible alliance colors and alliance station number.
pub struct AllianceStation {
    pub alliance_color: AllianceColor,
    pub alliance_station: u8,
}

impl AllianceStation {
    pub fn init(alliance_color: AllianceColor, alliance_station: u8) -> Self {
        Self {
            alliance_color,
            alliance_station,
        }
    }

    pub fn to_ds_number(&self) -> u8 {
        let alliance_station: u8 = ((self.alliance_station - 1) % 3).try_into().unwrap();

        match self.alliance_station {
            AllianceColor::Blue => alliance_station + 3,
            AllianceColor::Red => alliance_station,
        }
    }
}
