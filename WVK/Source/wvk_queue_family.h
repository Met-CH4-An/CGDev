#ifndef CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_H
#define CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_H
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
		struct WvkQueueFamilyCreateInfo {
			std::optional<uint32_t> index;
			WvkInstanceDtPtr instance_dt_ptr = nullptr;
			WvkPhysicalDevicePtr wvk_physical_device_ptr = nullptr;
		}; // struct WvkQueueFamilyCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkQueueFamily : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkQueueFamily(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkQueueFamily(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*     Инициализирует объект WvkQueueFamily, сохраняя параметры создания и выполняя валидацию.
			*
			* @details
			*     Метод сохраняет переданную структуру WvkQueueFamilyCreateInfo, затем вызывает валидацию входных данных.
			*     Если валидация не пройдена, объект сбрасывается в невалидное состояние и возвращается статус с кодом FAIL и сообщением об ошибке.
			*     В случае успеха объект помечается как валидный и возвращается статус OK.
			*
			* @param[in] create_info
			*     Структура с параметрами для создания семейства очередей (индекс семейства, указатель на instance dispatch table, указатель на физическое устройство).
			*
			* @return WvkStatus
			*     - VknStatusCode::SUCCESSFUL — если инициализация прошла успешно.
			*     - VknStatusCode::FAIL — если валидация входных данных завершилась неудачей.
			*
			* @code
			* WvkQueueFamily queue_family;
			* WvkQueueFamilyCreateInfo create_info = { ... };
			* WvkStatus status = queue_family.create(create_info);
			* if (!status) {
			*     std::cerr << status.getMessage() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkQueueFamilyCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;					

		public:

			inline const uint32_t getIndexFamily(void) const noexcept;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*     Извлекает свойства конкретного семейства очередей физического устройства (VkQueueFamilyProperties).
			*
			* @details
			*     Метод выполняет два последовательных запроса к Vulkan API:
			*     1. Получает общее количество семейств очередей для физического устройства.
			*     2. Запрашивает свойства всех семейств очередей и копирует нужное семейство по индексу в выходной параметр.
			*
			* @param[out] vk_queue_family_properties
			*     Структура, в которую будут записаны свойства семейства очередей (флаги, количество очередей, поддержка операций и т.д.).
			*
			* @note
			*     Метод предполагает, что индекс семейства очередей (`m_create_info.index`) валиден.
			*
			* @code
			* VkQueueFamilyProperties props;
			* queue_family.requestQueueFamilyProperties(props);
			* // props теперь содержит, например, flags и количество очередей
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline void requestProperties(VkQueueFamilyProperties& vk_queue_family_properties) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*     Запрашивает расширенные свойства семейства очередей устройства с использованием VkQueueFamilyProperties2 и цепочки pNext.
			*
			* @details
			*     Метод предназначен для получения расширенных свойств семейства очередей физического устройства через структуру VkQueueFamilyProperties2
			*     и дополнительную структуру (например, VkQueueFamilyCheckpointPropertiesNV, VkQueueFamilyGlobalPriorityPropertiesEXT и др.), переданную через pNext.
			*     Работает только при сборке с поддержкой Vulkan 1.1 и выше или при наличии расширения VK_KHR_get_physical_device_properties2.
			*
			*     Алгоритм работы:
			*     - Шаг 1. Получает количество семейств очередей у физического устройства.
			*     - Шаг 2. Выделяет массивы для VkQueueFamilyProperties2 и расширенной структуры Properties.
			*     - Шаг 3. Инициализирует поля sType и pNext для каждой структуры.
			*     - Шаг 4. Запрашивает свойства всех семейств очередей с помощью vkGetPhysicalDeviceQueueFamilyProperties2.
			*     - Шаг 5. Копирует свойства интересующего семейства (по индексу) в выходной параметр out.
			*
			* @tparam Properties
			*     Структура расширенных свойств семейства очередей (например, VkQueueFamilyCheckpointPropertiesNV).
			*
			* @param[out] out
			*     Структура, в которую будут записаны расширенные свойства выбранного семейства очередей.
			*
			* @note
			*     Метод имеет смысл использовать только при наличии поддержки Vulkan 1.1+ или расширения VK_KHR_get_physical_device_properties2,
			*     и если структура Properties валидна для устройства.
			*
			* @code
			* VkQueueFamilyCheckpointPropertiesNV checkpointProps{};
			* queue_family.requestProperties(checkpointProps);
			* // checkpointProps теперь содержит расширенные свойства семейства очередей
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Properties>
			inline void requestProperties(Properties& out) const noexcept;
					
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief 
			*
			* функция рабочая, но это какой-то черный ящик сгенерированный gpt
			* оставляю для дальнейшего изучения
			* пользуйся осторожно, лучше не пользуйся ;))
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename ... Chains>
			inline void requestQueueFamilyProperties(Chains &... chains) const noexcept;

		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief Проверяет валидность структуры WvkQueueFamilyCreateInfo, если включена сборка с валидацией.
			* 
			* @return [out] WvkStatus Статус проверки:
			* - VknStatusCode::SUCCESSFUL — если все обязательные поля заданы корректно;
			* - VknStatusCode::CREATE_INFO_NO_VALID — если структура содержит ошибочные или отсутствующие данные.
			* 
			* Проверка выполняется только при активной сборке с валидацией (`wvk::Build::ValidationBuildInfo::enable == true`).
			* В противном случае метод всегда возвращает `VknStatusCode::SUCCESSFUL`.
			* 
			* @code
			* WvkQueueFamily queue_family;
			* WvkStatus status = queue_family.validationCreateInfo();
			* if (status.code != VknStatusCode::SUCCESSFUL) {
			*     std::cerr << "Ошибка валидации: " << status.message << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void reset(void) noexcept;

		private:
				
			WvkQueueFamilyCreateInfo m_create_info = {};
		}; // class WvkQueueFamily

	} // namespace wvk

} // namespace CGDev

#include "wvk_queue_family.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_H
