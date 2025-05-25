#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_HPP
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_HPP
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

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline VkResult WvkKhrGetSurfaceCapabilities2::vkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept {
				return m_vk_get_physical_device_surface_capabilities2_khr(physicalDevice, pSurfaceInfo, pSurfaceCapabilities);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline VkResult WvkKhrGetSurfaceCapabilities2::vkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats) const noexcept {
				return m_vk_get_physical_device_surface_formats2_khr(physicalDevice, pSurfaceInfo, pSurfaceFormatCount, pSurfaceFormats);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline const WvkKhrGetSurfaceCapabilities2CreateInfo& WvkKhrGetSurfaceCapabilities2::getCreateInfo(void) const noexcept {
				return m_create_info;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_HPP
