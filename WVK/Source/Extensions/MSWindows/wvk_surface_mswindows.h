#ifndef CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS__WVK_SURFACE_MSWINDOWS_H
#define CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS__WVK_SURFACE_MSWINDOWS_H
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
#include "_mswindows.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
#include "../../wvk_base_object.h"
#include "../wvk_surface.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			namespace mswindows {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct WvkSurfaceMSWindowsCreateInfo : public WvkSurfacePlatformCreateInfo {
					HINSTANCE hInstance = NULL;
					HWND hWnd = NULL;
					WvkKhrWin32SurfaceDispatchTablePtr wvk_khr_win32_surface_dispatch_table = nullptr;
				}; // struct WvkSurfaceMSWindowsCreateInfo

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class WvkSurfaceMSWindows : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkSurfaceMSWindows(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~WvkSurfaceMSWindows(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* ������ Vulkan surface ��� Win32-��������� �� ������ ���������� ��������� `WvkSurfaceMSWindowsCreateInfo`.
					*
					* @details
					* ����� ��������� ���������� ��������� ��������, ��� ���������� ������ ��������� ��������� �������� ������� ������.
					* � ������ �������� ��������� ������ Win32-surface � ������� `vkCreateWin32SurfaceKHR`. ��� ������ � ���������� ��������� �������.
					*
					* ��������� � �������� ���� ������ ��������, ����� ������������� ������������ ��������� ����� ������� Vulkan API.
					*
					* @param[in] create_info
					* * ��������� `WvkSurfaceMSWindowsCreateInfo`, ���������� ����������� ���� Win32 � dispatch-�������.
					*
					* @return WvkStatus
					* * `OK`, ���� surface ������ �������.
					* * `FAIL` � ��������������� ���������� � ������ ������ ��������� ��� ��������.
					*
					* @code
					* WvkSurfaceMSWindows surface;
					* WvkSurfaceMSWindowsCreateInfo info = { ... };
					* WvkStatus status = surface.create(info);
					* if (!status) {
					*     log->error("Failed to create Win32 surface: {}", status.message());
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus create(const WvkSurfaceMSWindowsCreateInfo& create_info) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void destroy(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void requestVkSurface(VkSurfaceKHR& vk_surface_khr) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* ��������� ������������ ��������� `WvkSurfaceMSWindowsCreateInfo` ����� ��������� Vulkan surface.
					*
					* @details
					* ����� ��������� ��������� ������������ ����� ��������� `m_create_info`. �����������,
					* ��� ��������� `hInstance`, `hWnd` � `wvk_khr_win32_surface_dispatch_table` �� ����� `NULL` ��� `nullptr`.
					* ��� ����������� ������ ���������� ������ � ����������, ����� ������ ���� �����������.
					*
					* @return WvkStatus
					* * ��� `OK`, ���� ��� ������������ ���� ������ ���������.
					* * ��� `FAIL` � ��������� � ��������� ����������� ���� � ������ ������.
					*
					* @code
					* WvkStatus status = surface.validationCreateInfo();
					* if (!status.isOk()) {
					*     log->error("Validation failed: {}", status.message());
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus validationCreateInfo(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* ������ Vulkan surface ��� Windows-��������� � �������������� ���������� VK_KHR_win32_surface.
					*
					* @details
					* ����� �������������� ��������� `VkWin32SurfaceCreateInfoKHR`, �������� � ������� �� `m_create_info`,
					* � �������� ������� `wvkCreateWin32SurfaceKHR` ����� ����� ����������� dispatch-�������.
					* ��� �������� ���������� ������ � ��������� ��������� ������.
					*
					* @return WvkStatus
					* * ��� `OK` ��� �������� �������� surface.
					* * ��� `FAIL` � ���������� �� ������, ���� `wvkCreateWin32SurfaceKHR` ���������� ������.
					*
					* @code
					* WvkSurfaceMSWindows surface;
					* WvkStatus status = surface.createVkSurface();
					* if (!status.isOk()) {
					*     log->error(status.message());
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus createVkSurface(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void reset(void) noexcept;

				private:

					WvkSurfaceMSWindowsCreateInfo m_create_info = {};
					VkSurfaceKHR m_vk_surface = VK_NULL_HANDLE;
				}; // class WvkSurfaceMSWindows

			} // namespace mswindows

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_surface_mswindows.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS__WVK_SURFACE_MSWINDOWS_H
