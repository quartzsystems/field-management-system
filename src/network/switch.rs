// Copyright (C), 2025 Quartz Systems. Some rights reserved. This work is
// licensed under the terms of the MIT license which can be found in the
// root directory of this project.

use std::net::Ipv4Addr;

pub struct Switch {
  ip_address: Ipv4Addr,
  username: &'static str,
  password: &'static str,
}

impl Switch {
  #[must_use]
  pub const fn init() {
    
  }
}
