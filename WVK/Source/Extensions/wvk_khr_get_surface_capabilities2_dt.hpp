#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DT_HPP
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DT_HPP
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
			struct WvkKhrGetSurfaceCapabilities2DTCreateInfo {
				WvkLoaderPtr wvk_loader = nullptr;
				WvkInstancePtr wvk_instance = nullptr;
			}; // WvkKhrGetSurfaceCapabilities2DTCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrGetSurfaceCapabilities2DT : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static constexpr std::string_view s_getName(void) noexcept {
					return s_name;
				}

			private:

				inline static constexpr std::string_view s_name = "VK_KHR_get_surface_capabilities2";

			public:

				inline VkResult wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice, pSurfaceInfo, pSurfaceCapabilities); }
				inline VkResult wvkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceFormats2KHR(physicalDevice, pSurfaceInfo, pSurfaceFormatCount, pSurfaceFormats); }

			private:

				PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceFormats2KHR m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkKhrGetSurfaceCapabilities2DT(void) noexcept {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline ~WvkKhrGetSurfaceCapabilities2DT(void) noexcept {}

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
				inline WvkStatus create(const WvkKhrGetSurfaceCapabilities2DTCreateInfo& create_info) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Сохраняем переданную структуру создания.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Запускаем проверку данных.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~					
					_status = validationCreateInfo();

					if (!_status) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DT::validationCreateInfo() - fail.");
					}					

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Загружаем указатели на Vulkan-функции, связанные с Surface.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = loadVulkanCommand();

					if (!_status) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DT::loadVulkanCommand() - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Успешное завершение создания.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_valid = true;
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void destroy(void) noexcept {
					reset();
				}

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
				inline WvkStatus validationCreateInfo(void) const noexcept {
					WvkStatus _status;

					if (m_create_info.wvk_loader == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo::wvk_loader - nullptr.");
					}
					else if (m_create_info.wvk_instance == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo::wvk_instance - nullptr.");
					}

					return _status.setOk();
				}

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
				inline WvkStatus loadVulkanCommand(void) noexcept {
					WvkStatus _status;

					std::vector<WvkVulkanProcedureInfo> _procedures = {
						{ "vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR) },
						{ "vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR) }
					};

					_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
						&WvkLoader::loadProcedure,
						m_create_info.wvk_loader,
						_procedures
					);

					if (!_status) {
						return _status.set(VknStatusCode::FAIL,
							"\n\tWvkInstance::invokeWithVkInstanceMethod - fail.");
					}

					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void reset(void) noexcept {
					m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

					m_create_info = {};

					m_valid = false;
				}

			private:

				WvkKhrGetSurfaceCapabilities2DTCreateInfo m_create_info = {};
			}; // class WvkKhrGetSurfaceCapabilities2DT

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DT_HPP
