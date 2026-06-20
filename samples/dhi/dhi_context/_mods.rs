// SPDX-License-Identifier: None
// Copyright (c) 2026 None

fn main() {
    if let Err(dhi_error) = dhi::DHIContext::new() {
        println!("Не удалось создать DHIContext: {}", dhi_error.getMessage());
    }

    let _ = std::io::stdin().read_line(&mut String::new());
}