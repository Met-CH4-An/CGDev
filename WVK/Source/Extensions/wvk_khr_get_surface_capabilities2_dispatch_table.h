#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DISPATCH_TABLE_H
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
#include "../wvk_commands.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo {
				WvkLoaderPtr wvk_loader = nullptr;
				WvkInstancePtr wvk_instance = nullptr;
			}; // WvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo





			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrGetSurfaceCapabilities2DispatchTable : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static const std::string& s_getName(void) noexcept;

			private:

				inline static const std::string s_name = "VK_KHR_get_surface_capabilities2";

			public:

				inline VkResult wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats) const noexcept;				

			private:

				PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceFormats2KHR m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkKhrGetSurfaceCapabilities2DispatchTable(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkKhrGetSurfaceCapabilities2DispatchTable(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Инициализирует объект WvkKhrSurface, выполняя валидацию и загрузку Vulkan-команд.
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
				* Структура с параметрами создания поверхности. Обязательные поля: wvk_commands.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* Поверхность успешно создана, все функции загружены.
				*
				* @retval VknStatusCode::FAIL
				* Ошибка валидации или при загрузке Vulkan-функций.
				*
				* @code
				* WvkKhrSurface surface;
				* WvkKhrSurfaceCreateInfo info = { ... };
				* WvkStatus status = surface.create(info);
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*
				* @see WvkKhrSurface::validationCreateInfo
				* @see WvkKhrSurface::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			public:

			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Проверяет корректность структуры WvkKhrSurfaceCreateInfo.
				*
				* @details
				* Метод выполняет базовую валидацию входных данных, необходимых для корректной
				* работы WvkKhrSurface. На текущем этапе проверяется только наличие указателя
				* на WvkCommands. Если он равен nullptr, возвращается ошибка.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* Все обязательные данные присутствуют.
				*
				* @retval VknStatusCode::FAIL
				* Указатель на WvkCommands равен nullptr.
				*
				* @code
				* WvkKhrSurface surface;
				* WvkStatus status = surface.validationCreateInfo();
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*
				* @see WvkKhrSurface::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(void) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Загружает указатели на Vulkan-функции, связанные с KHR Surface расширением.
				*
				* @details
				* Использует механизм загрузки команд из объекта `wvk_commands`, чтобы получить
				* указатели на основные функции управления поверхностями (Surface) в Vulkan.
				*
				* @return
				* Возвращает статус выполнения операции:
				* - WvkStatus::ok(), если все функции были успешно загружены.
				* - WvkStatus::fail(), если произошла ошибка при загрузке хотя бы одной функции.
				*
				* @code
				* WvkKhrSurface surface;
				* WvkStatus status = surface.loadVulkanCommand();
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*
				* @see WvkCommands::tryLoadFunction
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus loadVulkanCommand(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void reset(void) noexcept;

			private:

				WvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo m_create_info = {};
			}; // class WvkKhrGetSurfaceCapabilities2DispatchTable

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_khr_get_surface_capabilities2_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DISPATCH_TABLE_H
