////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_khr_win32_surface_dispatch_table.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "../../wvk_loader.h"
#include "../../wvk_instance.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			namespace mswindows {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkKhrWin32SurfaceDispatchTable::WvkKhrWin32SurfaceDispatchTable(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkKhrWin32SurfaceDispatchTable::~WvkKhrWin32SurfaceDispatchTable(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkKhrWin32SurfaceDispatchTable::create(const WvkKhrWin32SurfaceDispatchTableCreateInfo& create_info) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. �������� �� ��������� �������������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (m_valid) {
						return _status.set(VknStatusCode::ALREADY_INITIALIZED);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. ��������� ��������� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ���������� ������� ������, ���� �������� ������������� ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
						_status = validationCreateInfo();

						if (!_status) {
							reset();
							return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrWin32SurfaceDispatchTable::validationCreateInfo - fail.");
						}
					//}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. ��������� ��������� �� vkCreateWin32SurfaceKHR
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = loadVulkanCommand();

					if (!_status) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrWin32SurfaceDispatchTable::loadVulkanCommand - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. ���������� �������� ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_valid = true;
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void WvkKhrWin32SurfaceDispatchTable::destroy(void) noexcept {
					reset();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkKhrWin32SurfaceDispatchTable::validationCreateInfo(void) const noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. �������� ������������� ��������� �� nullptr
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (m_create_info.wvk_loader == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrWin32SurfaceDispatchTable::wvk_loader - nullptr.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. �������� ������������� ��������� �� nullptr
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					else if (m_create_info.wvk_instance == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrWin32SurfaceDispatchTable::wvk_instance - nullptr.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. �� � ������� � ���������� �������� ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkKhrWin32SurfaceDispatchTable::loadVulkanCommand(void) noexcept {
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ��������� ���������� ������� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. ��������� ������ �������� Vulkan, ������� ����� ���������
					// � ������ ������ � ������ vkCreateWin32SurfaceKHR
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					std::vector<WvkVulkanProcedureInfo> _procedures = {
						{ "vkCreateWin32SurfaceKHR", reinterpret_cast<void**>(&m_vkCreateWin32SurfaceKHR) }
					};

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ��������� ��������� ����� ������ WvkInstance::invokeWithVkInstanceMethod
					// ����� �������� WvkLoader::loadProcedure � ��������� ���������� � ������ ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
						&WvkLoader::loadProcedure,
						m_create_info.wvk_loader,
						_procedures
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. ������������ ��������� ��������
					// ���� ��������� ������, ���������� ������ � ����� FAIL � ����������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (!_status) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkInstance::invokeWithVkInstanceMethod - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. ���������� ������ �������� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void WvkKhrWin32SurfaceDispatchTable::reset(void) noexcept {
					m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;

					m_create_info = {};

					m_valid = false;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace mswindows

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev