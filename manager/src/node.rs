//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use tokio::process;

pub struct Node {
    pub name: String,
    pub process: process::Child
}