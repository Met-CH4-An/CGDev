#ifndef CGDEV_WVK_SOURCE_MSWINDOWS__WVK_DISPATCH_TABLE_MSWINDOWS_HPP
#define CGDEV_WVK_SOURCE_MSWINDOWS__WVK_DISPATCH_TABLE_MSWINDOWS_HPP
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
#include "../wvk_dispatch_table.h"
#include "../wvk_instance.h"

namespace CGDev {

	namespace wvk {

		namespace mswindows {

			// =======================================
			// [Category]: Surface
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_KHR_win32_surface
			// ~~~~~~~~~~~~~~~~
			inline VkResult WvkDispatchTableMSWindows::wvkCreateWin32SurfaceKHR(const WvkSurfaceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface) const noexcept {
				VkWin32SurfaceCreateInfoKHR _create_info = {
				.sType = VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
				.pNext = nullptr,
				.flags = 0,
				.hinstance = static_cast<HINSTANCE>(pCreateInfo->win32.hinstance),
				.hwnd = static_cast<HWND>(pCreateInfo->win32.hwnd),
				};
				return m_vkCreateWin32SurfaceKHR(m_create_info.wvk_dispatch_table_ptr->getWvkInstance()->getVkInstance(), &_create_info, pAllocator, pSurface);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline PFN_vkGetInstanceProcAddr WvkDispatchTableMSWindows::getVkGetInstanceProcAddr(void) const noexcept {
				return m_vkGetInstanceProcAddr;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline std::vector<WvkVulkanProcedureInfo> WvkDispatchTableMSWindows::getWvkVulkanProcedureInfos(void) noexcept {
				WvkStatus _status;

				std::vector<WvkVulkanProcedureInfo> _procedures;
				
#if WVK_KHR_win32_surface == WVK_ENABLE
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateWin32SurfaceKHR", reinterpret_cast<void**>(&m_vkCreateWin32SurfaceKHR)));
#endif				
				return _procedures;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace mswindows

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_MSWINDOWS__WVK_DISPATCH_TABLE_MSWINDOWS_HPP
