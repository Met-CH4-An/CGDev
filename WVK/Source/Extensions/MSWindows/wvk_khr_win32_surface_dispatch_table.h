#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_WIN32_SURFACE_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_WIN32_SURFACE_DISPATCH_TABLE_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_mswindows.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../../wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
namespace CGDev {

	namespace wvk {

		namespace Extensions {

			namespace mswindows {
				
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct WvkKhrWin32SurfaceDispatchTableCreateInfo {
					WvkLoaderPtr wvk_loader = nullptr;
					WvkInstancePtr wvk_instance = nullptr;
				}; // WvkKhrWin32SurfaceDispatchTableCreateInfo

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class WvkKhrWin32SurfaceDispatchTable : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline static const std::string& s_getName(void) noexcept;

				private:

					inline static const std::string s_name = "VK_KHR_win32_surface";

				public:

					inline VkResult wvkCreateWin32SurfaceKHR(const VkWin32SurfaceCreateInfoKHR* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSurfaceKHR* pSurface) const noexcept;

				private:

					PFN_vkCreateWin32SurfaceKHR m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkKhrWin32SurfaceDispatchTable(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~WvkKhrWin32SurfaceDispatchTable(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* **Создаёт и инициализирует объект WvkKhrWin32SurfaceCommands.**
					*
					* @details
					* Метод принимает структуру `WvkKhrWin32SurfaceDispatchTableCreateInfo`, сохраняет её,
					* валидирует (если включена сборка с валидацией) и выполняет загрузку нужных команд Vulkan.
					*
					* Ожидается, что внутри `create_info` установлен валидный интерфейс `wvk_loader` и `wvk_instance`,
					* через который будет происходить загрузка указателя на `vkCreateWin32SurfaceKHR`.
					*
					* @param[in] create_info
					* Структура с данными, необходимыми для создания команды расширения `VK_KHR_win32_surface`.
					*
					* @return
					* Возвращает `OK` в случае успешного создания.
					* Если валидация или загрузка команды завершается неудачей — возвращает `FAIL` с диагностикой.
					*
					* @code
					* WvkKhrWin32SurfaceDispatchTable _dispatch_table;
					* WvkKhrWin32SurfaceDispatchTableCreateInfo info;
					* info.wvk_loader = &some_loader;
					*
					* WvkStatus status = _dispatch_table.create(info);
					* if (!status) {
					*     // Обработка ошибки
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus create(const WvkKhrWin32SurfaceDispatchTableCreateInfo& create_info) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void destroy(void) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const WvkKhrWin32SurfaceDispatchTableCreateInfo& getCreateInfo(void) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* **Проверяет валидность входной информации при создании WvkKhrWin32SurfaceDispatchTable.**
					*
					* @details
					* Метод выполняет базовую проверку обязательного указателя `wvk_loader` и `wvk_instance` 
					* в структуре `m_create_info`. Без этого интерфейса невозможно загрузить функции Vulkan.
					*
					* Используется, как правило, внутри метода `create`, до загрузки указателей на команды.
					*
					* @return
					* Возвращает объект `WvkStatus`. Если проверка прошла успешно — возвращается `OK`.
					* Если `wvk_commands == nullptr`, возвращается `FAIL` с диагностическим сообщением.
					*
					* @code
					* WvkKhrWin32SurfaceDispatchTable _table;
					* WvkKhrWin32SurfaceDispatchTableCreateInfo create_info;
					* create_info.wvk_loader = ...;
					* _table.create(create_info); // Внутри будет вызван validationCreateInfo
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus validationCreateInfo(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* Загружает указатель на функцию vkCreateWin32SurfaceKHR через WvkLoader.
					*
					* Метод подготавливает список процедур Vulkan, которые нужно загрузить,
					* и использует метод `invokeWithVkInstanceMethod` из `WvkInstance`, чтобы
					* загрузить указатель на функцию `vkCreateWin32SurfaceKHR` в таблицу вызовов.
					*
					* @return
					* Возвращает статус операции типа WvkStatus. В случае успеха — статус OK,
					* в противном случае содержит код ошибки и сообщение об ошибке.
					*
					* @code
					* WvkKhrWin32SurfaceDispatchTable table;
					* WvkStatus status = table.loadVulkanCommand();
					* if (!status) {
					*     std::cerr << status.what() << std::endl;
					* }
					* @endcode
					*
					* @warning
					* Если поле `m_create_info.wvk_instance` или `m_create_info.wvk_loader` не инициализировано,
					* произойдёт ошибка доступа к памяти. Убедись, что структура `m_create_info` корректно
					* настроена до вызова этого метода.
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus loadVulkanCommand(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void reset(void) noexcept;

				private:

					WvkKhrWin32SurfaceDispatchTableCreateInfo m_create_info = {};
				}; // class WvkKhrWin32SurfaceDispatchTable

			} // namespace mswindows

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_khr_win32_surface_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_WIN32_SURFACE_DISPATCH_TABLE_H
