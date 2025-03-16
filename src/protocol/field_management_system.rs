// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

// These are the statuses that are sent from the FMS to the Driver Station.
pub struct FMSToDS {
    pub sequence_num: u16,
    pub comm_version: u8,
}

impl FMSToDS {
    pub fn init(sequence_num: u16, comm_version: u8) -> FMSToDS {
        FMSToDS {

        }
    }
}

impl Codec for FMSToDS {
    fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
    }
}
