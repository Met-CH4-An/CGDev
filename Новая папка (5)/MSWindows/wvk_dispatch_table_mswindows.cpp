////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_dispatch_table_mswindows.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "../wvk_instance.h"
#include "../wvk_dispatch_table.h"

namespace CGDev {

	namespace wvk {
	
		namespace mswindows {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkDispatchTableMSWindows::WvkDispatchTableMSWindows(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkDispatchTableMSWindows::~WvkDispatchTableMSWindows(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDispatchTableMSWindows::create(const WvkDispatchTableMSWindowsCreateInfo& create_info) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� �� ��������� �������������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_valid) {
					return _status.set(VknStatusCode::ALREADY_INITIALIZED);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� ���������� ������� ���������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = validationCreateInfo(create_info);
				if (!_status) {
					destroy();
					return _status.setFail("WvkDispatchTableMSWindows::validationCreateInfo is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� ������ ������� vkGetInstanceAddr
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = create();
				if (!_status) {
					destroy();
					return _status.setFail("WvkDispatchTableMSWindows::create is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� ���������� �� Vulkan-�������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = loadProcedure();
				if (!_status) {
					destroy();
					return _status.setFail("WvkDispatchTableMSWindows::loadProcedure is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_valid = true;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkDispatchTableMSWindows::destroy(void) noexcept {
				// =======================================
				// [Category]: Surface
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_win32_surface
				// ~~~~~~~~~~~~~~~~
				m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;

				if(m_vulkan_dll != NULL)
					FreeLibrary(m_vulkan_dll);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkDispatchTableMSWindowsCreateInfo m_create_info = {};
				m_vulkan_dll = NULL;
				m_vkGetInstanceProcAddr = VK_NULL_HANDLE;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDispatchTableMSWindows::validationCreateInfo(const WvkDispatchTableMSWindowsCreateInfo& create_info) noexcept {
				WvkStatus _status;

				if(create_info.wvk_dispatch_table_ptr == nullptr) {
					return _status.setFail("WvkDispatchTableMSWindowsCreateInfo::wvk_dispatch_table_ptr is nullptr.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info = create_info;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDispatchTableMSWindows::create(void) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��������� Vulkan-���������� (vulkan-1.dll)
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_vulkan_dll = LoadLibraryA("vulkan-1.dll");
				
				if (m_vulkan_dll == NULL) {
					return _status.setFail("LoadLibraryA is NULL.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� ����� ������� vkGetInstanceProcAddr �� ����������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_vkGetInstanceProcAddr = reinterpret_cast<PFN_vkGetInstanceProcAddr>(
					GetProcAddress(m_vulkan_dll, "vkGetInstanceProcAddr")
					);

				if (m_vkGetInstanceProcAddr == NULL) {
					return _status.setFail("GetProcAddress is NULL.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDispatchTableMSWindows::loadProcedure(void) noexcept {
				WvkStatus _status;

				std::vector<WvkVulkanProcedureInfo> _procedures;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� ���������, ������� ����� �������� ������ ��� VkInstance
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_create_info.wvk_dispatch_table_ptr->getWvkInstance() != nullptr) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateWin32SurfaceKHR", reinterpret_cast<void**>(&m_vkCreateWin32SurfaceKHR)));

					_status = m_create_info.wvk_dispatch_table_ptr->loadProcedure([this](const char* name) {
						return m_vkGetInstanceProcAddr(m_create_info.wvk_dispatch_table_ptr->getWvkInstance()->getVkInstance(), name);
						},
						_procedures);

					if (!_status) {
						return _status.setFail("WvkDispatchTable::loadInstanceProcs is fail.");
					}
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� ���������, ������� ����� �������� ��� ��� VkInstance, ��� � ��� VkDevice
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_create_info.wvk_dispatch_table_ptr->getWvkInstance() != nullptr || m_create_info.wvk_dispatch_table_ptr->getWvkLogicalDevice() != nullptr) {
					_procedures.clear();
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace mswindows

	} // namespace wvk

} // namespace CGDev