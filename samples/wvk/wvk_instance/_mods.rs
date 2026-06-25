// SPDX-License-Identifier: None
// Copyright (c) 2026 None

fn main() {
    let wvk_library_ = wvk::WvkLibrary::create().ok().unwrap();
    
    wvk::WvkInstance::builder(&wvk_library_);
}