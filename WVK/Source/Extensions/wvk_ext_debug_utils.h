#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_extensions.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../wvk_extension.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Extensions {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Представляет одно отладочное сообщение от Vulkan через VK_EXT_debug_utils.
					*
					* Структура `DebugMessage` хранит параметры одного сообщения, сгенерированного
					* системой отладки Vulkan. Она используется в коллекции внутри `VknExtDebugUtils`
					* для хранения истории сообщений и последующего анализа.
					*
					* \sa WvkExtDebugUtils::hasStatus
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct DebugMessage {
						VkDebugUtilsMessageSeverityFlagBitsEXT severity;  //!< Уровень серьёзности сообщения (ошибка, предупреждение и т.п.).
						VkDebugUtilsMessageTypeFlagsEXT type;             //!< Тип сообщения (валидация, производительность, общее).
						std::string message;                              //!< Текстовое описание сообщения.
					};





					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Режим фильтрации и подписки на сообщения отладки Vulkan.
					*
					* `VknDebugUtilsMode` — это перечисление с флагами, определяющее, какие
					* сообщения отладки следует перехватывать. Оно объединяет уровни серьёзности
					* (ошибки, предупреждения, информация, подробности) и типы сообщений
					* (валидация, производительность, общее).
					*
					* Значения можно комбинировать с помощью побитовых операций:
					* @code
					* VknDebugUtilsMode mode = VknDebugUtilsMode::ERRORS_ONLY | VknDebugUtilsMode::VALIDATION;
					* @endcode
					*
					* \note Макрос ENABLE_ENUM_FLAGS позволяет использовать побитовые операции с этим перечислением.
					*
					* \sa WvkExtDebugUtils::create
					* \sa WvkExtDebugUtils::convertDebugUtilsModeToVkFlags
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					enum class VknDebugUtilsMode : uint32_t {
						UNKNOWN = 0,                                    //!< Режим не задан или неизвестен.
						ERRORS_ONLY = 1 << 0,                           //!< Только сообщения об ошибках.
						WARNINGS_ONLY = 1 << 1,                         //!< Только предупреждения.
						INFO_ONLY = 1 << 2,                             //!< Информационные сообщения.
						VERBOSE = 1 << 3,                               //!< Подробные (debug-level) сообщения.
						ALL_SEVERITIES = ERRORS_ONLY | WARNINGS_ONLY | INFO_ONLY | VERBOSE, //!< Все уровни серьёзности.

						VALIDATION = 1 << 4,                            //!< Сообщения, связанные с валидацией.
						PERFORMANCE = 1 << 5,                           //!< Производственные (performance) сообщения.
						GENERAL = 1 << 6,                               //!< Общие сообщения.
						ALL_TYPES = VALIDATION | PERFORMANCE | GENERAL  //!< Все типы сообщений.
					};
					ENABLE_ENUM_FLAGS(VknDebugUtilsMode);





					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Структура параметров создания для `VknExtDebugUtils`.
					*
					* Содержит указатель на обёртку команд расширения отладки, а также
					* режим отладки `VknDebugUtilsMode`, который определяет, какие сообщения
					* должны перехватываться.
					*
					* Используется при вызове `VknExtDebugUtils::create()`.
					*
					* \note Указатель `wvk_debug_utils_commands` обязателен и не должен быть nullptr.
					*
					* \sa WvkExtDebugUtils
					* \sa WvkExtDebugUtils::create
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct WvkExtDebugUtilsCreateInfo {
						WvkInstanceDispatchTablePtr wvk_dispatch_table_ptr = nullptr;
						VknDebugUtilsMode mode = VknDebugUtilsMode::UNKNOWN;
					};





					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Расширение отладки Vulkan с поддержкой VK_EXT_debug_utils.
					*
					* Класс `VknExtDebugUtils` инкапсулирует логику создания и управления
					* отладочными сообщениями в Vulkan через расширение `VK_EXT_debug_utils`.
					*
					* Он предоставляет:
					* - создание объекта `VkDebugUtilsMessengerEXT`,
					* - фильтрацию сообщений по типу и уровню серьёзности,
					* - внутреннюю валидацию входных параметров,
					* - вспомогательные преобразования режимов пользователя в флаги Vulkan.
					*
					* Используется для удобной интеграции отладки при разработке Vulkan-приложений.
					*
					* Пример:
					* @code
					* WvkExtDebugUtils debugUtils;
					* WvkExtDebugUtilsCreateInfo info = {...};
					* debugUtils.create(info);
					* if (debugUtils.hasStatus(VknDebugUtilsMode::ERRORS_ONLY)) {
					*     std::cerr << "Ошибки отладки обнаружены!" << std::endl;
					* }
					* @endcode
					*
					* \see VkDebugUtilsMessengerEXT
					* \see VknDebugUtilsMode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					class WvkExtDebugUtils : public VknExtension {

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						inline static const std::string& s_getName(void) noexcept;

					private:

						inline static const std::string s_name = "VK_EXT_debug_utils";





					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkExtDebugUtils(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						~WvkExtDebugUtils(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Инициализирует расширение отладчика Vulkan с заданной конфигурацией.
						*
						* Метод сохраняет входную структуру `VknExtDebugUtilsCreateInfo` во внутреннее состояние,
						* выполняет валидацию полей, если сборка включает проверки (`ValidationBuildInfo::enable == true`),
						* и вызывает создание `VkDebugUtilsMessengerEXT`.
						*
						* Если любое из действий завершается неудачно — метод возвращает объект `VknStatus` с описанием ошибки.
						*
						* @param[in] create_info Структура, содержащая параметры создания отладочного расширения.
						* @return [out] WvkStatus Объект статуса выполнения, содержащий код результата и возможное сообщение об ошибке.
						*
						* Пример использования:
						* @code
						* WvkExtDebugUtilsCreateInfo info = {};
						* info.mode = VknDebugUtilsMode::ALL;
						* info.wvk_debug_utils_commands = &debugCommands;
						*
						* WvkExtDebugUtils debugUtils;
						* WvkStatus status = debugUtils.create(info);
						* if (!status) {
						*     std::cerr << "Failed to create debug utils: " << status.message_old << std::endl;
						* }
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus create(const WvkExtDebugUtilsCreateInfo& create_info) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						void destroy(void) noexcept;

					public:

						inline const WvkExtDebugUtilsCreateInfo& getCreateInfo(void) const noexcept {
							return m_create_info;
						}

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Проверяет, содержит ли коллекция сообщений сообщения с заданными флагами серьёзности и типа.
						*
						* Эта функция позволяет определить, были ли зарегистрированы сообщения отладки Vulkan,
						* соответствующие заданному режиму `VknDebugUtilsMode`, который включает флаги уровней серьёзности
						* (такие как ERROR, WARNING и т.п.) и типы сообщений (VALIDATION, PERFORMANCE и т.п.).
						*
						* @param[in] mode Режим фильтрации сообщений, содержащий комбинацию флагов VknDebugUtilsMode.
						* @return true, если найдено хотя бы одно сообщение, удовлетворяющее условиям.
						* @return false, если таких сообщений нет.
						*
						* Пример:
						* @code
						* WvkExtDebugUtils debugUtils;
						* if (debugUtils.check(VknDebugUtilsMode::ERRORS_ONLY | VknDebugUtilsMode::VALIDATION)) {
						*     std::cerr << "Validation error detected!" << std::endl;
						* }
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						bool hasStatus(const VknDebugUtilsMode& mode) const noexcept;

					private:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Проверяет корректность полей структуры VknExtDebugUtilsCreateInfo.
						* 
						* Метод используется для базовой валидации входных данных перед созданием объекта отладки Vulkan.
						* Включается только при компиляции с включённой валидацией (см. ValidationBuildInfo::enable).
						*
						* @return WvkStatus
						*         Возвращает объект со статусом проверки:
						*         - VknStatusCode::SUCCESSFUL — если все проверки пройдены успешно.
						*         - VknStatusCode::FAIL — если обнаружена ошибка, с поясняющим сообщением в поле `message`.
						* 
						* @code
						* WvkExtDebugUtils debugUtils;
						* WvkExtDebugUtilsCreateInfo info = {...};
						* debugUtils.create(info); // Внутри будет вызвана validationCreateInfo()
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus validationCreateInfo(void) const noexcept;
						
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Преобразует пользовательский режим отладки VknDebugUtilsMode в соответствующие флаги Vulkan.
						*
						* Эта функция сопоставляет значения перечисления VknDebugUtilsMode с соответствующими
						* битовыми флагами Vulkan `VkDebugUtilsMessageSeverityFlagsEXT` и `VkDebugUtilsMessageTypeFlagsEXT`.
						* Она используется при создании объекта `VkDebugUtilsMessengerEXT` для указания, какие сообщения
						* следует перехватывать: ошибки, предупреждения, информационные или подробные сообщения,
						* а также их типы: общие, валидационные или производительные.
						*
						* @param[in] mode Пользовательский режим отладки, объединяющий уровни серьёзности и типы сообщений.
						* @param[out] outSeverity Выходной параметр, в который будут установлены флаги уровней серьёзности (`VkDebugUtilsMessageSeverityFlagsEXT`).
						* @param[out] outType Выходной параметр, в который будут установлены флаги типов сообщений (`VkDebugUtilsMessageTypeFlagsEXT`).
						*
						* @note Эта функция использует битовую маску для определения активных флагов в `mode`.
						*       Значения `outSeverity` и `outType` не обнуляются внутри — предполагается, что вызывающий код
						*       инициализирует их до вызова, если нужно.
						*
						* @see VknDebugUtilsMode
						* @see VkDebugUtilsMessageSeverityFlagBitsEXT
						* @see VkDebugUtilsMessageTypeFlagBitsEXT
						* 
						* @code
						* VknDebugUtilsMode mode = VknDebugUtilsMode::ERRORS_ONLY | VknDebugUtilsMode::VALIDATION;
						* VkDebugUtilsMessageSeverityFlagsEXT severity = 0;
						* VkDebugUtilsMessageTypeFlagsEXT type = 0;
						*
						* WvkExtDebugUtils debugUtils;
						* debugUtils.convertDebugUtilsModeToVkFlags(mode, severity, type);
						* // Теперь severity содержит VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT
						* // а type содержит VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						void convertDebugUtilsModeToVkFlags(const VknDebugUtilsMode& mode, VkDebugUtilsMessageSeverityFlagsEXT& outSeverity, VkDebugUtilsMessageTypeFlagsEXT& outType) const noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Создаёт мессенджер для отладки с использованием расширения Vulkan VK_EXT_debug_utils.
						* 
						* Этот метод инициализирует структуру VkDebugUtilsMessengerCreateInfoEXT, заполняет её необходимыми значениями, 
						* устанавливает callback-функцию для получения сообщений от Vulkan и пытается создать мессенджер.
						* Если создание мессенджера не удаётся, возвращается код ошибки.
						*
						* @return Возвращает объект типа WvkStatus, который содержит код результата операции.
						*
						* @code
						* // Пример использования:
						* WvkExtDebugUtils debugUtils;
						* WvkStatus status = debugUtils.createVkDebugUtilsMessenger();
						* if (status) {
						*     // Мессенджер успешно создан
						* } else {
						*     // Обработка ошибки
						* }
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus createVkDebugUtilsMessenger(void) noexcept;

					private:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						static VkBool32 s_callback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept;

					private:

						WvkExtDebugUtilsCreateInfo				m_create_info = {};
						VkDebugUtilsMessengerEXT				m_vk_debug_utils_messenger = nullptr;
						std::vector<DebugMessage>				m_debug_message_collection;

					}; // class WvkExtDebugUtils

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_ext_debug_utils.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_H
