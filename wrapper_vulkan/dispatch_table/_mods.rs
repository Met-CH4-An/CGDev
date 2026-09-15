// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[path = "global/_mods.rs"]
mod global;

#[path = "instance/_mods.rs"]
mod instance;

mod wvk_dispatch_table;
pub use wvk_dispatch_table::WvkDispatchTable;

mod wvk_dispatch_table_builder;
pub use wvk_dispatch_table_builder::WvkDispatchTableBuilder;

#[cfg(test)]
mod _tests;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// маркеры
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct WVK_DISPATCH_TABLE_GLOBAL;
pub struct WVK_DISPATCH_TABLE_INSTANCE;
pub struct WVK_DISPATCH_TABLE_DEVICE;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod private {
    pub trait WvkDispatchTableLevelPrivate {}
}

pub trait WvkDispatchTableLevel : private::WvkDispatchTableLevelPrivate {}

pub trait WvkDispatchTableGlobal : WvkDispatchTableLevel {}
pub trait WvkDispatchTableInstance : WvkDispatchTableLevel {}
pub trait WvkDispatchTableDevice : WvkDispatchTableLevel {}