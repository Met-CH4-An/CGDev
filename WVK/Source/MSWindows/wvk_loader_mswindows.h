#ifndef CGDEV_WVK_SOURCE_MSWINDOWS__WVK_LOADER_MSWINDOWS_H
#define CGDEV_WVK_SOURCE_MSWINDOWS__WVK_LOADER_MSWINDOWS_H
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
#include "../wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace mswindows {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkLoaderMSWindowsCreateInfo {
			}; // struct WvkLoaderMSWindowsCreateInfo
					




			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkLoaderMSWindows : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkLoaderMSWindows(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkLoaderMSWindows(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*   Загружает библиотеку Vulkan и инициализирует объект VknLoaderMSWindows.
				*
				* @details
				*   Метод выполняет однократную инициализацию загрузчика Vulkan:
				*   сохраняет параметры создания, проводит валидацию, загружает `vulkan-1.dll`
				*   и устанавливает флаг успешной инициализации. Повторный вызов приведёт
				*   к возврату статуса `ALREADY_INITIALIZED`, без повторной загрузки.
				*
				* @param[in] create_info
				*   Структура с параметрами создания.
				*
				* @return
				*   Статус операции:
				*   - `OK` если инициализация успешна.
				*   - `ALREADY_INITIALIZED` если объект уже был создан.
				*   - `FAIL` если валидация не прошла или DLL не загружена.
				*
				* @code
				*   CGDev::wvk::WvkLoaderMSWindows loader;
				*   WvkLoaderMSWindowsCreateInfo info = { /* ... * / };
				*   WvkStatus status = loader.create(info);
				*   if (!status) {
				*       log_error(status.message());
				*   }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkLoaderMSWindowsCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Запрашивает адрес функции vkGetInstanceProcAddr из загруженной библиотеки Vulkan (vulkan-1.dll).
				*
				* @param[out] vkGetInstanceProcAddr
				* Ссылка на переменную, в которую будет записан указатель на функцию vkGetInstanceProcAddr, если она найдена.
				*
				* @return
				* WvkStatus с кодом результата: OK, если функция найдена, иначе FAIL.
				*
				* @note
				* Перед вызовом этого метода необходимо убедиться, что библиотека Vulkan уже загружена и дескриптор m_vulkan_dll действителен.
				*
				* @code
				* WvkLoaderMSWindows loader;
				* loader.loadLibrary(); // Предположительно предварительная инициализация
				*
				* PFN_vkGetInstanceProcAddr getProc = nullptr;
				* WvkStatus status = loader.requestVkGetInstanceProcAddr(getProc);
				*
				* if (status.isOk()) {
				*     // Можно использовать getProc
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestVkGetInstanceProcAddr(PFN_vkGetInstanceProcAddr& vkGetInstanceProcAddr) const noexcept;

			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(const WvkLoaderMSWindowsCreateInfo& create_info) const noexcept;

			private:

				WvkLoaderMSWindowsCreateInfo			m_create_info = {};
				HMODULE									m_vulkan_dll = NULL;
			}; // class WvkLoaderMSWindows

		} // namespace mswindows

	} // namespace wvk

} // namespace CGDev

#include "wvk_loader_mswindows.hpp"

#endif // CGDEV_WVK_SOURCE_MSWINDOWS__WVK_LOADER_MSWINDOWS_H
