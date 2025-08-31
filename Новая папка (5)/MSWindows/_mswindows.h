#ifndef CGDEV_WVK_SOURCE_MSWINDOWS___MSWINDOWS_H
#define CGDEV_WVK_SOURCE_MSWINDOWS___MSWINDOWS_H
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
#include <Windows.h>
#include "vulkan/vulkan_win32.h"

namespace CGDev {

	namespace wvk {

		namespace mswindows {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkDispatchTableMSWindows;
			using WvkInstanceMSWindowsPtr = WvkDispatchTableMSWindows * ;
			using WvkInstanceMSWindowsSptr = std::shared_ptr<WvkDispatchTableMSWindows>;
			using WvkInstanceMSWindowsUptr = std::unique_ptr<WvkDispatchTableMSWindows>;
			using WvkInstanceMSWindowsWptr = std::weak_ptr<WvkDispatchTableMSWindows>;
			
		} // namespace mswindows

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_MSWINDOWS___MSWINDOWS_H