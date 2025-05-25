#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_LIBRARY_HPP
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_LIBRARY_HPP
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace GPU {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const GpuVknLibraryCreateInfo& GpuVknLibrary::getCreateInfo(void) const noexcept {

					return m_create_info;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const VknCommandsPtr& GpuVknLibrary::getVknCommandsGlobalLevel(void) const noexcept {

					return s_vkn_commands_global_level;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const VknCommandsPtr& GpuVknLibrary::getVknCommandsInstanceLevel(void) const noexcept {

					return s_vkn_commands_instance_level;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const VknInstancePtr& GpuVknLibrary::getVknInstance(void) const noexcept {

					return m_vkn_instance;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const VknPhysicalDevicePtrArr2& GpuVknLibrary::getVknPhysicalDevice(void) const noexcept {

					return m_vkn_physical_device_collection2;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace Vulkan

		} // namespace Private

	} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__GPU_VKN_LIBRARY_HPP
