// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// Unlike OpenGL, most tokens in Vulkan are actual typed enumerants in their own numeric namespaces.
// The "name" attribute is the C enum type name, and is pulled in from a type tag definition 
// above (slightly clunky, but retains the type / enum distinction).
// "type" attributes of "enum" or "bitmask" indicate that these values should be generated inside an appropriate definition.

// Vulkan hardcoded constants - not an enumerated type, part of the header boilerplate
pub const VK_MAX_EXTENSION_NAME_SIZE : u32 = 256;
pub const VK_MAX_DESCRIPTION_SIZE : u32 = 256;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE : u32 = 256;
pub const VK_UUID_SIZE : u32 = 16;