// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS
	#include <windows.h>
	#include "vulkan/vulkan_win32.h"
#endif

namespace CGDev {

	namespace wvk {
		
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkVulkanProcedureInfo {
			const char* name;       ///< Название функции Vulkan, например "vkDestroySurfaceKHR"
			void** targetPtr;       ///< Указатель на переменную, куда будет сохранён загруженный адрес
			WvkVulkanProcedureInfo(const char* n, void** ptr)
				: name(n), targetPtr(ptr) {
			}
		}; // struct WvkVulkanProcedureInfo



		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkDispatchTableCreateInfo {
			WvkInstancePtr wvk_instance_ptr = nullptr;
			WvkLogicalDevicePtr wvk_logical_device_ptr = nullptr;
		}; // class WvkDispatchTableCreateInfo



		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkDispatchTable : public GpuObject {

		public:

			// =======================================
			// [Category]: Global
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept;
			inline VkResult wvkEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept;
			inline VkResult wvkCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) const noexcept;						

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumerateInstanceVersion(uint32_t* pApiVersion) const noexcept;

			// =======================================
			// [Category]: Physical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumeratePhysicalDevices(uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices) const noexcept;
			inline void wvkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures) const noexcept;
			inline void wvkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties) const noexcept;
			inline void wvkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties) const noexcept;
			inline VkResult wvkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties) const noexcept;
			inline void wvkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties) const noexcept;
			inline void wvkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties) const noexcept;
			inline void wvkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties) const noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1 / WVK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumeratePhysicalDeviceGroups(uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties) const noexcept;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~
			inline void wvkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures) const noexcept;
			inline void wvkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) const noexcept;
			inline void wvkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties) const noexcept;
			inline VkResult wvkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties) const noexcept;
			inline void wvkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties) const noexcept;
			inline void wvkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties) const noexcept;
			inline void wvkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties) const noexcept;

			// =======================================
			// [Category]: Logical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateDevice(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice) const noexcept;

			// =======================================
			// [Category]: CommandPool
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateCommandPool(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) const noexcept;
			inline VkResult wvkCreateCommandPool(const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) const noexcept;
			inline VkResult wvkResetCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags) const noexcept;
			inline VkResult wvkResetCommandPool(VkCommandPool commandPool, VkCommandPoolResetFlags flags) const noexcept;
			inline void wvkDestroyCommandPool(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) const noexcept;
			inline void wvkDestroyCommandPool(VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) const noexcept;
			inline VkResult wvkAllocateCommandBuffers(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) const noexcept;
			inline VkResult wvkAllocateCommandBuffers(const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) const noexcept;
			inline void wvkFreeCommandBuffers(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) const noexcept;
			inline void wvkFreeCommandBuffers(VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) const noexcept;
						
			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_maintenance1
			// ~~~~~~~~~~~~~~~~
			inline void wvkTrimCommandPool(VkCommandPool commandPool, VkCommandPoolTrimFlags flags) const noexcept;

			// =======================================
			// [Category]: CommandBuffer
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkResetCommandBuffer(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags) const noexcept;
			inline VkResult wvkBeginCommandBuffer(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo) const noexcept;
			inline VkResult wvkEndCommandBuffer(VkCommandBuffer commandBuffer) const noexcept;

			// =======================================
			// [Category]: Shader
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateShaderModule(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule) const noexcept;
			inline VkResult wvkCreateShaderModule(const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule) const noexcept;
			inline void wvkDestroyShaderModule(VkDevice device, VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator) const noexcept;			
			inline void wvkDestroyShaderModule(VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator) const noexcept;
			
			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_shader_object
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateShaders(VkDevice device, uint32_t createInfoCount, const VkShaderCreateInfoEXT* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkShaderEXT* pShaders) const noexcept;
			inline VkResult wvkCreateShaders(uint32_t createInfoCount, const VkShaderCreateInfoEXT* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkShaderEXT* pShaders) const noexcept;
			inline void wvkDestroyShader(VkDevice device, VkShaderEXT shader, const VkAllocationCallbacks* pAllocator) const noexcept;			
			inline void wvkDestroyShader(VkShaderEXT shader, const VkAllocationCallbacks* pAllocator) const noexcept;
			inline VkResult wvkGetShaderBinaryDataEXT(VkDevice device, VkShaderEXT shader, size_t* pDataSize, void* pData) const noexcept;
			inline VkResult wvkGetShaderBinaryDataEXT(VkShaderEXT shader, size_t* pDataSize, void* pData) const noexcept;
			
			// =======================================
			// [Category]: Image
			// =======================================
			
			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateImageView(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImageView* pView) const noexcept;
			inline void wvkCmdBeginRendering(VkCommandBuffer commandBuffer, const VkRenderingInfo* pRenderingInfo) const noexcept;
			inline void wvkCmdEndRendering(VkCommandBuffer commandBuffer) const noexcept;
			inline void wvkCmdPipelineBarrier(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers) const noexcept;
			inline void wvkGetDeviceQueue(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue) const noexcept;
			inline VkResult wvkQueueSubmit(VkQueue queue, uint32_t submitCount, const VkSubmitInfo* pSubmits, VkFence fence) const noexcept;
			// =======================================
			// [Category]: Fence
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateFence(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence) const noexcept;
			inline VkResult wvkWaitForFences(VkDevice device, uint32_t fenceCount, const VkFence* pFences, VkBool32 waitAll, uint64_t timeout) const noexcept;
			inline VkResult wvkResetFences(VkDevice device, uint32_t fenceCount, const VkFence* pFences) const noexcept;

			// =======================================
			// [Category]: Swapchain
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_KHR_swapchain
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateSwapchainKHR(VkDevice device, const VkSwapchainCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchain) const noexcept;
			inline VkResult wvkCreateSwapchainKHR(const VkSwapchainCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchain) const noexcept;
			inline void wvkDestroySwapchainKHR(VkDevice device, VkSwapchainKHR swapchain, const VkAllocationCallbacks* pAllocator) const noexcept;
			inline void wvkDestroySwapchainKHR(VkSwapchainKHR swapchain, const VkAllocationCallbacks* pAllocator) const noexcept;
			inline VkResult wvkGetSwapchainImagesKHR(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages) const noexcept;
			inline VkResult wvkGetSwapchainImagesKHR(VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages) const noexcept;
			inline VkResult wvkAcquireNextImageKHR(VkDevice device, VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex) const noexcept;
			inline VkResult wvkAcquireNextImageKHR(VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex) const noexcept;
			inline VkResult wvkQueuePresentKHR(VkQueue queue, const VkPresentInfoKHR* pPresentInfo) const noexcept;
			
			// =======================================
			// [Category]: Surface
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_KHR_surface
			// ~~~~~~~~~~~~~~~~
