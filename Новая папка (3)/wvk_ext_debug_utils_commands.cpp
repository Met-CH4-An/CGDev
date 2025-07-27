////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_ext_debug_utils_commands.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Extensions {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					VknExtDebugUtilsCommands::VknExtDebugUtilsCommands(void) noexcept {
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					VknExtDebugUtilsCommands::~VknExtDebugUtilsCommands(void) noexcept {
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus VknExtDebugUtilsCommands::create(const VknExtDebugUtilsCommandsCreateInfo& create_info) noexcept {
						WvkStatus _status;

						// Сохраняем входную структуру инициализации.
						m_create_info = create_info;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Если сборка производится с включённой валидацией,
						// выполняем проверку корректности структуры CreateInfo.
						// Это позволяет перехватывать ошибки на этапе разработки.
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						//if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
							_status = validationCreateInfo();

							// Если структура невалидна — завершаем с ошибкой.
							if (_status.m_code != VknStatusCode::SUCCESSFUL) {
								_status.m_code = VknStatusCode::FAIL;
								_status.append("\n\tVknExtDebugUtilsCommands::validationCreateInfo() - fail");
								return _status;
							}
						//}

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Пытаемся загрузить адреса функций расширения VK_EXT_debug_utils.
						// Используется вспомогательный метод loadVulkanCommand().
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						_status = loadVulkanCommand();

						// Обработка возможной ошибки загрузки.
						if (_status.m_code != VknStatusCode::SUCCESSFUL) {
							_status.m_code = VknStatusCode::FAIL;
							_status.append("\n\tVknExtDebugUtilsCommands::loadVulkanCommand() - fail");
							return _status;
						}

						// Успешная инициализация.
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					void VknExtDebugUtilsCommands::destroy(void) noexcept {

						// ~~~~~~~~~~~~~~~~
						// очистка данных
						// ~~~~~~~~~~~~~~~~

						m_create_info = {};

						m_vkCreateDebugUtilsMessengerEXT = nullptr;
						m_vkDestroyDebugUtilsMessengerEXT = nullptr;
						m_vkSubmitDebugUtilsMessageEXT = nullptr;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus VknExtDebugUtilsCommands::validationCreateInfo(void) const noexcept {
						WvkStatus _status;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Проверяем, что обязательное поле wvk_commands не равно nullptr.
						// Это критично для последующей загрузки Vulkan-функций.
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						if (m_create_info.wvk_commands == nullptr) {
							_status.m_code = VknStatusCode::FAIL;

							// Добавляем описание ошибки в сообщение.
							_status.append("\n\tVknExtDebugUtilsCommandsCreateInfo::wvk_commands = nullptr");

							return _status;
						}

						// Все проверки пройдены успешно.
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus VknExtDebugUtilsCommands::loadVulkanCommand(void) noexcept {
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 1. Объявляем переменную для хранения результата загрузки
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus _status;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 2. Вызываем вспомогательную функцию tryLoadFunction,
						//        передавая список Vulkan-функций, которые нужно загрузить
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						_status = m_create_info.wvk_commands->tryLoadFunction({
						//tryLoadFunction(m_create_info.wvk_commands, {
							{ "vkCreateDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkCreateDebugUtilsMessengerEXT) },
							{ "vkDestroyDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkDestroyDebugUtilsMessengerEXT) },
							{ "vkSubmitDebugUtilsMessageEXT", reinterpret_cast<void**>(&m_vkSubmitDebugUtilsMessageEXT) }
						});

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 3. Возвращаем результат попытки загрузки функций
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev