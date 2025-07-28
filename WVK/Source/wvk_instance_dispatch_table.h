#ifndef CGDEV_WVK_SOURCE__WVK_INSTANCE_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE__WVK_INSTANCE_DISPATCH_TABLE_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "wvk_dispatch_table.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkInstanceDtCreateInfo {
			WvkInstancePtr wvk_instance = nullptr;
			WvkLoaderPtr wvk_loader = nullptr;
		}; // class WvkInstanceDtCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkInstanceDispatchTable : public WvkDispatchTable {

		public:

			// =======================================
			// [Category]: Physical Device
			// =======================================
			
			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumeratePhysicalDevices(uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices)const noexcept {
				return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkEnumeratePhysicalDevices, pPhysicalDeviceCount, pPhysicalDevices); }
			inline void wvkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures) const noexcept {
				m_vkGetPhysicalDeviceFeatures(physicalDevice, pFeatures); }
			inline void wvkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties) const noexcept {
				m_vkGetPhysicalDeviceProperties(physicalDevice, pProperties); }
			inline void wvkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties) const noexcept {
				m_vkGetPhysicalDeviceFormatProperties(physicalDevice, format, pFormatProperties); }
			inline VkResult wvkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties) const noexcept {
				m_vkGetPhysicalDeviceImageFormatProperties(physicalDevice, format, type, tiling, usage, flags, pImageFormatProperties); }
			inline void wvkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties) const noexcept {
				m_vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties); }
			inline void wvkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties) const noexcept {
				m_vkGetPhysicalDeviceMemoryProperties(physicalDevice, pMemoryProperties); }
			inline void wvkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties) const noexcept {
				m_vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice, format, type, samples, usage, tiling, pPropertyCount, pProperties); }

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / WVK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumeratePhysicalDeviceGroups(uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11 
				return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkEnumeratePhysicalDeviceGroups, pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_device_group_creation == WVK_EXTENSION_ENABLE 
				return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkEnumeratePhysicalDeviceGroupsKHR, pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties);
#endif
			}

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~
			inline void wvkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				m_vkGetPhysicalDeviceFeatures2(physicalDevice, pFeatures);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
				m_vkGetPhysicalDeviceFeatures2KHR(physicalDevice, pFeatures);
#endif
			}
			inline void wvkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				m_vkGetPhysicalDeviceProperties2(physicalDevice, pProperties);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
				m_vkGetPhysicalDeviceProperties2KHR(physicalDevice, pProperties);
#endif
			}
			inline void wvkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				m_vkGetPhysicalDeviceFormatProperties2(physicalDevice, format, pFormatProperties);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
				m_vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice, format, pFormatProperties);
#endif
			}
			inline VkResult wvkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				return m_vkGetPhysicalDeviceImageFormatProperties2(physicalDevice, pImageFormatInfo, pImageFormatProperties);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
				return m_vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice, pImageFormatInfo, pImageFormatProperties);
#endif
			}
			inline void wvkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				m_vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
				m_vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);
#endif
			}
			inline void wvkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				m_vkGetPhysicalDeviceMemoryProperties2(physicalDevice, pMemoryProperties);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
				m_vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice, pMemoryProperties);
#endif
			}
			inline void wvkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				m_vkGetPhysicalDeviceSparseImageFormatProperties2(physicalDevice, pFormatInfo, pPropertyCount, pProperties);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
				m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice, pFormatInfo, pPropertyCount, pProperties);
#endif
			}
			
			// ~~~~~~~~~~~~~~~~
			// [Version] 1.1 / VK_KHR_get_surface_capabilities2
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				return m_vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice, pSurfaceInfo, pSurfaceCapabilities);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_surface_capabilities2 == WVK_EXTENSION_ENABLE
				return m_vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice, pSurfaceInfo, pSurfaceCapabilities);
#endif
			}
			inline VkResult wvkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats) const noexcept {
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				return m_vkGetPhysicalDeviceSurfaceFormats2KHR(physicalDevice, pSurfaceInfo, pSurfaceFormatCount, pSurfaceFormats);
#endif
#if VULKAN_API_VERSION < VULKAN_API_VERSION_11 && WVK_KHR_get_surface_capabilities2 == WVK_EXTENSION_ENABLE
				return m_vkGetPhysicalDeviceSurfaceFormats2KHR(physicalDevice, pSurfaceInfo, pSurfaceFormatCount, pSurfaceFormats);
#endif
			}

			// =======================================
			// [Category]: Logical Device
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline PFN_vkVoidFunction wvkGetDeviceProcAddr(VkDevice device, const char* pName) const noexcept {
				return m_vkGetDeviceProcAddr(device, pName); }
			inline VkResult wvkCreateDevice(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice) const noexcept {
				return m_vkCreateDevice(physicalDevice, pCreateInfo, pAllocator, pDevice); }

			// =======================================
			// [Category]: Debug
			// =======================================
			
			// ~~~~~~~~~~~~~~~~
			// [Extension] VK_EXT_debug_utils
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateDebugUtilsMessengerEXT(const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pMessenger) const noexcept {
#if WVK_EXT_debug_utils == WVK_EXTENSION_ENABLE
				return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkCreateDebugUtilsMessengerEXT, pCreateInfo, pAllocator, pMessenger);
