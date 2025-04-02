// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use std::net::Ipv4Addr;

use crate::plc::plc_inputs::PLCInputs;
use crate::plc::plc_outputs::PLCOutputs;
use crate::protocol::common::{AllianceColor, AllianceStation};

pub struct PLC {
    pub ip_address: Ipv4Addr,
    pub plc_inputs: PLCInputs,
    pub plc_outputs: PLCOutputs,
}

impl PLC {
    // Initializes a new PLC object.
    pub fn init(ip_address: Ipv4Addr, plc_inputs: PLCInputs, plc_outputs: PLCOutputs) -> Self {
        return Self {
            ip_address,
            plc_inputs,
            plc_outputs,
        };
    }

    // Sets the specific alliance station's stack light to be ready or not.
    pub fn set_alliance_station_stack_light(&mut self) {}

    // Sets the specific alliance station's software estop.
    pub fn set_alliance_station_estop(&mut self) {}

    // Sets the status of the field stack light.
    pub fn set_field_stack_light(
        &mut self,
        field_stack_light_red: bool,
        field_stack_light_blue: bool,
        field_stack_light_amber: bool,
        field_stack_light_green: bool,
    ) {
        self.plc_outputs.field_stack_light_red = field_stack_light_red;
        self.plc_outputs.field_stack_light_blue = field_stack_light_blue;
        self.plc_outputs.field_stack_light_amber = field_stack_light_amber;
        self.plc_outputs.field_stack_light_green = field_stack_light_green;
    }
}
