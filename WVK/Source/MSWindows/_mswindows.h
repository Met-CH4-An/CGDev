#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_MSWINDOWS___MSWINDOWS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_MSWINDOWS___MSWINDOWS_H
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

namespace CGDev {

	namespace wvk {

		namespace mswindows {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			class			WvkLoaderMSWindows;
			using			WvkLoaderMSWindowsPtr = WvkLoaderMSWindows * ;
			using			WvkLoaderMSWindowsPtrArr1 = std::vector<WvkLoaderMSWindowsPtr>;
			using			WvkLoaderMSWindowsSptr = std::shared_ptr<WvkLoaderMSWindows>;
			using			WvkLoaderMSWindowsSptrArr1 = std::vector<WvkLoaderMSWindowsSptr>;
			using			WvkLoaderMSWindowsUptr = std::unique_ptr<WvkLoaderMSWindows>;
			using			WvkLoaderMSWindowsUptrArr1 = std::vector<WvkLoaderMSWindowsUptr>;
			using			WvkLoaderMSWindowsWptr = std::weak_ptr<WvkLoaderMSWindows>;
			using			WvkLoaderMSWindowsWptrArr1 = std::vector<WvkLoaderMSWindowsWptr>;
			
		} // namespace mswindows

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_MSWINDOWS___MSWINDOWS_H