#endif
			}
			inline void wvkDestroyDebugUtilsMessengerEXT(VkDebugUtilsMessengerEXT messenger, const VkAllocationCallbacks* pAllocator) const noexcept {
#if WVK_EXT_debug_utils == WVK_EXTENSION_ENABLE
				return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkDestroyDebugUtilsMessengerEXT, messenger, pAllocator);
#endif
			}
			inline void wvkSubmitDebugUtilsMessageEXT(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData) const noexcept {
#if WVK_EXT_debug_utils == WVK_EXTENSION_ENABLE
				return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkSubmitDebugUtilsMessageEXT, messageSeverity, messageTypes, pCallbackData);
#endif
			}

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkInstanceDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkInstanceDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* Инициализирует таблицу вызовов Vulkan-функций для указанного `VkInstance`.
			*
			* Метод сохраняет переданную структуру `WvkInstanceDispatchTableCreateInfo`, проверяет её на корректность,
			* а затем загружает указатели на необходимые Vulkan-функции. Повторный вызов приведёт к ошибке `ALREADY_INITIALIZED`.
			*
			* @param[in] create_info
			*  Структура с параметрами и зависимостями, необходимыми для создания таблицы вызовов:
			*  - `wvk_loader` — объект загрузки функций;
			*  - `wvk_instance` — обёртка над `VkInstance`.
			*
			* @return
			*  Возвращает объект `WvkStatus`:
			*  - `OK`, если все шаги выполнены успешно;
			*  - `FAIL`, если произошла ошибка загрузки функций или валидации;
			*  - `ALREADY_INITIALIZED`, если метод уже был вызван.
			*
			* @code
			* WvkInstanceDtCreateInfo info = { ... };
			* WvkInstanceDispatchTable dispatch_table;
			* WvkStatus status = dispatch_table.create(info);
			* if (!status) {
			*     std::cerr << status.what() << std::endl;
			*     return false;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkInstanceDtCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Выполняет валидацию полей структуры `WvkInstanceDispatchTableCreateInfo`,
			* чтобы убедиться, что переданные указатели на `WvkLoader` и `WvkInstance` не равны `nullptr`.
			*
			* Метод проверяет критические зависимости и предотвращает дальнейшие вызовы с неинициализированными
			* указателями, которые могут привести к неопределённому поведению.
			*
			* @return
			*  Возвращает объект WvkStatus с кодом:
			*  - `OK`, если все необходимые поля структуры инициализированы корректно;
			*  - `FAIL`, если хотя бы одно поле (loader или instance) не задано.
			*
			* @code
			* WvkInstanceDispatchTable dispatch_table;
			* WvkStatus status = dispatch_table.validationCreateInfo();
			* if (!status) {
			*     std::cerr << status.what() << std::endl;
			*     return false;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Загружает указатели на все необходимые глобальные и физически-зависимые функции Vulkan
			* с использованием переданного загрузчика `WvkLoader` и объекта `WvkInstance`.
			*
			* Метод формирует список процедур Vulkan, которые необходимо загрузить (включая функции
			* Vulkan 1.0, 1.1 и 1.2), и передаёт его в `WvkLoader::loadProcedure`, используя
			* защищённый вызов через `invokeWithVkInstance`, чтобы не раскрывать `VkInstance`.
			*
			* @return
			*  Возвращает объект WvkStatus с кодом:
			*  - `OK`, если все процедуры были успешно загружены;
			*  - `FAIL`, если загрузка хотя бы одной процедуры завершилась неудачей.
			*
			* @code
			* WvkInstanceDispatchTable table;
			* WvkStatus status = table.loadProcedure();
			* if (!status) {
			*     std::cerr << status.what() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void reset(void) noexcept;
			
		private:

			WvkInstanceDtCreateInfo m_create_info;
		}; // class WvkInstanceDispatchTable

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_DISPATCH_TABLE_H
