#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_LIBRARY_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_LIBRARY_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_vulkan.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../gpu_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace GPU {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct GpuVknLibraryCreateInfo {

					Tools::LogPtr					log = nullptr;
					GpuLibraryPtr					gpu_library = nullptr;

				}; // struct GpuVknLibraryCreateInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class GpuVknLibrary : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					GpuVknLibrary(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~GpuVknLibrary(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool create(const GpuVknLibraryCreateInfo& create_info) noexcept;

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
					bool checkValidation(void) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const GpuVknLibraryCreateInfo& getCreateInfo(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VknCommandsPtr& getVknCommandsGlobalLevel(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VknCommandsPtr& getVknCommandsInstanceLevel(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VknInstancePtr& getVknInstance(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VknPhysicalDevicePtrArr2& getVknPhysicalDevice(void) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool validationCreateInfo(const GpuVknLibraryCreateInfo& create_info) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool createVknLoader(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					Status createVknInstance(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool prepareVknPhysicalDevice(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool prepareNextForDeviceGroupDeviceCreateInfo(VkPhysicalDeviceArr2& vk_physical_device_collection2) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//bool prepareVknPhysicalDevice(VknPhysicalDevicePtr& vkn_physical_device, const VkPhysicalDevice& vk_physical_device) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool createVknCommandsGlobalLevel(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool createVknCommandsInstanceLevel(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool createVknExtDebugUtils(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Создаёт объекты VknQueueFamily для всех доступных семейств очередей данного физического устройства.
					*
					* Эта функция запрашивает у Vulkan список всех поддерживаемых семейств очередей(`VkQueueFamilyProperties`)
					* для текущего физического устройства, создает соответствующие объекты `VknQueueFamily`,
					* и сохраняет их во внутреннюю коллекцию `m_vkn_queue_family_collection`.
					*
					* Алгоритм работы :
					* 1. Первый вызов `vkGetPhysicalDeviceQueueFamilyProperties` используется для получения количества доступных семейств.
					* 2. Затем выделяется буфер нужного размера для хранения свойств всех семейств.
					* 3. Второй вызов `vkGetPhysicalDeviceQueueFamilyProperties` заполняет этот буфер данными.
					* 4. Каждый элемент буфера используется для создания объекта `VknQueueFamily`.
					* 5. Все успешно созданные объекты помещаются в вектор `m_vkn_queue_family_collection`.
					*
					* Если создание любого из объектов завершается с ошибкой, функция немедленно возвращает соответствующий статус с описанием.
					*
					* @return VknStatus
					* Возвращает статус выполнения.
					* Если статус `VknStatusCode::SUCCESSFUL`, все семейства были успешно созданы.
					* В противном случае статус будет содержать описание ошибки.
					*
					* \note Эта функция должна вызываться один раз при инициализации физического устройства.
					* Повторный вызов без очистки `m_vkn_queue_family_collection` может привести к накоплению лишних объектов.
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VknStatus createVknQueueFamily(void) noexcept;

				private:

					GpuVknLibraryCreateInfo								m_create_info = {};
					VknLoaderPtr										s_vkn_loader = nullptr;				
					VknInstancePtr										m_vkn_instance = nullptr;
					VknPhysicalDevicePtrArr2							m_vkn_physical_device_collection2 = {};
					VknPhysicalDevicePtrArr2							m_vkn_physical_device_free_collection2 = {};
					VknCommandsPtr										s_vkn_commands_global_level = nullptr;
					VknCommandsPtr										s_vkn_commands_instance_level = nullptr;
					VknExtensionPtr										m_vkn_ext_debug_utils = nullptr;

				}; // class GpuVknLibrary

			} // namespace Vulkan

		} // namespace Private

	} // namespace GPU

} // namespace CGDev

#include "gpu_vkn_library.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_LIBRARY_H
