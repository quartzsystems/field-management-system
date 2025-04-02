// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

pub struct PLCOutputs {
    pub field_stack_light_red: bool,
    pub field_stack_light_blue: bool,
    pub field_stack_light_amber: bool,
    pub field_stack_light_green: bool,
    pub red_one_software_estop: bool,
    pub red_two_software_estop: bool,
    pub red_three_software_estop: bool,
    pub blue_one_software_estop: bool,
    pub blue_two_software_estop: bool,
    pub blue_three_software_estop: bool,
    pub output_count: u8,
}

impl PLCOutputs {
    // Converts an array of PLC outputs to a PLCOutputs object.
    pub fn from_array(&mut self, outputs: [bool; 10]) -> &mut PLCOutputs {
        self.field_stack_light_red = outputs[0];
        self.field_stack_light_blue = outputs[1];
        self.field_stack_light_amber = outputs[2];
        self.field_stack_light_green = outputs[3];
        self.red_one_software_estop = outputs[4];
        self.red_two_software_estop = outputs[5];
        self.red_three_software_estop = outputs[6];
        self.blue_one_software_estop = outputs[7];
        self.blue_two_software_estop = outputs[8];
        self.blue_three_software_estop = outputs[9];

        return self;
    }

    // Converts the PLCOutputs object into an array of PLC outputs.
    pub fn to_array(&self) -> [bool; 10] {
        return [
            self.field_stack_light_red,
            self.field_stack_light_blue,
            self.field_stack_light_amber,
            self.field_stack_light_green,
            self.red_one_software_estop,
            self.red_two_software_estop,
            self.red_three_software_estop,
            self.blue_one_software_estop,
            self.blue_two_software_estop,
            self.blue_three_software_estop,
        ];
    }

    // Checks if two PLCOutputs objects are equal.
    pub fn equals(&self, plc_outputs: Self) -> bool {
        return self.field_stack_light_red == plc_outputs.field_stack_light_red
            && self.field_stack_light_blue == plc_outputs.field_stack_light_blue
            && self.field_stack_light_amber == plc_outputs.field_stack_light_amber
            && self.field_stack_light_green == plc_outputs.field_stack_light_green
            && self.red_one_software_estop == plc_outputs.red_one_software_estop
            && self.red_two_software_estop == plc_outputs.red_two_software_estop
            && self.red_three_software_estop == plc_outputs.red_three_software_estop
            && self.blue_one_software_estop == plc_outputs.blue_one_software_estop
            && self.blue_two_software_estop == plc_outputs.blue_two_software_estop
            && self.blue_three_software_estop == plc_outputs.blue_three_software_estop;
    }
}
