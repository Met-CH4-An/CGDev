#ifndef CGDEV_SOURCE_RPI_PRIVATE_VULKAN__LIBRARYVULKAN_H
#define CGDEV_SOURCE_RPI_PRIVATE_VULKAN__LIBRARYVULKAN_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_rpi_vulkan.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../hcpi_library.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace RPI {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct HCPI_LIBRARY_VULKAN_CREATE_INFO {

					HCPILibraryPtr						library = nullptr;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool isValid(void) const noexcept;

				}; // struct HCPI_LIBRARY_VULKAN_CREATE_INFO



				

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class HCPILibraryVulkan : public Object {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					HCPILibraryVulkan(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~HCPILibraryVulkan(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void create(const HCPI_LIBRARY_VULKAN_CREATE_INFO& create_info) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const ExtensionSptrArr& getExtensionSupported(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const ExtensionSptrArr& getExtensionEnabled(void) const noexcept;
					
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VulkanCommandSptr& getVulkanCommand(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const PhysicalDeviceSptrArrArr& getPhysicalDeviceGroup(void) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void createLoader(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void createVulkanCommand(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//void initializeVulkanCommand(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//void prepareVersion(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void prepareSupportedLayersAndExtensions(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void prepareEnabledLayersAndExtensions(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//void prepareDebug(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void createInstance(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void createDebug(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void createPhysicalDevice(void) noexcept;

				protected:

					PFN_vkGetInstanceProcAddr		m_vkGetInstanceProcAddr = nullptr;

				private:

					HCPI_LIBRARY_VULKAN_CREATE_INFO					m_create_info = {};
					LoaderSptr										m_loader = nullptr;
					Extensions::VkExtDebugUtilsSptr					m_extension_debug_utils = nullptr;

					LibraryVulkanVersion							m_library_vulkan_version = LibraryVulkanVersion::UNKNOWN;

					LayerSptrArr									m_layer_supported_collection;
					LayerSptrArr									m_layer_enabled_collection;
					ExtensionSptrArr								m_extension_supported_collection;
					ExtensionSptrArr								m_extension_enabled_collection;

					VulkanCommandSptr								m_vulkan_command = nullptr;

					InstanceSptr									m_instance = nullptr;
					PhysicalDeviceSptrArrArr						m_physical_device_group_collection = {};

				}; // class HCPILibraryVulkan

			} // namespace Vulkan

		} // namespace Private

	} // namespace RPI

} // namespace CGDev

#include "hcpi_library_vulkan.hpp"

#endif // CGDEV_SOURCE_RPI_PRIVATE_VULKAN__LIBRARYVULKAN_H
