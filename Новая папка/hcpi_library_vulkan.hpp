#ifndef CGDEV_SOURCE_RPI_PRIVATE_VULKAN__LIBRARYOBJECTVULKAN_HPP
#define CGDEV_SOURCE_RPI_PRIVATE_VULKAN__LIBRARYOBJECTVULKAN_HPP
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

	namespace RPI {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline bool HCPI_LIBRARY_VULKAN_CREATE_INFO::isValid(void) const noexcept {

					if (library == nullptr) {

						return false;
					}

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const ExtensionSptrArr& HCPILibraryVulkan::getExtensionSupported(void) const noexcept {

					return m_extension_supported_collection;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const ExtensionSptrArr& HCPILibraryVulkan::getExtensionEnabled(void) const noexcept {

					return m_extension_enabled_collection;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const VulkanCommandSptr& HCPILibraryVulkan::getVulkanCommand(void) const noexcept {

					return m_vulkan_command;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const PhysicalDeviceSptrArrArr& HCPILibraryVulkan::getPhysicalDeviceGroup(void) const noexcept {

					return m_physical_device_group_collection;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace Vulkan

		} // namespace Private

	} // namespace RPI

} // namespace CGDev

#endif // CGDEV_SOURCE_RPI_PRIVATE_VULKAN__LIBRARYOBJECTVULKAN_HPP
