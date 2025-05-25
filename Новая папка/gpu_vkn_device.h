#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_DEVICE_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_DEVICE_H
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

				enum class GpuVknDeviceVknQueueInfoType {

					UNKNOWN					= 0,
					GRAPHICS				= 1,
					COMPUTE					= 2,
					VIDEO_DECODE			= 3,
					MAX						= VIDEO_DECODE + 1

				}; // enum class GpuVknDeviceVknQueueInfoType





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct GpuVknDeviceVknQueueInfo {

					VknQueuePtr								vkn_queue = nullptr;
					GpuVknDeviceVknQueueInfoType			type = GpuVknDeviceVknQueueInfoType::UNKNOWN;

				}; // struct GpuVknDeviceVknQueueInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct GpuVknDeviceCreateInfo {

					Tools::LogPtr					log = nullptr;
					GpuDevicePtr					gpu_device = nullptr;

				}; // struct GpuVknDeviceCreateInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class GpuVknDevice : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					GpuVknDevice(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~GpuVknDevice(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool create(const GpuVknDeviceCreateInfo& create_info) noexcept;

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
					bool execute(void) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//inline const LogicalDeviceSptr& getLogicalDevice(void) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* 
					* Подготовка и создание VknLogicalDevice
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool prepareVknLogicalDevice(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					* 
					* Подготовка и создание VknQueue
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool prepareVknQueue(void) noexcept;

					bool findQueueFamily();
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//void createCommandPool(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//void createRenderMap(void) noexcept;

				private:

					GpuVknDeviceCreateInfo							m_create_info = {};
					VknLogicalDevicePtr								m_vkn_logical_device = nullptr;
					VknQueuePtr										m_queue_graphics = {};
					VknQueuePtr										m_queue_compute = {};
					VknQueuePtr										m_queue_video_decode = {};
										
				}; // class GpuVknDevice

			} // namespace Vulkan

		} // namespace Private

	} // namespace GPU

} // namespace CGDev

#include "gpu_vkn_device.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_DEVICE_H
