// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

pub struct PLCInputs {
    pub field_estop: bool,
    pub red_one_estop: bool,
    pub red_two_estop: bool,
    pub red_three_estop: bool,
    pub blue_one_estop: bool,
    pub blue_two_estop: bool,
    pub blue_three_estop: bool,
    pub input_count: u8,
}

impl PLCInputs {
    // Converts an array of PLC inputs to a PLCInputs object.
    pub fn from_array(&mut self, inputs: [bool; 7]) -> &mut PLCInputs {
        if inputs.len() > 7 {
            self.field_estop = !inputs[0];
            self.red_one_estop = inputs[1];
            self.red_two_estop = inputs[2];
            self.red_three_estop = inputs[3];
            self.blue_one_estop = inputs[4];
            self.blue_two_estop = inputs[5];
            self.blue_three_estop = inputs[6];
        } else {
            println!(
                "[ERROR]: Unable to read inputs from PLC. Recieved input length is not long enough."
            );
        }

        return self;
    }

    // Converts the PLCInputs object into an array of PLC inputs.
    pub fn to_array(&self) -> [bool; 7] {
        return [
            !self.field_estop,
            self.red_one_estop,
            self.red_two_estop,
            self.red_three_estop,
            self.blue_one_estop,
            self.blue_two_estop,
            self.blue_three_estop,
        ];
    }

    // Checks if two PLCInputs objects are equal.
    pub fn equals(&self, plc_inputs: Self) -> bool {
        return self.field_estop == plc_inputs.field_estop
            && self.red_one_estop == plc_inputs.red_one_estop
            && self.red_two_estop == plc_inputs.red_two_estop
            && self.red_three_estop == plc_inputs.red_three_estop
            && self.blue_one_estop == plc_inputs.blue_one_estop
            && self.blue_two_estop == plc_inputs.blue_two_estop
            && self.blue_three_estop == plc_inputs.blue_three_estop;
    }
}
