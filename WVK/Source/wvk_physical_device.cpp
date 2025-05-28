////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_physical_device.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_instance_dt.hpp"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkPhysicalDevice::WvkPhysicalDevice(void) noexcept {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkPhysicalDevice::~WvkPhysicalDevice(void) noexcept {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkPhysicalDevice::create(const WvkPhysicalDeviceCreateInfo& create_info) noexcept {
			WvkStatus _status;

			if (m_valid) {
				return _status.set(VknStatusCode::ALREADY_INITIALIZED);
			}

			m_create_info = create_info;

			if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
				_status = validationCreateInfo();

				if (!_status) {
					reset();
					return _status.set(VknStatusCode::FAIL, "\n\tVknPhysicalDevice::validationCreateInfo() - fail.");
				}
			}

			_status = createVkPhysicalDevice();

			if (!_status) {
				reset();
				return _status.set(VknStatusCode::FAIL, "\n\tcreateVkPhysicalDevice - fail.");
			}

			m_valid = true;
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkPhysicalDevice::destroy(void) noexcept {
			reset();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkPhysicalDevice::requestPhysicalDeviceProperties(VkPhysicalDeviceProperties& vk_physical_device_properties) const noexcept {
			m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceProperties(m_vk_physical_device, &vk_physical_device_properties);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkPhysicalDevice::requestPhysicalDeviceProperties(VkPhysicalDeviceProperties2& vk_physical_device_properties) const noexcept {
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

				vk_physical_device_properties.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;
		
				m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceProperties2(m_vk_physical_device, &vk_physical_device_properties);
			}

			else {
				// � ������� Vulkan < 1.1 ����� ������ �� ������
			}
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkPhysicalDevice::validationCreateInfo(void) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ��������: vk_physical_device �� ������ ���� VK_NULL_HANDLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.vk_physical_device == VK_NULL_HANDLE) {
				return _status.set(VknStatusCode::FAIL,
					"\n\tWvkPhysicalDeviceCreateInfo::vk_physical_device - VK_NULL_HANDLE.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. ��������: wvk_instance_dispatch_table �� ������ ���� nullptr
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			else if (m_create_info.wvk_instance_dispatch_table == nullptr) {
				return _status.set(VknStatusCode::FAIL,
					"\n\tWvkPhysicalDeviceCreateInfo::wvk_instance_dispatch_table - nullptr.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. ��� �������� �������� � ���������� �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkPhysicalDevice::createVkPhysicalDevice(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ������������� ����������� ���������� Vulkan
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vk_physical_device = m_create_info.vk_physical_device;

			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkPhysicalDevice::reset(void) noexcept {
			m_create_info = {};
			m_vk_physical_device = VK_NULL_HANDLE;

			m_valid = false;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev