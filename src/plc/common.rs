// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

// The possible states of a lamp on a stack light.
pub enum StackLight {
    Off = 0x00,
    On = 0x01,
}

// The possible states of the robot's connection.
pub enum RobotStatus {
    Disconnected = 0x00,
    Connected = 0x01,
}

// The possible states of if a team is bypassed or not.
pub enum BypassStatus {
    Enabled = 0x00,
    Bypassed = 0x01,
}
