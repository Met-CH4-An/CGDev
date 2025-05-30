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

					class			VknExtDebugUtils;
					using			VknExtDebugUtilsPtr = VknExtDebugUtils * ;
					using			VknExtDebugUtilsPtrArr = std::vector<VknExtDebugUtilsPtr>;
					using			VknExtDebugUtilsWptr = std::weak_ptr<VknExtDebugUtils>;
					using			VknExtDebugUtilsWptrArr = std::vector<VknExtDebugUtilsWptr>;
					using			VknExtDebugUtilsSptr = std::shared_ptr<VknExtDebugUtils>;
					using			VknExtDebugUtilsSptrArr = std::vector<VknExtDebugUtilsSptr>;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class WvkKhrGetPhysicalDeviceProperties2DT;
					using WvkKhrGetPhysicalDeviceProperties2DTPtr = WvkKhrGetPhysicalDeviceProperties2DT * ;
					using WvkKhrGetPhysicalDeviceProperties2DTPtrArr1 = std::vector<WvkKhrGetPhysicalDeviceProperties2DTPtr>;
					using WvkKhrGetPhysicalDeviceProperties2DTSptr = std::shared_ptr<WvkKhrGetPhysicalDeviceProperties2DT>;
					using WvkKhrGetPhysicalDeviceProperties2DTSptrArr1 = std::vector<WvkKhrGetPhysicalDeviceProperties2DTSptr>;
					using WvkKhrGetPhysicalDeviceProperties2DTUptr = std::unique_ptr<WvkKhrGetPhysicalDeviceProperties2DT>;
					using WvkKhrGetPhysicalDeviceProperties2DTUptrArr1 = std::vector<WvkKhrGetPhysicalDeviceProperties2DTUptr>;
					using WvkKhrGetPhysicalDeviceProperties2DTWptr = std::weak_ptr<WvkKhrGetPhysicalDeviceProperties2DT>;
					using WvkKhrGetPhysicalDeviceProperties2DTWptrArr1 = std::vector<WvkKhrGetPhysicalDeviceProperties2DTWptr>;

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
					
				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS___EXTENSIONS_H