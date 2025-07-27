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
#include "wvk_loader.h"
#include "wvk_instance.h"
#include "wvk_physical_device.h"
#include "wvk_queue_family.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkLogicalDevice::WvkLogicalDevice(void) noexcept {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkLogicalDevice::~WvkLogicalDevice(void) noexcept {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::create(const WvkLogicalDeviceCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка: если уже инициализирован, возвращаем ALREADY_INITIALIZED
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_valid) {
				return _status.set(VknStatusCode::ALREADY_INITIALIZED);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Сохраняем параметры создания
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Валидация create_info
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = validationCreateInfo();
			if (!_status) {
				reset();
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::validationCreateInfo() - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 4. Создание логического устройства
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = createVkDevice();
			if (!_status) {
				reset();
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::createVkDevice() - fail.");
			}

			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkLogicalDevice::destroy(void) noexcept {

			// ~~~~~~~~~~~~~~~~
			// 
			// ~~~~~~~~~~~~~~~~

			//m_create_info.wvk_instance_dispatch_table->vknDestroyDevice(m_vk_device, VK_NULL_HANDLE);

			// ~~~~~~~~~~~~~~~~
			// очистка данных
			// ~~~~~~~~~~~~~~~~

			m_create_info				= {};
			m_vk_device					= VK_NULL_HANDLE;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::validationCreateInfo(void) const noexcept {
			WvkStatus _status;

			if (m_create_info.wvk_logical_device_queue_create_info_collection.empty() == true) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDeviceCreateInfo::wvk_logical_device_queue_create_info_collection - empty.");
			}

			else {
				for(const auto& it_0 : m_create_info.wvk_logical_device_queue_create_info_collection) {
					if (it_0.wvk_queue_family_ptr == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDeviceCreateInfo::wvk_queue_family_ptr - nullptr.");
					}
					if (it_0.queue_count.has_value() == false) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDeviceCreateInfo::wvk_logical_device_queue_create_info_collection::queue_count - not value.");
					}
					if (it_0.priority_collection.empty() == true) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDeviceCreateInfo::wvk_logical_device_queue_create_info_collection::priority_collection - empty.");
					}
				}
			}

			if (m_create_info.wvk_instance_dispatch_table == nullptr) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDeviceCreateInfo::wvk_instance_dispatch_table - nullptr.");
			}
					
			if (m_create_info.wvk_physical_device_collection.empty() == true) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDeviceCreateInfo::wvk_physical_device_collection - empty.");
			}
					
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::prepareVkQueueCreateInfo(std::vector<VkDeviceQueueCreateInfo>& queue_create_info_collection1) const noexcept {
			WvkStatus _status;

			// Перебираем каждую пользовательскую конфигурацию очереди
			for (const auto& it_0 : m_create_info.wvk_logical_device_queue_create_info_collection) {

				// Создаём и заполняем описание очереди
				VkDeviceQueueCreateInfo vk_queue_create_info{};
				vk_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
				vk_queue_create_info.pNext = nullptr;
				vk_queue_create_info.flags = 0; // зарезервировано, всегда 0
				vk_queue_create_info.queueFamilyIndex = it_0.wvk_queue_family_ptr->getIndexFamily();		// индекс семейства очередей
				vk_queue_create_info.queueCount = it_0.queue_count.value();			// количество создаваемых очередей
				vk_queue_create_info.pQueuePriorities = it_0.priority_collection.data();	// приоритеты очередей (от 0.0f до 1.0f)

				// Добавляем в выходную коллекцию
				queue_create_info_collection1.emplace_back(std::move(vk_queue_create_info));
			}

			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLogicalDevice::createVkDevice(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Формирование списка очередей
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkDeviceQueueCreateInfo> vk_queue_create_info_vec1;
			for (const auto& it_0 : m_create_info.wvk_logical_device_queue_create_info_collection) {
				VkDeviceQueueCreateInfo vk_queue_create_info{};
				vk_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
				vk_queue_create_info.pNext = nullptr;
				vk_queue_create_info.flags = 0;
				vk_queue_create_info.queueFamilyIndex = it_0.wvk_queue_family_ptr->getIndexFamily();
				vk_queue_create_info.queueCount = it_0.queue_count.value();
				vk_queue_create_info.pQueuePriorities = it_0.priority_collection.data();
				vk_queue_create_info_vec1.emplace_back(std::move(vk_queue_create_info));
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Подготовка feature
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkPhysicalDeviceFeatures* _vk_physical_device_features = nullptr;
			std::vector<VkBaseInStructure*> _vk_base_in_collection;

#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_DISABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Vulkan 1.0 без расширения get_physical_device_properties2
			//        Используем обычную структуру VkPhysicalDeviceFeatures
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_vk_physical_device_features = &m_create_info.m_vk_physical_device_features;

#else

#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Vulkan 1.0 с расширением VK_KHR_get_physical_device_properties2
			//        Используем VkPhysicalDeviceFeatures2KHR и добавляем в цепочку
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkPhysicalDeviceFeatures2KHR _features2_khr = {
				.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
				.pNext = nullptr,
				.features = m_create_info.m_vk_physical_device_features
			};
			_vk_base_in_collection.push_back(reinterpret_cast<VkBaseInStructure*>(&_features2_khr));
			_vk_physical_device_features = nullptr;

#elif VULKAN_API_VERSION >= VULKAN_API_VERSION_11
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Vulkan 1.1 или выше — используем VkPhysicalDeviceFeatures2
			//        Добавляем в цепочку расширений
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkPhysicalDeviceFeatures2 _features2 = {
				.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
				.pNext = nullptr,
				.features = m_create_info.m_vk_physical_device_features
			};
			_vk_base_in_collection.push_back(reinterpret_cast<VkBaseInStructure*>(&_features2));
			_vk_physical_device_features = nullptr;
#endif

#endif

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Подготовка VkDeviceGroupDeviceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_EXTENSION_DISABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Расширение VK_KHR_device_group_creation отключено при Vulkan 1.0.
			//        Никакой дополнительной логики не требуется.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#else
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получаем первый физический GPU из коллекции.
			//        Этот объект будет использоваться как базовый для проверки совместимости.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			const auto& _wvk_phys_dev = m_create_info.wvk_physical_device_collection.front();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Проверяем совместимость устройств в коллекции.
			//        Метод `checkCompatibility` заполняет _compatibility.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			bool _compatibility = false;
			auto _wvk_status = _wvk_phys_dev->checkCompatibility(m_create_info.wvk_physical_device_collection, _compatibility);

			if (!_wvk_status.isOk()) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLogicalDevice::createVkDevice - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создаём вектор `VkPhysicalDevice` из коллекции WvkPhysicalDevice.
			//        Он понадобится для инициализации структуры группы устройств.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkPhysicalDevice> _vk_physical_device_collection;
			std::transform(
				m_create_info.wvk_physical_device_collection.begin(),
				m_create_info.wvk_physical_device_collection.end(),
				std::back_inserter(_vk_physical_device_collection),
				[](const WvkPhysicalDevice* wvk_phys_dev) {
					return wvk_phys_dev->getVkPhysicalDevice();
				}
			);
#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_EXTENSION_ENABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создаём структуру `VkDeviceGroupDeviceCreateInfoKHR`
			//        для расширения `VK_KHR_device_group_creation` при Vulkan 1.0.
			//        Указываем все доступные физические устройства.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkDeviceGroupDeviceCreateInfoKHR _device_group_create_info_khr = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR,
				.pNext = nullptr,
				.physicalDeviceCount = static_cast<uint32_t>(_vk_physical_device_collection.size()),
				.pPhysicalDevices = _vk_physical_device_collection.data()
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Добавляем структуру в цепочку `pNext` через обёртку VkBaseInStructure.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_vk_base_in_collection.push_back(reinterpret_cast<VkBaseInStructure*>(&_device_group_create_info_khr));
#elif VULKAN_API_VERSION >= VULKAN_API_VERSION_11
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создаём структуру `VkDeviceGroupDeviceCreateInfo`
			//        для API версии 1.1 и выше (входит в ядро).
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkDeviceGroupDeviceCreateInfoKHR _device_group_create_info = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO,
				.pNext = nullptr,
				.physicalDeviceCount = static_cast<uint32_t>(_vk_physical_device_collection.size()),
				.pPhysicalDevices = _vk_physical_device_collection.data()
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Добавляем структуру в цепочку `pNext`.
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_vk_base_in_collection.push_back(reinterpret_cast<VkBaseInStructure*>(&_device_group_create_info));
#endif
#endif

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 4. Преобразуем коллекцию wvk_feature'ов в цепочку VkBaseInStructure*
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::transform(
				m_create_info.m_wvk_logical_device_feature_collection.begin(),
				m_create_info.m_wvk_logical_device_feature_collection.end(),
				std::back_inserter(_vk_base_in_collection),
				[](const auto& wvk_wvk_logic_dev_feature) {
					return wvk_wvk_logic_dev_feature.vk_base_in;
				}
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 5. Связываем элементы коллекции через поле pNext
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			for (size_t ct_0 = 0; ct_0 + 1 < _vk_base_in_collection.size(); ++ct_0) {
				const auto& it_0 = _vk_base_in_collection[ct_0];
				const auto& it_1 = _vk_base_in_collection[ct_0 + 1];
				it_0->pNext = it_1;
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 6. Завершаем цепочку, выставляя pNext последнего элемента в nullptr
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_vk_base_in_collection.back()->pNext = nullptr;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 7. Заполнение структуры VkDeviceCreateInfo
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkDeviceCreateInfo _vk_device_create_info = {};
			_vk_device_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
			_vk_device_create_info.pNext = _vk_base_in_collection.front();
			_vk_device_create_info.flags = 0;
			_vk_device_create_info.queueCreateInfoCount = static_cast<uint32_t>(vk_queue_create_info_vec1.size());
			_vk_device_create_info.pQueueCreateInfos = vk_queue_create_info_vec1.data();
			_vk_device_create_info.enabledLayerCount = 0;
			_vk_device_create_info.ppEnabledLayerNames = nullptr;
			_vk_device_create_info.enabledExtensionCount = 0;
			_vk_device_create_info.ppEnabledExtensionNames = nullptr;
			_vk_device_create_info.pEnabledFeatures = _vk_physical_device_features;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 8. Вызов vkCreateDevice через таблицу диспетчиризации
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			auto _vk_result = m_create_info.wvk_instance_dispatch_table->wvkCreateDevice(
				m_create_info.wvk_physical_device_collection[0]->getVkPhysicalDevice(),
				&_vk_device_create_info,
				VK_NULL_HANDLE,
				&m_vk_device
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 9. Обработка результата и ошибок
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_DEVICE_LOST:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_DEVICE_LOST."); break;
				case VK_ERROR_EXTENSION_NOT_PRESENT:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_EXTENSION_NOT_PRESENT."); break;
				case VK_ERROR_FEATURE_NOT_PRESENT:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_FEATURE_NOT_PRESENT."); break;
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_INITIALIZATION_FAILED."); break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_DEVICE_MEMORY."); break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_OUT_OF_HOST_MEMORY."); break;
				case VK_ERROR_TOO_MANY_OBJECTS:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_TOO_MANY_OBJECTS."); break;
				case VK_ERROR_UNKNOWN:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_UNKNOWN."); break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = VK_ERROR_VALIDATION_FAILED_EXT."); break;
				default:
					_status.append("\n\tvkEnumerateInstanceLayerProperties = Unknown error."); break;
				}
				return _status.set(VknStatusCode::FAIL, "\n\tvknEnumerateInstanceLayerProperties - fail");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 10. Успешное завершение
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status.m_code = VknStatusCode::SUCCESSFUL;
			return _status;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkLogicalDevice::reset(void) noexcept {

		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev