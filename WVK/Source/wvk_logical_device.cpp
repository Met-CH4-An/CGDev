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
#include "wvk_logical_device.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
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
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::validationCreateInfo() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание логического устройства
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = create();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание таблицы диспетчиризации
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createWvkDispatchTable();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
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
			// очистка данных
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
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::prepareFeatures(std::vector<std::unique_ptr<VkBaseInStructure, void(*)(VkBaseInStructure*)>>& pNext) const noexcept {
			WvkStatus _status;

			VkBaseInStructure* _pNext_last = pNext.empty() ? nullptr : pNext.back().get();

			auto append = [&](auto&& feature) {
				VkBaseInStructure* newNode = pNext.emplace_back(
					reinterpret_cast<VkBaseInStructure*>(new std::decay_t<decltype(feature)>(feature)),
					[](VkBaseInStructure* ptr) { delete ptr; }
				).get();

				if (_pNext_last != nullptr) {
					_pNext_last->pNext = newNode;
				}
				_pNext_last = newNode;
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Добавляем в список фичи, зависящие от сборки
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (build::vulkan_version == build::VulkanVersion::VERSION_10 && !build::isExtensionEnabled("VK_KHR_get_physical_device_properties2")) {
			}
			else if constexpr (build::vulkan_version == build::VulkanVersion::VERSION_10 && build::isExtensionEnabled("VK_KHR_get_physical_device_properties2")) {
				VkPhysicalDeviceFeatures2KHR _features = {
					.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
					.pNext = nullptr,
					.features = m_create_info.m_vk_physical_device_features
				};
				append(_features);
			}
			else if constexpr (build::vulkan_version == build::VulkanVersion::VERSION_11 && build::isExtensionEnabled("VK_KHR_get_physical_device_properties2")) {
				VkPhysicalDeviceFeatures2 _features = {
					.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
					.pNext = nullptr,
					.features = m_create_info.m_vk_physical_device_features
				};
				append(_features);
			}
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// VK_EXT_shader_object
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (build::isExtensionEnabled("VK_EXT_shader_object")) {
				VkPhysicalDeviceShaderObjectFeaturesEXT _feature{
					.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT,
					.pNext = nullptr,
					.shaderObject = VK_TRUE,
				};
				append(_feature);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// VK_KHR_dynamic_rendering
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (build::isExtensionEnabled("VK_KHR_dynamic_rendering")) {
				VkPhysicalDeviceDynamicRenderingFeaturesKHR _feature{
					.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR,
					.pNext = nullptr,
					.dynamicRendering = VK_TRUE,
				};
				append(_feature);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::prepareVkQueueCreateInfo(std::vector<VkDeviceQueueCreateInfo>& queue_create_infos) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Формирование структур VkDeviceQueueCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			for (const auto& it_0 : m_create_info.wvk_logical_device_queue_create_infos) {
				VkDeviceQueueCreateInfo vk_queue_create_info {
					.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
					.pNext = nullptr,
					.flags = 0,
					.queueFamilyIndex = it_0.index.value(),
					.queueCount = it_0.queue_count.value(),
					.pQueuePriorities = it_0.priority_collection.data(),
				};	

				// Добавляем в выходную коллекцию
				queue_create_infos.emplace_back(std::move(vk_queue_create_info));
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::prepareExtensions(std::vector<const char*>& extension_names) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Добавляем в список расширения, которые зависят от сборки
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::ranges::copy(build::getDeviceExtensionNames(), std::back_inserter(extension_names));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Добавляем в список расширения, которые задаются при создании логического устройства
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
			// Успех.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::create(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Забираем из WvkInstance физическое устройство
			// заданное в WvkLogicalDeviceCreateInfo
			// physical_device_group_index - группа
			// physical_device_indices - номер в группе
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			const auto& _group_index = m_create_info.physical_device_group_index.value();
			const auto& _in_group_index = m_create_info.physical_device_indices[0];
			const auto& _wvk_phys_dev = m_create_info.wvk_instance_ptr->getWvkPhysicalDevices()[_group_index][_in_group_index];
			const auto& _dispatch_table_ptr = m_create_info.wvk_instance_ptr->getWvkDispatchTable();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Подготовка feature
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<std::unique_ptr<VkBaseInStructure, void(*)(VkBaseInStructure*)>> _pNext;
			prepareFeatures(_pNext);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Формирование списка очередей
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkDeviceQueueCreateInfo> vk_queue_create_infos;
			prepareVkQueueCreateInfo(vk_queue_create_infos);
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Подготовка VkDeviceGroupDeviceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/*#if WVK_VULKAN_API_VERSION == WVK_VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_DISABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Расширение VK_KHR_device_group_creation отключено при Vulkan 1.0.
			//        Никакой дополнительной логики не требуется.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#else
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получаем первый физический GPU из коллекции.
			//        Этот объект будет использоваться как базовый для проверки совместимости.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			const auto& wvk_phys_devs = m_create_info.wvk_instance_ptr->getWvkPhysicalDevices()[m_create_info.physical_device_group_index.value()];

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Проверяем совместимость устройств в коллекции.
			//        Метод `checkCompatibility` заполняет _compatibility.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			bool _compatibility = false;
			//auto _wvk_status = _wvk_phys_dev->checkCompatibility(m_create_info.wvk_physical_devices, _compatibility);

			//if (!_wvk_status.isOk()) {
			//	return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::create - fail.");
			//}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создаём вектор `VkPhysicalDevice` из коллекции WvkPhysicalDevice.
			//        Он понадобится для инициализации структуры группы устройств.
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
			// Создаём структуру `VkDeviceGroupDeviceCreateInfoKHR`
			//        для расширения `VK_KHR_device_group_creation` при Vulkan 1.0.
			//        Указываем все доступные физические устройства.
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
			// Создаём структуру `VkDeviceGroupDeviceCreateInfo`
			//        для API версии 1.1 и выше (входит в ядро).
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkDeviceGroupDeviceCreateInfoKHR _device_group_create_info = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO,
				.pNext = _pNext,
				.physicalDeviceCount = static_cast<uint32_t>(_vk_physical_device_collection.size()),
				.pPhysicalDevices = _vk_physical_device_collection.data()
			};
			_pNext = &_device_group_create_info;
#endif
#endif*/

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Подготовка расширений
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<const char*> _ext_names;
			_status = prepareExtensions(_ext_names);

			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\nWvkLogicalDevice::prepareExtensions() is fail.");
			}
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 7. Заполнение структуры VkDeviceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkDeviceCreateInfo _vk_device_create_info = {};
			_vk_device_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
			_vk_device_create_info.pNext = _pNext.empty() ? nullptr : _pNext.front().get();
			_vk_device_create_info.flags = 0;
			_vk_device_create_info.queueCreateInfoCount = static_cast<uint32_t>(vk_queue_create_infos.size());
			_vk_device_create_info.pQueueCreateInfos = vk_queue_create_infos.data();
			_vk_device_create_info.enabledLayerCount = 0;
			_vk_device_create_info.ppEnabledLayerNames = nullptr;
			_vk_device_create_info.enabledExtensionCount = static_cast<uint32_t>(_ext_names.size());
			_vk_device_create_info.ppEnabledExtensionNames = _ext_names.data();
			//_vk_device_create_info.pEnabledFeatures = _pEnabledFeatures;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание vkDevice
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
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status.m_code = VknStatusCode::SUCCESSFUL;
			return _status;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::createWvkDispatchTable(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Выделяем память,
			// описываем WvkDispatchTableCreateInfo
			// и создаём WvkDispatchTable
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_wvk_dispatch_table_ptr = std::make_unique<WvkDispatchTable>();
			
			WvkDispatchTableCreateInfo _create_info = {
				.wvk_instance_ptr = m_create_info.wvk_instance_ptr,
				.wvk_logical_device_ptr = this,
			};

			_status = m_wvk_dispatch_table_ptr->create(_create_info);

			if (!_status) {
				return _status.setFail("\nWvkDispatchTable::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
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