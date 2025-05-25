#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_LOGICAL_DEVICE_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_LOGICAL_DEVICE_H
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

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct VknLogicalDeviceQueueCreateInfo {

					VknQueueFamilyPtr				wvk_queue_family = nullptr;
					std::optional<uint32_t> queue_count;
					std::vector<float>				priority_collection = {};

				}; // struct VknLogicalDeviceQueueCreateInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct VknLogicalDeviceCreateInfo {

					WvkCommandsPtr											wvk_commands = nullptr;
					WvkPhysicalDevicePtrArr1								wvk_physical_device_collection;
					VknLogicalDeviceQueueCreateInfoArr						wvk_logical_device_queue_create_info_collection = {};

				}; // struct VknLogicalDeviceCreateInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class VknLogicalDevice : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VknLogicalDevice(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~VknLogicalDevice(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool create(const VknLogicalDeviceCreateInfo& create_info) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void destroy(void) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VknLogicalDeviceCreateInfo& getCreateInfo(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VkDevice& getVkDevice(void) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus validationCreateInfo(const VknLogicalDeviceCreateInfo& create_info) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	brief Подготавливает коллекцию VkDeviceQueueCreateInfo на основе пользовательской конфигурации.
					*
					* Метод инициализирует структуру VkDeviceQueueCreateInfo для каждой очереди,
					* указанной в коллекции `wvk_logical_device_queue_create_info_collection`, и
					* добавляет их в выходной контейнер `queue_create_info_collection1`.
					*
					* @param[out] queue_create_info_collection1 Коллекция, куда добавляются описания очередей.
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void prepareVkQueueCreateInfo(VkDeviceQueueCreateInfoArr1& queue_create_info_collection1) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Создаёт объект логического устройства Vulkan (`VkDevice`) на основе заданного физического устройства и настроек очередей.
					* 
					* Метод инициализирует структуру `VkDeviceCreateInfo`, используя ранее подготовленные параметры очередей (`VkDeviceQueueCreateInfoArr1`),
					* и вызывает функцию `vknCreateDevice` для создания устройства. Ошибки Vulkan логируются в сообщение объекта `VknStatus`.
					* 
					* @note Метод вызывается только один раз при инициализации логического устройства.
					* @note В текущей реализации используется только первое физическое устройство из коллекции.
					* 
					* @return [out] WvkStatus — результат создания логического устройства: SUCCESSFUL или FAIL с описанием ошибки.
					* 
					* @code
					* VknLogicalDevice device;
					* WvkStatus status = device.createVkDevice();
					* if (status.failed()) {
					*     log->error(status.message_old);
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus createVkDevice(void) noexcept;

				private:
					
					VknLogicalDeviceCreateInfo						m_create_info = {};
					//VkPhysicalDevice								m_vk_physical_device = VK_NULL_HANDLE;
					//void*											m_next = VK_NULL_HANDLE;
					//std::vector<void*>								m_next_collection1;
					//std::vector<void*>								m_allocate_memory_collection1;
					VkDevice										m_vk_device = VK_NULL_HANDLE;
					//VkPhysicalDeviceArr1							m_vk_physical_device_collection;

				}; // class VknLogicalDevice

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_logical_device.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_LOGICAL_DEVICE_H
