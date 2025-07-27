#ifndef CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_H
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
		struct WvkLoaderDispatchTableCreateInfo {
			WvkLoaderPtr wvk_loader = nullptr;
		}; // struct WvkLoaderDispatchTableCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkLoaderDispatchTable : public GpuObject {

		public:

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) const noexcept;
			inline VkResult wvkEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept;
			inline VkResult wvkEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.1
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumerateInstanceVersion(uint32_t* pApiVersion) const noexcept;

		private:

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateInstance                           m_vkCreateInstance = VK_NULL_HANDLE;
			PFN_vkEnumerateInstanceExtensionProperties     m_vkEnumerateInstanceExtensionProperties = VK_NULL_HANDLE;
			PFN_vkEnumerateInstanceLayerProperties         m_vkEnumerateInstanceLayerProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.1
			// ~~~~~~~~~~~~~~~~
			PFN_vkEnumerateInstanceVersion                 m_vkEnumerateInstanceVersion = VK_NULL_HANDLE;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkLoaderDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkLoaderDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* Создаёт таблицу диспетчеризации функций Vulkan на основе переданной информации.
			*
			* @details
			* Метод выполняет проверку на повторную инициализацию, сохраняет параметры,
			* проверяет структуру входных данных,
			* инициализирует таблицу функций через внутреннюю процедуру загрузки.
			*
			* @param[in] create_info
			* Структура с параметрами создания таблицы загрузки функций.
			*
			* @return
			* Возвращает статус операции, отражающий успех или причину неудачи.
			*
			* @retval VknStatusCode::OK
			* Таблица успешно создана.
			* @retval VknStatusCode::ALREADY_INITIALIZED
			* Таблица уже была создана ранее.
			* @retval VknStatusCode::FAIL
			* Ошибка валидации или загрузки процедур.
			*
			* @code
			* WvkLoaderDispatchTable table;
			* WvkLoaderDispatchTableCreateInfo info = { /* параметры * / };
			* WvkStatus status = table.create(info);
			* if (!status) {
			*     std::cerr << "Ошибка создания таблицы диспетчеризации: " << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkLoaderDispatchTableCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* * Выполняет проверку валидности структуры `WvkLoaderDispatchTableCreateInfo`.
			*
			* @details
			* * Метод проверяет, что в структуре `m_create_info` указатель `wvk_loader` не равен `nullptr`.
			* * Если указатель валиден, возвращается успешный статус. Иначе — ошибка с соответствующим сообщением.
			*
			* @return
			* * Объект `WvkStatus`, отражающий результат проверки.
			*
			* @retval VknStatusCode::OK
			* * Все обязательные поля заданы корректно.
			* @retval VknStatusCode::FAIL
			* * Одно из обязательных полей не задано (в частности, `wvk_loader == nullptr`).
			*
			* @code
			* WvkLoaderDispatchTable table;
			* // ... заполнение table.m_create_info.wvk_loader ...
			* WvkStatus status = table.validationCreateInfo();
			* if (!status) {
			*     std::cerr << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*  Загружает указатели на глобальные Vulkan-функции через `WvkLoader`.
			*
			* @details
			*  Метод вызывает `WvkLoader::loadProcedure`, передавая список процедур
			*  начальной загрузки Vulkan API (глобальные функции до создания экземпляра).
			*  Каждой функции сопоставляется указатель, в который будет записан результат.
			*
			* @return
			*  Статус выполнения. Если хотя бы одна процедура не загружена — возвращается FAIL.
			*
			* @code
			*  WvkLoaderDispatchTable dispatch_table;
			*  WvkStatus status = dispatch_table.loadProcedure();
			*  if (!status) {
			*      // Обработка ошибки загрузки процедур
			*  }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(void) noexcept;

			void reset(void) noexcept;

		private:

			WvkLoaderDispatchTableCreateInfo m_create_info;
		}; // class WvkLoaderDispatchTable

	} // namespace wvk

} // namespace CGDev

#include "wvk_loader_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_H