#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS
			inline VkResult wvkCreateWin32SurfaceKHR(const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface) const noexcept;
#endif // WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS

			// =======================================
			// [Category]: Debug
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~
			inline void wvkCmdBeginDebugUtilsLabel(VkCommandBuffer commandBuffer, const VkDebugUtilsLabelEXT* pLabelInfo) const noexcept;
			inline void wvkCmdEndDebugUtilsLabel(VkCommandBuffer commandBuffer) const noexcept;
			inline void wvkCmdInsertDebugUtilsLabel(VkCommandBuffer commandBuffer, const VkDebugUtilsLabelEXT* pLabelInfo) const noexcept;
			inline VkResult wvkCreateDebugUtilsMessenger(VkInstance instance, const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pMessenger) const noexcept;
			inline void wvkDestroyDebugUtilsMessenger(VkInstance instance, VkDebugUtilsMessengerEXT messenger, const VkAllocationCallbacks* pAllocator) const noexcept;
			inline void wvkQueueBeginDebugUtilsLabel(VkQueue queue, const VkDebugUtilsLabelEXT* pLabelInfo) const noexcept;
			inline void wvkQueueEndDebugUtilsLabel(VkQueue queue) const noexcept;
			inline void wvkQueueInsertDebugUtilsLabel(VkQueue queue, const VkDebugUtilsLabelEXT* pLabelInfo) const noexcept;
			inline VkResult wvkSetDebugUtilsObjectName(VkDevice device, const VkDebugUtilsObjectNameInfoEXT* pNameInfo) const noexcept;
			inline VkResult wvkSetDebugUtilsObjectTag(VkDevice device, const VkDebugUtilsObjectTagInfoEXT* pTagInfo) const noexcept;
			inline void wvkSubmitDebugUtilsMessage(VkInstance instance, VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData) const noexcept;
		
		protected:

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			//PFN_vkDestroyInstance m_vkDestroyInstance = VK_NULL_HANDLE;

			//PFN_vkEnumerateDeviceExtensionProperties m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			//PFN_vkEnumerateDeviceLayerProperties m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;

			PFN_vkGetInstanceProcAddr m_vkGetInstanceProcAddr = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Global
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateInstance m_vkCreateInstance = VK_NULL_HANDLE;
			PFN_vkEnumerateInstanceExtensionProperties m_vkEnumerateInstanceExtensionProperties = VK_NULL_HANDLE;
			PFN_vkEnumerateInstanceLayerProperties m_vkEnumerateInstanceLayerProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~
			PFN_vkEnumerateInstanceVersion m_vkEnumerateInstanceVersion = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Physical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkEnumeratePhysicalDevices m_vkEnumeratePhysicalDevices = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFeatures m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceProperties m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFormatProperties m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceImageFormatProperties m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceQueueFamilyProperties m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceMemoryProperties m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSparseImageFormatProperties m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~
			PFN_vkEnumeratePhysicalDeviceGroups m_vkEnumeratePhysicalDeviceGroups = VK_NULL_HANDLE;
				PFN_vkEnumeratePhysicalDeviceGroupsKHR m_vkEnumeratePhysicalDeviceGroupsKHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceFeatures2 m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceProperties2 m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFormatProperties2 m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceImageFormatProperties2 m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceQueueFamilyProperties2 m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceMemoryProperties2 m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceFeatures2KHR m_vkGetPhysicalDeviceFeatures2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceProperties2KHR m_vkGetPhysicalDeviceProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceFormatProperties2KHR m_vkGetPhysicalDeviceFormatProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceImageFormatProperties2KHR m_vkGetPhysicalDeviceImageFormatProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR m_vkGetPhysicalDeviceQueueFamilyProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceMemoryProperties2KHR m_vkGetPhysicalDeviceMemoryProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_surface_capabilities2
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSurfaceFormats2KHR m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Logical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetDeviceProcAddr m_vkGetDeviceProcAddr = VK_NULL_HANDLE;
			PFN_vkCreateDevice m_vkCreateDevice = VK_NULL_HANDLE;

			// =======================================
			// [Category]: CommandPool
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateCommandPool m_vkCreateCommandPool = VK_NULL_HANDLE;
			PFN_vkResetCommandPool m_vkResetCommandPool = VK_NULL_HANDLE;
			PFN_vkDestroyCommandPool m_vkDestroyCommandPool = VK_NULL_HANDLE;
			PFN_vkAllocateCommandBuffers m_vkAllocateCommandBuffers = VK_NULL_HANDLE;
			PFN_vkFreeCommandBuffers m_vkFreeCommandBuffers = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_maintenance1
			// ~~~~~~~~~~~~~~~~
			PFN_vkTrimCommandPool m_vkTrimCommandPool = VK_NULL_HANDLE;
			PFN_vkTrimCommandPoolKHR m_vkTrimCommandPoolKHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: CommandBuffer
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkResetCommandBuffer m_vkResetCommandBuffer = VK_NULL_HANDLE;
			PFN_vkBeginCommandBuffer m_vkBeginCommandBuffer = VK_NULL_HANDLE;
			PFN_vkEndCommandBuffer m_vkEndCommandBuffer = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Shader
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateShaderModule m_vkCreateShaderModule = VK_NULL_HANDLE;
			PFN_vkDestroyShaderModule m_vkDestroyShaderModule = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_shader_object
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateShadersEXT m_vkCreateShadersEXT = VK_NULL_HANDLE;
			PFN_vkDestroyShaderEXT m_vkDestroyShaderEXT = VK_NULL_HANDLE;
			PFN_vkGetShaderBinaryDataEXT m_vkGetShaderBinaryDataEXT = VK_NULL_HANDLE;
			PFN_vkCreateImageView m_vkCreateImageView;
			PFN_vkCmdBeginRenderingKHR m_vkCmdBeginRenderingKHR;
			PFN_vkCmdEndRenderingKHR m_vkCmdEndRenderingKHR;
			PFN_vkCmdPipelineBarrier m_vkCmdPipelineBarrier;
			PFN_vkGetDeviceQueue m_vkGetDeviceQueue;
			PFN_vkQueueSubmit m_vkQueueSubmit;
			// =======================================
			// [Category]: Fence
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateFence m_vkCreateFence = VK_NULL_HANDLE;
			PFN_vkWaitForFences m_vkWaitForFences = VK_NULL_HANDLE;
			PFN_vkResetFences m_vkResetFences = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Swapchain
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_KHR_swapchain
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateSwapchainKHR m_vkCreateSwapchainKHR = VK_NULL_HANDLE;
			PFN_vkDestroySwapchainKHR m_vkDestroySwapchainKHR = VK_NULL_HANDLE;
			PFN_vkGetSwapchainImagesKHR m_vkGetSwapchainImagesKHR = VK_NULL_HANDLE;
			PFN_vkAcquireNextImageKHR m_vkAcquireNextImageKHR = VK_NULL_HANDLE;
			PFN_vkQueuePresentKHR m_vkQueuePresentKHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Surface
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_KHR_surface
			// ~~~~~~~~~~~~~~~~
