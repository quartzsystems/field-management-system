// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

pub struct PLCInputs {
    pub field_estop: u8,
    pub red_one_estop: bool,
    pub red_two_estop: bool,
    pub red_three_estop: bool,
    pub blue_one_estop: bool,
    pub blue_two_estop: bool,
    pub blue_three_estop: bool,
    pub input_count: u8,
}
