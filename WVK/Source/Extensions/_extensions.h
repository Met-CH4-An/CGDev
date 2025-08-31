// SPDX-License-Identifier: AGPL-3.0-or-later
#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS___EXTENSIONS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS___EXTENSIONS_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../_wvk.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Extensions {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class			VknExtDebugUtilsCommands;
					using			VknExtDebugUtilsCommandsPtr = VknExtDebugUtilsCommands * ;
					using			VknExtDebugUtilsCommandsPtrArr = std::vector<VknExtDebugUtilsCommandsPtr>;
					using			VknExtDebugUtilsCommandsWptr = std::weak_ptr<VknExtDebugUtilsCommands>;
					using			VknExtDebugUtilsCommandsWptrArr = std::vector<VknExtDebugUtilsCommandsWptr>;
					using			VknExtDebugUtilsCommandsSptr = std::shared_ptr<VknExtDebugUtilsCommands>;
					using			VknExtDebugUtilsCommandsSptrArr = std::vector<VknExtDebugUtilsCommandsSptr>;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class WvkDebugUtilsMessenger;
					using WvkDebugUtilsMessengerPtr = WvkDebugUtilsMessenger * ;
					using WvkDebugUtilsMessengerPtrVec1 = std::vector<WvkDebugUtilsMessengerPtr>;
					using WvkDebugUtilsMessengerSptr = std::shared_ptr<WvkDebugUtilsMessenger>;
					using WvkDebugUtilsMessengerSptrVec1 = std::vector<WvkDebugUtilsMessengerSptr>;
					using WvkDebugUtilsMessengerUptr = std::unique_ptr<WvkDebugUtilsMessenger>;
					using WvkDebugUtilsMessengerUptrVec1 = std::vector<WvkDebugUtilsMessengerUptr>;
					using WvkDebugUtilsMessengerWptr = std::weak_ptr<WvkDebugUtilsMessenger>;
					using WvkDebugUtilsMessengerWptrVec1 = std::vector<WvkDebugUtilsMessengerWptr>;
					

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class WvkKhrGetPhysicalDeviceProperties2DispatchTable;
					using WvkKhrGetPhysicalDeviceProperties2DispatchTablePtr = WvkKhrGetPhysicalDeviceProperties2DispatchTable * ;
					using WvkKhrGetPhysicalDeviceProperties2DispatchTablePtrArr1 = std::vector<WvkKhrGetPhysicalDeviceProperties2DispatchTablePtr>;
					using WvkKhrGetPhysicalDeviceProperties2DispatchTableSptr = std::shared_ptr<WvkKhrGetPhysicalDeviceProperties2DispatchTable>;
					using WvkKhrGetPhysicalDeviceProperties2DispatchTableSptrArr1 = std::vector<WvkKhrGetPhysicalDeviceProperties2DispatchTableSptr>;
					using WvkKhrGetPhysicalDeviceProperties2DispatchTableUptr = std::unique_ptr<WvkKhrGetPhysicalDeviceProperties2DispatchTable>;
					using WvkKhrGetPhysicalDeviceProperties2DispatchTableUptrArr1 = std::vector<WvkKhrGetPhysicalDeviceProperties2DispatchTableUptr>;
					using WvkKhrGetPhysicalDeviceProperties2DispatchTableWptr = std::weak_ptr<WvkKhrGetPhysicalDeviceProperties2DispatchTable>;
					using WvkKhrGetPhysicalDeviceProperties2DispatchTableWptrArr1 = std::vector<WvkKhrGetPhysicalDeviceProperties2DispatchTableWptr>;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class WvkKhrSurfaceDT;
					using WvkKhrSurfaceDTPtr = WvkKhrSurfaceDT * ;
					using WvkKhrSurfaceDTPtrArr1 = std::vector<WvkKhrSurfaceDTPtr>;
					using WvkKhrSurfaceDTSptr = std::shared_ptr<WvkKhrSurfaceDT>;
					using WvkKhrSurfaceDTSptrArr1 = std::vector<WvkKhrSurfaceDTSptr>;
					using WvkKhrSurfaceDTUptr = std::unique_ptr<WvkKhrSurfaceDT>;
					using WvkKhrSurfaceDTUptrArr1 = std::vector<WvkKhrSurfaceDTUptr>;
					using WvkKhrSurfaceDTWptr = std::weak_ptr<WvkKhrSurfaceDT>;
					using WvkKhrSurfaceDTWptrArr1 = std::vector<WvkKhrSurfaceDTWptr>;
					
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class			WvkKhrGetSurfaceCapabilities2DT;
					using			WvkKhrGetSurfaceCapabilities2DispatchTablePtr = WvkKhrGetSurfaceCapabilities2DT * ;
					using			WvkKhrGetSurfaceCapabilities2DispatchTablePtrArr1 = std::vector<WvkKhrGetSurfaceCapabilities2DispatchTablePtr>;
					using			WvkKhrGetSurfaceCapabilities2DispatchTableSptr = std::shared_ptr<WvkKhrGetSurfaceCapabilities2DT>;
					using			WvkKhrGetSurfaceCapabilities2DispatchTableSptrArr1 = std::vector<WvkKhrGetSurfaceCapabilities2DispatchTableSptr>;
					using			WvkKhrGetSurfaceCapabilities2DispatchTableUptr = std::unique_ptr<WvkKhrGetSurfaceCapabilities2DT>;
					using			WvkKhrGetSurfaceCapabilities2DispatchTableUptrArr1 = std::vector<WvkKhrGetSurfaceCapabilities2DispatchTableUptr>;
					using			WvkKhrGetSurfaceCapabilities2DispatchTableWptr = std::weak_ptr<WvkKhrGetSurfaceCapabilities2DT>;
					using			WvkKhrGetSurfaceCapabilities2DispatchTableWptrArr1 = std::vector<WvkKhrGetSurfaceCapabilities2DispatchTableWptr>;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class			WvkSurface;
					using			WvkSurfacePtr = WvkSurface * ;
					using			WvkSurfacePtrArr1 = std::vector<WvkSurfacePtr>;
					using			WvkSurfaceSptr = std::shared_ptr<WvkSurface>;
					using			WvkSurfaceSptrArr1 = std::vector<WvkSurfaceSptr>;
					using			WvkSurfaceUptr = std::unique_ptr<WvkSurface>;
					using			WvkSurfaceUptrArr1 = std::vector<WvkSurfaceUptr>;
					using			WvkSurfaceWptr = std::weak_ptr<WvkSurface>;
					using			WvkSurfaceWptrArr1 = std::vector<WvkSurfaceWptr>;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class			WvkSwapchain;
					using			WvkSwapchainPtr = WvkSwapchain*;
					using			WvkSwapchainPtrArr1 = std::vector<WvkSwapchainPtr>;
					using			WvkSwapchainSptr = std::shared_ptr<WvkSwapchain>;
					using			WvkSwapchainSptrArr1 = std::vector<WvkSwapchainSptr>;
					using			WvkSwapchainUptr = std::unique_ptr<WvkSwapchain>;
					using			WvkSwapchainUptrArr1 = std::vector<WvkSwapchainUptr>;
					using			WvkSwapchainWptr = std::weak_ptr<WvkSwapchain>;
					using			WvkSwapchainWptrArr1 = std::vector<WvkSwapchainWptr>;
					
				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS___EXTENSIONS_H