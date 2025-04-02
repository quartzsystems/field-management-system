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
