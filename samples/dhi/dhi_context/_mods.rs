// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use {
    dhi::dhi::{ DHI_WVK_0_1_0_0, DHI_WVK_0_1_1_0, DHI_WVK_0_1_2_0, DHI_WVK_0_1_3_0, DHI_WVK_0_1_4_0 },
    dhi::dhi_context::DHIContext,
};

fn main() {
    if let Err(dhi_error) = DHIContext::<DHI_WVK_0_1_4_0>::s_create() {
        println!("Не удалось создать DHIContext: {}", dhi_error.getMessage());
    }

    let _ = std::io::stdin().read_line(&mut String::new());
}