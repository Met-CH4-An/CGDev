#ifndef CGDEV_WVK_SOURCE__WVK_LOGICAL_DEVICE_H
#define CGDEV_WVK_SOURCE__WVK_LOGICAL_DEVICE_H
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
		struct WvkLogicalDeviceFeature {
			std::string identifier = {};		
			VkBaseInStructure* vk_base_in = nullptr;

			template<typename In>
			WvkLogicalDeviceFeature(const char* extension_name, In& feature) {
				vk_base_in = reinterpret_cast<VkBaseInStructure*>(&feature);
			}
		};
		using WvkLogicalDeviceFeatureVec1 = std::vector<WvkLogicalDeviceFeature>;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkLogicalDeviceQueueCreateInfo {
			WvkQueueFamilyPtr wvk_queue_family_ptr = nullptr;
			std::optional<uint32_t> queue_count;
			std::vector<float> priority_collection = {};
		}; // struct WvkLogicalDeviceQueueCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkLogicalDeviceCreateInfo {
			WvkInstanceDispatchTablePtr wvk_instance_dispatch_table = nullptr;
			WvkPhysicalDevicePtrVec1 wvk_physical_device_collection;
			WvkLogicalDeviceQueueCreateInfoVec1 wvk_logical_device_queue_create_info_collection = {};
			VkPhysicalDeviceFeatures m_vk_physical_device_features = {};
			WvkLogicalDeviceFeatureVec1 m_wvk_logical_device_feature_collection;
		}; // struct WvkLogicalDeviceCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkLogicalDevice : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkLogicalDevice(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkLogicalDevice(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkLogicalDeviceCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		// hpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkLogicalDeviceCreateInfo& getCreateInfo(void) const noexcept;

		// hpp
		private:

			friend class WvkLogicalDeviceDispatchTable;
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const VkDevice& getVkDevice(void) const noexcept;

		// cpp
		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

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
			WvkStatus prepareVkQueueCreateInfo(std::vector<VkDeviceQueueCreateInfo>& queue_create_info_collection1) const noexcept;

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
			* WvkLogicalDevice device;
			* WvkStatus status = device.createVkDevice();
			* if (status.failed()) {
			*     log->error(status.message_old);
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createVkDevice(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void reset(void) noexcept;

		private:
					
			WvkLogicalDeviceCreateInfo						m_create_info = {};
			VkDevice										m_vk_device = VK_NULL_HANDLE;
		}; // class WvkLogicalDevice

	} // namespace wvk

} // namespace CGDev

#include "wvk_logical_device.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_LOGICAL_DEVICE_H
