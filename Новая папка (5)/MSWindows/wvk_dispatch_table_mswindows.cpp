////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_dispatch_table_mswindows.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../wvk_instance.h"
#include "../wvk_dispatch_table.h"

namespace CGDev {

	namespace wvk {
	
		namespace mswindows {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkDispatchTableMSWindows::WvkDispatchTableMSWindows(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkDispatchTableMSWindows::~WvkDispatchTableMSWindows(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDispatchTableMSWindows::create(const WvkDispatchTableMSWindowsCreateInfo& create_info) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Проверка на повторную инициализацию
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_valid) {
					return _status.set(VknStatusCode::ALREADY_INITIALIZED);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Проверка валидности входной структуры
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = validationCreateInfo(create_info);
				if (!_status) {
					destroy();
					return _status.setFail("WvkDispatchTableMSWindows::validationCreateInfo is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Получаем адресс функции vkGetInstanceAddr
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = create();
				if (!_status) {
					destroy();
					return _status.setFail("WvkDispatchTableMSWindows::create is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Загрузка указателей на Vulkan-функции
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = loadProcedure();
				if (!_status) {
					destroy();
					return _status.setFail("WvkDispatchTableMSWindows::loadProcedure is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Успех
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_valid = true;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkDispatchTableMSWindows::destroy(void) noexcept {
				// =======================================
				// [Category]: Surface
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_KHR_win32_surface
				// ~~~~~~~~~~~~~~~~
				m_vkCreateWin32SurfaceKHR = VK_NULL_HANDLE;

				if(m_vulkan_dll != NULL)
					FreeLibrary(m_vulkan_dll);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// очистка данных
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkDispatchTableMSWindowsCreateInfo m_create_info = {};
				m_vulkan_dll = NULL;
				m_vkGetInstanceProcAddr = VK_NULL_HANDLE;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDispatchTableMSWindows::validationCreateInfo(const WvkDispatchTableMSWindowsCreateInfo& create_info) noexcept {
				WvkStatus _status;

				if(create_info.wvk_dispatch_table_ptr == nullptr) {
					return _status.setFail("WvkDispatchTableMSWindowsCreateInfo::wvk_dispatch_table_ptr is nullptr.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Успех
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info = create_info;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDispatchTableMSWindows::create(void) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Загружаем Vulkan-библиотеку (vulkan-1.dll)
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_vulkan_dll = LoadLibraryA("vulkan-1.dll");
				
				if (m_vulkan_dll == NULL) {
					return _status.setFail("LoadLibraryA is NULL.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// получаем адрес функции vkGetInstanceProcAddr из библиотеки
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_vkGetInstanceProcAddr = reinterpret_cast<PFN_vkGetInstanceProcAddr>(
					GetProcAddress(m_vulkan_dll, "vkGetInstanceProcAddr")
					);

				if (m_vkGetInstanceProcAddr == NULL) {
					return _status.setFail("GetProcAddress is NULL.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Успех
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDispatchTableMSWindows::loadProcedure(void) noexcept {
				WvkStatus _status;

				std::vector<WvkVulkanProcedureInfo> _procedures;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Получаем процедуры, которые можно получить только при VkInstance
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_create_info.wvk_dispatch_table_ptr->getWvkInstance() != nullptr) {
					_procedures.emplace_back(WvkVulkanProcedureInfo("vkCreateWin32SurfaceKHR", reinterpret_cast<void**>(&m_vkCreateWin32SurfaceKHR)));

					_status = m_create_info.wvk_dispatch_table_ptr->loadProcedure([this](const char* name) {
						return m_vkGetInstanceProcAddr(m_create_info.wvk_dispatch_table_ptr->getWvkInstance()->getVkInstance(), name);
						},
						_procedures);

					if (!_status) {
						return _status.setFail("WvkDispatchTable::loadInstanceProcs is fail.");
					}
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Получаем процедуры, которые можно получить как при VkInstance, так и при VkDevice
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_create_info.wvk_dispatch_table_ptr->getWvkInstance() != nullptr || m_create_info.wvk_dispatch_table_ptr->getWvkLogicalDevice() != nullptr) {
					_procedures.clear();
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Успех
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace mswindows

	} // namespace wvk

} // namespace CGDev