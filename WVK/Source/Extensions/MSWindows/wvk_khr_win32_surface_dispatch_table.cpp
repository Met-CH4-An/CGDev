////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_khr_win32_surface_dispatch_table.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../../wvk_loader.h"
#include "../../wvk_instance.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			namespace mswindows {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkKhrWin32SurfaceDispatchTable::WvkKhrWin32SurfaceDispatchTable(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkKhrWin32SurfaceDispatchTable::~WvkKhrWin32SurfaceDispatchTable(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkKhrWin32SurfaceDispatchTable::create(const WvkKhrWin32SurfaceDispatchTableCreateInfo& create_info) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Проверка на повторную инициализацию
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (m_valid) {
						return _status.set(VknStatusCode::ALREADY_INITIALIZED);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Сохраняем структуру создания
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Валидируем входные данные, если включена валидационная сборка
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
						_status = validationCreateInfo();

						if (!_status) {
							reset();
							return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrWin32SurfaceDispatchTable::validationCreateInfo - fail.");
						}
					//}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Загружаем указатель на vkCreateWin32SurfaceKHR
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = loadVulkanCommand();

					if (!_status) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrWin32SurfaceDispatchTable::loadVulkanCommand - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 5. Возвращаем успешный статус
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_valid = true;
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void WvkKhrWin32SurfaceDispatchTable::destroy(void) noexcept {
					reset();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkKhrWin32SurfaceDispatchTable::validationCreateInfo(void) const noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Проверка обязательного указателя на nullptr
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (m_create_info.wvk_loader == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrWin32SurfaceDispatchTable::wvk_loader - nullptr.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Проверка обязательного указателя на nullptr
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					else if (m_create_info.wvk_instance == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrWin32SurfaceDispatchTable::wvk_instance - nullptr.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Всё в порядке — возвращаем успешный статус
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkKhrWin32SurfaceDispatchTable::loadVulkanCommand(void) noexcept {
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Объявляем переменную статуса операции
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Формируем список процедур Vulkan, которые нужно загрузить
					// В данном случае — только vkCreateWin32SurfaceKHR
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					std::vector<WvkVulkanProcedureInfo> _procedures = {
						{ "vkCreateWin32SurfaceKHR", reinterpret_cast<void**>(&m_vkCreateWin32SurfaceKHR) }
					};

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Загружаем процедуру через обёртку WvkInstance::invokeWithVkInstanceMethod
					// Метод вызывает WvkLoader::loadProcedure с передачей указателей и списка процедур
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
						&WvkLoader::loadProcedure,
						m_create_info.wvk_loader,
						_procedures
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Обрабатываем результат загрузки
					// Если произошла ошибка, возвращаем статус с кодом FAIL и сообщением
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (!_status) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkInstance::invokeWithVkInstanceMethod - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 5. Возвращаем статус успешной загрузки
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void WvkKhrWin32SurfaceDispatchTable::reset(void) noexcept {
					m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;

					m_create_info = {};

					m_valid = false;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace mswindows

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev