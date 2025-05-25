#ifndef CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS__WVK_SURFACE_MSWINDOWS_H
#define CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS__WVK_SURFACE_MSWINDOWS_H
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
#include "../wvk_surface.h"
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
				struct WvkSurfaceMSWindowsCreateInfo : public WvkSurfacePlatformCreateInfo {
					HINSTANCE hInstance = NULL;
					HWND hWnd = NULL;
					WvkKhrWin32SurfaceDispatchTablePtr wvk_khr_win32_surface_dispatch_table = nullptr;
				}; // struct WvkSurfaceMSWindowsCreateInfo

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class WvkSurfaceMSWindows : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkSurfaceMSWindows(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~WvkSurfaceMSWindows(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* Создаёт Vulkan surface для Win32-платформы на основе переданной структуры `WvkSurfaceMSWindowsCreateInfo`.
					*
					* @details
					* Метод сохраняет переданную структуру создания, при включённой сборке валидации выполняет проверку входных данных.
					* В случае успешной валидации создаёт Win32-surface с помощью `vkCreateWin32SurfaceKHR`. При ошибке — сбрасывает состояние объекта.
					*
					* Валидация и создание идут строго пошагово, чтобы гарантировать корректность структуры перед вызовом Vulkan API.
					*
					* @param[in] create_info
					* * Структура `WvkSurfaceMSWindowsCreateInfo`, содержащая дескрипторы окна Win32 и dispatch-таблицу.
					*
					* @return WvkStatus
					* * `OK`, если surface создан успешно.
					* * `FAIL` с соответствующим сообщением в случае ошибки валидации или создания.
					*
					* @code
					* WvkSurfaceMSWindows surface;
					* WvkSurfaceMSWindowsCreateInfo info = { ... };
					* WvkStatus status = surface.create(info);
					* if (!status) {
					*     log->error("Failed to create Win32 surface: {}", status.message());
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus create(const WvkSurfaceMSWindowsCreateInfo& create_info) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void destroy(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void requestVkSurface(VkSurfaceKHR& vk_surface_khr) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* Проверяет корректность структуры `WvkSurfaceMSWindowsCreateInfo` перед созданием Vulkan surface.
					*
					* @details
					* Метод выполняет валидацию обязательных полей структуры `m_create_info`. Проверяется,
					* что указатели `hInstance`, `hWnd` и `wvk_khr_win32_surface_dispatch_table` не равны `NULL` или `nullptr`.
					* При обнаружении ошибки возвращает статус с пояснением, какое именно поле некорректно.
					*
					* @return WvkStatus
					* * Код `OK`, если все обязательные поля заданы корректно.
					* * Код `FAIL` и сообщение с указанием проблемного поля в случае ошибки.
					*
					* @code
					* WvkStatus status = surface.validationCreateInfo();
					* if (!status.isOk()) {
					*     log->error("Validation failed: {}", status.message());
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus validationCreateInfo(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* Создаёт Vulkan surface для Windows-платформы с использованием расширения VK_KHR_win32_surface.
					*
					* @details
					* Метод инициализирует структуру `VkWin32SurfaceCreateInfoKHR`, заполняя её данными из `m_create_info`,
					* и вызывает функцию `wvkCreateWin32SurfaceKHR` через ранее загруженную dispatch-таблицу.
					* При неуспехе возвращает статус с подробным описанием ошибки.
					*
					* @return WvkStatus
					* * Код `OK` при успешном создании surface.
					* * Код `FAIL` с сообщением об ошибке, если `wvkCreateWin32SurfaceKHR` возвращает ошибку.
					*
					* @code
					* WvkSurfaceMSWindows surface;
					* WvkStatus status = surface.createVkSurface();
					* if (!status.isOk()) {
					*     log->error(status.message());
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus createVkSurface(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void reset(void) noexcept;

				private:

					WvkSurfaceMSWindowsCreateInfo m_create_info = {};
					VkSurfaceKHR m_vk_surface = VK_NULL_HANDLE;
				}; // class WvkSurfaceMSWindows

			} // namespace mswindows

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_surface_mswindows.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS__WVK_SURFACE_MSWINDOWS_H
