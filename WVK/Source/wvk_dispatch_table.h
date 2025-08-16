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

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkVulkanProcedureInfo {
			const char* name;       ///< Название функции Vulkan, например "vkDestroySurfaceKHR"
			void** targetPtr;       ///< Указатель на переменную, куда будет сохранён загруженный адрес
			WvkVulkanProcedureInfo(const char* n, void** ptr)
				: name(n), targetPtr(ptr) {
			}
		}; // struct WvkVulkanProcedureInfo



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkDispatchTableCreateInfo {
			VkInstance vkInstance = VK_NULL_HANDLE;
			VkDevice vkDevice = VK_NULL_HANDLE;
		}; // class WvkDispatchTableCreateInfo



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
			inline VkResult wvkCreateCommandPool(const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) const noexcept;
			inline VkResult wvkResetCommandPool(VkCommandPool commandPool, VkCommandPoolResetFlags flags) const noexcept;
			inline void wvkDestroyCommandPool(VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) const noexcept;
			inline VkResult wvkAllocateCommandBuffers(const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) const noexcept;
			inline void wvkFreeCommandBuffers(VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) const noexcept;
						
			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1
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
			// [Category]: Debug
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateDebugUtilsMessengerEXT m_vkCreateDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			PFN_vkDestroyDebugUtilsMessengerEXT m_vkDestroyDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			PFN_vkSubmitDebugUtilsMessageEXT m_vkSubmitDebugUtilsMessageEXT = VK_NULL_HANDLE;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkDispatchTableCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*! \brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;
			
		// cpp
		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*! \brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(const WvkDispatchTableCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*! \brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*! \brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(std::function<void* (const char*)> getProc, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) const noexcept;

		private:

			struct WvkDispatchTableImpl;
			std::unique_ptr<WvkDispatchTableImpl> m_dispatch_table_impl = nullptr;

			WvkDispatchTableCreateInfo m_create_info;
		}; // class WvkDispatchTable

	} // namespace wvk

} // namespace CGDev

#include "wvk_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_H