#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS
			PFN_vkCreateWin32SurfaceKHR m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;
#endif // WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS

			// =======================================
			// [Category]: Debug
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~
			PFN_vkCmdBeginDebugUtilsLabelEXT m_vkCmdBeginDebugUtilsLabelEXT = VK_NULL_HANDLE;
			PFN_vkCmdEndDebugUtilsLabelEXT m_vkCmdEndDebugUtilsLabelEXT = VK_NULL_HANDLE;
			PFN_vkCmdInsertDebugUtilsLabelEXT m_vkCmdInsertDebugUtilsLabelEXT = VK_NULL_HANDLE;
			PFN_vkCreateDebugUtilsMessengerEXT m_vkCreateDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			PFN_vkDestroyDebugUtilsMessengerEXT m_vkDestroyDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			PFN_vkQueueBeginDebugUtilsLabelEXT m_vkQueueBeginDebugUtilsLabelEXT = VK_NULL_HANDLE;
			PFN_vkQueueEndDebugUtilsLabelEXT m_vkQueueEndDebugUtilsLabelEXT = VK_NULL_HANDLE;
			PFN_vkQueueInsertDebugUtilsLabelEXT m_vkQueueInsertDebugUtilsLabelEXT = VK_NULL_HANDLE;
			PFN_vkSetDebugUtilsObjectNameEXT m_vkSetDebugUtilsObjectNameEXT = VK_NULL_HANDLE;
			PFN_vkSetDebugUtilsObjectTagEXT m_vkSetDebugUtilsObjectTagEXT = VK_NULL_HANDLE;
			PFN_vkSubmitDebugUtilsMessageEXT m_vkSubmitDebugUtilsMessageEXT = VK_NULL_HANDLE;		

		public:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkDispatchTable(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkDispatchTable(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkDispatchTableCreateInfo& create_info) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;
			
		// cpp
		public:

		// hpp
		public:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkInstancePtr getWvkInstance(void) const noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkLogicalDevicePtr getWvkLogicalDevice(void) const noexcept;

		// hpp
		private:		

		// cpp
		private:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(const WvkDispatchTableCreateInfo& create_info) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(void) noexcept;

			friend class mswindows::WvkDispatchTableMSWindows;
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(std::function<void* (const char*)> getProc, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_proc_infos) const noexcept;

		// data
		private:

			WvkDispatchTableCreateInfo m_create_info;

			// ================================
			// [Platform]: MSWIndows
			// ================================
#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS
		// cpp
		private:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createMSWindows(void) noexcept;

		// data
		private:
			HMODULE m_vulkan_dll = NULL;
#endif // WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS

		}; // class WvkDispatchTable

	} // namespace wvk

} // namespace CGDev

#include "wvk_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_H
