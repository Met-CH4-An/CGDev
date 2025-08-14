////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_dispatch_table.h"
#include "wvk_physical_device.h"
#include "wvk_layer.h"
#include "wvk_extension.h"

#include "Layers/wvk_layer_khronos_validation.h"
#include "Extensions/wvk_ext_debug_utils.h"

namespace CGDev {

	namespace wvk {
		//WvkStatus _status;

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��� 1. ��������� ��� �������� ��� �������� ����������� ��������
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//std::vector<std::string> _failed_procedures;

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��� 2. ������� ���� �������� � ������� �������� ������
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//for (const auto& it_0 : wvk_vulkan_procedure_collection1) {
			// �������� ����� ������� ����� vkGetInstanceProcAddr � ���������� � targetPtr
		//	*it_0.targetPtr = m_vkGetInstanceProcAddr(vk_instance, it_0.name);

			// ���� ������� �� ������� � ���������� ���
		//	if (*it_0.targetPtr == nullptr) {
		//		_failed_procedures.emplace_back(it_0.name);
		//	}
		//}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��� 3. ���� ���� ������, �������� ��������� � ���������� FAIL
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//if (!_failed_procedures.empty()) {
		//	std::string _error_message = "\n\tVulkan procedures not found:";
		//	for (const auto& _name : _failed_procedures) {
		//		_error_message += "\n\t- " + _name;
		//	}
		//	return _status.set(VknStatusCode::FAIL, _error_message.c_str());
		//}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��� 4. ��� ������� ������� ���������
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//return _status.setOk();
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstance::WvkInstance(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstance::~WvkInstance(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::create(const WvkInstanceCreateInfo& create_info) noexcept {
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
				return _status.set(VknStatusCode::FAIL, "\nWvkInstance::validationCreateInfo() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ������������� ������� ����������� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createGlobalDispatchTable();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\nWvkInstance::createGlobalDispatchTable() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ������ VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createVkInstance();
			if (_status.m_code != VknStatusCode::SUCCESSFUL) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\nWvkInstance::createVkInstance() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ������������� ������� ������ ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createInstanceDispatchTable();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\nWvkInstance::createInstanceDispatchTable() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ���������� ���������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createPhysicalDevices();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\nWvkInstance::createPhysicalDevices() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstance::destroy(void) noexcept {
			//m_create_info.wvk_dispatch_table_ptr->vkDestroyInstance(m_vk_instance, VK_NULL_HANDLE);
			
			if (m_wvk_global_dispatch_table_ptr != nullptr) m_wvk_global_dispatch_table_ptr->destroy();
			if (m_wvk_instance_dispatch_table_ptr != nullptr) m_wvk_instance_dispatch_table_ptr->destroy();
			for (auto& it_0 : m_wvk_physical_devices) {
				for (auto& it_1 : it_0) {
					it_1->destroy();
				}
			}
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = false;
			m_create_info = {};
			m_wvk_global_dispatch_table_ptr = nullptr;
			m_wvk_instance_dispatch_table_ptr = nullptr;
			m_wvk_physical_devices.clear();
			m_vk_instance = VK_NULL_HANDLE;
			m_layer_properties_collection.clear();
			m_extension_properties_collection.clear();
			m_layer_name_collection.clear();
			m_extension_name_collection.clear();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::requestLayerProperties(std::vector<VkLayerProperties>& vk_layer_properties) const noexcept {
			WvkStatus _status;

			vk_layer_properties.clear();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� ���������� ��������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			VkResult _vk_result = m_wvk_global_dispatch_table_ptr->wvkEnumerateInstanceLayerProperties(&_count, nullptr);

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				default:
					_status.append("\nvkEnumerateInstanceLayerProperties is Unknown error.");
					break;
				}
				return _status.setFail("\nvknEnumerateInstanceLayerProperties is fail");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� ������ � ��������� ������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			vk_layer_properties.resize(_count);
			_vk_result = m_wvk_global_dispatch_table_ptr->wvkEnumerateInstanceLayerProperties(&_count, vk_layer_properties.data());

			if (_vk_result != VK_SUCCESS) {
				if (_vk_result == VK_INCOMPLETE) {
					_status.append("\nvkEnumerateInstanceLayerProperties is VK_INCOMPLETE.");
				}
				else {
					switch (_vk_result) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\nvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\nvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					default:
						_status.append("\nvkEnumerateInstanceLayerProperties is Unknown error.");
						break;
					}
					return _status.setFail("\nvknEnumerateInstanceLayerProperties is fail");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::requestExtensionProperties(std::vector<VkExtensionProperties>& vk_extension_properties) const noexcept {
			WvkStatus _status;

			vk_extension_properties.clear();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� ���������� ��������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			VkResult _vk_result = m_wvk_global_dispatch_table_ptr->wvkEnumerateInstanceExtensionProperties(nullptr, &_count, nullptr);

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				default:
					_status.append("\nvkEnumerateInstanceLayerProperties is Unknown error.");
					break;
				}
				return _status.setFail("\nvknEnumerateInstanceExtensionProperties is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� ������ � ��������� ������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			vk_extension_properties.resize(_count);
			_vk_result = m_wvk_global_dispatch_table_ptr->wvkEnumerateInstanceExtensionProperties(nullptr, &_count, vk_extension_properties.data());

			if (_vk_result != VK_SUCCESS) {
				if (_vk_result == VK_INCOMPLETE) {
					_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_INCOMPLETE.");
				}
				else {
					switch (_vk_result) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tvkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					default:
						_status.append("\n\tvkEnumerateInstanceLayerProperties is Unknown error.");
						break;
					}
					return _status.setFail("\n\tvknEnumerateInstanceExtensionProperties is fail.");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::requestPhysicalDevices(std::vector<WvkPhysicalDevicePtr>& vk_physical_devices) const noexcept {
			WvkStatus _status;

			
			//VK_ERROR_INITIALIZATION_FAILED

			//	VK_ERROR_OUT_OF_DEVICE_MEMORY

			//	VK_ERROR_OUT_OF_HOST_MEMORY

			//	VK_ERROR_UNKNOWN

			//	VK_ERROR_VALIDATION_FAILED
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//m_create_info = create_info;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::validationCreateInfo(const WvkInstanceCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::createGlobalDispatchTable(void) noexcept {
			WvkStatus _status;

			m_wvk_global_dispatch_table_ptr = std::make_unique<WvkDispatchTable>();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������ WvkDispatchTable
			// ��������� ����� ��������� ������ ��, ������� �� ����� VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkDispatchTableCreateInfo _create_info = {};

			_status = m_wvk_global_dispatch_table_ptr->create(_create_info);
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkDispatchTable::create() - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::createInstanceDispatchTable(void) noexcept {
			WvkStatus _status;

			m_wvk_instance_dispatch_table_ptr = std::make_unique<WvkDispatchTable>();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������ WvkDispatchTable
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkDispatchTableCreateInfo _create_info = {
				.vkInstance = m_vk_instance,
			};

			_status = m_wvk_instance_dispatch_table_ptr->create(_create_info);
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkDispatchTable::create() - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::createPhysicalDevices(void) noexcept {
			WvkStatus _status;

			std::vector<std::vector<VkPhysicalDevice>> _vk_phys_device_groups;
			uint32_t _count = 0;

#if WVK_VULKAN_API_VERSION == WVK_VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_DISABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ���������� ��������� ���������� ���������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~			
			auto _vk_result = m_wvk_instance_dispatch_table_ptr->wvkEnumeratePhysicalDevices(&_count, nullptr);

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_INITIALIZATION_FAILED.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_VALIDATION_FAILED_EXT.");
					break;
				default:
					_status.append("\nvkEnumeratePhysicalDevices is Unknown error.");
					break;
				}
				return _status.setFail("\nvkEnumeratePhysicalDevices is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ��������� ���������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkPhysicalDevice> _vk_phys_devices(_count);
			_vk_result = m_wvk_instance_dispatch_table_ptr->wvkEnumeratePhysicalDevices(&_count, _vk_phys_devices.data());
			
			if (_vk_result != VK_SUCCESS) {
				if (_vk_result == VK_INCOMPLETE) {
					_status.append("\nvkEnumeratePhysicalDevices is VK_INCOMPLETE.");
				}
				else {
					switch (_vk_result) {
					case VK_ERROR_INITIALIZATION_FAILED:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_INITIALIZATION_FAILED.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_UNKNOWN:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_UNKNOWN.");
						break;
					case VK_ERROR_VALIDATION_FAILED_EXT:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_VALIDATION_FAILED_EXT.");
						break;
					default:
						_status.append("\nvkEnumeratePhysicalDevices is Unknown error.");
						break;
					}
					return _status.setFail("\nvkEnumeratePhysicalDevices is fail.");
				}
			}

			_vk_phys_device_groups.emplace_back(std::move(_vk_phys_devices));
#endif

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_device_group_creation == WVK_ENABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ���������� ��������� ���������� ���������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			auto _vk_result = m_wvk_instance_dispatch_table_ptr->wvkEnumeratePhysicalDeviceGroups(&_count, nullptr);

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_INITIALIZATION_FAILED.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_VALIDATION_FAILED_EXT.");
					break;
				default:
					_status.append("\nvkEnumeratePhysicalDevices is Unknown error.");
					break;
				}
				return _status.setFail("\nvkEnumeratePhysicalDevices is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �������� ��������� ���������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkPhysicalDeviceGroupProperties> _vk_phys_dev_groups(_count);
			_vk_result = m_wvk_instance_dispatch_table_ptr->wvkEnumeratePhysicalDeviceGroups(&_count, _vk_phys_dev_groups.data());
			
			if (_vk_result != VK_SUCCESS) {
				if (_vk_result == VK_INCOMPLETE) {
					_status.append("\nvkEnumeratePhysicalDevices is VK_INCOMPLETE.");
				}
				else {
					switch (_vk_result) {
					case VK_ERROR_INITIALIZATION_FAILED:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_INITIALIZATION_FAILED.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_UNKNOWN:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_UNKNOWN.");
						break;
					case VK_ERROR_VALIDATION_FAILED_EXT:
						_status.append("\nvkEnumeratePhysicalDevices is VK_ERROR_VALIDATION_FAILED_EXT.");
						break;
					default:
						_status.append("\nvkEnumeratePhysicalDevices is Unknown error.");
						break;
					}
					return _status.setFail("\nvkEnumeratePhysicalDevices is fail.");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ����������� ������ ���������� ��������� � ������ ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::transform(
				_vk_phys_dev_groups.begin(), _vk_phys_dev_groups.end(),
				std::back_inserter(_vk_phys_device_groups),
				[](const VkPhysicalDeviceGroupProperties& group) {
					return std::vector<VkPhysicalDevice>(
						group.physicalDevices,
						group.physicalDevices + group.physicalDeviceCount
					);
				}
			);
#endif

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������ WvkPhysicalDevice �� ��������� VkPhysicalDevice
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			for (const auto& it_0 : _vk_phys_device_groups) {

				WvkPhysicalDeviceUptrVec1 _wvk_phys_dev_group;
				for (const auto& it_1 : it_0) {
					
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��������� WvkPhysicalDeviceCreateInfo
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkPhysicalDeviceCreateInfo _create_info = {
						.wvk_instance_ptr = this,
						.vk_physical_device = it_1,
					};

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					auto _wvk_phys_dev = std::make_unique<WvkPhysicalDevice>();
					_status = _wvk_phys_dev->create(_create_info);

					if (_status) {
						_wvk_phys_dev_group.push_back(std::move(_wvk_phys_dev));
					}
				}

				m_wvk_physical_devices.push_back(std::move(_wvk_phys_dev_group));
			}
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		
		WvkStatus WvkInstance::prepareLayers(std::vector<const char*>& layer_names) const noexcept {
			WvkStatus _status;		

			layer_names.clear();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� � ������ ����,
			// ������� ������� �� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			layer_names = {
				#define X(str) str,
				WVK_COMPILE_TIME_LAYERS
				#undef X
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� � ������ ����, 
			// ������� �������� ��� �������� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::transform(
				m_create_info.vk_layer_names.cbegin(),
				m_create_info.vk_layer_names.cend(),
				std::back_inserter(layer_names),
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

		WvkStatus WvkInstance::prepareExtensions(std::vector<const char*>& extension_names) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� � ������ ����������, ������� ������� �� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			extension_names = {
				#define X(str) str,
				WVK_COMPILE_TIME_EXTENSIONS
				#undef X
			};
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� � ������ ����������, ������� �������� ��� �������� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::transform(
				m_create_info.vk_extension_names.cbegin(),
				m_create_info.vk_extension_names.cend(),
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

		WvkStatus WvkInstance::createVkInstance(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ���������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<const char*> _layer_names;
			_status = prepareLayers(_layer_names);
			
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstance::prepareLayer() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ���������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			std::vector<const char*> _ext_names;
			_status = prepareExtensions(_ext_names);
			
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstance::prepareExtensions() is fail.");
			}

			void* _pNext = nullptr;
				
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ���������� ���������� VK_EXT_debug_utils, ���� ��� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~	
#if WVK_EXT_debug_utils == WVK_ENABLE
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������������� ��������� VkDebugUtilsMessengerCreateInfoEXT
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~	
			VkDebugUtilsMessengerCreateInfoEXT _vk_debug_utils_messenger_create_info = {
				.sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
				.pNext = nullptr,
				.flags = 0,
				.messageSeverity = 
					VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT |
					VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT |
					VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT |
					VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
				.messageType =
					VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT |
					VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT |
					VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
					//VK_DEBUG_UTILS_MESSAGE_TYPE_DEVICE_ADDRESS_BINDING_BIT_EXT,
				.pfnUserCallback = &s_debugCallback,
				.pUserData = &_status
			};

			_pNext = &_vk_debug_utils_messenger_create_info;
#endif

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������������� ��������� VkApplicationInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkApplicationInfo _vkApplicationInfo = {};
			_vkApplicationInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
			_vkApplicationInfo.pNext = nullptr;
			_vkApplicationInfo.pApplicationName = "GPU";
			_vkApplicationInfo.applicationVersion = 0;
			_vkApplicationInfo.pEngineName = "GPU";
			_vkApplicationInfo.engineVersion = 0;
			_vkApplicationInfo.apiVersion = VK_API_VERSION_FROM_WVK(WVK_VULKAN_API_VERSION);
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ������������� ��������� VkInstanceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkInstanceCreateInfo _instanceCreateInfo = {};
			_instanceCreateInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
			_instanceCreateInfo.pNext = VK_NULL_HANDLE;
			_instanceCreateInfo.pNext = _pNext;
			_instanceCreateInfo.flags = 0;
			_instanceCreateInfo.pApplicationInfo = &_vkApplicationInfo;
			_instanceCreateInfo.enabledLayerCount = static_cast<uint32_t>(_layer_names.size());
			_instanceCreateInfo.ppEnabledLayerNames = _layer_names.data();
			_instanceCreateInfo.enabledExtensionCount = static_cast<uint32_t>(_ext_names.size());
			_instanceCreateInfo.ppEnabledExtensionNames = _ext_names.data();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ����� ������� ��� �������� ���������� Vulkan
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkResult _vk_result = m_wvk_global_dispatch_table_ptr->wvkCreateInstance(&_instanceCreateInfo, VK_NULL_HANDLE, &m_vk_instance);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� ������, ���� �������� ���������� �� �������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nvkCreateInstance is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nvkCreateInstance is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("\nvkCreateInstance is VK_ERROR_INITIALIZATION_FAILED.");
					break;
				case VK_ERROR_LAYER_NOT_PRESENT:
					_status.append("\nvkCreateInstance is VK_ERROR_LAYER_NOT_PRESENT.");
					break;
				case VK_ERROR_EXTENSION_NOT_PRESENT:
					_status.append("\nvkCreateInstance is VK_ERROR_EXTENSION_NOT_PRESENT.");
					break;
				case VK_ERROR_INCOMPATIBLE_DRIVER:
					_status.append("\nvkCreateInstance is VK_ERROR_INCOMPATIBLE_DRIVER.");
					break;
				default:
					_status.append("\nvkCreateInstance is unknown VK_ERROR.");
					break;
				}

				// ���������� ������, ���� �������� ���������� �� �������
				return _status.setFail("\n\tvknCreateInstance is fail.");
			}
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��������� ������ ��� ���������� ���������� VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#if WVK_EXT_debug_utils == WVK_ENABLE
			if (!_status) {
				return _status.setFail("\n\twvkCreateInstance is fail.");
			};
#endif

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// �����.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		VkBool32 WvkInstance::s_debugCallback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept {
			if (pUserData) {
				auto& _status = *static_cast<WvkStatus*>(pUserData);
				
				if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT) {
					_status << "\n[error::"; 			
				} else if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT) {
					_status << "\n[warning::";
				} else if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT) {
					_status << "\n[info::";
				} else if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT) {
					_status << "\n[verbose::";
				}
				
				if (messageTypes & VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT) {
					_status << "general]: " << pCallbackData->pMessage;
				} else if (messageTypes & VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT) {
					_status << "validation]: " << pCallbackData->pMessage;
				} else if (messageTypes & VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT) {
					_status << "performance]: " << pCallbackData->pMessage;
				} else if (messageTypes & VK_DEBUG_UTILS_MESSAGE_TYPE_DEVICE_ADDRESS_BINDING_BIT_EXT) {
					_status << "address_binding]: " << pCallbackData->pMessage;
				}
				_status << pCallbackData->pMessage;

				if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT) {
					_status.setFail();
				}
			}

			return VK_FALSE; // ���������� VK_FALSE, ����� ���������� ��������� ���������
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev