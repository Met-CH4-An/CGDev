#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_H
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
#include "../wvk_base_commands.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../wvk_commands.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Extensions {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct VknExtDebugUtilsCommandsCreateInfo {					
						WvkInstanceDispatchTablePtr wvk_commands = nullptr;
					}; // VknExtDebugUtilsCommandsCreateInfo





					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					class VknExtDebugUtilsCommands : public VknBaseCommands {

					public:

						inline VkResult vknCreateDebugUtilsMessengerEXT(const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pMessenger) const noexcept;
						inline void vknDestroyDebugUtilsMessengerEXT(VkDebugUtilsMessengerEXT messenger, const VkAllocationCallbacks* pAllocator) const noexcept;
						inline void vknSubmitDebugUtilsMessageEXT(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData) const noexcept;

					private:

						PFN_vkCreateDebugUtilsMessengerEXT									m_vkCreateDebugUtilsMessengerEXT = nullptr;
						PFN_vkDestroyDebugUtilsMessengerEXT									m_vkDestroyDebugUtilsMessengerEXT = nullptr;
						PFN_vkSubmitDebugUtilsMessageEXT									m_vkSubmitDebugUtilsMessageEXT = nullptr;

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						VknExtDebugUtilsCommands(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						~VknExtDebugUtilsCommands(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Инициализирует объект и загружает функции расширения `VK_EXT_debug_utils`.
						*
						* Метод сохраняет переданную структуру `VknExtDebugUtilsCommandsCreateInfo`, выполняет её
						* проверку (при включённой сборке с валидацией), а затем пытается загрузить все необходимые
						* функции расширения `VK_EXT_debug_utils`. Используется внутренний метод `loadVulkanCommand()`.
						*
						* При возникновении ошибки на любом этапе возвращается статус `VknStatusCode::FAIL` с детальным
						* описанием проблемы.
						*
						* @param [in] create_info Структура с параметрами инициализации, включая обязательный указатель
						* на `VknCommands`, необходимый для загрузки адресов функций.
						*
						* @return [out] Статус выполнения операции. При неудаче содержит описание ошибки.
						*
						* @code
						* VknExtDebugUtilsCommands debug_utils_cmds;
						* VknExtDebugUtilsCommandsCreateInfo info;
						* info.wvk_commands = &commands;
						* info.mode = VknDebugUtilsMode::ALL_SEVERITIES | VknDebugUtilsMode::ALL_TYPES;
						*
						* WvkStatus status = debug_utils_cmds.create(info);
						* if (status.m_code != VknStatusCode::SUCCESSFUL) {
						*     std::cerr << "Ошибка инициализации DebugUtils: " << status.message_old << std::endl;
						* }
						* @endcode
						*
						* @sa VknExtDebugUtilsCommandsCreateInfo
						* @sa VknExtDebugUtilsCommands::validationCreateInfo
						* @sa VknExtDebugUtilsCommands::loadVulkanCommand
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus create(const VknExtDebugUtilsCommandsCreateInfo& create_info) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						void destroy(void) noexcept;

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Возвращает ссылку на структуру параметров инициализации.
						*
						* Метод предоставляет доступ к внутреннему объекту `VknExtDebugUtilsCommandsCreateInfo`,
						* который был передан при создании объекта с помощью метода `create()`.
						*
						* Используется, если необходимо получить текущее состояние или параметры,
						* связанные с конфигурацией отладки расширения `VK_EXT_debug_utils`.
						*
						* @return [out] Константная ссылка на структуру `VknExtDebugUtilsCommandsCreateInfo`.
						*
						* @code
						* const auto& info = debug_utils_cmds.getCreateInfo();
						* if (info.wvk_debug_utils_commands) {
						*     // Используем доступные команды отладчика
						* }
						* @endcode
						*
						* @sa VknExtDebugUtilsCommandsCreateInfo
						* @sa VknExtDebugUtilsCommands::create
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						inline const VknExtDebugUtilsCommandsCreateInfo& getCreateInfo(void) const noexcept;

					private:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Выполняет валидацию входных данных, необходимых для работы отладочного модуля.
						*
						* Метод проверяет корректность структуры `VknExtDebugUtilsCreateInfo`, переданной при создании
						* объекта `VknExtDebugUtilsCommands`. В частности, он удостоверяется, что поле `wvk_commands`
						* не является `nullptr`, так как оно критически необходимо для загрузки функций расширения
						* `VK_EXT_debug_utils`.
						*
						* В случае, если `wvk_commands` не установлен, метод возвращает код ошибки `VknStatusCode::FAIL`
						* и указывает в сообщении, какое именно поле невалидно.
						*
						* @return [out] Статус выполнения валидации. При ошибке содержит сообщение с указанием причины.
						*
						* @code
						* VknExtDebugUtilsCommands debug_utils_cmds;
						* WvkStatus status = debug_utils_cmds.validationCreateInfo();
						* if (status.m_code != VknStatusCode::SUCCESSFUL) {
						*     std::cerr << "Ошибка валидации CreateInfo: " << status.message_old << std::endl;
						* }
						* @endcode
						*
						* @sa VknExtDebugUtilsCreateInfo
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus validationCreateInfo(void) const noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief Загружает указатели на функции расширения VK_EXT_debug_utils.
						* 
						* Метод загружает три функции из расширения VK_EXT_debug_utils:
						* - vkCreateDebugUtilsMessengerEXT
						* - vkDestroyDebugUtilsMessengerEXT
						* - vkSubmitDebugUtilsMessageEXT
						* 
						* Для загрузки используется вспомогательная функция tryLoadFunction, 
						* которой передаются пары "имя функции — указатель для записи".
						* 
						* @return WvkStatus 
						* - VKN_SUCCESS — если все функции успешно загружены.
						* - Иной код ошибки — если хотя бы одна функция не была загружена.
						* 
						* @note Метод не генерирует исключений (noexcept).
						* 
						* @code
						* VknExtDebugUtilsCommands debugUtils;
						* WvkStatus status = debugUtils.loadVulkanCommand();
						* if (status != VKN_SUCCESS) {
						*     // Обработка ошибки
						* }
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus loadVulkanCommand(void) noexcept;

					private:

						VknExtDebugUtilsCommandsCreateInfo											m_create_info = {};
						
					}; // class VknExtDebugUtilsCommands

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_ext_debug_utils_commands.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_H
