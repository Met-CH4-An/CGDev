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

			void requestPhysicalDeviceProperties(VkPhysicalDeviceProperties& vk_physical_device_properties) const noexcept;

			void requestPhysicalDeviceProperties(VkPhysicalDeviceProperties2& vk_physical_device_properties) const noexcept;

		public:			

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*  Универсальный метод запроса расширенных свойств физического устройства Vulkan.
			*  Использует VkPhysicalDeviceProperties2 и цепочку pNext для получения информации
			*  о свойствах, специфичных для переданной структуры VkPhysicalDeviceXProperties.
			*
			* @tparam VkPhysicalDeviceXProperties Тип расширенной структуры свойств Vulkan.
			*
			* @param[out] vk_physical_device_x_properties
			*  Ссылка на структуру расширенных свойств физического устройства, которая будет заполнена.
			*
			* @note
			*  Метод работает только с Vulkan API версии 1.1 и выше.
			*  Для более старых версий метод не производит никаких действий.
			*
			* @code
			* VkPhysicalDeviceVulkan11Properties props11{};
			* wvkPhysicalDevice.requestPhysicalDeviceProperties(props11);
			* // props11 заполнен расширенными свойствами Vulkan 1.1
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename VkPhysicalDeviceXProperties>
			inline void requestPhysicalDeviceProperties(VkPhysicalDeviceXProperties& vk_physical_device_x_properties) const noexcept;

		public:
			
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
			* wvk_physical_device->invokeWithVkPhysicalDevice(&DeviceHandler::handle, handler, 5);
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
