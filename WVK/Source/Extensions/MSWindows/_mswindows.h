#ifndef CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS___MSWINDOWS_H
#define CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS___MSWINDOWS_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../_extensions.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include <Windows.h>
#include "vulkan/vulkan_win32.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			namespace mswindows {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class			WvkKhrWin32SurfaceDispatchTable;
				using			WvkKhrWin32SurfaceDispatchTablePtr = WvkKhrWin32SurfaceDispatchTable * ;
				using			WvkKhrWin32SurfaceDispatchTablePtrArr1 = std::vector<WvkKhrWin32SurfaceDispatchTablePtr>;
				using			WvkKhrWin32SurfaceDispatchTableSptr = std::shared_ptr<WvkKhrWin32SurfaceDispatchTable>;
				using			WvkKhrWin32SurfaceDispatchTableSptrArr1 = std::vector<WvkKhrWin32SurfaceDispatchTableSptr>;
				using			WvkKhrWin32SurfaceDispatchTableUptr = std::unique_ptr<WvkKhrWin32SurfaceDispatchTable>;
				using			WvkKhrWin32SurfaceDispatchTableUptrArr1 = std::vector<WvkKhrWin32SurfaceDispatchTableUptr>;
				using			WvkKhrWin32SurfaceDispatchTableWptr = std::weak_ptr<WvkKhrWin32SurfaceDispatchTable>;
				using			WvkKhrWin32SurfaceDispatchTableWptrArr1 = std::vector<WvkKhrWin32SurfaceDispatchTableWptr>;
				

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class			WvkSurfaceMSWindows;
				using			WvkSurfaceMSWindowsPtr = WvkSurfaceMSWindows * ;
				using			WvkSurfaceMSWindowsPtrArr1 = std::vector<WvkSurfaceMSWindowsPtr>;
				using			WvkSurfaceMSWindowsWptr = std::weak_ptr<WvkSurfaceMSWindows>;
				using			WvkSurfaceMSWindowsWptrArr1 = std::vector<WvkSurfaceMSWindowsWptr>;
				using			WvkSurfaceMSWindowsSptr = std::shared_ptr<WvkSurfaceMSWindows>;
				using			WvkSurfaceMSWindowsSptrArr1 = std::vector<WvkSurfaceMSWindowsSptr>;

			} // namespace mswindows

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS_MSWINDOWS___MSWINDOWS_H