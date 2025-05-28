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
#include "Extensions/wvk_khr_get_physical_device_properties2_dt.hpp"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkDispatchTableCreateInfo {
			WvkInstancePtr wvk_instance = nullptr;
			WvkLoaderPtr wvk_loader = nullptr;
		}; // class WvkDispatchTableCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkDispatchTable : public GpuObject {

		public:

			// ~~~~~~~~~~~~~~~~
			// WvkQueueFamily 1.0
			// ~~~~~~~~~~~~~~~~
			//inline void wvkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pCount, VkQueueFamilyProperties* pProperties) const noexcept;

			// ~~~~~~~~~~~~~~~~
			// WvkQueueFamily 1.1
			// ~~~~~~~~~~~~~~~~
			//inline void wvkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pCount, VkQueueFamilyProperties2* pProperties) const noexcept;

		private:

			// ~~~~~~~~~~~~~~~~
			// 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceFeatures m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceProperties m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFormatProperties m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceImageFormatProperties m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceQueueFamilyProperties m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceMemoryProperties m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSparseImageFormatProperties m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// 1.1 or VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceFeatures2 m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceProperties2 m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFormatProperties2 m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceImageFormatProperties2 m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceQueueFamilyProperties2 m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceMemoryProperties2 m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkDispatchTable(void) noexcept {}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline ~WvkDispatchTable(void) noexcept {}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus create(const WvkDispatchTableCreateInfo& create_info) noexcept { 
				

				return WvkStatus(); }

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline void destroy(void) noexcept {}

			//inline void setUp(WvkInstanceDtPtr dt) {
			//	std::vector<WvkVulkanProcedureInfo> _procedures = {
			//			{ "vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2) }
			//	};
			//	dt->loadInto(_procedures);
			//}

			//inline void setUp(Extensions::WvkKhrGetPhysicalDeviceProperties2DTPtr dt) {
			//	std::vector<WvkVulkanProcedureInfo> _procedures = {
			//			{ "vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2) }						
			//	};
			//	dt->loadInto(_procedures);
			//}

		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
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
