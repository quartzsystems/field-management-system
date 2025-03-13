// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use std::io::Error;

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
    pub ds_status: &'static DSStatus,
}

impl DriverStation {
    // Initializes a new driver station object.
    #[must_use]
    pub fn init(
        ap_status: &'static APStatus,
        fms_status: FMSStatus,
        robot_status: RobotStatus,
        ds_status: &'static DSStatus,
    ) -> Self {
        return Self {
            ap_status,
            fms_status,
            robot_status,
            ds_status,
        };
    }

    // Encodes the driver station control information into a packet.
    fn encode_control_packet(
        &mut self,
        alliance_station: AllianceStation,
    ) -> Result<Vec<u8>, Error> {
        let mut packet: Vec<u8> = vec![];

        // Packet number, stored big-endian in two bytes
        packet.push((self.ds_status.packet_count >> 8) & 0xff);
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

        // Unknown or unused
        packet.push(0x00);

        // Driver station number
        packet.push(alliance_station.to_ds_number());

        Ok(packet)
    }
}
