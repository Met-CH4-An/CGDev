////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_khr_surface_dispatch_table.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkKhrSurfaceDispatchTable::WvkKhrSurfaceDispatchTable(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkKhrSurfaceDispatchTable::~WvkKhrSurfaceDispatchTable(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrSurfaceDispatchTable::create(const WvkKhrSurfaceDispatchTableCreateInfo& create_info) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Проверка на повторную инициализацию
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_valid) {
					return _status.set(VknStatusCode::ALREADY_INITIALIZED);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Сохраняем переданную структуру создания.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info = create_info;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Если включена сборка с валидацией, запускаем проверку данных.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
					_status = validationCreateInfo();

					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTable::validationCreateInfo() - fail.");
					}
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Загружаем указатели на Vulkan-функции, связанные с Surface.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = loadVulkanCommand();

				if (_status.m_code != VknStatusCode::SUCCESSFUL) {
					reset();
					return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTable::loadVulkanCommand() - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Успешное завершение создания.
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_valid = true;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkKhrSurfaceDispatchTable::destroy(void) noexcept {
				reset();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrSurfaceDispatchTable::validationCreateInfo(void) const noexcept {
				WvkStatus _status;

				if (m_create_info.wvk_loader == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTableCreateInfo::wvk_loader - nullptr.");
				}
				else if(m_create_info.wvk_instance == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTableCreateInfo::wvk_instance - nullptr.");
				}

				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkKhrSurfaceDispatchTable::loadVulkanCommand(void) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Подготовка списка процедур, которые необходимо загрузить
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<WvkVulkanProcedureInfo> _procedures = {
					{ "vkDestroySurfaceKHR", reinterpret_cast<void**>(&m_vkDestroySurfaceKHR) },
					{ "vkGetPhysicalDeviceSurfaceSupportKHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceSupportKHR) },
					{ "vkGetPhysicalDeviceSurfaceCapabilitiesKHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR) },
					{ "vkGetPhysicalDeviceSurfaceFormatsKHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormatsKHR) },
					{ "vkGetPhysicalDeviceSurfacePresentModesKHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfacePresentModesKHR) }
				};

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Вызов метода загрузки через обёртку с VkInstance
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
					&WvkLoader::loadProcedure,
					m_create_info.wvk_loader,
					_procedures
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Проверка результата загрузки
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (!_status) {
					return _status.set(VknStatusCode::FAIL,
						"\n\tWvkInstance::invokeWithVkInstanceMethod - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Возврат успешного статуса
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkKhrSurfaceDispatchTable::reset(void) noexcept {
				m_vkDestroySurfaceKHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSurfaceSupportKHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSurfaceFormatsKHR = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSurfacePresentModesKHR = VK_NULL_HANDLE;

				m_create_info = {};

				m_valid = false;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev