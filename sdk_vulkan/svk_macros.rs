// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// Provided by VK_VERSION_1_0
pub const fn VK_API_VERSION_VARIANT(pack : u32) -> u32 {
    return pack >> 29;
}

// Provided by VK_VERSION_1_0
pub const fn VK_API_VERSION_MAJOR(pack : u32) -> u32 {
    return (pack >> 22) & 0x7F;
}

// Provided by VK_VERSION_1_0
pub const fn VK_API_VERSION_MINOR(pack : u32) -> u32 {
    return (pack >> 12) & 0xFF;
}

// Provided by VK_VERSION_1_0
pub const fn VK_API_VERSION_PATCH(pack : u32) -> u32 {
    return pack & 0xFFF;
}

// Provided by VK_VERSION_1_0
pub const fn VK_MAKE_API_VERSION(variant : u32, major : u32, minor : u32, patch : u32) -> u32 {
    return variant << 29u32 | major << 22u32 | minor << 12u32 | patch;
}