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
			return m_vkEnumeratePhysicalDevices(m_create_info.vkInstance, pPhysicalDeviceCount, pPhysicalDevices);
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
			return m_vkEnumeratePhysicalDeviceGroupsKHR(m_create_info.vkInstance, pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties);
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
		inline VkResult WvkDispatchTable::wvkCreateCommandPool(const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) const noexcept {
			return m_vkCreateCommandPool(m_create_info.vkDevice, pCreateInfo, pAllocator, pCommandPool);
		}
		inline VkResult WvkDispatchTable::wvkResetCommandPool(VkCommandPool commandPool, VkCommandPoolResetFlags flags) const noexcept {
			return m_vkResetCommandPool(m_create_info.vkDevice, commandPool, flags);
		}
		inline void WvkDispatchTable::wvkDestroyCommandPool(VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) const noexcept {
			m_vkDestroyCommandPool(m_create_info.vkDevice, commandPool, pAllocator);
		}
		inline VkResult WvkDispatchTable::wvkAllocateCommandBuffers(const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) const noexcept {
			return m_vkAllocateCommandBuffers(m_create_info.vkDevice, pAllocateInfo, pCommandBuffers);
		}
		inline void WvkDispatchTable::wvkFreeCommandBuffers(VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) const noexcept {
			m_vkFreeCommandBuffers(m_create_info.vkDevice, commandPool, commandBufferCount, pCommandBuffers);
		}

		// ~~~~~~~~~~~~~~~~
		// [Version] 1.1 / VK_KHR_maintenance1
		// ~~~~~~~~~~~~~~~~
		inline void WvkDispatchTable::wvkTrimCommandPool(VkCommandPool commandPool, VkCommandPoolTrimFlags flags) const noexcept {
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11		
			m_vkTrimCommandPool(m_create_info.vkDevice, commandPool, flags);
#elif WVK_KHR_maintenance1 == WVK_ENABLE
			m_vkTrimCommandPoolKHR(m_create_info.vkDevice, commandPool, flags);
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
	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_HPP
