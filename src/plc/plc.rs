// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use s7::client::Client;
use s7::field::Bool;
use s7::tcp::{self, Transport};
use s7::transport::Connection;

use crate::plc::plc_inputs::PLCInputs;
use crate::plc::plc_outputs::PLCOutputs;

pub struct PLC {
    pub ip_address: Ipv4Addr,
    pub s7_client: Client<Transport>,
    pub plc_inputs: PLCInputs,
    pub plc_outputs: PLCOutputs,
}

impl PLC {
    // Initializes a new PLC object.
    pub fn init(
        ip_address: Ipv4Addr,
        s7_client: Client<Transport>,
        plc_inputs: PLCInputs,
        plc_outputs: PLCOutputs,
    ) -> Self {
        return Self {
            ip_address,
            s7_client,
            plc_inputs,
            plc_outputs,
        };
    }

    // Connects to the PLC.
    pub fn connect(&mut self) {
        let mut options = tcp::Options::new(IpAddr::from(self.ip_address), 0, 1, Connection::PG);

        options.read_timeout = Duration::from_secs(2);
        options.write_timeout = Duration::from_secs(2);

        let transport = tcp::Transport::connect(options).unwrap();

        self.s7_client = Client::new(transport).unwrap();
    }

    // Main loop to read inputs and write outputs to the PLC.
    pub fn run(&mut self) {
        loop {
            self.connect();
        }
    }

    pub fn read_inputs(&mut self) {
        let data_block = 1;
        let offset = 1;

        let buffer = &mut vec![0u8; Bool::size() as usize];

        self.s7_client
            .ag_read(data_block, offset as i32, Bool::size(), buffer)
            .unwrap();
    }

    pub fn write_outputs(&mut self) {
        let data_block = 1;
        let offset = 1;

        let buffer = &mut vec![0u8; Bool::size() as usize];

        self.s7_client
            .ag_write(data_block, offset, Bool::size(), buffer)
            .unwrap();
    }

    // Sets the specific alliance station's stack light to be ready or not.
    pub fn set_alliance_station_stack_light(&mut self) {}

    // Sets the specific alliance station's software estop.
    pub fn set_alliance_station_estop(&mut self) {}

    // Sets the status of the field stack light.
    pub fn set_field_stack_light(&mut self, red: bool, blue: bool, amber: bool, green: bool) {
        self.plc_outputs.field_stack_light_red = red;
        self.plc_outputs.field_stack_light_blue = blue;
        self.plc_outputs.field_stack_light_amber = amber;
        self.plc_outputs.field_stack_light_green = green;
    }
}
