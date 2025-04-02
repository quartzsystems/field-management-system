// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use std::io::Error;

use byteorder::{BigEndian, WriteBytesExt};
use chrono::prelude::*;

use crate::protocol::codec::Encoder;
use crate::protocol::common::*;

// Contains control bits
pub struct Control {
    e_stop: u8,
    enabled: u8,
    mode: Mode,
}

// Current date and time.
pub struct Date {
    pub microseconds: u32,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

impl Date {
    pub fn init() -> Self {
        let local = Utc::now();

        let microseconds = local.naive_utc().and_utc().timestamp_subsec_micros();
        let second = local.second() as u8;
        let minute = local.minute() as u8;
        let hour = local.hour() as u8;
        let day = local.day() as u8;
        let month = local.month0() as u8;
        let year = (local.year() - 1900) as u8;

        return Self {
            microseconds,
            second,
            minute,
            hour,
            day,
            month,
            year,
        };
    }
}

// These are the statuses that are sent from the FMS to the Driver Station.
pub struct UDPControlPacket {
    // Packet number, stored big-endian in two bytes.
    pub sequence_num: u16,

    // Only 0x00 has been observed.
    pub comm_version: u8,

    // Contains control bits.
    pub control_byte: Control,

    // Stays at 0x00, not used.
    pub request_byte: u8,

    // Represents the station the DS connects to.
    pub alliance_station: AllianceStation,

    // What level of competition this is.
    pub tournament_level: TournamentLevel,

    // Represents the number of current match.
    pub match_number: u16,

    // Increments if there’s a replay
    pub play_number: u8,

    // Current date and time.
    pub date: Date,

    // Time left in current mode.
    pub remaining_time: u16,
}

impl UDPControlPacket {
    pub fn init(
        sequence_num: u16,
        control_byte: Control,
        alliance_station: AllianceStation,
        tournament_level: TournamentLevel,
        match_number: u16,
        play_number: u8,
        remaining_time: u16,
    ) -> Self {
        return Self {
            sequence_num,
            comm_version: 0x00,
            control_byte,
            request_byte: 0x00,
            alliance_station,
            tournament_level,
            match_number,
            play_number,
            date: Date::init(),
            remaining_time,
        };
    }
}

impl Encoder for UDPControlPacket {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buffer = Vec::new();

        // Packet number, stored big-endian in two bytes.
        buffer.write_u16::<BigEndian>(self.sequence_num).unwrap();

        // Only 0x00 has been observed.
        buffer.write_u8(self.comm_version).unwrap();

        // Contains control bits.
        let mode: u8 = match self.control_byte.mode {
            Mode::Autonomous => 2,
            Mode::Test => 1,
            Mode::TeleOp => 0,
        };
        let control: u8 = ((self.control_byte.e_stop as u8) << 7)
            | ((self.control_byte.enabled as u8) << 7)
            | (mode & 0b11);
        buffer.write_u8(control).unwrap();

        // Stays at 0x00, not used.
        buffer.write_u8(self.request_byte).unwrap();

        // Represents the station the DS connects to.
        buffer
            .write_u8(self.alliance_station.to_ds_number())
            .unwrap();

        // What level of competition this is.
        buffer.write_u8(self.tournament_level as u8).unwrap();

        // Represents the number of current match.
        buffer.write_u16::<BigEndian>(self.match_number).unwrap();

        // Increments if there’s a replay
        buffer.write_u8(self.play_number).unwrap();

        // Current date and time.
        buffer
            .write_u32::<BigEndian>(self.date.microseconds)
            .unwrap();
        buffer.write_u8(self.date.second as u8).unwrap();
        buffer.write_u8(self.date.minute as u8).unwrap();
        buffer.write_u8(self.date.hour as u8).unwrap();
        buffer.write_u8(self.date.day as u8).unwrap();
        buffer.write_u8(self.date.month as u8).unwrap();
        buffer.write_u8(self.date.year as u8).unwrap();

        // Time left in current mode.
        buffer.write_u16::<BigEndian>(self.remaining_time).unwrap();

        return Ok(buffer);
    }
}
