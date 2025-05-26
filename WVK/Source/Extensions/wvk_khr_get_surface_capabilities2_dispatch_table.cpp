////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_khr_get_surface_capabilities2_dispatch_table.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkKhrGetSurfaceCapabilities2DispatchTable::WvkKhrGetSurfaceCapabilities2DispatchTable(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkKhrGetSurfaceCapabilities2DispatchTable::~WvkKhrGetSurfaceCapabilities2DispatchTable(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrGetSurfaceCapabilities2DispatchTable::create(const WvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo& create_info) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ��������� ���������� ��������� ��������.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info = create_info;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. ���� �������� ������ � ����������, ��������� �������� ������.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
					_status = validationCreateInfo();

					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DispatchTable::validationCreateInfo() - fail.");
					}
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. ��������� ��������� �� Vulkan-�������, ��������� � Surface.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = loadVulkanCommand();

				if (_status.m_code != VknStatusCode::SUCCESSFUL) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DispatchTable::loadVulkanCommand() - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 4. �������� ���������� ��������.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkKhrGetSurfaceCapabilities2DispatchTable::destroy(void) noexcept {
				reset();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrGetSurfaceCapabilities2DispatchTable::validationCreateInfo(void) const noexcept {
				WvkStatus _status;

				if (m_create_info.wvk_loader == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo::wvk_loader - nullptr.");
				}
				else if (m_create_info.wvk_instance == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo::wvk_instance - nullptr.");
				}

				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrGetSurfaceCapabilities2DispatchTable::loadVulkanCommand(void) noexcept {
				WvkStatus _status;

				std::vector<WvkVulkanProcedureInfo> _procedures = {
					{ "vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR) },
					{ "vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR) }
				};

				_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
					&WvkLoader::loadProcedure,
					m_create_info.wvk_loader,
					_procedures
				);

				if (!_status) {
					return _status.set(VknStatusCode::FAIL,
						"\n\tWvkInstance::invokeWithVkInstanceMethod - fail.");
				}

				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkKhrGetSurfaceCapabilities2DispatchTable::reset(void) noexcept {
				m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

				m_create_info = {};

				m_valid = false;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev