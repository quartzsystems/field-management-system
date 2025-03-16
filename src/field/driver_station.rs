// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use std::io::Error;
use std::time::SystemTime;

use chrono::{Datelike, Timelike};

use crate::field::alliance::*;
use crate::field::driver_station_status::*;

pub struct DriverStation {
    // These are the statuses we extract from the field AP.
    pub ap_status: &'static APStatus,

    // These are the statuses that the FMS sends to the DS.
    pub fms_status: FMSStatus,

    // These are the statuses that the robot sends to the FMS.
    pub robot_status: RobotStatus,

    // These are the statuses that we extract from the DS.
    pub ds_status: DSStatus,

    pub alliance_station: AllianceStation,
}

impl DriverStation {
    // Initializes a new driver station object.
    #[must_use]
    pub fn init(
        ap_status: &'static APStatus,
        fms_status: FMSStatus,
        robot_status: RobotStatus,
        ds_status: DSStatus,
        alliance_station: AllianceStation,
    ) -> Self {
        return Self {
            ap_status,
            fms_status,
            robot_status,
            ds_status,
            alliance_station,
        };
    }

    // Loops to read driver station packets and updates connection status.
    fn listen_for_packets(&mut self) {
        self.ds_status.linked = true;
        self.ds_status.last_packet_time = SystemTime::now();
    }

    // Encodes the driver station control information into a packet.
    fn encode_control_packet(&mut self) -> Result<(), Error> {
        let mut packet: Vec<u8> = vec![];

        // Packet number, stored big-endian in two bytes.
        packet.push((self.ds_status.packet_count >> 7 as u8) & 0xff);
        packet.push(self.ds_status.packet_count & 0xff);

        // Protocol version
        packet.push(0x00);

        // Robot status
        let mode: u8 = match self.robot_status.mode {
            RobotState::Auto => 2,
            RobotState::Test => 1,
            RobotState::Teleop => 0,
        };

        let control: u8 = ((self.robot_status.estop as u8) << 7)
            | ((self.robot_status.enabled as u8) << 2)
            | (mode & 0x03);

        packet.push(control);

        // Stays at 0x00, not used
        packet.push(0x00);

        // Represents the station the DS connects to
        packet.push(self.alliance_station.to_ds_number());

        // What level of competition this is
        packet.push(self.fms_status.tournament_level as u8);

        // Represents the number of current match
        packet.push(self.fms_status.match_number as u8);

        // Increments if there’s a replay
        packet.push(self.fms_status.play_number);

        // Date
        packet.push(self.fms_status.current_date.timestamp_subsec_micros() as u8);
        packet.push(self.fms_status.current_date.second() as u8);
        packet.push(self.fms_status.current_date.minute() as u8);
        packet.push(self.fms_status.current_date.hour() as u8);
        packet.push(self.fms_status.current_date.day() as u8);
        packet.push(self.fms_status.current_date.month() as u8);
        packet.push((self.fms_status.current_date.year() - 1900) as u8);

        // Time left in current mode
        packet.push(self.fms_status.remaining_seconds as u8);

        Ok(())
    }

    pub async fn send_control_packet(&mut self) -> Result<(), Error> {
        let packet = self
            .encode_control_packet()
            .expect("[ERROR] Unable to construct control packet.");

        Ok(())
    }
}
