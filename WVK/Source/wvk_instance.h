#ifndef CGDEV_WVK_SOURCE__WVK_INSTANCE_H
#define CGDEV_WVK_SOURCE__WVK_INSTANCE_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkInstanceCreateInfo {
			WvkLoaderDispatchTablePtr wvk_loader_dispatch_table = nullptr;
		}; // struct WvkInstanceCreateInfo





		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkInstance : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkInstance(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkInstance(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* Создаёт объект WvkInstance и инициализирует VkInstance.
			*
			* @details
			* Метод выполняет пошаговую инициализацию Vulkan-инстанса:
			* проверяет входные данные, подготавливает слои и расширения,
			* затем создаёт VkInstance и обновляет флаг `m_valid`.
			* В случае ошибки внутреннее состояние сбрасывается.
			*
			* @param[in] create_info
			* Структура, содержащая настройки для создания инстанса.
			*
			* @return
			* Возвращает статус выполнения операции:
			* - `SUCCESSFUL` — успешное создание.
			* - `ALREADY_INITIALIZED` — инстанс уже создан.
			* - `FAIL` — при любой стадии ошибки.
			*
			* @code
			* WvkInstance instance;
			* WvkInstanceCreateInfo info = { ... };
			* WvkStatus status = instance.create(info);
			* if (!status) {
			*     std::cerr << "Ошибка: " << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkInstanceCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Уничтожение Vulkan-экземпляра.
				* 
				* Этот метод отвечает за корректное освобождение всех ресурсов, связанных с Vulkan-экземпляром.
				* Включает в себя:
				* - Вызов функции vkDestroyInstance для уничтожения Vulkan-экземпляра,
				* - Очистку всех данных, связанных с созданием экземпляра, таких как:
				*   - Очистка структуры m_create_info,
				*   - Обнуление указателя на Vulkan-экземпляр (m_vk_instance),
				*   - Очистка коллекций слоёв и расширений (m_layer_properties_collection, m_extension_properties_collection, m_layer_name_collection, m_extension_name_collection).
				* 
				* После выполнения этой функции экземпляр Vulkan больше не будет доступен, а все связанные с ним ресурсы будут освобождены.
				*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, Object, VkInstance, Args...>&&
				std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
				void
			>
				invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args);

			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, Object, VkInstance, Args...> &&
				!std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
				std::invoke_result_t<Method, Object, VkInstance, Args...>
			> invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args);

			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				!std::is_invocable_v<Method, Object, VkInstance, Args...>,
				void
			>
				invokeWithVkInstanceMethod(Method&&, Object&&, Args&&...);

			template<typename Method, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, VkInstance, Args...>&&
				std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
				void
			>
				invokeWithVkInstanceFunction(Method&& method, Args&&... args);

			template<typename Method, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, VkInstance, Args...> &&
				!std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
				std::invoke_result_t<Method, VkInstance, Args...>
			>
				invokeWithVkInstanceFunction(Method&& method, Args&&... args);

		// hpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkInstanceCreateInfo& getCreateInfo(void) const noexcept;

		// hpp
		private:

			friend class WvkInstanceDispatchTable;
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief Возвращает нативный объект VkInstance.
			* 
			* Этот метод предоставляет доступ к внутреннему Vulkan-инстансу (`VkInstance`),
			* созданному в процессе инициализации объекта `VknInstance`. 
			* Метод безопасен для вызова и не изменяет состояние объекта.
			*
			* @note Возвращаемое значение является ссылкой на внутренний объект.
			* Пользователь не должен вручную уничтожать или изменять `VkInstance`.
			* Управление временем жизни объекта остаётся за `VknInstance`.
			*
			* @return Константная ссылка на объект `VkInstance`.
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const VkInstance& getVkInstance(void) const noexcept;

		// cpp
		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Проверяет валидность структуры `WvkInstanceCreateInfo`.
			*
			* Метод используется для проверки корректности входных данных, переданных при создании
			* экземпляра `WvkInstance`. В частности, он проверяет, что обязательный указатель на
			* `wvk_loader_dispatch_table` не является `nullptr`.
			*
			* Это важно для предотвращения аварий при дальнейшем использовании неинициализированных
			* указателей в Vulkan API.
			*
			* @note Метод не выбрасывает исключения и является `noexcept`.
			*
			* @return Возвращает `WvkStatus::OK()`, если все проверки пройдены успешно.
			*         В противном случае — код ошибки с подробным описанием причины.
			*
			* @code
			* WvkInstance instance;
			* WvkStatus status = instance.validationCreateInfo();
			* if (!status) {
			*     log->error("Ошибка валидации: {}", status.message());
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Подготавливает и активирует слои Vulkan для экземпляра.
			*
			* Метод выполняет перечисление всех доступных слоёв Vulkan и сравнивает их с
			* предустановленными слоями, указанными в сборке (если включена поддержка валидации).
			* В случае, если слой не найден, метод возвращает ошибку. Также метод обрабатывает ошибки,
			* возникающие при перечислении слоёв, и предоставляет подробную информацию о возникших проблемах.
			*
			* @note Метод использует `vkEnumerateInstanceLayerProperties` для получения списка доступных слоёв.
			*
			* @note Если включена поддержка валидации, метод проверяет наличие требуемых слоёв
			*       и добавляет их в коллекцию активированных слоёв.
			*
			* @note Метод не выбрасывает исключений (`noexcept`).
			*
			* @return Возвращает статус выполнения. Если слои успешно подготовлены, возвращает
			*         статус "OK". В случае ошибки — соответствующий код ошибки с описанием проблемы.
			*
			* @code
			* WvkInstance instance;
			* WvkStatus status = instance.prepareLayer();
			* if (!status) {
			*     log->error("Ошибка подготовки слоёв Vulkan: {}", status.message());
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus prepareLayer(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Подготавливает список расширений Vulkan, необходимых для создания экземпляра.
			*
			* Метод выполняет запрос доступных расширений на платформе, сравнивает их с требуемыми
			* (если включена валидация через `wvk::Build::ValidationBuildInfo`) и заполняет
			* внутренние коллекции для дальнейшего использования.
			*
			* @return WvkStatus
			* - [out] Код состояния и сообщение об ошибке при необходимости.
			*
			* @code
			* WvkStatus status = instance.prepareExtension();
			* if (status.failed()) {
			*     std::cerr << status.message_old;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus prepareExtension(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Создаёт экземпляр Vulkan.
			*
			* Метод инициализирует и создаёт экземпляр Vulkan с использованием переданных
			* настроек и обязательных слоёв и расширений. Он заполняет структуру `VkApplicationInfo`
			* и структуру `VkInstanceCreateInfo`, которая передаётся в функцию `vknCreateInstance()`.
			* Если создание экземпляра Vulkan завершилось неудачно, метод возвращает подробное сообщение
			* об ошибке.
			*
			* @note Метод использует `VkResult` для проверки результата создания экземпляра.
			*       Ошибки классифицируются в зависимости от типа ошибки Vulkan.
			*
			* @note Метод не выбрасывает исключений (`noexcept`).
			*
			* @return Возвращает статус выполнения. Если создание экземпляра прошло успешно,
			*         возвращает статус "OK". В случае ошибки — соответствующий код ошибки
			*         с описанием проблемы.
			*
			* @code
			* WvkInstance instance;
			* WvkStatus status = instance.createVkInstance();
			* if (!status) {
			*     log->error("Ошибка создания экземпляра Vulkan: {}", status.message());
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createVkInstance(void) noexcept;

			void reset(void) noexcept;

		private:

			WvkInstanceCreateInfo m_create_info = {};
			VkInstance m_vk_instance = nullptr;
			VkLayerPropertiesArr m_layer_properties_collection;
			VkExtensionPropertiesArr m_extension_properties_collection;
			std::vector<const char*> m_layer_name_collection;
			std::vector<const char*> m_extension_name_collection;
		}; // class WvkInstance

	} // namespace wvk

} // namespace CGDev

#include "wvk_instance.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_H
