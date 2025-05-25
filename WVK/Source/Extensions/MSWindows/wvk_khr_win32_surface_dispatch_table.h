#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_WIN32_SURFACE_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_WIN32_SURFACE_DISPATCH_TABLE_H
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
				struct WvkKhrWin32SurfaceDispatchTableCreateInfo {
					WvkLoaderPtr wvk_loader = nullptr;
					WvkInstancePtr wvk_instance = nullptr;
				}; // WvkKhrWin32SurfaceDispatchTableCreateInfo

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class WvkKhrWin32SurfaceDispatchTable : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline static const std::string& s_getName(void) noexcept;

				private:

					inline static const std::string s_name = "VK_KHR_win32_surface";

				public:

					inline VkResult wvkCreateWin32SurfaceKHR(const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface) const noexcept;

				private:

					PFN_vkCreateWin32SurfaceKHR m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkKhrWin32SurfaceDispatchTable(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~WvkKhrWin32SurfaceDispatchTable(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* **������ � �������������� ������ WvkKhrWin32SurfaceCommands.**
					*
					* @details
					* ����� ��������� ��������� `WvkKhrWin32SurfaceDispatchTableCreateInfo`, ��������� �,
					* ���������� (���� �������� ������ � ����������) � ��������� �������� ������ ������ Vulkan.
					*
					* ���������, ��� ������ `create_info` ���������� �������� ��������� `wvk_loader` � `wvk_instance`,
					* ����� ������� ����� ����������� �������� ��������� �� `vkCreateWin32SurfaceKHR`.
					*
					* @param[in] create_info
					* ��������� � �������, ������������ ��� �������� ������� ���������� `VK_KHR_win32_surface`.
					*
					* @return
					* ���������� `OK` � ������ ��������� ��������.
					* ���� ��������� ��� �������� ������� ����������� �������� � ���������� `FAIL` � ������������.
					*
					* @code
					* WvkKhrWin32SurfaceDispatchTable _dispatch_table;
					* WvkKhrWin32SurfaceDispatchTableCreateInfo info;
					* info.wvk_loader = &some_loader;
					*
					* WvkStatus status = _dispatch_table.create(info);
					* if (!status) {
					*     // ��������� ������
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus create(const WvkKhrWin32SurfaceDispatchTableCreateInfo& create_info) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void destroy(void) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const WvkKhrWin32SurfaceDispatchTableCreateInfo& getCreateInfo(void) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* **��������� ���������� ������� ���������� ��� �������� WvkKhrWin32SurfaceDispatchTable.**
					*
					* @details
					* ����� ��������� ������� �������� ������������� ��������� `wvk_loader` � `wvk_instance` 
					* � ��������� `m_create_info`. ��� ����� ���������� ���������� ��������� ������� Vulkan.
					*
					* ������������, ��� �������, ������ ������ `create`, �� �������� ���������� �� �������.
					*
					* @return
					* ���������� ������ `WvkStatus`. ���� �������� ������ ������� � ������������ `OK`.
					* ���� `wvk_commands == nullptr`, ������������ `FAIL` � ��������������� ����������.
					*
					* @code
					* WvkKhrWin32SurfaceDispatchTable _table;
					* WvkKhrWin32SurfaceDispatchTableCreateInfo create_info;
					* create_info.wvk_loader = ...;
					* _table.create(create_info); // ������ ����� ������ validationCreateInfo
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus validationCreateInfo(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* ��������� ��������� �� ������� vkCreateWin32SurfaceKHR ����� WvkLoader.
					*
					* ����� �������������� ������ �������� Vulkan, ������� ����� ���������,
					* � ���������� ����� `invokeWithVkInstanceMethod` �� `WvkInstance`, �����
					* ��������� ��������� �� ������� `vkCreateWin32SurfaceKHR` � ������� �������.
					*
					* @return
					* ���������� ������ �������� ���� WvkStatus. � ������ ������ � ������ OK,
					* � ��������� ������ �������� ��� ������ � ��������� �� ������.
					*
					* @code
					* WvkKhrWin32SurfaceDispatchTable table;
					* WvkStatus status = table.loadVulkanCommand();
					* if (!status) {
					*     std::cerr << status.what() << std::endl;
					* }
					* @endcode
					*
					* @warning
					* ���� ���� `m_create_info.wvk_instance` ��� `m_create_info.wvk_loader` �� ����������������,
					* ��������� ������ ������� � ������. �������, ��� ��������� `m_create_info` ���������
					* ��������� �� ������ ����� ������.
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus loadVulkanCommand(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void reset(void) noexcept;

				private:

					WvkKhrWin32SurfaceDispatchTableCreateInfo m_create_info = {};
				}; // class WvkKhrWin32SurfaceDispatchTable

			} // namespace mswindows

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_khr_win32_surface_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_WIN32_SURFACE_DISPATCH_TABLE_H
