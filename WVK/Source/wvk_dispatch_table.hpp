// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_HPP
#define CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_HPP
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"
#include "wvk_logical_device.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		// =======================================
		// [Category]: Global
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.0
		// ~~~~~~~~~~~~~~~~

		inline VkResult WvkDispatchTable::wvkEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept {
			return m_vkEnumerateInstanceLayerProperties(pPropertyCount, pProperties);
		}
		inline VkResult WvkDispatchTable::wvkEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept {
			return m_vkEnumerateInstanceExtensionProperties(pLayerName, pPropertyCount, pProperties);
		}
		inline VkResult WvkDispatchTable::wvkCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) const noexcept {
			return m_vkCreateInstance(pCreateInfo, pAllocator, pInstance);
		}

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.1
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkEnumerateInstanceVersion(uint32_t* pApiVersion) const noexcept {
			return m_vkEnumerateInstanceVersion(pApiVersion);
		}

		// =======================================
		// [Category]: Physical Device
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.0
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkEnumeratePhysicalDevices(uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices) const noexcept {
			return m_vkEnumeratePhysicalDevices(m_create_info.wvk_instance_ptr->getVkInstance(), pPhysicalDeviceCount, pPhysicalDevices);
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures) const noexcept {
			m_vkGetPhysicalDeviceFeatures(physicalDevice, pFeatures);
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties) const noexcept {
			m_vkGetPhysicalDeviceProperties(physicalDevice, pProperties);
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties) const noexcept {
			m_vkGetPhysicalDeviceFormatProperties(physicalDevice, format, pFormatProperties);
		}
		inline VkResult WvkDispatchTable::wvkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties) const noexcept {
			m_vkGetPhysicalDeviceImageFormatProperties(physicalDevice, format, type, tiling, usage, flags, pImageFormatProperties);
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties) const noexcept {
			m_vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties) const noexcept {
			m_vkGetPhysicalDeviceMemoryProperties(physicalDevice, pMemoryProperties);
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties) const noexcept {
			m_vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice, format, type, samples, usage, tiling, pPropertyCount, pProperties);
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// [Version] 1.1 / WVK_KHR_device_group_creation
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkEnumeratePhysicalDeviceGroups(uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 
			return m_vkEnumeratePhysicalDeviceGroups(m_create_info.vkInstance, pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties);
#elif WVK_KHR_device_group_creation == WVK_ENABLE 
			return m_vkEnumeratePhysicalDeviceGroupsKHR(m_create_info.wvk_instance_ptr->getVkInstance(), pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.1 / VK_KHR_get_physical_device_properties2
		// ~~~~~~~~~~~~~~~~
		inline void WvkDispatchTable::wvkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 
			m_vkGetPhysicalDeviceFeatures2(physicalDevice, pFeatures);
#elif WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			m_vkGetPhysicalDeviceFeatures2KHR(physicalDevice, pFeatures);
#endif
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 
			m_vkGetPhysicalDeviceProperties2(physicalDevice, pProperties);
#elif WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			m_vkGetPhysicalDeviceProperties2KHR(physicalDevice, pProperties);
#endif
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 
			m_vkGetPhysicalDeviceFormatProperties2(physicalDevice, format, pFormatProperties);
#elif WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			m_vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice, format, pFormatProperties);
#endif
		}
		inline VkResult WvkDispatchTable::wvkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 
			return m_vkGetPhysicalDeviceImageFormatProperties2(physicalDevice, pImageFormatInfo, pImageFormatProperties);
#elif WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			return m_vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice, pImageFormatInfo, pImageFormatProperties);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline void WvkDispatchTable::wvkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 
			m_vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
#elif WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			m_vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
#endif
		}

		inline void WvkDispatchTable::wvkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 
			m_vkGetPhysicalDeviceMemoryProperties2(physicalDevice, pMemoryProperties);
#elif WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			m_vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice, pMemoryProperties);
#endif
		}

		inline void WvkDispatchTable::wvkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 
			m_vkGetPhysicalDeviceSparseImageFormatProperties2(physicalDevice, pFormatInfo, pPropertyCount, pProperties);
