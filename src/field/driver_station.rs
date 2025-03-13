// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

pub struct DriverStation {
    // These are the statuses we extract from the field AP.
    pub ap_status: APStatus,

    // These are the statuses that the FMS sends to the DS.
    pub fms_status: FMSStatus,

    // These are the statuses that the robot sends to the FMS.
    pub robot_status: RobotStatus,

    // These are the statuses that we extract from the DS.
    pub ds_status: DSStatus,
}

impl DriverStation {
    #[must_use]
    pub fn init(
        ap_status: APStatus,
        fms_status: FMSStatus,
        robot_status: RobotStatus,
        ds_status: DSStatus,
    ) -> Self {
        return Self {
            ap_status,
            fms_status,
            robot_status,
            ds_status,
        };
    }

    fn encode_control_packet() {
        
    }
}