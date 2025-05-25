#ifndef CGDEV_WVK_SOURCE_MSWINDOWS__WVK_LOADER_MSWINDOWS_H
#define CGDEV_WVK_SOURCE_MSWINDOWS__WVK_LOADER_MSWINDOWS_H
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
#include "../wvk_base_object.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace mswindows {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkLoaderMSWindowsCreateInfo {
			}; // struct WvkLoaderMSWindowsCreateInfo
					




			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkLoaderMSWindows : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkLoaderMSWindows(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkLoaderMSWindows(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*   ��������� ���������� Vulkan � �������������� ������ VknLoaderMSWindows.
				*
				* @details
				*   ����� ��������� ����������� ������������� ���������� Vulkan:
				*   ��������� ��������� ��������, �������� ���������, ��������� `vulkan-1.dll`
				*   � ������������� ���� �������� �������������. ��������� ����� �������
				*   � �������� ������� `ALREADY_INITIALIZED`, ��� ��������� ��������.
				*
				* @param[in] create_info
				*   ��������� � ����������� ��������.
				*
				* @return
				*   ������ ��������:
				*   - `OK` ���� ������������� �������.
				*   - `ALREADY_INITIALIZED` ���� ������ ��� ��� ������.
				*   - `FAIL` ���� ��������� �� ������ ��� DLL �� ���������.
				*
				* @code
				*   CGDev::wvk::WvkLoaderMSWindows loader;
				*   WvkLoaderMSWindowsCreateInfo info = { /* ... * / };
				*   WvkStatus status = loader.create(info);
				*   if (!status) {
				*       log_error(status.message());
				*   }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkLoaderMSWindowsCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* ����������� ����� ������� vkGetInstanceProcAddr �� ����������� ���������� Vulkan (vulkan-1.dll).
				*
				* @param[out] vkGetInstanceProcAddr
				* ������ �� ����������, � ������� ����� ������� ��������� �� ������� vkGetInstanceProcAddr, ���� ��� �������.
				*
				* @return
				* WvkStatus � ����� ����������: OK, ���� ������� �������, ����� FAIL.
				*
				* @note
				* ����� ������� ����� ������ ���������� ���������, ��� ���������� Vulkan ��� ��������� � ���������� m_vulkan_dll ������������.
				*
				* @code
				* WvkLoaderMSWindows loader;
				* loader.loadLibrary(); // ���������������� ��������������� �������������
				*
				* PFN_vkGetInstanceProcAddr getProc = nullptr;
				* WvkStatus status = loader.requestVkGetInstanceProcAddr(getProc);
				*
				* if (status.isOk()) {
				*     // ����� ������������ getProc
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestVkGetInstanceProcAddr(PFN_vkGetInstanceProcAddr& vkGetInstanceProcAddr) const noexcept;

			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(const WvkLoaderMSWindowsCreateInfo& create_info) const noexcept;

			private:

				WvkLoaderMSWindowsCreateInfo			m_create_info = {};
				HMODULE									m_vulkan_dll = NULL;
			}; // class WvkLoaderMSWindows

		} // namespace mswindows

	} // namespace wvk

} // namespace CGDev

#include "wvk_loader_mswindows.hpp"

#endif // CGDEV_WVK_SOURCE_MSWINDOWS__WVK_LOADER_MSWINDOWS_H
