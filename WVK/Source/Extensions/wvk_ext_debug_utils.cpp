////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_ext_debug_utils.h"
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

					WvkExtDebugUtils::WvkExtDebugUtils(void) noexcept
						: VknExtension(s_name) {
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkExtDebugUtils::~WvkExtDebugUtils(void) noexcept {
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus WvkExtDebugUtils::create(const WvkExtDebugUtilsCreateInfo& create_info) noexcept {
						WvkStatus _status;

						// Сохраняем переданную структуру создания во внутренний член класса
						m_create_info = create_info;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Если сборка содержит валидацию, то проверяем структуру
						// WvkExtDebugUtilsCreateInfo на корректность заполнения
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						//if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
							// Выполняем валидацию структуры создания
							_status = validationCreateInfo();

							// Если хотя бы один параметр некорректен — возвращаем ошибку
							if (_status.m_code != VknStatusCode::SUCCESSFUL) {
								_status.m_code = VknStatusCode::FAIL;
								_status.append("\n\tVknExtDebugUtils::validationCreateInfo() - fail");
								return _status;
							}
						//}

						// Создаём объект VkDebugUtilsMessengerEXT через соответствующий метод
						_status = createVkDebugUtilsMessenger();
						if (_status.m_code != VknStatusCode::SUCCESSFUL) {
							_status.m_code = VknStatusCode::FAIL;
							_status.append("\n\tVknExtDebugUtils::createVkDebugUtilsMessenger() - fail");
							return _status;
						}

						// Если всё прошло успешно — возвращаем статус SUCCESSFUL
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					void WvkExtDebugUtils::destroy(void) noexcept {

						//m_create_info.wvk_debug_utils_commands->vkDestroyDebugUtilsMessengerEXT();

						// ~~~~~~~~~~~~~~~~
						// очистка данных
						// ~~~~~~~~~~~~~~~~

						m_create_info = {};
						m_vk_debug_utils_messenger = nullptr;
						m_debug_message_collection.clear();
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					bool WvkExtDebugUtils::hasStatus(const VknDebugUtilsMode& mode) const noexcept {
						// Флаги серьёзности и типа, соответствующие переданному режиму
						VkDebugUtilsMessageSeverityFlagsEXT _severity = 0;
						VkDebugUtilsMessageTypeFlagsEXT _type = 0;

						// Преобразуем режим пользователя в флаги Vulkan
						convertDebugUtilsModeToVkFlags(mode, _severity, _type);

						// Проходим по всем накопленным сообщениям отладки
						for (const auto& msg : m_debug_message_collection) {
							// Проверяем, соответствует ли сообщение хотя бы одному из заданных флагов
							if ((msg.severity & _severity) && (msg.type & _type)) {
								return true; // Найдено соответствующее сообщение
							}
						}

						// Нет сообщений, соответствующих заданным условиям
						return false;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus WvkExtDebugUtils::validationCreateInfo(void) const noexcept {
						WvkStatus _status;

						// ----------------------------------------------
						// Проверка: должен быть передан валидный указатель 
						// на объект команд для работы с отладкой Vulkan
						// ----------------------------------------------
						//if (m_create_info.wvk_dispatch_table_ptr == nullptr) {
						//	_status.m_code = VknStatusCode::FAIL;
						//	_status.append("\n\tVknExtDebugUtilsCreateInfo::wvk_dispatch_table_ptr - nullptr.");
						//	return _status;
						//}

						// ----------------------------------------------
						// Проверка: режим отладки должен быть установлен 
						// в допустимое значение, не UNKNOWN
						// ----------------------------------------------
						if (m_create_info.mode == VknDebugUtilsMode::UNKNOWN) {
							_status.m_code = VknStatusCode::FAIL;
							_status.append("\n\tVknExtDebugUtilsCreateInfo::mode = VknDebugUtilsMode::UNKNOWN");
							return _status;
						}

						// ------------------------------------------------
						// Все проверки пройдены успешно, возвращаем OK
						// ------------------------------------------------
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					void WvkExtDebugUtils::convertDebugUtilsModeToVkFlags(const VknDebugUtilsMode& mode, VkDebugUtilsMessageSeverityFlagsEXT& outSeverity, VkDebugUtilsMessageTypeFlagsEXT& outType) const noexcept {
						
						// --------------------------------------
						// Уровни серьёзности (Severity flags)
						// --------------------------------------

						// Добавляем флаг ошибок, если режим включает ERRORS_ONLY
						if (has_flag(mode, VknDebugUtilsMode::ERRORS_ONLY)) {
							outSeverity |= VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
						}

						// Добавляем флаг предупреждений, если режим включает WARNINGS_ONLY
						if (has_flag(mode, VknDebugUtilsMode::WARNINGS_ONLY)) {
							outSeverity |= VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT;
						}

						// Добавляем флаг информационных сообщений, если включён INFO_ONLY
						if (has_flag(mode, VknDebugUtilsMode::INFO_ONLY)) {
							outSeverity |= VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT;
						}

						// Добавляем флаг подробного вывода, если включён VERBOSE
						if (has_flag(mode, VknDebugUtilsMode::VERBOSE)) {
							outSeverity |= VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT;
						}

						// --------------------------------------
						// Типы сообщений (Message Type flags)
						// --------------------------------------

						// Добавляем тип валидации, если включён VALIDATION
						if (has_flag(mode, VknDebugUtilsMode::VALIDATION)) {
							outType |= VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT;
						}

						// Добавляем тип производительности, если включён PERFORMANCE
						if (has_flag(mode, VknDebugUtilsMode::PERFORMANCE)) {
							outType |= VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
						}

						// Добавляем общий тип, если включён GENERAL
						if (has_flag(mode, VknDebugUtilsMode::GENERAL)) {
							outType |= VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT;
						}
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus WvkExtDebugUtils::createVkDebugUtilsMessenger(void) noexcept {
						WvkStatus _status;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 1. Инициализация структуры VkDebugUtilsMessengerCreateInfoEXT
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Инициализируем структуру VkDebugUtilsMessengerCreateInfoEXT нулями
						VkDebugUtilsMessengerCreateInfoEXT _debug_utils_messenger_create_info = {};
						_debug_utils_messenger_create_info.sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
						_debug_utils_messenger_create_info.pNext = nullptr;
						_debug_utils_messenger_create_info.flags = 0;

						// Уровни серьёзности и типы сообщений будут установлены ниже на основе режима
						_debug_utils_messenger_create_info.messageSeverity = 0;
						_debug_utils_messenger_create_info.messageType = 0;

						// Назначаем callback-функцию и передаём this в качестве пользовательских данных
						_debug_utils_messenger_create_info.pfnUserCallback = &s_callback;
						_debug_utils_messenger_create_info.pUserData = this;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 2. Преобразование режима отладки в соответствующие флаги Vulkan
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Преобразуем режим отладки в соответствующие флаги Vulkan
						convertDebugUtilsModeToVkFlags(m_create_info.mode,
							_debug_utils_messenger_create_info.messageSeverity,
							_debug_utils_messenger_create_info.messageType);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 3. Попытка создания мессенджера для отладки
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Пытаемся создать мессенджер через командный интерфейс
						//VkResult _vk_result = m_create_info.wvk_debug_utils_commands->vknCreateDebugUtilsMessengerEXT(
						//	&_debug_utils_messenger_create_info,
						//	nullptr,
						//	&m_vk_debug_utils_messenger);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 4. Обработка результата создания мессенджера
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Обработка результата
						//if (_vk_result != VK_SUCCESS) {
						//	_status.m_code = VknStatusCode::FAIL;

							// Добавим конкретное описание ошибки, если известно
						//	switch (_vk_result) {
						//	case VK_ERROR_OUT_OF_HOST_MEMORY:
						//		_status.append("\n\tvknCreateDebugUtilsMessengerEXT - VK_ERROR_OUT_OF_HOST_MEMORY");
						//		break;
						//	default:
						//		_status.append("\n\tvknCreateDebugUtilsMessengerEXT - Unknown error");
						//		break;
						//	}

						//	return _status;
						//}

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 5. Успешное завершение
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Успешное завершение
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					VkBool32 WvkExtDebugUtils::s_callback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept {

						auto* _this = static_cast<WvkExtDebugUtils*>(pUserData);

						// Записываем сообщение в очередь
						if (_this) {
							DebugMessage _msg;
							_msg.severity = messageSeverity;
							_msg.type = messageTypes;
							_msg.message = pCallbackData->pMessage;
							_this->m_debug_message_collection.push_back(std::move(_msg));
						}

						return VK_FALSE; // не прерываем выполнение
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev