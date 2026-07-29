// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// папка global
// folder global
#[path = "global/_mods.rs"]
pub(crate) mod global;

// папка instance
// folder instance
#[path = "instance/_mods.rs"]
pub(crate) mod instance;

// файл wvk_dispatch_table.rs
// file wvk_dispatch_table.rs
pub(crate) mod wvk_dispatch_table;
pub use wvk_dispatch_table::WvkDispatchTable;

// файл wvk_dispatch_table_builder.rs
// file wvk_dispatch_table_builder.rs
pub(crate) mod wvk_dispatch_table_builder;
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

mod sealed {
    pub trait Sealed {}
}

pub trait WvkDispatchTableLevel : sealed::Sealed {}

/*pub trait WvkDispatchTableGlobal : WvkDispatchTableLevel {}
pub trait WvkDispatchTableInstance : WvkDispatchTableGlobal {}
pub trait WvkDispatchTableDevice : WvkDispatchTableInstance {}*/

pub trait WvkDispatchTableGlobal : WvkDispatchTableLevel {
    fn a(){}
}
pub trait WvkDispatchTableInstance : WvkDispatchTableLevel {
    fn a(){}
}
pub trait WvkDispatchTableDevice : WvkDispatchTableLevel {
    fn a(){}
}