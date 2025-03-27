// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use std::net::Ipv4Addr;

pub struct PLC {
    ip_address: Ipv4Addr,
}

impl PLC {

}

pub struct PLCInputs {
    pub field_estop: u8,
    pub red_estop_1: bool,
    pub red_estop_2: bool,
    pub red_estop_3: bool,
    pub blue_estop_1: bool,
    pub blue_estop_2: bool,
    pub blue_estop_3: bool,
}

pub struct PLCOutputs {
    pub field_stack_light_red: bool,
    pub field_stack_light_blue: bool,
    pub field_stack_light_amber: bool,
    pub field_stack_light_green: bool,
}
