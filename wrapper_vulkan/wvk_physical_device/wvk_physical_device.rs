// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::sync::Arc;
use crate::wvk_error::WvkError;
use crate::wvk_instance::WvkInstance;
use crate::wvk_physical_device::WvkPhysicalDeviceBuilder;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkPhysicalDevice {
    pub(in crate::wvk_physical_device) builder: WvkPhysicalDeviceBuilder,
}

impl WvkPhysicalDevice {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_physical_device) fn create(builder: WvkPhysicalDeviceBuilder) -> Result<Arc<Self>, WvkError> {
        let self_ = Self {
            builder: builder
        };

        Ok(Arc::new(self_))
    }
}