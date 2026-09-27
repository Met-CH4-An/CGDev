// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::panic::panic_any;
use std::sync::Arc;
use crate::wvk_error::WvkError;
use crate::wvk_library::WvkLibrary;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// VK_EXT_debug_utils
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct VkExtDebugUtils {
    pub(in crate::extensions) vk_cmd_begin_debug_utils_label_ext: svk::PFN_vkCmdBeginDebugUtilsLabelEXT,
    pub(in crate::extensions) vk_cmd_end_debug_utils_label_ext: svk::PFN_vkCmdEndDebugUtilsLabelEXT,
    pub(in crate::extensions) vk_cmd_insert_debug_utils_label_ext: svk::PFN_vkCmdInsertDebugUtilsLabelEXT,
    pub(in crate::extensions) vk_create_debug_utils_messenger_ext: svk::PFN_vkCreateDebugUtilsMessengerEXT,
    pub(in crate::extensions) vk_destroy_debug_utils_messenger_ext: svk::PFN_vkDestroyDebugUtilsMessengerEXT,
    pub(in crate::extensions) vk_queue_begin_debug_utils_label_ext: svk::PFN_vkQueueBeginDebugUtilsLabelEXT,
    pub(in crate::extensions) vk_queue_end_debug_utils_label_ext: svk::PFN_vkQueueEndDebugUtilsLabelEXT,
    pub(in crate::extensions) vk_queue_insert_debug_utils_label_ext: svk::PFN_vkQueueInsertDebugUtilsLabelEXT,
    pub(in crate::extensions) vk_set_debug_utils_object_name_ext: svk::PFN_vkSetDebugUtilsObjectNameEXT,
    pub(in crate::extensions) vk_set_begin_debug_object_tag_ext: svk::PFN_vkSetDebugUtilsObjectTagEXT,
    pub(in crate::extensions) vk_submit_debug_utils_message_ext: svk::PFN_vkSubmitDebugUtilsMessageEXT,
}

