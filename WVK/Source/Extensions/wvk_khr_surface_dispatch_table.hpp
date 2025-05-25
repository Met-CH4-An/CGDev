#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_HPP
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_HPP
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
#include "../wvk_loader.h"
#include "../wvk_instance.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline void WvkKhrSurfaceDispatchTable::wvkDestroySurfaceKHR(VkSurfaceKHR surface, const VkAllocationCallbacks* pAllocator) const noexcept {
				m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkDestroySurfaceKHR, surface, pAllocator); }

			inline VkResult WvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported) const noexcept {
				return m_vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice, queueFamilyIndex, surface, pSupported); }

			inline VkResult WvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities) const noexcept {
				return m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice, surface, pSurfaceCapabilities); }

			inline VkResult WvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats) const noexcept {
				return m_vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice, surface, pSurfaceFormatCount, pSurfaceFormats); }

			inline VkResult WvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes) const noexcept {
				return m_vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice, surface, pPresentModeCount, pPresentModes); }

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_HPP
