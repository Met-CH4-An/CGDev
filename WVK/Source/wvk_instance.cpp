// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_dispatch_table.h"
#include "wvk_physical_device.h"
#include "wvk_layer.h"
#include "wvk_extension.h"

#include "Layers/wvk_layer_khronos_validation.h"

namespace CGDev {

	namespace wvk {

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
			// Проверка на повторную инициализацию
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_valid) {
				return _status.set(VknStatusCode::ALREADY_INITIALIZED);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Проверка валидности входной структуры
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = validationCreateInfo(create_info);
			if (!_status) {
				destroy();
				return _status.setFail("WvkInstance::validationCreateInfo() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание диспетчерской таблицы глобального уровня
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createGlobalDispatchTable();
			if (!_status) {
				destroy();
				return _status.setFail("WvkInstance::createGlobalDispatchTable() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание самого VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createVkInstance();
			if (_status.m_code != VknStatusCode::SUCCESSFUL) {
				destroy();
				return _status.setFail("wvkInstance::createVkInstance() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание диспетчерской таблицы уровня инстанса
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createInstanceDispatchTable();
			if (!_status) {
				destroy();
				return _status.setFail("WvkInstance::createInstanceDispatchTable() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание физических устройств
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createPhysicalDevices();
			if (!_status) {
				destroy();
				return _status.setFail("WvkInstance::createPhysicalDevices() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstance::destroy(void) noexcept {
			if (m_vk_instance != VK_NULL_HANDLE) m_wvk_instance_dispatch_table_ptr->wvkDestroyInstance(nullptr);
			if (m_wvk_global_dispatch_table_ptr != nullptr) m_wvk_global_dispatch_table_ptr->destroy();
			if (m_wvk_instance_dispatch_table_ptr != nullptr) m_wvk_instance_dispatch_table_ptr->destroy();
			for (auto& it_0 : m_wvk_physical_devices) {
				for (auto& it_1 : it_0) {
					it_1->destroy();
				}
			}
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// очистка данных
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = false;
			m_create_info = {};
			m_wvk_global_dispatch_table_ptr = nullptr;
			m_wvk_instance_dispatch_table_ptr = nullptr;
			m_wvk_physical_devices.clear();
			m_vk_instance = VK_NULL_HANDLE;
			m_layer_properties_collection.clear();
			m_extension_properties_collection.clear();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::validationCreateInfo(const WvkInstanceCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех.
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
			// Создаём WvkDispatchTable
			// Процедуры будут загружены только те, которым не нужен VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkDispatchTableCreateInfo _create_info = {};

			_status = m_wvk_global_dispatch_table_ptr->create(_create_info);
			if (!_status) {
				return _status.setFail("WvkDispatchTable::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::createInstanceDispatchTable(void) noexcept {
			WvkStatus _status;

			m_wvk_instance_dispatch_table_ptr = std::make_unique<WvkDispatchTable>();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создаём WvkDispatchTable
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkDispatchTableCreateInfo _create_info = {
				.wvk_instance_ptr = this,
			};

			_status = m_wvk_instance_dispatch_table_ptr->create(_create_info);
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "WvkDispatchTable::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::createPhysicalDevices(void) noexcept {
			WvkStatus _status;

			std::vector<std::vector<VkPhysicalDevice>> _vk_phys_device_groups;
			uint32_t _count = 0;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (build::vulkan_version == build::VulkanVersion::VERSION_10 && !build::isExtensionEnabled("VK_KHR_device_group_creation")) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Получаем количество доступных физических устройств
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~			
				auto _vk_result = m_wvk_instance_dispatch_table_ptr->wvkEnumeratePhysicalDevices(&_count, nullptr);

				if (_vk_result != VK_SUCCESS) {
					switch (_vk_result) {
					case VK_ERROR_INITIALIZATION_FAILED:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_INITIALIZATION_FAILED.";
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_DEVICE_MEMORY.";
						break;
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_HOST_MEMORY.";
						break;
					case VK_ERROR_UNKNOWN:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_UNKNOWN.";
						break;
					case VK_ERROR_VALIDATION_FAILED_EXT:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_VALIDATION_FAILED_EXT.";
						break;
					default:
						_status << "vkEnumeratePhysicalDevices is Unknown error.";
						break;
					}
					return _status.setFail("vkEnumeratePhysicalDevices is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Получаем доступные физические устройства
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<VkPhysicalDevice> _vk_phys_devices(_count);
				_vk_result = m_wvk_instance_dispatch_table_ptr->wvkEnumeratePhysicalDevices(&_count, _vk_phys_devices.data());

				if (_vk_result != VK_SUCCESS) {
					if (_vk_result == VK_INCOMPLETE) {
						_status << "vkEnumeratePhysicalDevices is VK_INCOMPLETE.";
					}
					else {
						switch (_vk_result) {
						case VK_ERROR_INITIALIZATION_FAILED:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_INITIALIZATION_FAILED.";
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_DEVICE_MEMORY.";
							break;
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_HOST_MEMORY.";
							break;
						case VK_ERROR_UNKNOWN:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_UNKNOWN.";
							break;
						case VK_ERROR_VALIDATION_FAILED_EXT:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_VALIDATION_FAILED_EXT.";
							break;
						default:
							_status << "vkEnumeratePhysicalDevices is Unknown error.";
							break;
						}
						return _status.setFail("vkEnumeratePhysicalDevices is fail.");
					}
				}

				_vk_phys_device_groups.emplace_back(std::move(_vk_phys_devices));
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Vulkan 1.1 и выше или включено расширение VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (build::vulkan_version >= build::VulkanVersion::VERSION_11 || build::isExtensionEnabled("VK_KHR_device_group_creation")) {
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Получаем количество доступных физических устройств
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				auto _vk_result = m_wvk_instance_dispatch_table_ptr->wvkEnumeratePhysicalDeviceGroups(&_count, nullptr);

				if (_vk_result != VK_SUCCESS) {
					switch (_vk_result) {
					case VK_ERROR_INITIALIZATION_FAILED:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_INITIALIZATION_FAILED.";
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_DEVICE_MEMORY.";
						break;
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_HOST_MEMORY.";
						break;
					case VK_ERROR_UNKNOWN:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_UNKNOWN.";
						break;
					case VK_ERROR_VALIDATION_FAILED_EXT:
						_status << "vkEnumeratePhysicalDevices is VK_ERROR_VALIDATION_FAILED_EXT.";
						break;
					default:
						_status << "vkEnumeratePhysicalDevices is Unknown error.";
						break;
					}
					return _status.setFail("vkEnumeratePhysicalDevices is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Получаем доступные физические устройства
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<VkPhysicalDeviceGroupProperties> _props(_count);
				_vk_result = m_wvk_instance_dispatch_table_ptr->wvkEnumeratePhysicalDeviceGroups(&_count, _props.data());

				if (_vk_result != VK_SUCCESS) {
					if (_vk_result == VK_INCOMPLETE) {
						_status << "vkEnumeratePhysicalDevices is VK_INCOMPLETE.";
					}
					else {
						switch (_vk_result) {
						case VK_ERROR_INITIALIZATION_FAILED:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_INITIALIZATION_FAILED.";
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_DEVICE_MEMORY.";
							break;
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_OUT_OF_HOST_MEMORY.";
							break;
						case VK_ERROR_UNKNOWN:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_UNKNOWN.";
							break;
						case VK_ERROR_VALIDATION_FAILED_EXT:
							_status << "vkEnumeratePhysicalDevices is VK_ERROR_VALIDATION_FAILED_EXT.";
							break;
						default:
							_status << "vkEnumeratePhysicalDevices is Unknown error.";
							break;
						}
						return _status.setFail("vkEnumeratePhysicalDevices is fail.");
					}
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Преобразуем группы физических устройств в вектор векторов
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::transform(
					_props.begin(),
					_props.end(),
					std::back_inserter(_vk_phys_device_groups),
					[](const VkPhysicalDeviceGroupProperties& group) {
						return std::vector<VkPhysicalDevice>(
							group.physicalDevices,
							group.physicalDevices + group.physicalDeviceCount
						);
					}
				);
			} // Vulkan 1.1 и выше или включено расширение VK_KHR_device_group_creation

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создаём WvkPhysicalDevice из доступных VkPhysicalDevice
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			for (const auto& it_0 : _vk_phys_device_groups) {

				WvkPhysicalDeviceUptrVec1 _wvk_phys_dev_group;
				for (const auto& it_1 : it_0) {
					
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Описываем WvkPhysicalDeviceCreateInfo
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkPhysicalDeviceCreateInfo _create_info = {
						.wvk_instance_ptr = this,
						.vk_physical_device = it_1,
					};

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Создаём
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
			// Успех.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::preparePNext(std::vector<std::unique_ptr<VkBaseInStructure, void(*)(VkBaseInStructure*)>>& pNext) const noexcept {
			WvkStatus _status;

			void* _pNext = pNext.empty() ? nullptr : pNext.back().get();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Добавление расширения VK_EXT_debug_utils, если оно включено
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~	
			if constexpr (build::isExtensionEnabled("VK_EXT_debug_utils")) {
				VkDebugUtilsMessengerCreateInfoEXT _info = {
					.sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
					.pNext = _pNext,
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
					.pfnUserCallback = &s_debugCallback,
					.pUserData = &_status
				};

				pNext.emplace_back(
					reinterpret_cast<VkBaseInStructure*>(
						new VkDebugUtilsMessengerCreateInfoEXT(_info)
						),
					[](VkBaseInStructure* p) {
						delete reinterpret_cast<VkDebugUtilsMessengerCreateInfoEXT*>(p);
					}
				);

				_pNext = pNext.back().get();
			} // if constexpr (Build::isExtensionEnabled("VK_EXT_debug_utils")

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		
		WvkStatus WvkInstance::prepareLayers(std::vector<const char*>& layer_names) const noexcept {
			WvkStatus _status;		

			layer_names.clear();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Добавляем в список слои, которые зависят от сборки
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::ranges::copy(build::getInstanceLayerNames(), std::back_inserter(layer_names));
								
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::prepareExtensions(std::vector<const char*>& extension_names) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Добавляем в список расширения, которые зависят от сборки
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::ranges::copy(build::getInstanceExtensionNames(), std::back_inserter(extension_names));
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstance::createVkInstance(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Подготовка pNext цепочки	
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<std::unique_ptr<VkBaseInStructure, void(*)(VkBaseInStructure*)>> _pNext_chain;
			preparePNext(_pNext_chain);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Подготовка слоёв
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<const char*> _layer_names;
			_status = prepareLayers(_layer_names);
			
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "WvkInstance::prepareLayer is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Подготовка расширений
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<const char*> _ext_names;
			_status = prepareExtensions(_ext_names);
			
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "WvkInstance::prepareExtensions is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Инициализация структуры VkApplicationInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkApplicationInfo _vkApplicationInfo = {};
			_vkApplicationInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
			_vkApplicationInfo.pNext = nullptr;
			_vkApplicationInfo.pApplicationName = build::application_name.data();
			_vkApplicationInfo.applicationVersion = build::application_version;
			_vkApplicationInfo.pEngineName = build::engine_name.data();
			_vkApplicationInfo.engineVersion = build::engine_version;
			_vkApplicationInfo.apiVersion = static_cast<uint32_t>(build::vulkan_version);
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Инициализация структуры VkInstanceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkInstanceCreateInfo _instanceCreateInfo = {};
			_instanceCreateInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
			_instanceCreateInfo.pNext = VK_NULL_HANDLE;
			_instanceCreateInfo.pNext = _pNext_chain.empty() ? nullptr : _pNext_chain.front().get();
			_instanceCreateInfo.flags = 0;
			_instanceCreateInfo.pApplicationInfo = &_vkApplicationInfo;
			_instanceCreateInfo.enabledLayerCount = static_cast<uint32_t>(_layer_names.size());
			_instanceCreateInfo.ppEnabledLayerNames = _layer_names.data();
			_instanceCreateInfo.enabledExtensionCount = static_cast<uint32_t>(_ext_names.size());
			_instanceCreateInfo.ppEnabledExtensionNames = _ext_names.data();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Вызов функции для создания экземпляра Vulkan
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkResult _vk_result = m_wvk_global_dispatch_table_ptr->wvkCreateInstance(&_instanceCreateInfo, VK_NULL_HANDLE, &m_vk_instance);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Обработка ошибки, если создание экземпляра не удалось
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("vkCreateInstance is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("vkCreateInstance is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("vkCreateInstance is VK_ERROR_INITIALIZATION_FAILED.");
					break;
				case VK_ERROR_LAYER_NOT_PRESENT:
					_status.append("vkCreateInstance is VK_ERROR_LAYER_NOT_PRESENT.");
					break;
				case VK_ERROR_EXTENSION_NOT_PRESENT:
					_status.append("vkCreateInstance is VK_ERROR_EXTENSION_NOT_PRESENT.");
					break;
				case VK_ERROR_INCOMPATIBLE_DRIVER:
					_status.append("vkCreateInstance is VK_ERROR_INCOMPATIBLE_DRIVER.");
					break;
				default:
					_status.append("vkCreateInstance is unknown VK_ERROR.");
					break;
				}

				// Возвращаем ошибку, если создание экземпляра не удалось
				return _status.setFail("vknCreateInstance is fail.");
			}
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Обработка ошибки при включенном расширении VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (build::isExtensionEnabled("VK_EXT_debug_utils")) {
				if (!_status) {
					return _status.setFail("wvkCreateInstance is fail.");
				};
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		VkBool32 WvkInstance::s_debugCallback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept {
			if (pUserData) {
				auto& _status = *static_cast<WvkStatus*>(pUserData);
				
				if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT) {
					_status << "[error::"; 			
				} else if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT) {
					_status << "[warning::";
				} else if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT) {
					_status << "[info::";
				} else if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT) {
					_status << "[verbose::";
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

			return VK_FALSE; // Возвращаем VK_FALSE, чтобы продолжить обработку сообщений
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev