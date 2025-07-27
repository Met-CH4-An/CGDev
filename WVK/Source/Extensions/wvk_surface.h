#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
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
			struct WvkSurfacePlatformCreateInfo {
			}; // struct WvkSurfacePlatformCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief 
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkSurfaceCreateInfo {
				WvkInstanceDispatchTablePtr wvk_instance_dt_ptr = nullptr;
				WvkKhrSurfaceDTPtr wvk_khr_surface_dt = nullptr;
				WvkKhrGetSurfaceCapabilities2DispatchTablePtr wvk_khr_get_surface_capabilities2_dispatch_table = nullptr;
				WvkSurfacePlatformCreateInfo* wvk_surface_platform_create_info = nullptr;
			}; // struct WvkSurfaceCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkSurface : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkSurface(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkSurface(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkSurfaceCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Запрашивает список поддерживаемых форматов поверхности (VkSurfaceFormatKHR) для указанного физического устройства.
				*
				* @details
				*     Метод реализует двухшаговый вызов vkGetPhysicalDeviceSurfaceFormatsKHR через dispatch table:
				*     сначала получает количество доступных форматов, затем — сами форматы.
				*     Если расширение VK_KHR_surface не активировано, возвращает статус FEATURE_NOT_ENABLED.
				*     В случае ошибки возвращает подробное сообщение и код FAIL.
				*
				*     Алгоритм работы:
				*     - Проверяет, активировано ли расширение VK_KHR_surface.
				*     - Очищает выходной вектор и запрашивает количество поддерживаемых форматов.
				*     - Если количество больше нуля, выделяет память под вектор форматов.
				*     - Повторно вызывает vkGetPhysicalDeviceSurfaceFormatsKHR для получения самих форматов.
				*     - Обрабатывает возможные ошибки (в том числе VK_INCOMPLETE) и возвращает статус.
				*
				* @param[in]  wvk_physical_device_ptr
				*     Указатель на объект физического устройства Vulkan (WvkPhysicalDevicePtr).
				*
				* @param[out] out
				*     Вектор, в который будут записаны поддерживаемые форматы поверхности (VkSurfaceFormatKHR).
				*
				* @retval WvkStatus
				*     - VknStatusCode::SUCCESSFUL — если форматы успешно получены.
				*     - VknStatusCode::FAIL — если произошла ошибка при вызове Vulkan API.
				*     - VknStatusCode::FEATURE_NOT_ENABLED — если VK_KHR_surface не активирован.
				*
				* @code
				* std::vector<VkSurfaceFormatKHR> formats;
				* WvkStatus status = surface.requestFormats(physicalDevice, formats);
				* if (!status) {
				*     std::cerr << status.getMessage() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestFormats(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkSurfaceFormatKHR>& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Проверяет поддержку презентации (present support) для указанного семейства очередей на данном surface.
				*
				* @details
				*     Метод вызывает vkGetPhysicalDeviceSurfaceSupportKHR (через dispatch table) для проверки,
				*     может ли выбранное семейство очередей физического устройства выполнять презентацию на текущий surface.
				*     Если расширение VK_KHR_surface не активировано, возвращает статус FEATURE_NOT_ENABLED.
				*     В случае успеха в параметр out записывается true (если поддержка есть) или false (если нет).
				*
				*     Алгоритм работы:
				*     - Проверяет наличие поддержки VK_KHR_surface.
				*     - Вызывает vkGetPhysicalDeviceSurfaceSupportKHR для выбранного физического устройства и семейства очередей.
				*     - Обрабатывает возможные ошибки и возвращает статус.
				*
				* @param[in]  wvk_physical_device_ptr
				*     Указатель на объект физического устройства Vulkan (WvkPhysicalDevicePtr).
				*
				* @param[in]  wvk_queue_family_ptr
				*     Указатель на объект семейства очередей (WvkQueueFamilyPtr), для которого проверяется поддержка презентации.
				*
				* @param[out] out
				*     Булево значение: true, если семейство очередей поддерживает презентацию на данном surface, иначе false.
				*
				* @retval WvkStatus
				*     - VknStatusCode::SUCCESSFUL — если проверка выполнена успешно.
				*     - VknStatusCode::FAIL — если произошла ошибка при вызове Vulkan API.
				*     - VknStatusCode::FEATURE_NOT_ENABLED — если VK_KHR_surface не активирован.
				*
				* @code
				* bool presentSupported = false;
				* WvkStatus status = surface.requestSupport(physicalDevice, queueFamily, presentSupported);
				* if (status && presentSupported) {
				*     // Очередь поддерживает present
				* } else if (!status) {
				*     std::cerr << status.getMessage() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestSupport(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const WvkQueueFamilyPtr wvk_queue_family_ptr, bool& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Получает список поддерживаемых режимов презентации (VkPresentModeKHR) для текущей поверхности.
				*
				* @param[in]  wvk_physical_device_ptr   Указатель на физическое устройство Vulkan (обёртка WvkPhysicalDevicePtr).
				* @param[out] vk_present_modes      Вектор, в который будут записаны поддерживаемые режимы презентации.
				*
				* @return Возвращает статус выполнения операции.
				*
				* Возможные ошибки:
				* - VK_ERROR_OUT_OF_HOST_MEMORY
				* - VK_ERROR_OUT_OF_DEVICE_MEMORY
				* - VK_ERROR_SURFACE_LOST_KHR
				* - VK_INCOMPLETE (при втором вызове)
				*
				* @code
				* WvkSurface surface = ...;
				* WvkPhysicalDevicePtr device = ...;
				* std::vector<VkPresentModeKHR> modes;
				* WvkStatus status = surface.requestPresentModes(device, modes);
				* if (status) {
				*     // Успешно
				*     printPresentModes(modes);
				* } else {
				*     std::cerr << status.message() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestPresentModes(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkPresentModeKHR>& vk_present_modes) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Запрашивает основные возможности поверхности (surface capabilities) для указанного физического устройства.
				*
				* @details
				*     Метод вызывает функцию vkGetPhysicalDeviceSurfaceCapabilitiesKHR через dispatch table, чтобы получить
				*     параметры поверхности (например, минимальное и максимальное количество изображений, размеры, поддерживаемые трансформации и т.д.).
				*     Если расширение VK_KHR_surface не активировано, возвращает статус FEATURE_NOT_ENABLED.
				*     В случае ошибки возвращает подробное сообщение и код FAIL.
				*
				*     Алгоритм работы:
				*     - Проверяет, активировано ли расширение VK_KHR_surface.
				*     - Вызывает vkGetPhysicalDeviceSurfaceCapabilitiesKHR для текущей поверхности и физического устройства.
				*     - Обрабатывает возможные ошибки и возвращает статус.
				*
				* @param[in]  wvk_physical_device_ptr
				*     Указатель на объект физического устройства Vulkan (WvkPhysicalDevicePtr).
				*
				* @param[out] vk_surface_capabilities_khr
				*     Структура VkSurfaceCapabilitiesKHR, в которую будут записаны возможности поверхности.
				*
				* @retval WvkStatus
				*     - VknStatusCode::SUCCESSFUL — если запрос выполнен успешно.
				*     - VknStatusCode::FAIL — если произошла ошибка при вызове Vulkan API.
				*     - VknStatusCode::FEATURE_NOT_ENABLED — если VK_KHR_surface не активирован.
				*
				* @code
				* VkSurfaceCapabilitiesKHR caps{};
				* WvkStatus status = surface.requestCapabilities(physicalDevice, caps);
				* if (!status) {
				*     std::cerr << status.getMessage() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceCapabilitiesKHR& vk_surface_capabilities_khr) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Запрашивает список режимов презентации, совместимых с заданным `VkPresentModeKHR`.
				*
				* @param [in]  wvk_physical_device_ptr
				*     Указатель на обёртку физического устройства, на котором выполняется запрос.
				*
				* @param [in]  vk_present_mode
				*     Режим презентации, для которого требуется получить список совместимых режимов.
				*
				* @param [out] out
				*     Вектор, в который будет записан список совместимых режимов презентации (`VkPresentModeKHR`).
				*
				* @return
				*     Объект `WvkStatus`, отражающий успешность операции.
				*
				* @code
				* std::vector<VkPresentModeKHR> compatible_modes;
				* WvkStatus status = wvk_surface->requestPresentModeCompatibility(
				*     wvk_physical_device_ptr,
				*     VK_PRESENT_MODE_MAILBOX_KHR,
				*     compatible_modes
				* );
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestPresentModeCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, std::vector<VkPresentModeKHR>& out) const noexcept;

				

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Выполняет запрос расширенных возможностей поверхности (surface capabilities) с использованием цепочек pNext.
				*
				* @details
				*     Метод реализует универсальный механизм получения расширенных возможностей поверхности через функцию
				*     vkGetPhysicalDeviceSurfaceCapabilities2KHR (или аналогичную для вашей платформы/dispatch table).
				*     Позволяет передавать цепочки входных (VkBaseInStructure) и выходных (VkBaseOutStructure) структур для поддержки расширений.
				*
				*     Алгоритм работы:
				*     - Проверяет, что активирован Vulkan 1.1 или расширение VK_KHR_surface.
				*     - Формирует структуру VkPhysicalDeviceSurfaceInfo2KHR, указывая surface и цепочку in.
				*     - Формирует структуру VkSurfaceCapabilities2KHR, указывая цепочку out.
				*     - Вызывает функцию получения surface capabilities через dispatch table.
				*     - Обрабатывает возможные ошибки и возвращает статус.
				*     - Если VK_KHR_surface не активирован — возвращает FEATURE_NOT_ENABLED.
				*
				* @param[in]  wvk_physical_device_ptr
				*     Указатель на объект физического устройства Vulkan (WvkPhysicalDevicePtr).
				*
				* @param[in]  in
				*     Указатель на цепочку входных структур, унаследованных от VkBaseInStructure (или nullptr, если не требуется).
				*
				* @param[out] out
				*     Указатель на цепочку выходных структур, унаследованных от VkBaseOutStructure, куда будут записаны результаты.
				*
				* @retval WvkStatus
				*     - VknStatusCode::SUCCESSFUL — если запрос выполнен успешно.
				*     - VknStatusCode::FAIL — если произошла ошибка при вызове Vulkan API.
				*     - VknStatusCode::FEATURE_NOT_ENABLED — если VK_KHR_surface не активирован.
				*
				* @code
				* VkSurfacePresentScalingCapabilitiesEXT scaling_caps = { VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT };
				* WvkStatus status = surface.requestCapabilities(device, &inStruct, &scaling_caps);
				* if (!status) {
				*     std::cerr << status.getMessage() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkBaseInStructure* in, VkBaseOutStructure* out) const noexcept;

			// hpp
			public:

				template<typename In, typename Out>
				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const In& in, Out& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Запрашивает расширенный список поддерживаемых форматов поверхности с использованием vkGetPhysicalDeviceSurfaceFormats2KHR.
				*
				* @details
				*     Метод позволяет получить расширенную информацию о форматах поверхности, используя цепочку pNext для передачи дополнительных структур.
				*     Сначала производится запрос количества доступных форматов, затем выделяется память и запрашиваются сами форматы с дополнительными свойствами.
				*
				* @tparam T
				*     Тип структуры, унаследованной от VkBaseOutStructure, в которую будут записаны дополнительные свойства формата (например, VkSurfaceFormatKHR + расширения).
				*
				* @param[in]  wvk_physical_device_ptr
				*     Указатель на обёртку физического устройства Vulkan, для которого выполняется запрос.
				* @param[in]  in
				*     Указатель на цепочку входных структур (VkBaseInStructure), либо nullptr, если не требуется.
				* @param[in]  prop_sType
				*     Значение VkStructureType для структуры T, которая будет помещена в pNext каждого VkSurfaceFormat2KHR.
				* @param[out] out_props
				*     Вектор, в который будут записаны структуры T с расширенной информацией о форматах.
				*
				* @return
				*     Объект WvkStatus, отражающий успешность операции.
				*
				* @code
				* std::vector<VkDrmFormatModifierPropertiesEXT> modifiers;
				* WvkStatus status = surface->requestFormats(device, &inStruct, VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT, modifiers);
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				template<typename T>
				inline WvkStatus requestFormats(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkBaseInStructure* in, const VkStructureType& prop_sType, std::vector<T>& out_props) const noexcept;

			// helper
			public:

				WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceProtectedCapabilitiesKHR& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Запрашивает возможности масштабирования изображения (scaling capabilities)
				*     для заданного режима презентации (`VkPresentModeKHR`) через цепочку `pNext`.
				*
				* @param [in]  wvk_physical_device_ptr
				*     Указатель на объект логического представления физического устройства.
				*
				* @param [in]  vk_present_mode
				*     Режим презентации, для которого требуется получить информацию о масштабировании.
				*
				* @param [out] out
				*     Структура `VkSurfacePresentScalingCapabilitiesEXT`, куда будет записана
				*     информация о поддерживаемых режимах масштабирования.
				*
				* @return
				*     Объект `WvkStatus`, содержащий результат операции.
				*
				* @code
				* VkSurfacePresentScalingCapabilitiesEXT scaling_caps = {};
				* WvkStatus status = wvk_surface->requestScalingCompatibility(
				*     wvk_physical_device_ptr,
				*     VK_PRESENT_MODE_FIFO_KHR,
				*     scaling_caps
				* );
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestScalingCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, VkSurfacePresentScalingCapabilitiesEXT& out) const noexcept;		

			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(void) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief 
				* Создаёт VkSurfaceKHR в зависимости от текущей платформы.
				* 
				* @details
				* На данный момент реализована поддержка только платформы MSWindows. Используется
				* структура WvkSurfaceMSWindowsCreateInfo, которая содержит дескрипторы окна и инстанса,
				* а также указатель на расширение WvkKhrWin32Surface.
				* 
				* @return WvkStatus 
				* Возвращает статус выполнения. Если операция прошла успешно — `OK`. В противном случае — код ошибки.
				* 
				* @code
				* WvkSurface surface;
				* surface.setCreateInfo(...); // Установить платформенно-зависимую информацию
				* WvkStatus status = surface.createVkSurface();
				* if (!status.isOk()) {
				*     log->error(status.message());
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus createVkSurface(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void reset(void) noexcept;

			private:

				WvkSurfaceCreateInfo m_create_info = {};
				VkSurfaceKHR m_vk_surface = VK_NULL_HANDLE;
				struct WvkSurfaceImpl;
				std::unique_ptr<WvkSurfaceImpl> m_surface_impl = nullptr;
			}; // class WvkSurface

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_surface.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
