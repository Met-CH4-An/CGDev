#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DT_HPP
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DT_HPP
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
#include "../wvk_loader.h"
#include "../wvk_instance.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkKhrSurfaceDTCreateInfo {
				WvkLoaderPtr wvk_loader = nullptr;
				WvkInstancePtr wvk_instance = nullptr;
			}; // WvkKhrSurfaceDTCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrSurfaceDT : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static constexpr std::string_view s_getName(void) noexcept {
					return s_name;
				}

			private:

				inline static constexpr std::string_view s_name = "VK_KHR_surface";

			public:
						
				inline void wvkDestroySurfaceKHR(VkSurfaceKHR surface, const VkAllocationCallbacks* pAllocator) const noexcept {
					m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkDestroySurfaceKHR, surface, pAllocator); }
				inline VkResult wvkGetPhysicalDeviceSurfaceSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice, queueFamilyIndex, surface, pSupported); }
				inline VkResult wvkGetPhysicalDeviceSurfaceCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice, surface, pSurfaceCapabilities); }
				inline VkResult wvkGetPhysicalDeviceSurfaceFormatsKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice, surface, pSurfaceFormatCount, pSurfaceFormats); }
				inline VkResult wvkGetPhysicalDeviceSurfacePresentModesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes) const noexcept {
					return m_vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice, surface, pPresentModeCount, pPresentModes); }
						
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
				inline WvkKhrSurfaceDT(void) noexcept {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline ~WvkKhrSurfaceDT(void) noexcept {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Инициализирует объект WvkKhrSurfaceDT, выполняя валидацию и загрузку Vulkan-команд.
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
				* WvkKhrSurfaceDT surface;
				* WvkKhrSurfaceDTCreateInfo info = { ... };
				* WvkStatus status = surface.create(info);
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*
				* @see WvkKhrSurfaceDT::validationCreateInfo
				* @see WvkKhrSurfaceDT::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus create(const WvkKhrSurfaceDTCreateInfo& create_info) noexcept {
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
					// Шаг 3. Запускаем проверку данных.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~				
					_status = validationCreateInfo();

					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTable::validationCreateInfo() - fail.");
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
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void destroy(void) noexcept {
					reset();
				}
						
			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Проверяет корректность структуры WvkKhrSurfaceCreateInfo.
				*
				* @details
				* Метод выполняет базовую валидацию входных данных, необходимых для корректной
				* работы WvkKhrSurfaceDT. На текущем этапе проверяется только наличие указателя
				* на WvkCommands. Если он равен nullptr, возвращается ошибка.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* Все обязательные данные присутствуют.
				*
				* @retval VknStatusCode::FAIL
				* Указатель на WvkCommands равен nullptr.
				*
				* @code
				* WvkKhrSurfaceDT surface;
				* WvkStatus status = surface.validationCreateInfo();
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*
				* @see WvkKhrSurfaceDT::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus validationCreateInfo(void) const noexcept {
					WvkStatus _status;

					if (m_create_info.wvk_loader == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTableCreateInfo::wvk_loader - nullptr.");
					}
					else if (m_create_info.wvk_instance == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTableCreateInfo::wvk_instance - nullptr.");
					}

					return _status.setOk();
				}

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
				* WvkKhrSurfaceDT table;
				* WvkStatus status = table.loadVulkanCommand();
				* if (!status) {
				*     std::cerr << "Не удалось загрузить команды VK_KHR_surface: " << status.message() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus loadVulkanCommand(void) noexcept {
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
					//_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
					//	&WvkLoader::loadProcedure,
					//	m_create_info.wvk_loader,
					//	_procedures
					//);

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
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void reset(void) noexcept {
					m_vkDestroySurfaceKHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfaceSupportKHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfaceFormatsKHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfacePresentModesKHR = VK_NULL_HANDLE;

					m_create_info = {};

					m_valid = false;
				}

			private:

				WvkKhrSurfaceDTCreateInfo m_create_info = {};
			}; // class WvkKhrSurfaceDT

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DT_HPP
