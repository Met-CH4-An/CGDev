// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// зависимости
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use std::marker::PhantomData;

use wvk::wvk_library::{ WvkLibrary };
use wvk::wvk_instance::{ WvkInstance };

use crate::backend::dhi_wvk::dhi_wvk::DhiWvkBackend;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// бэкенд для Device Hardware Interface на основе Wrapper Vulkan (WVK)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct DhiWvkContext<TDhiWvkBackend>
where TDhiWvkBackend : DhiWvkBackend {
    pub(in crate::backend::dhi_wvk::dhi_wvk_context) phantom_data: PhantomData<TDhiWvkBackend>,
    pub(in crate::backend::dhi_wvk::dhi_wvk_context) wvk_library : WvkLibrary<TDhiWvkBackend::WvkVersion>,
    pub(in crate::backend::dhi_wvk::dhi_wvk_context) wvk_instance : WvkInstance<TDhiWvkBackend::WvkVersion>,
}