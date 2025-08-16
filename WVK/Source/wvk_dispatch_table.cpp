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
#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS
#include "MSWindows/wvk_dispatch_table_mswindows.h"
using WvkDispatchTablePlatform = CGDev::wvk::mswindows::WvkDispatchTableMSWindows;
using WvkDispatchTablePlatformCreateInfo = CGDev::wvk::mswindows::WvkDispatchTableMSWindowsCreateInfo;
#endif

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		struct WvkDispatchTable::WvkDispatchTableImpl {
		public:
			WvkDispatchTableImpl() = default;
			~WvkDispatchTableImpl() = default;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create() {
				WvkStatus _status;

#if WVK_PLATFORM == WVK_PLATFORM_MSWINDOWS
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Описываем WvkDispatchTableMSWindows
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				mswindows::WvkDispatchTableMSWindowsCreateInfo _create_info = {};
#endif
				_status = m_dispatch_table_platform.create(_create_info);
				if (!_status) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkDispatchTablePlatform::create - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Успех
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy() {
				m_dispatch_table_platform.destroy();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			PFN_vkGetInstanceProcAddr getVkGetInstanceProcAddr(void) const noexcept {
				return m_dispatch_table_platform.getVkGetInstanceProcAddr();
			}

		private:

			WvkDispatchTablePlatform m_dispatch_table_platform;
		};

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkDispatchTable::WvkDispatchTable(void) noexcept {}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkDispatchTable::~WvkDispatchTable(void) noexcept {}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::create(const WvkDispatchTableCreateInfo& create_info) noexcept {
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
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTable::validationCreateInfo - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Загрузка указателей на Vulkan-функции
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = loadProcedure();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTable::loadProcedure - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkDispatchTable::destroy(void) noexcept {	
			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			//m_vkDestroyInstance = VK_NULL_HANDLE;

			//m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			//m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;

			m_vkGetInstanceProcAddr = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Global
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkCreateInstance = VK_NULL_HANDLE;
			m_vkEnumerateInstanceExtensionProperties = VK_NULL_HANDLE;
			m_vkEnumerateInstanceLayerProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkEnumerateInstanceVersion = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Physical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkEnumeratePhysicalDevices = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkEnumeratePhysicalDeviceGroups = VK_NULL_HANDLE;
			m_vkEnumeratePhysicalDeviceGroupsKHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_surface_capabilities2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
			m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Logical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkGetDeviceProcAddr = VK_NULL_HANDLE;
			m_vkCreateDevice = VK_NULL_HANDLE;

			// =======================================
			// [Category]: CommandPool
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkCreateCommandPool = VK_NULL_HANDLE;
			m_vkResetCommandPool = VK_NULL_HANDLE;
			m_vkDestroyCommandPool = VK_NULL_HANDLE;
			m_vkAllocateCommandBuffers = VK_NULL_HANDLE;
			m_vkFreeCommandBuffers = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_maintenance1
			// ~~~~~~~~~~~~~~~~
			m_vkTrimCommandPool = VK_NULL_HANDLE;
				m_vkTrimCommandPoolKHR = VK_NULL_HANDLE;

			// =======================================
			// [Category]: CommandBuffer
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkResetCommandBuffer = VK_NULL_HANDLE;
			m_vkBeginCommandBuffer = VK_NULL_HANDLE;
			m_vkEndCommandBuffer = VK_NULL_HANDLE;

			// =======================================
			// [Category]: Debug
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_vkCreateDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			m_vkDestroyDebugUtilsMessengerEXT = VK_NULL_HANDLE;
			m_vkSubmitDebugUtilsMessageEXT = VK_NULL_HANDLE;

			if (m_dispatch_table_impl != nullptr) m_dispatch_table_impl->destroy();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// очистка данных
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = false;
			m_dispatch_table_impl = nullptr;
			m_create_info = {};
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::validationCreateInfo(const WvkDispatchTableCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::loadProcedure(void) noexcept {
			WvkStatus _status;

			std::vector<WvkVulkanProcedureInfo> _procedures;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создаём платформо-зависимую часть и получаем PFN_vkGetInstanceProcAddr
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_dispatch_table_impl = std::make_unique<WvkDispatchTableImpl>();

			_status = m_dispatch_table_impl->create();
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkDispatchTableImpl::create is fail.");
			}

			m_vkGetInstanceProcAddr = m_dispatch_table_impl->getVkGetInstanceProcAddr();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получаем процедуры, для которых не требуется VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			// =======================================
			// [Category]: Global
			// =======================================

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumerateInstanceLayerProperties", reinterpret_cast<void**>(&m_vkEnumerateInstanceLayerProperties)));
			_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumerateInstanceExtensionProperties", reinterpret_cast<void**>(&m_vkEnumerateInstanceExtensionProperties)));
			_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateInstance", reinterpret_cast<void**>(&m_vkCreateInstance)));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// [Version] 1.1
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumerateInstanceVersion", reinterpret_cast<void**>(&m_vkEnumerateInstanceVersion)));
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Загрузка
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = loadProcedure(
				[this](const char* name){
					return m_vkGetInstanceProcAddr(VK_NULL_HANDLE, name); },
				_procedures);
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получаем процедуры, которые можно получить только при VkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.vkInstance != nullptr) {
				_procedures.clear();

				// =======================================
				// [Category]: Physical Device
				// =======================================

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumeratePhysicalDevices", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDevices)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFeatures", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceQueueFamilyProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceMemoryProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSparseImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties)));

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.1 / WVK_KHR_device_group_creation
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumeratePhysicalDeviceGroups", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroups)));
#endif
#if WVK_VULKAN_API_VERSION == WVK_VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_ENABLE
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkEnumeratePhysicalDeviceGroupsKHR", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroupsKHR)));
#endif 

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_get_physical_device_properties2
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFeatures2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceMemoryProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSparseImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2)));
#endif
#if WVK_VULKAN_API_VERSION == WVK_VULKAN_API_VERSION_10 && WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFeatures2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2KHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2KHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2KHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2KHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceMemoryProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2KHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceQueueFamilyProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2KHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSparseImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR)));
#endif 

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_get_surface_capabilities2
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR)));
#endif
#if WVK_VULKAN_API_VERSION == WVK_VULKAN_API_VERSION_10 && WVK_KHR_get_surface_capabilities2 == WVK_ENABLE
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR)));
#endif
				// =======================================
				// [Category]: Logical Device
				// =======================================

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkGetDeviceProcAddr", reinterpret_cast<void**>(&m_vkGetDeviceProcAddr)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateDevice", reinterpret_cast<void**>(&m_vkCreateDevice)));

				// =======================================
				// [Category]: Debug
				// =======================================

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// [Extension] VK_EXT_debug_utils
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#if WVK_EXT_debug_utils == WVK_ENABLE
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkCreateDebugUtilsMessengerEXT)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkDestroyDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkDestroyDebugUtilsMessengerEXT)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkSubmitDebugUtilsMessageEXT", reinterpret_cast<void**>(&m_vkSubmitDebugUtilsMessageEXT)));
#endif
 
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Загрузка
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = loadProcedure(
					[this](const char* name) {
						return m_vkGetInstanceProcAddr(m_create_info.vkInstance, name); },
						_procedures);
			}	

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получаем процедуры, которые можно получить как при VkInstance, так и при VkDevice
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.vkInstance != nullptr || m_create_info.vkDevice != nullptr) {
				_procedures.clear();

				// =======================================
				// [Category]: CommandPool
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateCommandPool", reinterpret_cast<void**>(&m_vkCreateCommandPool)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkResetCommandPool", reinterpret_cast<void**>(&m_vkResetCommandPool)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkDestroyCommandPool", reinterpret_cast<void**>(&m_vkDestroyCommandPool)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkAllocateCommandBuffers", reinterpret_cast<void**>(&m_vkAllocateCommandBuffers)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkFreeCommandBuffers", reinterpret_cast<void**>(&m_vkFreeCommandBuffers)));

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_maintenance1
				// ~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkTrimCommandPool", reinterpret_cast<void**>(&m_vkTrimCommandPool)));
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkTrimCommandPoolKHR", reinterpret_cast<void**>(&m_vkTrimCommandPoolKHR)));

				// =======================================
				// [Category]: CommandBuffer
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkResetCommandBuffer", reinterpret_cast<void**>(&m_vkResetCommandBuffer)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkBeginCommandBuffer", reinterpret_cast<void**>(&m_vkBeginCommandBuffer)));
				_procedures.emplace_back(WvkVulkanProcedureInfo("vkEndCommandBuffer", reinterpret_cast<void**>(&m_vkEndCommandBuffer)));

				if (m_create_info.vkDevice != nullptr) {
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Загрузка
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = loadProcedure(
						[this](const char* name) {
							return m_vkGetDeviceProcAddr(m_create_info.vkDevice, name); },
							_procedures);
				}
				else {
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Загрузка
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = loadProcedure(
						[this](const char* name) {
							return m_vkGetInstanceProcAddr(m_create_info.vkInstance, name); },
							_procedures);
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkDispatchTable::loadProcedure(std::function<void* (const char*)> getProc, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Перебор всех процедур и попытка загрузки каждой
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<std::string> _failed_procedures;
			for (const auto& it_0 : wvk_vulkan_procedure_collection1) {
				// Получаем адрес функции через vkGetInstanceProcAddr и записываем в targetPtr
				*it_0.targetPtr = getProc(it_0.name);

				// Если функция не найдена — запоминаем имя
				if (*it_0.targetPtr == nullptr) {
					_failed_procedures.emplace_back(it_0.name);
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Если есть ошибки, собираем сообщение и возвращаем FAIL
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (!_failed_procedures.empty()) {
				std::string _error_message = "\n\tVulkan procedures not found:";
				for (const auto& _name : _failed_procedures) {
					_error_message += "\n\t- " + _name;
				}
				return _status.set(VknStatusCode::FAIL, _error_message.c_str());
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev