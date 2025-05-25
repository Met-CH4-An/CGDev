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
#include "wvk_instance.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstanceDispatchTable::WvkInstanceDispatchTable(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstanceDispatchTable::~WvkInstanceDispatchTable(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstanceDispatchTable::create(const WvkInstanceDispatchTableCreateInfo& create_info) noexcept {
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

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstanceDispatchTable::destroy(void) noexcept {
			reset();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstanceDispatchTable::validationCreateInfo(void) const noexcept {
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

		WvkStatus WvkInstanceDispatchTable::loadProcedure(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Формируем список процедур Vulkan, которые нужно загрузить
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<WvkVulkanProcedureInfo> _procedures = {
				// ~~~~~~~~~~~~~~~~
				// Vulkan 1.0
				// ~~~~~~~~~~~~~~~~
				{ "vkDestroyInstance", reinterpret_cast<void**>(&m_vkDestroyInstance) },
				{ "vkEnumeratePhysicalDevices", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDevices) },
				{ "vkGetPhysicalDeviceFeatures", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures) },
				{ "vkGetPhysicalDeviceFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties) },
				{ "vkGetPhysicalDeviceImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties) },
				{ "vkGetPhysicalDeviceMemoryProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties) },
				{ "vkGetPhysicalDeviceProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties) },
				{ "vkGetPhysicalDeviceQueueFamilyProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties) },
				{ "vkGetPhysicalDeviceSparseImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties) },
				{ "vkCreateDevice", reinterpret_cast<void**>(&m_vkCreateDevice) },
				{ "vkEnumerateDeviceExtensionProperties", reinterpret_cast<void**>(&m_vkEnumerateDeviceExtensionProperties) },
				{ "vkEnumerateDeviceLayerProperties", reinterpret_cast<void**>(&m_vkEnumerateDeviceLayerProperties) },
				{ "vkGetDeviceProcAddr", reinterpret_cast<void**>(&m_vkGetDeviceProcAddr) },

				// ~~~~~~~~~~~~~~~~
				// Vulkan 1.1
				// ~~~~~~~~~~~~~~~~
				{ "vkGetPhysicalDeviceFeatures2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2) },
				{ "vkGetPhysicalDeviceProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2) },
				{ "vkGetPhysicalDeviceFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2) },
				{ "vkGetPhysicalDeviceImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2) },
				{ "vkGetPhysicalDeviceMemoryProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2) },
				{ "vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2) },
				{ "vkGetPhysicalDeviceSparseImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2) },
				{ "vkGetPhysicalDeviceExternalBufferProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceExternalBufferProperties) },
				{ "vkGetPhysicalDeviceExternalFenceProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceExternalFenceProperties) },
				{ "vkGetPhysicalDeviceExternalSemaphoreProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceExternalSemaphoreProperties) },

				// ~~~~~~~~~~~~~~~~
				// VkSurface 1.1
				// ~~~~~~~~~~~~~~~~
				{ "vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR) },
			
				// ~~~~~~~~~~~~~~~~
				// Vulkan 1.2
				// ~~~~~~~~~~~~~~~~
				{ "vkGetPhysicalDeviceToolProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceToolProperties) },
				{ "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV) },
				{ "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV) }
			};

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

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstanceDispatchTable::reset(void) noexcept {
			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkDestroyInstance = VK_NULL_HANDLE;
			m_vkEnumeratePhysicalDevices = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;
			m_vkCreateDevice = VK_NULL_HANDLE;
			m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;
			m_vkGetDeviceProcAddr = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.1
			// ~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceExternalBufferProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceExternalFenceProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceExternalSemaphoreProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.2
			// ~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceToolProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = VK_NULL_HANDLE;
			
			m_create_info = {};

			m_valid = false;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev
