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
#include "Extensions/wvk_khr_get_physical_device_properties2_dispatch_table.hpp"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkDispatchTableCreateInfo {
			//WvkInstancePtr wvk_instance = nullptr;
			//WvkLoaderPtr wvk_loader = nullptr;
		}; // class WvkDispatchTableCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkDispatchTable : public GpuObject {

		protected:

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkDestroyInstance m_vkDestroyInstance = VK_NULL_HANDLE;

			PFN_vkEnumerateDeviceExtensionProperties m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			PFN_vkEnumerateDeviceLayerProperties m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;

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

		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*! \brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*! \brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(void) noexcept;

			void reset(void) noexcept;

		private:

			WvkDispatchTableCreateInfo m_create_info;
		}; // class WvkDispatchTable

	} // namespace wvk

} // namespace CGDev

//#include "wvk_instance_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_DISPATCH_TABLE_H
