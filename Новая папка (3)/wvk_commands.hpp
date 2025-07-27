#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMANDS_HPP
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMANDS_HPP
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

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				// ~~~~~~~~~~~~~~~~
				//
				// ~~~~~~~~~~~~~~~~

				inline VkResult WvkGlobalCommands::vknEnumerateInstanceVersion(uint32_t* pApiVersion) const noexcept {
					return vkEnumerateInstanceVersion(pApiVersion);
				}
				inline VkResult WvkGlobalCommands::vknEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept {
					return m_vkEnumerateInstanceLayerProperties(pPropertyCount, pProperties);
				}
				inline VkResult WvkGlobalCommands::vknEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept {
					return m_vkEnumerateInstanceExtensionProperties(pLayerName, pPropertyCount, pProperties);
				}
				inline VkResult WvkGlobalCommands::vknCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) const noexcept {
					return m_vkCreateInstance(pCreateInfo, pAllocator, pInstance);
				}

				// ~~~~~~~~~~~~~~~~
				//
				// ~~~~~~~~~~~~~~~~

				inline PFN_vkVoidFunction WvkGlobalCommands::vknGetInstanceProcAddr(const char* pName) const noexcept {
					return m_vkGetInstanceProcAddr(m_create_info.wvk_instance->getVkInstance(), pName);
				}
				inline void WvkGlobalCommands::vknDestroyInstance(const VkAllocationCallbacks* pAllocator) const noexcept {
					m_vkDestroyInstance(m_create_info.wvk_instance->getVkInstance(), pAllocator);
				}

				// ~~~~~~~~~~~~~~~~
				//
				// ~~~~~~~~~~~~~~~~

				inline VkResult WvkGlobalCommands::vknEnumeratePhysicalDevices(uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices) const noexcept {
					return m_vkEnumeratePhysicalDevices(m_create_info.wvk_instance->getVkInstance(), pPhysicalDeviceCount, pPhysicalDevices);
				}
				inline void WvkGlobalCommands::vknGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties) const noexcept {
					m_vkGetPhysicalDeviceProperties(physicalDevice, pProperties);
				}
				inline void WvkGlobalCommands::vknGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties) const noexcept {
					m_vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
				}

				// ~~~~~~~~~~~~~~~~
				// 
				// ~~~~~~~~~~~~~~~~

				inline VkResult WvkGlobalCommands::vknEnumeratePhysicalDeviceGroups(uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties) const noexcept {
					return m_vkEnumeratePhysicalDeviceGroups(m_create_info.wvk_instance->getVkInstance(), pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties);
				}
				inline void WvkGlobalCommands::vknGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) const noexcept {
					m_vkGetPhysicalDeviceProperties2(physicalDevice, pProperties);
				}
				inline void WvkGlobalCommands::vknGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties) const noexcept {
					m_vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
				}

				// ~~~~~~~~~~~~~~~~
				//
				// ~~~~~~~~~~~~~~~~

				inline VkResult WvkGlobalCommands::vknCreateDevice(VkPhysicalDevice vk_physical_device, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice) const noexcept {
					return m_vkCreateDevice(vk_physical_device, pCreateInfo, pAllocator, pDevice);
				}
				inline void WvkGlobalCommands::vknDestroyDevice(VkDevice device, const VkAllocationCallbacks* pAllocator) const noexcept {
					m_vkDestroyDevice(device, pAllocator);
				}
				inline PFN_vkVoidFunction WvkGlobalCommands::vknGetDeviceProcAddr(VkDevice device, const char* pName) const noexcept {
					m_vkGetDeviceProcAddr(device, pName);
				}

				// ~~~~~~~~~~~~~~~~
				//
				// ~~~~~~~~~~~~~~~~

				inline VkResult WvkGlobalCommands::vknCreateCommandPool(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) const noexcept {
					return m_vkCreateCommandPool(device, pCreateInfo, pAllocator, pCommandPool);
				}
				inline void WvkGlobalCommands::vknDestroyCommandPool(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) const noexcept {
					m_vkDestroyCommandPool(device, commandPool, pAllocator);
				}
				inline VkResult WvkGlobalCommands::vknAllocateCommandBuffers(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) const noexcept {
					return m_vkAllocateCommandBuffers(device, pAllocateInfo, pCommandBuffers);
				}
				inline void WvkGlobalCommands::vknFreeCommandBuffers(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) const noexcept {
					m_vkFreeCommandBuffers(device, commandPool, commandBufferCount, pCommandBuffers);
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				template<typename Command>
				inline void WvkGlobalCommands::reinterpreted(Command& command, const std::string& name_command, const VkInstance& vk_instance) const noexcept {

					command = reinterpret_cast<Command>(m_vkGetInstanceProcAddr(vk_instance, name_command.c_str()));
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				// ~~~~~~~~~~~~~~~~
				// 
				// ~~~~~~~~~~~~~~~~

				inline void WvkGlobalCommands::vknCmdBeginRenderPass(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, VkSubpassContents contents) const noexcept {
					m_vkCmdBeginRenderPass(commandBuffer, pRenderPassBegin, contents);
				}
				inline void WvkGlobalCommands::vknCmdEndRenderPass(VkCommandBuffer commandBuffer) const noexcept {
					m_vkCmdEndRenderPass(commandBuffer);
				}

				// ~~~~~~~~~~~~~~~~
				// 
				// ~~~~~~~~~~~~~~~~

				inline void WvkGlobalCommands::vknCmdBeginRendering(VkCommandBuffer commandBuffer, const VkRenderingInfo* pRenderingInfo) const noexcept {
					m_vkCmdBeginRendering(commandBuffer, pRenderingInfo);
				}
				inline void WvkGlobalCommands::vknCmdEndRendering(VkCommandBuffer commandBuffer) const noexcept {
					m_vkCmdEndRendering(commandBuffer);
				}

				// ~~~~~~~~~~~~~~~~
				// VkSurface. 1.1
				// ~~~~~~~~~~~~~~~~

				inline VkResult WvkGlobalCommands::wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice, pSurfaceInfo, pSurfaceCapabilities);
				}

				inline VkResult WvkGlobalCommands::wvkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats) const noexcept {
					return vkGetPhysicalDeviceSurfaceFormats2KHR(physicalDevice, pSurfaceInfo, pSurfaceFormatCount, pSurfaceFormats);
				}
			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMANDS_HPP
