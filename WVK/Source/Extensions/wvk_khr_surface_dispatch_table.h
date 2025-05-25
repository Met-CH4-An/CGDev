#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DISPATCH_TABLE_H
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
#include "../wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkKhrSurfaceDispatchTableCreateInfo {
				WvkLoaderPtr wvk_loader = nullptr;
				WvkInstancePtr wvk_instance = nullptr;
			}; // WvkKhrSurfaceDispatchTableCreateInfo





			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrSurfaceDispatchTable : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static const std::string& s_getName(void) noexcept;

			private:

				inline static const std::string s_name = "VK_KHR_surface";

			public:
						
				inline void wvkDestroySurfaceKHR(VkSurfaceKHR surface, const VkAllocationCallbacks* pAllocator) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfaceSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfaceCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfaceFormatsKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfacePresentModesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes) const noexcept;
						
			private:

				PFN_vkDestroySurfaceKHR m_vkDestroySurfaceKHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceSupportKHR m_vkGetPhysicalDeviceSurfaceSupportKHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceFormatsKHR m_vkGetPhysicalDeviceSurfaceFormatsKHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfacePresentModesKHR m_vkGetPhysicalDeviceSurfacePresentModesKHR = VK_NULL_HANDLE;
					
			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkKhrSurfaceDispatchTable(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkKhrSurfaceDispatchTable(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Инициализирует объект WvkKhrSurfaceDispatchTable, выполняя валидацию и загрузку Vulkan-команд.
				*
				* @details
				* Метод сохраняет переданную структуру создания и выполняет два ключевых шага:
				* валидацию данных (при включённой сборке с проверками) и загрузку Vulkan-функций,
				* необходимых для работы с поверхностями (surface).
				*
				* Если валидация или загрузка функций завершаются с ошибкой, метод возвращает
				* соответствующий статус с подробным сообщением.
				*
				* @param[in] create_info
				* Структура с параметрами создания поверхности.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* Поверхность успешно создана, все функции загружены.
				*
				* @retval VknStatusCode::FAIL
				* Ошибка валидации или при загрузке Vulkan-функций.
				*
				* @code
				* WvkKhrSurfaceDispatchTable surface;
				* WvkKhrSurfaceDispatchTableCreateInfo info = { ... };
				* WvkStatus status = surface.create(info);
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*
				* @see WvkKhrSurfaceDispatchTable::validationCreateInfo
				* @see WvkKhrSurfaceDispatchTable::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkKhrSurfaceDispatchTableCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;
						
			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Проверяет корректность структуры WvkKhrSurfaceCreateInfo.
				*
				* @details
				* Метод выполняет базовую валидацию входных данных, необходимых для корректной
				* работы WvkKhrSurfaceDispatchTable. На текущем этапе проверяется только наличие указателя
				* на WvkCommands. Если он равен nullptr, возвращается ошибка.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* Все обязательные данные присутствуют.
				*
				* @retval VknStatusCode::FAIL
				* Указатель на WvkCommands равен nullptr.
				*
				* @code
				* WvkKhrSurfaceDispatchTable surface;
				* WvkStatus status = surface.validationCreateInfo();
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*
				* @see WvkKhrSurfaceDispatchTable::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(void) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* *Загружает адреса процедур Vulkan для расширения VK_KHR_surface.*
				*
				* Метод использует `WvkLoader::loadProcedure` для загрузки указателей на функции, связанные с `VK_KHR_surface`,
				* включая: `vkDestroySurfaceKHR`, `vkGetPhysicalDeviceSurfaceSupportKHR`, `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`,
				* `vkGetPhysicalDeviceSurfaceFormatsKHR`, `vkGetPhysicalDeviceSurfacePresentModesKHR`.
				*
				* Загрузка происходит через `invokeWithVkInstanceMethod`, который вызывает метод с текущим VkInstance.
				*
				* @return
				* Возвращает объект `WvkStatus`. В случае успеха — `WvkStatus::setOk()`, иначе код ошибки с сообщением.
				*
				* @code
				* WvkKhrSurfaceDispatchTable table;
				* WvkStatus status = table.loadVulkanCommand();
				* if (!status) {
				*     std::cerr << "Не удалось загрузить команды VK_KHR_surface: " << status.message() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus loadVulkanCommand(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void reset(void) noexcept;

			private:

				WvkKhrSurfaceDispatchTableCreateInfo m_create_info = {};
			}; // class WvkKhrSurfaceDispatchTable

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_khr_surface_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DISPATCH_TABLE_H