#elif WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice, pFormatInfo, pPropertyCount, pProperties);
#endif
		}

		// =======================================
		// [Category]: Logical Device
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.0
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkCreateDevice(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice) const noexcept {
			return m_vkCreateDevice(physicalDevice, pCreateInfo, pAllocator, pDevice);
		}

		// =======================================
		// [Category]: CommandPool
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.0
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkCreateCommandPool(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) const noexcept {
			return m_vkCreateCommandPool(device, pCreateInfo, pAllocator, pCommandPool);
		}
		inline VkResult WvkDispatchTable::wvkCreateCommandPool(const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) const noexcept {
			return m_vkCreateCommandPool(m_create_info.wvk_logical_device_ptr->getVkDevice(), pCreateInfo, pAllocator, pCommandPool);
		}
		inline VkResult WvkDispatchTable::wvkResetCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags) const noexcept {
			return m_vkResetCommandPool(device, commandPool, flags);
		}
		inline VkResult WvkDispatchTable::wvkResetCommandPool(VkCommandPool commandPool, VkCommandPoolResetFlags flags) const noexcept {
			return m_vkResetCommandPool(m_create_info.wvk_logical_device_ptr->getVkDevice(), commandPool, flags);
		}
		inline void WvkDispatchTable::wvkDestroyCommandPool(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) const noexcept {
			m_vkDestroyCommandPool(device, commandPool, pAllocator);
		}
		inline void WvkDispatchTable::wvkDestroyCommandPool(VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) const noexcept {
			m_vkDestroyCommandPool(m_create_info.wvk_logical_device_ptr->getVkDevice(), commandPool, pAllocator);
		}
		inline VkResult WvkDispatchTable::wvkAllocateCommandBuffers(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) const noexcept {
			return m_vkAllocateCommandBuffers(device, pAllocateInfo, pCommandBuffers);
		}
		inline VkResult WvkDispatchTable::wvkAllocateCommandBuffers(const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) const noexcept {
			return m_vkAllocateCommandBuffers(m_create_info.wvk_logical_device_ptr->getVkDevice(), pAllocateInfo, pCommandBuffers);
		}
		inline void WvkDispatchTable::wvkFreeCommandBuffers(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) const noexcept {
			m_vkFreeCommandBuffers(device, commandPool, commandBufferCount, pCommandBuffers);
		}
		inline void WvkDispatchTable::wvkFreeCommandBuffers(VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) const noexcept {
			m_vkFreeCommandBuffers(m_create_info.wvk_logical_device_ptr->getVkDevice(), commandPool, commandBufferCount, pCommandBuffers);
		}

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.1 / VK_KHR_maintenance1
		// ~~~~~~~~~~~~~~~~
		inline void WvkDispatchTable::wvkTrimCommandPool(VkCommandPool commandPool, VkCommandPoolTrimFlags flags) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11		
			m_vkTrimCommandPool(m_create_info.wvk_logical_device_ptr->getVkDevice(), commandPool, flags);
#elif WVK_KHR_maintenance1 == WVK_ENABLE
			m_vkTrimCommandPoolKHR(m_create_info.wvk_logical_device_ptr->getVkDevice(), commandPool, flags);
#endif
		}

		// =======================================
		// [Category]: CommandBuffer
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.0
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkResetCommandBuffer(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags) const noexcept {
			return m_vkResetCommandBuffer(commandBuffer, flags);
		}
		inline VkResult WvkDispatchTable::wvkBeginCommandBuffer(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo) const noexcept {
			return m_vkBeginCommandBuffer(commandBuffer, pBeginInfo);
		}
		inline VkResult WvkDispatchTable::wvkEndCommandBuffer(VkCommandBuffer commandBuffer) const noexcept {
			return m_vkEndCommandBuffer(commandBuffer);
		}

		// =======================================
		// [Category]: Shader
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.0
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkCreateShaderModule(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule) const noexcept {
			return m_vkCreateShaderModule(device, pCreateInfo, pAllocator, pShaderModule);
		}
		inline VkResult WvkDispatchTable::wvkCreateShaderModule(const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule) const noexcept {
			return m_vkCreateShaderModule(m_create_info.wvk_logical_device_ptr->getVkDevice(), pCreateInfo, pAllocator, pShaderModule);
		}
		inline void WvkDispatchTable::wvkDestroyShaderModule(VkDevice device, VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator) const noexcept {
			m_vkDestroyShaderModule(device, shaderModule, pAllocator);
		}		
		inline void WvkDispatchTable::wvkDestroyShaderModule(VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator) const noexcept {
			m_vkDestroyShaderModule(m_create_info.wvk_logical_device_ptr->getVkDevice(), shaderModule, pAllocator);
		}

		// ~~~~~~~~~~~~~~~~
		// [Extension] VK_EXT_shader_object
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkCreateShaders(VkDevice device, uint32_t createInfoCount, const VkShaderCreateInfoEXT* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkShaderEXT* pShaders) const noexcept {
#if WVK_EXT_shader_object == WVK_ENABLE
			return m_vkCreateShadersEXT(device, createInfoCount, pCreateInfos, pAllocator, pShaders);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline VkResult WvkDispatchTable::wvkCreateShaders(uint32_t createInfoCount, const VkShaderCreateInfoEXT* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkShaderEXT* pShaders) const noexcept {
#if WVK_EXT_shader_object == WVK_ENABLE
			return m_vkCreateShadersEXT(m_create_info.wvk_logical_device_ptr->getVkDevice(), createInfoCount, pCreateInfos, pAllocator, pShaders);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline void WvkDispatchTable::wvkDestroyShader(VkDevice device, VkShaderEXT shader, const VkAllocationCallbacks* pAllocator) const noexcept {
#if WVK_EXT_shader_object == WVK_ENABLE
			m_vkDestroyShaderEXT(device, shader, pAllocator);
#endif
		}
		inline void WvkDispatchTable::wvkDestroyShader(VkShaderEXT shader, const VkAllocationCallbacks* pAllocator) const noexcept {
#if WVK_EXT_shader_object == WVK_ENABLE
			m_vkDestroyShaderEXT(m_create_info.wvk_logical_device_ptr->getVkDevice(), shader, pAllocator);
#endif
		}
		inline VkResult WvkDispatchTable::wvkGetShaderBinaryDataEXT(VkDevice device, VkShaderEXT shader, size_t* pDataSize, void* pData) const noexcept {
#if WVK_EXT_shader_object == WVK_ENABLE
			return m_vkGetShaderBinaryDataEXT(device, shader, pDataSize, pData);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline VkResult WvkDispatchTable::wvkGetShaderBinaryDataEXT(VkShaderEXT shader, size_t* pDataSize, void* pData) const noexcept {
#if WVK_EXT_shader_object == WVK_ENABLE
			return m_vkGetShaderBinaryDataEXT(m_create_info.wvk_logical_device_ptr->getVkDevice(), shader, pDataSize, pData);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}

		inline VkResult WvkDispatchTable::wvkCreateImageView(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImageView* pView) const noexcept {
			return m_vkCreateImageView(m_create_info.wvk_logical_device_ptr->getVkDevice(), pCreateInfo, pAllocator, pView);
		}
		inline void WvkDispatchTable::wvkCmdBeginRendering(VkCommandBuffer commandBuffer, const VkRenderingInfo* pRenderingInfo) const noexcept {
			m_vkCmdBeginRenderingKHR(commandBuffer, pRenderingInfo);
		}
		inline void WvkDispatchTable::wvkCmdEndRendering(VkCommandBuffer commandBuffer) const noexcept {
			m_vkCmdEndRenderingKHR(commandBuffer);
		}
		inline void WvkDispatchTable::wvkCmdPipelineBarrier(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, uint32_t memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, uint32_t bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, uint32_t imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers) const noexcept {
			m_vkCmdPipelineBarrier(commandBuffer, srcStageMask, dstStageMask, dependencyFlags, memoryBarrierCount, pMemoryBarriers, bufferMemoryBarrierCount, pBufferMemoryBarriers, imageMemoryBarrierCount, pImageMemoryBarriers);
		}
		inline void WvkDispatchTable::wvkGetDeviceQueue(VkDevice device, uint32_t queueFamilyIndex, uint32_t queueIndex, VkQueue* pQueue) const noexcept {
			m_vkGetDeviceQueue(m_create_info.wvk_logical_device_ptr->getVkDevice(), queueFamilyIndex, queueIndex, pQueue);
		}
		inline VkResult WvkDispatchTable::wvkQueueSubmit(VkQueue queue, uint32_t submitCount, const VkSubmitInfo* pSubmits, VkFence fence) const noexcept {
			return m_vkQueueSubmit(queue, submitCount, pSubmits, fence);
		}
		// =======================================
		// [Category]: Fence
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.0
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkCreateFence(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence) const noexcept {
			return m_vkCreateFence(m_create_info.wvk_logical_device_ptr->getVkDevice(), pCreateInfo, pAllocator, pFence);
		}
		inline VkResult WvkDispatchTable::wvkWaitForFences(VkDevice device, uint32_t fenceCount, const VkFence* pFences, VkBool32 waitAll, uint64_t timeout) const noexcept {
			return m_vkWaitForFences(m_create_info.wvk_logical_device_ptr->getVkDevice(), fenceCount, pFences, waitAll, timeout);
		}
		inline VkResult WvkDispatchTable::wvkResetFences(VkDevice device, uint32_t fenceCount, const VkFence* pFences) const noexcept {
			return m_vkResetFences(m_create_info.wvk_logical_device_ptr->getVkDevice(), fenceCount, pFences);
		}

		// =======================================
		// [Category]: Swapchain
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Extension] VK_KHR_swapchain
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkCreateSwapchainKHR(VkDevice device, const VkSwapchainCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchain) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return m_vkCreateSwapchainKHR(device, pCreateInfo, pAllocator, pSwapchain);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline VkResult WvkDispatchTable::wvkCreateSwapchainKHR(const VkSwapchainCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSwapchainKHR* pSwapchain) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return m_vkCreateSwapchainKHR(m_create_info.wvk_logical_device_ptr->getVkDevice(), pCreateInfo, pAllocator, pSwapchain);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline void WvkDispatchTable::wvkDestroySwapchainKHR(VkDevice device, VkSwapchainKHR swapchain, const VkAllocationCallbacks* pAllocator) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return m_vkDestroySwapchainKHR(device, swapchain, pAllocator);
#endif
		}
		inline void WvkDispatchTable::wvkDestroySwapchainKHR(VkSwapchainKHR swapchain, const VkAllocationCallbacks* pAllocator) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return wvkDestroySwapchainKHR(m_create_info.wvk_logical_device_ptr->getVkDevice(), swapchain, pAllocator);
#endif
		}
		inline VkResult WvkDispatchTable::wvkGetSwapchainImagesKHR(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return m_vkGetSwapchainImagesKHR(device, swapchain, pSwapchainImageCount, pSwapchainImages);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline VkResult WvkDispatchTable::wvkGetSwapchainImagesKHR(VkSwapchainKHR swapchain, uint32_t* pSwapchainImageCount, VkImage* pSwapchainImages) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return m_vkGetSwapchainImagesKHR(m_create_info.wvk_logical_device_ptr->getVkDevice(), swapchain, pSwapchainImageCount, pSwapchainImages);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline VkResult WvkDispatchTable::wvkAcquireNextImageKHR(VkDevice device, VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return m_vkAcquireNextImageKHR(device, swapchain, timeout, semaphore, fence, pImageIndex);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline VkResult WvkDispatchTable::wvkAcquireNextImageKHR(VkSwapchainKHR swapchain, uint64_t timeout, VkSemaphore semaphore, VkFence fence, uint32_t* pImageIndex) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return m_vkAcquireNextImageKHR(m_create_info.wvk_logical_device_ptr->getVkDevice(), swapchain, timeout, semaphore, fence, pImageIndex);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline VkResult WvkDispatchTable::wvkQueuePresentKHR(VkQueue queue, const VkPresentInfoKHR* pPresentInfo) const noexcept {
#if WVK_KHR_swapchain == WVK_ENABLE
			return m_vkQueuePresentKHR(queue, pPresentInfo);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}

		// =======================================
		// [Category]: Surface
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Extension] VK_KHR_surface
		// ~~~~~~~~~~~~~~~~
		inline VkResult WvkDispatchTable::wvkCreateWin32SurfaceKHR(const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface) const noexcept {
			return m_vkCreateWin32SurfaceKHR(m_create_info.wvk_instance_ptr->getVkInstance(), pCreateInfo, pAllocator, pSurface);
		}

		// =======================================
		// [Category]: Debug
		// =======================================

		// ~~~~~~~~~~~~~~~~
		// [Extension] VK_EXT_debug_utils
		// ~~~~~~~~~~~~~~~~
		inline void WvkDispatchTable::wvkCmdBeginDebugUtilsLabel(VkCommandBuffer commandBuffer, const VkDebugUtilsLabelEXT* pLabelInfo) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			m_vkCmdBeginDebugUtilsLabelEXT(commandBuffer, pLabelInfo);
#endif
		}
		inline void WvkDispatchTable::wvkCmdEndDebugUtilsLabel(VkCommandBuffer commandBuffer) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			m_vkCmdEndDebugUtilsLabelEXT(commandBuffer);
#endif
		}
		inline void WvkDispatchTable::wvkCmdInsertDebugUtilsLabel(VkCommandBuffer commandBuffer, const VkDebugUtilsLabelEXT* pLabelInfo) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			m_vkCmdInsertDebugUtilsLabelEXT(commandBuffer, pLabelInfo);
#endif
		}
		inline VkResult WvkDispatchTable::wvkCreateDebugUtilsMessenger(VkInstance instance, const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pMessenger) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			return m_vkCreateDebugUtilsMessengerEXT(instance, pCreateInfo, pAllocator, pMessenger);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline void WvkDispatchTable::wvkDestroyDebugUtilsMessenger(VkInstance instance, VkDebugUtilsMessengerEXT messenger, const VkAllocationCallbacks* pAllocator) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			m_vkDestroyDebugUtilsMessengerEXT(instance, messenger, pAllocator);
#endif
		}
		inline void WvkDispatchTable::wvkQueueBeginDebugUtilsLabel(VkQueue queue, const VkDebugUtilsLabelEXT* pLabelInfo) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			m_vkQueueBeginDebugUtilsLabelEXT(queue, pLabelInfo);
#endif
		}
		inline void WvkDispatchTable::wvkQueueEndDebugUtilsLabel(VkQueue queue) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			m_vkQueueEndDebugUtilsLabelEXT(queue);
#endif
		}
		inline void WvkDispatchTable::wvkQueueInsertDebugUtilsLabel(VkQueue queue, const VkDebugUtilsLabelEXT* pLabelInfo) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			m_vkQueueInsertDebugUtilsLabelEXT(queue, pLabelInfo);
#endif
		}
		inline VkResult WvkDispatchTable::wvkSetDebugUtilsObjectName(VkDevice device, const VkDebugUtilsObjectNameInfoEXT* pNameInfo) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			return m_vkSetDebugUtilsObjectNameEXT(device, pNameInfo);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline VkResult WvkDispatchTable::wvkSetDebugUtilsObjectTag(VkDevice device, const VkDebugUtilsObjectTagInfoEXT* pTagInfo) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			return m_vkSetDebugUtilsObjectTagEXT(device, pTagInfo);
#else
			return VK_ERROR_UNKNOWN;
#endif
		}
		inline void WvkDispatchTable::wvkSubmitDebugUtilsMessage(VkInstance instance, VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData) const noexcept {
#if WVK_EXT_debug_utils == WVK_ENABLE
			m_vkSubmitDebugUtilsMessageEXT(instance, messageSeverity, messageTypes, pCallbackData);
#endif
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const WvkInstancePtr WvkDispatchTable::getWvkInstance(void) const noexcept {
			return m_create_info.wvk_instance_ptr;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const WvkLogicalDevicePtr WvkDispatchTable::getWvkLogicalDevice(void) const noexcept {
			return m_create_info.wvk_logical_device_ptr;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_HPP
