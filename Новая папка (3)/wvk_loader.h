#ifndef CGDEV_WVK_SOURCE__WVK_LOADER_H
#define CGDEV_WVK_SOURCE__WVK_LOADER_H
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
		struct WvkVulkanProcedureInfo {
			const char* name;       ///< Название функции Vulkan, например "vkDestroySurfaceKHR"
			void** targetPtr;       ///< Указатель на переменную, куда будет сохранён загруженный адрес
			WvkVulkanProcedureInfo(const char* n, void** ptr)
				: name(n), targetPtr(ptr) {
			}
		}; // struct WvkVulkanProcedureInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkLoaderCreateInfo {
		}; // WvkLoaderCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkLoader : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkLoader(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkLoader(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* * Инициализирует загрузчик Vulkan-функций и подготавливает внутреннюю реализацию.
			*
			* @details
			* Метод выполняет инициализацию на основе структуры `WvkLoaderCreateInfo`.
			* Выполняет проверку входных данных.
			* После успешной инициализации создаётся внутренняя реализация (`WvkLoaderImpl`)
			* и запрашивается функция `vkGetInstanceProcAddr`. Если любой шаг завершается неудачей —
			* вызывается `reset()` и возвращается статус ошибки.
			*
			* @param[in] create_info
			*	Структура с параметрами инициализации загрузчика.
			*
			* @return
			*	Объект `WvkStatus`, указывающий результат выполнения операции.
			*
			* @retval VknStatusCode::OK
			*	Инициализация завершена успешно.
			* @retval VknStatusCode::ALREADY_INITIALIZED
			*	Метод вызван повторно, загрузчик уже инициализирован.
			* @retval VknStatusCode::FAIL
			*	Произошла ошибка при одном из этапов инициализации.
			*
			* @code
			* WvkLoaderCreateInfo create_info = {};
			* create_info.log = &logger;
			* create_info.vkn_commands = &vkn_commands;
			* 
			* WvkLoader loader;
			* WvkStatus status = loader.create(create_info);
			* if (!status) {
			*     std::cerr << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkLoaderCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Загрузка процедур Vulkan уровня загрузчика (loader-level) с использованием vkGetInstanceProcAddr.
			*
			* @param[in,out] wvk_vulkan_procedure_collection1
			* * Контейнер с описаниями процедур Vulkan (имя + указатель на место хранения адреса).
			*
			* @return
			* * Объект WvkStatus, содержащий код и сообщение об ошибке, если загрузка не удалась.
			*
			* @code
			* std::vector<WvkVulkanProcedureInfo> procedures = {
			*     { "vkCreateInstance", reinterpret_cast<void**>(&m_vkCreateInstance) },
			*     { "vkEnumerateInstanceVersion", reinterpret_cast<void**>(&m_vkEnumerateInstanceVersion) }
			* };
			* _status = wvk_loader->loadProcedure(procedures);
			* if (!_status) {
			*     // Обработка ошибки
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* * Загружает указатели на Vulkan-функции для заданного экземпляра Vulkan.
			*
			* @details
			* * Метод получает адреса функций Vulkan с помощью vkGetInstanceProcAddr и записывает
			* * их в указатели, заданные в коллекции `wvk_vulkan_procedure_collection1`.
			* * В случае неудачи по любой из процедур возвращает статус с кодом FAIL и списком
			* * не найденных функций.
			*
			* @param[in] vk_instance
			* * Хэндл Vulkan-инстанса, для которого нужно загрузить функции.
			*
			* @param[in] wvk_vulkan_procedure_collection1
			* * Коллекция описателей процедур, содержащих имена функций и указатели для записи.
			*
			* @return
			* * Объект WvkStatus, описывающий результат выполнения операции.
			*
			* @retval VknStatusCode::OK
			* * Все функции успешно загружены.
			* @retval VknStatusCode::FAIL
			* * Одна или несколько функций не были найдены, подробности — в сообщении.
			*
			* @code
			* std::vector<WvkVulkanProcedureInfo> procs = {
			*     { "vkDestroyInstance", reinterpret_cast<void**>(&vkDestroyInstance) },
			*     { "vkEnumeratePhysicalDevices", reinterpret_cast<void**>(&vkEnumeratePhysicalDevices) }
			* };
			* 
			* WvkStatus status = loader.loadProcedure(instance, procs);
			* if (!status) {
			*     std::cerr << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(VkInstance vk_instance, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Загрузка процедур Vulkan уровня загрузчика (loader-level) с использованием vkGetInstanceProcAddr.
			*
			* @param[in,out] wvk_vulkan_procedure_collection1
			* * Контейнер с описаниями процедур Vulkan (имя + указатель на место хранения адреса).
			*
			* @return
			* * Объект WvkStatus, содержащий код и сообщение об ошибке, если загрузка не удалась.
			*
			* @code
			* std::vector<WvkVulkanProcedureInfo> procedures = {
			*     { "vkCreateInstance", reinterpret_cast<void**>(&m_vkCreateInstance) },
			*     { "vkEnumerateInstanceVersion", reinterpret_cast<void**>(&m_vkEnumerateInstanceVersion) }
			* };
			* _status = wvk_loader->loadProcedure(procedures);
			* if (!_status) {
			*     // Обработка ошибки
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(VkInstance vk_instance, VkDevice vk_device, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) const noexcept;
		
		// hpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkLoaderCreateInfo& getCreateInfo(void) const noexcept;

		// hpp
		private:

		// cpp
		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* * Сбрасывает внутреннее состояние загрузчика `WvkLoader` в начальное состояние.
			*
			* @details
			* * Метод освобождает ресурсы, очищает структуру `create_info`, сбрасывает указатель
			*   на `vkGetInstanceProcAddr`, удаляет реализацию загрузчика и помечает объект как невалидный.
			*
			* * Вызывается при ошибках в `create`, либо при переинициализации загрузчика.
			*
			* @code
			* WvkLoader loader;
			* // ... после использования ...
			* loader.reset(); // очистка и освобождение всех ресурсов
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void reset(void) noexcept;

		private:

			WvkLoaderCreateInfo m_create_info = {};
			PFN_vkGetInstanceProcAddr m_vkGetInstanceProcAddr = VK_NULL_HANDLE;
			struct WvkLoaderImpl;
			std::unique_ptr<WvkLoaderImpl> m_loader_impl = nullptr;
		}; // class WvkLoader

	} // namespace wvk

} // namespace CGDev

#include "wvk_loader.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_LOADER_H
