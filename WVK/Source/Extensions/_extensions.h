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

					class			WvkKhrSurfaceDispatchTable;
					using			WvkKhrSurfaceDispatchTablePtr = WvkKhrSurfaceDispatchTable * ;
					using			WvkKhrSurfaceDispatchTablePtrArr1 = std::vector<WvkKhrSurfaceDispatchTablePtr>;
					using			WvkKhrSurfaceDispatchTableSptr = std::shared_ptr<WvkKhrSurfaceDispatchTable>;
					using			WvkKhrSurfaceDispatchTableSptrArr1 = std::vector<WvkKhrSurfaceDispatchTableSptr>;
					using			WvkKhrSurfaceDispatchTableUptr = std::unique_ptr<WvkKhrSurfaceDispatchTable>;
					using			WvkKhrSurfaceDispatchTableUptrArr1 = std::vector<WvkKhrSurfaceDispatchTableUptr>;
					using			WvkKhrSurfaceDispatchTableWptr = std::weak_ptr<WvkKhrSurfaceDispatchTable>;
					using			WvkKhrSurfaceDispatchTableWptrArr1 = std::vector<WvkKhrSurfaceDispatchTableWptr>;
					

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

					class			WvkKhrGetSurfaceCapabilities2;
					using			WvkKhrGetSurfaceCapabilities2Ptr = WvkKhrGetSurfaceCapabilities2 * ;
					using			WvkKhrGetSurfaceCapabilities2PtrArr1 = std::vector<WvkKhrGetSurfaceCapabilities2Ptr>;
					using			WvkKhrGetSurfaceCapabilities2Wptr = std::weak_ptr<WvkKhrGetSurfaceCapabilities2>;
					using			WvkKhrGetSurfaceCapabilities2WptrArr1 = std::vector<WvkKhrGetSurfaceCapabilities2Wptr>;
					using			WvkKhrGetSurfaceCapabilities2Sptr = std::shared_ptr<WvkKhrGetSurfaceCapabilities2>;
					using			WvkKhrGetSurfaceCapabilities2SptrArr1 = std::vector<WvkKhrGetSurfaceCapabilities2Sptr>;
					
				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS___EXTENSIONS_H