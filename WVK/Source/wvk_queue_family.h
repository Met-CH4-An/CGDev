#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_H
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
#include "wvk_commands.h"
#include "wvk_physical_device.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct VknQueueFamilyCreateInfo {

					std::optional<uint32_t>			index;
					WvkCommandsPtr					wvk_commands = nullptr;
					WvkPhysicalDevicePtr			wvk_physical_device = nullptr;

				}; // struct VknQueueFamilyCreateInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class VknQueueFamily : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VknQueueFamily(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~VknQueueFamily(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Инициализирует объект VknQueueFamily, сохраняя переданные параметры и (опционально) проверяя их валидность.
					* 
					* @param [in] create_info Структура с параметрами для создания VknQueueFamily (указатель на физическое устройство, индекс семейства, указатель на команды и т.п.).
					* 
					* @return [out] WvkStatus Возвращает статус операции:
					* - VknStatusCode::SUCCESSFUL — если инициализация успешна;
					* - VknStatusCode::CREATE_INFO_NO_VALID — если включена валидация и входные данные невалидны.
					* 
					* Метод сохраняет входную структуру `create_info` и при активной валидации выполняет проверку её корректности.
					* 
					* @code
					* VknQueueFamily queue_family;
					* VknQueueFamilyCreateInfo create_info = { ... };
					* WvkStatus status = queue_family.create(create_info);
					* if (status.code != VknStatusCode::SUCCESSFUL) {
					*     std::cerr << "Ошибка: " << status.message << std::endl;
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus create(const VknQueueFamilyCreateInfo& create_info) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void destroy(void) noexcept;					

				public:

					inline const VknQueueFamilyCreateInfo& getCreateInfo(void) const noexcept;

					inline const uint32_t getIndexFamily(void) const noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Извлекает свойства конкретного семейства очередей физического устройства.
					* 
					* Используется для получения характеристик семейства очередей, таких как поддержка графики, compute и transfer операций.
					* Метод сначала получает общее количество семейств, затем извлекает информацию обо всех, и в конце возвращает данные по заданному индексу.
					* 
					* @param[out] vk_queue_family_properties Структура, в которую будут записаны свойства семейства очередей.
					* 
					* @note Метод предполагает, что `m_create_info.index` уже валиден.
					*
					* @code
					* VkQueueFamilyProperties props;
					* queue_family.requestQueueFamilyProperties(props);
					* // props теперь содержит, например, flags и количество очередей
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline void requestQueueFamilyProperties(VkQueueFamilyProperties& vk_queue_family_properties) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Запрашивает свойства указанного семейства очередей с использованием структуры VkQueueFamilyProperties2.
					*
					* Используется в случае, если приложение собрано с поддержкой Vulkan 1.1 или выше. Метод позволяет
					* получить расширенные свойства семейства очередей, потенциально включая цепочки расширений через `pNext`.
					* На данный момент цепочки не используются (pNext = nullptr).
					*
					* @param[out] vk_queue_family_properties2 Структура, в которую будут записаны свойства интересующего семейства.
					*
					* @note Метод не делает ничего, если сборка использует Vulkan ниже версии 1.1.
					*
					* @code
					* VkQueueFamilyProperties2 props2{};
					* queue_family.requestQueueFamilyProperties(props2);
					* // props2.queueFamilyProperties содержит флаги, queueCount и прочее
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline void requestQueueFamilyProperties(VkQueueFamilyProperties2& vk_queue_family_properties2) const noexcept;
					
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Запрашивает расширенные свойства семейства очередей устройства.
					*
					* \tparam VkQueueFamilyXProperties
					* Расширенный тип структуры, который должен быть связан с VkQueueFamilyProperties2 через pNext.
					* Например: `VkQueueFamilyCheckpointPropertiesNV`, `VkQueueFamilyGlobalPriorityPropertiesEXT` и т.д.
					*
					* \param [out] vk_queue_family_x_properties
					* Структура, в которую будут записаны свойства выбранного семейства очередей.
					*
					* \details
					* Метод проверяет, поддерживается ли Vulkan 1.1 или выше, и если да — производит
					* следующий порядок действий:
					*
					* - Шаг 1. Получает количество доступных семейств очередей через `vknGetPhysicalDeviceQueueFamilyProperties2`.
					* - Шаг 2. Создаёт массив структур `VkQueueFamilyProperties2` и соответствующих расширенных структур `VkQueueFamilyXProperties`.
					* - Шаг 3. Инициализирует поля `sType` и `pNext` для каждой пары структур.
					* - Шаг 4. Повторно запрашивает свойства всех семейств, теперь уже с заполнением.
					* - Шаг 5. Копирует расширенные свойства семейства очередей по индексу `m_create_info.index` в выходной параметр.
					*
					* Если активная версия Vulkan ниже 1.1, метод ничего не делает.
					*
					* \note
					* Использование этого метода имеет смысл только при наличии расширения/поддержки структуры `VkQueueFamilyXProperties`,
					* и если она валидна для версии Vulkan и конкретного физического устройства.
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					template<typename VkQueueFamilyXProperties>
					inline void requestQueueFamilyProperties(VkQueueFamilyXProperties& vk_queue_family_x_properties) const noexcept;

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
					/*!	\brief Проверяет валидность структуры VknQueueFamilyCreateInfo, если включена сборка с валидацией.
					* 
					* @return [out] WvkStatus Статус проверки:
					* - VknStatusCode::SUCCESSFUL — если все обязательные поля заданы корректно;
					* - VknStatusCode::CREATE_INFO_NO_VALID — если структура содержит ошибочные или отсутствующие данные.
					* 
					* Проверка выполняется только при активной сборке с валидацией (`wvk::Build::ValidationBuildInfo::enable == true`).
					* В противном случае метод всегда возвращает `VknStatusCode::SUCCESSFUL`.
					* 
					* @code
					* VknQueueFamily queue_family;
					* WvkStatus status = queue_family.validationCreateInfo();
					* if (status.code != VknStatusCode::SUCCESSFUL) {
					*     std::cerr << "Ошибка валидации: " << status.message << std::endl;
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus validationCreateInfo(void) const noexcept;

				private:
				
					VknQueueFamilyCreateInfo		m_create_info = {};

				}; // class VknQueueFamily

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_queue_family.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_H
