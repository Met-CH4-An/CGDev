// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_dispatch_table.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"
#include "wvk_logical_device.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkDispatchTable::WvkDispatchTable(void) noexcept {}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkDispatchTable::~WvkDispatchTable(void) noexcept {}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::create(const WvkDispatchTableCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Проверка на повторную инициализацию
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_valid) {
				return _status.set(VknStatusCode::ALREADY_INITIALIZED);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Проверка валидности входной структуры
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = validationCreateInfo(create_info);
			if (!_status) {
				destroy();
				return _status.setFail("WvkInstanceDispatchTable::validationCreateInfo is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = create();
			if (!_status) {
				destroy();
				return _status.setFail("WvkInstanceDispatchTable::create is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Загрузка указателей на Vulkan-функции
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = loadProcedure();
			if (!_status) {
				destroy();
				return _status.setFail("WvkInstanceDispatchTable::loadProcedure is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkDispatchTable::destroy(void) noexcept {	
			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			//m_vkDestroyInstance = VK_NULL_HANDLE;

			//m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			//m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;

			m_vkGetInstanceProcAddr = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Global
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkCreateInstance = VK_NULL_HANDLE;
			m_vkEnumerateInstanceExtensionProperties = VK_NULL_HANDLE;
			m_vkEnumerateInstanceLayerProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkEnumerateInstanceVersion = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Instance
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkDestroyInstance = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Physical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkEnumeratePhysicalDevices = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkEnumeratePhysicalDeviceGroups = VK_NULL_HANDLE;
			m_vkEnumeratePhysicalDeviceGroupsKHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFeatures2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_surface_capabilities2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Logical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkGetDeviceProcAddr = VK_NULL_HANDLE;
			m_vkCreateDevice = VK_NULL_HANDLE;

			// =======================================
			// [Category]: CommandPool
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkCreateCommandPool = VK_NULL_HANDLE;
			m_vkResetCommandPool = VK_NULL_HANDLE;
			m_vkDestroyCommandPool = VK_NULL_HANDLE;
			m_vkAllocateCommandBuffers = VK_NULL_HANDLE;
			m_vkFreeCommandBuffers = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_maintenance1
			// ~~~~~~~~~~~~~~~~
			m_vkTrimCommandPool = VK_NULL_HANDLE;
				m_vkTrimCommandPoolKHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: CommandBuffer
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkResetCommandBuffer = VK_NULL_HANDLE;
			m_vkBeginCommandBuffer = VK_NULL_HANDLE;
			m_vkEndCommandBuffer = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Shader
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkCreateShaderModule = VK_NULL_HANDLE;
			m_vkDestroyShaderModule = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_shader_object
			// ~~~~~~~~~~~~~~~~
			m_vkCreateShadersEXT = VK_NULL_HANDLE;
			m_vkDestroyShaderEXT = VK_NULL_HANDLE;
			m_vkGetShaderBinaryDataEXT = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Fence
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkCreateFence = VK_NULL_HANDLE;
			m_vkWaitForFences = VK_NULL_HANDLE;
			m_vkResetFences = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Swapchain
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_KHR_swapchain
			// ~~~~~~~~~~~~~~~~
			m_vkCreateSwapchainKHR = VK_NULL_HANDLE;
			m_vkDestroySwapchainKHR = VK_NULL_HANDLE;
			m_vkGetSwapchainImagesKHR = VK_NULL_HANDLE;
			m_vkAcquireNextImageKHR = VK_NULL_HANDLE;
			m_vkQueuePresentKHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Surface
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_KHR_surface
			// ~~~~~~~~~~~~~~~~
#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS
			m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;
#endif // WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS

			// =======================================
			// [Category]: Debug
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~
			m_vkCmdBeginDebugUtilsLabelEXT = VK_NULL_HANDLE;
			m_vkCmdEndDebugUtilsLabelEXT = VK_NULL_HANDLE;
			m_vkCmdInsertDebugUtilsLabelEXT = VK_NULL_HANDLE;
			m_vkCreateDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			m_vkDestroyDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			m_vkQueueBeginDebugUtilsLabelEXT = VK_NULL_HANDLE;
			m_vkQueueEndDebugUtilsLabelEXT = VK_NULL_HANDLE;
			m_vkQueueInsertDebugUtilsLabelEXT = VK_NULL_HANDLE;
			m_vkSetDebugUtilsObjectNameEXT = VK_NULL_HANDLE;
			m_vkSetDebugUtilsObjectTagEXT = VK_NULL_HANDLE;
			m_vkSubmitDebugUtilsMessageEXT = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// очистка данных
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = false;
			m_create_info = {};
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::validationCreateInfo(const WvkDispatchTableCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::create(void) noexcept {
			WvkStatus _status;

			// ================================
			// [Platform]: MSWIndows
			// ================================
#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS	
			_status = createMSWindows();

			if(!_status) {
				return _status.setFail("WvkDispatchTable::createMSWindows is fail.");
			}
#endif // WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::loadProcedure(void) noexcept {
			WvkStatus _status;

			std::vector<WvkVulkanProcedureInfo> _procedures;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получаем процедуры, для которых не требуется VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			// =======================================
			// [Category]: Global
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumerateInstanceLayerProperties", reinterpret_cast<void**>(&m_vkEnumerateInstanceLayerProperties)));
			_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumerateInstanceExtensionProperties", reinterpret_cast<void**>(&m_vkEnumerateInstanceExtensionProperties)));
			_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateInstance", reinterpret_cast<void**>(&m_vkCreateInstance)));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumerateInstanceVersion", reinterpret_cast<void**>(&m_vkEnumerateInstanceVersion)));
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Загрузка
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = loadProcedure(
				[this](const char* name) {
					return m_vkGetInstanceProcAddr(VK_NULL_HANDLE, name); },
					_procedures);
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получаем процедуры, которые можно получить только при VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.wvk_instance_ptr != nullptr) {
				_procedures.clear();

				// =======================================
				// [Category]: Instance
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkDestroyInstance", reinterpret_cast<void**>(&m_vkDestroyInstance)));

				// =======================================
				// [Category]: Physical Device
				// =======================================

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumeratePhysicalDevices", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDevices)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFeatures", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceQueueFamilyProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceMemoryProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSparseImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties)));

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_device_group_creation
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (build::vulkan_version >= build::VulkanVersion::VERSION_11) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumeratePhysicalDeviceGroups", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroups)));
				if constexpr (build::isExtensionEnabled("VK_KHR_device_group_creation"))
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumeratePhysicalDeviceGroupsKHR", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroupsKHR)));
				}
				
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_get_physical_device_properties2
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (build::vulkan_version >= build::VulkanVersion::VERSION_11) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFeatures2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceMemoryProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSparseImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2)));
				}
				if constexpr (build::isExtensionEnabled("VK_KHR_get_physical_device_properties2")) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFeatures2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2KHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2KHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2KHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2KHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceMemoryProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2KHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceQueueFamilyProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2KHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSparseImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR)));
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_get_surface_capabilities2
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (build::vulkan_version >= build::VulkanVersion::VERSION_11) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR)));
				}
				if constexpr (build::isExtensionEnabled("VK_KHR_get_surface_capabilities2")) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR)));
				}

				// =======================================
				// [Category]: Logical Device
				// =======================================

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetDeviceProcAddr", reinterpret_cast<void**>(&m_vkGetDeviceProcAddr)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateDevice", reinterpret_cast<void**>(&m_vkCreateDevice)));

				// =======================================
				// [Category]: Swapchain
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_swapchain
				// ~~~~~~~~~~~~~~~~
				if constexpr (build::isExtensionEnabled("VK_KHR_swapchain")) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateSwapchainKHR", reinterpret_cast<void**>(&m_vkCreateSwapchainKHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkDestroySwapchainKHR", reinterpret_cast<void**>(&m_vkDestroySwapchainKHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetSwapchainImagesKHR", reinterpret_cast<void**>(&m_vkGetSwapchainImagesKHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkAcquireNextImageKHR", reinterpret_cast<void**>(&m_vkAcquireNextImageKHR)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkQueuePresentKHR", reinterpret_cast<void**>(&m_vkQueuePresentKHR)));
				}

				// =======================================
				// [Category]: Surface
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_surface
				// ~~~~~~~~~~~~~~~~
				if constexpr (build::isExtensionEnabled("VK_KHR_surface")) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateWin32SurfaceKHR", reinterpret_cast<void**>(&m_vkCreateWin32SurfaceKHR)));
				}
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCmdBeginRenderingKHR", reinterpret_cast<void**>(&m_vkCmdBeginRenderingKHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCmdEndRenderingKHR", reinterpret_cast<void**>(&m_vkCmdEndRenderingKHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCmdPipelineBarrier", reinterpret_cast<void**>(&m_vkCmdPipelineBarrier)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetDeviceQueue", reinterpret_cast<void**>(&m_vkGetDeviceQueue)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkQueueSubmit", reinterpret_cast<void**>(&m_vkQueueSubmit)));
				// =======================================
				// [Category]: Debug
				// =======================================

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Extension] VK_EXT_debug_utils
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (build::isExtensionEnabled("VK_EXT_debug_utils")) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCmdBeginDebugUtilsLabelEXT", reinterpret_cast<void**>(&m_vkCmdBeginDebugUtilsLabelEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCmdEndDebugUtilsLabelEXT", reinterpret_cast<void**>(&m_vkCmdEndDebugUtilsLabelEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCmdInsertDebugUtilsLabelEXT", reinterpret_cast<void**>(&m_vkCmdInsertDebugUtilsLabelEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkCreateDebugUtilsMessengerEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkDestroyDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkDestroyDebugUtilsMessengerEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkQueueBeginDebugUtilsLabelEXT", reinterpret_cast<void**>(&m_vkQueueBeginDebugUtilsLabelEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkQueueEndDebugUtilsLabelEXT", reinterpret_cast<void**>(&m_vkQueueEndDebugUtilsLabelEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkQueueInsertDebugUtilsLabelEXT", reinterpret_cast<void**>(&m_vkQueueInsertDebugUtilsLabelEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkSetDebugUtilsObjectNameEXT", reinterpret_cast<void**>(&m_vkSetDebugUtilsObjectNameEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkSetDebugUtilsObjectTagEXT", reinterpret_cast<void**>(&m_vkSetDebugUtilsObjectTagEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkSubmitDebugUtilsMessageEXT", reinterpret_cast<void**>(&m_vkSubmitDebugUtilsMessageEXT)));
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Загрузка
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = loadProcedure(
					[this](const char* name) {
						return m_vkGetInstanceProcAddr(m_create_info.wvk_instance_ptr->getVkInstance(), name); },
						_procedures);
			}	

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получаем процедуры, которые можно получить как при VkInstance, так и при VkDevice
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.wvk_instance_ptr != nullptr || m_create_info.wvk_logical_device_ptr != nullptr) {
				_procedures.clear();

				// =======================================
				// [Category]: CommandPool
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateCommandPool", reinterpret_cast<void**>(&m_vkCreateCommandPool)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkResetCommandPool", reinterpret_cast<void**>(&m_vkResetCommandPool)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkDestroyCommandPool", reinterpret_cast<void**>(&m_vkDestroyCommandPool)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkAllocateCommandBuffers", reinterpret_cast<void**>(&m_vkAllocateCommandBuffers)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkFreeCommandBuffers", reinterpret_cast<void**>(&m_vkFreeCommandBuffers)));

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_maintenance1
				// ~~~~~~~~~~~~~~~~
				if constexpr (build::vulkan_version >= build::VulkanVersion::VERSION_11) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkTrimCommandPool", reinterpret_cast<void**>(&m_vkTrimCommandPool)));
				}
				if constexpr (build::isExtensionEnabled("VK_KHR_maintenance1")) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkTrimCommandPoolKHR", reinterpret_cast<void**>(&m_vkTrimCommandPoolKHR)));
				}

				// =======================================
				// [Category]: CommandBuffer
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkResetCommandBuffer", reinterpret_cast<void**>(&m_vkResetCommandBuffer)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkBeginCommandBuffer", reinterpret_cast<void**>(&m_vkBeginCommandBuffer)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkEndCommandBuffer", reinterpret_cast<void**>(&m_vkEndCommandBuffer)));

				// =======================================
				// [Category]: Shader
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateShaderModule", reinterpret_cast<void**>(&m_vkCreateShaderModule)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkDestroyShaderModule", reinterpret_cast<void**>(&m_vkDestroyShaderModule)));
				
				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_EXT_shader_object
				// ~~~~~~~~~~~~~~~~
				if constexpr (build::isExtensionEnabled("VK_EXT_shader_object")) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateShadersEXT", reinterpret_cast<void**>(&m_vkCreateShadersEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkDestroyShaderEXT", reinterpret_cast<void**>(&m_vkDestroyShaderEXT)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetShaderBinaryDataEXT", reinterpret_cast<void**>(&m_vkGetShaderBinaryDataEXT)));
				}
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateImageView", reinterpret_cast<void**>(&m_vkCreateImageView)));
				
				// =======================================
				// [Category]: Fence
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateFence", reinterpret_cast<void**>(&m_vkCreateFence)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkWaitForFences", reinterpret_cast<void**>(&m_vkWaitForFences)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkResetFences", reinterpret_cast<void**>(&m_vkResetFences)));

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Загрузка адресов
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_create_info.wvk_logical_device_ptr != nullptr) {
					_status = loadProcedure(
						[this](const char* name) {
							return m_vkGetDeviceProcAddr(m_create_info.wvk_logical_device_ptr->getVkDevice(), name); },
							_procedures);
				}
				else {
					_status = loadProcedure(
						[this](const char* name) {
							return m_vkGetInstanceProcAddr(m_create_info.wvk_instance_ptr->getVkInstance(), name); },
							_procedures);
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::loadProcedure(std::function<void* (const char*)> getProc, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_proc_infos) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Перебор всех процедур и попытка загрузки каждой
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<std::string> _failed_procedures;
			for (const auto& it_0 : wvk_vulkan_proc_infos) {
				// Получаем адрес функции через vkGetInstanceProcAddr и записываем в targetPtr
				*it_0.targetPtr = getProc(it_0.name);

				// Если функция не найдена — запоминаем имя
				if (*it_0.targetPtr == nullptr) {
					_failed_procedures.emplace_back(it_0.name);
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Если есть ошибки, собираем сообщение и возвращаем FAIL
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (!_failed_procedures.empty()) {
				std::string _error_message = "\n\tVulkan procedures not found:";
				for (const auto& _name : _failed_procedures) {
					_error_message += "\n\t- " + _name;
				}
				return _status.set(VknStatusCode::FAIL, _error_message.c_str());
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		
		// ================================
		// [Platform]: MSWIndows
		// ================================
#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS	

		WvkStatus WvkDispatchTable::createMSWindows(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Загружаем Vulkan-библиотеку (vulkan-1.dll)
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vulkan_dll = LoadLibraryA("vulkan-1.dll");

			if (m_vulkan_dll == NULL) {
				return _status.setFail("LoadLibraryA is NULL.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// получаем адрес функции vkGetInstanceProcAddr из библиотеки
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkGetInstanceProcAddr = reinterpret_cast<PFN_vkGetInstanceProcAddr>(
				GetProcAddress(m_vulkan_dll, "vkGetInstanceProcAddr")
				);

			if (m_vkGetInstanceProcAddr == NULL) {
				return _status.setFail("GetProcAddress is NULL.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#endif // WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev