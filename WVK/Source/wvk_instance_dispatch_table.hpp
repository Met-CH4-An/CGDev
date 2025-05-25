#ifndef CGDEV_WVK_SOURCE__WVK_INSTANCE_DISPATCH_TABLE_HPP
#define CGDEV_WVK_SOURCE__WVK_INSTANCE_DISPATCH_TABLE_HPP
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
#include "wvk_loader.h"
#include "wvk_instance.h"

namespace CGDev {

	namespace wvk {

		// 1.0
		inline void WvkInstanceDispatchTable::wvkDestroyInstance(const VkAllocationCallbacks* pAllocator) const noexcept {
			m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkDestroyInstance, pAllocator); }

		inline VkResult WvkInstanceDispatchTable::wvkEnumeratePhysicalDevices(uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices) const noexcept {
			return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkEnumeratePhysicalDevices, pPhysicalDeviceCount, pPhysicalDevices); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures * pFeatures) const noexcept {
			m_vkGetPhysicalDeviceFeatures, physicalDevice, pFeatures; }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties * pFormatProperties) const noexcept {
			m_vkGetPhysicalDeviceFormatProperties, physicalDevice, format, pFormatProperties; }

		inline VkResult WvkInstanceDispatchTable::wvkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties * pImageFormatProperties) const noexcept {
			return m_vkGetPhysicalDeviceImageFormatProperties(physicalDevice, format, type, tiling, usage, flags, pImageFormatProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties * pMemoryProperties) const noexcept {
			m_vkGetPhysicalDeviceMemoryProperties(physicalDevice, pMemoryProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties * pProperties) const noexcept {
			m_vkGetPhysicalDeviceProperties(physicalDevice, pProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t * pCount, VkQueueFamilyProperties * pProperties) const noexcept {
			m_vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, pCount, pProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t * pCount, VkSparseImageFormatProperties * pProperties) const noexcept {
			m_vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice, format, type, samples, usage, tiling, pCount, pProperties); }

		inline VkResult WvkInstanceDispatchTable::wvkCreateDevice(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo * pCreateInfo, const VkAllocationCallbacks * pAllocator, VkDevice * pDevice) const noexcept {
			return m_vkCreateDevice(physicalDevice, pCreateInfo, pAllocator, pDevice); }

		inline VkResult WvkInstanceDispatchTable::wvkEnumerateDeviceExtensionProperties(VkPhysicalDevice physicalDevice, const char* pLayerName, uint32_t * pPropertyCount, VkExtensionProperties * pProperties) const noexcept {
			return m_vkEnumerateDeviceExtensionProperties(physicalDevice, pLayerName, pPropertyCount, pProperties); }

		inline VkResult WvkInstanceDispatchTable::wvkEnumerateDeviceLayerProperties(VkPhysicalDevice physicalDevice, uint32_t * pPropertyCount, VkLayerProperties * pProperties) const noexcept {
			return m_vkEnumerateDeviceLayerProperties(physicalDevice, pPropertyCount, pProperties); }

		inline PFN_vkVoidFunction WvkInstanceDispatchTable::wvkGetDeviceProcAddr(VkDevice device, const char* pName) const noexcept {
			return m_vkGetDeviceProcAddr(device, pName); }

		// Vulkan 1.1
		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures) const noexcept {
			m_vkGetPhysicalDeviceFeatures2(physicalDevice, pFeatures); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2 * pProperties) const noexcept {
			m_vkGetPhysicalDeviceProperties2(physicalDevice, pProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2 * pFormatProperties) const noexcept {
			m_vkGetPhysicalDeviceFormatProperties2(physicalDevice, format, pFormatProperties); }

		inline VkResult WvkInstanceDispatchTable::wvkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2 * pImageFormatInfo, VkImageFormatProperties2 * pImageFormatProperties) const noexcept {
			return m_vkGetPhysicalDeviceImageFormatProperties2(physicalDevice, pImageFormatInfo, pImageFormatProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2 * pMemoryProperties) const noexcept {
			m_vkGetPhysicalDeviceMemoryProperties2(physicalDevice, pMemoryProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t * pCount, VkQueueFamilyProperties2 * pProperties) const noexcept {
			m_vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice, pCount, pProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2 * pFormatInfo, uint32_t * pPropertyCount, VkSparseImageFormatProperties2 * pProperties) const noexcept {
			m_vkGetPhysicalDeviceSparseImageFormatProperties2(physicalDevice, pFormatInfo, pPropertyCount, pProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceExternalBufferProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalBufferInfo * pExternalBufferInfo, VkExternalBufferProperties * pExternalBufferProperties) const noexcept {
			m_vkGetPhysicalDeviceExternalBufferProperties(physicalDevice, pExternalBufferInfo, pExternalBufferProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceExternalFenceProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalFenceInfo * pExternalFenceInfo, VkExternalFenceProperties * pExternalFenceProperties) const noexcept {
			m_vkGetPhysicalDeviceExternalFenceProperties(physicalDevice, pExternalFenceInfo, pExternalFenceProperties); }

		inline void WvkInstanceDispatchTable::wvkGetPhysicalDeviceExternalSemaphoreProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalSemaphoreInfo * pExternalSemaphoreInfo, VkExternalSemaphoreProperties * pExternalSemaphoreProperties) const noexcept {
			m_vkGetPhysicalDeviceExternalSemaphoreProperties(physicalDevice, pExternalSemaphoreInfo, pExternalSemaphoreProperties); }

		// ~~~~~~~~~~~~~~~~
		// VkSurface 1.1
		// ~~~~~~~~~~~~~~~~

		inline VkResult WvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept {
			return m_vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice, pSurfaceInfo, pSurfaceCapabilities); }

		// Vulkan 1.2
		inline VkResult WvkInstanceDispatchTable::wvkGetPhysicalDeviceToolProperties(VkPhysicalDevice physicalDevice, uint32_t * pToolCount, VkPhysicalDeviceToolProperties * pToolProperties) const noexcept {
			return m_vkGetPhysicalDeviceToolProperties(physicalDevice, pToolCount, pToolProperties); }

		inline VkResult WvkInstanceDispatchTable::wvkGetPhysicalDeviceCooperativeMatrixPropertiesNV(VkPhysicalDevice physicalDevice, uint32_t * pPropertyCount, VkCooperativeMatrixPropertiesNV * pProperties) const noexcept {
			return m_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(physicalDevice, pPropertyCount, pProperties); }

		inline VkResult WvkInstanceDispatchTable::wvkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(VkPhysicalDevice physicalDevice, uint32_t * pCombinationCount, VkFramebufferMixedSamplesCombinationNV * pCombinations) const noexcept {
			return m_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(physicalDevice, pCombinationCount, pCombinations); }
	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_DISPATCH_TABLE_HPP