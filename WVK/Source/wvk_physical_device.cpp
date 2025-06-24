////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_physical_device.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
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

		WvkStatus WvkPhysicalDevice::checkCompatibility(const WvkPhysicalDevicePtrArr1& wvk_physical_device_collection, bool& compatibility) const noexcept {
			WvkStatus _status;

			uint32_t _count = 0;
			VkResult _vk_res = m_create_info.wvk_instance_dispatch_table->wvkEnumeratePhysicalDeviceGroups(&_count, nullptr);

			if (_vk_res != VK_SUCCESS) {
				switch (_vk_res) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_INITIALIZATION_FAILED.");
					break;
				}
				return _status.set(VknStatusCode::FAIL);
			}

			std::vector<VkPhysicalDeviceGroupProperties> _vk_physical_device_group_properties_collection(_count);
			_vk_res = m_create_info.wvk_instance_dispatch_table->wvkEnumeratePhysicalDeviceGroups(&_count, _vk_physical_device_group_properties_collection.data());

			if (_vk_res != VK_SUCCESS) {
				switch (_vk_res) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_INITIALIZATION_FAILED.");
					break;
				case VK_INCOMPLETE:
					_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_INCOMPLETE.");
					break;
				}
				return _status.set(VknStatusCode::FAIL);
			}

			auto makeDeviceSet = [](const VkPhysicalDeviceGroupProperties& group) {
				return std::unordered_set<VkPhysicalDevice>(
					group.physicalDevices,
					group.physicalDevices + group.physicalDeviceCount
				);
				};

			std::vector<const WvkPhysicalDevice*> _wvk_physical_device_collection_temp;
			std::transform(
				wvk_physical_device_collection.begin(),
				wvk_physical_device_collection.end(),
				std::back_inserter(_wvk_physical_device_collection_temp),
				[](WvkPhysicalDevice* ptr) {
					return static_cast<const WvkPhysicalDevice*>(ptr);
				}
			);
			_wvk_physical_device_collection_temp.push_back(this);
			
			for (size_t ct_0 = 0; ct_0 < _vk_physical_device_group_properties_collection.size(); ++ct_0) {
				const auto& group = _vk_physical_device_group_properties_collection[ct_0];
				std::unordered_set<VkPhysicalDevice> device_set = makeDeviceSet(group);

				// Проверяем, все ли устройства из a есть в текущей группе
				bool all_found = std::all_of(_wvk_physical_device_collection_temp.begin(), _wvk_physical_device_collection_temp.end(), [&](const WvkPhysicalDevice* wvk_phys_dev) {
					return device_set.contains(wvk_phys_dev->m_vk_physical_device);
					});

				if (all_found) {
					compatibility = true;
					return _status.setOk();
				}
			}

			compatibility = false;
			return _status.setOk();
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
				// В версиях Vulkan < 1.1 метод ничего не делает
			}
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkPhysicalDevice::validationCreateInfo(void) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка: vk_physical_device не должен быть VK_NULL_HANDLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.vk_physical_device == VK_NULL_HANDLE) {
				return _status.set(VknStatusCode::FAIL,
					"\n\tWvkPhysicalDeviceCreateInfo::vk_physical_device - VK_NULL_HANDLE.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Проверка: wvk_instance_dispatch_table не должен быть nullptr
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			else if (m_create_info.wvk_instance_dispatch_table == nullptr) {
				return _status.set(VknStatusCode::FAIL,
					"\n\tWvkPhysicalDeviceCreateInfo::wvk_instance_dispatch_table - nullptr.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Все проверки пройдены — возвращаем успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkPhysicalDevice::createVkPhysicalDevice(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Инициализация физического устройства Vulkan
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