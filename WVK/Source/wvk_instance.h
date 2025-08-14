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
			std::vector<std::string> vk_layer_names;
			std::vector<std::string> vk_extension_names;
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
			//template<typename Method, typename Object, typename... Args>
			//inline std::enable_if_t<
			//	std::is_invocable_v<Method, Object, VkInstance, Args...>&&
			//	std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
			//	void
			//>
			//	invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args);

			//template<typename Method, typename Object, typename... Args>
			//inline std::enable_if_t<
			//	std::is_invocable_v<Method, Object, VkInstance, Args...> &&
			//	!std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
			//	std::invoke_result_t<Method, Object, VkInstance, Args...>
			//> invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args);

			//template<typename Method, typename Object, typename... Args>
			//inline std::enable_if_t<
			//	!std::is_invocable_v<Method, Object, VkInstance, Args...>,
			//	void
			//>
			//	invokeWithVkInstanceMethod(Method&&, Object&&, Args&&...);

			//template<typename Method, typename... Args>
			//inline std::enable_if_t<
			//	std::is_invocable_v<Method, VkInstance, Args...>&&
			//	std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
			//	void
			//>
			//	invokeWithVkInstanceFunction(Method&& method, Args&&... args);

			//template<typename Method, typename... Args>
			//inline std::enable_if_t<
			//	std::is_invocable_v<Method, VkInstance, Args...> &&
			//	!std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
			//	std::invoke_result_t<Method, VkInstance, Args...>
			//>
			//	invokeWithVkInstanceFunction(Method&& method, Args&&... args);

		// cpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus requestLayerProperties(std::vector<VkLayerProperties>& vk_layer_properties) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus requestExtensionProperties(std::vector<VkExtensionProperties>& vk_extension_properties) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus requestPhysicalDevices(std::vector<WvkPhysicalDevicePtr>& vk_physical_devices) const noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkInstanceCreateInfo& getCreateInfo(void) const noexcept;

		// hpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkPhysicalDeviceUptrVec2& getWvkPhysicalDevices(void) const noexcept;

		// hpp
		private:
		
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			friend class WvkPhysicalDevice;
			friend class WvkLogicalDevice;
			inline const WvkDispatchTableUptr& getWvkDispatchTable(void) const noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			friend class WvkInstanceDispatchTable;
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
			WvkStatus validationCreateInfo(const WvkInstanceCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createGlobalDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createInstanceDispatchTable(void) noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createPhysicalDevices(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus prepareLayers(std::vector<const char*>& layer_names) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus prepareExtensions(std::vector<const char*>& extension_names) const noexcept;

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

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static VkBool32 s_debugCallback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept;

		private:
			
			WvkInstanceCreateInfo m_create_info = {};
			WvkDispatchTableUptr m_wvk_global_dispatch_table_ptr = nullptr;
			WvkDispatchTableUptr m_wvk_instance_dispatch_table_ptr = nullptr;
			WvkPhysicalDeviceUptrVec2 m_wvk_physical_devices;
			VkInstance m_vk_instance = VK_NULL_HANDLE;
			VkLayerPropertiesArr m_layer_properties_collection;
			VkExtensionPropertiesArr m_extension_properties_collection;
			std::vector<const char*> m_layer_name_collection;
			std::vector<const char*> m_extension_name_collection;
		}; // class WvkInstance

	} // namespace wvk

} // namespace CGDev

#include "wvk_instance.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_H
