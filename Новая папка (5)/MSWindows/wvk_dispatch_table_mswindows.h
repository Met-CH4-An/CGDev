#ifndef CGDEV_WVK_SOURCE_MSWINDOWS__WVK_DISPATCH_TABLE_MSWINDOWS_H
#define CGDEV_WVK_SOURCE_MSWINDOWS__WVK_DISPATCH_TABLE_MSWINDOWS_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_mswindows.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../wvk_dispatch_table.h"

namespace CGDev {

	namespace wvk {

		namespace mswindows {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkDispatchTableMSWindowsCreateInfo {
				WvkDispatchTablePtr wvk_dispatch_table_ptr = nullptr;
			}; // struct WvkDispatchTableMSWindowsCreateInfo



			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkDispatchTableMSWindows : public GpuObject {

			public:

				// =======================================
				// [Category]: Surface
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_win32_surface
				// ~~~~~~~~~~~~~~~~
				inline VkResult wvkCreateWin32SurfaceKHR(const WvkSurfaceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface) const noexcept;

			public:

				// =======================================
				// [Category]: Surface
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_win32_surface
				// ~~~~~~~~~~~~~~~~
				PFN_vkCreateWin32SurfaceKHR m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;
				
			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkDispatchTableMSWindows(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkDispatchTableMSWindows(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkDispatchTableMSWindowsCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			// cpp
			public:

			// hpp
			private:

				friend class WvkDispatchTable;
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline PFN_vkGetInstanceProcAddr getVkGetInstanceProcAddr(void) const noexcept;

				friend class WvkDispatchTable;
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline std::vector<WvkVulkanProcedureInfo> getWvkVulkanProcedureInfos(void) noexcept;

			// cpp
			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(const WvkDispatchTableMSWindowsCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*! \brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus loadProcedure(void) noexcept;

			// data
			private:

				WvkDispatchTableMSWindowsCreateInfo m_create_info = {};
				HMODULE m_vulkan_dll = NULL;
				PFN_vkGetInstanceProcAddr m_vkGetInstanceProcAddr = VK_NULL_HANDLE;
			}; // class WvkDispatchTableMSWindows

		} // namespace mswindows

	} // namespace wvk

} // namespace CGDev

#include "wvk_dispatch_table_mswindows.hpp"

#endif // CGDEV_WVK_SOURCE_MSWINDOWS__WVK_DISPATCH_TABLE_MSWINDOWS_H
