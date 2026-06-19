// SPDX-License-Identifier: None
// Copyright (c) 2026 None

//pub use common::Error as WvkError;
pub type WvkError = common::Error::<WvkErrorType>;

////////////////////////////////////////////////////////////////
//
////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum WvkErrorType {
    /// ф ыв
    WVK_VK_RESULT(svk::VkResult),
    WVK_INPUT_PARAMETER_FAILED,
    WVK_RUNTIME_VULKAN_LIBRARY_LOAD_FAILED,
    WVK_RUNTIME_VULKAN_COMMAND_LOAD_FAILED,
}