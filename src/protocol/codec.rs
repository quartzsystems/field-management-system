// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use std::io::Error;

pub trait Encoder {
    // Encoder for outgoing packets.
    fn encode(&self) -> Result<Vec<u8>, Error>;
}

pub trait Decoder: Sized {
    // Decoder for incoming packets.
    fn decode(buffer: &[u8]) -> Result<Self, Error>;
}
