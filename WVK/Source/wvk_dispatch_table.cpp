////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_dispatch_table.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkDispatchTable::WvkDispatchTable(void) noexcept {}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkDispatchTable::~WvkDispatchTable(void) noexcept {}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::create(const WvkDispatchTableCreateInfo& create_info) noexcept {
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
			// Шаг 3. Валидация входных параметров
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = validationCreateInfo();
			if (!_status) {
				reset(); // очищаем внутреннее состояние
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTable::validationCreateInfo - fail.");
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

		void WvkDispatchTable::destroy(void) noexcept {
			reset();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::validationCreateInfo(void) const noexcept {
			WvkStatus _status;


			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Все поля валидны — возвращаем OK
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::loadProcedure(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 4. Успешное завершение
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		WvkStatus WvkDispatchTable::loadProcedure(std::function<void* (const char*)>& get_addr, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) noexcept {
			WvkStatus _status;

			std::vector<std::string> _failed_procedures;
			for (const auto& it_0 : wvk_vulkan_procedure_collection1) {
				*it_0.targetPtr = get_addr(it_0.name);

				// Если функция не найдена — запоминаем имя
				if (*it_0.targetPtr == nullptr) {
					_failed_procedures.emplace_back(it_0.name);
				}
			}

			if (!_failed_procedures.empty()) {
				std::string _error_message = "\n\tVulkan procedures not found:";
				for (const auto& _name : _failed_procedures) {
					_error_message += "\n\t- " + _name;
				}
				return _status.set(VknStatusCode::FAIL, _error_message.c_str());
			}


			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 4. Успешное завершение
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkDispatchTable::reset(void) noexcept {
			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkDestroyInstance = VK_NULL_HANDLE;

			m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Global
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkGetInstanceProcAddr = VK_NULL_HANDLE;
			m_vkCreateInstance = VK_NULL_HANDLE;
			m_vkEnumerateInstanceExtensionProperties = VK_NULL_HANDLE;
			m_vkEnumerateInstanceLayerProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~
			m_vkEnumerateInstanceVersion = VK_NULL_HANDLE;

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
			// [Version] 1.1 / VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~
			m_vkEnumeratePhysicalDeviceGroups = VK_NULL_HANDLE;
				m_vkEnumeratePhysicalDeviceGroupsKHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceFeatures2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceFormatProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceImageFormatProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceQueueFamilyProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceMemoryProperties2KHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_surface_capabilities2
			// ~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Logical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkGetDeviceProcAddr = VK_NULL_HANDLE;
			m_vkCreateDevice = VK_NULL_HANDLE;

			// =======================================
			// [Category]: CommandPool
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkCreateCommandPool = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Debug
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~
			m_vkCreateDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			m_vkDestroyDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			m_vkSubmitDebugUtilsMessageEXT = VK_NULL_HANDLE;

			m_create_info = {};

			m_valid = false;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev