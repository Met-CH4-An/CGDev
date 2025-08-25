////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_logical_device.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"
#include "wvk_physical_device.h"
#include "wvk_dispatch_table.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkLogicalDevice::WvkLogicalDevice(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkLogicalDevice::~WvkLogicalDevice(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::create(const WvkLogicalDeviceCreateInfo& create_info) noexcept {
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
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::validationCreateInfo() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ����������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = create();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ������� ���������������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createWvkDispatchTable();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkLogicalDevice::destroy(void) noexcept {

			// ~~~~~~~~~~~~~~~~
			// 
			// ~~~~~~~~~~~~~~~~

			//m_create_info.wvk_dispatch_table_ptr->vknDestroyDevice(m_vk_device, VK_NULL_HANDLE);

			// ~~~~~~~~~~~~~~~~
			// ������� ������
			// ~~~~~~~~~~~~~~~~

			m_create_info				= {};
			m_vk_device					= VK_NULL_HANDLE;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::validationCreateInfo(const WvkLogicalDeviceCreateInfo& create_info) noexcept {
			WvkStatus _status(VknStatusCode::SUCCESSFUL);

			if (create_info.wvk_instance_ptr == nullptr) {
				_status.setFail("\n\tWvkLogicalDeviceCreateInfo::wvk_instance_ptr is nullptr.");
			}
			if (!create_info.physical_device_group_index.has_value()) {
				_status.setFail("\n\tWvkLogicalDeviceCreateInfo::physical_device_group_index is not value.");
			}
			if (create_info.physical_device_indices.empty()) {
				_status.setFail("\n\tWvkLogicalDeviceCreateInfo::physical_device_indices is empty.");
			}
			if (create_info.wvk_logical_device_queue_create_infos.empty()) {
				_status.setFail("\n\tWvkLogicalDeviceCreateInfo::wvk_logical_device_queue_create_infos is empty.");
			}
			else {
				for(const auto& it_0 : create_info.wvk_logical_device_queue_create_infos) {
					if (!it_0.index.has_value()) {
						_status.setFail("\n\tWvkLogicalDeviceCreateInfo::wvk_logical_device_queue_create_infos::index is not value.");
					}
					if (!it_0.queue_count.has_value()) {
						_status.setFail("\n\tWvkLogicalDeviceCreateInfo::wvk_logical_device_queue_create_infos::queue_count is not value.");
					}
					if (it_0.priority_collection.empty()) {
						_status.setFail("\n\tWvkLogicalDeviceCreateInfo::wvk_logical_device_queue_create_infos::priority_collection is empty.");
					}
				}
			}
			
			if (!_status) return _status;			
				
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::prepareVkQueueCreateInfo(std::vector<VkDeviceQueueCreateInfo>& queue_create_info_collection1) const noexcept {
			WvkStatus _status;

			// ���������� ������ ���������������� ������������ �������
			for (const auto& it_0 : m_create_info.wvk_logical_device_queue_create_infos) {

				// ������ � ��������� �������� �������
				VkDeviceQueueCreateInfo vk_queue_create_info{};
				vk_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
				vk_queue_create_info.pNext = nullptr;
				vk_queue_create_info.flags = 0; // ���������������, ������ 0
				//vk_queue_create_info.queueFamilyIndex = it_0.wvk_queue_family_ptr->getIndexFamily();		// ������ ��������� ��������
				vk_queue_create_info.queueCount = it_0.queue_count.value();			// ���������� ����������� ��������
				vk_queue_create_info.pQueuePriorities = it_0.priority_collection.data();	// ���������� �������� (�� 0.0f �� 1.0f)

				// ��������� � �������� ���������
				queue_create_info_collection1.emplace_back(std::move(vk_queue_create_info));
			}

			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::prepareExtensions(std::vector<const char*>& extension_names) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� � ������ ����������, ������� ������� �� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			extension_names = {
				#define X(str) str,
				WVK_COMPILE_TIME_LOGICAL_DEVICE_EXTENSIONS
				#undef X
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� � ������ ����������, ������� �������� ��� �������� ����������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::transform(
				m_create_info.extension_names.cbegin(),
				m_create_info.extension_names.cend(),
				std::back_inserter(extension_names),
				[](const std::string& s) {
					return s.c_str();
				}
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::create(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� �� WvkInstance ���������� ����������
			// �������� � WvkLogicalDeviceCreateInfo
			// physical_device_group_index - ������
			// physical_device_indices - ����� � ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			const auto& _group_index = m_create_info.physical_device_group_index.value();
			const auto& _in_group_index = m_create_info.physical_device_indices[0];
			const auto& _wvk_phys_dev = m_create_info.wvk_instance_ptr->getWvkPhysicalDevices()[_group_index][_in_group_index];
			const auto& _dispatch_table_ptr = m_create_info.wvk_instance_ptr->getWvkDispatchTable();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������������ ������ ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkDeviceQueueCreateInfo> vk_queue_create_infos;
			for (const auto& it_0 : m_create_info.wvk_logical_device_queue_create_infos) {
				VkDeviceQueueCreateInfo vk_queue_create_info{};
				vk_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
				vk_queue_create_info.pNext = nullptr;
				vk_queue_create_info.flags = 0;
				vk_queue_create_info.queueFamilyIndex = it_0.index.value();
				vk_queue_create_info.queueCount = it_0.queue_count.value();
				vk_queue_create_info.pQueuePriorities = it_0.priority_collection.data();
				vk_queue_create_infos.push_back(vk_queue_create_info);
			}
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ���������� feature
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkPhysicalDeviceFeatures* _pEnabledFeatures = nullptr;
			void* _pNext = nullptr;

			//std::transform(
			//	m_create_info.m_wvk_logical_device_feature_collection.begin(),
			//	m_create_info.m_wvk_logical_device_feature_collection.end(),
			//	std::back_inserter(_vk_base_in_collection),
			//	[](const auto& wvk_wvk_logic_dev_feature) {
			//		return wvk_wvk_logic_dev_feature.vk_base_in;
			//	}
			//);


#if WVK_VULKAN_API_VERSION == WVK_VULKAN_API_VERSION_10 && WVK_KHR_get_physical_device_properties2 == WVK_DISABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Vulkan 1.0 ��� ���������� get_physical_device_properties2
			//        ���������� ������� ��������� VkPhysicalDeviceFeatures
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_pEnabledFeatures = &m_create_info.m_vk_physical_device_features;

#elif WVK_VULKAN_API_VERSION == WVK_VULKAN_API_VERSION_10 && WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Vulkan 1.0 � ����������� VK_KHR_get_physical_device_properties2
			//        ���������� VkPhysicalDeviceFeatures2KHR � ��������� � �������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkPhysicalDeviceFeatures2KHR _features2_khr = {
				.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
				.pNext = nullptr,
				.features = m_create_info.m_vk_physical_device_features
			};
			_pNext = &_features2_khr;
			_pEnabledFeatures = nullptr;

#elif WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Vulkan 1.1 ��� ���� � ���������� VkPhysicalDeviceFeatures2
			//        ��������� � ������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkPhysicalDeviceFeatures2 _features2 = {
				.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
				.pNext = nullptr,
				.features = m_create_info.m_vk_physical_device_features
			};
			_pNext = &_features2;
			_pEnabledFeatures = nullptr;
#endif

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ���������� VkDeviceGroupDeviceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#if WVK_VULKAN_API_VERSION == WVK_VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_DISABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ���������� VK_KHR_device_group_creation ��������� ��� Vulkan 1.0.
			//        ������� �������������� ������ �� ���������.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#else
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ������ ���������� GPU �� ���������.
			//        ���� ������ ����� �������������� ��� ������� ��� �������� �������������.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			const auto& wvk_phys_devs = m_create_info.wvk_instance_ptr->getWvkPhysicalDevices()[m_create_info.physical_device_group_index.value()];

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� ������������� ��������� � ���������.
			//        ����� `checkCompatibility` ��������� _compatibility.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			bool _compatibility = false;
			//auto _wvk_status = _wvk_phys_dev->checkCompatibility(m_create_info.wvk_physical_devices, _compatibility);

			//if (!_wvk_status.isOk()) {
			//	return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::create - fail.");
			//}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������ ������ `VkPhysicalDevice` �� ��������� WvkPhysicalDevice.
			//        �� ����������� ��� ������������� ��������� ������ ���������.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkPhysicalDevice> _vk_phys_devs;
			std::transform(
				wvk_phys_devs.begin(),
				wvk_phys_devs.end(),
				std::back_inserter(_vk_phys_devs),
				[](const WvkPhysicalDeviceUptr& wvk_phys_dev) {
					return wvk_phys_dev->getVkPhysicalDevice();
				}
			);
#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_ENABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������ ��������� `VkDeviceGroupDeviceCreateInfoKHR`
			//        ��� ���������� `VK_KHR_device_group_creation` ��� Vulkan 1.0.
			//        ��������� ��� ��������� ���������� ����������.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkDeviceGroupDeviceCreateInfoKHR _device_group_create_info_khr = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR,
				.pNext = _pNext,
				.physicalDeviceCount = static_cast<uint32_t>(_vk_phys_devs.size()),
				.pPhysicalDevices = _vk_phys_devs.data()
			};
			_pNext = &_device_group_create_info_khr;
#elif VULKAN_API_VERSION >= VULKAN_API_VERSION_11
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������ ��������� `VkDeviceGroupDeviceCreateInfo`
			//        ��� API ������ 1.1 � ���� (������ � ����).
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkDeviceGroupDeviceCreateInfoKHR _device_group_create_info = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO,
				.pNext = _pNext,
				.physicalDeviceCount = static_cast<uint32_t>(_vk_physical_device_collection.size()),
				.pPhysicalDevices = _vk_physical_device_collection.data()
			};
			_pNext = &_device_group_create_info;
#endif
#endif

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ���������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			std::vector<const char*> _ext_names;
			_status = prepareExtensions(_ext_names);

			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\nWvkLogicalDevice::prepareExtensions() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 7. ���������� ��������� VkDeviceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkDeviceCreateInfo _vk_device_create_info = {};
			_vk_device_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
			_vk_device_create_info.pNext = _pNext;
			_vk_device_create_info.flags = 0;
			_vk_device_create_info.queueCreateInfoCount = static_cast<uint32_t>(vk_queue_create_infos.size());
			_vk_device_create_info.pQueueCreateInfos = vk_queue_create_infos.data();
			_vk_device_create_info.enabledLayerCount = 0;
			_vk_device_create_info.ppEnabledLayerNames = nullptr;
			_vk_device_create_info.enabledExtensionCount = static_cast<uint32_t>(_ext_names.size());
			_vk_device_create_info.ppEnabledExtensionNames = _ext_names.data();
			_vk_device_create_info.pEnabledFeatures = _pEnabledFeatures;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� vkDevice
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			auto _vk_result = _dispatch_table_ptr->wvkCreateDevice(_wvk_phys_dev->getVkPhysicalDevice(), &_vk_device_create_info, VK_NULL_HANDLE, &m_vk_device);
			
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_DEVICE_LOST:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_DEVICE_LOST."); break;
				case VK_ERROR_EXTENSION_NOT_PRESENT:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_EXTENSION_NOT_PRESENT."); break;
				case VK_ERROR_FEATURE_NOT_PRESENT:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_FEATURE_NOT_PRESENT."); break;
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_INITIALIZATION_FAILED."); break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY."); break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY."); break;
				case VK_ERROR_TOO_MANY_OBJECTS:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_TOO_MANY_OBJECTS."); break;
				case VK_ERROR_UNKNOWN:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_UNKNOWN."); break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_VALIDATION_FAILED_EXT."); break;
				default:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is Unknown error."); break;
				}
				return _status.set(VknStatusCode::FAIL, "\n\twvkCreateDevice is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status.m_code = VknStatusCode::SUCCESSFUL;
			return _status;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::createWvkDispatchTable(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ������,
			// ��������� WvkDispatchTableCreateInfo
			// � ������ WvkDispatchTable
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_wvk_dispatch_table_ptr = std::make_unique<WvkDispatchTable>();
			
			WvkDispatchTableCreateInfo _create_info = {
				.vkInstance = m_create_info.wvk_instance_ptr->getVkInstance(),
				.vkDevice = m_vk_device,
			};

			_status = m_wvk_dispatch_table_ptr->create(_create_info);

			if (!_status) {
				return _status.setFail("\nWvkDispatchTable::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status.m_code = VknStatusCode::SUCCESSFUL;
			return _status;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkLogicalDevice::reset(void) noexcept {

		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev