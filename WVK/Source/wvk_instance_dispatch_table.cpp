////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_instance_dispatch_table.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_loader.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstanceDt::WvkInstanceDt(void) noexcept {}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstanceDt::~WvkInstanceDt(void) noexcept {}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstanceDt::create(const WvkInstanceDtCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка на повторную инициализацию
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_valid) {
				return _status.set(VknStatusCode::ALREADY_INITIALIZED);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Сохраняем параметры создания
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Валидация входных параметров (если включена)
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (Build::ValidationBuildInfo::enable) {
				_status = validationCreateInfo();
				if (!_status) {
					reset(); // очищаем внутреннее состояние
					return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTable::validationCreateInfo - fail.");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 4. Загрузка указателей на Vulkan-функции
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = loadProcedure();
			if (!_status) {
				reset(); // очищаем внутреннее состояние
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTable::loadProcedure - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 5. Устанавливаем флаг успешной инициализации
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;

			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstanceDt::destroy(void) noexcept {
			reset();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstanceDt::validationCreateInfo(void) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка указателя на WvkLoader
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.wvk_loader == nullptr) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTableCreateInfo::wvk_loader - nullptr.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Проверка указателя на WvkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			else if (m_create_info.wvk_instance == nullptr) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTableCreateInfo::wvk_instance - nullptr.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Все поля валидны — возвращаем OK
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstanceDt::loadProcedure(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Формируем список процедур Vulkan, которые нужно загрузить
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<WvkVulkanProcedureInfo> _procedures;
				
			// ~~~~~~~~~~~~~~~~
			// 1.0
			// ~~~~~~~~~~~~~~~~

			_procedures.emplace_back("vkDestroyInstance", reinterpret_cast<void**>(&m_vkDestroyInstance));
			_procedures.emplace_back("vkCreateDevice", reinterpret_cast<void**>(&m_vkCreateDevice));
			_procedures.emplace_back("vkEnumerateDeviceExtensionProperties", reinterpret_cast<void**>(&m_vkEnumerateDeviceExtensionProperties));
			_procedures.emplace_back("vkEnumerateDeviceLayerProperties", reinterpret_cast<void**>(&m_vkEnumerateDeviceLayerProperties));
			_procedures.emplace_back("vkGetDeviceProcAddr", reinterpret_cast<void**>(&m_vkGetDeviceProcAddr));

			// =======================================
			// [Category]: Physical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~						
			_procedures.emplace_back("vkEnumeratePhysicalDevices", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDevices));
			_procedures.emplace_back("vkGetPhysicalDeviceFeatures", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures));
			_procedures.emplace_back("vkGetPhysicalDeviceProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties));
			_procedures.emplace_back("vkGetPhysicalDeviceFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties));
			_procedures.emplace_back("vkGetPhysicalDeviceImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties));
			_procedures.emplace_back("vkGetPhysicalDeviceMemoryProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties));
			_procedures.emplace_back("vkGetPhysicalDeviceQueueFamilyProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties));
			_procedures.emplace_back("vkGetPhysicalDeviceSparseImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties));

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {
				_procedures.emplace_back("vkEnumeratePhysicalDeviceGroups", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroups));
			}

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_device_group_creation
				// ~~~~~~~~~~~~~~~~			
				if constexpr (Build::WvkBuildInfo::find("VK_KHR_device_group_creation")) {
					_procedures.emplace_back("vkEnumeratePhysicalDeviceGroupsKHR", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroupsKHR));
				}
				
			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {
				_procedures.emplace_back("vkGetPhysicalDeviceFeatures2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2));
				_procedures.emplace_back("vkGetPhysicalDeviceProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2));
				_procedures.emplace_back("vkGetPhysicalDeviceFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2));
				_procedures.emplace_back("vkGetPhysicalDeviceImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2));
				_procedures.emplace_back("vkGetPhysicalDeviceMemoryProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2));
				_procedures.emplace_back("vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2));
				_procedures.emplace_back("vkGetPhysicalDeviceSparseImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2));
			}

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_get_physical_device_properties2
				// ~~~~~~~~~~~~~~~~	
				if constexpr (Build::WvkBuildInfo::find("VK_KHR_get_physical_device_properties2")) {
					_procedures.emplace_back("vkGetPhysicalDeviceFeatures2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2KHR));
					_procedures.emplace_back("vkGetPhysicalDeviceProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2KHR));
					_procedures.emplace_back("vkGetPhysicalDeviceFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2KHR));
					_procedures.emplace_back("vkGetPhysicalDeviceImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2KHR));
					_procedures.emplace_back("vkGetPhysicalDeviceMemoryProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2KHR));
					_procedures.emplace_back("vkGetPhysicalDeviceQueueFamilyProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2KHR));
					_procedures.emplace_back("vkGetPhysicalDeviceSparseImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR));
				}
				
			// ~~~~~~~~~~~~~~~~
			// VkSurface 1.1
			// ~~~~~~~~~~~~~~~~
			_procedures.emplace_back("vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR));
			

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Загружаем процедуры через WvkLoader с передачей VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
				&WvkLoader::loadProcedure,          // Указатель на метод, который загружает процедуры
				m_create_info.wvk_loader,           // Объект WvkLoader
				_procedures                         // Список процедур
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Проверка статуса после загрузки
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLoader::loadProcedure - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 4. Успешное завершение
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstanceDt::reset(void) noexcept {
			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkDestroyInstance = VK_NULL_HANDLE;

			m_vkCreateDevice = VK_NULL_HANDLE;
			m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;
			m_vkGetDeviceProcAddr = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Physical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkEnumeratePhysicalDevices = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~
			m_vkEnumeratePhysicalDeviceGroups = VK_NULL_HANDLE;

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_device_group_creation
				// ~~~~~~~~~~~~~~~~
				m_vkEnumeratePhysicalDeviceGroupsKHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_get_physical_device_properties2
				// ~~~~~~~~~~~~~~~~
				m_vkGetPhysicalDeviceFeatures2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceFormatProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceImageFormatProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceQueueFamilyProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceMemoryProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 or VK_KHR_get_surface_capabilities2
			// ~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;

			m_create_info = {};

			m_valid = false;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev