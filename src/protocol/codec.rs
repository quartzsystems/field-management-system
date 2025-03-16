// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

pub trait Codec {
    // Encoder for outgoing packets.
    fn encode(&self) -> Vec<u8>;

    // Decoder for incoming packets.
    fn decode(buffer: &[u8]) -> Result<Self>;
}
