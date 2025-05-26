////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_surface_mswindows.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_khr_win32_surface_dispatch_table.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			namespace mswindows {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkSurfaceMSWindows::WvkSurfaceMSWindows(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkSurfaceMSWindows::~WvkSurfaceMSWindows(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkSurfaceMSWindows::create(const WvkSurfaceMSWindowsCreateInfo& create_info) noexcept {
					WvkStatus _status;
					
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Сохраняем входную структуру создания
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Валидация структуры, если включена сборка с проверками
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
						_status = validationCreateInfo();

						if (!_status) {
							// Сбросим объект при неудачной валидации
							reset();
							return _status.set(VknStatusCode::FAIL,
								"\n\tWvkSurfaceMSWindows::validationCreateInfo - fail.");
						}
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Создание Vulkan surface
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = createVkSurface();

					if (!_status) {
						// Сбросим объект при неудачном создании surface
						reset();
						return _status.set(VknStatusCode::FAIL,
							"\n\tWvkSurfaceMSWindows::createVkSurface - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Успешное создание, отмечаем объект как валидный
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_valid = true;
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void WvkSurfaceMSWindows::destroy(void) noexcept {
					reset();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void WvkSurfaceMSWindows::requestVkSurface(VkSurfaceKHR& vk_surface_khr) const noexcept {
					vk_surface_khr = m_vk_surface;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkSurfaceMSWindows::validationCreateInfo(void) const noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Проверка указателя на HINSTANCE
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (m_create_info.hInstance == NULL) {
						return _status.set(VknStatusCode::FAIL,
							"\n\tWvkSurfaceMSWindowsCreateInfo::hInstance - NULL.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Проверка указателя на HWND
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					else if (m_create_info.hWnd == NULL) {
						return _status.set(VknStatusCode::FAIL,
							"\n\tWvkSurfaceMSWindowsCreateInfo::hWnd - NULL.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Проверка указателя на dispatch-таблицу
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					else if (m_create_info.wvk_khr_win32_surface_dispatch_table == nullptr) {
						return _status.set(VknStatusCode::FAIL,
							"\n\tWvkSurfaceMSWindowsCreateInfo::wvk_khr_win32_surface_dispatch_table - nullptr.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Все проверки пройдены успешно
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkSurfaceMSWindows::createVkSurface(void) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Инициализация структуры VkWin32SurfaceCreateInfoKHR
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VkWin32SurfaceCreateInfoKHR _vk_win32_surface_create_info = {};
					_vk_win32_surface_create_info.sType = VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;
					_vk_win32_surface_create_info.pNext = nullptr;
					_vk_win32_surface_create_info.flags = 0;
					_vk_win32_surface_create_info.hinstance = m_create_info.hInstance;
					_vk_win32_surface_create_info.hwnd = m_create_info.hWnd;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Вызов функции создания Vulkan surface
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VkResult _result = m_create_info.wvk_khr_win32_surface_dispatch_table->wvkCreateWin32SurfaceKHR(
						&_vk_win32_surface_create_info,
						nullptr,
						&m_vk_surface
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Обработка ошибок вызова wvkCreateWin32SurfaceKHR
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (_result != VK_SUCCESS) {
						switch (_result) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.set(VknStatusCode::FAIL,
								"\n\twvkCreateWin32SurfaceKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.set(VknStatusCode::FAIL,
								"\n\twvkCreateWin32SurfaceKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						default:
							_status.set(VknStatusCode::FAIL,
								"\n\twvkCreateWin32SurfaceKHR - unknown.");
							break;
						}

						return _status;
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Возврат успешного статуса
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void WvkSurfaceMSWindows::reset(void) noexcept {
					m_create_info = {};
					m_vk_surface = VK_NULL_HANDLE;
					m_valid = false;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace mswindows

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev