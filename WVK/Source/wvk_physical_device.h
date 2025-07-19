#ifndef CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_H
#define CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_H
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
		struct WvkPhysicalDeviceCreateInfo {
			VkPhysicalDevice vk_physical_device = VK_NULL_HANDLE;
			WvkInstanceDtPtr wvk_instance_dispatch_table = nullptr;
		}; // struct WvkPhysicalDeviceCreateInfo
				
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkPhysicalDevice : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkPhysicalDevice(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkPhysicalDevice(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Создаёт физическое устройство Vulkan.
			* 
			* Метод выполняет полную инициализацию физического устройства Vulkan, включая проверку
			* данных через метод `validationCreateInfo()`, а также инициализацию физического устройства
			* через метод `createVkPhysicalDevice()`. Эти шаги обеспечивают корректное создание устройства,
			* готового к дальнейшему использованию.
			*
			* @note Если включены проверки на этапе сборки (через `ValidationBuildInfo`), метод
			*      сначала выполняет валидацию с использованием `validationCreateInfo()`.
			*      Если валидация не проходит, метод возвращает ошибку.
			*
			* @code
			* WvkPhysicalDevice physical_device;
			* WvkPhysicalDeviceCreateInfo create_info;
			* // Заполнение create_info необходимыми данными...
			* physical_device.create(create_info); // Создание и инициализация физического устройства
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkPhysicalDeviceCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		// hpp
		public:			

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Запрашивает базовые свойства физического устройства Vulkan.
			*
			* Данный метод получает основные свойства физического устройства Vulkan и заполняет
			* переданную структуру VkPhysicalDeviceProperties с помощью функции
			* wvkGetPhysicalDeviceProperties из таблицы диспетчеризации инстанса.
			*
			* @param[out] vk_physical_device_properties
			*   Структура VkPhysicalDeviceProperties, которая будет заполнена свойствами физического устройства.
			*
			* @note
			*   Метод не выполняет дополнительных проверок и всегда обращается к Vulkan через таблицу диспетчеризации.
			*
			* @code
			* VkPhysicalDeviceProperties props{};
			* wvk_physical_device.requestPhysicalDeviceProperties(props);
			* // props теперь содержит основные свойства физического устройства
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline void requestProperties(VkPhysicalDeviceProperties& vk_physical_device_properties) const noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Запрашивает свойства физического устройства через цепочку `pNext`, используя `vkGetPhysicalDeviceProperties2`.
			*
			* Метод активен только если доступен Vulkan 1.1 или включено расширение `VK_KHR_get_physical_device_properties2`.
			* Если функциональность недоступна — возвращается ошибка `FEATURE_NOT_ENABLED`.
			*
			* @param[out] out
			* Указатель на первую структуру в цепочке `VkBaseOutStructure`, содержащую расширенные свойства.
			* Например, это могут быть :
			* -`VkPhysicalDeviceIDProperties`
			* -`VkPhysicalDeviceSubgroupProperties`
			* -`VkPhysicalDevicePointClippingProperties`
			* и т.п.
			*
			* @return
			* Возвращает `WvkStatus::ok()`, если свойства успешно получены.
			* Иначе — возвращает `FEATURE_NOT_ENABLED` с пояснением.
			*
			* @code
			* VkPhysicalDeviceIDProperties id_props = {};
			* id_props.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES;
			*
			* WvkStatus status = physical_device->requestProperties(reinterpret_cast<VkBaseOutStructure*>(&id_props));
			* if (status.isOk()) {
			*   // Свойства успешно загружены
			*	processUUID(id_props.deviceUUID);
			* } else {
			*	log->error("Свойства устройства не были загружены: {}", status.message());
			* }
			*@endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus requestProperties(VkBaseOutStructure* out) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Out>
			inline WvkStatus requestProperties(Out& out) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Запрашивает свойства всех семейств очередей** у физического устройства и сохраняет их в переданный контейнер.
			*
			* @param[out] queue_family_properties_collection
			* Контейнер, в который будут записаны структуры `VkQueueFamilyProperties` — одна для каждого семейства очередей.
			*
			* @return
			* Объект `WvkStatus`, указывающий на успешность операции.
			*
			* @code
			* WvkPhysicalDevice device = ...;
			* VkQueueFamilyPropertiesVec1 props;
			* WvkStatus status = device.requestQueueFamilyProperties(props);
			* if (status.isOk()) {
			*     // Обрабатываем props
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus requestQueueFamilyProperties(VkQueueFamilyPropertiesVec1& queue_family_properties_collection) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\briefbrief
			* Requests extended properties of all available queue families using the `vkGetPhysicalDeviceQueueFamilyProperties2` mechanism.
			*
			* @tparam Out
			* Type of the extended structure to retrieve per queue family (must begin with `VkStructureType` and be chainable via `pNext`).
			*
			* @param[out] out
			* Vector of `Out` structures to be filled with extended queue family properties.
			*
			* @param[in] vk_structure_type
			* The `VkStructureType` corresponding to the `Out` type (e.g., `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2` or an extension type).
			*
			* @return
			* `WvkStatus::ok()` if the query was successful and the feature is available.
			* Otherwise returns `FEATURE_NOT_ENABLED` if Vulkan 1.1 or the extension `VK_KHR_get_physical_device_properties2` is not present.
			*
			* @code
			* std::vector<VkQueueFamilyVideoPropertiesKHR> video_queue_props;
			* status = device.requestQueueFamilyProperties(video_queue_props, VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR);
			* if (!status) {
			*     log->error("Query failed: {}", status.message());
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Out>
			inline WvkStatus requestQueueFamilyProperties(std::vector<Out>& out, const VkStructureType& vk_structure_type) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Проверяет, входят ли все заданные физические устройства Vulkan в одну группу устройств.
			*
			* Метод использует Vulkan API `vkEnumeratePhysicalDeviceGroups` (или `KHR`-вариант),
			* чтобы определить, входят ли `this` и все устройства из `wvk_physical_device_collection`
			* в одну и ту же физическую группу.
			*
			* @param[in]  wvk_physical_device_collection  Коллекция указателей на физические устройства.
			* @param[out] compatibility                   Становится `true`, если все устройства совместимы (в одной группе).
			*
			* @return
			* Возвращает статус выполнения. В случае ошибки — содержит описание причины.
			*
			* @code
			* bool is_compatible = false;
			* WvkStatus status = wvk_physical_device->checkCompatibility(devices, is_compatible);
			* if (status.isOk() && is_compatible) {
			*     // Все устройства в одной группе
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus checkCompatibility(const WvkPhysicalDevicePtrArr1& wvk_physical_device_collection, bool& compatibility) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//template<typename Out>
			//inline WvkStatus requestQueueFamilyProperties(std::vector<Out>& out, const VkStructureType& vk_structure_type) const noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Универсальный вызов метода с автоматической подстановкой VkPhysicalDevice.
			*
			* Этот шаблон предназначен для вызова метода (или любого вызываемого объекта),
			* которому требуется `VkPhysicalDevice` в качестве аргумента. Метод будет вызван
			* в форме `method(object, m_vk_physical_device, args...)`, где `args` —
			* дополнительные пользовательские аргументы.
			*
			* @tparam Method Тип вызываемого объекта (указатель на метод, лямбда и т.п.).
			* @tparam Object Тип объекта, на котором вызывается метод.
			* @tparam Args Типы дополнительных аргументов, передаваемых методу.
			*
			* @param[in] method Метод, который необходимо вызвать.
			* @param[in] object Объект, на котором вызывается метод.
			* @param[in] args Дополнительные аргументы, передаваемые методу.
			*
			* @note Эта перегрузка работает только для методов, возвращающих void и вызываемых
			*       в виде `method(object, VkPhysicalDevice, args...)`.
			*
			* @code
			* struct DeviceHandler {
			*     void handle(VkPhysicalDevice device, int index) {
			*         std::cout << "Handling device " << device << " at index " << index << std::endl;
			*     }
			* };
			*
			* DeviceHandler handler;
			* wvk_physical_device_ptr->invokeWithVkPhysicalDevice(&DeviceHandler::handle, handler, 5);
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...>&&
				std::is_void_v<std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>>,
				void
			> 
				invokeWithVkPhysicalDeviceMethod(Method&& method, Object&& object, Args&&... args) const noexcept;

			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...> &&
				!std::is_void_v<std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>>,
				std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>
			> invokeWithVkPhysicalDeviceMethod(Method&& method, Object&& object, Args&&... args) const noexcept;

			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				!std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...>,
				void
			>
				invokeWithVkPhysicalDeviceMethod(Method&&, Object&&, Args&&...) const noexcept;

			template<typename Method, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, VkPhysicalDevice, Args...>&&
				std::is_void_v<std::invoke_result_t<Method, VkPhysicalDevice, Args...>>,
				void
			>
				invokeWithVkPhysicalDeviceFunction(Method&& method, Args&&... args) const noexcept;

			template<typename Method, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, VkPhysicalDevice, Args...> &&
				!std::is_void_v<std::invoke_result_t<Method, VkPhysicalDevice, Args...>>,
				std::invoke_result_t<Method, VkPhysicalDevice, Args...>
			> invokeWithVkPhysicalDeviceFunction(Method&& method, Args&&... args) const noexcept;
		
		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* *Выполняет валидацию полей структуры `WvkPhysicalDeviceCreateInfo`.*
			*
			* Метод проверяет, что ключевые поля структуры `m_create_info` не содержат некорректных значений:
			* - `vk_physical_device` не должен быть `VK_NULL_HANDLE`
			* - `wvk_instance_dispatch_table` не должен быть `nullptr`
			*
			* В случае нарушения условий возвращается статус ошибки с описанием.
			*
			* @return
			* Возвращает `WvkStatus::setOk()` в случае успеха, либо ошибку с соответствующим кодом `VknStatusCode::FAIL`.
			*
			* @code
			* WvkPhysicalDevice device;
			* WvkStatus status = device.validationCreateInfo();
			* if (!status) {
			*     std::cerr << "Ошибка валидации: " << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Создаёт физическое устройство Vulkan.
			* 
			* Метод инициализирует объект физического устройства Vulkan, используя переданное в
			* структуре `WvkPhysicalDeviceCreateInfo` значение `vk_physical_device`. 
			* Это физическое устройство будет использовано для последующих операций с Vulkan.
			*
			* @note Метод не выполняет дополнительных проверок или инициализации,
			*      так как предполагается, что `vk_physical_device` был ранее правильно настроен.
			*
			* @code
			* WvkPhysicalDevice physical_device;
			* WvkPhysicalDeviceCreateInfo create_info;
			* create_info.vk_physical_device = some_physical_device_handle; // Получение физического устройства
			* physical_device.createVkPhysicalDevice(); // Инициализация физического устройства
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createVkPhysicalDevice(void) noexcept;

			void reset(void) noexcept;

		private:

			WvkPhysicalDeviceCreateInfo						m_create_info = {};
			VkPhysicalDevice								m_vk_physical_device = VK_NULL_HANDLE;
		}; // class WvkPhysicalDevice

	} // namespace wvk

} // namespace CGDev

#include "wvk_physical_device.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_H
