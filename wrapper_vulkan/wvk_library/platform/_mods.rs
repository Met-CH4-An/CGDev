// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(target_os = "windows")]
#[path = "windows/_mods.rs"]
pub(in crate::wvk_library::platform) mod windows;
pub(in crate::wvk_library) type WvkLibraryPlatform = windows::WvkLibraryWindows;