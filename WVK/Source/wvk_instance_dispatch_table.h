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
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkInstanceDispatchTableCreateInfo {
			WvkInstancePtr wvk_instance = nullptr;
			WvkLoaderPtr wvk_loader = nullptr;
		}; // class WvkInstanceDispatchTableCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkInstanceDispatchTable : public GpuObject {

		public:

			// 1.0
			inline void wvkDestroyInstance(const VkAllocationCallbacks* pAllocator) const noexcept;
			inline VkResult wvkEnumeratePhysicalDevices(uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices) const noexcept;
			inline void wvkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures) const noexcept;
			inline void wvkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties) const noexcept;
			inline VkResult wvkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties) const noexcept;
			inline void wvkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties) const noexcept;
			inline void wvkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties) const noexcept;
			inline void wvkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pCount, VkQueueFamilyProperties* pProperties) const noexcept;
			inline void wvkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pCount, VkSparseImageFormatProperties* pProperties) const noexcept;
			inline VkResult wvkCreateDevice(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice) const noexcept;
			inline VkResult wvkEnumerateDeviceExtensionProperties(VkPhysicalDevice physicalDevice, const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept;
			inline VkResult wvkEnumerateDeviceLayerProperties(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept;
			inline PFN_vkVoidFunction wvkGetDeviceProcAddr(VkDevice device, const char* pName) const noexcept;
			// Vulkan 1.1
			inline void wvkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures) const noexcept;
			inline void wvkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) const noexcept;
			inline void wvkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties) const noexcept;
			inline VkResult wvkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties) const noexcept;
			inline void wvkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties) const noexcept;
			inline void wvkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pCount, VkQueueFamilyProperties2* pProperties) const noexcept;
			inline void wvkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties) const noexcept;
			inline void wvkGetPhysicalDeviceExternalBufferProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalBufferInfo* pExternalBufferInfo, VkExternalBufferProperties* pExternalBufferProperties) const noexcept;
			inline void wvkGetPhysicalDeviceExternalFenceProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalFenceInfo* pExternalFenceInfo, VkExternalFenceProperties* pExternalFenceProperties) const noexcept;
			inline void wvkGetPhysicalDeviceExternalSemaphoreProperties(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceExternalSemaphoreInfo* pExternalSemaphoreInfo, VkExternalSemaphoreProperties* pExternalSemaphoreProperties) const noexcept;

			// ~~~~~~~~~~~~~~~~
			// VkSurface 1.1
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept;

			// Vulkan 1.2
			inline VkResult wvkGetPhysicalDeviceToolProperties(VkPhysicalDevice physicalDevice, uint32_t* pToolCount, VkPhysicalDeviceToolProperties* pToolProperties) const noexcept;
			inline VkResult wvkGetPhysicalDeviceCooperativeMatrixPropertiesNV(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkCooperativeMatrixPropertiesNV* pProperties) const noexcept;
			inline VkResult wvkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(VkPhysicalDevice physicalDevice, uint32_t* pCombinationCount, VkFramebufferMixedSamplesCombinationNV* pCombinations) const noexcept;
		
		private:

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkDestroyInstance m_vkDestroyInstance = VK_NULL_HANDLE;
			PFN_vkEnumeratePhysicalDevices m_vkEnumeratePhysicalDevices = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFeatures m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFormatProperties m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceImageFormatProperties m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceMemoryProperties m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceProperties m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceQueueFamilyProperties m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSparseImageFormatProperties m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;
			PFN_vkCreateDevice m_vkCreateDevice = VK_NULL_HANDLE;
			PFN_vkEnumerateDeviceExtensionProperties m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			PFN_vkEnumerateDeviceLayerProperties m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;
			PFN_vkGetDeviceProcAddr	m_vkGetDeviceProcAddr = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.1
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceFeatures2 m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceProperties2 m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFormatProperties2 m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceImageFormatProperties2 m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceMemoryProperties2 m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceQueueFamilyProperties2 m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceExternalBufferProperties m_vkGetPhysicalDeviceExternalBufferProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceExternalFenceProperties m_vkGetPhysicalDeviceExternalFenceProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceExternalSemaphoreProperties m_vkGetPhysicalDeviceExternalSemaphoreProperties = VK_NULL_HANDLE;

			// VkSurface 1.1
			PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.2
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceToolProperties m_vkGetPhysicalDeviceToolProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV m_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV m_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = VK_NULL_HANDLE;

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
			* Метод сохраняет переданную структуру `WvkInstanceDispatchTableCreateInfo`, проверяет её на корректность (если включена валидация),
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
			* WvkInstanceDispatchTableCreateInfo info = { ... };
			* WvkInstanceDispatchTable dispatch_table;
			* WvkStatus status = dispatch_table.create(info);
			* if (!status) {
			*     std::cerr << status.what() << std::endl;
			*     return false;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkInstanceDispatchTableCreateInfo& create_info) noexcept;

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

			void reset(void) noexcept;
			
		private:

			WvkInstanceDispatchTableCreateInfo m_create_info;
		}; // class WvkInstanceDispatchTable

	} // namespace wvk

} // namespace CGDev

#include "wvk_instance_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_DISPATCH_TABLE_H
