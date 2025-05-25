#ifndef CGDEV_SOURCE_RPI_PRIVATE_VULKAN_EXTENSIONS__VKEXTDEBUGREPORT_H
#define CGDEV_SOURCE_RPI_PRIVATE_VULKAN_EXTENSIONS__VKEXTDEBUGREPORT_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_extensions.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../extension.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace RPI {

		//namespace Private {

			namespace wvk {

				namespace Extensions {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					class VkExtDebugReport : public Extension {

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						inline VkExtDebugReport(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						inline ~VkExtDebugReport(void) noexcept;

					}; // class VkExtDebugReport

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace RPI

} // namespace CGDev

#include "vk_ext_debug_report.hpp"

#endif // CGDEV_SOURCE_RPI_PRIVATE_VULKAN_EXTENSIONS__VKEXTDEBUGREPORT_H