impl VkExtDebugUtils {
    pub const NAME: &'static str = "VK_EXT_debug_utils";
    pub const NAME_C: &'static std::ffi::CStr = c"VK_EXT_debug_utils";

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Result<Self, WvkError> {
        Ok(Self {
            vk_cmd_begin_debug_utils_label_ext: Self::vkCmdBeginDebugUtilsLabelEXTDummy,
            vk_cmd_end_debug_utils_label_ext: Self::vkCmdEndDebugUtilsLabelEXTDummy,
            vk_cmd_insert_debug_utils_label_ext: Self::vkCmdInsertDebugUtilsLabelEXTDummy,
            vk_create_debug_utils_messenger_ext: Self::vkCreateDebugUtilsMessengerEXTDummy,
            vk_destroy_debug_utils_messenger_ext: Self::vkDestroyDebugUtilsMessengerEXTDummy,
            vk_queue_begin_debug_utils_label_ext: Self::vkQueueBeginDebugUtilsLabelEXTDummy,
            vk_queue_end_debug_utils_label_ext: Self::vkQueueEndDebugUtilsLabelEXTDummy,
            vk_queue_insert_debug_utils_label_ext: Self::vkQueueInsertDebugUtilsLabelEXTDummy,
            vk_set_debug_utils_object_name_ext: Self::vkSetDebugUtilsObjectNameEXTDummy,
            vk_set_begin_debug_object_tag_ext: Self::vkSetDebugUtilsObjectTagEXTDummy,
            vk_submit_debug_utils_message_ext: Self::vkSubmitDebugUtilsMessageEXTDummy,
        })
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn createWithInstance(
        wvk_library: &Arc<WvkLibrary>,
        vk_instance: svk::VkInstance
    ) -> Result<Self, WvkError> {
        let mut self_ = Self::create()?;

        self_.vk_create_debug_utils_messenger_ext = wvk_library.wvkGetInstanceProcAddr::<svk::PFN_vkCreateDebugUtilsMessengerEXT>(vk_instance, c"vkCreateDebugUtilsMessengerEXT")?;
        self_.vk_destroy_debug_utils_messenger_ext = wvk_library.wvkGetInstanceProcAddr::<svk::PFN_vkDestroyDebugUtilsMessengerEXT>(vk_instance, c"vkDestroyDebugUtilsMessengerEXT")?;
        self_.vk_submit_debug_utils_message_ext = wvk_library.wvkGetInstanceProcAddr::<svk::PFN_vkSubmitDebugUtilsMessageEXT>(vk_instance, c"vkSubmitDebugUtilsMessageEXT")?;

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkCmdBeginDebugUtilsLabelEXT(
        &self,
        commandBuffer: svk::VkCommandBuffer,
        pLabelInfo: *const svk::VkDebugUtilsLabelEXT
    ) {
        unsafe {(self.vk_cmd_begin_debug_utils_label_ext)(commandBuffer, pLabelInfo)}
    }
    pub(crate) unsafe extern "system" fn vkCmdBeginDebugUtilsLabelEXTDummy(
        commandBuffer: svk::VkCommandBuffer,
        pLabelInfo: *const svk::VkDebugUtilsLabelEXT
    ) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkCmdEndDebugUtilsLabelEXT(
        &self,
        commandBuffer: svk::VkCommandBuffer
    ) {
        unsafe {(self.vk_cmd_end_debug_utils_label_ext)(commandBuffer)}
    }
    pub(crate) unsafe extern "system" fn vkCmdEndDebugUtilsLabelEXTDummy(
        commandBuffer: svk::VkCommandBuffer
    ) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkCmdInsertDebugUtilsLabelEXT(
        &self,
        commandBuffer: svk::VkCommandBuffer,
        pLabelInfo: *const svk::VkDebugUtilsLabelEXT,
    ) {
        unsafe {(self.vk_cmd_insert_debug_utils_label_ext)(commandBuffer, pLabelInfo)}
    }

    pub(crate) unsafe extern "system" fn vkCmdInsertDebugUtilsLabelEXTDummy(
        commandBuffer: svk::VkCommandBuffer,
        pLabelInfo: *const svk::VkDebugUtilsLabelEXT,
    ) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkCreateDebugUtilsMessengerEXT(
        &self,
        instance: svk::VkInstance,
        pCreateInfo: *const svk::VkDebugUtilsMessengerCreateInfoEXT,
        pAllocator: *const svk::VkAllocationCallbacks,
        pMessenger: *mut svk::VkDebugUtilsMessengerEXT,
    ) -> svk::VkResult {
        unsafe {
            (self.vk_create_debug_utils_messenger_ext)(
                instance,
                pCreateInfo,
                pAllocator,
                pMessenger,
            )
        }
    }

    pub(crate) unsafe extern "system" fn vkCreateDebugUtilsMessengerEXTDummy(
        instance: svk::VkInstance,
        pCreateInfo: *const svk::VkDebugUtilsMessengerCreateInfoEXT,
        pAllocator: *const svk::VkAllocationCallbacks,
        pMessenger: *mut svk::VkDebugUtilsMessengerEXT,
    ) -> svk::VkResult {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkDestroyDebugUtilsMessengerEXT(
        &self,
        instance: svk::VkInstance,
        messenger: svk::VkDebugUtilsMessengerEXT,
        pAllocator: *const svk::VkAllocationCallbacks,
    ) {
        unsafe {
            (self.vk_destroy_debug_utils_messenger_ext)(
                instance,
                messenger,
                pAllocator,
            )
        }
    }

    pub(crate) unsafe extern "system" fn vkDestroyDebugUtilsMessengerEXTDummy(
        instance: svk::VkInstance,
        messenger: svk::VkDebugUtilsMessengerEXT,
        pAllocator: *const svk::VkAllocationCallbacks,
    ) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkQueueBeginDebugUtilsLabelEXT(
        &self,
        queue: svk::VkQueue,
        pLabelInfo: *const svk::VkDebugUtilsLabelEXT,
    ) {
        unsafe {(self.vk_queue_begin_debug_utils_label_ext)(queue, pLabelInfo)}
    }

    pub(crate) unsafe extern "system" fn vkQueueBeginDebugUtilsLabelEXTDummy(
        queue: svk::VkQueue,
        pLabelInfo: *const svk::VkDebugUtilsLabelEXT,
    ) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkQueueEndDebugUtilsLabelEXT(
        &self,
        queue: svk::VkQueue,
    ) {
        unsafe {(self.vk_queue_end_debug_utils_label_ext)(queue)}
    }

    pub(crate) unsafe extern "system" fn vkQueueEndDebugUtilsLabelEXTDummy(
        queue: svk::VkQueue,
    ) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkQueueInsertDebugUtilsLabelEXT(
        &self,
        queue: svk::VkQueue,
        pLabelInfo: *const svk::VkDebugUtilsLabelEXT,
    ) {
        unsafe {(self.vk_queue_insert_debug_utils_label_ext)(queue, pLabelInfo)}
    }

    pub(crate) unsafe extern "system" fn vkQueueInsertDebugUtilsLabelEXTDummy(
        queue: svk::VkQueue,
        pLabelInfo: *const svk::VkDebugUtilsLabelEXT,
    ) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkSetDebugUtilsObjectNameEXT(
        &self,
        device: svk::VkDevice,
        pNameInfo: *const svk::VkDebugUtilsObjectNameInfoEXT,
    ) -> svk::VkResult {
        unsafe {
            (self.vk_set_debug_utils_object_name_ext)(
                device,
                pNameInfo,
            )
        }
    }

    pub(crate) unsafe extern "system" fn vkSetDebugUtilsObjectNameEXTDummy(
        device: svk::VkDevice,
        pNameInfo: *const svk::VkDebugUtilsObjectNameInfoEXT,
    ) -> svk::VkResult {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkSetDebugUtilsObjectTagEXT(
        &self,
        device: svk::VkDevice,
        pTagInfo: *const svk::VkDebugUtilsObjectTagInfoEXT,
    ) -> svk::VkResult {
        unsafe {
            (self.vk_set_begin_debug_object_tag_ext)(
                device,
                pTagInfo,
            )
        }
    }

    pub(crate) unsafe extern "system" fn vkSetDebugUtilsObjectTagEXTDummy(
        device: svk::VkDevice,
        pTagInfo: *const svk::VkDebugUtilsObjectTagInfoEXT,
    ) -> svk::VkResult {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    /// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkSubmitDebugUtilsMessageEXT(
        &self,
        instance: svk::VkInstance,
        messageSeverity: svk::VkDebugUtilsMessageSeverityFlagBitsEXT,
        messageTypes: svk::VkDebugUtilsMessageTypeFlagsEXT,
        pCallbackData: *const svk::VkDebugUtilsMessengerCallbackDataEXT,
    ) {
        unsafe {
            (self.vk_submit_debug_utils_message_ext)(
                instance,
                messageSeverity,
                messageTypes,
                pCallbackData,
            )
        }
    }

    pub(crate) unsafe extern "system" fn vkSubmitDebugUtilsMessageEXTDummy(
        instance: svk::VkInstance,
        messageSeverity: svk::VkDebugUtilsMessageSeverityFlagBitsEXT,
        messageTypes: svk::VkDebugUtilsMessageTypeFlagsEXT,
        pCallbackData: *const svk::VkDebugUtilsMessengerCallbackDataEXT,
    ) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

}