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
#include "wvk_loader_dispatch_table.h"
#include "wvk_layer.h"
#include "wvk_extension.h"

#include "Layers/wvk_layer_khronos_validation.h"
#include "Extensions/wvk_ext_debug_utils.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstance::WvkInstance(void) noexcept {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstance::~WvkInstance(void) noexcept {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::create(const WvkInstanceCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ���������, ��� �� ������� ��� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_valid) {
				return _status.set(VknStatusCode::ALREADY_INITIALIZED);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. ��������� ������� ��������� � ���� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. �������� ���������� ������� ���������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = validationCreateInfo();
			if (!_status) {
				reset(); // ������� ���������� ���������
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstance::validationCreateInfo() - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 4. ���������� ���� (layers) ��� �������� VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = prepareLayer();
			if (_status.m_code != VknStatusCode::SUCCESSFUL) {
				reset(); // ������� ���������� ���������
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstance::prepareLayer() - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 5. ���������� ���������� (extensions) ��� VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = prepareExtension();
			if (_status.m_code != VknStatusCode::SUCCESSFUL) {
				reset(); // ������� ���������� ���������
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstance::prepareExtension() - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 6. �������� ������ VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createVkInstance();
			if (_status.m_code != VknStatusCode::SUCCESSFUL) {
				reset(); // ������� ���������� ���������
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstance::createVkInstance() - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 7. �������� ���������� �������� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstance::destroy(void) noexcept {

			//m_create_info.wvk_instance_dispatch_table->vkDestroyInstance(m_vk_instance, VK_NULL_HANDLE);
				
			// ~~~~~~~~~~~~~~~~
			// ������� ������
			// ~~~~~~~~~~~~~~~~

			m_create_info = {};
			m_vk_instance = nullptr;
			m_layer_properties_collection.clear();
			m_extension_properties_collection.clear();
			m_layer_name_collection.clear();
			m_extension_name_collection.clear();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::validationCreateInfo(void) const noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ���������� ���������� �������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. �������� ������������� ����: wvk_loader_dispatch_table
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.wvk_loader_dispatch_table == nullptr) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceCreateInfo::wvk_loader_dispatch_table - nullptr.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. ��� �������� �������� � ���������� �����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::prepareLayer(void) noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ��������� ���������� ��������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus _status;

			uint32_t _vk_layer_properties_count = 0;
			VkResult _vk_result = m_create_info.wvk_loader_dispatch_table->wvkEnumerateInstanceLayerProperties(&_vk_layer_properties_count, nullptr);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. ��������� ������ ��� ��������� ���������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				default:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = Unknown error.");
					break;
				}
				return _status.set(VknStatusCode::FAIL, "\n\tvknEnumerateInstanceLayerProperties - fail");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. ��������� ������ ��� �������� ������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_layer_properties_collection.resize(_vk_layer_properties_count);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 4. ��������� ������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_vk_result = m_create_info.wvk_loader_dispatch_table->wvkEnumerateInstanceLayerProperties(&_vk_layer_properties_count, m_layer_properties_collection.data());

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 5. ��������� ������ ��� ��������� ������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (_vk_result != VK_SUCCESS) {
				if (_vk_result == VK_INCOMPLETE) {
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_INCOMPLETE.");
				}
				else {
					switch (_vk_result) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					default:
						_status.append("\n\tvkEnumerateInstanceLayerProperties = Unknown error.");
						break;
					}
					return _status.set(VknStatusCode::FAIL, "\n\tvknEnumerateInstanceLayerProperties - fail");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 6. �������� ������� ����������� ���� � ������ ��������� ����
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
				// ������� ����, ��������� � ������������ ������
				//for (const auto& it_name : wvk::Build::layer_name_collection) {
				//	bool _found = false;

					// ������� ��������� ���� Vulkan
				//	for (const auto& it_layer_properties : m_layer_properties_collection) {
						// ���������� ��� ���� � ������ �� ���������
				//		if (std::strcmp(it_layer_properties.layerName, it_name.data()) == 0) {
							// ��������� ��� ���������� ���� � ������ ��������������
				//			m_layer_name_collection.push_back(it_layer_properties.layerName);
				//			_found = true;
				//			break;
				//		}
				//	}

					// ���� ���� �� ���� ���� �� ������ �� ������ � ������� � �������
				//	if (!_found) {
				//		return _status.set(VknStatusCode::FAIL, "\n\t%s - not found.", std::string(it_name));
				//	}
				//}
			//}
			m_layer_name_collection.push_back("VK_LAYER_KHRONOS_validation");
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 7. ���������� �������� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::prepareExtension(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ����������� ���������� ��������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			uint32_t _vk_extension_properties_count = 0;
			VkResult _vk_result = m_create_info.wvk_loader_dispatch_table->wvkEnumerateInstanceExtensionProperties(
				nullptr, &_vk_extension_properties_count, nullptr);

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				default:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = Unknown error.");
					break;
				}
				return _status.set(VknStatusCode::FAIL, "\n\tvknEnumerateInstanceExtensionProperties - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. �������� �������� ���� ���������� � ���������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_extension_properties_collection.resize(_vk_extension_properties_count);

			_vk_result = m_create_info.wvk_loader_dispatch_table->wvkEnumerateInstanceExtensionProperties(
				nullptr, &_vk_extension_properties_count, m_extension_properties_collection.data());

			if (_vk_result != VK_SUCCESS) {
				if (_vk_result == VK_INCOMPLETE) {
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_INCOMPLETE.");
				}
				else {
					switch (_vk_result) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					default:
						_status.append("\n\tvkEnumerateInstanceLayerProperties = Unknown error.");
						break;
					}
					return _status.set(VknStatusCode::FAIL, "\n\tvknEnumerateInstanceExtensionProperties - fail.");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. ��� ���������� ��������� ��������� ������� ������ ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
				for (const auto& it_name : wvk::Build::ValidationBuildInfo::extension_name_collection) {
					bool _found = false;

					for (const auto& it_extension_properties : m_extension_properties_collection) {
						if (std::strcmp(it_extension_properties.extensionName, it_name.data()) == 0) {
							// ��������� ��������� ���������� � ������ ��������
							m_extension_name_collection.push_back(it_extension_properties.extensionName);
							_found = true;
							break;
						}
					}

					// ���� ���� �� ���� ������ ���������� �� ������� � ���������� ������
					if (_found == false) {
						return _status.set(VknStatusCode::FAIL, "\n\t%s - not found.", std::string(it_name));
					}
				}
			}*/

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 4. ��� ���������� ������������ ��������� ������� ������ ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*if constexpr (wvk::Build::SurfaceBuildInfo::enable == true) {
				for (const auto& it_name : wvk::Build::SurfaceBuildInfo::extension_name_collection) {
					bool _found = false;

					for (const auto& it_extension_properties : m_extension_properties_collection) {
						if (std::strcmp(it_extension_properties.extensionName, it_name.data()) == 0) {
							// ��������� ��������� ���������� � ������ ��������
							m_extension_name_collection.push_back(it_extension_properties.extensionName);
							_found = true;
							break;
						}
					}

					// ���� ���� �� ���� ������ ���������� �� ������� � ���������� ������
					if (_found == false) {
						return _status.set(VknStatusCode::FAIL, "\n\t%s - not found.", std::string(it_name));
					}
				}
			}*/
			m_extension_name_collection.insert(m_extension_name_collection.end(), Build::getInstanceExtensions().begin(), Build::getInstanceExtensions().end());
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 5. ���������� �������� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::createVkInstance(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ������������� ��������� VkApplicationInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkApplicationInfo _vkApplicationInfo = {};
			_vkApplicationInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
			_vkApplicationInfo.pNext = nullptr;
			_vkApplicationInfo.pApplicationName = "GPU";
			_vkApplicationInfo.applicationVersion = 0;
			_vkApplicationInfo.pEngineName = "GPU";
			_vkApplicationInfo.engineVersion = 0;
			_vkApplicationInfo.apiVersion = static_cast<uint32_t>(wvk::Build::vulkan_api_version);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. ������������� ��������� VkInstanceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkInstanceCreateInfo _instanceCreateInfo = {};
			_instanceCreateInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
			_instanceCreateInfo.pNext = VK_NULL_HANDLE;
			_instanceCreateInfo.pNext = nullptr;
			_instanceCreateInfo.flags = 0;
			_instanceCreateInfo.pApplicationInfo = &_vkApplicationInfo;
			_instanceCreateInfo.enabledLayerCount = static_cast<uint32_t>(m_layer_name_collection.size());
			_instanceCreateInfo.ppEnabledLayerNames = m_layer_name_collection.data();
			_instanceCreateInfo.enabledExtensionCount = static_cast<uint32_t>(m_extension_name_collection.size());
			_instanceCreateInfo.ppEnabledExtensionNames = m_extension_name_collection.data();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. ����� ������� ��� �������� ���������� Vulkan
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkResult _vk_result = m_create_info.wvk_loader_dispatch_table->wvkCreateInstance(&_instanceCreateInfo, VK_NULL_HANDLE, &m_vk_instance);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 4. ��������� ������, ���� �������� ���������� �� �������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\n\tvkCreateInstance = VK_ERROR_OUT_OF_HOST_MEMORY");
					break;

				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\n\tvkCreateInstance = VK_ERROR_OUT_OF_DEVICE_MEMORY");
					break;

				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("\n\tvkCreateInstance = VK_ERROR_INITIALIZATION_FAILED");
					break;

				case VK_ERROR_LAYER_NOT_PRESENT:
					_status.append("\n\tvkCreateInstance = VK_ERROR_LAYER_NOT_PRESENT");
					break;

				case VK_ERROR_EXTENSION_NOT_PRESENT:
					_status.append("\n\tvkCreateInstance = VK_ERROR_EXTENSION_NOT_PRESENT");
					break;

				case VK_ERROR_INCOMPATIBLE_DRIVER:
					_status.append("\n\tvkCreateInstance = VK_ERROR_INCOMPATIBLE_DRIVER");
					break;

				default:
					_status.append("\n\tvkCreateInstance = unknown VK_ERROR");
					break;
				}

				// ���������� ������, ���� �������� ���������� �� �������
				return _status.set(VknStatusCode::FAIL, "\n\tvknCreateInstance - fail");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 5. ���������� �������� ������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstance::reset(void) noexcept {

		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev