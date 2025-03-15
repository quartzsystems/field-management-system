// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use chrono::{DateTime, Local};

use std::{net::SocketAddr, time::SystemTime};

// The state of the robot.
#[derive(Clone, Copy, Debug)]
pub enum RobotState {
    Auto,
    Test,
    Teleop,
}

// Tournament level of the 
#[derive(Clone, Copy, Debug)]
pub enum TournamentLevel {
    Test = 0,
    Practice = 1,
    Qualification = 2,
    Playoff = 3,
}

// These are the statuses we extract from the field AP.
pub struct APStatus {
    pub linked: bool,
    pub signal: &'static str,
    pub quality: [&'static str],
}

// These are the statuses that the FMS sends to the DS.
pub struct FMSStatus {
    pub bypassed: bool,
    pub auto: bool,
    pub enabled: bool,
    pub estop: bool,
    pub tournament_level: TournamentLevel,
    pub match_number: u16,
    pub play_number: u8,
    pub current_date: DateTime<Local>,
    pub remaining_seconds: u16,
}

// These are the statuses that the robot sends to the FMS.
pub struct RobotStatus {
    pub enabled: bool,
    pub mode: RobotState,
    pub estop: bool,
    pub radio_ping: bool,
    pub rio_ping: bool,
    pub last_linked_time: u16,
    pub comms_active: bool,
    pub battery_voltage: u16,
    pub trip_time_ms: u16,
    pub brownout: bool,
    pub bandwidth: u16,
}

// These are the statuses that we extract from the DS.
pub struct DSStatus {
    pub linked: bool,
    pub missed_packed_count: u16,
    pub last_packet_time: SystemTime,
    pub packet_count: u8,
    pub ip_address: SocketAddr,
    pub missed_packet_offset: u16,
    pub computer_battery_percent: u16,
    pub computer_cpu_percent: u16,
    pub last_log: &'static str,
}
