////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_khr_get_surface_capabilities2.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkKhrGetSurfaceCapabilities2::WvkKhrGetSurfaceCapabilities2(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkKhrGetSurfaceCapabilities2::~WvkKhrGetSurfaceCapabilities2(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrGetSurfaceCapabilities2::create(const WvkKhrGetSurfaceCapabilities2CreateInfo& create_info) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Сохраняем переданную структуру создания.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info = create_info;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Если включена сборка с валидацией, запускаем проверку данных.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
					_status = validationCreateInfo();

					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2::validationCreateInfo() - fail.");
					}
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Загружаем указатели на Vulkan-функции, связанные с Surface.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = loadVulkanCommand();

				if (_status.m_code != VknStatusCode::SUCCESSFUL) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2::loadVulkanCommand() - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Успешное завершение создания.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkKhrGetSurfaceCapabilities2::destroy(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// очистка данных
				// ~~~~~~~~~~~~~~~~

				m_create_info = {};

				m_vk_get_physical_device_surface_capabilities2_khr = VK_NULL_HANDLE;
				m_vk_get_physical_device_surface_formats2_khr = VK_NULL_HANDLE;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrGetSurfaceCapabilities2::validationCreateInfo(void) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Проверяем, что wvk_commands задан.
				// Это обязательное поле для дальнейшей загрузки Vulkan-команд.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_create_info.wvk_commands == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2CreateInfo::wvk_commands - nullptr.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Все проверки пройдены — возвращаем успешный статус.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrGetSurfaceCapabilities2::loadVulkanCommand(void) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Пытаемся загрузить все необходимые функции расширения KHR Surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = m_create_info.wvk_commands->tryLoadFunction({
					{ "vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vk_get_physical_device_surface_capabilities2_khr) },
					{ "vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vk_get_physical_device_surface_formats2_khr) }
					});

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Проверяем результат загрузки
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (!_status) {
					return _status.set(VknStatusCode::FAIL,
						"\n\tWvkCommands::tryLoadFunction - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Все функции загружены успешно
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev